use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

use crate::{
    current_unix_ms,
    delivery_queue::ReadbackEvidenceLedger,
    session_transcript::{
        SessionTranscriptAppendHandoffInput, SessionTranscriptAppendHandoffReport,
        SessionTranscriptStore,
    },
};

pub const DEFAULT_INBOUND_ROUTER_PATH: &str = ".hepta/inbound-router-v0.json";
pub const DEFAULT_INBOUND_ROUTER_ID: &str = "hepta-native-inbound-router";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InboundRouteStatus {
    Received,
    Duplicate,
    Routed,
    Suppressed,
}

impl InboundRouteStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Received => "received",
            Self::Duplicate => "duplicate",
            Self::Routed => "routed",
            Self::Suppressed => "suppressed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundEventInput {
    pub channel: String,
    pub source: String,
    pub chat_id: String,
    pub message_id: String,
    pub normalized_text: String,
    pub idempotency_key: String,
    pub direct_mention: bool,
    pub group_chat: bool,
    pub reply_to_message_id: Option<String>,
    pub topic_id: Option<String>,
    pub active_session_id: Option<String>,
    pub reset_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundRouterFile {
    pub version: u32,
    pub router_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default = "default_next_sequence")]
    pub next_sequence: u64,
    #[serde(default)]
    pub events: Vec<InboundEventRecord>,
    #[serde(default)]
    pub spool: Vec<InboundSpoolRecord>,
    #[serde(default)]
    pub session_handoffs: Vec<InboundSessionTranscriptHandoffRecord>,
    #[serde(default)]
    pub audit_events: Vec<InboundRouterAuditEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundEventRecord {
    pub event_id: String,
    #[serde(default)]
    pub sequence: u64,
    pub channel: String,
    pub source: String,
    pub chat_id: String,
    pub message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_session_id: Option<String>,
    #[serde(default)]
    pub reset_generation: u64,
    #[serde(default)]
    pub session_boundary_key: String,
    pub normalized_text_preview: String,
    pub idempotency_key: String,
    pub provenance_hash: String,
    pub status: InboundRouteStatus,
    pub route_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duplicate_of: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routed_session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundSpoolStatus {
    Pending,
    Routed,
    Suppressed,
}

impl InboundSpoolStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Routed => "routed",
            Self::Suppressed => "suppressed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundSpoolRecord {
    pub spool_id: String,
    pub event_id: String,
    #[serde(default)]
    pub sequence: u64,
    pub status: InboundSpoolStatus,
    pub channel: String,
    pub chat_id: String,
    pub message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_session_id: Option<String>,
    #[serde(default)]
    pub reset_generation: u64,
    pub route_kind: String,
    pub idempotency_key: String,
    pub normalized_text_preview: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    pub durable_local_spool: bool,
    pub external_download_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundRouterAuditEvent {
    pub audit_id: String,
    pub event_type: String,
    pub event_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundSessionTranscriptHandoffRecord {
    pub handoff_id: String,
    pub event_id: String,
    #[serde(default)]
    pub sequence: u64,
    pub session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(default)]
    pub session_boundary_key: String,
    pub transcript_handoff_id: String,
    pub transcript_turn_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub router_mutated_by_gate: bool,
    pub transcript_store_mutated_by_gate: bool,
    pub external_reply_performed_by_gate: bool,
    pub model_invoked_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InboundRouterReport {
    pub router_path: String,
    pub router: InboundRouterFile,
    pub received_count: usize,
    pub duplicate_count: usize,
    pub routed_count: usize,
    pub suppressed_count: usize,
    pub spool_pending_count: usize,
    pub spool_routed_count: usize,
    pub spool_suppressed_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InboundIngestReport {
    pub router_path: String,
    pub event: InboundEventRecord,
    pub duplicate_idempotency_key: bool,
    pub spooled: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InboundRouteReport {
    pub router_path: String,
    pub event_id: String,
    pub status: InboundRouteStatus,
    pub route_kind: String,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InboundSessionTranscriptHandoffInput {
    pub session_id: String,
    pub session_title: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub expected_topic_id: Option<String>,
    pub expected_reset_generation: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InboundSessionTranscriptHandoffReport {
    pub router_path: String,
    pub transcript_store_path: String,
    pub evidence_ledger_path: String,
    pub handoff: InboundSessionTranscriptHandoffRecord,
    pub transcript_handoff: SessionTranscriptAppendHandoffReport,
    pub duplicate_idempotency_key: bool,
    pub router_mutated_by_gate: bool,
    pub transcript_store_mutated_by_gate: bool,
    pub external_reply_performed_by_gate: bool,
    pub model_invoked_by_gate: bool,
    pub persisted: bool,
}

pub struct InboundRouterStore {
    path: PathBuf,
}

impl InboundRouterStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!("failed to resolve cwd for inbound-router: {err}"))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_INBOUND_ROUTER_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<InboundRouterReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let router = self.load_or_default(now)?;
        Ok(InboundRouterReport {
            router_path: self.path_display(),
            received_count: count_status(&router, InboundRouteStatus::Received),
            duplicate_count: count_status(&router, InboundRouteStatus::Duplicate),
            routed_count: count_status(&router, InboundRouteStatus::Routed),
            suppressed_count: count_status(&router, InboundRouteStatus::Suppressed),
            spool_pending_count: count_spool_status(&router, InboundSpoolStatus::Pending),
            spool_routed_count: count_spool_status(&router, InboundSpoolStatus::Routed),
            spool_suppressed_count: count_spool_status(&router, InboundSpoolStatus::Suppressed),
            persisted: self.path.exists(),
            router,
        })
    }

    pub fn ingest(&self, input: InboundEventInput) -> Result<InboundIngestReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let channel = normalize_non_empty(&input.channel, "channel")?;
        let source = normalize_non_empty(&input.source, "source")?;
        let chat_id = normalize_non_empty(&input.chat_id, "chat id")?;
        let message_id = normalize_non_empty(&input.message_id, "message id")?;
        let normalized_text = normalize_non_empty(&input.normalized_text, "normalized text")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        let reply_to_message_id =
            normalize_optional_single_line(input.reply_to_message_id.as_deref(), "reply target")?;
        let topic_id = normalize_optional_single_line(input.topic_id.as_deref(), "topic id")?;
        let active_session_id = normalize_optional_single_line(
            input.active_session_id.as_deref(),
            "active session id",
        )?;
        if let Some(existing) = router
            .events
            .iter()
            .find(|event| event.idempotency_key == idempotency_key)
            .cloned()
        {
            let mut duplicate = existing.clone();
            duplicate.status = InboundRouteStatus::Duplicate;
            duplicate.duplicate_of = Some(existing.event_id);
            return Ok(InboundIngestReport {
                router_path: self.path_display(),
                event: duplicate,
                duplicate_idempotency_key: true,
                spooled: false,
                persisted: self.path.exists(),
            });
        }
        let sequence = allocate_sequence(&mut router);
        let event_id = format!("inbound-{now}-{sequence}");
        let route_kind = classify_route(input.group_chat, input.direct_mention, &normalized_text);
        let status = if route_kind == "group_suppressed" {
            InboundRouteStatus::Suppressed
        } else {
            InboundRouteStatus::Received
        };
        let session_boundary_key = session_boundary_key(
            &channel,
            &chat_id,
            topic_id.as_deref(),
            active_session_id.as_deref(),
            input.reset_generation,
        );
        let provenance_hash = stable_hash(&format!(
            "{channel}\n{source}\n{chat_id}\n{message_id}\n{session_boundary_key}\n{normalized_text}"
        ));
        let event = InboundEventRecord {
            event_id: event_id.clone(),
            sequence,
            channel,
            source,
            chat_id,
            message_id,
            reply_to_message_id,
            topic_id,
            active_session_id,
            reset_generation: input.reset_generation,
            session_boundary_key,
            normalized_text_preview: redact_preview(&normalized_text),
            idempotency_key,
            provenance_hash,
            status,
            route_kind,
            duplicate_of: None,
            routed_session_id: None,
            readback_evidence_id: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        router.events.push(event.clone());
        push_spool_record(&mut router, &event, spool_status_for_route(status), now);
        push_audit_event(
            &mut router,
            status.label(),
            &event_id,
            now,
            "inbound event normalized and persisted; no reply delivered by router store",
        );
        self.save(&mut router, now)?;
        Ok(InboundIngestReport {
            router_path: self.path_display(),
            event,
            duplicate_idempotency_key: false,
            spooled: true,
            persisted: true,
        })
    }

    pub fn mark_routed(
        &self,
        event_id: &str,
        session_id: &str,
        readback_evidence_id: &str,
    ) -> Result<InboundRouteReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let event_id = normalize_non_empty(event_id, "event id")?;
        let session_id = normalize_non_empty(session_id, "session id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let event = router
            .events
            .iter_mut()
            .find(|candidate| candidate.event_id == event_id)
            .ok_or_else(|| HeptaError(format!("inbound event not found: {event_id}")))?;
        if event.status == InboundRouteStatus::Suppressed {
            return Err(HeptaError(format!(
                "inbound event {event_id} is suppressed and cannot be routed"
            )));
        }
        event.status = InboundRouteStatus::Routed;
        event.routed_session_id = Some(session_id);
        event.readback_evidence_id = Some(readback_evidence_id);
        event.updated_at_unix_ms = now;
        let route_kind = event.route_kind.clone();
        mark_spool_status(&mut router, &event_id, InboundSpoolStatus::Routed, now);
        push_audit_event(
            &mut router,
            "routed",
            &event_id,
            now,
            "inbound event routed to Hepta session with readback evidence",
        );
        self.save(&mut router, now)?;
        Ok(InboundRouteReport {
            router_path: self.path_display(),
            event_id,
            status: InboundRouteStatus::Routed,
            route_kind,
            persisted: true,
        })
    }

    pub fn gated_session_transcript_handoff(
        &self,
        transcript_store: &SessionTranscriptStore,
        evidence_ledger: &ReadbackEvidenceLedger,
        event_id: &str,
        input: InboundSessionTranscriptHandoffInput,
    ) -> Result<InboundSessionTranscriptHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut router = self.load_or_default(now)?;
        let event_id = normalize_non_empty(event_id, "event id")?;
        let session_id = normalize_non_empty(&input.session_id, "session id")?;
        let session_title = normalize_non_empty(&input.session_title, "session title")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        let expected_topic_id = normalize_optional_single_line(
            input.expected_topic_id.as_deref(),
            "expected topic id",
        )?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "inbound session transcript handoff for {event_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "inbound session transcript handoff for {event_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = router
            .session_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            let transcript_handoff = transcript_store.gated_append_turn_handoff(
                evidence_ledger,
                SessionTranscriptAppendHandoffInput {
                    session_id: existing.session_id.clone(),
                    title: session_title,
                    role: "user".into(),
                    text: "duplicate inbound session transcript handoff".into(),
                    policy_decision: policy_decision.clone(),
                    operator_confirmed: input.operator_confirmed,
                    idempotency_key: format!("{idempotency_key}:transcript"),
                },
            )?;
            return Ok(InboundSessionTranscriptHandoffReport {
                router_path: self.path_display(),
                transcript_store_path: transcript_store.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                handoff: existing,
                transcript_handoff,
                duplicate_idempotency_key: true,
                router_mutated_by_gate: false,
                transcript_store_mutated_by_gate: false,
                external_reply_performed_by_gate: false,
                model_invoked_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let event = router
            .events
            .iter()
            .find(|candidate| candidate.event_id == event_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("inbound event not found: {event_id}")))?;
        if event.status == InboundRouteStatus::Suppressed {
            return Err(HeptaError(format!(
                "inbound event {event_id} is suppressed and cannot hand off to a transcript"
            )));
        }
        if event.status == InboundRouteStatus::Duplicate {
            return Err(HeptaError(format!(
                "inbound event {event_id} is duplicate-only and cannot hand off to a transcript"
            )));
        }
        if event.status == InboundRouteStatus::Routed {
            return Err(HeptaError(format!(
                "inbound event {event_id} has already been routed"
            )));
        }
        if let Some(active_session_id) = event.active_session_id.as_deref() {
            if active_session_id != session_id {
                return Err(HeptaError(format!(
                    "inbound event {event_id} belongs to active session {active_session_id}, not {session_id}"
                )));
            }
        }
        if let Some(expected_topic_id) = expected_topic_id.as_deref() {
            if event.topic_id.as_deref() != Some(expected_topic_id) {
                return Err(HeptaError(format!(
                    "inbound event {event_id} topic boundary mismatch"
                )));
            }
        }
        if let Some(expected_reset_generation) = input.expected_reset_generation {
            if event.reset_generation != expected_reset_generation {
                return Err(HeptaError(format!(
                    "inbound event {event_id} reset generation mismatch"
                )));
            }
        }
        let transcript_handoff = transcript_store.gated_append_turn_handoff(
            evidence_ledger,
            SessionTranscriptAppendHandoffInput {
                session_id: session_id.clone(),
                title: session_title,
                role: "user".into(),
                text: event.normalized_text_preview.clone(),
                policy_decision: policy_decision.clone(),
                operator_confirmed: input.operator_confirmed,
                idempotency_key: format!("{idempotency_key}:transcript"),
            },
        )?;
        let handoff_id = format!(
            "inboundsessionhandoff-{}-{}",
            now,
            router.session_handoffs.len() + 1
        );
        let evidence = evidence_ledger.append(
            "inbound_session_transcript_handoff",
            &handoff_id,
            "routed_to_transcript",
            &format!(
                "inbound event {event_id} routed to Hepta session transcript {session_id}; external reply/model invocation not performed by this gate"
            ),
        )?;
        let readback_evidence_id = evidence.entry.evidence_id.clone();
        if let Some(event) = router
            .events
            .iter_mut()
            .find(|candidate| candidate.event_id == event_id)
        {
            event.status = InboundRouteStatus::Routed;
            event.routed_session_id = Some(session_id.clone());
            event.readback_evidence_id = Some(readback_evidence_id.clone());
            event.updated_at_unix_ms = now;
        }
        mark_spool_status(&mut router, &event_id, InboundSpoolStatus::Routed, now);
        let handoff = InboundSessionTranscriptHandoffRecord {
            handoff_id: handoff_id.clone(),
            event_id: event_id.clone(),
            sequence: event.sequence,
            session_id,
            topic_id: event.topic_id.clone(),
            session_boundary_key: event.session_boundary_key.clone(),
            transcript_handoff_id: transcript_handoff.handoff.handoff_id.clone(),
            transcript_turn_id: transcript_handoff.handoff.turn_id.clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id,
            created_at_unix_ms: now,
            router_mutated_by_gate: true,
            transcript_store_mutated_by_gate: transcript_handoff.transcript_store_mutated_by_gate,
            external_reply_performed_by_gate: false,
            model_invoked_by_gate: false,
        };
        router.session_handoffs.push(handoff.clone());
        router.session_handoffs.truncate(1024);
        push_audit_event(
            &mut router,
            "session_transcript_handoff_recorded",
            &event_id,
            now,
            "inbound event handed off to session transcript with readback evidence",
        );
        self.save(&mut router, now)?;
        Ok(InboundSessionTranscriptHandoffReport {
            router_path: self.path_display(),
            transcript_store_path: transcript_store.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            transcript_handoff,
            duplicate_idempotency_key: false,
            router_mutated_by_gate: true,
            transcript_store_mutated_by_gate: true,
            external_reply_performed_by_gate: false,
            model_invoked_by_gate: false,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<InboundRouterFile, HeptaError> {
        if !self.path.exists() {
            return Ok(InboundRouterFile {
                version: 1,
                router_id: DEFAULT_INBOUND_ROUTER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                next_sequence: default_next_sequence(),
                events: Vec::new(),
                spool: Vec::new(),
                session_handoffs: Vec::new(),
                audit_events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read inbound-router {}: {err}",
                self.path.display()
            ))
        })?;
        let mut router: InboundRouterFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse inbound-router {}: {err}",
                self.path.display()
            ))
        })?;
        if router.version != 1 {
            return Err(HeptaError(format!(
                "unsupported inbound-router version {} in {}",
                router.version,
                self.path.display()
            )));
        }
        repair_sequences(&mut router);
        router.audit_events.truncate(1024);
        Ok(router)
    }

    fn save(&self, router: &mut InboundRouterFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        router.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create inbound-router directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(router)
            .map_err(|err| HeptaError(format!("failed to serialize inbound-router: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write inbound-router {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(router: &InboundRouterFile, status: InboundRouteStatus) -> usize {
    router
        .events
        .iter()
        .filter(|event| event.status == status)
        .count()
}

fn count_spool_status(router: &InboundRouterFile, status: InboundSpoolStatus) -> usize {
    router
        .spool
        .iter()
        .filter(|record| record.status == status)
        .count()
}

const fn default_next_sequence() -> u64 {
    1
}

fn allocate_sequence(router: &mut InboundRouterFile) -> u64 {
    repair_sequences(router);
    let sequence = router.next_sequence.max(default_next_sequence());
    router.next_sequence = sequence.saturating_add(1);
    sequence
}

fn repair_sequences(router: &mut InboundRouterFile) {
    let mut highest = router
        .events
        .iter()
        .map(|event| event.sequence)
        .chain(router.spool.iter().map(|record| record.sequence))
        .max()
        .unwrap_or(0);
    for event in &mut router.events {
        if event.sequence == 0 {
            highest = highest.saturating_add(1);
            event.sequence = highest;
        }
        if event.session_boundary_key.is_empty() {
            event.session_boundary_key = session_boundary_key(
                &event.channel,
                &event.chat_id,
                event.topic_id.as_deref(),
                event.active_session_id.as_deref(),
                event.reset_generation,
            );
        }
    }
    for record in &mut router.spool {
        if record.sequence == 0 {
            highest = highest.saturating_add(1);
            record.sequence = highest;
        }
    }
    router.next_sequence = router
        .next_sequence
        .max(highest.saturating_add(1))
        .max(default_next_sequence());
}

fn classify_route(group_chat: bool, direct_mention: bool, normalized_text: &str) -> String {
    let trimmed = normalized_text.trim();
    if trimmed.starts_with('/') {
        "slash_command".into()
    } else if group_chat && !direct_mention {
        "group_suppressed".into()
    } else if group_chat {
        "group_mention".into()
    } else {
        "direct_session".into()
    }
}

fn spool_status_for_route(status: InboundRouteStatus) -> InboundSpoolStatus {
    match status {
        InboundRouteStatus::Suppressed => InboundSpoolStatus::Suppressed,
        _ => InboundSpoolStatus::Pending,
    }
}

fn push_spool_record(
    router: &mut InboundRouterFile,
    event: &InboundEventRecord,
    status: InboundSpoolStatus,
    now_unix_ms: u64,
) {
    router.spool.push(InboundSpoolRecord {
        spool_id: format!("inboundspool-{}-{}", now_unix_ms, event.sequence),
        event_id: event.event_id.clone(),
        sequence: event.sequence,
        status,
        channel: event.channel.clone(),
        chat_id: event.chat_id.clone(),
        message_id: event.message_id.clone(),
        topic_id: event.topic_id.clone(),
        active_session_id: event.active_session_id.clone(),
        reset_generation: event.reset_generation,
        route_kind: event.route_kind.clone(),
        idempotency_key: event.idempotency_key.clone(),
        normalized_text_preview: event.normalized_text_preview.clone(),
        created_at_unix_ms: now_unix_ms,
        updated_at_unix_ms: now_unix_ms,
        durable_local_spool: true,
        external_download_performed: false,
    });
    router.spool.truncate(2048);
}

fn mark_spool_status(
    router: &mut InboundRouterFile,
    event_id: &str,
    status: InboundSpoolStatus,
    now_unix_ms: u64,
) {
    for record in router
        .spool
        .iter_mut()
        .filter(|record| record.event_id == event_id)
    {
        record.status = status;
        record.updated_at_unix_ms = now_unix_ms;
    }
}

fn redact_preview(value: &str) -> String {
    let value = value.trim();
    let preview = if value.chars().count() > 96 {
        value.chars().take(96).collect::<String>()
    } else {
        value.to_string()
    };
    preview
        .split_whitespace()
        .map(|part| {
            if part.len() > 24 || part.contains("token=") || part.contains("secret") {
                "<redacted>"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn stable_hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "inbound router {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional_single_line(
    value: Option<&str>,
    label: &str,
) -> Result<Option<String>, HeptaError> {
    value
        .map(|raw| {
            let normalized = normalize_non_empty(raw, label)?;
            if normalized.contains('\n') || normalized.contains('\r') || normalized.contains("..") {
                return Err(HeptaError(format!(
                    "inbound router {label} must be single-line and scoped"
                )));
            }
            Ok(normalized)
        })
        .transpose()
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn session_boundary_key(
    channel: &str,
    chat_id: &str,
    topic_id: Option<&str>,
    active_session_id: Option<&str>,
    reset_generation: u64,
) -> String {
    stable_hash(&format!(
        "{channel}\n{chat_id}\n{}\n{}\n{reset_generation}",
        topic_id.unwrap_or(""),
        active_session_id.unwrap_or("")
    ))
}

fn push_audit_event(
    router: &mut InboundRouterFile,
    event_type: &str,
    event_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    router.audit_events.push(InboundRouterAuditEvent {
        audit_id: format!("inbaudit-{}-{}", now_unix_ms, router.audit_events.len() + 1),
        event_type: event_type.into(),
        event_id: event_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    router.audit_events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-inbound-router-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    fn direct_input(idempotency_key: &str) -> InboundEventInput {
        InboundEventInput {
            channel: "telegram".into(),
            source: "telegram:6476198178".into(),
            chat_id: "chat-1".into(),
            message_id: "msg-1".into(),
            normalized_text: "Continue the OpenClaw runtime event.".into(),
            idempotency_key: idempotency_key.into(),
            direct_mention: true,
            group_chat: false,
            reply_to_message_id: None,
            topic_id: Some("topic-openclaw-parity".into()),
            active_session_id: Some("session-main".into()),
            reset_generation: 1,
        }
    }

    #[test]
    fn inbound_router_ingests_deduplicates_and_routes_with_readback() {
        let path = temp_file("route");
        let store = InboundRouterStore::new(&path);
        let ingest = store.ingest(direct_input("inbound-idem-1")).unwrap();
        assert_eq!(ingest.event.status, InboundRouteStatus::Received);
        assert_eq!(ingest.event.route_kind, "direct_session");
        assert_eq!(ingest.event.sequence, 1);
        assert!(!ingest.event.session_boundary_key.is_empty());
        assert!(ingest.spooled);
        assert!(!ingest.event.provenance_hash.is_empty());
        let duplicate = store.ingest(direct_input("inbound-idem-1")).unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert_eq!(duplicate.event.status, InboundRouteStatus::Duplicate);
        assert!(!duplicate.spooled);
        let routed = store
            .mark_routed(&ingest.event.event_id, "session-main", "rb-inbound-1")
            .unwrap();
        assert_eq!(routed.status, InboundRouteStatus::Routed);
        let report = store.report(None).unwrap();
        assert_eq!(report.routed_count, 1);
        assert_eq!(report.spool_pending_count, 0);
        assert_eq!(report.spool_routed_count, 1);
        assert_eq!(
            report.router.events[0].readback_evidence_id.as_deref(),
            Some("rb-inbound-1")
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn inbound_router_suppresses_unmentioned_group_messages() {
        let path = temp_file("suppress");
        let store = InboundRouterStore::new(&path);
        let mut input = direct_input("inbound-idem-group");
        input.group_chat = true;
        input.direct_mention = false;
        input.normalized_text = "casual chatter".into();
        let ingest = store.ingest(input).unwrap();
        assert_eq!(ingest.event.status, InboundRouteStatus::Suppressed);
        assert_eq!(ingest.event.route_kind, "group_suppressed");
        assert!(
            store
                .mark_routed(&ingest.event.event_id, "session-main", "rb-suppressed")
                .is_err()
        );
        let report = store.report(None).unwrap();
        assert_eq!(report.suppressed_count, 1);
        assert_eq!(report.spool_suppressed_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn inbound_router_gated_handoff_appends_session_transcript_without_replying() {
        use crate::{ReadbackEvidenceLedger, SessionTranscriptStore};

        let path = temp_file("handoff-router");
        let transcript_path = temp_file("handoff-transcript");
        let ledger_path = temp_file("handoff-ledger");
        let store = InboundRouterStore::new(&path);
        let transcripts = SessionTranscriptStore::new(&transcript_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let ingest = store.ingest(direct_input("inbound-idem-handoff")).unwrap();
        let input = InboundSessionTranscriptHandoffInput {
            session_id: "session-main".into(),
            session_title: "Inbound routed session".into(),
            policy_decision: "approved-inbound-session-route".into(),
            operator_confirmed: true,
            idempotency_key: "inbound-session-handoff-1".into(),
            expected_topic_id: Some("topic-openclaw-parity".into()),
            expected_reset_generation: Some(1),
        };
        let unconfirmed = InboundSessionTranscriptHandoffInput {
            operator_confirmed: false,
            ..input.clone()
        };
        assert!(
            store
                .gated_session_transcript_handoff(
                    &transcripts,
                    &ledger,
                    &ingest.event.event_id,
                    unconfirmed
                )
                .is_err()
        );
        let handoff = store
            .gated_session_transcript_handoff(&transcripts, &ledger, &ingest.event.event_id, input)
            .unwrap();
        assert!(handoff.router_mutated_by_gate);
        assert!(handoff.transcript_store_mutated_by_gate);
        assert!(!handoff.external_reply_performed_by_gate);
        assert!(!handoff.model_invoked_by_gate);
        let router = store.report(None).unwrap();
        assert_eq!(router.routed_count, 1);
        assert_eq!(router.router.session_handoffs.len(), 1);
        let transcript = transcripts.report(None).unwrap();
        assert_eq!(transcript.session_count, 1);
        assert_eq!(transcript.store.sessions[0].turn_count, 1);
        let evidence = ledger.report(None).unwrap();
        assert_eq!(evidence.evidence_count, 2);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(transcript_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn inbound_router_enforces_active_session_topic_and_reset_boundaries() {
        use crate::{ReadbackEvidenceLedger, SessionTranscriptStore};

        let path = temp_file("boundary-router");
        let transcript_path = temp_file("boundary-transcript");
        let ledger_path = temp_file("boundary-ledger");
        let store = InboundRouterStore::new(&path);
        let transcripts = SessionTranscriptStore::new(&transcript_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let ingest = store.ingest(direct_input("inbound-idem-boundary")).unwrap();

        let wrong_session = InboundSessionTranscriptHandoffInput {
            session_id: "session-other".into(),
            session_title: "Wrong session".into(),
            policy_decision: "approved-inbound-session-route".into(),
            operator_confirmed: true,
            idempotency_key: "boundary-wrong-session".into(),
            expected_topic_id: Some("topic-openclaw-parity".into()),
            expected_reset_generation: Some(1),
        };
        assert!(
            store
                .gated_session_transcript_handoff(
                    &transcripts,
                    &ledger,
                    &ingest.event.event_id,
                    wrong_session
                )
                .is_err()
        );

        let wrong_topic = InboundSessionTranscriptHandoffInput {
            session_id: "session-main".into(),
            session_title: "Wrong topic".into(),
            policy_decision: "approved-inbound-session-route".into(),
            operator_confirmed: true,
            idempotency_key: "boundary-wrong-topic".into(),
            expected_topic_id: Some("topic-after-reset".into()),
            expected_reset_generation: Some(1),
        };
        assert!(
            store
                .gated_session_transcript_handoff(
                    &transcripts,
                    &ledger,
                    &ingest.event.event_id,
                    wrong_topic
                )
                .is_err()
        );

        let wrong_generation = InboundSessionTranscriptHandoffInput {
            session_id: "session-main".into(),
            session_title: "Wrong generation".into(),
            policy_decision: "approved-inbound-session-route".into(),
            operator_confirmed: true,
            idempotency_key: "boundary-wrong-generation".into(),
            expected_topic_id: Some("topic-openclaw-parity".into()),
            expected_reset_generation: Some(2),
        };
        assert!(
            store
                .gated_session_transcript_handoff(
                    &transcripts,
                    &ledger,
                    &ingest.event.event_id,
                    wrong_generation
                )
                .is_err()
        );

        let report = store.report(None).unwrap();
        assert_eq!(report.received_count, 1);
        assert_eq!(report.spool_pending_count, 1);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(transcript_path);
        let _ = fs::remove_file(ledger_path);
    }
}
