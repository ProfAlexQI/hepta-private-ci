use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_client::RemoteAppServerConnectArgs;
use codex_app_server_client::RemoteAppServerEndpoint;
use codex_app_server_protocol::ClientRequest;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::ThreadQueueAddParams;
use codex_app_server_protocol::ThreadQueueAddResponse;
use codex_app_server_protocol::UserInput;
use codex_hepta_automation::AutomationAdmission;
use codex_hepta_automation::AutomationError;
use codex_hepta_automation::AutomationFuture;
use codex_hepta_automation::AutomationQueueReceipt;
use codex_hepta_automation::AutomationScheduler;
use codex_hepta_automation::AutomationStore;
use codex_hepta_automation::AutomationTurnQueue;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio_util::sync::CancellationToken;

use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;

const AUTOMATION_TICK_INTERVAL: Duration = Duration::from_millis(250);
const AUTOMATION_LEASE_DURATION: Duration = Duration::from_secs(30);
const AUTOMATION_DISPATCH_TIMEOUT: Duration = Duration::from_secs(5);
const APP_SERVER_COMMAND_CAPACITY: usize = 8;
const APP_SERVER_EVENT_CAPACITY: usize = 16;

pub(crate) struct AgentdAutomationQueue {
    state: Arc<AgentdState>,
    identity: AgentdIdentity,
}

impl AgentdAutomationQueue {
    pub(crate) fn new(state: Arc<AgentdState>, identity: AgentdIdentity) -> Self {
        Self { state, identity }
    }

    async fn enqueue_inner(
        &self,
        admission: AutomationAdmission,
    ) -> Result<AutomationQueueReceipt, AgentdError> {
        if admission.agent_id != self.identity.agent_id
            || !self.state.automation_admission_ready()?
        {
            return Err(AgentdError::GenerationFenced(
                "automation admission does not belong to a ready Agent generation".to_string(),
            ));
        }
        let socket_path = AbsolutePathBuf::from_absolute_path(&self.identity.app_server_socket)?;
        let client = RemoteAppServerClient::connect_with_bounded_events(
            RemoteAppServerConnectArgs {
                endpoint: RemoteAppServerEndpoint::UnixSocket { socket_path },
                client_name: "hepta-agentd-automation".to_string(),
                client_version: env!("CARGO_PKG_VERSION").to_string(),
                experimental_api: true,
                mcp_server_openai_form_elicitation: false,
                opt_out_notification_methods: Vec::new(),
                channel_capacity: APP_SERVER_COMMAND_CAPACITY,
            },
            APP_SERVER_EVENT_CAPACITY,
        )
        .await?;
        let expected_home = self.identity.home_root.to_string_lossy();
        if client.codex_home() != Some(expected_home.as_ref()) {
            let _ = client.shutdown().await;
            return Err(AgentdError::GenerationFenced(
                "automation App Server home differs from the owning Agent home".to_string(),
            ));
        }
        let request = automation_queue_request(&admission);
        let response: ThreadQueueAddResponse = client
            .request_handle()
            .request_typed(request)
            .await
            .map_err(|_| {
                AgentdError::Protocol("App Server rejected automation thread/queue/add".to_string())
            })?;
        let _ = client.shutdown().await;
        self.state.refresh_generation()?;
        if !self.state.automation_admission_ready()? {
            return Err(AgentdError::GenerationFenced(
                "Agent generation changed during automation admission".to_string(),
            ));
        }
        if response.queued_submission.client_user_message_id != admission.client_user_message_id
            || response.queued_submission.id.is_empty()
        {
            return Err(AgentdError::Protocol(
                "App Server returned a mismatched automation queue receipt".to_string(),
            ));
        }
        Ok(AutomationQueueReceipt {
            queued_submission_id: response.queued_submission.id,
            client_user_message_id: response.queued_submission.client_user_message_id,
        })
    }
}

impl AutomationTurnQueue for AgentdAutomationQueue {
    fn enqueue(
        &self,
        admission: AutomationAdmission,
    ) -> AutomationFuture<'_, AutomationQueueReceipt> {
        Box::pin(async move {
            self.enqueue_inner(admission)
                .await
                .map_err(|_| AutomationError::Dispatch)
        })
    }
}

pub(crate) async fn run_automation_scheduler(
    store: AutomationStore,
    state: Arc<AgentdState>,
    identity: AgentdIdentity,
    cancellation: CancellationToken,
) -> Result<(), AgentdError> {
    store
        .recover_stale_generation(identity.spawn_generation)
        .await?;
    let queue = Arc::new(AgentdAutomationQueue::new(
        Arc::clone(&state),
        identity.clone(),
    ));
    let scheduler = AutomationScheduler::new(
        store,
        queue,
        identity.spawn_generation,
        AUTOMATION_LEASE_DURATION,
        AUTOMATION_DISPATCH_TIMEOUT,
    )?;
    loop {
        tokio::select! {
            _ = cancellation.cancelled() => return Ok(()),
            _ = tokio::time::sleep(AUTOMATION_TICK_INTERVAL) => {}
        }
        if !state.automation_admission_ready()? {
            continue;
        }
        let now_ms = unix_time_ms()?;
        tokio::select! {
            _ = cancellation.cancelled() => return Ok(()),
            result = scheduler.tick(now_ms) => {
                result?;
            }
        }
    }
}

fn automation_queue_request(admission: &AutomationAdmission) -> ClientRequest {
    ClientRequest::ThreadQueueAdd {
        request_id: RequestId::Integer(1),
        params: ThreadQueueAddParams {
            thread_id: admission.thread_id.clone(),
            input: vec![UserInput::Text {
                text: admission.prompt.clone(),
                text_elements: Vec::new(),
            }],
            client_user_message_id: admission.client_user_message_id.clone(),
        },
    }
}

fn unix_time_ms() -> Result<u64, AgentdError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AgentdError::Protocol("system clock precedes Unix epoch".to_string()))?
        .as_millis();
    u64::try_from(millis)
        .map_err(|_| AgentdError::Protocol("system clock exceeds u64 milliseconds".to_string()))
}

#[cfg(test)]
mod tests {
    use codex_hepta_automation::AutomationTaskId;
    use codex_hepta_contracts::AgentId;

    use super::*;

    #[test]
    fn automation_has_only_normal_app_server_queue_admission() {
        let admission = AutomationAdmission {
            agent_id: AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("agent id"),
            task_id: AutomationTaskId::parse("019153a4-3088-7000-a56a-9b1964f75007")
                .expect("task id"),
            occurrence: 3,
            scheduled_for_ms: 44,
            thread_id: "019153a4-3088-7e03-a56a-9b1964f75ddd".to_string(),
            prompt: "run through governance".to_string(),
            client_user_message_id: "hepta.automation.test".to_string(),
        };
        let ClientRequest::ThreadQueueAdd { params, .. } = automation_queue_request(&admission)
        else {
            panic!("automation must only enter via thread/queue/add");
        };
        assert_eq!(params.thread_id, admission.thread_id);
        assert_eq!(
            params.client_user_message_id,
            admission.client_user_message_id
        );
        assert_eq!(
            params.input,
            vec![UserInput::Text {
                text: admission.prompt,
                text_elements: Vec::new(),
            }]
        );
    }
}
