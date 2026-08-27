use std::collections::BTreeMap;

use crate::QualificationError;
use crate::browser_contracts::BROWSER_CONTROL_SCHEMA_VERSION;
use crate::browser_contracts::BrowserAction;
use crate::browser_contracts::BrowserActivityReceipt;
use crate::browser_contracts::BrowserActorKind;
use crate::browser_contracts::BrowserAuthorityStatus;
use crate::browser_contracts::BrowserCommand;
use crate::browser_contracts::BrowserControlMode;
use crate::browser_contracts::BrowserDenialCode;
use crate::browser_contracts::BrowserIndeterminateCode;
use crate::browser_contracts::BrowserOutcome;
use crate::browser_contracts::BrowserRequest;
use crate::browser_contracts::BrowserResponse;
use crate::browser_contracts::BrowserWaitCondition;
use crate::browser_contracts::SemanticNode;
use crate::browser_contracts::SemanticRef;
use crate::browser_contracts::SemanticSnapshot;
use crate::browser_contracts::WebEvidenceReceipt;

use super::BrowserActor;
use super::BrowserEngine;
use super::BrowserEngineError;
use super::MAX_ENGINE_NODES;
use super::MAX_EXTRACT_BYTES;
use super::MAX_HUMAN_LEASE_MS;
use super::MAX_NODE_NAME_BYTES;
use super::MAX_NODE_ROLE_BYTES;
use super::MAX_NODE_VALUE_BYTES;
use super::MAX_OBSERVE_NODES;
use super::MAX_TITLE_BYTES;
use super::MAX_TYPED_TEXT_BYTES;
use super::MAX_URL_BYTES;

impl<E> BrowserActor<E>
where
    E: BrowserEngine,
{
    pub(super) fn apply_targeted_action(
        &mut self,
        target: &SemanticRef,
        action: &BrowserAction,
    ) -> Result<BrowserOutcome, QualificationError> {
        let Some(target_key) = self.active_refs.get(target).cloned() else {
            return Ok(BrowserOutcome::Denied {
                code: BrowserDenialCode::StaleSemanticRef,
            });
        };
        let outcome = match self.engine.act(&target_key, action) {
            Ok(()) => {
                self.advance_page_revision()?;
                BrowserOutcome::Applied {
                    command: if self.mode == BrowserControlMode::HumanTurn {
                        crate::browser_contracts::BrowserCommandKind::HumanInput
                    } else {
                        crate::browser_contracts::BrowserCommandKind::Act
                    },
                }
            }
            Err(error) => outcome_from_engine_error(error),
        };
        Ok(outcome)
    }

    pub(super) fn agent_controls(&self, actor: BrowserActorKind) -> bool {
        actor == BrowserActorKind::Agent && self.mode == BrowserControlMode::AgentTurn
    }

    pub(super) fn advance_page_revision(&mut self) -> Result<(), QualificationError> {
        self.page_revision = increment(self.page_revision, "browser page revision")?;
        self.active_refs.clear();
        Ok(())
    }

    pub(super) fn expire_human_lease(&mut self, now_ms: u64) -> Result<(), QualificationError> {
        let expired = self.mode == BrowserControlMode::HumanTurn
            && self
                .human_lease_expires_at_ms
                .is_some_and(|expires_at| now_ms >= expires_at);
        if expired {
            self.owner_epoch = increment(self.owner_epoch, "browser owner epoch")?;
            self.advance_page_revision()?;
            self.mode = BrowserControlMode::AgentTurn;
            self.human_lease_expires_at_ms = None;
        }
        Ok(())
    }

    pub(super) fn capture_snapshot(
        &mut self,
        max_nodes: usize,
        publish_refs: bool,
    ) -> Result<SemanticSnapshot, BrowserEngineError> {
        let snapshot = self.engine.snapshot()?;
        if snapshot.raw_secret_bytes_present {
            return Err(BrowserEngineError::Denied(
                BrowserDenialCode::SensitiveDataDenied,
            ));
        }
        if snapshot.cross_tenant_data_present {
            return Err(BrowserEngineError::Denied(
                BrowserDenialCode::CrossTenantDataDenied,
            ));
        }
        if snapshot.nodes.len() > MAX_ENGINE_NODES
            || snapshot.url.is_empty()
            || snapshot.url.len() > MAX_URL_BYTES
            || snapshot.title.len() > MAX_TITLE_BYTES
        {
            return Err(BrowserEngineError::Denied(
                BrowserDenialCode::ResourceLimit,
            ));
        }
        let mut refs = BTreeMap::new();
        let mut nodes = Vec::new();
        for node in snapshot.nodes.into_iter().take(max_nodes) {
            if node.role.is_empty()
                || node.role.len() > MAX_NODE_ROLE_BYTES
                || node.name.len() > MAX_NODE_NAME_BYTES
                || node.value.len() > MAX_NODE_VALUE_BYTES
            {
                return Err(BrowserEngineError::Denied(
                    BrowserDenialCode::ResourceLimit,
                ));
            }
            let semantic_ref = SemanticRef::derive(
                &self.session_id,
                self.page_revision,
                &node.key,
            )
            .map_err(|_| BrowserEngineError::Denied(BrowserDenialCode::InvalidCommand))?;
            if refs.insert(semantic_ref.clone(), node.key).is_some() {
                return Err(BrowserEngineError::Denied(
                    BrowserDenialCode::InvalidCommand,
                ));
            }
            nodes.push(SemanticNode {
                semantic_ref,
                role: node.role,
                name: node.name,
                value: node.value,
                interactive: node.interactive,
            });
        }
        if publish_refs {
            self.active_refs = refs;
        }
        SemanticSnapshot::seal(snapshot.url, snapshot.title, self.page_revision, nodes).map_err(
            |_| BrowserEngineError::Indeterminate(BrowserIndeterminateCode::RendererFailure),
        )
    }

    pub(super) fn response(
        &mut self,
        request: &BrowserRequest,
        request_sha256: String,
        outcome: BrowserOutcome,
        owner_epoch_before: u64,
        page_revision_before: u64,
    ) -> Result<BrowserResponse, QualificationError> {
        let evidence_receipt = match &outcome {
            BrowserOutcome::Observed { snapshot } => Some(WebEvidenceReceipt::seal(
                &self.session_id,
                self.generation,
                self.owner_epoch,
                snapshot,
            )?),
            BrowserOutcome::Extracted { .. }
            | BrowserOutcome::Applied { .. }
            | BrowserOutcome::WaitSatisfied { .. }
            | BrowserOutcome::ControlTransferred { .. } => {
                match self.capture_snapshot(MAX_ENGINE_NODES, false) {
                    Ok(snapshot) => Some(WebEvidenceReceipt::seal(
                        &self.session_id,
                        self.generation,
                        self.owner_epoch,
                        &snapshot,
                    )?),
                    Err(_) => None,
                }
            }
            BrowserOutcome::Denied { .. }
            | BrowserOutcome::Challenge { .. }
            | BrowserOutcome::Indeterminate { .. } => None,
        };
        let activity_receipt = BrowserActivityReceipt::seal(
            request,
            &self.session_id,
            self.generation,
            request_sha256,
            &outcome,
            owner_epoch_before,
            self.owner_epoch,
            page_revision_before,
            self.page_revision,
        )?;
        Ok(BrowserResponse {
            schema_version: BROWSER_CONTROL_SCHEMA_VERSION,
            request_id: request.request_id,
            session_id: self.session_id.clone(),
            generation: self.generation,
            owner_epoch: self.owner_epoch,
            page_revision: self.page_revision,
            mode: self.mode,
            outcome,
            authority: BrowserAuthorityStatus::default(),
            activity_receipt,
            evidence_receipt,
        })
    }
}

pub(super) fn valid_command_shape(command: &BrowserCommand) -> bool {
    match command {
        BrowserCommand::Navigate { url } => (1..=MAX_URL_BYTES).contains(&url.len()),
        BrowserCommand::Observe { max_nodes } => (1..=MAX_OBSERVE_NODES).contains(max_nodes),
        BrowserCommand::Act { action, .. } | BrowserCommand::HumanInput { action, .. } => {
            valid_action_shape(action)
        }
        BrowserCommand::Wait { condition } => match condition {
            BrowserWaitCondition::DocumentReady => true,
            BrowserWaitCondition::TextContains { text } => {
                (1..=MAX_TYPED_TEXT_BYTES).contains(&text.len())
            }
            BrowserWaitCondition::HistoryLengthAtLeast { length } => *length > 0,
        },
        BrowserCommand::Extract { query, max_bytes } => {
            (1..=512).contains(&query.len()) && (1..=MAX_EXTRACT_BYTES).contains(max_bytes)
        }
        BrowserCommand::HumanTakeControl { lease_ms } => {
            (1..=MAX_HUMAN_LEASE_MS).contains(lease_ms)
        }
        BrowserCommand::HumanReleaseControl => true,
    }
}

pub(super) fn valid_action_shape(action: &BrowserAction) -> bool {
    match action {
        BrowserAction::Click | BrowserAction::Clear => true,
        BrowserAction::TypeText { text } => {
            (1..=MAX_TYPED_TEXT_BYTES).contains(&text.len())
        }
    }
}

pub(super) fn outcome_from_engine_error(error: BrowserEngineError) -> BrowserOutcome {
    match error {
        BrowserEngineError::Denied(code) => BrowserOutcome::Denied { code },
        BrowserEngineError::Challenge(code) => BrowserOutcome::Challenge { code },
        BrowserEngineError::Indeterminate(code) => BrowserOutcome::Indeterminate { code },
    }
}

pub(super) fn increment(value: u64, description: &str) -> Result<u64, QualificationError> {
    value
        .checked_add(1)
        .ok_or_else(|| invalid(format!("{description} overflowed")))
}

pub(super) fn truncate_utf8(mut value: String, max_bytes: usize) -> (String, bool) {
    if value.len() <= max_bytes {
        return (value, false);
    }
    let mut boundary = max_bytes;
    while !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    value.truncate(boundary);
    (value, true)
}

pub(super) fn invalid(message: impl Into<String>) -> QualificationError {
    QualificationError::Invalid(message.into())
}
