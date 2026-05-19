use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

/// Durable, runtime-owned session lifecycle state machine for Hepta-style
/// `/new`, `/reset`, `/compact`, `/focus`, and `/unfocus` flows.
///
/// The state machine is intentionally side-effect-light: it mutates only this
/// explicit lifecycle state object, records checkpoint requirements, and never
/// calls a model provider, channel adapter, hook queue, or external process.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableSessionLifecyclePlane {
    pub active_session_id: Option<String>,
    pub reset_generation: u64,
    pub focus_target: Option<String>,
    pub checkpoint_count: u64,
    pub compacted_turn_count: u64,
    #[serde(default)]
    pub events: Vec<SessionLifecycleEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleCommand {
    New,
    Reset,
    Compact,
    Focus,
    Unfocus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionLifecycleEvent {
    pub command: SessionLifecycleCommand,
    pub confirmed: bool,
    pub applied: bool,
    pub checkpoint_required: bool,
    pub provider_call_performed: bool,
    pub channel_send_performed: bool,
    pub persistent_store_touched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionLifecycleOperationReport {
    pub command: SessionLifecycleCommand,
    pub confirmed: bool,
    pub applied: bool,
    pub active_session_id: Option<String>,
    pub focus_target: Option<String>,
    pub checkpoint_count: u64,
    pub compacted_turn_count: u64,
    pub provider_call_performed: bool,
    pub channel_send_performed: bool,
    pub hook_enqueued: bool,
    pub external_side_effects: bool,
}

impl DurableSessionLifecyclePlane {
    pub fn apply(
        &mut self,
        command: SessionLifecycleCommand,
        argument: Option<&str>,
        confirmed: bool,
    ) -> Result<SessionLifecycleOperationReport, HeptaError> {
        let mut applied = false;
        let checkpoint_required = matches!(
            command,
            SessionLifecycleCommand::Reset | SessionLifecycleCommand::Compact
        );

        if confirmed {
            match command {
                SessionLifecycleCommand::New => {
                    let session_id = argument
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .unwrap_or("session-new");
                    validate_lifecycle_arg(session_id, "session id")?;
                    self.active_session_id = Some(session_id.to_string());
                    applied = true;
                }
                SessionLifecycleCommand::Reset => {
                    self.reset_generation = self.reset_generation.saturating_add(1);
                    self.checkpoint_count = self.checkpoint_count.saturating_add(1);
                    applied = true;
                }
                SessionLifecycleCommand::Compact => {
                    self.checkpoint_count = self.checkpoint_count.saturating_add(1);
                    self.compacted_turn_count = self.compacted_turn_count.saturating_add(3);
                    applied = true;
                }
                SessionLifecycleCommand::Focus => {
                    let target = argument
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .ok_or_else(|| HeptaError("focus target must not be empty".into()))?;
                    validate_lifecycle_arg(target, "focus target")?;
                    self.focus_target = Some(target.to_string());
                    applied = true;
                }
                SessionLifecycleCommand::Unfocus => {
                    self.focus_target = None;
                    applied = true;
                }
            }
        }

        self.events.push(SessionLifecycleEvent {
            command,
            confirmed,
            applied,
            checkpoint_required,
            provider_call_performed: false,
            channel_send_performed: false,
            persistent_store_touched: false,
        });

        Ok(SessionLifecycleOperationReport {
            command,
            confirmed,
            applied,
            active_session_id: self.active_session_id.clone(),
            focus_target: self.focus_target.clone(),
            checkpoint_count: self.checkpoint_count,
            compacted_turn_count: self.compacted_turn_count,
            provider_call_performed: false,
            channel_send_performed: false,
            hook_enqueued: false,
            external_side_effects: false,
        })
    }

    pub fn apply_sequence(
        &mut self,
        confirmed: bool,
    ) -> Result<Vec<SessionLifecycleOperationReport>, HeptaError> {
        let operations = [
            (SessionLifecycleCommand::New, Some("session-durable")),
            (
                SessionLifecycleCommand::Focus,
                Some("telegram:topic:durable"),
            ),
            (SessionLifecycleCommand::Compact, None),
            (SessionLifecycleCommand::Reset, None),
            (SessionLifecycleCommand::Unfocus, None),
        ];
        operations
            .into_iter()
            .map(|(command, argument)| self.apply(command, argument, confirmed))
            .collect()
    }

    pub fn ready(&self) -> bool {
        self.events.iter().all(|event| {
            !event.provider_call_performed
                && !event.channel_send_performed
                && !event.persistent_store_touched
        })
    }
}

fn validate_lifecycle_arg(value: &str, label: &str) -> Result<(), HeptaError> {
    if value.contains('\n') || value.contains('\r') {
        return Err(HeptaError(format!("{label} must be single-line")));
    }
    if value.contains("..") {
        return Err(HeptaError(format!(
            "{label} must not contain parent traversal"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_commands_require_confirmation_before_mutation() {
        let mut plane = DurableSessionLifecyclePlane::default();
        let report = plane
            .apply(SessionLifecycleCommand::New, Some("session-1"), false)
            .expect("unconfirmed command should report without mutating");

        assert!(!report.applied);
        assert!(plane.active_session_id.is_none());
        assert_eq!(plane.events.len(), 1);
        assert!(plane.ready());
    }

    #[test]
    fn lifecycle_sequence_records_checkpoints_without_external_side_effects() {
        let mut plane = DurableSessionLifecyclePlane::default();
        let reports = plane
            .apply_sequence(true)
            .expect("confirmed lifecycle sample should apply");

        assert_eq!(reports.len(), 5);
        assert_eq!(plane.active_session_id.as_deref(), Some("session-durable"));
        assert!(plane.focus_target.is_none());
        assert_eq!(plane.checkpoint_count, 2);
        assert_eq!(plane.compacted_turn_count, 3);
        assert!(reports.iter().all(|report| !report.external_side_effects));
        assert!(plane.ready());
    }

    #[test]
    fn lifecycle_rejects_multiline_or_traversing_targets() {
        let mut plane = DurableSessionLifecyclePlane::default();
        assert!(
            plane
                .apply(SessionLifecycleCommand::Focus, Some("bad\ntarget"), true)
                .is_err()
        );
        assert!(
            plane
                .apply(SessionLifecycleCommand::New, Some("../session"), true)
                .is_err()
        );
    }
}
