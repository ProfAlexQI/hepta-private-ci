use std::collections::BTreeMap;

use hepta_contracts::ContentHash;
use hepta_contracts::Revision;

use super::HardFeasibilityMask;
use super::NduBaselineKind;
use super::NduShadowArmResult;
use super::NduShadowBoundary;
use super::NduShadowObservation;
use super::NduThreatModel;
use super::evaluate_ndu_shadow_arm;
use super::stable_hash;

const H1_BASELINES: [NduBaselineKind; 4] = [
    NduBaselineKind::CurrentHeuristic,
    NduBaselineKind::ContextualBandit,
    NduBaselineKind::FrozenGruMlp,
    NduBaselineKind::NduShadow,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduH1ShadowConfig {
    tenant_scope_hash: ContentHash,
    consent_scope_hash: ContentHash,
    revocation_snapshot_hash: ContentHash,
    model_hash: ContentHash,
    scorer_config_hash: ContentHash,
    initial_state_hash: ContentHash,
    max_events: u64,
    enabled: bool,
}

impl NduH1ShadowConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tenant_scope_hash: ContentHash,
        consent_scope_hash: ContentHash,
        revocation_snapshot_hash: ContentHash,
        model_hash: ContentHash,
        scorer_config_hash: ContentHash,
        initial_state_hash: ContentHash,
        max_events: u64,
        enabled: bool,
    ) -> Self {
        Self {
            tenant_scope_hash,
            consent_scope_hash,
            revocation_snapshot_hash,
            model_hash,
            scorer_config_hash,
            initial_state_hash,
            max_events,
            enabled,
        }
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub const fn max_events(&self) -> u64 {
        self.max_events
    }

    pub fn initial_state_hash(&self) -> &ContentHash {
        &self.initial_state_hash
    }

    pub fn tenant_scope_hash(&self) -> &ContentHash {
        &self.tenant_scope_hash
    }

    pub fn consent_scope_hash(&self) -> &ContentHash {
        &self.consent_scope_hash
    }

    pub fn revocation_snapshot_hash(&self) -> &ContentHash {
        &self.revocation_snapshot_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduH1ShadowRequest {
    tenant_scope_hash: ContentHash,
    consent_scope_hash: ContentHash,
    revocation_snapshot_hash: ContentHash,
    observation: NduShadowObservation,
    feasibility: HardFeasibilityMask,
}

impl NduH1ShadowRequest {
    pub fn new(
        tenant_scope_hash: ContentHash,
        consent_scope_hash: ContentHash,
        revocation_snapshot_hash: ContentHash,
        observation: NduShadowObservation,
        feasibility: HardFeasibilityMask,
    ) -> Self {
        Self {
            tenant_scope_hash,
            consent_scope_hash,
            revocation_snapshot_hash,
            observation,
            feasibility,
        }
    }

    pub fn event_hash(&self) -> &ContentHash {
        self.observation.event().event_hash()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduH1ShadowReceipt {
    revision: Revision,
    event_hash: ContentHash,
    previous_journal_hash: ContentHash,
    journal_hash: ContentHash,
    arm_results: Vec<NduShadowArmResult>,
    propensity_basis_points: u16,
    delayed_outcome_hash: Option<ContentHash>,
    production_authority_granted: bool,
}

impl NduH1ShadowReceipt {
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    pub fn event_hash(&self) -> &ContentHash {
        &self.event_hash
    }

    pub fn previous_journal_hash(&self) -> &ContentHash {
        &self.previous_journal_hash
    }

    pub fn journal_hash(&self) -> &ContentHash {
        &self.journal_hash
    }

    pub fn arm_results(&self) -> &[NduShadowArmResult] {
        &self.arm_results
    }

    pub const fn propensity_basis_points(&self) -> u16 {
        self.propensity_basis_points
    }

    pub fn delayed_outcome_hash(&self) -> Option<&ContentHash> {
        self.delayed_outcome_hash.as_ref()
    }

    pub const fn production_authority_granted(&self) -> bool {
        self.production_authority_granted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NduH1ShadowError {
    Disabled,
    InvalidThreatModel,
    InvalidCapacity,
    TenantScopeMismatch,
    ConsentScopeMismatch,
    RevocationSnapshotMismatch,
    CapacityExceeded,
    ObservationRejected,
}

#[derive(Debug)]
pub struct NduH1ShadowController {
    config: NduH1ShadowConfig,
    boundary: NduShadowBoundary,
    journal_head: ContentHash,
    observed_event_count: u64,
    receipts_by_event: BTreeMap<String, NduH1ShadowReceipt>,
}

impl NduH1ShadowController {
    pub fn new(config: NduH1ShadowConfig) -> Result<Self, NduH1ShadowError> {
        if !config.is_enabled() {
            return Err(NduH1ShadowError::Disabled);
        }
        if config.max_events() == 0 {
            return Err(NduH1ShadowError::InvalidCapacity);
        }
        if !NduThreatModel::canonical().is_complete() {
            return Err(NduH1ShadowError::InvalidThreatModel);
        }
        let journal_head = config.initial_state_hash.clone();
        Ok(Self {
            config,
            boundary: NduShadowBoundary::required(),
            journal_head,
            observed_event_count: 0,
            receipts_by_event: BTreeMap::new(),
        })
    }

    pub fn resume(
        config: NduH1ShadowConfig,
        journal_head: ContentHash,
        observed_event_count: u64,
    ) -> Result<Self, NduH1ShadowError> {
        let mut controller = Self::new(config)?;
        if observed_event_count > controller.config.max_events {
            return Err(NduH1ShadowError::CapacityExceeded);
        }
        controller.journal_head = journal_head;
        controller.observed_event_count = observed_event_count;
        Ok(controller)
    }

    pub fn observe(
        &mut self,
        request: NduH1ShadowRequest,
    ) -> Result<NduH1ShadowReceipt, NduH1ShadowError> {
        self.validate_scope(&request)?;
        let event_key = request.observation.event().event_hash().as_str().to_owned();
        if let Some(receipt) = self.receipts_by_event.get(&event_key) {
            return Ok(receipt.clone());
        }
        if self.observed_event_count >= self.config.max_events {
            return Err(NduH1ShadowError::CapacityExceeded);
        }
        let revision = Revision::new(self.observed_event_count + 1);
        let previous_journal_hash = self.journal_head.clone();
        let mut arm_results = Vec::with_capacity(H1_BASELINES.len());
        for baseline in H1_BASELINES {
            let result = evaluate_ndu_shadow_arm(
                &self.boundary,
                baseline,
                revision,
                previous_journal_hash.clone(),
                request.observation.clone(),
                self.config.model_hash.clone(),
                self.config.scorer_config_hash.clone(),
                request.feasibility,
            )
            .ok_or(NduH1ShadowError::ObservationRejected)?;
            arm_results.push(result);
        }
        let receipt_parts = arm_results
            .iter()
            .map(|result| result.replay_receipt_hash().as_str())
            .collect::<Vec<_>>();
        let mut hash_parts = vec![
            "hepta_ndu_h1_shadow_journal_v1",
            previous_journal_hash.as_str(),
            event_key.as_str(),
        ];
        hash_parts.extend(receipt_parts);
        let journal_hash = stable_hash(&hash_parts);
        let receipt = NduH1ShadowReceipt {
            revision,
            event_hash: request.observation.event().event_hash().clone(),
            previous_journal_hash,
            journal_hash: journal_hash.clone(),
            arm_results,
            propensity_basis_points: request.observation.propensity_basis_points,
            delayed_outcome_hash: request.observation.delayed_outcome_hash.clone(),
            production_authority_granted: false,
        };
        self.journal_head = journal_hash;
        self.observed_event_count += 1;
        self.receipts_by_event.insert(event_key, receipt.clone());
        Ok(receipt)
    }

    pub fn journal_head(&self) -> &ContentHash {
        &self.journal_head
    }

    pub fn observed_event_count(&self) -> u64 {
        self.observed_event_count
    }

    fn validate_scope(&self, request: &NduH1ShadowRequest) -> Result<(), NduH1ShadowError> {
        if request.tenant_scope_hash != self.config.tenant_scope_hash {
            return Err(NduH1ShadowError::TenantScopeMismatch);
        }
        if request.consent_scope_hash != self.config.consent_scope_hash {
            return Err(NduH1ShadowError::ConsentScopeMismatch);
        }
        if request.revocation_snapshot_hash != self.config.revocation_snapshot_hash {
            return Err(NduH1ShadowError::RevocationSnapshotMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BoundedUtilityScore;
    use crate::HardFeasibilityVerdict;
    use crate::NduUtilityEventRef;

    fn hash(value: &str) -> ContentHash {
        ContentHash::new(value)
    }

    fn config(max_events: u64) -> NduH1ShadowConfig {
        NduH1ShadowConfig::new(
            hash("tenant"),
            hash("consent"),
            hash("revocation"),
            hash("model"),
            hash("config"),
            hash("initial"),
            max_events,
            true,
        )
    }

    fn request(event: &str) -> NduH1ShadowRequest {
        let observation = NduShadowObservation::new(
            NduUtilityEventRef::new(hash(event), hash("receipt"), hash("subject"), None),
            1_000,
            500,
            750,
            100,
            300,
            200,
            5_000,
            None,
        );
        NduH1ShadowRequest::new(
            hash("tenant"),
            hash("consent"),
            hash("revocation"),
            observation,
            HardFeasibilityMask::new(
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
                HardFeasibilityVerdict::Satisfied,
            ),
        )
    }

    #[test]
    fn observes_four_arms_and_never_grants_authority() {
        let mut controller = NduH1ShadowController::new(config(2)).unwrap();
        let receipt = controller.observe(request("event-1")).unwrap();

        assert_eq!(receipt.revision(), Revision::new(1));
        assert_eq!(receipt.arm_results().len(), 4);
        assert!(!receipt.production_authority_granted());
        assert_eq!(controller.observed_event_count(), 1);
        assert_eq!(controller.journal_head(), receipt.journal_hash());
    }

    #[test]
    fn duplicate_event_is_idempotent_and_capacity_is_fail_closed() {
        let mut controller = NduH1ShadowController::new(config(1)).unwrap();
        let first = controller.observe(request("event-1")).unwrap();
        let replay = controller.observe(request("event-1")).unwrap();

        assert_eq!(first, replay);
        assert_eq!(controller.observed_event_count(), 1);
        assert_eq!(
            controller.observe(request("event-2")),
            Err(NduH1ShadowError::CapacityExceeded)
        );
    }

    #[test]
    fn tenant_consent_and_revocation_must_match() {
        let mut controller = NduH1ShadowController::new(config(1)).unwrap();
        let mut mismatched = request("event-1");
        mismatched.tenant_scope_hash = hash("other-tenant");
        assert_eq!(
            controller.observe(mismatched),
            Err(NduH1ShadowError::TenantScopeMismatch)
        );
    }

    #[test]
    fn utility_bounds_remain_available_to_h1_callers() {
        assert_eq!(
            BoundedUtilityScore::try_new(10_000).unwrap().basis_points(),
            10_000
        );
    }
}
