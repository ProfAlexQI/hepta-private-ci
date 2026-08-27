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
use crate::browser_contracts::BrowserOutcome;
use crate::browser_contracts::BrowserRequest;
use crate::browser_contracts::BrowserResponse;
use crate::browser_contracts::BrowserSessionId;
use crate::browser_contracts::BrowserWaitCondition;
use crate::browser_contracts::SemanticNode;
use crate::browser_contracts::SemanticRef;
use crate::browser_contracts::SemanticSnapshot;
use crate::browser_contracts::WebEvidenceReceipt;

use super::BrowserEngine;
use super::BrowserEngineError;
use super::MAX_CACHED_RESPONSES;
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

#[path = "actor_support.rs"]
mod support;

use support::increment;
use support::invalid;
use support::outcome_from_engine_error;
use support::truncate_utf8;
use support::valid_command_shape;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BrowserActorStatus {
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub owner_epoch: u64,
    pub page_revision: u64,
    pub mode: BrowserControlMode,
    pub human_lease_expires_at_ms: Option<u64>,
}

#[derive(Clone, Debug)]
struct CachedResponse {
    request_sha256: String,
    response: BrowserResponse,
}

#[derive(Debug)]
pub struct BrowserActor<E>
where
    E: BrowserEngine,
{
    session_id: BrowserSessionId,
    generation: u64,
    owner_epoch: u64,
    page_revision: u64,
    mode: BrowserControlMode,
    human_lease_expires_at_ms: Option<u64>,
    engine: E,
    active_refs: BTreeMap<SemanticRef, String>,
    responses: BTreeMap<u64, CachedResponse>,
}

impl<E> BrowserActor<E>
where
    E: BrowserEngine,
{
    pub fn new(
        session_id: BrowserSessionId,
        generation: u64,
        engine: E,
    ) -> Result<Self, QualificationError> {
        if generation == 0 {
            return Err(invalid("browser actor generation must be nonzero"));
        }
        Ok(Self {
            session_id,
            generation,
            owner_epoch: 1,
            page_revision: 0,
            mode: BrowserControlMode::AgentTurn,
            human_lease_expires_at_ms: None,
            engine,
            active_refs: BTreeMap::new(),
            responses: BTreeMap::new(),
        })
    }

    pub fn status(&self) -> BrowserActorStatus {
        BrowserActorStatus {
            session_id: self.session_id.clone(),
            generation: self.generation,
            owner_epoch: self.owner_epoch,
            page_revision: self.page_revision,
            mode: self.mode,
            human_lease_expires_at_ms: self.human_lease_expires_at_ms,
        }
    }

    pub fn handle(
        &mut self,
        request: BrowserRequest,
        now_ms: u64,
    ) -> Result<BrowserResponse, QualificationError> {
        self.expire_human_lease(now_ms)?;
        let request_sha256 = request.digest()?;
        if let Some(cached) = self.responses.get(&request.request_id) {
            if cached.request_sha256 == request_sha256 {
                return Ok(cached.response.clone());
            }
            return self.response(
                &request,
                request_sha256,
                BrowserOutcome::Denied {
                    code: BrowserDenialCode::RequestIdConflict,
                },
                self.owner_epoch,
                self.page_revision,
            );
        }
        if self.responses.len() >= MAX_CACHED_RESPONSES {
            return self.response(
                &request,
                request_sha256,
                BrowserOutcome::Denied {
                    code: BrowserDenialCode::ResourceLimit,
                },
                self.owner_epoch,
                self.page_revision,
            );
        }

        let validation = self.validate_request(&request);
        let response = match validation {
            Some(code) => self.response(
                &request,
                request_sha256.clone(),
                BrowserOutcome::Denied { code },
                self.owner_epoch,
                self.page_revision,
            )?,
            None => self.execute(&request, request_sha256.clone(), now_ms)?,
        };
        self.responses.insert(
            request.request_id,
            CachedResponse {
                request_sha256,
                response: response.clone(),
            },
        );
        Ok(response)
    }

    fn validate_request(&self, request: &BrowserRequest) -> Option<BrowserDenialCode> {
        if request.schema_version != BROWSER_CONTROL_SCHEMA_VERSION {
            Some(BrowserDenialCode::UnsupportedSchema)
        } else if request.request_id == 0 || !valid_command_shape(&request.command) {
            Some(BrowserDenialCode::InvalidCommand)
        } else if request.session_id != self.session_id {
            Some(BrowserDenialCode::WrongSession)
        } else if request.generation != self.generation {
            Some(BrowserDenialCode::StaleGeneration)
        } else if request.owner_epoch != self.owner_epoch {
            Some(BrowserDenialCode::StaleOwnerEpoch)
        } else if request.expected_page_revision != self.page_revision {
            Some(BrowserDenialCode::StalePageRevision)
        } else {
            None
        }
    }

    fn execute(
        &mut self,
        request: &BrowserRequest,
        request_sha256: String,
        now_ms: u64,
    ) -> Result<BrowserResponse, QualificationError> {
        let owner_epoch_before = self.owner_epoch;
        let page_revision_before = self.page_revision;
        let outcome = match &request.command {
            BrowserCommand::Navigate { url } => {
                if !self.agent_controls(request.actor) {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ControlNotOwned,
                    }
                } else {
                    match self.engine.navigate(url) {
                        Ok(()) => {
                            self.advance_page_revision()?;
                            BrowserOutcome::Applied {
                                command: request.command.kind(),
                            }
                        }
                        Err(error) => outcome_from_engine_error(error),
                    }
                }
            }
            BrowserCommand::Observe { max_nodes } => {
                if !(1..=MAX_OBSERVE_NODES).contains(max_nodes) {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ResourceLimit,
                    }
                } else {
                    match self.capture_snapshot(usize::from(*max_nodes), true) {
                        Ok(snapshot) => BrowserOutcome::Observed { snapshot },
                        Err(error) => outcome_from_engine_error(error),
                    }
                }
            }
            BrowserCommand::Act { target, action } => {
                if !self.agent_controls(request.actor) {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ControlNotOwned,
                    }
                } else {
                    self.apply_targeted_action(target, action)?
                }
            }
            BrowserCommand::Wait { condition } => match self.engine.wait(condition) {
                Ok(()) => BrowserOutcome::WaitSatisfied {
                    condition: condition.clone(),
                },
                Err(error) => outcome_from_engine_error(error),
            },
            BrowserCommand::Extract { query, max_bytes } => {
                if query.is_empty() || !(1..=MAX_EXTRACT_BYTES).contains(max_bytes) {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::InvalidCommand,
                    }
                } else {
                    match self.engine.extract(query) {
                        Ok(extracted) if extracted.raw_secret_bytes_present => {
                            BrowserOutcome::Denied {
                                code: BrowserDenialCode::SensitiveDataDenied,
                            }
                        }
                        Ok(extracted) if extracted.cross_tenant_data_present => {
                            BrowserOutcome::Denied {
                                code: BrowserDenialCode::CrossTenantDataDenied,
                            }
                        }
                        Ok(extracted) => {
                            let (value, truncated) =
                                truncate_utf8(extracted.value, *max_bytes as usize);
                            let snapshot_sha256 =
                                match self.capture_snapshot(MAX_ENGINE_NODES, false) {
                                    Ok(snapshot) => snapshot.snapshot_sha256,
                                    Err(error) => {
                                        return self.response(
                                            request,
                                            request_sha256,
                                            outcome_from_engine_error(error),
                                            owner_epoch_before,
                                            page_revision_before,
                                        );
                                    }
                                };
                            BrowserOutcome::Extracted {
                                query: query.clone(),
                                value,
                                truncated,
                                snapshot_sha256,
                            }
                        }
                        Err(error) => outcome_from_engine_error(error),
                    }
                }
            }
            BrowserCommand::HumanTakeControl { lease_ms } => {
                if request.actor != BrowserActorKind::Human
                    || self.mode != BrowserControlMode::AgentTurn
                {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ControlNotOwned,
                    }
                } else if !(1..=MAX_HUMAN_LEASE_MS).contains(lease_ms) {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ResourceLimit,
                    }
                } else {
                    self.owner_epoch = increment(self.owner_epoch, "browser owner epoch")?;
                    self.mode = BrowserControlMode::HumanTurn;
                    self.human_lease_expires_at_ms = Some(
                        now_ms
                            .checked_add(*lease_ms)
                            .ok_or_else(|| invalid("browser human lease expiry overflowed"))?,
                    );
                    BrowserOutcome::ControlTransferred { mode: self.mode }
                }
            }
            BrowserCommand::HumanReleaseControl => {
                if request.actor != BrowserActorKind::Human
                    || self.mode != BrowserControlMode::HumanTurn
                {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ControlNotOwned,
                    }
                } else {
                    self.owner_epoch = increment(self.owner_epoch, "browser owner epoch")?;
                    self.advance_page_revision()?;
                    self.mode = BrowserControlMode::AgentTurn;
                    self.human_lease_expires_at_ms = None;
                    BrowserOutcome::ControlTransferred { mode: self.mode }
                }
            }
            BrowserCommand::HumanInput { target, action } => {
                if request.actor != BrowserActorKind::Human
                    || self.mode != BrowserControlMode::HumanTurn
                    || self.human_lease_expires_at_ms.is_none()
                {
                    BrowserOutcome::Denied {
                        code: BrowserDenialCode::ControlNotOwned,
                    }
                } else {
                    self.apply_targeted_action(target, action)?
                }
            }
        };
        self.response(
            request,
            request_sha256,
            outcome,
            owner_epoch_before,
            page_revision_before,
        )
    }
}
