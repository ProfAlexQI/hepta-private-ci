use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::approval_broker::ApprovalBroker;
use crate::approval_broker::ApprovalBrokerRequestInput;
use crate::approval_broker::ApprovalBrokerResolveReport;
use crate::approval_broker::ApprovalBrokerStatus;
use crate::current_unix_ms;

pub const DEFAULT_TOOL_INVOCATION_LEDGER_PATH: &str = ".hepta/tool-invocation-ledger-v0.json";
pub const DEFAULT_TOOL_INVOCATION_LEDGER_ID: &str = "hepta-native-tool-invocation-ledger";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolInvocationStatus {
    Planned,
    RequiresApproval,
    Approved,
    Completed,
    Failed,
    Blocked,
}

impl ToolInvocationStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::RequiresApproval => "requires_approval",
            Self::Approved => "approved",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolInvocationLedgerFile {
    pub version: u32,
    pub ledger_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub invocations: Vec<ToolInvocationRecord>,
    #[serde(default)]
    pub events: Vec<ToolInvocationEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolInvocationRecord {
    pub invocation_id: String,
    pub tool_name: String,
    pub arguments_preview: String,
    pub policy_decision: String,
    pub idempotency_key: String,
    pub status: ToolInvocationStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_preview: Option<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolInvocationEvent {
    pub event_id: String,
    pub event_type: String,
    pub invocation_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolInvocationLedgerReport {
    pub ledger_path: String,
    pub ledger: ToolInvocationLedgerFile,
    pub planned_count: usize,
    pub requires_approval_count: usize,
    pub approved_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub blocked_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolInvocationPlanReport {
    pub ledger_path: String,
    pub invocation: ToolInvocationRecord,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolInvocationTransitionReport {
    pub ledger_path: String,
    pub invocation_id: String,
    pub status: ToolInvocationStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolInvocationApprovalRequestReport {
    pub ledger_path: String,
    pub broker_path: String,
    pub invocation_id: String,
    pub approval_id: String,
    pub persisted: bool,
}

pub struct ToolInvocationLedger {
    path: PathBuf,
}

impl ToolInvocationLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_TOOL_INVOCATION_LEDGER_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<ToolInvocationLedgerReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let ledger = self.load_or_default(now)?;
        Ok(ToolInvocationLedgerReport {
            ledger_path: self.path_display(),
            planned_count: count_status(&ledger, ToolInvocationStatus::Planned),
            requires_approval_count: count_status(&ledger, ToolInvocationStatus::RequiresApproval),
            approved_count: count_status(&ledger, ToolInvocationStatus::Approved),
            completed_count: count_status(&ledger, ToolInvocationStatus::Completed),
            failed_count: count_status(&ledger, ToolInvocationStatus::Failed),
            blocked_count: count_status(&ledger, ToolInvocationStatus::Blocked),
            persisted: self.path.exists(),
            ledger,
        })
    }

    pub fn plan_invocation(
        &self,
        tool_name: &str,
        arguments_preview: &str,
        policy_decision: &str,
        idempotency_key: &str,
    ) -> Result<ToolInvocationPlanReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let tool_name = normalize_tool_name(tool_name)?;
        let arguments_preview = normalize_non_empty(arguments_preview, "arguments preview")?;
        let policy_decision = normalize_non_empty(policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(idempotency_key, "idempotency key")?;
        if let Some(existing) = ledger
            .invocations
            .iter()
            .find(|invocation| invocation.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ToolInvocationPlanReport {
                ledger_path: self.path_display(),
                invocation: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let status = classify_policy(&policy_decision);
        let invocation_id = format!("toolinv-{}-{}", now, ledger.invocations.len() + 1);
        let invocation = ToolInvocationRecord {
            invocation_id: invocation_id.clone(),
            tool_name,
            arguments_preview: redact_preview(&arguments_preview),
            policy_decision,
            idempotency_key,
            status,
            approval_id: None,
            readback_evidence_id: None,
            result_preview: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        ledger.invocations.push(invocation.clone());
        push_event(
            &mut ledger,
            status.label(),
            &invocation_id,
            now,
            "tool invocation planned; no tool executed by ledger",
        );
        self.save(&mut ledger, now)?;
        Ok(ToolInvocationPlanReport {
            ledger_path: self.path_display(),
            invocation,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn attach_approval(
        &self,
        invocation_id: &str,
        approval_id: &str,
    ) -> Result<ToolInvocationTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let invocation_id = normalize_non_empty(invocation_id, "invocation id")?;
        let approval_id = normalize_non_empty(approval_id, "approval id")?;
        let invocation = find_invocation_mut(&mut ledger, &invocation_id)?;
        if invocation.status != ToolInvocationStatus::RequiresApproval {
            return Err(HeptaError(format!(
                "tool invocation {invocation_id} does not require approval; current status is {}",
                invocation.status.label()
            )));
        }
        invocation.status = ToolInvocationStatus::Approved;
        invocation.approval_id = Some(approval_id);
        invocation.updated_at_unix_ms = now;
        push_event(
            &mut ledger,
            "approved",
            &invocation_id,
            now,
            "tool invocation approved by ApprovalBroker readback",
        );
        self.save(&mut ledger, now)?;
        Ok(ToolInvocationTransitionReport {
            ledger_path: self.path_display(),
            invocation_id,
            status: ToolInvocationStatus::Approved,
            persisted: true,
        })
    }

    pub fn request_broker_approval(
        &self,
        broker: &ApprovalBroker,
        invocation_id: &str,
        expires_at_unix_ms: Option<u64>,
    ) -> Result<ToolInvocationApprovalRequestReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let invocation_id = normalize_non_empty(invocation_id, "invocation id")?;
        let (tool_name, arguments_preview, policy_decision, idempotency_key, existing_approval_id) = {
            let invocation = find_invocation_mut(&mut ledger, &invocation_id)?;
            if invocation.status != ToolInvocationStatus::RequiresApproval {
                return Err(HeptaError(format!(
                    "tool invocation {invocation_id} does not require broker approval; current status is {}",
                    invocation.status.label()
                )));
            }
            (
                invocation.tool_name.clone(),
                invocation.arguments_preview.clone(),
                invocation.policy_decision.clone(),
                invocation.idempotency_key.clone(),
                invocation.approval_id.clone(),
            )
        };
        if let Some(approval_id) = existing_approval_id {
            return Ok(ToolInvocationApprovalRequestReport {
                ledger_path: self.path_display(),
                broker_path: broker.path_display(),
                invocation_id,
                approval_id,
                persisted: self.path.exists(),
            });
        }
        let approval = broker.request_approval(ApprovalBrokerRequestInput {
            subject_kind: "tool_invocation".into(),
            subject_id: invocation_id.clone(),
            action_label: format!("tool:{tool_name}"),
            exact_payload_preview: arguments_preview,
            policy_decision,
            idempotency_key: format!("tool-approval:{idempotency_key}"),
            expires_at_unix_ms,
        })?;
        let approval_id = approval.request.approval_id;
        {
            let invocation = find_invocation_mut(&mut ledger, &invocation_id)?;
            invocation.approval_id = Some(approval_id.clone());
            invocation.updated_at_unix_ms = now;
        }
        push_event(
            &mut ledger,
            "approval_requested",
            &invocation_id,
            now,
            "tool invocation approval requested through ApprovalBroker",
        );
        self.save(&mut ledger, now)?;
        Ok(ToolInvocationApprovalRequestReport {
            ledger_path: self.path_display(),
            broker_path: broker.path_display(),
            invocation_id,
            approval_id,
            persisted: approval.persisted,
        })
    }

    pub fn attach_approved_broker_resolution(
        &self,
        invocation_id: &str,
        resolution: &ApprovalBrokerResolveReport,
    ) -> Result<ToolInvocationTransitionReport, HeptaError> {
        if resolution.status != ApprovalBrokerStatus::Approved {
            return Err(HeptaError(format!(
                "tool invocation approval {} is not approved; current status is {}",
                resolution.approval_id,
                resolution.status.label()
            )));
        }
        self.attach_approval(invocation_id, &resolution.approval_id)
    }

    pub fn finish_invocation(
        &self,
        invocation_id: &str,
        success: bool,
        readback_evidence_id: &str,
        result_preview: &str,
    ) -> Result<ToolInvocationTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let invocation_id = normalize_non_empty(invocation_id, "invocation id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let result_preview = normalize_non_empty(result_preview, "result preview")?;
        let invocation = find_invocation_mut(&mut ledger, &invocation_id)?;
        if !matches!(
            invocation.status,
            ToolInvocationStatus::Planned | ToolInvocationStatus::Approved
        ) {
            return Err(HeptaError(format!(
                "tool invocation {invocation_id} cannot finish from {}",
                invocation.status.label()
            )));
        }
        invocation.status = if success {
            ToolInvocationStatus::Completed
        } else {
            ToolInvocationStatus::Failed
        };
        invocation.readback_evidence_id = Some(readback_evidence_id);
        invocation.result_preview = Some(redact_preview(&result_preview));
        invocation.updated_at_unix_ms = now;
        let status = invocation.status;
        push_event(
            &mut ledger,
            status.label(),
            &invocation_id,
            now,
            "tool invocation finished with readback evidence",
        );
        self.save(&mut ledger, now)?;
        Ok(ToolInvocationTransitionReport {
            ledger_path: self.path_display(),
            invocation_id,
            status,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ToolInvocationLedgerFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ToolInvocationLedgerFile {
                version: 1,
                ledger_id: DEFAULT_TOOL_INVOCATION_LEDGER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                invocations: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read tool-invocation ledger {}: {err}",
                self.path.display()
            ))
        })?;
        let mut ledger: ToolInvocationLedgerFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse tool-invocation ledger {}: {err}",
                self.path.display()
            ))
        })?;
        if ledger.version != 1 {
            return Err(HeptaError(format!(
                "unsupported tool-invocation ledger version {} in {}",
                ledger.version,
                self.path.display()
            )));
        }
        ledger.events.truncate(1024);
        Ok(ledger)
    }

    fn save(
        &self,
        ledger: &mut ToolInvocationLedgerFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        ledger.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create tool-invocation ledger directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(ledger).map_err(|err| {
            HeptaError(format!("failed to serialize tool-invocation ledger: {err}"))
        })?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write tool-invocation ledger {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(ledger: &ToolInvocationLedgerFile, status: ToolInvocationStatus) -> usize {
    ledger
        .invocations
        .iter()
        .filter(|invocation| invocation.status == status)
        .count()
}

fn classify_policy(policy_decision: &str) -> ToolInvocationStatus {
    let lower = policy_decision.to_ascii_lowercase();
    if lower.contains("block") || lower.contains("deny") {
        ToolInvocationStatus::Blocked
    } else if lower.contains("approval") || lower.contains("confirm") {
        ToolInvocationStatus::RequiresApproval
    } else {
        ToolInvocationStatus::Planned
    }
}

fn find_invocation_mut<'a>(
    ledger: &'a mut ToolInvocationLedgerFile,
    invocation_id: &str,
) -> Result<&'a mut ToolInvocationRecord, HeptaError> {
    ledger
        .invocations
        .iter_mut()
        .find(|invocation| invocation.invocation_id == invocation_id)
        .ok_or_else(|| HeptaError(format!("tool invocation not found: {invocation_id}")))
}

fn normalize_tool_name(value: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, "tool name")?;
    if value.contains('/') || value.contains("..") || value.contains('\n') || value.contains('\r') {
        return Err(HeptaError(
            "tool name must be scoped and single-line".into(),
        ));
    }
    Ok(value)
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "tool invocation {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn redact_preview(value: &str) -> String {
    value
        .split_whitespace()
        .map(|part| {
            if part.len() > 48 || part.contains("token=") || part.contains("secret") {
                "<redacted>"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn push_event(
    ledger: &mut ToolInvocationLedgerFile,
    event_type: &str,
    invocation_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    ledger.events.push(ToolInvocationEvent {
        event_id: format!("toolevt-{}-{}", now_unix_ms, ledger.events.len() + 1),
        event_type: event_type.into(),
        invocation_id: invocation_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    ledger.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-tool-invocation-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn tool_invocation_ledger_plans_approval_and_completion() {
        let path = temp_file("approved");
        let ledger = ToolInvocationLedger::new(&path);
        let planned = ledger
            .plan_invocation(
                "exec",
                "command=cargo test token=secret",
                "requires approval",
                "tool-idem-1",
            )
            .unwrap();
        assert_eq!(
            planned.invocation.status,
            ToolInvocationStatus::RequiresApproval
        );
        assert!(planned.invocation.arguments_preview.contains("<redacted>"));
        let duplicate = ledger
            .plan_invocation(
                "exec",
                "command=cargo test",
                "requires approval",
                "tool-idem-1",
            )
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        ledger
            .attach_approval(&planned.invocation.invocation_id, "ap-1")
            .unwrap();
        let finished = ledger
            .finish_invocation(&planned.invocation.invocation_id, true, "rb-tool-1", "ok")
            .unwrap();
        assert_eq!(finished.status, ToolInvocationStatus::Completed);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.completed_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn tool_invocation_requests_and_attaches_broker_approval() {
        let ledger_path = temp_file("broker-ledger");
        let broker_path = temp_file("broker-approval");
        let ledger = ToolInvocationLedger::new(&ledger_path);
        let broker = ApprovalBroker::new(&broker_path);
        let planned = ledger
            .plan_invocation(
                "exec",
                "command=cargo test",
                "requires approval",
                "tool-idem-broker",
            )
            .unwrap();
        let request = ledger
            .request_broker_approval(&broker, &planned.invocation.invocation_id, None)
            .unwrap();
        assert_eq!(request.invocation_id, planned.invocation.invocation_id);
        assert!(request.approval_id.starts_with("ap-"));
        let duplicate_request = ledger
            .request_broker_approval(&broker, &planned.invocation.invocation_id, None)
            .unwrap();
        assert_eq!(duplicate_request.approval_id, request.approval_id);
        let rejected = broker
            .reject(&request.approval_id, "operator", "rb-reject", "not safe")
            .unwrap();
        assert!(
            ledger
                .attach_approved_broker_resolution(&planned.invocation.invocation_id, &rejected)
                .is_err()
        );

        let second = ledger
            .plan_invocation(
                "exec",
                "command=cargo fmt",
                "requires approval",
                "tool-idem-broker-2",
            )
            .unwrap();
        let second_request = ledger
            .request_broker_approval(&broker, &second.invocation.invocation_id, None)
            .unwrap();
        let approved = broker
            .approve(&second_request.approval_id, "operator", "rb-approve")
            .unwrap();
        let attached = ledger
            .attach_approved_broker_resolution(&second.invocation.invocation_id, &approved)
            .unwrap();
        assert_eq!(attached.status, ToolInvocationStatus::Approved);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.approved_count, 1);
        let _ = fs::remove_file(ledger_path);
        let _ = fs::remove_file(broker_path);
    }

    #[test]
    fn tool_invocation_ledger_blocks_denied_policy_and_bad_names() {
        let path = temp_file("blocked");
        let ledger = ToolInvocationLedger::new(&path);
        assert!(
            ledger
                .plan_invocation("../bad", "{}", "allow", "tool-idem-bad")
                .is_err()
        );
        let planned = ledger
            .plan_invocation(
                "message",
                "send preview",
                "deny external send",
                "tool-idem-2",
            )
            .unwrap();
        assert_eq!(planned.invocation.status, ToolInvocationStatus::Blocked);
        assert!(
            ledger
                .finish_invocation(
                    &planned.invocation.invocation_id,
                    true,
                    "rb",
                    "should not finish"
                )
                .is_err()
        );
        let _ = fs::remove_file(path);
    }
}
