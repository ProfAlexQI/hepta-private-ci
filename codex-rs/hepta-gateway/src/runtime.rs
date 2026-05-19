use crate::{
    GatewayEnvelope, GatewayPluginResolutionSnapshot, GatewayRouteIntegritySnapshot,
    GatewaySurface, GatewayTransport,
};
use hepta_core::channels::InboundMessage;
use std::{
    collections::VecDeque,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayDeliveryState {
    Accepted,
    Delivered,
    Retrying,
    DeadLettered,
}

impl GatewayDeliveryState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Delivered => "delivered",
            Self::Retrying => "retrying",
            Self::DeadLettered => "dead_lettered",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayAdapterDescriptor {
    pub adapter_id: String,
    pub surface_id: String,
    pub transport: GatewayTransport,
    pub delivery_status_supported: bool,
    pub retry_supported: bool,
    pub shutdown_notice_supported: bool,
    pub policy_overlay_id: String,
}

impl GatewayAdapterDescriptor {
    pub fn new(
        adapter_id: impl Into<String>,
        surface_id: impl Into<String>,
        transport: GatewayTransport,
        policy_overlay_id: impl Into<String>,
    ) -> Self {
        Self {
            adapter_id: adapter_id.into().trim().to_string(),
            surface_id: surface_id.into().trim().to_ascii_lowercase(),
            transport,
            delivery_status_supported: true,
            retry_supported: true,
            shutdown_notice_supported: true,
            policy_overlay_id: policy_overlay_id.into().trim().to_string(),
        }
    }

    pub fn matches(&self, surface_id: &str, transport: GatewayTransport) -> bool {
        self.transport == transport
            && (self.surface_id == "*" || self.surface_id == surface_id.trim().to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDeliveryRecord {
    pub receipt_id: String,
    pub surface_id: String,
    pub session_key: String,
    pub transport_key: String,
    pub adapter_id: Option<String>,
    pub plugin_id: Option<String>,
    pub state: GatewayDeliveryState,
    pub attempt_count: usize,
    pub normalized_payload_digest: String,
    pub detail: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GatewayDeliveryLedger {
    records: Vec<GatewayDeliveryRecord>,
}

impl GatewayDeliveryLedger {
    pub fn from_records(records: Vec<GatewayDeliveryRecord>) -> Self {
        Self { records }
    }

    pub fn append(&mut self, record: GatewayDeliveryRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[GatewayDeliveryRecord] {
        &self.records
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn delivered_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.state == GatewayDeliveryState::Delivered)
            .count()
    }

    pub fn retrying_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.state == GatewayDeliveryState::Retrying)
            .count()
    }

    pub fn dead_letter_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.state == GatewayDeliveryState::DeadLettered)
            .count()
    }

    pub fn last(&self) -> Option<&GatewayDeliveryRecord> {
        self.records.last()
    }

    pub fn replay_from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path)?;
        let mut records = Vec::new();
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            records.push(decode_ledger_record(line)?);
        }
        Ok(Self::from_records(records))
    }

    pub fn append_record_to_path(
        path: impl AsRef<Path>,
        record: &GatewayDeliveryRecord,
    ) -> io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", encode_ledger_record(record))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayAdapterSendResult {
    pub state: GatewayDeliveryState,
    pub attempt_count: usize,
    pub provider_receipt_fragment: String,
}

pub trait GatewayAdapter: Send + Sync {
    fn descriptor(&self) -> &GatewayAdapterDescriptor;
    fn send(&self, resolution: &GatewayPluginResolutionSnapshot) -> GatewayAdapterSendResult;
    fn shutdown_notice(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDeterministicAdapter {
    descriptor: GatewayAdapterDescriptor,
}

impl GatewayDeterministicAdapter {
    pub fn new(descriptor: GatewayAdapterDescriptor) -> Self {
        Self { descriptor }
    }
}

impl GatewayAdapter for GatewayDeterministicAdapter {
    fn descriptor(&self) -> &GatewayAdapterDescriptor {
        &self.descriptor
    }

    fn send(&self, resolution: &GatewayPluginResolutionSnapshot) -> GatewayAdapterSendResult {
        GatewayAdapterSendResult {
            state: GatewayDeliveryState::Delivered,
            attempt_count: 1,
            provider_receipt_fragment: stable_digest(&[
                &self.descriptor.adapter_id,
                &resolution.session_key,
                &resolution.normalized_text,
            ]),
        }
    }

    fn shutdown_notice(&self) -> String {
        format!("adapter {} shutdown-ready", self.descriptor.adapter_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDispatchOutcome {
    pub ready: bool,
    pub delivered: bool,
    pub receipt_id: String,
    pub adapter_id: Option<String>,
    pub plugin_id: Option<String>,
    pub delivery_state: GatewayDeliveryState,
    pub ledger_index: usize,
    pub blockers: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRuntimeReadinessReport {
    pub adapter_count: usize,
    pub supported_transport_count: usize,
    pub cli_transport_ready: bool,
    pub webhook_transport_ready: bool,
    pub queue_transport_ready: bool,
    pub delivery_ledger_ready: bool,
    pub retry_dead_letter_ready: bool,
    pub dispatch_execution_ready: bool,
    pub all_gateway_runtime_lanes_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GatewayRetryPolicy {
    pub max_attempts: usize,
    pub initial_backoff_ms: u64,
}

impl Default for GatewayRetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff_ms: 250,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRuntimeStatusReport {
    pub running: bool,
    pub adapter_count: usize,
    pub queue_depth: usize,
    pub ledger_path: Option<String>,
    pub delivered_count: usize,
    pub retrying_count: usize,
    pub dead_letter_count: usize,
    pub last_receipt_id: Option<String>,
    pub runtime_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayFrameBridgeReport {
    pub channel: String,
    pub sender: String,
    pub session_hint_present: bool,
    pub attachment_count: usize,
    pub transport_key: String,
    pub normalized_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GatewayQueuedDispatch {
    envelope: GatewayEnvelope,
    resolution: GatewayPluginResolutionSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDeterministicDispatcher {
    adapters: Vec<GatewayDeterministicAdapter>,
    ledger: GatewayDeliveryLedger,
}

impl GatewayDeterministicDispatcher {
    pub fn new(adapters: Vec<GatewayAdapterDescriptor>) -> Self {
        Self {
            adapters: adapters
                .into_iter()
                .map(GatewayDeterministicAdapter::new)
                .collect(),
            ledger: GatewayDeliveryLedger::default(),
        }
    }

    pub fn with_ledger(
        adapters: Vec<GatewayAdapterDescriptor>,
        ledger: GatewayDeliveryLedger,
    ) -> Self {
        Self {
            adapters: adapters
                .into_iter()
                .map(GatewayDeterministicAdapter::new)
                .collect(),
            ledger,
        }
    }

    pub fn adapter_descriptors(&self) -> Vec<&GatewayAdapterDescriptor> {
        self.adapters
            .iter()
            .map(GatewayAdapter::descriptor)
            .collect()
    }

    pub fn ledger(&self) -> &GatewayDeliveryLedger {
        &self.ledger
    }

    pub fn dispatch(
        &mut self,
        surface: &GatewaySurface,
        envelope: &GatewayEnvelope,
        resolution: &GatewayPluginResolutionSnapshot,
    ) -> GatewayDispatchOutcome {
        let integrity =
            GatewayRouteIntegritySnapshot::from_resolution(surface, envelope, resolution);
        let readiness = integrity.dispatch_readiness_report(resolution);
        let adapter = self.select_adapter(&resolution.surface_id, envelope.transport);
        let plugin_id = resolution
            .candidates
            .first()
            .map(|candidate| candidate.plugin_id.clone());
        let mut blockers = readiness.blockers.clone();
        if adapter.is_none() {
            blockers.push(format!(
                "no gateway adapter registered for surface {} transport {}",
                resolution.surface_id, resolution.transport_key
            ));
        }
        let ready = readiness.ready && adapter.is_some();
        let adapter_send = if ready {
            adapter.map(|adapter| adapter.send(resolution))
        } else {
            None
        };
        let delivery_state = adapter_send
            .as_ref()
            .map(|result| result.state)
            .unwrap_or(GatewayDeliveryState::DeadLettered);
        let adapter_id = adapter.map(|adapter| adapter.descriptor().adapter_id.clone());
        let detail = if ready {
            "delivered through deterministic gateway adapter".to_string()
        } else {
            format!(
                "fail-closed before live adapter side effect: {}",
                blockers.join("; ")
            )
        };
        let receipt_id = gateway_receipt_id(
            &resolution.surface_id,
            &resolution.session_key,
            &resolution.transport_key,
            plugin_id.as_deref().unwrap_or("no-plugin"),
            &resolution.normalized_text,
            self.ledger.len(),
        );
        let record = GatewayDeliveryRecord {
            receipt_id: receipt_id.clone(),
            surface_id: resolution.surface_id.clone(),
            session_key: resolution.session_key.clone(),
            transport_key: resolution.transport_key.clone(),
            adapter_id: adapter_id.clone(),
            plugin_id: plugin_id.clone(),
            state: delivery_state,
            attempt_count: adapter_send
                .as_ref()
                .map(|result| result.attempt_count)
                .unwrap_or(0),
            normalized_payload_digest: stable_digest(&[&resolution.normalized_text]),
            detail,
        };
        self.ledger.append(record);
        GatewayDispatchOutcome {
            ready,
            delivered: ready,
            receipt_id,
            adapter_id,
            plugin_id,
            delivery_state,
            ledger_index: self.ledger.len() - 1,
            blockers,
            warnings: readiness.warnings,
        }
    }

    pub fn dispatch_with_retry(
        &mut self,
        surface: &GatewaySurface,
        envelope: &GatewayEnvelope,
        resolution: &GatewayPluginResolutionSnapshot,
        policy: GatewayRetryPolicy,
    ) -> GatewayDispatchOutcome {
        if resolution.has_candidates() {
            return self.dispatch(surface, envelope, resolution);
        }

        let mut last_receipt_id = String::new();
        for attempt in 1..=policy.max_attempts {
            let state = if attempt < policy.max_attempts {
                GatewayDeliveryState::Retrying
            } else {
                GatewayDeliveryState::DeadLettered
            };
            let receipt_id = gateway_receipt_id(
                &resolution.surface_id,
                &resolution.session_key,
                &resolution.transport_key,
                "retry-no-plugin",
                &resolution.normalized_text,
                self.ledger.len(),
            );
            last_receipt_id = receipt_id.clone();
            self.ledger.append(GatewayDeliveryRecord {
                receipt_id,
                surface_id: resolution.surface_id.clone(),
                session_key: resolution.session_key.clone(),
                transport_key: resolution.transport_key.clone(),
                adapter_id: self
                    .select_adapter(&resolution.surface_id, envelope.transport)
                    .map(|adapter| adapter.descriptor().adapter_id.clone()),
                plugin_id: None,
                state,
                attempt_count: attempt,
                normalized_payload_digest: stable_digest(&[&resolution.normalized_text]),
                detail: if state == GatewayDeliveryState::Retrying {
                    format!(
                        "retry scheduled after {}ms because plugin resolution is empty",
                        policy.initial_backoff_ms * attempt as u64
                    )
                } else {
                    "dead-lettered after deterministic retry budget was exhausted".into()
                },
            });
        }

        GatewayDispatchOutcome {
            ready: false,
            delivered: false,
            receipt_id: last_receipt_id,
            adapter_id: self
                .select_adapter(&resolution.surface_id, envelope.transport)
                .map(|adapter| adapter.descriptor().adapter_id.clone()),
            plugin_id: None,
            delivery_state: GatewayDeliveryState::DeadLettered,
            ledger_index: self.ledger.len().saturating_sub(1),
            blockers: vec!["no plugin candidate resolved after retry budget".into()],
            warnings: Vec::new(),
        }
    }

    pub fn readiness_report(&self) -> GatewayRuntimeReadinessReport {
        let cli_transport_ready = self.has_transport(GatewayTransport::Cli);
        let webhook_transport_ready = self.has_transport(GatewayTransport::Webhook);
        let queue_transport_ready = self.has_transport(GatewayTransport::Queue);
        let retry_dead_letter_ready = self
            .adapters
            .iter()
            .map(GatewayAdapter::descriptor)
            .all(|adapter| adapter.retry_supported && adapter.delivery_status_supported);
        let delivery_ledger_ready = true;
        let dispatch_execution_ready =
            cli_transport_ready && webhook_transport_ready && queue_transport_ready;
        let supported_transport_count = [
            cli_transport_ready,
            webhook_transport_ready,
            queue_transport_ready,
        ]
        .into_iter()
        .filter(|ready| *ready)
        .count();
        let all_gateway_runtime_lanes_ready = dispatch_execution_ready
            && delivery_ledger_ready
            && retry_dead_letter_ready
            && self
                .adapters
                .iter()
                .map(GatewayAdapter::descriptor)
                .all(|adapter| {
                    !adapter.adapter_id.is_empty() && !adapter.policy_overlay_id.is_empty()
                });
        GatewayRuntimeReadinessReport {
            adapter_count: self.adapters.len(),
            supported_transport_count,
            cli_transport_ready,
            webhook_transport_ready,
            queue_transport_ready,
            delivery_ledger_ready,
            retry_dead_letter_ready,
            dispatch_execution_ready,
            all_gateway_runtime_lanes_ready,
        }
    }

    fn select_adapter(
        &self,
        surface_id: &str,
        transport: GatewayTransport,
    ) -> Option<&GatewayDeterministicAdapter> {
        self.adapters
            .iter()
            .find(|adapter| {
                adapter.descriptor().matches(surface_id, transport)
                    && adapter.descriptor().surface_id != "*"
            })
            .or_else(|| {
                self.adapters
                    .iter()
                    .find(|adapter| adapter.descriptor().matches(surface_id, transport))
            })
    }

    fn has_transport(&self, transport: GatewayTransport) -> bool {
        self.adapters
            .iter()
            .map(GatewayAdapter::descriptor)
            .any(|adapter| adapter.transport == transport && adapter.delivery_status_supported)
    }
}

impl Default for GatewayDeterministicDispatcher {
    fn default() -> Self {
        Self::new(default_gateway_adapters())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayRuntime {
    dispatcher: GatewayDeterministicDispatcher,
    queue: VecDeque<GatewayQueuedDispatch>,
    running: bool,
    ledger_path: Option<PathBuf>,
}

impl GatewayRuntime {
    pub fn new(dispatcher: GatewayDeterministicDispatcher) -> Self {
        Self {
            dispatcher,
            queue: VecDeque::new(),
            running: false,
            ledger_path: None,
        }
    }

    pub fn with_persistent_ledger(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        let ledger = GatewayDeliveryLedger::replay_from_path(&path)?;
        Ok(Self {
            dispatcher: GatewayDeterministicDispatcher::with_ledger(
                default_gateway_adapters(),
                ledger,
            ),
            queue: VecDeque::new(),
            running: false,
            ledger_path: Some(path),
        })
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn ingest(
        &mut self,
        envelope: GatewayEnvelope,
        resolution: GatewayPluginResolutionSnapshot,
    ) -> usize {
        self.queue.push_back(GatewayQueuedDispatch {
            envelope,
            resolution,
        });
        self.queue.len()
    }

    pub fn drain(&mut self, surface: &GatewaySurface) -> io::Result<Vec<GatewayDispatchOutcome>> {
        let mut outcomes = Vec::new();
        while let Some(item) = self.queue.pop_front() {
            let outcome = self
                .dispatcher
                .dispatch(surface, &item.envelope, &item.resolution);
            self.persist_last_record()?;
            outcomes.push(outcome);
        }
        Ok(outcomes)
    }

    pub fn dispatch_with_retry(
        &mut self,
        surface: &GatewaySurface,
        envelope: &GatewayEnvelope,
        resolution: &GatewayPluginResolutionSnapshot,
        policy: GatewayRetryPolicy,
    ) -> io::Result<GatewayDispatchOutcome> {
        let before_len = self.dispatcher.ledger().len();
        let outcome = self
            .dispatcher
            .dispatch_with_retry(surface, envelope, resolution, policy);
        if let Some(path) = &self.ledger_path {
            for record in &self.dispatcher.ledger().records()[before_len..] {
                GatewayDeliveryLedger::append_record_to_path(path, record)?;
            }
        }
        Ok(outcome)
    }

    pub fn status_report(&self) -> GatewayRuntimeStatusReport {
        GatewayRuntimeStatusReport {
            running: self.running,
            adapter_count: self.dispatcher.adapters.len(),
            queue_depth: self.queue.len(),
            ledger_path: self
                .ledger_path
                .as_ref()
                .map(|path| path.display().to_string()),
            delivered_count: self.dispatcher.ledger().delivered_count(),
            retrying_count: self.dispatcher.ledger().retrying_count(),
            dead_letter_count: self.dispatcher.ledger().dead_letter_count(),
            last_receipt_id: self
                .dispatcher
                .ledger()
                .last()
                .map(|record| record.receipt_id.clone()),
            runtime_ready: self.running
                && self
                    .dispatcher
                    .readiness_report()
                    .all_gateway_runtime_lanes_ready,
        }
    }

    pub fn ledger(&self) -> &GatewayDeliveryLedger {
        self.dispatcher.ledger()
    }

    fn persist_last_record(&self) -> io::Result<()> {
        if let (Some(path), Some(record)) = (&self.ledger_path, self.dispatcher.ledger().last()) {
            GatewayDeliveryLedger::append_record_to_path(path, record)?;
        }
        Ok(())
    }
}

impl Default for GatewayRuntime {
    fn default() -> Self {
        Self::new(GatewayDeterministicDispatcher::default())
    }
}

pub fn default_gateway_adapters() -> Vec<GatewayAdapterDescriptor> {
    vec![
        GatewayAdapterDescriptor::new(
            "local-cli-adapter",
            "*",
            GatewayTransport::Cli,
            "local-cli-policy",
        ),
        GatewayAdapterDescriptor::new(
            "webhook-ingress-adapter",
            "*",
            GatewayTransport::Webhook,
            "webhook-policy",
        ),
        GatewayAdapterDescriptor::new(
            "durable-queue-adapter",
            "*",
            GatewayTransport::Queue,
            "queue-policy",
        ),
    ]
}

pub fn envelope_from_inbound_message(
    message: &InboundMessage,
    transport: GatewayTransport,
) -> GatewayEnvelope {
    let mut envelope = GatewayEnvelope::new(
        &message.envelope.channel,
        &message.envelope.sender,
        transport,
        &message.text,
    );
    if let Some(session_id) = &message.envelope.session_id {
        envelope = envelope.with_session_hint(session_id.0.clone());
    }
    envelope
}

pub fn frame_bridge_report(
    message: &InboundMessage,
    transport: GatewayTransport,
) -> GatewayFrameBridgeReport {
    let envelope = envelope_from_inbound_message(message, transport);
    GatewayFrameBridgeReport {
        channel: message.envelope.channel.clone(),
        sender: message.envelope.sender.clone(),
        session_hint_present: envelope.session_hint.is_some(),
        attachment_count: message.attachments.len(),
        transport_key: transport_key_for_report(transport).into(),
        normalized_text: envelope.payload_text.trim().into(),
    }
}

fn gateway_receipt_id(
    surface_id: &str,
    session_key: &str,
    transport_key: &str,
    plugin_id: &str,
    normalized_text: &str,
    ledger_len: usize,
) -> String {
    format!(
        "hepta-gateway-receipt:{}",
        stable_digest(&[
            surface_id,
            session_key,
            transport_key,
            plugin_id,
            normalized_text,
            &ledger_len.to_string(),
        ])
    )
}

fn stable_digest(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

fn encode_ledger_record(record: &GatewayDeliveryRecord) -> String {
    [
        record.receipt_id.as_str(),
        record.surface_id.as_str(),
        record.session_key.as_str(),
        record.transport_key.as_str(),
        record.adapter_id.as_deref().unwrap_or(""),
        record.plugin_id.as_deref().unwrap_or(""),
        record.state.as_str(),
        &record.attempt_count.to_string(),
        record.normalized_payload_digest.as_str(),
        record.detail.as_str(),
    ]
    .into_iter()
    .map(escape_ledger_field)
    .collect::<Vec<_>>()
    .join("\t")
}

fn decode_ledger_record(line: &str) -> io::Result<GatewayDeliveryRecord> {
    let parts = line
        .split('\t')
        .map(unescape_ledger_field)
        .collect::<Vec<_>>();
    if parts.len() != 10 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected 10 ledger fields, got {}", parts.len()),
        ));
    }
    let state = match parts[6].as_str() {
        "accepted" => GatewayDeliveryState::Accepted,
        "delivered" => GatewayDeliveryState::Delivered,
        "retrying" => GatewayDeliveryState::Retrying,
        "dead_lettered" => GatewayDeliveryState::DeadLettered,
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown gateway delivery state {other}"),
            ));
        }
    };
    Ok(GatewayDeliveryRecord {
        receipt_id: parts[0].clone(),
        surface_id: parts[1].clone(),
        session_key: parts[2].clone(),
        transport_key: parts[3].clone(),
        adapter_id: (!parts[4].is_empty()).then(|| parts[4].clone()),
        plugin_id: (!parts[5].is_empty()).then(|| parts[5].clone()),
        state,
        attempt_count: parts[7].parse().map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid attempt count: {err}"),
            )
        })?,
        normalized_payload_digest: parts[8].clone(),
        detail: parts[9].clone(),
    })
}

fn escape_ledger_field(field: &str) -> String {
    field
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape_ledger_field(field: &str) -> String {
    let mut out = String::new();
    let mut chars = field.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn transport_key_for_report(transport: GatewayTransport) -> &'static str {
    match transport {
        GatewayTransport::Cli => "cli",
        GatewayTransport::Webhook => "webhook",
        GatewayTransport::Queue => "queue",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GatewayDeliveryLedger, GatewayDeliveryRecord, GatewayDeliveryState,
        GatewayDeterministicDispatcher, GatewayRetryPolicy, GatewayRuntime,
        envelope_from_inbound_message, frame_bridge_report,
    };
    use crate::{
        GatewayEnvelope, GatewayPluginHandoffDraft, GatewayPluginResolutionSnapshot,
        GatewayResolvedPluginCandidate, GatewaySurface, GatewayTransport,
    };
    use hepta_core::channels::{AttachmentRef, InboundMessage, MessageEnvelope};
    use hepta_core::runtime_types::SessionId;
    use std::{env, fs, path::PathBuf};

    fn resolved_queue_snapshot() -> (
        GatewaySurface,
        GatewayEnvelope,
        GatewayPluginResolutionSnapshot,
    ) {
        let surface = GatewaySurface;
        let envelope =
            GatewayEnvelope::new("hepta", "user-7", GatewayTransport::Queue, "/status --json");
        let draft = GatewayPluginHandoffDraft::from_route(&surface.route_plan(&envelope));
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft)
            .with_candidates([
                GatewayResolvedPluginCandidate::new(
                    "status-plugin",
                    "surface=hepta|transport=queue|command=/status",
                    2,
                ),
                GatewayResolvedPluginCandidate::new(
                    "queue-fallback",
                    "surface=hepta|transport=queue",
                    1,
                ),
                GatewayResolvedPluginCandidate::new("surface-fallback", "surface=hepta", 0),
            ]);
        (surface, envelope, resolution)
    }

    fn unique_ledger_path(label: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "hepta-gateway-{label}-{}-{}.ledger",
            std::process::id(),
            stable_test_nonce()
        ))
    }

    fn stable_test_nonce() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos()
    }

    #[test]
    fn dispatcher_covers_cli_webhook_and_queue_transports() {
        let dispatcher = GatewayDeterministicDispatcher::default();
        let report = dispatcher.readiness_report();

        assert_eq!(report.supported_transport_count, 3);
        assert!(report.cli_transport_ready);
        assert!(report.webhook_transport_ready);
        assert!(report.queue_transport_ready);
        assert!(report.dispatch_execution_ready);
        assert!(report.delivery_ledger_ready);
        assert!(report.retry_dead_letter_ready);
        assert!(report.all_gateway_runtime_lanes_ready);
    }

    #[test]
    fn queue_dispatch_writes_delivery_receipt_to_ledger() {
        let (surface, envelope, resolution) = resolved_queue_snapshot();
        let mut dispatcher = GatewayDeterministicDispatcher::default();

        let outcome = dispatcher.dispatch(&surface, &envelope, &resolution);

        assert!(outcome.ready);
        assert!(outcome.delivered);
        assert_eq!(outcome.delivery_state, GatewayDeliveryState::Delivered);
        assert_eq!(outcome.adapter_id.as_deref(), Some("durable-queue-adapter"));
        assert_eq!(outcome.plugin_id.as_deref(), Some("status-plugin"));
        assert!(outcome.receipt_id.starts_with("hepta-gateway-receipt:"));
        assert_eq!(dispatcher.ledger().len(), 1);
        assert_eq!(dispatcher.ledger().delivered_count(), 1);
        let record = dispatcher
            .ledger()
            .last()
            .expect("ledger should contain delivery record");
        assert_eq!(record.session_key, "hepta:user:user-7");
        assert_eq!(record.transport_key, "queue");
        assert_eq!(record.state.as_str(), "delivered");
        assert_eq!(record.attempt_count, 1);
    }

    #[test]
    fn dispatch_fail_closes_into_dead_letter_without_plugin_candidate() {
        let surface = GatewaySurface;
        let envelope =
            GatewayEnvelope::new("telegram", "user-9", GatewayTransport::Webhook, "/missing");
        let draft = GatewayPluginHandoffDraft::from_route(&surface.route_plan(&envelope));
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft);
        let mut dispatcher = GatewayDeterministicDispatcher::default();

        let outcome = dispatcher.dispatch(&surface, &envelope, &resolution);

        assert!(!outcome.ready);
        assert!(!outcome.delivered);
        assert_eq!(outcome.delivery_state, GatewayDeliveryState::DeadLettered);
        assert!(
            outcome
                .blockers
                .iter()
                .any(|blocker| blocker.contains("no plugin candidate"))
        );
        assert_eq!(dispatcher.ledger().dead_letter_count(), 1);
        let record = dispatcher
            .ledger()
            .last()
            .expect("dead letter record should be present");
        assert_eq!(record.state, GatewayDeliveryState::DeadLettered);
        assert!(record.detail.contains("fail-closed"));
    }

    #[test]
    fn persistent_ledger_replays_delivered_and_dead_lettered_records() {
        let path = unique_ledger_path("replay");
        let mut delivered = GatewayDeliveryRecord {
            receipt_id: "receipt-1".into(),
            surface_id: "hepta".into(),
            session_key: "session-1".into(),
            transport_key: "queue".into(),
            adapter_id: Some("durable-queue-adapter".into()),
            plugin_id: Some("status-plugin".into()),
            state: GatewayDeliveryState::Delivered,
            attempt_count: 1,
            normalized_payload_digest: "digest-1".into(),
            detail: "delivered".into(),
        };
        GatewayDeliveryLedger::append_record_to_path(&path, &delivered)
            .expect("delivered ledger append should succeed");
        delivered.receipt_id = "receipt-2".into();
        delivered.plugin_id = None;
        delivered.state = GatewayDeliveryState::DeadLettered;
        delivered.detail = "dead-lettered".into();
        GatewayDeliveryLedger::append_record_to_path(&path, &delivered)
            .expect("dead-letter ledger append should succeed");

        let replayed =
            GatewayDeliveryLedger::replay_from_path(&path).expect("ledger replay should succeed");

        assert_eq!(replayed.len(), 2);
        assert_eq!(replayed.delivered_count(), 1);
        assert_eq!(replayed.dead_letter_count(), 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn gateway_runtime_restarts_without_duplicate_receipts() {
        let path = unique_ledger_path("restart");
        let (surface, envelope, resolution) = resolved_queue_snapshot();
        let first_receipt = {
            let mut runtime = GatewayRuntime::with_persistent_ledger(&path)
                .expect("runtime should create persistent ledger");
            runtime.start();
            runtime.ingest(envelope.clone(), resolution.clone());
            let outcomes = runtime
                .drain(&surface)
                .expect("drain should persist dispatch");
            assert_eq!(outcomes.len(), 1);
            outcomes[0].receipt_id.clone()
        };

        let mut restarted = GatewayRuntime::with_persistent_ledger(&path)
            .expect("runtime should replay existing ledger");
        restarted.start();
        assert_eq!(restarted.ledger().len(), 1);
        assert_eq!(
            restarted.status_report().last_receipt_id.as_deref(),
            Some(first_receipt.as_str())
        );
        restarted.ingest(envelope, resolution);
        let outcomes = restarted
            .drain(&surface)
            .expect("second drain should persist dispatch");
        assert_eq!(outcomes.len(), 1);
        assert_ne!(outcomes[0].receipt_id, first_receipt);
        assert_eq!(restarted.ledger().len(), 2);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn adapter_trait_dispatch_executes_selected_adapter_and_records_receipt() {
        let (surface, envelope, resolution) = resolved_queue_snapshot();
        let mut dispatcher = GatewayDeterministicDispatcher::default();

        let outcome = dispatcher.dispatch(&surface, &envelope, &resolution);

        assert_eq!(outcome.adapter_id.as_deref(), Some("durable-queue-adapter"));
        let record = dispatcher.ledger().last().expect("record should exist");
        assert_eq!(record.adapter_id.as_deref(), Some("durable-queue-adapter"));
        assert_eq!(record.plugin_id.as_deref(), Some("status-plugin"));
        assert_eq!(record.state, GatewayDeliveryState::Delivered);
        assert_eq!(record.attempt_count, 1);
    }

    #[test]
    fn retry_backoff_dead_letters_after_max_attempts() {
        let surface = GatewaySurface;
        let envelope =
            GatewayEnvelope::new("telegram", "user-9", GatewayTransport::Webhook, "/missing");
        let draft = GatewayPluginHandoffDraft::from_route(&surface.route_plan(&envelope));
        let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft);
        let mut runtime = GatewayRuntime::default();
        runtime.start();

        let outcome = runtime
            .dispatch_with_retry(
                &surface,
                &envelope,
                &resolution,
                GatewayRetryPolicy {
                    max_attempts: 3,
                    initial_backoff_ms: 10,
                },
            )
            .expect("retry dispatch should complete deterministically");

        assert_eq!(outcome.delivery_state, GatewayDeliveryState::DeadLettered);
        assert_eq!(runtime.ledger().retrying_count(), 2);
        assert_eq!(runtime.ledger().dead_letter_count(), 1);
        assert!(
            runtime
                .ledger()
                .records()
                .iter()
                .any(|record| record.detail.contains("retry scheduled"))
        );
    }

    #[test]
    fn gateway_frame_bridge_preserves_channel_sender_session_and_attachments() {
        let message = InboundMessage {
            envelope: MessageEnvelope {
                channel: "telegram".into(),
                sender: "user-9".into(),
                session_id: Some(SessionId("session-telegram-9".into())),
                correlation_id: None,
                agent_id: None,
            },
            text: "  /status --json  ".into(),
            attachments: vec![AttachmentRef {
                name: "image.png".into(),
                mime_type: "image/png".into(),
                uri: "media://redacted/image.png".into(),
            }],
        };

        let envelope = envelope_from_inbound_message(&message, GatewayTransport::Webhook);
        let report = frame_bridge_report(&message, GatewayTransport::Webhook);

        assert_eq!(envelope.surface_id, "telegram");
        assert_eq!(envelope.user_id, "user-9");
        assert_eq!(envelope.session_hint.as_deref(), Some("session-telegram-9"));
        assert_eq!(report.attachment_count, 1);
        assert_eq!(report.normalized_text, "/status --json");
        assert_eq!(report.transport_key, "webhook");
        assert!(report.session_hint_present);
    }
}
