use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use hepta_contracts::ContentHash;
use hepta_intelligence::HardFeasibilityMask;
use hepta_intelligence::NduH1ShadowConfig;
use hepta_intelligence::NduH1ShadowRequest;
use hepta_intelligence::NduH1ShadowService;
use hepta_intelligence::NduH1ShadowServiceError;
use hepta_intelligence::NduH1ShadowServiceResult;
use hepta_intelligence::NduShadowObservation;
use hepta_intelligence::NduUtilityEventRef;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct NduH1Runtime {
    state: Arc<Mutex<NduH1RuntimeState>>,
}

#[derive(Debug)]
struct NduH1RuntimeState {
    config: NduH1ShadowConfig,
    service: NduH1ShadowService,
    recorded_count: u64,
    replay_count: u64,
    rejected_count: u64,
    last_error: Option<String>,
    kill_switch_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduH1ShadowEvent {
    pub event_hash: ContentHash,
    pub source_receipt_hash: ContentHash,
    pub subject_pseudonym_hash: ContentHash,
    pub explicit_preference_evidence_hash: Option<ContentHash>,
    pub task_signal_basis_points: i32,
    pub learning_signal_basis_points: i32,
    pub trust_signal_basis_points: i32,
    pub memory_pollution_risk_basis_points: i32,
    pub resource_cost_basis_points: i32,
    pub uncertainty_basis_points: i32,
    pub propensity_basis_points: u16,
    pub delayed_outcome_hash: Option<ContentHash>,
    pub feasibility: HardFeasibilityMask,
}

impl NduH1Runtime {
    pub fn open(
        config: NduH1ShadowConfig,
        journal_path: impl AsRef<Path>,
    ) -> Result<Self, NduH1RuntimeError> {
        Self::open_with_kill_switch(config, journal_path, None)
    }

    pub fn open_with_kill_switch(
        config: NduH1ShadowConfig,
        journal_path: impl AsRef<Path>,
        kill_switch_path: Option<PathBuf>,
    ) -> Result<Self, NduH1RuntimeError> {
        let service = NduH1ShadowService::open(config.clone(), journal_path)
            .map_err(NduH1RuntimeError::Service)?;
        Ok(Self {
            state: Arc::new(Mutex::new(NduH1RuntimeState {
                config,
                service,
                recorded_count: 0,
                replay_count: 0,
                rejected_count: 0,
                last_error: None,
                kill_switch_path,
            })),
        })
    }

    pub fn observe_event(
        &self,
        event: NduH1ShadowEvent,
    ) -> Result<NduH1ShadowServiceResult, NduH1RuntimeError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| NduH1RuntimeError::WriterPoisoned)?;
        if state.kill_switch_active() {
            state.rejected_count += 1;
            state.last_error = Some("kill_switch_active".to_owned());
            return Err(NduH1RuntimeError::KillSwitchActive);
        }
        let request = NduH1ShadowRequest::new(
            state.config.tenant_scope_hash().clone(),
            state.config.consent_scope_hash().clone(),
            state.config.revocation_snapshot_hash().clone(),
            NduShadowObservation::new(
                NduUtilityEventRef::new(
                    event.event_hash,
                    event.source_receipt_hash,
                    event.subject_pseudonym_hash,
                    event.explicit_preference_evidence_hash,
                ),
                event.task_signal_basis_points,
                event.learning_signal_basis_points,
                event.trust_signal_basis_points,
                event.memory_pollution_risk_basis_points,
                event.resource_cost_basis_points,
                event.uncertainty_basis_points,
                event.propensity_basis_points,
                event.delayed_outcome_hash,
            ),
            event.feasibility,
        );
        match state.service.observe(request) {
            Ok(result @ NduH1ShadowServiceResult::Recorded(_)) => {
                state.recorded_count += 1;
                state.last_error = None;
                Ok(result)
            }
            Ok(result @ NduH1ShadowServiceResult::AlreadyObserved { .. }) => {
                state.replay_count += 1;
                state.last_error = None;
                Ok(result)
            }
            Err(error) => {
                state.rejected_count += 1;
                state.last_error = Some(format!("{error:?}"));
                Err(NduH1RuntimeError::Service(error))
            }
        }
    }

    pub fn status(&self) -> Result<NduH1RuntimeStatus, NduH1RuntimeError> {
        let state = self
            .state
            .lock()
            .map_err(|_| NduH1RuntimeError::WriterPoisoned)?;
        let kill_switch_active = state.kill_switch_active();
        Ok(NduH1RuntimeStatus {
            schema: "hepta_ndu_h1_runtime_status_v1",
            ready: state.last_error.is_none() && !kill_switch_active,
            accepting_observations: !kill_switch_active,
            kill_switch_active,
            shadow_only: true,
            production_authority_granted: false,
            observed_event_count: state.service.journal().record_count(),
            recorded_count: state.recorded_count,
            replay_count: state.replay_count,
            rejected_count: state.rejected_count,
            journal_head: state.service.journal().head().as_str().to_owned(),
            last_error: state.last_error.clone(),
        })
    }
}

impl NduH1RuntimeState {
    fn kill_switch_active(&self) -> bool {
        self.kill_switch_path
            .as_ref()
            .is_some_and(|path| path.try_exists().unwrap_or(true))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NduH1RuntimeStatus {
    pub schema: &'static str,
    pub ready: bool,
    pub accepting_observations: bool,
    pub kill_switch_active: bool,
    pub shadow_only: bool,
    pub production_authority_granted: bool,
    pub observed_event_count: u64,
    pub recorded_count: u64,
    pub replay_count: u64,
    pub rejected_count: u64,
    pub journal_head: String,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduH1RuntimeError {
    Service(NduH1ShadowServiceError),
    WriterPoisoned,
    KillSwitchActive,
}

#[cfg(test)]
mod tests {
    use hepta_intelligence::HardFeasibilityVerdict;
    use tempfile::tempdir;

    use super::*;

    fn hash(value: &str) -> ContentHash {
        ContentHash::new(value)
    }

    fn config() -> NduH1ShadowConfig {
        NduH1ShadowConfig::new(
            hash("tenant"),
            hash("consent"),
            hash("revocation"),
            hash("model"),
            hash("config"),
            hash("initial"),
            10,
            true,
        )
    }

    fn event(event: &str) -> NduH1ShadowEvent {
        NduH1ShadowEvent {
            event_hash: hash(event),
            source_receipt_hash: hash("receipt"),
            subject_pseudonym_hash: hash("subject"),
            explicit_preference_evidence_hash: None,
            task_signal_basis_points: 1_000,
            learning_signal_basis_points: 500,
            trust_signal_basis_points: 750,
            memory_pollution_risk_basis_points: 100,
            resource_cost_basis_points: 300,
            uncertainty_basis_points: 200,
            propensity_basis_points: 5_000,
            delayed_outcome_hash: None,
            feasibility: HardFeasibilityMask::new(
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
            ),
        }
    }

    #[test]
    fn runtime_is_single_writer_observable_and_shadow_only() {
        let directory = tempdir().unwrap();
        let runtime = NduH1Runtime::open(config(), directory.path().join("ndu.jsonl")).unwrap();
        runtime.observe_event(event("event-1")).unwrap();
        runtime.observe_event(event("event-1")).unwrap();

        let status = runtime.status().unwrap();
        assert!(status.ready);
        assert!(status.accepting_observations);
        assert!(!status.kill_switch_active);
        assert!(status.shadow_only);
        assert!(!status.production_authority_granted);
        assert_eq!(status.observed_event_count, 1);
        assert_eq!(status.recorded_count, 1);
        assert_eq!(status.replay_count, 1);
        assert_eq!(status.rejected_count, 0);
    }

    #[test]
    fn runtime_kill_switch_fails_closed_without_authority() {
        let directory = tempdir().unwrap();
        let kill_switch = directory.path().join("kill");
        let runtime = NduH1Runtime::open_with_kill_switch(
            config(),
            directory.path().join("ndu.jsonl"),
            Some(kill_switch.clone()),
        )
        .unwrap();
        std::fs::write(&kill_switch, b"disabled\n").unwrap();
        assert_eq!(
            runtime.observe_event(event("event-1")),
            Err(NduH1RuntimeError::KillSwitchActive)
        );
        let status = runtime.status().unwrap();
        assert!(!status.ready);
        assert!(!status.accepting_observations);
        assert!(status.kill_switch_active);
        assert!(!status.production_authority_granted);
    }

    #[tokio::test]
    async fn runtime_kernel_general_terminal_stream_observes_shadow_outcomes() {
        let directory = tempdir().unwrap();
        let runtime = NduH1Runtime::open(config(), directory.path().join("ndu.jsonl")).unwrap();
        let kernel = crate::RuntimeKernel::new().with_ndu_h1_shadow_observer(runtime.clone());

        let result = kernel
            .run_demo_turn_in_session(
                "general-terminal-stream",
                "Use the echo tool with arguments exactly {\"text\":\"shadow\"}. Do not answer directly.",
            )
            .await
            .unwrap();

        assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
        assert_eq!(
            result
                .execution_receipt
                .as_ref()
                .map(|receipt| receipt.terminal_status.as_str()),
            Some("succeeded")
        );
        let status = runtime.status().unwrap();
        assert_eq!(status.observed_event_count, 1);
        assert_eq!(status.recorded_count, 1);
        assert_eq!(status.replay_count, 0);
        assert!(!status.production_authority_granted);
    }
}
