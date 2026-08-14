//! Model-only contract for preparing a Linux-v8 install epoch.
//!
//! Production profiles remain unpublished and verification therefore fails
//! closed. Even a verified model token cannot install or activate services,
//! establish a trusted state root, claim external current-tip state, or begin
//! a qualification attempt.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::AuthoritySignatureAlgorithmV8;
use crate::AuthoritySignerBindingV8;
use crate::CryptographicSignatureObservation;
use crate::ExactRootInstallInventoryV8;
use crate::QualificationError;
use crate::RootFileInstallIdentityV8;
use crate::SshsigTrustPurposeV8;
use crate::TargetHostBindingV8;
use crate::VerifiedTrustPolicyBindingV8;
use crate::invalid;
use crate::required_frozen_trust_binding_v8;
use crate::verify_statement_sshsig_for_purpose_v8;

#[cfg(test)]
#[path = "install_epoch_v1_tests.rs"]
mod tests;

#[cfg(test)]
pub(crate) fn test_only_genesis_install_epoch_preparation_v1()
-> (VerifiedInstallEpochPreparationV1, InstallEpochReplayGuardV1) {
    test_only_genesis_install_epoch_preparation_at_v1(1_050)
}

#[cfg(test)]
pub(crate) fn test_only_genesis_install_epoch_preparation_at_v1(
    model_verified_at_unix_seconds: u64,
) -> (VerifiedInstallEpochPreparationV1, InstallEpochReplayGuardV1) {
    tests::verified_genesis_preparation_at(model_verified_at_unix_seconds)
}

#[cfg(test)]
pub(crate) fn test_only_successor_install_epoch_preparation_v1()
-> (VerifiedInstallEpochPreparationV1, InstallEpochReplayGuardV1) {
    tests::verified_successor_preparation()
}

pub const INSTALL_EPOCH_AUTHORITY_SCHEMA_V1: &str = "hepta_linux_v8_install_epoch_authority_v1";
pub const INSTALL_EPOCH_AUTHORITY_NAMESPACE_V1: &str = "hepta-linux-v8-install-epoch-v1";
pub const EXTERNAL_WATERMARK_LEASE_SCHEMA_V1: &str = "hepta_linux_v8_external_watermark_lease_v1";
pub const EXTERNAL_WATERMARK_LEASE_NAMESPACE_V1: &str =
    "hepta-linux-v8-external-watermark-lease-v1";
pub const STATE_ROOT_PROFILE_ID_V1: &str = "hepta-linux-v8-state-root-profile-v1";
pub const EXTERNAL_WATERMARK_PROVIDER_PROFILE_ID_V1: &str =
    "hepta-linux-v8-external-watermark-provider-profile-v1";
pub const MAX_INSTALL_EPOCH_AUTHORITY_LIFETIME_SECONDS_V1: u64 = 15 * 60;
pub const MAX_EXTERNAL_WATERMARK_LEASE_LIFETIME_SECONDS_V1: u64 = 2 * 60;

pub(crate) const INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1: &str =
    "hepta-linux-v8-install-epoch-authority-claim-v1";
pub(crate) const EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1: &str =
    "hepta-linux-v8-external-watermark-lease-claim-v1";
const INSTALL_EPOCH_PREPARATION_BUNDLE_DOMAIN_V1: &str =
    "hepta-linux-v8-install-epoch-preparation-bundle-v1";

const MAX_SIGNATURE_BYTES_V1: usize = 16 * 1024;

/// The successor install authority can only prepare an inert installation.
/// Daemon activation and target execution require separate future authority.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallEpochActivationV1 {
    InstallFilesAndCreateInertStateRootOnlyNoReloadEnableStartOrExecution,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StateRootProfileBindingV1 {
    pub layout_manifest_sha256: String,
    pub mode: u32,
    pub path: String,
    pub profile_id: String,
    pub profile_revision: u64,
    pub profile_sha256: String,
    pub gid: u32,
    pub uid: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExternalWatermarkProviderProfileV1 {
    genesis_epoch_binding_sha256: String,
    genesis_tip_sha256: String,
    profile_id: String,
    profile_revision: u64,
    profile_sha256: String,
    trust_policy_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallEpochBindingV1 {
    pub epoch_nonce_sha256: String,
    pub epoch_sequence: u64,
}

/// Exact externally stored predecessor. There is deliberately no local
/// "missing means genesis" branch: even a first installation needs a
/// provider-pinned revision-0 sentinel with non-zero tip and epoch digests.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "predecessor", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExternalWatermarkPredecessorV1 {
    GenesisPinnedSentinel {
        genesis_epoch_binding_sha256: String,
        provider_profile_sha256: String,
        revision: u64,
        stream_id_sha256: String,
        tip_sha256: String,
    },
    Successor {
        installed_epoch_binding_sha256: String,
        installed_epoch_sequence: u64,
        provider_profile_sha256: String,
        revision: u64,
        stream_id_sha256: String,
        tip_sha256: String,
    },
}

impl ExternalWatermarkPredecessorV1 {
    fn kind_name(&self) -> &'static str {
        match self {
            Self::GenesisPinnedSentinel { .. } => "genesis_pinned_sentinel",
            Self::Successor { .. } => "successor",
        }
    }

    fn installed_epoch_binding_sha256(&self) -> &str {
        match self {
            Self::GenesisPinnedSentinel {
                genesis_epoch_binding_sha256,
                ..
            } => genesis_epoch_binding_sha256,
            Self::Successor {
                installed_epoch_binding_sha256,
                ..
            } => installed_epoch_binding_sha256,
        }
    }

    fn installed_epoch_sequence(&self) -> u64 {
        match self {
            Self::GenesisPinnedSentinel { .. } => 0,
            Self::Successor {
                installed_epoch_sequence,
                ..
            } => *installed_epoch_sequence,
        }
    }

    fn provider_profile_sha256(&self) -> &str {
        match self {
            Self::GenesisPinnedSentinel {
                provider_profile_sha256,
                ..
            }
            | Self::Successor {
                provider_profile_sha256,
                ..
            } => provider_profile_sha256,
        }
    }

    fn revision(&self) -> u64 {
        match self {
            Self::GenesisPinnedSentinel { revision, .. } | Self::Successor { revision, .. } => {
                *revision
            }
        }
    }

    fn stream_id_sha256(&self) -> &str {
        match self {
            Self::GenesisPinnedSentinel {
                stream_id_sha256, ..
            }
            | Self::Successor {
                stream_id_sha256, ..
            } => stream_id_sha256,
        }
    }

    fn tip_sha256(&self) -> &str {
        match self {
            Self::GenesisPinnedSentinel { tip_sha256, .. } | Self::Successor { tip_sha256, .. } => {
                tip_sha256
            }
        }
    }
}

fn required_frozen_state_root_profile_v1() -> Result<StateRootProfileBindingV1, QualificationError>
{
    Err(invalid(
        "frozen install-epoch state-root profile is not independently published",
    ))
}

fn required_frozen_external_watermark_provider_profile_v1()
-> Result<ExternalWatermarkProviderProfileV1, QualificationError> {
    Err(invalid(
        "frozen external watermark provider profile is not independently published",
    ))
}

#[cfg(test)]
fn test_only_state_root_profile_v1() -> StateRootProfileBindingV1 {
    let mut profile = StateRootProfileBindingV1 {
        layout_manifest_sha256: sha256(b"test-only state-root layout manifest"),
        mode: 0o700,
        path: crate::STATE_ROOT_PATH_V8.to_string(),
        profile_id: STATE_ROOT_PROFILE_ID_V1.to_string(),
        profile_revision: 1,
        profile_sha256: String::new(),
        gid: 0,
        uid: 0,
    };
    profile.profile_sha256 = state_root_profile_sha256(&profile);
    profile
}

#[cfg(test)]
fn test_only_external_watermark_provider_profile_v1(
    trust: &VerifiedTrustPolicyBindingV8,
) -> ExternalWatermarkProviderProfileV1 {
    let mut profile = ExternalWatermarkProviderProfileV1 {
        genesis_epoch_binding_sha256: sha256(b"test-only external genesis epoch binding"),
        genesis_tip_sha256: sha256(b"test-only external genesis tip"),
        profile_id: EXTERNAL_WATERMARK_PROVIDER_PROFILE_ID_V1.to_string(),
        profile_revision: 1,
        profile_sha256: String::new(),
        trust_policy_sha256: trust.policy_sha256().to_string(),
    };
    profile.profile_sha256 = external_provider_profile_sha256(&profile);
    profile
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallEpochAuthorityChallengeV1 {
    pub activation: InstallEpochActivationV1,
    pub authority_nonce: String,
    pub epoch: InstallEpochBindingV1,
    pub expires_at_unix_seconds: u64,
    pub install_inventory: ExactRootInstallInventoryV8,
    pub issued_at_unix_seconds: u64,
    pub namespace: String,
    pub predecessor: ExternalWatermarkPredecessorV1,
    pub schema: String,
    pub signer: AuthoritySignerBindingV8,
    pub state_root_profile: StateRootProfileBindingV1,
    pub target_host: TargetHostBindingV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedInstallEpochAuthorityV1 {
    pub canonical_statement_sha256: String,
    pub challenge: InstallEpochAuthorityChallengeV1,
    pub detached_signature_bytes: Vec<u8>,
    pub detached_signature_sha256: String,
}

/// A provider-signed, short-lived reservation of exactly the next external
/// watermark revision. The provider is trusted to enforce one-shot CAS for
/// this lease nonce; the token still grants no target execution or activation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalWatermarkLeaseChallengeV1 {
    pub authority_nonce: String,
    pub epoch: InstallEpochBindingV1,
    pub expires_at_unix_seconds: u64,
    pub install_authority_statement_sha256: String,
    pub install_authority_trust_policy_sha256: String,
    pub issued_at_unix_seconds: u64,
    pub lease_nonce: String,
    pub namespace: String,
    pub predecessor: ExternalWatermarkPredecessorV1,
    pub provider_trust_policy_sha256: String,
    pub reserved_successor_revision: u64,
    pub schema: String,
    pub signer: AuthoritySignerBindingV8,
    pub state_root_profile_sha256: String,
    pub target_host: TargetHostBindingV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedExternalWatermarkLeaseV1 {
    pub canonical_statement_sha256: String,
    pub challenge: ExternalWatermarkLeaseChallengeV1,
    pub detached_signature_bytes: Vec<u8>,
    pub detached_signature_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InstallEpochNonceClaimV1 {
    binding_sha256: String,
    scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InstallEpochPhaseEdgeV1 {
    predecessor_revision: u64,
    predecessor_state_sha256: String,
    revision: u64,
    state_sha256: String,
}

const MAX_INSTALL_EPOCH_REPLAY_RECORDS_V1: usize = 4_096;
const MAX_INSTALL_EPOCH_PHASE_EDGES_V1: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InstallEpochExactPhaseLookupV1 {
    Exact,
    Absent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum InstallEpochDurableRecordV1 {
    Bundle {
        binding_sha256: String,
    },
    Claim(InstallEpochNonceClaimV1),
    PhaseHead {
        edges: Vec<InstallEpochPhaseEdgeV1>,
        genesis_revision: u64,
        genesis_state_sha256: String,
        revision: u64,
        state_sha256: String,
    },
}

/// Process-memory model of the one cross-namespace claim store shared by
/// install preparation and external-watermark completion. The value retained
/// for every nonce is an exact typed-claim binding, not only a presence bit.
/// A live successor must replace this model with atomic, durable no-replace
/// claims outside the state root; this type grants no replay or execution
/// authority by itself. Its composite methods are atomic only as one
/// in-process model call; a live implementation must persist each composite
/// record set and phase transition as one crash-recoverable transaction, not
/// as a sequence of independent publishes.
#[derive(Debug, Default)]
pub struct InstallEpochReplayGuardV1 {
    records: BTreeMap<String, InstallEpochDurableRecordV1>,
}

impl InstallEpochReplayGuardV1 {
    fn require_exact_records(
        &self,
        required_claims: &[(&str, &str, &str)],
        required_bundles: &[(&str, &str)],
    ) -> Result<(), QualificationError> {
        for (nonce, scope, binding_sha256) in required_claims {
            validate_digest("required predecessor claim nonce", nonce)?;
            validate_digest("required predecessor claim binding", binding_sha256)?;
            if scope.is_empty() {
                return Err(invalid("required predecessor claim scope is empty"));
            }
            if !matches!(
                self.records.get(*nonce),
                Some(InstallEpochDurableRecordV1::Claim(claim))
                    if claim.scope == *scope
                        && claim.binding_sha256 == *binding_sha256
            ) {
                return Err(invalid(
                    "required predecessor claim is missing or not exact",
                ));
            }
        }
        for (bundle_id_sha256, bundle_binding_sha256) in required_bundles {
            validate_digest("required predecessor bundle id", bundle_id_sha256)?;
            validate_digest("required predecessor bundle binding", bundle_binding_sha256)?;
            if !matches!(
                self.records.get(*bundle_id_sha256),
                Some(InstallEpochDurableRecordV1::Bundle { binding_sha256 })
                    if binding_sha256 == *bundle_binding_sha256
            ) {
                return Err(invalid(
                    "required predecessor bundle is missing or not exact",
                ));
            }
        }
        Ok(())
    }

    pub fn from_consumed_nonces(
        nonces: impl IntoIterator<Item = String>,
    ) -> Result<Self, QualificationError> {
        let nonces = nonces.into_iter().collect::<Vec<_>>();
        if nonces.iter().any(|nonce| !digest_shape(nonce)) {
            return Err(invalid("persisted install-epoch nonce is malformed"));
        }
        let records = nonces
            .into_iter()
            .map(|nonce| {
                (
                    nonce,
                    InstallEpochDurableRecordV1::Claim(InstallEpochNonceClaimV1 {
                        binding_sha256: "0".repeat(64),
                        scope: "opaque_prior_claim".to_string(),
                    }),
                )
            })
            .collect();
        Ok(Self { records })
    }

    pub fn nonce_is_consumed(&self, nonce: &str) -> bool {
        self.records.contains_key(nonce)
    }

    #[cfg(test)]
    pub(crate) fn claim_exact_or_replay(
        &mut self,
        nonce: &str,
        scope: &str,
        binding_sha256: &str,
    ) -> Result<bool, QualificationError> {
        validate_digest("cross-namespace claim nonce", nonce)?;
        validate_digest("cross-namespace claim binding", binding_sha256)?;
        if scope.is_empty() {
            return Err(invalid("cross-namespace claim scope is empty"));
        }
        if let Some(existing) = self.records.get(nonce) {
            if matches!(
                existing,
                InstallEpochDurableRecordV1::Claim(claim)
                    if claim.scope == scope && claim.binding_sha256 == binding_sha256
            ) {
                return Ok(false);
            }
            return Err(invalid(
                "cross-namespace nonce is already bound to a different claim",
            ));
        }
        self.records.insert(
            nonce.to_string(),
            InstallEpochDurableRecordV1::Claim(InstallEpochNonceClaimV1 {
                binding_sha256: binding_sha256.to_string(),
                scope: scope.to_string(),
            }),
        );
        Ok(true)
    }

    pub(crate) fn claim_pair_and_bundle_or_exact_recovery(
        &mut self,
        first: (&str, &str, &str),
        second: (&str, &str, &str),
        bundle_id_sha256: &str,
        bundle_binding_sha256: &str,
    ) -> Result<bool, QualificationError> {
        validate_digest("cross-namespace bundle id", bundle_id_sha256)?;
        validate_digest("cross-namespace bundle binding", bundle_binding_sha256)?;
        for (nonce, scope, binding_sha256) in [first, second] {
            validate_digest("cross-namespace claim nonce", nonce)?;
            validate_digest("cross-namespace claim binding", binding_sha256)?;
            if scope.is_empty() {
                return Err(invalid("cross-namespace claim scope is empty"));
            }
        }
        if first.0 == second.0 || first.0 == bundle_id_sha256 || second.0 == bundle_id_sha256 {
            return Err(invalid(
                "cross-namespace bundled nonce pair is not distinct",
            ));
        }
        let first_existing = self.records.get(first.0);
        let second_existing = self.records.get(second.0);
        let bundle_existing = self.records.get(bundle_id_sha256);
        if let (Some(first_claim), Some(second_claim), Some(bundle)) =
            (first_existing, second_existing, bundle_existing)
        {
            if matches!(
                first_claim,
                InstallEpochDurableRecordV1::Claim(claim)
                    if claim.scope == first.1 && claim.binding_sha256 == first.2
            ) && matches!(
                second_claim,
                InstallEpochDurableRecordV1::Claim(claim)
                    if claim.scope == second.1 && claim.binding_sha256 == second.2
            ) && matches!(
                bundle,
                InstallEpochDurableRecordV1::Bundle { binding_sha256 }
                    if binding_sha256 == bundle_binding_sha256
            ) {
                return Ok(false);
            }
            return Err(invalid(
                "cross-namespace bundled claims conflict with durable state",
            ));
        }
        if first_existing.is_some() || second_existing.is_some() || bundle_existing.is_some() {
            return Err(invalid(
                "cross-namespace bundled claims are partially persisted",
            ));
        }
        if self
            .records
            .len()
            .checked_add(3)
            .is_none_or(|count| count > MAX_INSTALL_EPOCH_REPLAY_RECORDS_V1)
        {
            return Err(invalid("cross-namespace replay record budget is exhausted"));
        }
        self.records.insert(
            first.0.to_string(),
            InstallEpochDurableRecordV1::Claim(InstallEpochNonceClaimV1 {
                binding_sha256: first.2.to_string(),
                scope: first.1.to_string(),
            }),
        );
        self.records.insert(
            second.0.to_string(),
            InstallEpochDurableRecordV1::Claim(InstallEpochNonceClaimV1 {
                binding_sha256: second.2.to_string(),
                scope: second.1.to_string(),
            }),
        );
        self.records.insert(
            bundle_id_sha256.to_string(),
            InstallEpochDurableRecordV1::Bundle {
                binding_sha256: bundle_binding_sha256.to_string(),
            },
        );
        Ok(true)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn claim_pair_bundle_and_phase_or_exact_recovery(
        &mut self,
        required_claims: [(&str, &str, &str); 2],
        required_bundle: (&str, &str),
        first: (&str, &str, &str),
        second: (&str, &str, &str),
        bundle_id_sha256: &str,
        bundle_binding_sha256: &str,
        phase_head_id_sha256: &str,
        phase_revision: u64,
        phase_state_sha256: &str,
    ) -> Result<bool, QualificationError> {
        validate_digest("cross-namespace phase-head id", phase_head_id_sha256)?;
        validate_digest("cross-namespace phase state", phase_state_sha256)?;
        validate_digest("required predecessor bundle id", required_bundle.0)?;
        validate_digest("required predecessor bundle binding", required_bundle.1)?;
        for (nonce, scope, binding_sha256) in required_claims {
            validate_digest("required predecessor claim nonce", nonce)?;
            validate_digest("required predecessor claim binding", binding_sha256)?;
            if scope.is_empty() {
                return Err(invalid("required predecessor claim scope is empty"));
            }
        }
        let identities = [
            required_claims[0].0,
            required_claims[1].0,
            required_bundle.0,
            first.0,
            second.0,
            bundle_id_sha256,
            phase_head_id_sha256,
        ];
        if phase_revision == 0
            || identities
                .iter()
                .enumerate()
                .any(|(left, value)| identities[(left + 1)..].contains(value))
        {
            return Err(invalid(
                "cross-namespace predecessor, claims, bundle, and phase identities are not distinct",
            ));
        }
        self.require_exact_records(&required_claims, &[required_bundle])?;
        let existing_phase = self.records.get(phase_head_id_sha256).cloned();
        if existing_phase.is_none() {
            if self.records.contains_key(first.0)
                || self.records.contains_key(second.0)
                || self.records.contains_key(bundle_id_sha256)
            {
                return Err(invalid(
                    "cross-namespace completion intent is partially persisted",
                ));
            }
            if self
                .records
                .len()
                .checked_add(4)
                .is_none_or(|count| count > MAX_INSTALL_EPOCH_REPLAY_RECORDS_V1)
            {
                return Err(invalid("cross-namespace replay record budget is exhausted"));
            }
            self.claim_pair_and_bundle_or_exact_recovery(
                first,
                second,
                bundle_id_sha256,
                bundle_binding_sha256,
            )?;
            self.records.insert(
                phase_head_id_sha256.to_string(),
                InstallEpochDurableRecordV1::PhaseHead {
                    edges: Vec::new(),
                    genesis_revision: phase_revision,
                    genesis_state_sha256: phase_state_sha256.to_string(),
                    revision: phase_revision,
                    state_sha256: phase_state_sha256.to_string(),
                },
            );
            return Ok(true);
        }
        let first_exact = matches!(
            self.records.get(first.0),
            Some(InstallEpochDurableRecordV1::Claim(claim))
                if claim.scope == first.1 && claim.binding_sha256 == first.2
        );
        let second_exact = matches!(
            self.records.get(second.0),
            Some(InstallEpochDurableRecordV1::Claim(claim))
                if claim.scope == second.1 && claim.binding_sha256 == second.2
        );
        let bundle_exact = matches!(
            self.records.get(bundle_id_sha256),
            Some(InstallEpochDurableRecordV1::Bundle { binding_sha256 })
                if binding_sha256 == bundle_binding_sha256
        );
        if !first_exact || !second_exact || !bundle_exact {
            return Err(invalid(
                "cross-namespace phase head exists without exact intent records",
            ));
        }
        if matches!(
            existing_phase,
            Some(InstallEpochDurableRecordV1::PhaseHead {
                genesis_revision,
                genesis_state_sha256,
                ..
            }) if genesis_revision == phase_revision
                && genesis_state_sha256 == phase_state_sha256
        ) {
            Ok(false)
        } else {
            Err(invalid(
                "cross-namespace completion phase head conflicts with exact replay",
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn advance_phase_or_exact_recovery(
        &mut self,
        required_claims: &[(&str, &str, &str)],
        required_bundles: &[(&str, &str)],
        phase_head_id_sha256: &str,
        expected_revision: u64,
        expected_state_sha256: &str,
        next_revision: u64,
        next_state_sha256: &str,
    ) -> Result<bool, QualificationError> {
        validate_digest("cross-namespace phase-head id", phase_head_id_sha256)?;
        validate_digest(
            "cross-namespace expected phase state",
            expected_state_sha256,
        )?;
        validate_digest("cross-namespace next phase state", next_state_sha256)?;
        let required_next_revision = expected_revision
            .checked_add(1)
            .ok_or_else(|| invalid("cross-namespace completion phase revision overflows"))?;
        if expected_revision == 0
            || next_revision != required_next_revision
            || next_state_sha256 == expected_state_sha256
        {
            return Err(invalid(
                "cross-namespace completion phase transition is not consecutive and distinct",
            ));
        }
        self.require_exact_records(required_claims, required_bundles)?;
        match self.records.get_mut(phase_head_id_sha256) {
            Some(InstallEpochDurableRecordV1::PhaseHead {
                edges,
                revision,
                state_sha256,
                ..
            }) if *revision == expected_revision && state_sha256 == expected_state_sha256 => {
                if edges.len() >= MAX_INSTALL_EPOCH_PHASE_EDGES_V1 {
                    return Err(invalid(
                        "cross-namespace completion phase edge budget is exhausted",
                    ));
                }
                let prior_revision = *revision;
                let prior_state = state_sha256.clone();
                edges.push(InstallEpochPhaseEdgeV1 {
                    predecessor_revision: prior_revision,
                    predecessor_state_sha256: prior_state,
                    revision: next_revision,
                    state_sha256: next_state_sha256.to_string(),
                });
                *revision = next_revision;
                *state_sha256 = next_state_sha256.to_string();
                Ok(true)
            }
            Some(InstallEpochDurableRecordV1::PhaseHead { edges, .. })
                if edges.iter().any(|edge| {
                    edge.predecessor_revision == expected_revision
                        && edge.predecessor_state_sha256 == expected_state_sha256
                        && edge.revision == next_revision
                        && edge.state_sha256 == next_state_sha256
                }) =>
            {
                Ok(false)
            }
            Some(_) => Err(invalid(
                "cross-namespace completion phase transition forks or regresses",
            )),
            None => Err(invalid("cross-namespace completion phase head is missing")),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn lookup_exact_phase_transition(
        &self,
        required_claims: &[(&str, &str, &str)],
        required_bundles: &[(&str, &str)],
        phase_head_id_sha256: &str,
        expected_revision: u64,
        expected_state_sha256: &str,
        next_revision: u64,
        next_state_sha256: &str,
    ) -> Result<InstallEpochExactPhaseLookupV1, QualificationError> {
        validate_digest("cross-namespace phase-head id", phase_head_id_sha256)?;
        validate_digest(
            "cross-namespace expected phase state",
            expected_state_sha256,
        )?;
        validate_digest("cross-namespace next phase state", next_state_sha256)?;
        let required_next_revision = expected_revision
            .checked_add(1)
            .ok_or_else(|| invalid("cross-namespace completion phase revision overflows"))?;
        if expected_revision == 0
            || next_revision != required_next_revision
            || next_state_sha256 == expected_state_sha256
        {
            return Err(invalid(
                "cross-namespace completion recovery edge is not consecutive and distinct",
            ));
        }
        self.require_exact_records(required_claims, required_bundles)?;
        match self.records.get(phase_head_id_sha256) {
            Some(InstallEpochDurableRecordV1::PhaseHead { edges, .. })
                if edges.iter().any(|edge| {
                    edge.predecessor_revision == expected_revision
                        && edge.predecessor_state_sha256 == expected_state_sha256
                        && edge.revision == next_revision
                        && edge.state_sha256 == next_state_sha256
                }) =>
            {
                Ok(InstallEpochExactPhaseLookupV1::Exact)
            }
            Some(InstallEpochDurableRecordV1::PhaseHead {
                revision,
                state_sha256,
                ..
            }) if *revision == expected_revision && state_sha256 == expected_state_sha256 => {
                Ok(InstallEpochExactPhaseLookupV1::Absent)
            }
            Some(_) => Err(invalid(
                "cross-namespace completion recovery edge is absent or forked",
            )),
            None => Err(invalid("cross-namespace completion phase head is missing")),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn claim_bundle_and_advance_phase_or_exact_recovery(
        &mut self,
        required_claims: &[(&str, &str, &str)],
        required_bundles: &[(&str, &str)],
        claim: (&str, &str, &str),
        bundle_id_sha256: &str,
        bundle_binding_sha256: &str,
        phase_head_id_sha256: &str,
        expected_revision: u64,
        expected_state_sha256: &str,
        next_revision: u64,
        next_state_sha256: &str,
    ) -> Result<bool, QualificationError> {
        let (nonce, scope, claim_binding_sha256) = claim;
        validate_digest("cross-namespace claim nonce", nonce)?;
        validate_digest("cross-namespace claim binding", claim_binding_sha256)?;
        validate_digest("cross-namespace bundle id", bundle_id_sha256)?;
        validate_digest("cross-namespace bundle binding", bundle_binding_sha256)?;
        validate_digest("cross-namespace phase-head id", phase_head_id_sha256)?;
        validate_digest(
            "cross-namespace expected phase state",
            expected_state_sha256,
        )?;
        validate_digest("cross-namespace next phase state", next_state_sha256)?;
        if scope.is_empty() {
            return Err(invalid("cross-namespace claim scope is empty"));
        }
        let required_next_revision = expected_revision
            .checked_add(1)
            .ok_or_else(|| invalid("cross-namespace completion phase revision overflows"))?;
        if expected_revision == 0
            || next_revision != required_next_revision
            || next_state_sha256 == expected_state_sha256
        {
            return Err(invalid(
                "cross-namespace completion phase transition is not consecutive and distinct",
            ));
        }
        if nonce == bundle_id_sha256
            || nonce == phase_head_id_sha256
            || bundle_id_sha256 == phase_head_id_sha256
        {
            return Err(invalid(
                "cross-namespace claim, bundle, and phase identities are not distinct",
            ));
        }
        self.require_exact_records(required_claims, required_bundles)?;

        let claim_exact = matches!(
            self.records.get(nonce),
            Some(InstallEpochDurableRecordV1::Claim(existing))
                if existing.scope == scope
                    && existing.binding_sha256 == claim_binding_sha256
        );
        let bundle_exact = matches!(
            self.records.get(bundle_id_sha256),
            Some(InstallEpochDurableRecordV1::Bundle { binding_sha256 })
                if binding_sha256 == bundle_binding_sha256
        );
        let phase_is_expected = matches!(
            self.records.get(phase_head_id_sha256),
            Some(InstallEpochDurableRecordV1::PhaseHead { revision, state_sha256, .. })
                if *revision == expected_revision && state_sha256 == expected_state_sha256
        );
        let phase_contains_next_edge = matches!(
            self.records.get(phase_head_id_sha256),
            Some(InstallEpochDurableRecordV1::PhaseHead {
                edges,
                ..
            }) if edges.iter().any(|edge| {
                edge.predecessor_revision == expected_revision
                    && edge.predecessor_state_sha256 == expected_state_sha256
                    && edge.revision == next_revision
                    && edge.state_sha256 == next_state_sha256
            })
        );
        let claim_absent = !self.records.contains_key(nonce);
        let bundle_absent = !self.records.contains_key(bundle_id_sha256);

        let next_phase_record = if claim_absent && bundle_absent && phase_is_expected {
            if self
                .records
                .len()
                .checked_add(2)
                .is_none_or(|count| count > MAX_INSTALL_EPOCH_REPLAY_RECORDS_V1)
            {
                return Err(invalid("cross-namespace replay record budget is exhausted"));
            }
            match self.records.get(phase_head_id_sha256).cloned() {
                Some(InstallEpochDurableRecordV1::PhaseHead {
                    mut edges,
                    genesis_revision,
                    genesis_state_sha256,
                    ..
                }) => {
                    if edges.len() >= MAX_INSTALL_EPOCH_PHASE_EDGES_V1 {
                        return Err(invalid(
                            "cross-namespace completion phase edge budget is exhausted",
                        ));
                    }
                    edges.push(InstallEpochPhaseEdgeV1 {
                        predecessor_revision: expected_revision,
                        predecessor_state_sha256: expected_state_sha256.to_string(),
                        revision: next_revision,
                        state_sha256: next_state_sha256.to_string(),
                    });
                    Some(InstallEpochDurableRecordV1::PhaseHead {
                        edges,
                        genesis_revision,
                        genesis_state_sha256,
                        revision: next_revision,
                        state_sha256: next_state_sha256.to_string(),
                    })
                }
                _ => unreachable!("phase predecessor was validated before mutation"),
            }
        } else {
            None
        };

        if let Some(next_phase_record) = next_phase_record {
            self.records.insert(
                nonce.to_string(),
                InstallEpochDurableRecordV1::Claim(InstallEpochNonceClaimV1 {
                    binding_sha256: claim_binding_sha256.to_string(),
                    scope: scope.to_string(),
                }),
            );
            self.records.insert(
                bundle_id_sha256.to_string(),
                InstallEpochDurableRecordV1::Bundle {
                    binding_sha256: bundle_binding_sha256.to_string(),
                },
            );
            self.records
                .insert(phase_head_id_sha256.to_string(), next_phase_record);
            return Ok(true);
        }
        if claim_exact && bundle_exact && phase_contains_next_edge {
            return Ok(false);
        }
        Err(invalid(
            "cross-namespace bundled phase transition is partial, forked, or regressed",
        ))
    }

    #[cfg(test)]
    pub(crate) fn claim_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| matches!(record, InstallEpochDurableRecordV1::Claim(_)))
            .count()
    }

    #[cfg(test)]
    pub(crate) fn bundle_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| matches!(record, InstallEpochDurableRecordV1::Bundle { .. }))
            .count()
    }

    #[cfg(test)]
    pub(crate) fn phase_head_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| matches!(record, InstallEpochDurableRecordV1::PhaseHead { .. }))
            .count()
    }

    #[cfg(test)]
    pub(crate) fn phase_head_for_test(&self, phase_head_id_sha256: &str) -> Option<(u64, &str)> {
        match self.records.get(phase_head_id_sha256) {
            Some(InstallEpochDurableRecordV1::PhaseHead {
                revision,
                state_sha256,
                ..
            }) => Some((*revision, state_sha256)),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn phase_head_edge_for_test(
        &self,
        phase_head_id_sha256: &str,
    ) -> Option<(Option<u64>, Option<&str>, u64, &str)> {
        match self.records.get(phase_head_id_sha256) {
            Some(InstallEpochDurableRecordV1::PhaseHead {
                edges,
                revision,
                state_sha256,
                ..
            }) => {
                let predecessor = edges.last();
                Some((
                    predecessor.map(|edge| edge.predecessor_revision),
                    predecessor.map(|edge| edge.predecessor_state_sha256.as_str()),
                    *revision,
                    state_sha256,
                ))
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_bundle_for_test(&mut self, bundle_id_sha256: &str) {
        if matches!(
            self.records.get(bundle_id_sha256),
            Some(InstallEpochDurableRecordV1::Bundle { .. })
        ) {
            self.records.remove(bundle_id_sha256);
        }
    }

    #[cfg(test)]
    pub(crate) fn remove_phase_head_for_test(&mut self, phase_head_id_sha256: &str) {
        if matches!(
            self.records.get(phase_head_id_sha256),
            Some(InstallEpochDurableRecordV1::PhaseHead { .. })
        ) {
            self.records.remove(phase_head_id_sha256);
        }
    }
}

/// Opaque model proof that two independent trust roots and keys bind the same
/// inert install preparation and an exact external predecessor reservation.
/// A live root installer still needs one native, state-root-external durable
/// nonce claim shared across all namespaces, a verified current-host token,
/// trusted time, and the provider CAS commit/current-tip protocol. This token
/// alone executes nothing.
#[derive(Debug)]
pub struct VerifiedInstallEpochPreparationV1 {
    authority_nonce: String,
    authority_expires_at_unix_seconds: u64,
    authority_issued_at_unix_seconds: u64,
    authority_signature_sha256: String,
    authority_statement_sha256: String,
    authority_trust_policy_sha256: String,
    epoch: InstallEpochBindingV1,
    install_inventory: ExactRootInstallInventoryV8,
    lease_nonce: String,
    lease_expires_at_unix_seconds: u64,
    lease_issued_at_unix_seconds: u64,
    lease_signature_sha256: String,
    lease_statement_sha256: String,
    lease_trust_policy_sha256: String,
    model_verified_at_unix_seconds: u64,
    predecessor: ExternalWatermarkPredecessorV1,
    reserved_successor_revision: u64,
    state_root_profile: StateRootProfileBindingV1,
    target_host: TargetHostBindingV8,
}

impl VerifiedInstallEpochPreparationV1 {
    pub fn authority_expires_at_unix_seconds(&self) -> u64 {
        self.authority_expires_at_unix_seconds
    }

    pub fn authority_issued_at_unix_seconds(&self) -> u64 {
        self.authority_issued_at_unix_seconds
    }

    pub fn authority_nonce(&self) -> &str {
        &self.authority_nonce
    }

    pub fn authority_signature_sha256(&self) -> &str {
        &self.authority_signature_sha256
    }

    pub fn authority_statement_sha256(&self) -> &str {
        &self.authority_statement_sha256
    }

    pub fn authority_trust_policy_sha256(&self) -> &str {
        &self.authority_trust_policy_sha256
    }

    pub fn epoch(&self) -> &InstallEpochBindingV1 {
        &self.epoch
    }

    pub fn install_inventory(&self) -> &ExactRootInstallInventoryV8 {
        &self.install_inventory
    }

    pub fn lease_nonce(&self) -> &str {
        &self.lease_nonce
    }

    pub fn lease_expires_at_unix_seconds(&self) -> u64 {
        self.lease_expires_at_unix_seconds
    }

    pub fn lease_issued_at_unix_seconds(&self) -> u64 {
        self.lease_issued_at_unix_seconds
    }

    pub fn lease_signature_sha256(&self) -> &str {
        &self.lease_signature_sha256
    }

    pub fn lease_statement_sha256(&self) -> &str {
        &self.lease_statement_sha256
    }

    pub fn lease_trust_policy_sha256(&self) -> &str {
        &self.lease_trust_policy_sha256
    }

    /// Time supplied to the model verifier. This is only a signed-window
    /// binding for successor contracts; it is not trusted wall-clock evidence.
    pub fn model_verified_at_unix_seconds(&self) -> u64 {
        self.model_verified_at_unix_seconds
    }

    pub fn predecessor(&self) -> &ExternalWatermarkPredecessorV1 {
        &self.predecessor
    }

    pub fn reserved_successor_revision(&self) -> u64 {
        self.reserved_successor_revision
    }

    pub fn state_root_profile(&self) -> &StateRootProfileBindingV1 {
        &self.state_root_profile
    }

    pub fn target_host(&self) -> &TargetHostBindingV8 {
        &self.target_host
    }

    pub(crate) fn authority_nonce_claim_binding_sha256(&self) -> String {
        install_epoch_nonce_claim_binding_sha256_v1(
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            &self.authority_nonce,
            &self.authority_statement_sha256,
            &self.authority_signature_sha256,
            &self.authority_trust_policy_sha256,
        )
    }

    pub(crate) fn lease_nonce_claim_binding_sha256(&self) -> String {
        install_epoch_nonce_claim_binding_sha256_v1(
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            &self.lease_nonce,
            &self.lease_statement_sha256,
            &self.lease_signature_sha256,
            &self.lease_trust_policy_sha256,
        )
    }

    pub(crate) fn preparation_bundle_id_sha256(&self) -> String {
        let mut bytes = b"hepta_linux_v8_install_epoch_preparation_bundle_id_v1\0".to_vec();
        append_field(
            &mut bytes,
            "bundle_domain",
            INSTALL_EPOCH_PREPARATION_BUNDLE_DOMAIN_V1.as_bytes(),
        );
        append_field(
            &mut bytes,
            "authority_nonce",
            self.authority_nonce.as_bytes(),
        );
        append_field(&mut bytes, "lease_nonce", self.lease_nonce.as_bytes());
        append_field(
            &mut bytes,
            "authority_claim_sha256",
            self.authority_nonce_claim_binding_sha256().as_bytes(),
        );
        append_field(
            &mut bytes,
            "lease_claim_sha256",
            self.lease_nonce_claim_binding_sha256().as_bytes(),
        );
        sha256(&bytes)
    }

    pub(crate) fn preparation_bundle_binding_sha256(&self) -> String {
        let mut bytes = b"hepta_linux_v8_install_epoch_preparation_bundle_binding_v1\0".to_vec();
        append_field(
            &mut bytes,
            "bundle_id_sha256",
            self.preparation_bundle_id_sha256().as_bytes(),
        );
        append_u64(
            &mut bytes,
            "model_verified_at_unix_seconds",
            self.model_verified_at_unix_seconds,
        );
        append_epoch(&mut bytes, &self.epoch);
        append_predecessor(&mut bytes, &self.predecessor);
        append_u64(
            &mut bytes,
            "reserved_successor_revision",
            self.reserved_successor_revision,
        );
        append_field(
            &mut bytes,
            "machine_id_sha256",
            self.target_host.machine_id_sha256.as_bytes(),
        );
        append_state_root_profile(&mut bytes, &self.state_root_profile);
        append_install_inventory(&mut bytes, &self.install_inventory);
        sha256(&bytes)
    }

    pub fn model_preparation_verified(&self) -> bool {
        true
    }

    pub fn root_install_execution_allowed(&self) -> bool {
        false
    }

    pub fn daemon_reload_enable_or_start_allowed(&self) -> bool {
        false
    }

    pub fn trusted_state_root_established(&self) -> bool {
        false
    }

    pub fn fresh_attempt_allowed(&self) -> bool {
        false
    }
}

pub fn canonical_install_epoch_authority_statement_v1(
    challenge: &InstallEpochAuthorityChallengeV1,
) -> Result<Vec<u8>, QualificationError> {
    let trust = required_frozen_trust_binding_v8(SshsigTrustPurposeV8::InstallEpochAuthorityV1)?;
    let state_root_profile = required_frozen_state_root_profile_v1()?;
    let provider_profile = required_frozen_external_watermark_provider_profile_v1()?;
    canonical_install_epoch_authority_statement_with_trust_v1(
        challenge,
        &trust,
        &state_root_profile,
        &provider_profile,
    )
}

pub fn canonical_external_watermark_lease_statement_v1(
    challenge: &ExternalWatermarkLeaseChallengeV1,
) -> Result<Vec<u8>, QualificationError> {
    let trust = required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkLeaseV1)?;
    let state_root_profile = required_frozen_state_root_profile_v1()?;
    let provider_profile = required_frozen_external_watermark_provider_profile_v1()?;
    canonical_external_watermark_lease_statement_with_trust_v1(
        challenge,
        &trust,
        &state_root_profile,
        &provider_profile,
    )
}

pub fn verify_install_epoch_preparation_v1(
    authority: &SignedInstallEpochAuthorityV1,
    lease: &SignedExternalWatermarkLeaseV1,
    now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<VerifiedInstallEpochPreparationV1, QualificationError> {
    let authority_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::InstallEpochAuthorityV1)?;
    let lease_trust =
        required_frozen_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkLeaseV1)?;
    let state_root_profile = required_frozen_state_root_profile_v1()?;
    let provider_profile = required_frozen_external_watermark_provider_profile_v1()?;
    let authority_statement = canonical_install_epoch_authority_statement_with_trust_v1(
        &authority.challenge,
        &authority_trust,
        &state_root_profile,
        &provider_profile,
    )?;
    let lease_statement = canonical_external_watermark_lease_statement_with_trust_v1(
        &lease.challenge,
        &lease_trust,
        &state_root_profile,
        &provider_profile,
    )?;
    let authority_observation = verify_statement_sshsig_for_purpose_v8(
        &authority_statement,
        &authority.detached_signature_bytes,
        SshsigTrustPurposeV8::InstallEpochAuthorityV1,
    )?;
    let lease_observation = verify_statement_sshsig_for_purpose_v8(
        &lease_statement,
        &lease.detached_signature_bytes,
        SshsigTrustPurposeV8::ExternalWatermarkLeaseV1,
    )?;
    verify_install_epoch_preparation_with_evidence_v1(
        authority,
        lease,
        &authority_statement,
        &lease_statement,
        &authority_observation,
        &lease_observation,
        authority_trust,
        lease_trust,
        now_unix_seconds,
        replay_guard,
    )
}

#[cfg(test)]
pub(crate) fn verify_install_epoch_preparation_for_test_v1(
    authority: &SignedInstallEpochAuthorityV1,
    lease: &SignedExternalWatermarkLeaseV1,
    authority_observation: &CryptographicSignatureObservation,
    lease_observation: &CryptographicSignatureObservation,
    now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<VerifiedInstallEpochPreparationV1, QualificationError> {
    let authority_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::InstallEpochAuthorityV1);
    let lease_trust =
        crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkLeaseV1);
    let state_root_profile = test_only_state_root_profile_v1();
    let provider_profile = test_only_external_watermark_provider_profile_v1(&lease_trust);
    let authority_statement = canonical_install_epoch_authority_statement_with_trust_v1(
        &authority.challenge,
        &authority_trust,
        &state_root_profile,
        &provider_profile,
    )?;
    let lease_statement = canonical_external_watermark_lease_statement_with_trust_v1(
        &lease.challenge,
        &lease_trust,
        &state_root_profile,
        &provider_profile,
    )?;
    verify_install_epoch_preparation_with_evidence_v1(
        authority,
        lease,
        &authority_statement,
        &lease_statement,
        authority_observation,
        lease_observation,
        authority_trust,
        lease_trust,
        now_unix_seconds,
        replay_guard,
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_install_epoch_preparation_with_evidence_v1(
    authority: &SignedInstallEpochAuthorityV1,
    lease: &SignedExternalWatermarkLeaseV1,
    authority_statement: &[u8],
    lease_statement: &[u8],
    authority_observation: &CryptographicSignatureObservation,
    lease_observation: &CryptographicSignatureObservation,
    authority_trust: VerifiedTrustPolicyBindingV8,
    lease_trust: VerifiedTrustPolicyBindingV8,
    now_unix_seconds: u64,
    replay_guard: &mut InstallEpochReplayGuardV1,
) -> Result<VerifiedInstallEpochPreparationV1, QualificationError> {
    validate_independent_trust_bindings_v1(&authority_trust, &lease_trust)?;
    let authority_statement_sha256 = sha256(authority_statement);
    let lease_statement_sha256 = sha256(lease_statement);
    let authority_signature_sha256 = validate_signed_envelope(
        &authority.canonical_statement_sha256,
        authority_statement,
        &authority.detached_signature_sha256,
        &authority.detached_signature_bytes,
        authority_observation,
        &authority_trust,
    )?;
    let lease_signature_sha256 = validate_signed_envelope(
        &lease.canonical_statement_sha256,
        lease_statement,
        &lease.detached_signature_sha256,
        &lease.detached_signature_bytes,
        lease_observation,
        &lease_trust,
    )?;
    validate_time_window(
        authority.challenge.issued_at_unix_seconds,
        authority.challenge.expires_at_unix_seconds,
        MAX_INSTALL_EPOCH_AUTHORITY_LIFETIME_SECONDS_V1,
        now_unix_seconds,
        "install-epoch authority",
    )?;
    validate_time_window(
        lease.challenge.issued_at_unix_seconds,
        lease.challenge.expires_at_unix_seconds,
        MAX_EXTERNAL_WATERMARK_LEASE_LIFETIME_SECONDS_V1,
        now_unix_seconds,
        "external watermark lease",
    )?;
    if authority.challenge.authority_nonce == lease.challenge.lease_nonce {
        return Err(invalid(
            "install-epoch authority and lease nonces are not distinct",
        ));
    }
    if lease.challenge.issued_at_unix_seconds < authority.challenge.issued_at_unix_seconds
        || lease.challenge.expires_at_unix_seconds > authority.challenge.expires_at_unix_seconds
    {
        return Err(invalid(
            "external watermark lease validity is not contained by the install authority",
        ));
    }
    if lease.challenge.authority_nonce != authority.challenge.authority_nonce
        || lease.challenge.install_authority_statement_sha256 != authority_statement_sha256
        || lease.challenge.install_authority_trust_policy_sha256 != authority_trust.policy_sha256()
        || lease.challenge.epoch != authority.challenge.epoch
        || lease.challenge.predecessor != authority.challenge.predecessor
        || lease.challenge.state_root_profile_sha256
            != authority.challenge.state_root_profile.profile_sha256
        || lease.challenge.target_host != authority.challenge.target_host
        || lease.challenge.reserved_successor_revision
            != authority
                .challenge
                .predecessor
                .revision()
                .checked_add(1)
                .ok_or_else(|| invalid("external watermark revision overflow"))?
    {
        return Err(invalid(
            "external watermark lease does not reserve the exact signed install epoch",
        ));
    }

    let verified = VerifiedInstallEpochPreparationV1 {
        authority_nonce: authority.challenge.authority_nonce.clone(),
        authority_expires_at_unix_seconds: authority.challenge.expires_at_unix_seconds,
        authority_issued_at_unix_seconds: authority.challenge.issued_at_unix_seconds,
        authority_signature_sha256,
        authority_statement_sha256,
        authority_trust_policy_sha256: authority_trust.policy_sha256().to_string(),
        epoch: authority.challenge.epoch.clone(),
        install_inventory: authority.challenge.install_inventory.clone(),
        lease_nonce: lease.challenge.lease_nonce.clone(),
        lease_expires_at_unix_seconds: lease.challenge.expires_at_unix_seconds,
        lease_issued_at_unix_seconds: lease.challenge.issued_at_unix_seconds,
        lease_signature_sha256,
        lease_statement_sha256,
        lease_trust_policy_sha256: lease_trust.policy_sha256().to_string(),
        model_verified_at_unix_seconds: now_unix_seconds,
        predecessor: authority.challenge.predecessor.clone(),
        reserved_successor_revision: lease.challenge.reserved_successor_revision,
        state_root_profile: authority.challenge.state_root_profile.clone(),
        target_host: authority.challenge.target_host.clone(),
    };
    let authority_claim = verified.authority_nonce_claim_binding_sha256();
    let lease_claim = verified.lease_nonce_claim_binding_sha256();
    let preparation_bundle_id = verified.preparation_bundle_id_sha256();
    let preparation_bundle_binding = verified.preparation_bundle_binding_sha256();
    replay_guard.claim_pair_and_bundle_or_exact_recovery(
        (
            verified.authority_nonce(),
            INSTALL_EPOCH_AUTHORITY_CLAIM_SCOPE_V1,
            &authority_claim,
        ),
        (
            verified.lease_nonce(),
            EXTERNAL_WATERMARK_LEASE_CLAIM_SCOPE_V1,
            &lease_claim,
        ),
        &preparation_bundle_id,
        &preparation_bundle_binding,
    )?;
    Ok(verified)
}

fn validate_independent_trust_bindings_v1(
    authority_trust: &VerifiedTrustPolicyBindingV8,
    lease_trust: &VerifiedTrustPolicyBindingV8,
) -> Result<(), QualificationError> {
    if authority_trust.policy_sha256() == lease_trust.policy_sha256()
        || authority_trust.trust_root_id() == lease_trust.trust_root_id()
        || authority_trust.key_fingerprint() == lease_trust.key_fingerprint()
    {
        return Err(invalid(
            "install authority and external watermark provider are not independent trust roots and keys",
        ));
    }
    Ok(())
}

fn canonical_install_epoch_authority_statement_with_trust_v1(
    challenge: &InstallEpochAuthorityChallengeV1,
    trust: &VerifiedTrustPolicyBindingV8,
    expected_state_root_profile: &StateRootProfileBindingV1,
    expected_provider_profile: &ExternalWatermarkProviderProfileV1,
) -> Result<Vec<u8>, QualificationError> {
    validate_authority_challenge(
        challenge,
        trust,
        expected_state_root_profile,
        expected_provider_profile,
    )?;
    let mut statement = b"hepta_linux_v8_install_epoch_authority_statement_v1\0".to_vec();
    append_field(&mut statement, "schema", challenge.schema.as_bytes());
    append_field(
        &mut statement,
        "authority_nonce",
        challenge.authority_nonce.as_bytes(),
    );
    append_u64(
        &mut statement,
        "issued_at_unix_seconds",
        challenge.issued_at_unix_seconds,
    );
    append_u64(
        &mut statement,
        "expires_at_unix_seconds",
        challenge.expires_at_unix_seconds,
    );
    append_field(&mut statement, "namespace", challenge.namespace.as_bytes());
    append_signer(&mut statement, &challenge.signer);
    append_field(
        &mut statement,
        "capability",
        b"install_files_and_create_inert_state_root_only_no_reload_enable_start_or_execution",
    );
    append_field(
        &mut statement,
        "machine_id_sha256",
        challenge.target_host.machine_id_sha256.as_bytes(),
    );
    append_epoch(&mut statement, &challenge.epoch);
    append_predecessor(&mut statement, &challenge.predecessor);
    append_state_root_profile(&mut statement, &challenge.state_root_profile);
    append_install_inventory(&mut statement, &challenge.install_inventory);
    Ok(statement)
}

fn canonical_external_watermark_lease_statement_with_trust_v1(
    challenge: &ExternalWatermarkLeaseChallengeV1,
    trust: &VerifiedTrustPolicyBindingV8,
    expected_state_root_profile: &StateRootProfileBindingV1,
    expected_provider_profile: &ExternalWatermarkProviderProfileV1,
) -> Result<Vec<u8>, QualificationError> {
    validate_lease_challenge(
        challenge,
        trust,
        expected_state_root_profile,
        expected_provider_profile,
    )?;
    let mut statement = b"hepta_linux_v8_external_watermark_lease_statement_v1\0".to_vec();
    append_field(&mut statement, "schema", challenge.schema.as_bytes());
    append_field(
        &mut statement,
        "authority_nonce",
        challenge.authority_nonce.as_bytes(),
    );
    append_field(
        &mut statement,
        "lease_nonce",
        challenge.lease_nonce.as_bytes(),
    );
    append_u64(
        &mut statement,
        "issued_at_unix_seconds",
        challenge.issued_at_unix_seconds,
    );
    append_u64(
        &mut statement,
        "expires_at_unix_seconds",
        challenge.expires_at_unix_seconds,
    );
    append_field(&mut statement, "namespace", challenge.namespace.as_bytes());
    append_signer(&mut statement, &challenge.signer);
    append_field(
        &mut statement,
        "capability",
        b"reserve_exact_next_external_watermark_revision_for_inert_install_only",
    );
    append_field(
        &mut statement,
        "machine_id_sha256",
        challenge.target_host.machine_id_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "install_authority_statement_sha256",
        challenge.install_authority_statement_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "install_authority_trust_policy_sha256",
        challenge.install_authority_trust_policy_sha256.as_bytes(),
    );
    append_epoch(&mut statement, &challenge.epoch);
    append_predecessor(&mut statement, &challenge.predecessor);
    append_field(
        &mut statement,
        "provider_trust_policy_sha256",
        challenge.provider_trust_policy_sha256.as_bytes(),
    );
    append_u64(
        &mut statement,
        "reserved_successor_revision",
        challenge.reserved_successor_revision,
    );
    append_field(
        &mut statement,
        "state_root_profile_sha256",
        challenge.state_root_profile_sha256.as_bytes(),
    );
    Ok(statement)
}

fn validate_authority_challenge(
    challenge: &InstallEpochAuthorityChallengeV1,
    trust: &VerifiedTrustPolicyBindingV8,
    expected_state_root_profile: &StateRootProfileBindingV1,
    expected_provider_profile: &ExternalWatermarkProviderProfileV1,
) -> Result<(), QualificationError> {
    if challenge.schema != INSTALL_EPOCH_AUTHORITY_SCHEMA_V1
        || challenge.namespace != INSTALL_EPOCH_AUTHORITY_NAMESPACE_V1
        || trust.purpose() != SshsigTrustPurposeV8::InstallEpochAuthorityV1
        || trust.namespace() != INSTALL_EPOCH_AUTHORITY_NAMESPACE_V1
        || !signer_matches_trust(&challenge.signer, trust)?
        || challenge.activation
            != InstallEpochActivationV1::InstallFilesAndCreateInertStateRootOnlyNoReloadEnableStartOrExecution
    {
        return Err(invalid("install-epoch authority family or trust binding is not exact"));
    }
    validate_digest("install-epoch authority nonce", &challenge.authority_nonce)?;
    validate_epoch(&challenge.epoch)?;
    validate_predecessor(
        &challenge.predecessor,
        &challenge.target_host,
        expected_state_root_profile,
        expected_provider_profile,
    )?;
    if challenge.epoch.epoch_sequence
        != challenge
            .predecessor
            .installed_epoch_sequence()
            .checked_add(1)
            .ok_or_else(|| invalid("install epoch sequence overflow"))?
    {
        return Err(invalid(
            "install epoch does not immediately succeed the external predecessor",
        ));
    }
    validate_target_host(&challenge.target_host)?;
    validate_state_root_profile(&challenge.state_root_profile, expected_state_root_profile)?;
    validate_install_inventory(&challenge.install_inventory)?;
    if challenge
        .install_inventory
        .state_root
        .layout_manifest_sha256
        != challenge.state_root_profile.layout_manifest_sha256
        || challenge.install_inventory.state_root.path != challenge.state_root_profile.path
        || challenge.install_inventory.state_root.uid != challenge.state_root_profile.uid
        || challenge.install_inventory.state_root.gid != challenge.state_root_profile.gid
        || challenge.install_inventory.state_root.mode != challenge.state_root_profile.mode
    {
        return Err(invalid("install inventory and state-root profile differ"));
    }
    validate_interval_shape(
        challenge.issued_at_unix_seconds,
        challenge.expires_at_unix_seconds,
        MAX_INSTALL_EPOCH_AUTHORITY_LIFETIME_SECONDS_V1,
        "install-epoch authority",
    )
}

fn validate_lease_challenge(
    challenge: &ExternalWatermarkLeaseChallengeV1,
    trust: &VerifiedTrustPolicyBindingV8,
    expected_state_root_profile: &StateRootProfileBindingV1,
    expected_provider_profile: &ExternalWatermarkProviderProfileV1,
) -> Result<(), QualificationError> {
    if challenge.schema != EXTERNAL_WATERMARK_LEASE_SCHEMA_V1
        || challenge.namespace != EXTERNAL_WATERMARK_LEASE_NAMESPACE_V1
        || trust.purpose() != SshsigTrustPurposeV8::ExternalWatermarkLeaseV1
        || trust.namespace() != EXTERNAL_WATERMARK_LEASE_NAMESPACE_V1
        || !signer_matches_trust(&challenge.signer, trust)?
    {
        return Err(invalid(
            "external watermark lease family or trust binding is not exact",
        ));
    }
    validate_digest("lease authority nonce", &challenge.authority_nonce)?;
    validate_digest("external watermark lease nonce", &challenge.lease_nonce)?;
    validate_digest(
        "install authority statement",
        &challenge.install_authority_statement_sha256,
    )?;
    validate_digest(
        "install authority trust policy",
        &challenge.install_authority_trust_policy_sha256,
    )?;
    validate_digest(
        "external provider trust policy",
        &challenge.provider_trust_policy_sha256,
    )?;
    validate_digest(
        "lease state-root profile",
        &challenge.state_root_profile_sha256,
    )?;
    validate_epoch(&challenge.epoch)?;
    validate_predecessor(
        &challenge.predecessor,
        &challenge.target_host,
        expected_state_root_profile,
        expected_provider_profile,
    )?;
    if challenge.provider_trust_policy_sha256 != trust.policy_sha256()
        || challenge.predecessor.provider_profile_sha256()
            != expected_provider_profile.profile_sha256
        || expected_provider_profile.trust_policy_sha256 != trust.policy_sha256()
        || challenge.state_root_profile_sha256 != expected_state_root_profile.profile_sha256
    {
        return Err(invalid(
            "external lease differs from its frozen provider or state-root profile",
        ));
    }
    if challenge.epoch.epoch_sequence
        != challenge
            .predecessor
            .installed_epoch_sequence()
            .checked_add(1)
            .ok_or_else(|| invalid("install epoch sequence overflow"))?
    {
        return Err(invalid(
            "leased install epoch does not immediately succeed the external predecessor",
        ));
    }
    validate_target_host(&challenge.target_host)?;
    if challenge.reserved_successor_revision
        != challenge
            .predecessor
            .revision()
            .checked_add(1)
            .ok_or_else(|| invalid("external watermark revision overflow"))?
    {
        return Err(invalid(
            "external watermark lease does not reserve exactly the next revision",
        ));
    }
    validate_interval_shape(
        challenge.issued_at_unix_seconds,
        challenge.expires_at_unix_seconds,
        MAX_EXTERNAL_WATERMARK_LEASE_LIFETIME_SECONDS_V1,
        "external watermark lease",
    )
}

fn validate_epoch(epoch: &InstallEpochBindingV1) -> Result<(), QualificationError> {
    if epoch.epoch_sequence == 0 {
        return Err(invalid("install epoch sequence must be non-zero"));
    }
    validate_digest("install epoch nonce", &epoch.epoch_nonce_sha256)
}

fn validate_predecessor(
    predecessor: &ExternalWatermarkPredecessorV1,
    host: &TargetHostBindingV8,
    state_root_profile: &StateRootProfileBindingV1,
    provider_profile: &ExternalWatermarkProviderProfileV1,
) -> Result<(), QualificationError> {
    validate_external_provider_profile(provider_profile)?;
    let expected_stream = external_watermark_stream_id_sha256(
        &host.machine_id_sha256,
        &state_root_profile.profile_sha256,
        &provider_profile.profile_sha256,
    )?;
    validate_digest(
        "external provider profile",
        predecessor.provider_profile_sha256(),
    )?;
    validate_digest("external watermark stream", predecessor.stream_id_sha256())?;
    validate_digest("external watermark tip", predecessor.tip_sha256())?;
    validate_digest(
        "installed epoch binding",
        predecessor.installed_epoch_binding_sha256(),
    )?;
    if predecessor.provider_profile_sha256() != provider_profile.profile_sha256
        || predecessor.stream_id_sha256() != expected_stream
    {
        return Err(invalid(
            "external predecessor differs from the frozen provider or derived stream",
        ));
    }
    match predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
            genesis_epoch_binding_sha256,
            revision,
            tip_sha256,
            ..
        } => {
            if *revision != 0
                || tip_sha256 != &provider_profile.genesis_tip_sha256
                || genesis_epoch_binding_sha256 != &provider_profile.genesis_epoch_binding_sha256
            {
                return Err(invalid(
                    "genesis predecessor is not the exact provider-pinned sentinel",
                ));
            }
        }
        ExternalWatermarkPredecessorV1::Successor {
            installed_epoch_sequence,
            revision,
            ..
        } => {
            if *revision == 0 || installed_epoch_sequence != revision {
                return Err(invalid(
                    "successor predecessor epoch and external revision are not identical",
                ));
            }
        }
    }
    Ok(())
}

fn validate_state_root_profile(
    profile: &StateRootProfileBindingV1,
    expected: &StateRootProfileBindingV1,
) -> Result<(), QualificationError> {
    if profile.profile_id != STATE_ROOT_PROFILE_ID_V1
        || profile.profile_revision == 0
        || profile.path != crate::STATE_ROOT_PATH_V8
        || profile.uid != 0
        || profile.gid != 0
        || profile.mode != 0o700
    {
        return Err(invalid("state-root profile identity is not exact"));
    }
    validate_digest("state-root profile", &profile.profile_sha256)?;
    validate_digest(
        "state-root layout manifest",
        &profile.layout_manifest_sha256,
    )?;
    if profile.profile_sha256 != state_root_profile_sha256(profile) || profile != expected {
        return Err(invalid(
            "state-root profile differs from its canonical digest or compiled binding",
        ));
    }
    Ok(())
}

fn validate_external_provider_profile(
    profile: &ExternalWatermarkProviderProfileV1,
) -> Result<(), QualificationError> {
    if profile.profile_id != EXTERNAL_WATERMARK_PROVIDER_PROFILE_ID_V1
        || profile.profile_revision == 0
        || profile.trust_policy_sha256.len() != 64
        || profile.profile_sha256 != external_provider_profile_sha256(profile)
    {
        return Err(invalid(
            "external watermark provider profile is not the compiled canonical binding",
        ));
    }
    validate_digest(
        "external provider trust policy",
        &profile.trust_policy_sha256,
    )?;
    validate_digest("external provider genesis tip", &profile.genesis_tip_sha256)?;
    validate_digest(
        "external provider genesis epoch binding",
        &profile.genesis_epoch_binding_sha256,
    )
}

fn state_root_profile_sha256(profile: &StateRootProfileBindingV1) -> String {
    let mut bytes = b"hepta_linux_v8_state_root_profile_binding_v1\0".to_vec();
    append_field(&mut bytes, "profile_id", profile.profile_id.as_bytes());
    append_u64(&mut bytes, "profile_revision", profile.profile_revision);
    append_field(&mut bytes, "path", profile.path.as_bytes());
    append_u64(&mut bytes, "uid", u64::from(profile.uid));
    append_u64(&mut bytes, "gid", u64::from(profile.gid));
    append_u64(&mut bytes, "mode", u64::from(profile.mode));
    append_field(
        &mut bytes,
        "layout_manifest_sha256",
        profile.layout_manifest_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn external_provider_profile_sha256(profile: &ExternalWatermarkProviderProfileV1) -> String {
    let mut bytes = b"hepta_linux_v8_external_watermark_provider_profile_v1\0".to_vec();
    append_field(&mut bytes, "profile_id", profile.profile_id.as_bytes());
    append_u64(&mut bytes, "profile_revision", profile.profile_revision);
    append_field(
        &mut bytes,
        "trust_policy_sha256",
        profile.trust_policy_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "genesis_tip_sha256",
        profile.genesis_tip_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "genesis_epoch_binding_sha256",
        profile.genesis_epoch_binding_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn external_watermark_stream_id_sha256(
    machine_id_sha256: &str,
    state_root_profile_sha256: &str,
    provider_profile_sha256: &str,
) -> Result<String, QualificationError> {
    validate_digest("stream machine", machine_id_sha256)?;
    validate_digest("stream state-root profile", state_root_profile_sha256)?;
    validate_digest("stream provider profile", provider_profile_sha256)?;
    let mut bytes = b"hepta_linux_v8_install_epoch_external_stream_v1\0".to_vec();
    append_field(
        &mut bytes,
        "machine_id_sha256",
        machine_id_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "state_root_profile_sha256",
        state_root_profile_sha256.as_bytes(),
    );
    append_field(
        &mut bytes,
        "provider_profile_sha256",
        provider_profile_sha256.as_bytes(),
    );
    Ok(sha256(&bytes))
}

fn validate_target_host(host: &TargetHostBindingV8) -> Result<(), QualificationError> {
    validate_digest("install-epoch target host", &host.machine_id_sha256)
}

fn validate_install_inventory(
    inventory: &ExactRootInstallInventoryV8,
) -> Result<(), QualificationError> {
    validate_root_file(
        &inventory.admissiond_binary,
        crate::ADMISSIOND_INSTALL_PATH_V8,
        0o555,
    )?;
    validate_root_file(
        &inventory.recovery_binary,
        crate::RECOVERY_INSTALL_PATH_V8,
        0o555,
    )?;
    validate_root_file(
        &inventory.admissiond_unit,
        crate::ADMISSIOND_UNIT_PATH_V8,
        0o444,
    )?;
    validate_root_file(
        &inventory.recovery_unit,
        crate::RECOVERY_UNIT_PATH_V8,
        0o444,
    )?;
    if inventory.state_root.path != crate::STATE_ROOT_PATH_V8
        || inventory.state_root.uid != 0
        || inventory.state_root.gid != 0
        || inventory.state_root.mode != 0o700
    {
        return Err(invalid("install-epoch state-root inventory is not exact"));
    }
    validate_digest(
        "install-epoch layout manifest",
        &inventory.state_root.layout_manifest_sha256,
    )
}

fn validate_root_file(
    file: &RootFileInstallIdentityV8,
    path: &str,
    mode: u32,
) -> Result<(), QualificationError> {
    if file.path != path
        || file.uid != 0
        || file.gid != 0
        || file.mode != mode
        || file.size_bytes == 0
    {
        return Err(invalid(format!(
            "install-epoch root file identity for {path} is not exact"
        )));
    }
    validate_digest("install-epoch root file", &file.content_sha256)
}

fn signer_matches_trust(
    signer: &AuthoritySignerBindingV8,
    trust: &VerifiedTrustPolicyBindingV8,
) -> Result<bool, QualificationError> {
    signer.validate()?;
    Ok(
        signer.allowed_signers_sha256 == trust.allowed_signers_sha256()
            && signer.key_fingerprint == trust.key_fingerprint()
            && signer.principal == trust.principal()
            && signer.signature_algorithm == trust.signature_algorithm(),
    )
}

fn validate_signed_envelope(
    declared_statement_sha256: &str,
    statement: &[u8],
    declared_signature_sha256: &str,
    signature_bytes: &[u8],
    observation: &CryptographicSignatureObservation,
    trust: &VerifiedTrustPolicyBindingV8,
) -> Result<String, QualificationError> {
    let statement_sha256 = sha256(statement);
    if declared_statement_sha256 != statement_sha256 {
        return Err(invalid("signed statement digest is not canonical"));
    }
    if signature_bytes.is_empty() || signature_bytes.len() > MAX_SIGNATURE_BYTES_V1 {
        return Err(invalid("detached signature bytes are empty or oversized"));
    }
    let signature_sha256 = sha256(signature_bytes);
    if declared_signature_sha256 != signature_sha256
        || !observation.exactly_matches(&signature_sha256, &statement_sha256, trust)
    {
        return Err(invalid(
            "detached signature observation does not bind the exact statement and trust policy",
        ));
    }
    Ok(signature_sha256)
}

fn validate_time_window(
    issued: u64,
    expires: u64,
    maximum_lifetime: u64,
    now: u64,
    label: &str,
) -> Result<(), QualificationError> {
    validate_interval_shape(issued, expires, maximum_lifetime, label)?;
    if now < issued || now >= expires {
        return Err(invalid(format!("{label} is not currently valid")));
    }
    Ok(())
}

fn validate_interval_shape(
    issued: u64,
    expires: u64,
    maximum_lifetime: u64,
    label: &str,
) -> Result<(), QualificationError> {
    if issued >= expires || expires - issued > maximum_lifetime {
        return Err(invalid(format!("{label} validity interval is invalid")));
    }
    Ok(())
}

fn validate_digest(label: &str, value: &str) -> Result<(), QualificationError> {
    if !digest_shape(value) {
        return Err(invalid(format!(
            "{label} must be one non-zero lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && !value.bytes().all(|byte| byte == b'0')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn install_epoch_nonce_claim_binding_sha256_v1(
    scope: &str,
    nonce: &str,
    statement_sha256: &str,
    signature_sha256: &str,
    trust_policy_sha256: &str,
) -> String {
    let mut bytes = b"hepta_linux_v8_install_epoch_nonce_claim_binding_v1\0".to_vec();
    append_field(&mut bytes, "scope", scope.as_bytes());
    append_field(&mut bytes, "nonce", nonce.as_bytes());
    append_field(&mut bytes, "statement_sha256", statement_sha256.as_bytes());
    append_field(&mut bytes, "signature_sha256", signature_sha256.as_bytes());
    append_field(
        &mut bytes,
        "trust_policy_sha256",
        trust_policy_sha256.as_bytes(),
    );
    sha256(&bytes)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn append_signer(statement: &mut Vec<u8>, signer: &AuthoritySignerBindingV8) {
    append_field(
        statement,
        "allowed_signers_sha256",
        signer.allowed_signers_sha256.as_bytes(),
    );
    append_field(
        statement,
        "key_fingerprint",
        signer.key_fingerprint.as_bytes(),
    );
    append_field(statement, "principal", signer.principal.as_bytes());
    append_field(
        statement,
        "signature_algorithm",
        match signer.signature_algorithm {
            AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519 => b"openssh_sshsig_ed25519_sha256",
        },
    );
}

fn append_epoch(statement: &mut Vec<u8>, epoch: &InstallEpochBindingV1) {
    append_u64(statement, "epoch_sequence", epoch.epoch_sequence);
    append_field(
        statement,
        "epoch_nonce_sha256",
        epoch.epoch_nonce_sha256.as_bytes(),
    );
}

fn append_predecessor(statement: &mut Vec<u8>, predecessor: &ExternalWatermarkPredecessorV1) {
    append_field(
        statement,
        "predecessor.kind",
        predecessor.kind_name().as_bytes(),
    );
    append_u64(
        statement,
        "predecessor.installed_epoch_sequence",
        predecessor.installed_epoch_sequence(),
    );
    append_field(
        statement,
        "predecessor.installed_epoch_binding_sha256",
        predecessor.installed_epoch_binding_sha256().as_bytes(),
    );
    append_field(
        statement,
        "predecessor.provider_profile_sha256",
        predecessor.provider_profile_sha256().as_bytes(),
    );
    append_field(
        statement,
        "predecessor.stream_id_sha256",
        predecessor.stream_id_sha256().as_bytes(),
    );
    append_u64(statement, "predecessor.revision", predecessor.revision());
    append_field(
        statement,
        "predecessor.tip_sha256",
        predecessor.tip_sha256().as_bytes(),
    );
}

fn append_state_root_profile(statement: &mut Vec<u8>, profile: &StateRootProfileBindingV1) {
    append_field(
        statement,
        "state_root.profile_id",
        profile.profile_id.as_bytes(),
    );
    append_u64(
        statement,
        "state_root.profile_revision",
        profile.profile_revision,
    );
    append_field(
        statement,
        "state_root.profile_sha256",
        profile.profile_sha256.as_bytes(),
    );
    append_field(statement, "state_root.path", profile.path.as_bytes());
    append_u64(statement, "state_root.uid", u64::from(profile.uid));
    append_u64(statement, "state_root.gid", u64::from(profile.gid));
    append_u64(statement, "state_root.mode", u64::from(profile.mode));
    append_field(
        statement,
        "state_root.layout_manifest_sha256",
        profile.layout_manifest_sha256.as_bytes(),
    );
}

fn append_install_inventory(statement: &mut Vec<u8>, inventory: &ExactRootInstallInventoryV8) {
    append_root_file(statement, "admissiond_binary", &inventory.admissiond_binary);
    append_root_file(statement, "recovery_binary", &inventory.recovery_binary);
    append_root_file(statement, "admissiond_unit", &inventory.admissiond_unit);
    append_root_file(statement, "recovery_unit", &inventory.recovery_unit);
}

fn append_root_file(statement: &mut Vec<u8>, label: &str, file: &RootFileInstallIdentityV8) {
    append_field(statement, &format!("{label}.path"), file.path.as_bytes());
    append_field(
        statement,
        &format!("{label}.content_sha256"),
        file.content_sha256.as_bytes(),
    );
    append_u64(statement, &format!("{label}.uid"), u64::from(file.uid));
    append_u64(statement, &format!("{label}.gid"), u64::from(file.gid));
    append_u64(statement, &format!("{label}.mode"), u64::from(file.mode));
    append_u64(statement, &format!("{label}.size_bytes"), file.size_bytes);
}

fn append_field(statement: &mut Vec<u8>, name: &str, value: &[u8]) {
    statement.extend_from_slice(&(name.len() as u64).to_be_bytes());
    statement.extend_from_slice(name.as_bytes());
    statement.extend_from_slice(&(value.len() as u64).to_be_bytes());
    statement.extend_from_slice(value);
}

fn append_u64(statement: &mut Vec<u8>, name: &str, value: u64) {
    append_field(statement, name, &value.to_be_bytes());
}
