use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use codex_hepta_matrix_protocol::MatrixUserId;
use codex_hepta_matrix_store::OutboxRecord;
use codex_hepta_paths::HeptaAgentLayout;
use matrix_sdk::Client;
use matrix_sdk::Error as MatrixSdkTransportError;
use matrix_sdk::HttpError;
use matrix_sdk::LoopCtrl;
use matrix_sdk::Room;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::OwnedRoomId;
use matrix_sdk::ruma::OwnedTransactionId;
use matrix_sdk::ruma::UInt;
use matrix_sdk::ruma::api::client::filter::FilterDefinition;
use matrix_sdk::ruma::api::client::sync::sync_events::v3::Filter;
use matrix_sdk::ruma::events::room::message::MessageType;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::IngressIgnoredReason;
use crate::MatrixIngress;
use crate::MatrixOutboundTransport;
use crate::MatrixSdkPaths;
use crate::MatrixSendFuture;
use crate::MatrixSession;
use crate::MatrixSidecarConfig;
use crate::MatrixSidecarConfigError;
use crate::MatrixTimelineEvent;
use crate::MatrixTransportError;

pub struct MatrixSdkClient {
    client: Client,
    config: MatrixSidecarConfig,
    paths: MatrixSdkPaths,
    handler_installed: AtomicBool,
}

impl MatrixSdkClient {
    pub async fn restore(
        layout: &HeptaAgentLayout,
        config: MatrixSidecarConfig,
        session: MatrixSession,
        store_passphrase: Option<&str>,
    ) -> Result<Self, MatrixSdkError> {
        verify_session_identity(&config, &session)?;
        let sidecar = Self::build(layout, config, store_passphrase).await?;
        sidecar
            .client
            .restore_session(session)
            .await
            .map_err(|_| MatrixSdkError::Authentication)?;
        sidecar.verify_authenticated_identity()?;
        sidecar.enable_event_cache()?;
        Ok(sidecar)
    }

    pub async fn login_password(
        layout: &HeptaAgentLayout,
        config: MatrixSidecarConfig,
        password: &str,
        store_passphrase: Option<&str>,
        device_display_name: Option<&str>,
    ) -> Result<(Self, MatrixSession), MatrixSdkError> {
        let sidecar = Self::build(layout, config, store_passphrase).await?;
        let mut login = sidecar
            .client
            .matrix_auth()
            .login_username(sidecar.config.binding.expected_mxid.as_str(), password)
            .device_id(sidecar.config.binding.expected_device_id.as_str())
            .request_refresh_token();
        if let Some(display_name) = device_display_name {
            login = login.initial_device_display_name(display_name);
        }
        let response = login
            .send()
            .await
            .map_err(|_| MatrixSdkError::Authentication)?;
        let session = MatrixSession::from(&response);
        verify_session_identity(&sidecar.config, &session)?;
        sidecar.verify_authenticated_identity()?;
        sidecar.enable_event_cache()?;
        Ok((sidecar, session))
    }

    async fn build(
        layout: &HeptaAgentLayout,
        config: MatrixSidecarConfig,
        store_passphrase: Option<&str>,
    ) -> Result<Self, MatrixSdkError> {
        let paths = MatrixSdkPaths::prepare(layout, &config)?;
        let client = Client::builder()
            .homeserver_url(config.binding.homeserver.as_str())
            .sqlite_store_with_cache_path(paths.state(), paths.cache(), store_passphrase)
            .handle_refresh_tokens()
            .build()
            .await
            .map_err(|_| MatrixSdkError::Initialization)?;
        verify_homeserver(&config, &client)?;
        Ok(Self {
            client,
            config,
            paths,
            handler_installed: AtomicBool::new(false),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn config(&self) -> &MatrixSidecarConfig {
        &self.config
    }

    pub fn paths(&self) -> &MatrixSdkPaths {
        &self.paths
    }

    pub fn install_ingress_handler(&self, ingress: MatrixIngress) -> Result<(), MatrixSdkError> {
        if self.handler_installed.swap(true, Ordering::AcqRel) {
            return Err(MatrixSdkError::HandlerAlreadyInstalled);
        }
        self.client
            .add_event_handler(move |event: OriginalSyncRoomMessageEvent, room: Room| {
                let ingress = ingress.clone();
                async move {
                    if !is_text_message(&event.content) {
                        ingress.record_ignored(IngressIgnoredReason::UnsupportedMessageType);
                        return;
                    }
                    let event = match timeline_event_from_sdk(event, &room) {
                        Ok(event) => event,
                        Err(_) => {
                            ingress.record_malformed_event();
                            return;
                        }
                    };
                    let _ = ingress.ingest(event).await;
                }
            });
        Ok(())
    }

    /// Run bounded long-poll sync until cancellation or an ingress fence.
    ///
    /// Cancellation is observed at a completed `/sync` boundary instead of by
    /// dropping an in-flight event handler. The configured request timeout
    /// therefore bounds shutdown while allowing the current handler to finish
    /// its durable inbox write.
    pub async fn sync_until_cancelled(
        &self,
        ingress: &MatrixIngress,
        cancel: &CancellationToken,
    ) -> Result<MatrixSyncExit, MatrixSdkError> {
        if !self.handler_installed.load(Ordering::Acquire) {
            return Err(MatrixSdkError::HandlerMissing);
        }
        if cancel.is_cancelled() {
            return Ok(MatrixSyncExit::Cancelled);
        }
        let settings = bounded_sync_settings(&self.config)?;
        let cancel = cancel.clone();
        let callback_ingress = ingress.clone();
        self.client
            .sync_with_callback(settings, move |_| {
                let cancel = cancel.clone();
                let ingress = callback_ingress.clone();
                async move {
                    if cancel.is_cancelled() || ingress.fatal() {
                        LoopCtrl::Break
                    } else {
                        LoopCtrl::Continue
                    }
                }
            })
            .await
            .map_err(|_| MatrixSdkError::Sync)?;
        if ingress.fatal() {
            Ok(MatrixSyncExit::IngressFenced)
        } else {
            Ok(MatrixSyncExit::Cancelled)
        }
    }

    fn enable_event_cache(&self) -> Result<(), MatrixSdkError> {
        self.client
            .event_cache()
            .subscribe()
            .map_err(|_| MatrixSdkError::Initialization)
    }

    fn verify_authenticated_identity(&self) -> Result<(), MatrixSdkError> {
        let user_id = self
            .client
            .user_id()
            .ok_or(MatrixSdkError::Authentication)?;
        let device_id = self
            .client
            .device_id()
            .ok_or(MatrixSdkError::Authentication)?;
        if user_id.as_str() != self.config.binding.expected_mxid.as_str()
            || device_id.as_str() != self.config.binding.expected_device_id.as_str()
        {
            return Err(MatrixSdkError::IdentityMismatch);
        }
        verify_homeserver(&self.config, &self.client)
    }
}

impl MatrixOutboundTransport for MatrixSdkClient {
    fn send<'a>(&'a self, record: &'a OutboxRecord) -> MatrixSendFuture<'a> {
        Box::pin(async move {
            if !self.config.binding.allowed_rooms.contains(&record.room_id)
                || record.binding_revision != self.config.binding.revision
                || record.generation != self.config.agent_generation
            {
                return Err(MatrixTransportError::Permanent);
            }
            let body = std::str::from_utf8(&record.payload)
                .map_err(|_| MatrixTransportError::Permanent)?;
            let room_id = OwnedRoomId::try_from(record.room_id.as_str())
                .map_err(|_| MatrixTransportError::Permanent)?;
            let room = self
                .client
                .get_room(&room_id)
                .ok_or(MatrixTransportError::Retryable)?;
            let txn_id = OwnedTransactionId::from(record.stable_txn_id.as_str());
            let content = serde_json::json!({
                "msgtype": "m.text",
                "body": body,
            });
            let response = room
                .send_raw("m.room.message", content)
                .with_transaction_id(&txn_id)
                .await
                .map_err(|error| classify_sdk_send_error(&error))?;
            MatrixEventId::parse(response.response.event_id.as_str())
                .map_err(|_| MatrixTransportError::Permanent)
        })
    }
}

fn classify_sdk_send_error(error: &MatrixSdkTransportError) -> MatrixTransportError {
    match error {
        MatrixSdkTransportError::Http(error) => classify_http_error(error),
        MatrixSdkTransportError::Timeout | MatrixSdkTransportError::ConcurrentRequestFailed => {
            MatrixTransportError::Retryable
        }
        _ => MatrixTransportError::Permanent,
    }
}

fn classify_http_error(error: &HttpError) -> MatrixTransportError {
    match error {
        HttpError::Reqwest(_) => MatrixTransportError::Retryable,
        HttpError::Api(_) => error
            .as_client_api_error()
            .map(|error| classify_http_status(error.status_code.as_u16()))
            .unwrap_or(MatrixTransportError::Permanent),
        HttpError::Cached(error) => classify_http_error(error),
        HttpError::IntoHttp(_) | HttpError::RefreshToken(_) => MatrixTransportError::Permanent,
        #[cfg(target_os = "android")]
        HttpError::VerifierBuilder(_) => MatrixTransportError::Permanent,
    }
}

fn classify_http_status(status: u16) -> MatrixTransportError {
    if status == 429 || (500..=599).contains(&status) {
        MatrixTransportError::Retryable
    } else {
        MatrixTransportError::Permanent
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixSyncExit {
    Cancelled,
    IngressFenced,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum MatrixSdkError {
    #[error("invalid Matrix sidecar configuration")]
    Configuration,
    #[error("Matrix SDK persistent store initialization failed")]
    Initialization,
    #[error("Matrix authentication failed")]
    Authentication,
    #[error("Matrix session identity differs from the exact binding")]
    IdentityMismatch,
    #[error("Matrix ingress handler is already installed")]
    HandlerAlreadyInstalled,
    #[error("Matrix ingress handler has not been installed")]
    HandlerMissing,
    #[error("Matrix sync failed")]
    Sync,
}

impl From<MatrixSidecarConfigError> for MatrixSdkError {
    fn from(_: MatrixSidecarConfigError) -> Self {
        Self::Configuration
    }
}

fn verify_session_identity(
    config: &MatrixSidecarConfig,
    session: &MatrixSession,
) -> Result<(), MatrixSdkError> {
    if session.meta.user_id.as_str() != config.binding.expected_mxid.as_str()
        || session.meta.device_id.as_str() != config.binding.expected_device_id.as_str()
    {
        return Err(MatrixSdkError::IdentityMismatch);
    }
    Ok(())
}

fn verify_homeserver(config: &MatrixSidecarConfig, client: &Client) -> Result<(), MatrixSdkError> {
    let expected = Url::parse(config.binding.homeserver.as_str())
        .map_err(|_| MatrixSdkError::Configuration)?;
    if client.homeserver() != expected {
        return Err(MatrixSdkError::IdentityMismatch);
    }
    Ok(())
}

fn bounded_sync_settings(config: &MatrixSidecarConfig) -> Result<SyncSettings, MatrixSdkError> {
    let mut definition = FilterDefinition::default();
    definition.room.rooms = Some(
        config
            .binding
            .allowed_rooms
            .iter()
            .map(|room_id| OwnedRoomId::try_from(room_id.as_str()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| MatrixSdkError::Configuration)?,
    );
    definition.room.timeline.limit = Some(UInt::from(config.sync_timeline_limit));
    definition.room.timeline.types = Some(vec!["m.room.message".to_string()]);
    Ok(SyncSettings::new()
        .timeout(config.sync_timeout)
        .filter(Filter::FilterDefinition(definition)))
}

fn timeline_event_from_sdk(
    event: OriginalSyncRoomMessageEvent,
    room: &Room,
) -> Result<MatrixTimelineEvent, MatrixSdkError> {
    let content = serde_json::to_value(&event.content).map_err(|_| MatrixSdkError::Sync)?;
    let payload = serde_json::to_vec(&content).map_err(|_| MatrixSdkError::Sync)?;
    let mentioned_user_ids = parse_mentioned_users(&content)?;
    Ok(MatrixTimelineEvent {
        event_id: MatrixEventId::parse(event.event_id.as_str())
            .map_err(|_| MatrixSdkError::Sync)?,
        room_id: MatrixRoomId::parse(room.room_id().as_str()).map_err(|_| MatrixSdkError::Sync)?,
        sender: MatrixUserId::parse(event.sender.as_str()).map_err(|_| MatrixSdkError::Sync)?,
        event_type: "m.room.message".to_string(),
        payload,
        mentioned_user_ids,
        origin_server_ts_ms: u64::from(event.origin_server_ts.get()),
        received_at_ms: system_time_ms()?,
    })
}

fn is_text_message(content: &RoomMessageEventContent) -> bool {
    matches!(&content.msgtype, MessageType::Text(_))
}

fn parse_mentioned_users(content: &Value) -> Result<Vec<MatrixUserId>, MatrixSdkError> {
    let Some(user_ids) = content
        .get("m.mentions")
        .and_then(|mentions| mentions.get("user_ids"))
    else {
        return Ok(Vec::new());
    };
    let user_ids = user_ids.as_array().ok_or(MatrixSdkError::Sync)?;
    user_ids
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or(MatrixSdkError::Sync)
                .and_then(|value| MatrixUserId::parse(value).map_err(|_| MatrixSdkError::Sync))
        })
        .collect()
}

fn system_time_ms() -> Result<u64, MatrixSdkError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| MatrixSdkError::Sync)?
        .as_millis();
    u64::try_from(millis).map_err(|_| MatrixSdkError::Sync)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_status_classification_bounds_permanent_and_transient_errors() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<MatrixSdkClient>();
        assert_eq!(classify_http_status(403), MatrixTransportError::Permanent);
        assert_eq!(classify_http_status(404), MatrixTransportError::Permanent);
        assert_eq!(classify_http_status(429), MatrixTransportError::Retryable);
        assert_eq!(classify_http_status(500), MatrixTransportError::Retryable);
        assert_eq!(classify_http_status(503), MatrixTransportError::Retryable);
    }

    #[test]
    fn ingress_accepts_only_text_message_content() {
        assert!(is_text_message(&RoomMessageEventContent::text_plain(
            "hello"
        )));
        assert!(!is_text_message(&RoomMessageEventContent::notice_plain(
            "notice"
        )));
    }
}
