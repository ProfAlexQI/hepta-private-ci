use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;

pub const DEFAULT_APPROVAL_BROKER_PATH: &str = ".hepta/approval-broker-v0.json";
pub const DEFAULT_APPROVAL_BROKER_ID: &str = "hepta-native-approval-broker";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalBrokerStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Superseded,
}

impl ApprovalBrokerStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalBrokerFile {
    pub version: u32,
    pub broker_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub requests: Vec<ApprovalBrokerRequest>,
    #[serde(default)]
    pub events: Vec<ApprovalBrokerEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalBrokerRequest {
    pub approval_id: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub action_label: String,
    pub exact_payload_preview: String,
    pub policy_decision: String,
    pub idempotency_key: String,
    pub status: ApprovalBrokerStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at_unix_ms: Option<u64>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalBrokerEvent {
    pub event_id: String,
    pub event_type: String,
    pub approval_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalBrokerReport {
    pub broker_path: String,
    pub broker: ApprovalBrokerFile,
    pub pending_count: usize,
    pub approved_count: usize,
    pub rejected_count: usize,
    pub expired_count: usize,
    pub superseded_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalBrokerRequestReport {
    pub broker_path: String,
    pub request: ApprovalBrokerRequest,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalBrokerResolveReport {
    pub broker_path: String,
    pub approval_id: String,
    pub status: ApprovalBrokerStatus,
    pub operator_id: String,
    pub readback_evidence_id: String,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalBrokerExpireReport {
    pub broker_path: String,
    pub now_unix_ms: u64,
    pub expired_count: usize,
    pub expired_approval_ids: Vec<String>,
    pub persisted: bool,
}

pub struct ApprovalBroker {
    path: PathBuf,
}

impl ApprovalBroker {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_APPROVAL_BROKER_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<ApprovalBrokerReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let broker = self.load_or_default(now)?;
        Ok(ApprovalBrokerReport {
            broker_path: self.path_display(),
            pending_count: count_status(&broker, ApprovalBrokerStatus::Pending),
            approved_count: count_status(&broker, ApprovalBrokerStatus::Approved),
            rejected_count: count_status(&broker, ApprovalBrokerStatus::Rejected),
            expired_count: count_status(&broker, ApprovalBrokerStatus::Expired),
            superseded_count: count_status(&broker, ApprovalBrokerStatus::Superseded),
            persisted: self.path.exists(),
            broker,
        })
    }

    pub fn request_approval(
        &self,
        input: ApprovalBrokerRequestInput,
    ) -> Result<ApprovalBrokerRequestReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut broker = self.load_or_default(now)?;
        let subject_kind = normalize_non_empty(&input.subject_kind, "subject kind")?;
        let subject_id = normalize_non_empty(&input.subject_id, "subject id")?;
        let action_label = normalize_non_empty(&input.action_label, "action label")?;
        let exact_payload_preview =
            normalize_non_empty(&input.exact_payload_preview, "exact payload preview")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if let Some(existing) = broker
            .requests
            .iter()
            .find(|request| request.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ApprovalBrokerRequestReport {
                broker_path: self.path_display(),
                request: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let approval_id = format!("ap-{}-{}", now, broker.requests.len() + 1);
        let request = ApprovalBrokerRequest {
            approval_id: approval_id.clone(),
            subject_kind,
            subject_id,
            action_label,
            exact_payload_preview,
            policy_decision,
            idempotency_key,
            status: ApprovalBrokerStatus::Pending,
            operator_id: None,
            resolution_reason: None,
            readback_evidence_id: None,
            expires_at_unix_ms: input.expires_at_unix_ms,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        broker.requests.push(request.clone());
        push_approval_event(
            &mut broker,
            "approval_requested",
            &approval_id,
            now,
            "approval requested with exact payload preview",
        );
        self.save(&mut broker, now)?;
        Ok(ApprovalBrokerRequestReport {
            broker_path: self.path_display(),
            request,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn approve(
        &self,
        approval_id: &str,
        operator_id: &str,
        readback_evidence_id: &str,
    ) -> Result<ApprovalBrokerResolveReport, HeptaError> {
        self.resolve(
            approval_id,
            operator_id,
            readback_evidence_id,
            ApprovalBrokerStatus::Approved,
            "operator approved exact payload",
        )
    }

    pub fn reject(
        &self,
        approval_id: &str,
        operator_id: &str,
        readback_evidence_id: &str,
        reason: &str,
    ) -> Result<ApprovalBrokerResolveReport, HeptaError> {
        self.resolve(
            approval_id,
            operator_id,
            readback_evidence_id,
            ApprovalBrokerStatus::Rejected,
            reason,
        )
    }

    pub fn expire_due(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<ApprovalBrokerExpireReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let mut broker = self.load_or_default(now)?;
        let mut expired_approval_ids = Vec::new();
        for request in &mut broker.requests {
            if request.status == ApprovalBrokerStatus::Pending
                && request
                    .expires_at_unix_ms
                    .is_some_and(|expires_at| expires_at <= now)
            {
                request.status = ApprovalBrokerStatus::Expired;
                request.updated_at_unix_ms = now;
                request.resolution_reason =
                    Some("approval expired before operator confirmation".into());
                expired_approval_ids.push(request.approval_id.clone());
            }
        }
        for approval_id in &expired_approval_ids {
            push_approval_event(
                &mut broker,
                "approval_expired",
                approval_id,
                now,
                "approval expired before operator confirmation",
            );
        }
        self.save(&mut broker, now)?;
        Ok(ApprovalBrokerExpireReport {
            broker_path: self.path_display(),
            now_unix_ms: now,
            expired_count: expired_approval_ids.len(),
            expired_approval_ids,
            persisted: true,
        })
    }

    fn resolve(
        &self,
        approval_id: &str,
        operator_id: &str,
        readback_evidence_id: &str,
        status: ApprovalBrokerStatus,
        reason: &str,
    ) -> Result<ApprovalBrokerResolveReport, HeptaError> {
        if !matches!(
            status,
            ApprovalBrokerStatus::Approved | ApprovalBrokerStatus::Rejected
        ) {
            return Err(HeptaError(
                "approval broker can only resolve approve/reject".into(),
            ));
        }
        let now = current_unix_ms()?;
        let mut broker = self.load_or_default(now)?;
        let approval_id = normalize_non_empty(approval_id, "approval id")?;
        let operator_id = normalize_non_empty(operator_id, "operator id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let reason = normalize_non_empty(reason, "resolution reason")?;
        let request = broker
            .requests
            .iter_mut()
            .find(|request| request.approval_id == approval_id)
            .ok_or_else(|| HeptaError(format!("approval request not found: {approval_id}")))?;
        if request.status != ApprovalBrokerStatus::Pending {
            return Err(HeptaError(format!(
                "approval request {approval_id} is not pending; current status is {}",
                request.status.label()
            )));
        }
        request.status = status;
        request.operator_id = Some(operator_id.clone());
        request.resolution_reason = Some(reason);
        request.readback_evidence_id = Some(readback_evidence_id.clone());
        request.updated_at_unix_ms = now;
        push_approval_event(
            &mut broker,
            status.label(),
            &approval_id,
            now,
            "approval resolved with readback evidence",
        );
        self.save(&mut broker, now)?;
        Ok(ApprovalBrokerResolveReport {
            broker_path: self.path_display(),
            approval_id,
            status,
            operator_id,
            readback_evidence_id,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ApprovalBrokerFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ApprovalBrokerFile {
                version: 1,
                broker_id: DEFAULT_APPROVAL_BROKER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                requests: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read approval-broker {}: {err}",
                self.path.display()
            ))
        })?;
        let mut broker: ApprovalBrokerFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse approval-broker {}: {err}",
                self.path.display()
            ))
        })?;
        if broker.version != 1 {
            return Err(HeptaError(format!(
                "unsupported approval-broker version {} in {}",
                broker.version,
                self.path.display()
            )));
        }
        broker.events.truncate(1024);
        Ok(broker)
    }

    fn save(&self, broker: &mut ApprovalBrokerFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        broker.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create approval-broker directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(broker)
            .map_err(|err| HeptaError(format!("failed to serialize approval-broker: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write approval-broker {}: {err}",
                self.path.display()
            ))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalBrokerRequestInput {
    pub subject_kind: String,
    pub subject_id: String,
    pub action_label: String,
    pub exact_payload_preview: String,
    pub policy_decision: String,
    pub idempotency_key: String,
    pub expires_at_unix_ms: Option<u64>,
}

fn count_status(broker: &ApprovalBrokerFile, status: ApprovalBrokerStatus) -> usize {
    broker
        .requests
        .iter()
        .filter(|request| request.status == status)
        .count()
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "approval broker {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn push_approval_event(
    broker: &mut ApprovalBrokerFile,
    event_type: &str,
    approval_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    broker.events.push(ApprovalBrokerEvent {
        event_id: format!("apevt-{}-{}", now_unix_ms, broker.events.len() + 1),
        event_type: event_type.into(),
        approval_id: approval_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    broker.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-approval-broker-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    fn sample_input(idempotency_key: &str) -> ApprovalBrokerRequestInput {
        ApprovalBrokerRequestInput {
            subject_kind: "tool_call".into(),
            subject_id: "tool-call-1".into(),
            action_label: "run local command".into(),
            exact_payload_preview: "exec: cargo test -q -p hepta-runtime".into(),
            policy_decision: "requires_operator_confirmation".into(),
            idempotency_key: idempotency_key.into(),
            expires_at_unix_ms: None,
        }
    }

    #[test]
    fn approval_broker_requests_and_resolves_with_readback_evidence() {
        let path = temp_file("approve");
        let broker = ApprovalBroker::new(&path);
        let request = broker
            .request_approval(sample_input("idem-approval-1"))
            .unwrap();
        assert_eq!(request.request.status, ApprovalBrokerStatus::Pending);
        assert!(request.request.exact_payload_preview.contains("cargo test"));
        let duplicate = broker
            .request_approval(sample_input("idem-approval-1"))
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        let approved = broker
            .approve(&request.request.approval_id, "operator-a", "rb-approval-1")
            .unwrap();
        assert_eq!(approved.status, ApprovalBrokerStatus::Approved);
        let report = broker.report(None).unwrap();
        assert_eq!(report.approved_count, 1);
        assert_eq!(report.pending_count, 0);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn approval_broker_rejects_expired_or_non_pending_resolution() {
        let path = temp_file("expire");
        let broker = ApprovalBroker::new(&path);
        let mut input = sample_input("idem-expire-1");
        input.expires_at_unix_ms = Some(current_unix_ms().unwrap_or(0));
        let request = broker.request_approval(input).unwrap();
        let expiry = broker
            .expire_due(Some(request.request.expires_at_unix_ms.unwrap_or(0)))
            .unwrap();
        assert_eq!(expiry.expired_count, 1);
        assert!(
            broker
                .approve(
                    &request.request.approval_id,
                    "operator-a",
                    "rb-after-expiry"
                )
                .is_err()
        );
        let report = broker.report(None).unwrap();
        assert_eq!(report.expired_count, 1);
        let _ = fs::remove_file(path);
    }
}
