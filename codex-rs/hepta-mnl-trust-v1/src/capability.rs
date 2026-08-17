use sha2::Digest;

use crate::AbsentCapabilityLedgerInspectionV1;
use crate::AuthorityDispositionV1;
use crate::CapabilityEntryV1;
use crate::CapabilityStateV1;
use crate::CompletionCapabilityLedgerV1;
use crate::CompletionCapabilityV1;
use crate::CompletionDispositionV1;
use crate::EvidenceOriginV1;
use crate::MnlTrustError;
use crate::ancestry::exact_phase_a_anchor;
use crate::invalid;
use crate::model::AbsentLedgerInspectionSealV1;

pub const CAPABILITY_LEDGER_SCHEMA: &str = "hepta_mnl_completion_capability_ledger_v1";
pub const MAX_CAPABILITY_LEDGER_BYTES: usize = 64 * 1024;

pub fn exact_phase_a_capability_ledger() -> CompletionCapabilityLedgerV1 {
    use CompletionCapabilityV1 as Capability;

    let capabilities = [
        Capability::ExternalRoleSeparatedSignatures,
        Capability::FinalToolingAncestry,
        Capability::ExactSourceAndRoleBinaryProvenance,
        Capability::DurableAtomicOneShotReplay,
        Capability::PreRunWallClockSupervisor,
        Capability::LiveReadOnlyCollectorAndClosedRunner,
        Capability::IndependentBundleCopyReadbackAndAckSigner,
        Capability::ImmutablePreRunAndPostRunPublication,
    ];
    CompletionCapabilityLedgerV1 {
        anchor: exact_phase_a_anchor(),
        authority: AuthorityDispositionV1::InspectionOnlyNoAuthority,
        disposition: CompletionDispositionV1::Blocked,
        entries: capabilities
            .into_iter()
            .map(|capability| CapabilityEntryV1 {
                authority: AuthorityDispositionV1::InspectionOnlyNoAuthority,
                capability,
                evidence_origin: EvidenceOriginV1::FutureExternalFrozenOnly,
                state: CapabilityStateV1::Absent,
            })
            .collect(),
        schema: CAPABILITY_LEDGER_SCHEMA.to_string(),
    }
}

pub fn validate_phase_a_capability_ledger(
    ledger: &CompletionCapabilityLedgerV1,
) -> Result<(), MnlTrustError> {
    if ledger != &exact_phase_a_capability_ledger() {
        return Err(invalid(
            "completion capability ledger differs from the exact non-authorizing Phase-A roster",
        ));
    }
    Ok(())
}

pub fn exact_phase_a_capability_ledger_bytes() -> Result<Vec<u8>, MnlTrustError> {
    serde_json::to_vec(&exact_phase_a_capability_ledger())
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))
}

pub fn inspect_canonical_phase_a_capability_ledger(
    canonical_ledger: &[u8],
) -> Result<AbsentCapabilityLedgerInspectionV1, MnlTrustError> {
    if canonical_ledger.is_empty() || canonical_ledger.len() > MAX_CAPABILITY_LEDGER_BYTES {
        return Err(invalid(
            "completion capability ledger byte length is outside its bound",
        ));
    }
    let ledger: CompletionCapabilityLedgerV1 = serde_json::from_slice(canonical_ledger)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    let reencoded = serde_json::to_vec(&ledger)
        .map_err(|error| MnlTrustError::Serialization(error.to_string()))?;
    if reencoded != canonical_ledger {
        return Err(invalid(
            "completion capability ledger is not exact canonical JSON",
        ));
    }
    validate_phase_a_capability_ledger(&ledger)?;
    Ok(AbsentCapabilityLedgerInspectionV1 {
        canonical_sha256: format!("{:x}", sha2::Sha256::digest(canonical_ledger)),
        entry_count: ledger.entries.len(),
        _seal: AbsentLedgerInspectionSealV1,
    })
}
