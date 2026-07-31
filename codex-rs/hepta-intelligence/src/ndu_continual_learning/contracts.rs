use hepta_contracts::ContentHash;
use hepta_contracts::Revision;

/// Inclusive lower bound for one normalized utility component.
pub const MIN_UTILITY_BASIS_POINTS: i32 = -10_000;

/// Inclusive upper bound for one normalized utility component.
pub const MAX_UTILITY_BASIS_POINTS: i32 = 10_000;

/// One bounded, fixed-point utility component measured in basis points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedUtilityScore(i32);

impl BoundedUtilityScore {
    /// Creates a score when it lies inside the canonical utility interval.
    pub const fn try_new(value: i32) -> Option<Self> {
        if value < MIN_UTILITY_BASIS_POINTS || value > MAX_UTILITY_BASIS_POINTS {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the fixed-point basis-point value.
    pub const fn basis_points(self) -> i32 {
        self.0
    }
}

/// Non-compensatory verdict for one hard feasibility constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardFeasibilityVerdict {
    /// The constraint was independently verified as satisfied.
    Satisfied,
    /// The constraint was independently verified as violated.
    Violated,
    /// The constraint lacks enough evidence and therefore fails closed.
    Unknown,
}

/// Hard feasibility mask applied before any utility optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardFeasibilityMask {
    safety: HardFeasibilityVerdict,
    permission: HardFeasibilityVerdict,
    budget: HardFeasibilityVerdict,
    correctability: HardFeasibilityVerdict,
}

impl HardFeasibilityMask {
    /// Creates a mask from four independently evaluated constraints.
    pub const fn new(
        safety: HardFeasibilityVerdict,
        permission: HardFeasibilityVerdict,
        budget: HardFeasibilityVerdict,
        correctability: HardFeasibilityVerdict,
    ) -> Self {
        Self {
            safety,
            permission,
            budget,
            correctability,
        }
    }

    /// Returns whether every hard constraint is explicitly satisfied.
    pub const fn permits_optimization(self) -> bool {
        matches!(self.safety, HardFeasibilityVerdict::Satisfied)
            && matches!(self.permission, HardFeasibilityVerdict::Satisfied)
            && matches!(self.budget, HardFeasibilityVerdict::Satisfied)
            && matches!(self.correctability, HardFeasibilityVerdict::Satisfied)
    }

    /// Returns the safety verdict.
    pub const fn safety(self) -> HardFeasibilityVerdict {
        self.safety
    }

    /// Returns the permission verdict.
    pub const fn permission(self) -> HardFeasibilityVerdict {
        self.permission
    }

    /// Returns the resource-budget verdict.
    pub const fn budget(self) -> HardFeasibilityVerdict {
        self.budget
    }

    /// Returns the correctability verdict.
    pub const fn correctability(self) -> HardFeasibilityVerdict {
        self.correctability
    }
}

/// Bounded utility vector kept separate from hard feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtilityVector {
    task_value: BoundedUtilityScore,
    learning_value: BoundedUtilityScore,
    trust: BoundedUtilityScore,
    memory_pollution_risk: BoundedUtilityScore,
    resource_cost: BoundedUtilityScore,
    uncertainty: BoundedUtilityScore,
}

impl UtilityVector {
    /// Creates one bounded utility vector.
    pub const fn new(
        task_value: BoundedUtilityScore,
        learning_value: BoundedUtilityScore,
        trust: BoundedUtilityScore,
        memory_pollution_risk: BoundedUtilityScore,
        resource_cost: BoundedUtilityScore,
        uncertainty: BoundedUtilityScore,
    ) -> Self {
        Self {
            task_value,
            learning_value,
            trust,
            memory_pollution_risk,
            resource_cost,
            uncertainty,
        }
    }

    /// Returns estimated task value.
    pub const fn task_value(self) -> BoundedUtilityScore {
        self.task_value
    }

    /// Returns estimated learning value.
    pub const fn learning_value(self) -> BoundedUtilityScore {
        self.learning_value
    }

    /// Returns estimated trust contribution.
    pub const fn trust(self) -> BoundedUtilityScore {
        self.trust
    }

    /// Returns estimated memory-pollution risk.
    pub const fn memory_pollution_risk(self) -> BoundedUtilityScore {
        self.memory_pollution_risk
    }

    /// Returns estimated resource cost.
    pub const fn resource_cost(self) -> BoundedUtilityScore {
        self.resource_cost
    }

    /// Returns model uncertainty.
    pub const fn uncertainty(self) -> BoundedUtilityScore {
        self.uncertainty
    }
}

/// Immutable reference to one de-identified event consumed by NDU shadowing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduUtilityEventRef {
    event_hash: ContentHash,
    source_receipt_hash: ContentHash,
    subject_pseudonym_hash: ContentHash,
    explicit_preference_evidence_hash: Option<ContentHash>,
}

impl NduUtilityEventRef {
    /// Creates an immutable reference without granting access to source data.
    pub fn new(
        event_hash: ContentHash,
        source_receipt_hash: ContentHash,
        subject_pseudonym_hash: ContentHash,
        explicit_preference_evidence_hash: Option<ContentHash>,
    ) -> Self {
        Self {
            event_hash,
            source_receipt_hash,
            subject_pseudonym_hash,
            explicit_preference_evidence_hash,
        }
    }

    /// Returns the canonical event digest.
    pub fn event_hash(&self) -> &ContentHash {
        &self.event_hash
    }

    /// Returns the source outcome receipt digest.
    pub fn source_receipt_hash(&self) -> &ContentHash {
        &self.source_receipt_hash
    }

    /// Returns the scoped pseudonym digest instead of a raw identity.
    pub fn subject_pseudonym_hash(&self) -> &ContentHash {
        &self.subject_pseudonym_hash
    }

    /// Returns separately authenticated explicit preference evidence, if any.
    pub fn explicit_preference_evidence_hash(&self) -> Option<&ContentHash> {
        self.explicit_preference_evidence_hash.as_ref()
    }
}

/// Replayable NDU state transition bound to exact inputs and implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduUtilityTransitionRef {
    revision: Revision,
    previous_state_hash: ContentHash,
    event: NduUtilityEventRef,
    model_hash: ContentHash,
    config_hash: ContentHash,
    next_state_hash: ContentHash,
    utility: UtilityVector,
    feasibility: HardFeasibilityMask,
}

impl NduUtilityTransitionRef {
    /// Creates a complete deterministic transition receipt.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        revision: Revision,
        previous_state_hash: ContentHash,
        event: NduUtilityEventRef,
        model_hash: ContentHash,
        config_hash: ContentHash,
        next_state_hash: ContentHash,
        utility: UtilityVector,
        feasibility: HardFeasibilityMask,
    ) -> Self {
        Self {
            revision,
            previous_state_hash,
            event,
            model_hash,
            config_hash,
            next_state_hash,
            utility,
            feasibility,
        }
    }

    /// Returns the transition revision.
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    /// Returns the previous state digest.
    pub fn previous_state_hash(&self) -> &ContentHash {
        &self.previous_state_hash
    }

    /// Returns the immutable input event reference.
    pub fn event(&self) -> &NduUtilityEventRef {
        &self.event
    }

    /// Returns the frozen model digest.
    pub fn model_hash(&self) -> &ContentHash {
        &self.model_hash
    }

    /// Returns the frozen configuration digest.
    pub fn config_hash(&self) -> &ContentHash {
        &self.config_hash
    }

    /// Returns the resulting state digest.
    pub fn next_state_hash(&self) -> &ContentHash {
        &self.next_state_hash
    }

    /// Returns the bounded utility estimate.
    pub const fn utility(&self) -> UtilityVector {
        self.utility
    }

    /// Returns the non-compensatory hard feasibility mask.
    pub const fn feasibility(&self) -> HardFeasibilityMask {
        self.feasibility
    }
}

/// Immutable dataset manifest reference for offline NDU evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NduDatasetManifestRef {
    manifest_hash: ContentHash,
    schema_hash: ContentHash,
    row_count: u64,
    deidentified: bool,
    consent_scope_hash: ContentHash,
    revocation_snapshot_hash: ContentHash,
}

impl NduDatasetManifestRef {
    /// Creates a dataset manifest reference.
    pub fn new(
        manifest_hash: ContentHash,
        schema_hash: ContentHash,
        row_count: u64,
        deidentified: bool,
        consent_scope_hash: ContentHash,
        revocation_snapshot_hash: ContentHash,
    ) -> Self {
        Self {
            manifest_hash,
            schema_hash,
            row_count,
            deidentified,
            consent_scope_hash,
            revocation_snapshot_hash,
        }
    }

    /// Returns whether the manifest is eligible for offline evaluation.
    pub const fn is_offline_evaluation_eligible(&self) -> bool {
        self.deidentified && self.row_count > 0
    }

    /// Returns the manifest digest.
    pub fn manifest_hash(&self) -> &ContentHash {
        &self.manifest_hash
    }

    /// Returns the dataset schema digest.
    pub fn schema_hash(&self) -> &ContentHash {
        &self.schema_hash
    }

    /// Returns the number of de-identified rows.
    pub const fn row_count(&self) -> u64 {
        self.row_count
    }

    /// Returns whether direct identifiers were removed.
    pub const fn deidentified(&self) -> bool {
        self.deidentified
    }

    /// Returns the consent-scope snapshot digest.
    pub fn consent_scope_hash(&self) -> &ContentHash {
        &self.consent_scope_hash
    }

    /// Returns the revocation snapshot digest.
    pub fn revocation_snapshot_hash(&self) -> &ContentHash {
        &self.revocation_snapshot_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_constraints_cannot_be_compensated_by_utility() {
        let mask = HardFeasibilityMask::new(
            HardFeasibilityVerdict::Violated,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
            HardFeasibilityVerdict::Satisfied,
        );

        assert!(!mask.permits_optimization());
        assert_eq!(BoundedUtilityScore::try_new(10_001), None);
    }

    #[test]
    fn dataset_requires_deidentified_nonempty_rows() {
        let manifest = NduDatasetManifestRef::new(
            ContentHash::new("manifest"),
            ContentHash::new("schema"),
            12,
            true,
            ContentHash::new("consent"),
            ContentHash::new("revocation"),
        );

        assert!(manifest.is_offline_evaluation_eligible());
    }
}
