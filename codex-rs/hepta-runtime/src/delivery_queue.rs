use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;

pub const DEFAULT_DELIVERY_QUEUE_PATH: &str = ".hepta/delivery-queue-v0.json";
pub const DEFAULT_READBACK_EVIDENCE_PATH: &str = ".hepta/readback-evidence-v0.json";
pub const DEFAULT_DELIVERY_QUEUE_ID: &str = "hepta-native-delivery-queue";
pub const DEFAULT_READBACK_EVIDENCE_LEDGER_ID: &str = "hepta-readback-evidence-ledger";
pub const DEFAULT_DELIVERY_LEASE_MS: u64 = 5 * 60 * 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryQueueStatus {
    Queued,
    InFlight,
    Delivered,
    Failed,
    DeadLetter,
}

impl DeliveryQueueStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::InFlight => "in_flight",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::DeadLetter => "dead_letter",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryQueueFile {
    pub version: u32,
    pub queue_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub items: Vec<DeliveryQueueItem>,
    #[serde(default)]
    pub events: Vec<DeliveryQueueEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryQueueItem {
    pub delivery_id: String,
    pub delivery_kind: String,
    pub target: String,
    pub payload_preview: String,
    #[serde(default)]
    pub rich_content_present: bool,
    #[serde(default)]
    pub channel_native_payload_present: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reply_channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reply_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ui_turn_id: Option<String>,
    #[serde(default)]
    pub active_ui_turn_mirrored: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_completion_id: Option<String>,
    #[serde(default)]
    pub task_completion_routed_to_requester: bool,
    pub idempotency_key: String,
    pub status: DeliveryQueueStatus,
    pub attempt_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_expires_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryQueueEvent {
    pub event_id: String,
    pub event_type: String,
    pub delivery_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueReport {
    pub queue_path: String,
    pub queue: DeliveryQueueFile,
    pub queued_count: usize,
    pub in_flight_count: usize,
    pub delivered_count: usize,
    pub failed_count: usize,
    pub dead_letter_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueEnqueueReport {
    pub queue_path: String,
    pub item: DeliveryQueueItem,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueClaimReport {
    pub queue_path: String,
    pub worker_id: String,
    pub item: DeliveryQueueItem,
    pub lease_expires_unix_ms: u64,
    pub reclaimed_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueAckReport {
    pub queue_path: String,
    pub delivery_id: String,
    pub readback_evidence_id: String,
    pub status: DeliveryQueueStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueFailReport {
    pub queue_path: String,
    pub delivery_id: String,
    pub status: DeliveryQueueStatus,
    pub attempt_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueReadbackGateReport {
    pub queue_path: String,
    pub evidence_ledger_path: String,
    pub delivery_id: String,
    pub readback_evidence_id: String,
    pub status: DeliveryQueueStatus,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub external_send_performed_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChannelSendHandoffInput {
    pub delivery_kind: String,
    pub target: String,
    pub payload_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChannelSendHandoffReport {
    pub queue_path: String,
    pub evidence_ledger_path: String,
    pub delivery_id: String,
    pub readback_evidence_id: String,
    pub duplicate_idempotency_key: bool,
    pub status: DeliveryQueueStatus,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub queue_mutated_by_gate: bool,
    pub external_send_performed_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RichDeliveryHandoffInput {
    pub delivery_kind: String,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_preview: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rich_blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_native_payload_preview: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reply_channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reply_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_ui_turn_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_completion_id: Option<String>,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RichDeliveryHandoffReport {
    pub queue_path: String,
    pub evidence_ledger_path: String,
    pub delivery_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    pub duplicate_idempotency_key: bool,
    pub status: DeliveryQueueStatus,
    pub content_accepted_without_text: bool,
    pub rich_content_present: bool,
    pub channel_native_payload_present: bool,
    pub source_reply_metadata_preserved: bool,
    pub active_ui_turn_mirrored: bool,
    pub task_completion_routed_to_requester: bool,
    pub queue_mutated_by_gate: bool,
    pub external_send_performed_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeliveryQueueReclaimReport {
    pub queue_path: String,
    pub now_unix_ms: u64,
    pub reclaimed_count: usize,
    pub reclaimed_delivery_ids: Vec<String>,
    pub persisted: bool,
}

pub struct DurableDeliveryQueue {
    path: PathBuf,
}

impl DurableDeliveryQueue {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!("failed to resolve cwd for delivery-queue: {err}"))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_DELIVERY_QUEUE_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<DeliveryQueueReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let queue = self.load_or_default(now)?;
        Ok(DeliveryQueueReport {
            queue_path: self.path_display(),
            queued_count: count_status(&queue, DeliveryQueueStatus::Queued),
            in_flight_count: count_status(&queue, DeliveryQueueStatus::InFlight),
            delivered_count: count_status(&queue, DeliveryQueueStatus::Delivered),
            failed_count: count_status(&queue, DeliveryQueueStatus::Failed),
            dead_letter_count: count_status(&queue, DeliveryQueueStatus::DeadLetter),
            persisted: self.path.exists(),
            queue,
        })
    }

    pub fn enqueue(
        &self,
        delivery_kind: &str,
        target: &str,
        payload_preview: &str,
        idempotency_key: &str,
    ) -> Result<DeliveryQueueEnqueueReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut queue = self.load_or_default(now)?;
        let delivery_kind = normalize_non_empty(delivery_kind, "delivery kind")?;
        let target = normalize_non_empty(target, "target")?;
        let payload_preview = normalize_non_empty(payload_preview, "payload preview")?;
        let idempotency_key = normalize_non_empty(idempotency_key, "idempotency key")?;
        if let Some(existing) = queue
            .items
            .iter()
            .find(|item| item.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(DeliveryQueueEnqueueReport {
                queue_path: self.path_display(),
                item: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let delivery_id = format!("dq-{}-{}", now, queue.items.len() + 1);
        let item = DeliveryQueueItem {
            delivery_id: delivery_id.clone(),
            delivery_kind,
            target,
            payload_preview,
            rich_content_present: false,
            channel_native_payload_present: false,
            source_reply_channel: None,
            source_reply_message_id: None,
            active_ui_turn_id: None,
            active_ui_turn_mirrored: false,
            requester_session_key: None,
            task_completion_id: None,
            task_completion_routed_to_requester: false,
            idempotency_key,
            status: DeliveryQueueStatus::Queued,
            attempt_count: 0,
            claimed_by: None,
            lease_expires_unix_ms: None,
            last_error: None,
            readback_evidence_id: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        queue.items.push(item.clone());
        push_delivery_event(&mut queue, "queued", &delivery_id, now, "delivery queued");
        self.save(&mut queue, now)?;
        Ok(DeliveryQueueEnqueueReport {
            queue_path: self.path_display(),
            item,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn claim_next(
        &self,
        worker_id: &str,
        lease_ms: Option<u64>,
    ) -> Result<DeliveryQueueClaimReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut queue = self.load_or_default(now)?;
        let worker_id = normalize_non_empty(worker_id, "worker id")?;
        let reclaimed = reclaim_stale_in_queue(&mut queue, now);
        let selected = queue
            .items
            .iter()
            .position(|item| {
                matches!(
                    item.status,
                    DeliveryQueueStatus::Queued | DeliveryQueueStatus::Failed
                )
            })
            .ok_or_else(|| {
                HeptaError(format!("no claimable delivery item for worker {worker_id}"))
            })?;
        let lease_expires_unix_ms =
            now.saturating_add(lease_ms.unwrap_or(DEFAULT_DELIVERY_LEASE_MS).max(1_000));
        {
            let item = &mut queue.items[selected];
            item.status = DeliveryQueueStatus::InFlight;
            item.claimed_by = Some(worker_id.clone());
            item.lease_expires_unix_ms = Some(lease_expires_unix_ms);
            item.attempt_count = item.attempt_count.saturating_add(1);
            item.updated_at_unix_ms = now;
        }
        let item = queue.items[selected].clone();
        push_delivery_event(
            &mut queue,
            "claimed",
            &item.delivery_id,
            now,
            "delivery claimed",
        );
        self.save(&mut queue, now)?;
        Ok(DeliveryQueueClaimReport {
            queue_path: self.path_display(),
            worker_id,
            item,
            lease_expires_unix_ms,
            reclaimed_count: reclaimed.reclaimed_count,
            persisted: true,
        })
    }

    pub fn ack_delivered(
        &self,
        delivery_id: &str,
        readback_evidence_id: &str,
    ) -> Result<DeliveryQueueAckReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut queue = self.load_or_default(now)?;
        let delivery_id = normalize_non_empty(delivery_id, "delivery id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let item = queue
            .items
            .iter_mut()
            .find(|item| item.delivery_id == delivery_id)
            .ok_or_else(|| HeptaError(format!("delivery item not found: {delivery_id}")))?;
        item.status = DeliveryQueueStatus::Delivered;
        item.claimed_by = None;
        item.lease_expires_unix_ms = None;
        item.readback_evidence_id = Some(readback_evidence_id.clone());
        item.updated_at_unix_ms = now;
        push_delivery_event(
            &mut queue,
            "delivered",
            &delivery_id,
            now,
            "delivery acknowledged with readback evidence",
        );
        self.save(&mut queue, now)?;
        Ok(DeliveryQueueAckReport {
            queue_path: self.path_display(),
            delivery_id,
            readback_evidence_id,
            status: DeliveryQueueStatus::Delivered,
            persisted: true,
        })
    }

    pub fn record_adapter_readback(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        delivery_id: &str,
        policy_decision: &str,
        operator_confirmed: bool,
        readback_summary: &str,
    ) -> Result<DeliveryQueueReadbackGateReport, HeptaError> {
        let delivery_id = normalize_non_empty(delivery_id, "delivery id")?;
        let policy_decision = normalize_non_empty(policy_decision, "policy decision")?;
        let readback_summary = normalize_non_empty(readback_summary, "readback summary")?;
        if !operator_confirmed {
            return Err(HeptaError(format!(
                "delivery {delivery_id} readback requires explicit operator confirmation"
            )));
        }
        let policy_lower = policy_decision.to_ascii_lowercase();
        if !(policy_lower.contains("allow") || policy_lower.contains("approved")) {
            return Err(HeptaError(format!(
                "delivery {delivery_id} readback requires allow/approved policy decision"
            )));
        }
        let evidence =
            evidence_ledger.append("delivery", &delivery_id, "delivered", &readback_summary)?;
        let ack = self.ack_delivered(&delivery_id, &evidence.entry.evidence_id)?;
        Ok(DeliveryQueueReadbackGateReport {
            queue_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            delivery_id,
            readback_evidence_id: ack.readback_evidence_id,
            status: ack.status,
            policy_decision,
            operator_confirmed,
            external_send_performed_by_gate: false,
            persisted: ack.persisted && evidence.persisted,
        })
    }

    pub fn gated_channel_send_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ChannelSendHandoffInput,
    ) -> Result<ChannelSendHandoffReport, HeptaError> {
        let delivery_kind = normalize_non_empty(&input.delivery_kind, "delivery kind")?;
        let target = normalize_non_empty(&input.target, "target")?;
        let payload_preview = normalize_non_empty(&input.payload_preview, "payload preview")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "channel-send handoff for {target} requires explicit operator confirmation"
            )));
        }
        let policy_lower = policy_decision.to_ascii_lowercase();
        if !(policy_lower.contains("allow") || policy_lower.contains("approved")) {
            return Err(HeptaError(format!(
                "channel-send handoff for {target} requires allow/approved policy decision"
            )));
        }
        let enqueued = self.enqueue(&delivery_kind, &target, &payload_preview, &idempotency_key)?;
        let delivery_id = enqueued.item.delivery_id.clone();
        let evidence = evidence_ledger.append(
            "delivery_handoff",
            &delivery_id,
            enqueued.item.status.label(),
            &format!(
                "channel-send handoff queued for {target}; external send not performed by this gate"
            ),
        )?;
        let evidence_id = evidence.entry.evidence_id.clone();
        if !enqueued.duplicate_idempotency_key {
            let mut queue = self.load_or_default(current_unix_ms()?)?;
            if let Some(item) = queue
                .items
                .iter_mut()
                .find(|item| item.delivery_id == delivery_id)
            {
                item.readback_evidence_id = Some(evidence_id.clone());
            }
            let now = current_unix_ms()?;
            push_delivery_event(
                &mut queue,
                "handoff_readback_recorded",
                &delivery_id,
                now,
                "channel-send handoff readback evidence recorded",
            );
            self.save(&mut queue, now)?;
        }
        Ok(ChannelSendHandoffReport {
            queue_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            delivery_id,
            readback_evidence_id: evidence_id,
            duplicate_idempotency_key: enqueued.duplicate_idempotency_key,
            status: enqueued.item.status,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            queue_mutated_by_gate: !enqueued.duplicate_idempotency_key,
            external_send_performed_by_gate: false,
            persisted: enqueued.persisted && evidence.persisted,
        })
    }

    pub fn gated_rich_delivery_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: RichDeliveryHandoffInput,
    ) -> Result<RichDeliveryHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut queue = self.load_or_default(now)?;
        let delivery_kind = normalize_non_empty(&input.delivery_kind, "delivery kind")?;
        let target = normalize_non_empty(&input.target, "target")?;
        let text_preview = input
            .text_preview
            .as_deref()
            .map(|value| normalize_non_empty(value, "text preview"))
            .transpose()?;
        let rich_blocks = normalize_unique_values(&input.rich_blocks, "rich block")?;
        let media_refs = normalize_unique_values(&input.media_refs, "media ref")?;
        let channel_native_payload_preview = input
            .channel_native_payload_preview
            .as_deref()
            .map(|value| normalize_non_empty(value, "channel native payload preview"))
            .transpose()?;
        let (source_reply_channel, source_reply_message_id) = normalize_source_reply(
            input.source_reply_channel.as_deref(),
            input.source_reply_message_id.as_deref(),
        )?;
        let active_ui_turn_id = input
            .active_ui_turn_id
            .as_deref()
            .map(|value| normalize_non_empty(value, "active UI turn id"))
            .transpose()?;
        let requester_session_key = input
            .requester_session_key
            .as_deref()
            .map(|value| normalize_non_empty(value, "requester session key"))
            .transpose()?;
        let task_completion_id = input
            .task_completion_id
            .as_deref()
            .map(|value| normalize_non_empty(value, "task completion id"))
            .transpose()?;
        if task_completion_id.is_some() && requester_session_key.is_none() {
            return Err(HeptaError(
                "task completion rich delivery requires requester session key".into(),
            ));
        }
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "rich delivery handoff for {target} requires explicit operator confirmation"
            )));
        }
        let policy_lower = policy_decision.to_ascii_lowercase();
        if !(policy_lower.contains("allow") || policy_lower.contains("approved")) {
            return Err(HeptaError(format!(
                "rich delivery handoff for {target} requires allow/approved policy decision"
            )));
        }
        let rich_content_present = !rich_blocks.is_empty();
        let channel_native_payload_present = channel_native_payload_preview.is_some();
        let content_present = text_preview.is_some()
            || rich_content_present
            || !media_refs.is_empty()
            || channel_native_payload_present;
        if !content_present {
            return Err(HeptaError(
                "rich delivery handoff requires text, rich block, media, or native payload content"
                    .into(),
            ));
        }
        if let Some(existing) = queue
            .items
            .iter()
            .find(|item| item.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(RichDeliveryHandoffReport {
                queue_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                delivery_id: existing.delivery_id,
                readback_evidence_id: existing.readback_evidence_id,
                duplicate_idempotency_key: true,
                status: existing.status,
                content_accepted_without_text: text_preview.is_none()
                    && (rich_content_present
                        || !media_refs.is_empty()
                        || channel_native_payload_present),
                rich_content_present: existing.rich_content_present,
                channel_native_payload_present: existing.channel_native_payload_present,
                source_reply_metadata_preserved: existing.source_reply_channel.is_some()
                    && existing.source_reply_message_id.is_some(),
                active_ui_turn_mirrored: existing.active_ui_turn_mirrored,
                task_completion_routed_to_requester: existing.task_completion_routed_to_requester,
                queue_mutated_by_gate: false,
                external_send_performed_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let delivery_id = format!("dq-{}-{}", now, queue.items.len() + 1);
        let payload_preview = rich_payload_preview(
            text_preview.as_deref(),
            &rich_blocks,
            &media_refs,
            channel_native_payload_preview.as_deref(),
            source_reply_channel.as_deref(),
            source_reply_message_id.as_deref(),
            active_ui_turn_id.as_deref(),
            requester_session_key.as_deref(),
            task_completion_id.as_deref(),
        );
        let active_ui_turn_mirrored = active_ui_turn_id.is_some();
        let task_completion_routed_to_requester =
            requester_session_key.is_some() && task_completion_id.is_some();
        let mut item = DeliveryQueueItem {
            delivery_id: delivery_id.clone(),
            delivery_kind,
            target: target.clone(),
            payload_preview,
            rich_content_present,
            channel_native_payload_present,
            source_reply_channel,
            source_reply_message_id,
            active_ui_turn_id,
            active_ui_turn_mirrored,
            requester_session_key,
            task_completion_id,
            task_completion_routed_to_requester,
            idempotency_key,
            status: DeliveryQueueStatus::Queued,
            attempt_count: 0,
            claimed_by: None,
            lease_expires_unix_ms: None,
            last_error: None,
            readback_evidence_id: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        let evidence = evidence_ledger.append(
            "rich_delivery_handoff",
            &delivery_id,
            item.status.label(),
            &format!(
                "rich delivery handoff queued for {target}; source reply/UI turn/task routing metadata preserved; external send not performed"
            ),
        )?;
        item.readback_evidence_id = Some(evidence.entry.evidence_id.clone());
        queue.items.push(item.clone());
        push_delivery_event(
            &mut queue,
            "rich_handoff_queued",
            &delivery_id,
            now,
            "rich delivery handoff queued with reply metadata, UI mirror, idempotency, and readback evidence",
        );
        self.save(&mut queue, now)?;
        Ok(RichDeliveryHandoffReport {
            queue_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            delivery_id,
            readback_evidence_id: item.readback_evidence_id,
            duplicate_idempotency_key: false,
            status: DeliveryQueueStatus::Queued,
            content_accepted_without_text: text_preview.is_none()
                && (rich_content_present
                    || !media_refs.is_empty()
                    || channel_native_payload_present),
            rich_content_present,
            channel_native_payload_present,
            source_reply_metadata_preserved: item.source_reply_channel.is_some()
                && item.source_reply_message_id.is_some(),
            active_ui_turn_mirrored,
            task_completion_routed_to_requester,
            queue_mutated_by_gate: true,
            external_send_performed_by_gate: false,
            persisted: evidence.persisted,
        })
    }

    pub fn fail_delivery(
        &self,
        delivery_id: &str,
        error: &str,
        max_attempts: usize,
    ) -> Result<DeliveryQueueFailReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut queue = self.load_or_default(now)?;
        let delivery_id = normalize_non_empty(delivery_id, "delivery id")?;
        let error = normalize_non_empty(error, "delivery error")?;
        let item = queue
            .items
            .iter_mut()
            .find(|item| item.delivery_id == delivery_id)
            .ok_or_else(|| HeptaError(format!("delivery item not found: {delivery_id}")))?;
        item.status = if item.attempt_count >= max_attempts.max(1) {
            DeliveryQueueStatus::DeadLetter
        } else {
            DeliveryQueueStatus::Failed
        };
        item.claimed_by = None;
        item.lease_expires_unix_ms = None;
        item.last_error = Some(error);
        item.updated_at_unix_ms = now;
        let status = item.status;
        let attempt_count = item.attempt_count;
        push_delivery_event(
            &mut queue,
            status.label(),
            &delivery_id,
            now,
            "delivery failure recorded",
        );
        self.save(&mut queue, now)?;
        Ok(DeliveryQueueFailReport {
            queue_path: self.path_display(),
            delivery_id,
            status,
            attempt_count,
            persisted: true,
        })
    }

    pub fn reclaim_stale(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<DeliveryQueueReclaimReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let mut queue = self.load_or_default(now)?;
        let report = reclaim_stale_in_queue(&mut queue, now);
        self.save(&mut queue, now)?;
        Ok(DeliveryQueueReclaimReport {
            queue_path: self.path_display(),
            persisted: true,
            ..report
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<DeliveryQueueFile, HeptaError> {
        if !self.path.exists() {
            return Ok(DeliveryQueueFile {
                version: 1,
                queue_id: DEFAULT_DELIVERY_QUEUE_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                items: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read delivery-queue {}: {err}",
                self.path.display()
            ))
        })?;
        let mut queue: DeliveryQueueFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse delivery-queue {}: {err}",
                self.path.display()
            ))
        })?;
        if queue.version != 1 {
            return Err(HeptaError(format!(
                "unsupported delivery-queue version {} in {}",
                queue.version,
                self.path.display()
            )));
        }
        queue.events.truncate(1024);
        Ok(queue)
    }

    fn save(&self, queue: &mut DeliveryQueueFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        queue.updated_at_unix_ms = now_unix_ms;
        write_json_file(&self.path, queue, "delivery-queue")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadbackEvidenceLedgerFile {
    pub version: u32,
    pub ledger_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub entries: Vec<ReadbackEvidenceEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadbackEvidenceEntry {
    pub evidence_id: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub status: String,
    pub summary: String,
    pub observed_at_unix_ms: u64,
    pub payload_hash: String,
    pub previous_hash: String,
    pub chain_head: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReadbackEvidenceReport {
    pub ledger_path: String,
    pub ledger: ReadbackEvidenceLedgerFile,
    pub evidence_count: usize,
    pub chain_head: String,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReadbackEvidenceAppendReport {
    pub ledger_path: String,
    pub entry: ReadbackEvidenceEntry,
    pub evidence_count: usize,
    pub chain_head: String,
    pub persisted: bool,
}

pub struct ReadbackEvidenceLedger {
    path: PathBuf,
}

impl ReadbackEvidenceLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!(
                "failed to resolve cwd for readback-evidence: {err}"
            ))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_READBACK_EVIDENCE_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<ReadbackEvidenceReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let ledger = self.load_or_default(now)?;
        let chain_head = ledger
            .entries
            .last()
            .map(|entry| entry.chain_head.clone())
            .unwrap_or_else(|| "genesis".into());
        Ok(ReadbackEvidenceReport {
            ledger_path: self.path_display(),
            evidence_count: ledger.entries.len(),
            chain_head,
            persisted: self.path.exists(),
            ledger,
        })
    }

    pub fn append(
        &self,
        subject_kind: &str,
        subject_id: &str,
        status: &str,
        summary: &str,
    ) -> Result<ReadbackEvidenceAppendReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let subject_kind = normalize_non_empty(subject_kind, "subject kind")?;
        let subject_id = normalize_non_empty(subject_id, "subject id")?;
        let status = normalize_non_empty(status, "status")?;
        let summary = normalize_non_empty(summary, "summary")?;
        let previous_hash = ledger
            .entries
            .last()
            .map(|entry| entry.chain_head.clone())
            .unwrap_or_else(|| "genesis".into());
        let payload_hash = stable_hash(&format!(
            "{subject_kind}\n{subject_id}\n{status}\n{summary}\n{now}"
        ));
        let chain_head = stable_hash(&format!("{previous_hash}\n{payload_hash}"));
        let evidence_id = format!("rb-{}-{}", now, ledger.entries.len() + 1);
        let entry = ReadbackEvidenceEntry {
            evidence_id,
            subject_kind,
            subject_id,
            status,
            summary,
            observed_at_unix_ms: now,
            payload_hash,
            previous_hash,
            chain_head: chain_head.clone(),
        };
        ledger.entries.push(entry.clone());
        self.save(&mut ledger, now)?;
        Ok(ReadbackEvidenceAppendReport {
            ledger_path: self.path_display(),
            entry,
            evidence_count: ledger.entries.len(),
            chain_head,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ReadbackEvidenceLedgerFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ReadbackEvidenceLedgerFile {
                version: 1,
                ledger_id: DEFAULT_READBACK_EVIDENCE_LEDGER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                entries: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read readback-evidence ledger {}: {err}",
                self.path.display()
            ))
        })?;
        let ledger: ReadbackEvidenceLedgerFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse readback-evidence ledger {}: {err}",
                self.path.display()
            ))
        })?;
        if ledger.version != 1 {
            return Err(HeptaError(format!(
                "unsupported readback-evidence ledger version {} in {}",
                ledger.version,
                self.path.display()
            )));
        }
        Ok(ledger)
    }

    fn save(
        &self,
        ledger: &mut ReadbackEvidenceLedgerFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        ledger.updated_at_unix_ms = now_unix_ms;
        write_json_file(&self.path, ledger, "readback-evidence ledger")
    }
}

fn count_status(queue: &DeliveryQueueFile, status: DeliveryQueueStatus) -> usize {
    queue
        .items
        .iter()
        .filter(|item| item.status == status)
        .count()
}

fn reclaim_stale_in_queue(
    queue: &mut DeliveryQueueFile,
    now_unix_ms: u64,
) -> DeliveryQueueReclaimReport {
    let mut reclaimed_delivery_ids = Vec::new();
    for item in &mut queue.items {
        if item.status == DeliveryQueueStatus::InFlight
            && item
                .lease_expires_unix_ms
                .is_some_and(|lease| lease <= now_unix_ms)
        {
            item.status = DeliveryQueueStatus::Failed;
            item.claimed_by = None;
            item.lease_expires_unix_ms = None;
            item.last_error = Some("delivery lease expired before acknowledgement".into());
            item.updated_at_unix_ms = now_unix_ms;
            reclaimed_delivery_ids.push(item.delivery_id.clone());
        }
    }
    let reclaimed_count = reclaimed_delivery_ids.len();
    for delivery_id in &reclaimed_delivery_ids {
        push_delivery_event(
            queue,
            "lease_reclaimed",
            delivery_id,
            now_unix_ms,
            "delivery lease expired and returned to retry pool",
        );
    }
    DeliveryQueueReclaimReport {
        queue_path: String::new(),
        now_unix_ms,
        reclaimed_count,
        reclaimed_delivery_ids,
        persisted: false,
    }
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!("{label} must not be empty")));
    }
    Ok(trimmed.to_string())
}

fn normalize_unique_values(values: &[String], label: &str) -> Result<Vec<String>, HeptaError> {
    let mut out = Vec::new();
    for value in values {
        let value = normalize_non_empty(value, label)?;
        if !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
    }
    Ok(out)
}

fn normalize_source_reply(
    channel: Option<&str>,
    message_id: Option<&str>,
) -> Result<(Option<String>, Option<String>), HeptaError> {
    match (channel, message_id) {
        (Some(channel), Some(message_id)) => Ok((
            Some(normalize_non_empty(channel, "source reply channel")?),
            Some(normalize_non_empty(message_id, "source reply message id")?),
        )),
        (None, None) => Ok((None, None)),
        _ => Err(HeptaError(
            "source reply metadata requires both channel and message id".into(),
        )),
    }
}

fn rich_payload_preview(
    text_preview: Option<&str>,
    rich_blocks: &[String],
    media_refs: &[String],
    channel_native_payload_preview: Option<&str>,
    source_reply_channel: Option<&str>,
    source_reply_message_id: Option<&str>,
    active_ui_turn_id: Option<&str>,
    requester_session_key: Option<&str>,
    task_completion_id: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    if let Some(text) = text_preview {
        parts.push(format!("text={}", compact_preview(text, 160)));
    }
    if !rich_blocks.is_empty() {
        parts.push(format!("rich_blocks={}", rich_blocks.len()));
    }
    if !media_refs.is_empty() {
        parts.push(format!("media_refs={}", media_refs.len()));
    }
    if let Some(payload) = channel_native_payload_preview {
        parts.push(format!("native_payload={}", compact_preview(payload, 160)));
    }
    if let (Some(channel), Some(message_id)) = (source_reply_channel, source_reply_message_id) {
        parts.push(format!("source_reply={channel}/{message_id}"));
    }
    if let Some(turn_id) = active_ui_turn_id {
        parts.push(format!("active_ui_turn={turn_id}"));
    }
    if let (Some(session), Some(task_id)) = (requester_session_key, task_completion_id) {
        parts.push(format!(
            "task_completion={task_id}; requester_session={session}"
        ));
    }
    parts.join("; ")
}

fn compact_preview(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn push_delivery_event(
    queue: &mut DeliveryQueueFile,
    event_type: &str,
    delivery_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    queue.events.push(DeliveryQueueEvent {
        event_id: format!("dqevt-{}-{}", now_unix_ms, queue.events.len() + 1),
        event_type: event_type.into(),
        delivery_id: delivery_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    queue.events.truncate(1024);
}

fn stable_hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn write_json_file<T: Serialize>(path: &PathBuf, value: &T, label: &str) -> Result<(), HeptaError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            HeptaError(format!(
                "failed to create {label} directory {}: {err}",
                parent.display()
            ))
        })?;
    }
    let text = serde_json::to_string_pretty(value)
        .map_err(|err| HeptaError(format!("failed to serialize {label}: {err}")))?;
    fs::write(path, text)
        .map_err(|err| HeptaError(format!("failed to write {label} {}: {err}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-delivery-queue-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn delivery_queue_enqueues_claims_and_acks_with_readback_evidence() {
        let queue_path = temp_file("queue-ack");
        let ledger_path = temp_file("ledger-ack");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let enqueued = queue
            .enqueue("telegram-send", "telegram:chat", "hello preview", "idem-1")
            .unwrap();
        assert_eq!(enqueued.item.status, DeliveryQueueStatus::Queued);
        let duplicate = queue
            .enqueue("telegram-send", "telegram:chat", "hello preview", "idem-1")
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        let claimed = queue.claim_next("worker-a", Some(10_000)).unwrap();
        assert_eq!(claimed.item.status, DeliveryQueueStatus::InFlight);
        assert_eq!(claimed.item.attempt_count, 1);
        let evidence = ledger
            .append(
                "delivery",
                &claimed.item.delivery_id,
                "delivered",
                "provider accepted message id 42",
            )
            .unwrap();
        let ack = queue
            .ack_delivered(&claimed.item.delivery_id, &evidence.entry.evidence_id)
            .unwrap();
        assert_eq!(ack.status, DeliveryQueueStatus::Delivered);
        let report = queue.report(None).unwrap();
        assert_eq!(report.delivered_count, 1);
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        assert_ne!(ledger_report.chain_head, "genesis");
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn delivery_queue_records_adapter_readback_only_after_policy_and_confirmation() {
        let queue_path = temp_file("queue-gate");
        let ledger_path = temp_file("ledger-gate");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        queue
            .enqueue("channel-send", "telegram:chat", "hello", "idem-gate")
            .unwrap();
        let claimed = queue.claim_next("worker-a", Some(10_000)).unwrap();
        assert!(
            queue
                .record_adapter_readback(
                    &ledger,
                    &claimed.item.delivery_id,
                    "allow-send",
                    false,
                    "provider accepted message id 99",
                )
                .is_err()
        );
        assert!(
            queue
                .record_adapter_readback(
                    &ledger,
                    &claimed.item.delivery_id,
                    "deny-send",
                    true,
                    "provider accepted message id 99",
                )
                .is_err()
        );
        let report = queue
            .record_adapter_readback(
                &ledger,
                &claimed.item.delivery_id,
                "approved-send",
                true,
                "provider accepted message id 99",
            )
            .unwrap();
        assert_eq!(report.status, DeliveryQueueStatus::Delivered);
        assert!(report.operator_confirmed);
        assert!(!report.external_send_performed_by_gate);
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.delivered_count, 1);
        let evidence_report = ledger.report(None).unwrap();
        assert_eq!(evidence_report.evidence_count, 1);
        assert_eq!(
            queue_report.queue.items[0].readback_evidence_id.as_deref(),
            Some(report.readback_evidence_id.as_str())
        );
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn channel_send_handoff_requires_confirmation_and_records_readback() {
        let queue_path = temp_file("queue-handoff");
        let ledger_path = temp_file("ledger-handoff");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let base = ChannelSendHandoffInput {
            delivery_kind: "channel-send".into(),
            target: "telegram:chat".into(),
            payload_preview: "text=hello".into(),
            policy_decision: "allow-send".into(),
            operator_confirmed: false,
            idempotency_key: "handoff-1".into(),
        };
        assert!(
            queue
                .gated_channel_send_handoff(&ledger, base.clone())
                .is_err()
        );
        let denied = ChannelSendHandoffInput {
            operator_confirmed: true,
            policy_decision: "deny-send".into(),
            ..base.clone()
        };
        assert!(queue.gated_channel_send_handoff(&ledger, denied).is_err());

        let confirmed = ChannelSendHandoffInput {
            operator_confirmed: true,
            ..base
        };
        let report = queue
            .gated_channel_send_handoff(&ledger, confirmed.clone())
            .unwrap();
        assert_eq!(report.status, DeliveryQueueStatus::Queued);
        assert!(report.operator_confirmed);
        assert!(report.queue_mutated_by_gate);
        assert!(!report.external_send_performed_by_gate);
        assert!(report.readback_evidence_id.starts_with("rb-"));
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.queued_count, 1);
        assert_eq!(
            queue_report.queue.items[0].readback_evidence_id.as_deref(),
            Some(report.readback_evidence_id.as_str())
        );

        let duplicate = queue
            .gated_channel_send_handoff(&ledger, confirmed)
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.queue_mutated_by_gate);
        assert_eq!(duplicate.delivery_id, report.delivery_id);
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.queued_count, 1);
        let evidence_report = ledger.report(None).unwrap();
        assert_eq!(evidence_report.evidence_count, 2);
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn rich_delivery_handoff_accepts_rich_only_reply_and_preserves_context() {
        let queue_path = temp_file("queue-rich");
        let ledger_path = temp_file("ledger-rich");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        assert!(
            queue
                .gated_rich_delivery_handoff(
                    &ledger,
                    RichDeliveryHandoffInput {
                        delivery_kind: "rich-reply".into(),
                        target: "telegram:chat".into(),
                        text_preview: None,
                        rich_blocks: Vec::new(),
                        media_refs: Vec::new(),
                        channel_native_payload_preview: None,
                        source_reply_channel: None,
                        source_reply_message_id: Some("msg-1".into()),
                        active_ui_turn_id: None,
                        requester_session_key: None,
                        task_completion_id: None,
                        policy_decision: "allow-send".into(),
                        operator_confirmed: true,
                        idempotency_key: "rich-invalid".into(),
                    },
                )
                .is_err()
        );
        let report = queue
            .gated_rich_delivery_handoff(
                &ledger,
                RichDeliveryHandoffInput {
                    delivery_kind: "rich-reply".into(),
                    target: "telegram:chat".into(),
                    text_preview: None,
                    rich_blocks: vec!["button:Approve".into(), "select:route".into()],
                    media_refs: vec!["media://generated/image.png".into()],
                    channel_native_payload_preview: Some("{buttons:[approve]}".into()),
                    source_reply_channel: Some("telegram:chat".into()),
                    source_reply_message_id: Some("msg-1".into()),
                    active_ui_turn_id: Some("ui-turn-1".into()),
                    requester_session_key: Some("session-requester".into()),
                    task_completion_id: Some("task-42".into()),
                    policy_decision: "approved-send".into(),
                    operator_confirmed: true,
                    idempotency_key: "rich-handoff-1".into(),
                },
            )
            .unwrap();
        assert!(report.content_accepted_without_text);
        assert!(report.rich_content_present);
        assert!(report.channel_native_payload_present);
        assert!(report.source_reply_metadata_preserved);
        assert!(report.active_ui_turn_mirrored);
        assert!(report.task_completion_routed_to_requester);
        assert!(report.queue_mutated_by_gate);
        assert!(!report.external_send_performed_by_gate);
        let queue_report = queue.report(None).unwrap();
        let item = &queue_report.queue.items[0];
        assert!(item.rich_content_present);
        assert!(item.channel_native_payload_present);
        assert_eq!(item.source_reply_channel.as_deref(), Some("telegram:chat"));
        assert_eq!(item.source_reply_message_id.as_deref(), Some("msg-1"));
        assert_eq!(item.active_ui_turn_id.as_deref(), Some("ui-turn-1"));
        assert!(item.active_ui_turn_mirrored);
        assert_eq!(
            item.requester_session_key.as_deref(),
            Some("session-requester")
        );
        assert_eq!(item.task_completion_id.as_deref(), Some("task-42"));
        assert!(item.task_completion_routed_to_requester);
        assert!(item.payload_preview.contains("rich_blocks=2"));
        assert!(
            item.payload_preview
                .contains("source_reply=telegram:chat/msg-1")
        );
        assert!(item.payload_preview.contains("active_ui_turn=ui-turn-1"));
        assert!(
            item.payload_preview
                .contains("task_completion=task-42; requester_session=session-requester")
        );
        let evidence_report = ledger.report(None).unwrap();
        assert_eq!(evidence_report.evidence_count, 1);
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn rich_delivery_handoff_dedupes_existing_in_flight_item_without_new_evidence() {
        let queue_path = temp_file("queue-rich-dedupe");
        let ledger_path = temp_file("ledger-rich-dedupe");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let input = RichDeliveryHandoffInput {
            delivery_kind: "rich-reply".into(),
            target: "webchat:active".into(),
            text_preview: None,
            rich_blocks: vec!["presentation:block".into()],
            media_refs: Vec::new(),
            channel_native_payload_preview: None,
            source_reply_channel: Some("webchat:active".into()),
            source_reply_message_id: Some("turn-7".into()),
            active_ui_turn_id: Some("ui-turn-7".into()),
            requester_session_key: None,
            task_completion_id: None,
            policy_decision: "allow-send".into(),
            operator_confirmed: true,
            idempotency_key: "rich-inflight-idem".into(),
        };
        let first = queue
            .gated_rich_delivery_handoff(&ledger, input.clone())
            .unwrap();
        let claimed = queue.claim_next("delivery-worker", Some(10_000)).unwrap();
        assert_eq!(claimed.item.delivery_id, first.delivery_id);
        assert_eq!(claimed.item.status, DeliveryQueueStatus::InFlight);
        let duplicate = queue.gated_rich_delivery_handoff(&ledger, input).unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert_eq!(duplicate.status, DeliveryQueueStatus::InFlight);
        assert_eq!(duplicate.delivery_id, first.delivery_id);
        assert!(!duplicate.queue_mutated_by_gate);
        assert!(duplicate.active_ui_turn_mirrored);
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.in_flight_count, 1);
        assert_eq!(queue_report.queue.items.len(), 1);
        let evidence_report = ledger.report(None).unwrap();
        assert_eq!(evidence_report.evidence_count, 1);
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn delivery_queue_reclaims_stale_in_flight_items_for_retry() {
        let queue_path = temp_file("queue-reclaim");
        let queue = DurableDeliveryQueue::new(&queue_path);
        let enqueued = queue
            .enqueue("channel-send", "discord:chan", "payload", "idem-reclaim")
            .unwrap();
        let claimed = queue.claim_next("worker-a", Some(1_000)).unwrap();
        let reclaim = queue
            .reclaim_stale(Some(claimed.lease_expires_unix_ms.saturating_add(1)))
            .unwrap();
        assert_eq!(
            reclaim.reclaimed_delivery_ids,
            vec![enqueued.item.delivery_id]
        );
        let report = queue.report(None).unwrap();
        assert_eq!(report.failed_count, 1);
        let retried = queue.claim_next("worker-b", Some(1_000)).unwrap();
        assert_eq!(retried.item.attempt_count, 2);
        let _ = fs::remove_file(queue_path);
    }
}
