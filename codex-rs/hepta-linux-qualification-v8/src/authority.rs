use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::AttemptIdentityV8;
use crate::QualificationError;
#[cfg(test)]
use crate::SshsigTrustPurposeV8;
use crate::VerifiedTrustPolicyBindingV8;
use crate::authority_trust_purpose_v8;
use crate::invalid;
use crate::required_frozen_trust_binding_v8;
use crate::verify_signed_authority_sshsig_v8;

pub const AUTHORITY_SCHEMA_V8: &str = "hepta_linux_v8_signed_authority_v1";
pub const INSTALL_AUTHORITY_SCHEMA_V2: &str = "hepta_linux_v8_signed_install_authority_v2";
pub const ONE_SHOT_RUN_AUTHORITY_SCHEMA_V2: &str =
    "hepta_linux_v8_signed_one_shot_run_authority_v2";
pub const INSTALL_NAMESPACE_V8: &str = "hepta-linux-v8-install";
pub const INSTALL_NAMESPACE_V2: &str = "hepta-linux-v8-install-v2";
pub const ONE_SHOT_RUN_NAMESPACE_V8: &str = "hepta-linux-v8-execution";
pub const ONE_SHOT_RUN_NAMESPACE_V2: &str = "hepta-linux-v8-execution-v2";
pub const BREAK_GLASS_NAMESPACE_V8: &str = "hepta-linux-v8-break-glass";
pub const MAX_AUTHORITY_LIFETIME_SECONDS_V8: u64 = 15 * 60;

pub const ADMISSIOND_INSTALL_PATH_V8: &str = "/usr/local/libexec/hepta-linux-v8-admissiond";
pub const RECOVERY_INSTALL_PATH_V8: &str = "/usr/local/libexec/hepta-linux-v8-recover";
pub const ADMISSIOND_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-admissiond.service";
pub const RECOVERY_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-recover.service";
pub const ADMISSIOND_UNIT_NAME_V2: &str = "hepta-linux-v8-admissiond.service";
pub const STATE_ROOT_PATH_V8: &str = "/var/lib/hepta-linux-v8";
pub const INSTALL_BINARY_DIRECTORY_PATH_V2: &str = "/usr/local/libexec/hepta-linux-v8";
pub const CTL_INSTALL_PATH_V2: &str = "/usr/local/libexec/hepta-linux-v8/hepta-linux-v8ctl";
pub const ADMISSIOND_INSTALL_PATH_V2: &str =
    "/usr/local/libexec/hepta-linux-v8/hepta-linux-v8-admissiond";
pub const RECOVERY_INSTALL_PATH_V2: &str =
    "/usr/local/libexec/hepta-linux-v8/hepta-linux-v8-recover";
pub const ATTEMPTS_DIRECTORY_PATH_V2: &str = "/var/lib/hepta-linux-v8/attempts";
pub const INSTALL_EPOCH_DIRECTORY_PATH_V2: &str = "/var/lib/hepta-linux-v8/install-epoch";
pub const JOURNAL_DIRECTORY_PATH_V2: &str = "/var/lib/hepta-linux-v8/journal";
pub const NONCE_CLAIMS_DIRECTORY_PATH_V2: &str = "/var/lib/hepta-linux-v8/nonce-claims";
pub const QUARANTINE_DIRECTORY_PATH_V2: &str = "/var/lib/hepta-linux-v8/quarantine";
pub const STATE_LOCK_PATH_V2: &str = "/var/lib/hepta-linux-v8/state.lock";

const MAX_SIGNATURE_BYTES: usize = 16 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetHostBindingV8 {
    pub machine_id_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootFileInstallIdentityV8 {
    pub content_sha256: String,
    pub gid: u32,
    pub mode: u32,
    pub path: String,
    pub size_bytes: u64,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootStateIdentityV8 {
    pub gid: u32,
    pub layout_manifest_sha256: String,
    pub mode: u32,
    pub path: String,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRootInstallInventoryV8 {
    pub admissiond_binary: RootFileInstallIdentityV8,
    pub admissiond_unit: RootFileInstallIdentityV8,
    pub recovery_binary: RootFileInstallIdentityV8,
    pub recovery_unit: RootFileInstallIdentityV8,
    pub state_root: RootStateIdentityV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootDirectoryInstallIdentityV2 {
    pub gid: u32,
    pub mode: u32,
    pub path: String,
    pub uid: u32,
}

/// Complete v2 install inventory. The old four-file v1 inventory remains
/// frozen for historical model receipts and is never accepted by the native
/// formal installer.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRootInstallInventoryV2 {
    pub ctl_binary: RootFileInstallIdentityV8,
    pub admissiond_binary: RootFileInstallIdentityV8,
    pub recovery_binary: RootFileInstallIdentityV8,
    pub admissiond_unit: RootFileInstallIdentityV8,
    pub recovery_unit: RootFileInstallIdentityV8,
    pub binary_directory: RootDirectoryInstallIdentityV2,
    pub state_root: RootStateIdentityV8,
    pub attempts_directory: RootDirectoryInstallIdentityV2,
    pub install_epoch_directory: RootDirectoryInstallIdentityV2,
    pub journal_directory: RootDirectoryInstallIdentityV2,
    pub nonce_claims_directory: RootDirectoryInstallIdentityV2,
    pub quarantine_directory: RootDirectoryInstallIdentityV2,
    pub state_lock: RootFileInstallIdentityV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallTargetHostBindingV2 {
    pub boot_id: String,
    pub machine_id_sha256: String,
}

/// The state transition authorized by an InstallV2 statement. Native v8
/// currently executes only `FreshEmpty`; the exact-upgrade shape is reserved
/// so a future provider-backed upgrader cannot silently reuse fresh-install
/// authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case", deny_unknown_fields)]
pub enum InstallStateDispositionV2 {
    FreshEmpty,
    ExactUpgrade {
        predecessor_install_epoch_sha256: String,
        predecessor_provider_tip_sha256: String,
        predecessor_state_sha256: String,
    },
}

/// Boot- and namespace-bound host observation required by one-shot v2. This
/// deliberately differs from the frozen v1 machine-only binding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunTargetHostBindingV2 {
    pub boot_id: String,
    pub cgroup_namespace_inode: u64,
    pub machine_id_sha256: String,
    pub mount_namespace_inode: u64,
    pub pid_namespace_inode: u64,
    pub systemd_manager_pid: u32,
    pub systemd_manager_start_time_ticks: u64,
    pub systemd_unit_fragment_sha256: String,
    pub systemd_unit_name: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallActivationV8 {
    InstallFilesOnlyNoDaemonReloadEnableOrStart,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DriverPeerBindingV8 {
    pub executable_sha256: String,
    pub gid: u32,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryStateBindingV8 {
    pub boot_epoch: u64,
    pub boot_id: String,
    pub journal_tip_sha256: String,
    pub restore_state_sha256: String,
    pub runner_snapshot_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OneShotRunCapabilityV8 {
    Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakGlassCapabilityV8 {
    ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
}

/// The only encodable authority scopes. Operations omitted from these closed
/// variants are not granted by a signed authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "scope", rename_all = "snake_case", deny_unknown_fields)]
pub enum AuthorityScopeV8 {
    Install {
        activation: InstallActivationV8,
        inventory: ExactRootInstallInventoryV8,
        target_host: TargetHostBindingV8,
    },
    InstallV2 {
        activation: InstallActivationV8,
        install_plan_sha256: String,
        inventory: Box<ExactRootInstallInventoryV2>,
        state_disposition: InstallStateDispositionV2,
        target_host: InstallTargetHostBindingV2,
    },
    OneShotRun {
        attempt: AttemptIdentityV8,
        capability: OneShotRunCapabilityV8,
        driver_peer: DriverPeerBindingV8,
        target_host: TargetHostBindingV8,
    },
    OneShotRunV2 {
        attempt: AttemptIdentityV8,
        capability: OneShotRunCapabilityV8,
        containment: crate::CandidateContainmentProfileV2,
        driver_peer: DriverPeerBindingV8,
        target_host: RunTargetHostBindingV2,
    },
    BreakGlass {
        attempt: AttemptIdentityV8,
        capability: BreakGlassCapabilityV8,
        recovery_state: RecoveryStateBindingV8,
        restore_plan_sha256: String,
        target_host: TargetHostBindingV8,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySignatureAlgorithmV8 {
    OpenSshSshsigEd25519,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoritySignerBindingV8 {
    pub allowed_signers_sha256: String,
    pub key_fingerprint: String,
    pub principal: String,
    pub signature_algorithm: AuthoritySignatureAlgorithmV8,
}

impl AuthoritySignerBindingV8 {
    pub(crate) fn validate(&self) -> Result<(), QualificationError> {
        if self.principal.is_empty()
            || self.principal.len() > 256
            || !self.principal.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'@' | b'.' | b'_' | b'+' | b'-')
            })
        {
            return Err(invalid("authority signer principal is malformed"));
        }
        let fingerprint = self
            .key_fingerprint
            .strip_prefix("SHA256:")
            .ok_or_else(|| invalid("authority signer fingerprint is malformed"))?;
        if fingerprint.len() != 43
            || !fingerprint
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/'))
        {
            return Err(invalid("authority signer fingerprint is malformed"));
        }
        if !digest_shape(&self.allowed_signers_sha256) {
            return Err(invalid("allowed-signers digest is malformed"));
        }
        Ok(())
    }

    fn exactly_matches_trust_policy(&self, policy: &VerifiedTrustPolicyBindingV8) -> bool {
        self.allowed_signers_sha256 == policy.allowed_signers_sha256()
            && self.key_fingerprint == policy.key_fingerprint()
            && self.principal == policy.principal()
            && self.signature_algorithm == policy.signature_algorithm()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityChallengeV8 {
    pub authority_nonce: String,
    pub expires_at_unix_seconds: u64,
    pub issued_at_unix_seconds: u64,
    pub namespace: String,
    pub schema: String,
    pub scope: AuthorityScopeV8,
    pub signer: AuthoritySignerBindingV8,
}

impl AuthorityChallengeV8 {
    pub(crate) fn scope_kind(&self) -> AuthorityScopeKindV8 {
        self.scope.kind()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedAuthorityV8 {
    pub canonical_statement_sha256: String,
    pub challenge: AuthorityChallengeV8,
    pub detached_signature_bytes: Vec<u8>,
    pub detached_signature_sha256: String,
}

/// Opaque result produced by the trusted SSHSIG verification boundary.
///
/// Its fields are deliberately private and the type is not serializable: a
/// caller cannot turn a self-reported `signature_verified` value into trusted
/// state. The native verifier slice is the only production constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptographicSignatureObservation {
    signature_sha256: String,
    signed_statement_sha256: String,
    verified_trust_policy: VerifiedTrustPolicyBindingV8,
}

impl CryptographicSignatureObservation {
    pub(crate) fn from_verified_sshsig(
        signature_sha256: String,
        signed_statement_sha256: String,
        verified_trust_policy: VerifiedTrustPolicyBindingV8,
    ) -> Self {
        Self {
            signature_sha256,
            signed_statement_sha256,
            verified_trust_policy,
        }
    }

    pub(crate) fn exactly_matches(
        &self,
        signature_sha256: &str,
        statement_sha256: &str,
        trust_policy: &VerifiedTrustPolicyBindingV8,
    ) -> bool {
        self.signature_sha256 == signature_sha256
            && self.signed_statement_sha256 == statement_sha256
            && &self.verified_trust_policy == trust_policy
    }

    #[cfg(test)]
    pub(crate) fn for_test_only(
        signature_sha256: String,
        signed_statement_sha256: String,
        purpose: SshsigTrustPurposeV8,
    ) -> Self {
        Self::from_verified_sshsig(
            signature_sha256,
            signed_statement_sha256,
            crate::test_only_trust_binding_v8(purpose),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityScopeKindV8 {
    Install,
    InstallV2,
    OneShotRun,
    OneShotRunV2,
    BreakGlass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAuthorityV8 {
    authority_nonce: String,
    consumed_at_unix_seconds: u64,
    detached_signature_sha256: String,
    namespace: String,
    scope: AuthorityScopeV8,
    statement_sha256: String,
    trust_policy: VerifiedTrustPolicyBindingV8,
}

/// Opaque observation required to consume OneShotRunV2 authority. There is no
/// production constructor until the native admission guardian can bind all of
/// these facts from retained pid/mount/cgroup/systemd handles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedOneShotEnvironmentV2 {
    attempt: AttemptIdentityV8,
    containment: crate::CandidateContainmentProfileV2,
    driver_peer: DriverPeerBindingV8,
    target_host: RunTargetHostBindingV2,
}

impl VerifiedOneShotEnvironmentV2 {
    #[cfg(test)]
    pub(crate) fn for_test_only(
        attempt: AttemptIdentityV8,
        containment: crate::CandidateContainmentProfileV2,
        driver_peer: DriverPeerBindingV8,
        target_host: RunTargetHostBindingV2,
    ) -> Self {
        Self {
            attempt,
            containment,
            driver_peer,
            target_host,
        }
    }
}

impl VerifiedAuthorityV8 {
    pub fn scope_kind(&self) -> AuthorityScopeKindV8 {
        self.scope.kind()
    }

    pub fn authority_nonce(&self) -> &str {
        &self.authority_nonce
    }

    pub fn consumed_at_unix_seconds(&self) -> u64 {
        self.consumed_at_unix_seconds
    }

    pub fn detached_signature_sha256(&self) -> &str {
        &self.detached_signature_sha256
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn statement_sha256(&self) -> &str {
        &self.statement_sha256
    }

    pub fn trust_policy_binding(&self) -> &VerifiedTrustPolicyBindingV8 {
        &self.trust_policy
    }

    pub fn authorized_install_v2(
        &self,
    ) -> Option<(
        &str,
        &InstallStateDispositionV2,
        &ExactRootInstallInventoryV2,
        &InstallTargetHostBindingV2,
    )> {
        match &self.scope {
            AuthorityScopeV8::InstallV2 {
                activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
                install_plan_sha256,
                inventory,
                state_disposition,
                target_host,
            } => Some((
                install_plan_sha256,
                state_disposition,
                inventory.as_ref(),
                target_host,
            )),
            _ => None,
        }
    }

    pub(crate) fn authorizes_one_shot(&self, attempt: &AttemptIdentityV8) -> bool {
        matches!(
            &self.scope,
            AuthorityScopeV8::OneShotRun {
                attempt: authorized,
                capability:
                    OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
                driver_peer: _,
                target_host,
            } if authorized == attempt && target_host.machine_id_sha256 == attempt.machine_id_sha256
        )
    }

    pub fn authorizes_one_shot_v2(&self, observation: &VerifiedOneShotEnvironmentV2) -> bool {
        matches!(
            &self.scope,
            AuthorityScopeV8::OneShotRunV2 {
                attempt: authorized,
                capability:
                    OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
                containment: authorized_containment,
                driver_peer: authorized_driver_peer,
                target_host,
            } if authorized == &observation.attempt
                && authorized_containment == &observation.containment
                && authorized_driver_peer == &observation.driver_peer
                && target_host == &observation.target_host
                && target_host.machine_id_sha256 == observation.attempt.machine_id_sha256
        )
    }

    pub(crate) fn authorizes_break_glass(
        &self,
        attempt: &AttemptIdentityV8,
        current_state: &RecoveryStateBindingV8,
    ) -> bool {
        matches!(
            &self.scope,
            AuthorityScopeV8::BreakGlass {
                attempt: authorized,
                capability:
                    BreakGlassCapabilityV8::ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
                recovery_state,
                restore_plan_sha256,
                target_host,
            } if authorized == attempt
                && restore_plan_sha256 == &attempt.restore_plan_sha256
                && target_host.machine_id_sha256 == attempt.machine_id_sha256
                && recovery_state == current_state
        )
    }
}

/// In-memory model of the durable nonce-claim store required by the caller.
/// Production orchestration must persist the successful claim atomically.
#[derive(Debug, Default)]
pub struct AuthorityReplayGuardV8 {
    consumed_nonces: BTreeSet<String>,
}

impl AuthorityReplayGuardV8 {
    pub fn from_consumed_nonces(
        consumed_nonces: impl IntoIterator<Item = String>,
    ) -> Result<Self, QualificationError> {
        let consumed_nonces = consumed_nonces.into_iter().collect::<BTreeSet<_>>();
        if consumed_nonces.iter().any(|nonce| !digest_shape(nonce)) {
            return Err(invalid("persisted authority nonce claim is malformed"));
        }
        Ok(Self { consumed_nonces })
    }

    pub fn is_consumed(&self, nonce: &str) -> bool {
        self.consumed_nonces.contains(nonce)
    }
}

impl AuthorityScopeV8 {
    fn kind(&self) -> AuthorityScopeKindV8 {
        match self {
            Self::Install { .. } => AuthorityScopeKindV8::Install,
            Self::InstallV2 { .. } => AuthorityScopeKindV8::InstallV2,
            Self::OneShotRun { .. } => AuthorityScopeKindV8::OneShotRun,
            Self::OneShotRunV2 { .. } => AuthorityScopeKindV8::OneShotRunV2,
            Self::BreakGlass { .. } => AuthorityScopeKindV8::BreakGlass,
        }
    }

    fn namespace(&self) -> &'static str {
        match self {
            Self::Install { .. } => INSTALL_NAMESPACE_V8,
            Self::InstallV2 { .. } => INSTALL_NAMESPACE_V2,
            Self::OneShotRun { .. } => ONE_SHOT_RUN_NAMESPACE_V8,
            Self::OneShotRunV2 { .. } => ONE_SHOT_RUN_NAMESPACE_V2,
            Self::BreakGlass { .. } => BREAK_GLASS_NAMESPACE_V8,
        }
    }
}

/// Builds the exact bytes that an external SSHSIG verifier must verify.
pub fn canonical_authority_statement_v8(
    challenge: &AuthorityChallengeV8,
) -> Result<Vec<u8>, QualificationError> {
    let purpose = authority_trust_purpose_v8(challenge.scope_kind());
    let trust_policy = required_frozen_trust_binding_v8(purpose)?;
    canonical_authority_statement_with_trust_v8(challenge, &trust_policy)
}

#[cfg(test)]
pub(crate) fn canonical_authority_statement_for_test_v8(
    challenge: &AuthorityChallengeV8,
) -> Result<Vec<u8>, QualificationError> {
    let purpose = authority_trust_purpose_v8(challenge.scope_kind());
    canonical_authority_statement_with_trust_v8(
        challenge,
        &crate::test_only_trust_binding_v8(purpose),
    )
}

fn canonical_authority_statement_with_trust_v8(
    challenge: &AuthorityChallengeV8,
    trust_policy: &VerifiedTrustPolicyBindingV8,
) -> Result<Vec<u8>, QualificationError> {
    validate_challenge(challenge, trust_policy)?;
    let mut statement = match challenge.scope_kind() {
        AuthorityScopeKindV8::InstallV2 | AuthorityScopeKindV8::OneShotRunV2 => {
            b"hepta-linux-v8-authority-statement-v2\0".to_vec()
        }
        AuthorityScopeKindV8::Install
        | AuthorityScopeKindV8::OneShotRun
        | AuthorityScopeKindV8::BreakGlass => b"hepta-linux-v8-authority-statement-v1\0".to_vec(),
    };
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
    append_field(
        &mut statement,
        "principal",
        challenge.signer.principal.as_bytes(),
    );
    append_field(
        &mut statement,
        "key_fingerprint",
        challenge.signer.key_fingerprint.as_bytes(),
    );
    append_field(
        &mut statement,
        "allowed_signers_sha256",
        challenge.signer.allowed_signers_sha256.as_bytes(),
    );
    append_field(
        &mut statement,
        "signature_algorithm",
        b"openssh_sshsig_ed25519",
    );
    append_scope(&mut statement, &challenge.scope)?;
    Ok(statement)
}

pub fn verify_signed_authority_v8(
    signed: &SignedAuthorityV8,
    now_unix_seconds: u64,
    replay_guard: &mut AuthorityReplayGuardV8,
) -> Result<VerifiedAuthorityV8, QualificationError> {
    let statement = canonical_authority_statement_v8(&signed.challenge)?;
    let observation = verify_signed_authority_sshsig_v8(signed)?;
    let purpose = authority_trust_purpose_v8(signed.challenge.scope_kind());
    let trust_policy = required_frozen_trust_binding_v8(purpose)?;
    verify_signed_authority_with_evidence_v8(
        signed,
        &observation,
        &statement,
        trust_policy,
        now_unix_seconds,
        replay_guard,
    )
}

#[cfg(test)]
pub(crate) fn verify_signed_authority_with_observation_for_test_v8(
    signed: &SignedAuthorityV8,
    observation: &CryptographicSignatureObservation,
    now_unix_seconds: u64,
    replay_guard: &mut AuthorityReplayGuardV8,
) -> Result<VerifiedAuthorityV8, QualificationError> {
    let statement = canonical_authority_statement_for_test_v8(&signed.challenge)?;
    let purpose = authority_trust_purpose_v8(signed.challenge.scope_kind());
    verify_signed_authority_with_evidence_v8(
        signed,
        observation,
        &statement,
        crate::test_only_trust_binding_v8(purpose),
        now_unix_seconds,
        replay_guard,
    )
}

fn verify_signed_authority_with_evidence_v8(
    signed: &SignedAuthorityV8,
    observation: &CryptographicSignatureObservation,
    statement: &[u8],
    trust_policy: VerifiedTrustPolicyBindingV8,
    now_unix_seconds: u64,
    replay_guard: &mut AuthorityReplayGuardV8,
) -> Result<VerifiedAuthorityV8, QualificationError> {
    let statement_sha256 = sha256(statement);
    if !digest_shape(&signed.canonical_statement_sha256)
        || signed.canonical_statement_sha256 != statement_sha256
    {
        return Err(invalid(
            "authority statement differs from its canonical digest",
        ));
    }
    if signed.detached_signature_bytes.is_empty()
        || signed.detached_signature_bytes.len() > MAX_SIGNATURE_BYTES
    {
        return Err(invalid("detached signature bytes are empty or oversized"));
    }
    let signature_sha256 = sha256(&signed.detached_signature_bytes);
    if !digest_shape(&signed.detached_signature_sha256)
        || signed.detached_signature_sha256 != signature_sha256
    {
        return Err(invalid(
            "detached signature bytes differ from their pinned digest",
        ));
    }
    if now_unix_seconds < signed.challenge.issued_at_unix_seconds {
        return Err(invalid("authority is not yet valid"));
    }
    if now_unix_seconds >= signed.challenge.expires_at_unix_seconds {
        return Err(invalid("authority is stale"));
    }
    if replay_guard.is_consumed(&signed.challenge.authority_nonce) {
        return Err(invalid("authority nonce has already been consumed"));
    }
    if !observation.exactly_matches(&signature_sha256, &statement_sha256, &trust_policy) {
        return Err(invalid(
            "cryptographic verification observation does not bind the exact authority",
        ));
    }

    replay_guard
        .consumed_nonces
        .insert(signed.challenge.authority_nonce.clone());
    Ok(VerifiedAuthorityV8 {
        authority_nonce: signed.challenge.authority_nonce.clone(),
        consumed_at_unix_seconds: now_unix_seconds,
        detached_signature_sha256: signature_sha256,
        namespace: signed.challenge.namespace.clone(),
        scope: signed.challenge.scope.clone(),
        statement_sha256,
        trust_policy,
    })
}

fn validate_challenge(
    challenge: &AuthorityChallengeV8,
    trust_policy: &VerifiedTrustPolicyBindingV8,
) -> Result<(), QualificationError> {
    let expected_schema = match challenge.scope_kind() {
        AuthorityScopeKindV8::InstallV2 => INSTALL_AUTHORITY_SCHEMA_V2,
        AuthorityScopeKindV8::OneShotRunV2 => ONE_SHOT_RUN_AUTHORITY_SCHEMA_V2,
        AuthorityScopeKindV8::Install
        | AuthorityScopeKindV8::OneShotRun
        | AuthorityScopeKindV8::BreakGlass => AUTHORITY_SCHEMA_V8,
    };
    if challenge.schema != expected_schema {
        return Err(invalid("authority schema is not Linux v8"));
    }
    if challenge.namespace != challenge.scope.namespace() {
        return Err(invalid(
            "authority namespace does not match its closed scope",
        ));
    }
    if challenge.namespace != trust_policy.namespace()
        || authority_trust_purpose_v8(challenge.scope_kind()) != trust_policy.purpose()
        || !challenge.signer.exactly_matches_trust_policy(trust_policy)
    {
        return Err(invalid(
            "authority signer and namespace differ from the frozen trust policy",
        ));
    }
    if !digest_shape(&challenge.authority_nonce) {
        return Err(invalid("authority nonce is not a lowercase SHA-256 value"));
    }
    if challenge.issued_at_unix_seconds >= challenge.expires_at_unix_seconds {
        return Err(invalid("authority validity interval is empty or reversed"));
    }
    if challenge.expires_at_unix_seconds - challenge.issued_at_unix_seconds
        > MAX_AUTHORITY_LIFETIME_SECONDS_V8
    {
        return Err(invalid("authority lifetime exceeds the Linux v8 maximum"));
    }
    challenge.signer.validate()?;
    validate_scope(&challenge.scope)
}

fn validate_scope(scope: &AuthorityScopeV8) -> Result<(), QualificationError> {
    match scope {
        AuthorityScopeV8::Install {
            activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
            inventory,
            target_host,
        } => {
            validate_host(target_host)?;
            validate_install_inventory(inventory)
        }
        AuthorityScopeV8::InstallV2 {
            activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
            install_plan_sha256,
            inventory,
            state_disposition,
            target_host,
        } => {
            if !digest_shape(install_plan_sha256) {
                return Err(invalid("install-v2 plan digest is malformed"));
            }
            validate_install_state_disposition_v2(state_disposition)?;
            validate_install_target_host_v2(target_host)?;
            validate_install_inventory_v2(inventory)
        }
        AuthorityScopeV8::OneShotRunV2 {
            attempt,
            capability:
                OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
            containment,
            driver_peer,
            target_host,
        } => {
            attempt.validate()?;
            validate_run_target_host_v2(target_host)?;
            validate_driver_peer(driver_peer)?;
            if target_host.machine_id_sha256 != attempt.machine_id_sha256 {
                return Err(invalid("authority target host differs from its attempt"));
            }
            containment.validate(&attempt.sha256()?)
        }
        AuthorityScopeV8::OneShotRun {
            attempt,
            capability:
                OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
            driver_peer: _,
            target_host,
        }
        | AuthorityScopeV8::BreakGlass {
            attempt,
            target_host,
            ..
        } => {
            attempt.validate()?;
            validate_host(target_host)?;
            if let AuthorityScopeV8::OneShotRun { driver_peer, .. } = scope {
                validate_driver_peer(driver_peer)?;
            }
            if target_host.machine_id_sha256 != attempt.machine_id_sha256 {
                return Err(invalid("authority target host differs from its attempt"));
            }
            if let AuthorityScopeV8::BreakGlass {
                attempt,
                capability:
                    BreakGlassCapabilityV8::ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
                recovery_state,
                restore_plan_sha256,
                ..
            } = scope
            {
                validate_recovery_state(recovery_state)?;
                if restore_plan_sha256 != &attempt.restore_plan_sha256
                    || recovery_state.runner_snapshot_sha256 != attempt.runner_snapshot_sha256
                {
                    return Err(invalid(
                        "break-glass authority differs from the exact current recovery state",
                    ));
                }
            }
            Ok(())
        }
    }
}

fn validate_host(host: &TargetHostBindingV8) -> Result<(), QualificationError> {
    if !digest_shape(&host.machine_id_sha256) {
        return Err(invalid("target host machine-id digest is malformed"));
    }
    Ok(())
}

fn validate_driver_peer(peer: &DriverPeerBindingV8) -> Result<(), QualificationError> {
    if peer.uid == 0 || peer.gid == 0 || !digest_shape(&peer.executable_sha256) {
        return Err(invalid(
            "one-shot driver peer must bind one exact non-root executable identity",
        ));
    }
    Ok(())
}

fn validate_recovery_state(state: &RecoveryStateBindingV8) -> Result<(), QualificationError> {
    let compact_boot_id = state.boot_id.replace('-', "");
    if state.boot_epoch == 0
        || state.boot_id.len() != 36
        || state.boot_id.as_bytes().get(8) != Some(&b'-')
        || state.boot_id.as_bytes().get(13) != Some(&b'-')
        || state.boot_id.as_bytes().get(18) != Some(&b'-')
        || state.boot_id.as_bytes().get(23) != Some(&b'-')
        || compact_boot_id.len() != 32
        || !compact_boot_id
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        || !digest_shape(&state.journal_tip_sha256)
        || !digest_shape(&state.restore_state_sha256)
        || !digest_shape(&state.runner_snapshot_sha256)
    {
        return Err(invalid("break-glass recovery state binding is malformed"));
    }
    Ok(())
}

fn validate_install_inventory(
    inventory: &ExactRootInstallInventoryV8,
) -> Result<(), QualificationError> {
    validate_root_file(
        &inventory.admissiond_binary,
        ADMISSIOND_INSTALL_PATH_V8,
        0o555,
    )?;
    validate_root_file(&inventory.recovery_binary, RECOVERY_INSTALL_PATH_V8, 0o555)?;
    validate_root_file(&inventory.admissiond_unit, ADMISSIOND_UNIT_PATH_V8, 0o444)?;
    validate_root_file(&inventory.recovery_unit, RECOVERY_UNIT_PATH_V8, 0o444)?;
    let state = &inventory.state_root;
    if state.path != STATE_ROOT_PATH_V8
        || state.uid != 0
        || state.gid != 0
        || state.mode != 0o700
        || !digest_shape(&state.layout_manifest_sha256)
    {
        return Err(invalid("root state identity is not the frozen v8 identity"));
    }
    Ok(())
}

fn validate_install_target_host_v2(
    host: &InstallTargetHostBindingV2,
) -> Result<(), QualificationError> {
    if !digest_shape(&host.machine_id_sha256) || !canonical_boot_id_v2(&host.boot_id) {
        return Err(invalid("install-v2 target host binding is malformed"));
    }
    Ok(())
}

fn validate_install_state_disposition_v2(
    disposition: &InstallStateDispositionV2,
) -> Result<(), QualificationError> {
    match disposition {
        InstallStateDispositionV2::FreshEmpty => Ok(()),
        InstallStateDispositionV2::ExactUpgrade {
            predecessor_install_epoch_sha256,
            predecessor_provider_tip_sha256,
            predecessor_state_sha256,
        } if digest_shape(predecessor_install_epoch_sha256)
            && digest_shape(predecessor_provider_tip_sha256)
            && digest_shape(predecessor_state_sha256) =>
        {
            Ok(())
        }
        InstallStateDispositionV2::ExactUpgrade { .. } => Err(invalid(
            "install-v2 exact-upgrade predecessor binding is malformed",
        )),
    }
}

fn validate_run_target_host_v2(host: &RunTargetHostBindingV2) -> Result<(), QualificationError> {
    if !digest_shape(&host.machine_id_sha256)
        || !canonical_boot_id_v2(&host.boot_id)
        || host.pid_namespace_inode == 0
        || host.mount_namespace_inode == 0
        || host.cgroup_namespace_inode == 0
        || host.systemd_manager_pid != 1
        || host.systemd_manager_start_time_ticks == 0
        || host.systemd_unit_name != ADMISSIOND_UNIT_NAME_V2
        || !digest_shape(&host.systemd_unit_fragment_sha256)
    {
        return Err(invalid(
            "one-shot-v2 target boot, namespace, or systemd binding is malformed",
        ));
    }
    Ok(())
}

fn validate_install_inventory_v2(
    inventory: &ExactRootInstallInventoryV2,
) -> Result<(), QualificationError> {
    for file in [
        &inventory.ctl_binary,
        &inventory.admissiond_binary,
        &inventory.recovery_binary,
    ] {
        validate_root_file_v2(file, 0o555, false)?;
    }
    for file in [&inventory.admissiond_unit, &inventory.recovery_unit] {
        validate_root_file_v2(file, 0o444, false)?;
    }
    validate_root_file_v2(&inventory.state_lock, 0o600, true)?;
    if inventory.state_lock.size_bytes != 0 || inventory.state_lock.content_sha256 != sha256(b"") {
        return Err(invalid("install-v2 state lock is not the exact empty file"));
    }
    validate_root_directory_v2(&inventory.binary_directory, 0o755)?;
    for directory in [
        &inventory.attempts_directory,
        &inventory.install_epoch_directory,
        &inventory.journal_directory,
        &inventory.nonce_claims_directory,
        &inventory.quarantine_directory,
    ] {
        validate_root_directory_v2(directory, 0o700)?;
    }
    if inventory.state_root.uid != 0
        || inventory.state_root.gid != 0
        || inventory.state_root.mode != 0o700
        || inventory.state_root.path != STATE_ROOT_PATH_V8
        || !digest_shape(&inventory.state_root.layout_manifest_sha256)
    {
        return Err(invalid("install-v2 state-root identity is not exact"));
    }

    if inventory.ctl_binary.path != CTL_INSTALL_PATH_V2
        || inventory.admissiond_binary.path != ADMISSIOND_INSTALL_PATH_V2
        || inventory.recovery_binary.path != RECOVERY_INSTALL_PATH_V2
        || inventory.admissiond_unit.path != ADMISSIOND_UNIT_PATH_V8
        || inventory.recovery_unit.path != RECOVERY_UNIT_PATH_V8
        || inventory.binary_directory.path != INSTALL_BINARY_DIRECTORY_PATH_V2
        || inventory.attempts_directory.path != ATTEMPTS_DIRECTORY_PATH_V2
        || inventory.install_epoch_directory.path != INSTALL_EPOCH_DIRECTORY_PATH_V2
        || inventory.journal_directory.path != JOURNAL_DIRECTORY_PATH_V2
        || inventory.nonce_claims_directory.path != NONCE_CLAIMS_DIRECTORY_PATH_V2
        || inventory.quarantine_directory.path != QUARANTINE_DIRECTORY_PATH_V2
        || inventory.state_lock.path != STATE_LOCK_PATH_V2
    {
        return Err(invalid(
            "install-v2 inventory differs from the fixed production path roster",
        ));
    }

    let binary_prefix = format!("{}/", inventory.binary_directory.path);
    if ![
        &inventory.ctl_binary,
        &inventory.admissiond_binary,
        &inventory.recovery_binary,
    ]
    .iter()
    .all(|file| {
        file.path.starts_with(&binary_prefix) && direct_child_v2(&file.path, &binary_prefix)
    }) {
        return Err(invalid(
            "install-v2 binaries are not direct children of their exact directory",
        ));
    }
    let state_prefix = format!("{}/", inventory.state_root.path);
    if ![
        &inventory.attempts_directory,
        &inventory.install_epoch_directory,
        &inventory.journal_directory,
        &inventory.nonce_claims_directory,
        &inventory.quarantine_directory,
    ]
    .iter()
    .all(|directory| {
        directory.path.starts_with(&state_prefix) && direct_child_v2(&directory.path, &state_prefix)
    }) || !inventory.state_lock.path.starts_with(&state_prefix)
        || !direct_child_v2(&inventory.state_lock.path, &state_prefix)
    {
        return Err(invalid(
            "install-v2 state layout is not one closed direct-child roster",
        ));
    }

    let paths = [
        inventory.ctl_binary.path.as_str(),
        inventory.admissiond_binary.path.as_str(),
        inventory.recovery_binary.path.as_str(),
        inventory.admissiond_unit.path.as_str(),
        inventory.recovery_unit.path.as_str(),
        inventory.binary_directory.path.as_str(),
        inventory.state_root.path.as_str(),
        inventory.attempts_directory.path.as_str(),
        inventory.install_epoch_directory.path.as_str(),
        inventory.journal_directory.path.as_str(),
        inventory.nonce_claims_directory.path.as_str(),
        inventory.quarantine_directory.path.as_str(),
        inventory.state_lock.path.as_str(),
    ];
    if paths.iter().copied().collect::<BTreeSet<_>>().len() != paths.len() {
        return Err(invalid("install-v2 inventory contains duplicate paths"));
    }
    Ok(())
}

fn validate_root_file_v2(
    file: &RootFileInstallIdentityV8,
    mode: u32,
    allow_empty: bool,
) -> Result<(), QualificationError> {
    if file.uid != 0
        || file.gid != 0
        || file.mode != mode
        || (!allow_empty && file.size_bytes == 0)
        || !canonical_absolute_path_v2(&file.path)
        || !digest_shape(&file.content_sha256)
    {
        return Err(invalid("install-v2 root file identity is not exact"));
    }
    Ok(())
}

fn validate_root_directory_v2(
    directory: &RootDirectoryInstallIdentityV2,
    mode: u32,
) -> Result<(), QualificationError> {
    if directory.uid != 0
        || directory.gid != 0
        || directory.mode != mode
        || !canonical_absolute_path_v2(&directory.path)
    {
        return Err(invalid("install-v2 root directory identity is not exact"));
    }
    Ok(())
}

fn canonical_absolute_path_v2(path: &str) -> bool {
    path.starts_with('/')
        && path != "/"
        && !path.ends_with('/')
        && !path
            .split('/')
            .skip(1)
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
}

fn direct_child_v2(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix)
        .is_some_and(|leaf| !leaf.is_empty() && !leaf.contains('/'))
}

fn canonical_boot_id_v2(value: &str) -> bool {
    value.len() == 36
        && value.as_bytes().iter().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                *byte == b'-'
            } else {
                byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
            }
        })
        && value.bytes().any(|byte| !matches!(byte, b'0' | b'-'))
}

fn validate_root_file(
    file: &RootFileInstallIdentityV8,
    expected_path: &str,
    expected_mode: u32,
) -> Result<(), QualificationError> {
    if file.path != expected_path
        || file.uid != 0
        || file.gid != 0
        || file.mode != expected_mode
        || file.size_bytes == 0
        || !digest_shape(&file.content_sha256)
    {
        return Err(invalid(format!(
            "root install identity for {expected_path} is not exact"
        )));
    }
    Ok(())
}

fn append_scope(
    statement: &mut Vec<u8>,
    scope: &AuthorityScopeV8,
) -> Result<(), QualificationError> {
    match scope {
        AuthorityScopeV8::Install {
            activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
            inventory,
            target_host,
        } => {
            append_field(statement, "scope", b"install");
            append_field(
                statement,
                "machine_id_sha256",
                target_host.machine_id_sha256.as_bytes(),
            );
            append_field(
                statement,
                "capability",
                b"install_exact_frozen_root_inventory_only",
            );
            append_field(
                statement,
                "activation",
                b"install_files_only_no_daemon_reload_enable_or_start",
            );
            append_root_file(statement, "admissiond_binary", &inventory.admissiond_binary);
            append_root_file(statement, "recovery_binary", &inventory.recovery_binary);
            append_root_file(statement, "admissiond_unit", &inventory.admissiond_unit);
            append_root_file(statement, "recovery_unit", &inventory.recovery_unit);
            append_field(
                statement,
                "state_root.path",
                inventory.state_root.path.as_bytes(),
            );
            append_field(
                statement,
                "state_root.layout_manifest_sha256",
                inventory.state_root.layout_manifest_sha256.as_bytes(),
            );
            append_u64(
                statement,
                "state_root.uid",
                u64::from(inventory.state_root.uid),
            );
            append_u64(
                statement,
                "state_root.gid",
                u64::from(inventory.state_root.gid),
            );
            append_u64(
                statement,
                "state_root.mode",
                u64::from(inventory.state_root.mode),
            );
        }
        AuthorityScopeV8::InstallV2 {
            activation: InstallActivationV8::InstallFilesOnlyNoDaemonReloadEnableOrStart,
            install_plan_sha256,
            inventory,
            state_disposition,
            target_host,
        } => {
            append_field(statement, "scope", b"install_v2");
            append_field(
                statement,
                "machine_id_sha256",
                target_host.machine_id_sha256.as_bytes(),
            );
            append_field(statement, "boot_id", target_host.boot_id.as_bytes());
            append_field(
                statement,
                "install_plan_sha256",
                install_plan_sha256.as_bytes(),
            );
            match state_disposition {
                InstallStateDispositionV2::FreshEmpty => {
                    append_field(statement, "state_disposition", b"fresh_empty");
                }
                InstallStateDispositionV2::ExactUpgrade {
                    predecessor_install_epoch_sha256,
                    predecessor_provider_tip_sha256,
                    predecessor_state_sha256,
                } => {
                    append_field(statement, "state_disposition", b"exact_upgrade");
                    append_field(
                        statement,
                        "predecessor_install_epoch_sha256",
                        predecessor_install_epoch_sha256.as_bytes(),
                    );
                    append_field(
                        statement,
                        "predecessor_provider_tip_sha256",
                        predecessor_provider_tip_sha256.as_bytes(),
                    );
                    append_field(
                        statement,
                        "predecessor_state_sha256",
                        predecessor_state_sha256.as_bytes(),
                    );
                }
            }
            append_field(
                statement,
                "capability",
                b"install_exact_frozen_root_inventory_v2_only",
            );
            append_field(
                statement,
                "activation",
                b"install_files_only_no_daemon_reload_enable_or_start",
            );
            append_root_file(statement, "ctl_binary", &inventory.ctl_binary);
            append_root_file(statement, "admissiond_binary", &inventory.admissiond_binary);
            append_root_file(statement, "recovery_binary", &inventory.recovery_binary);
            append_root_file(statement, "admissiond_unit", &inventory.admissiond_unit);
            append_root_file(statement, "recovery_unit", &inventory.recovery_unit);
            append_root_directory(statement, "binary_directory", &inventory.binary_directory);
            append_field(
                statement,
                "state_root.path",
                inventory.state_root.path.as_bytes(),
            );
            append_field(
                statement,
                "state_root.layout_manifest_sha256",
                inventory.state_root.layout_manifest_sha256.as_bytes(),
            );
            append_u64(
                statement,
                "state_root.uid",
                u64::from(inventory.state_root.uid),
            );
            append_u64(
                statement,
                "state_root.gid",
                u64::from(inventory.state_root.gid),
            );
            append_u64(
                statement,
                "state_root.mode",
                u64::from(inventory.state_root.mode),
            );
            for (name, directory) in [
                ("attempts_directory", &inventory.attempts_directory),
                (
                    "install_epoch_directory",
                    &inventory.install_epoch_directory,
                ),
                ("journal_directory", &inventory.journal_directory),
                ("nonce_claims_directory", &inventory.nonce_claims_directory),
                ("quarantine_directory", &inventory.quarantine_directory),
            ] {
                append_root_directory(statement, name, directory);
            }
            append_root_file(statement, "state_lock", &inventory.state_lock);
        }
        AuthorityScopeV8::OneShotRun {
            attempt,
            capability:
                OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
            driver_peer,
            target_host,
        } => {
            append_field(statement, "scope", b"one_shot_run");
            append_field(
                statement,
                "machine_id_sha256",
                target_host.machine_id_sha256.as_bytes(),
            );
            append_field(statement, "attempt_identity", &attempt.canonical_bytes()?);
            append_field(
                statement,
                "attempt_identity_sha256",
                attempt.sha256()?.as_bytes(),
            );
            append_field(
                statement,
                "capability",
                b"runner_22_23_shared_process_group_sigstop_then_sigcont_only",
            );
            append_field(statement, "allowed_signals", b"SIGSTOP,SIGCONT");
            append_u64(statement, "driver_peer.uid", u64::from(driver_peer.uid));
            append_u64(statement, "driver_peer.gid", u64::from(driver_peer.gid));
            append_field(
                statement,
                "driver_peer.executable_sha256",
                driver_peer.executable_sha256.as_bytes(),
            );
            append_field(
                statement,
                "forbidden_operations",
                b"SIGKILL,unregister,delete,reconfigure,ref_mutation,production_mutation",
            );
        }
        AuthorityScopeV8::OneShotRunV2 {
            attempt,
            capability:
                OneShotRunCapabilityV8::Runner22And23SharedProcessGroupSigstopThenSigcontOnly,
            containment,
            driver_peer,
            target_host,
        } => {
            append_field(statement, "scope", b"one_shot_run_v2");
            append_field(
                statement,
                "machine_id_sha256",
                target_host.machine_id_sha256.as_bytes(),
            );
            append_field(statement, "boot_id", target_host.boot_id.as_bytes());
            append_u64(
                statement,
                "pid_namespace_inode",
                target_host.pid_namespace_inode,
            );
            append_u64(
                statement,
                "mount_namespace_inode",
                target_host.mount_namespace_inode,
            );
            append_u64(
                statement,
                "cgroup_namespace_inode",
                target_host.cgroup_namespace_inode,
            );
            append_u64(
                statement,
                "systemd_manager_pid",
                u64::from(target_host.systemd_manager_pid),
            );
            append_u64(
                statement,
                "systemd_manager_start_time_ticks",
                target_host.systemd_manager_start_time_ticks,
            );
            append_field(
                statement,
                "systemd_unit_name",
                target_host.systemd_unit_name.as_bytes(),
            );
            append_field(
                statement,
                "systemd_unit_fragment_sha256",
                target_host.systemd_unit_fragment_sha256.as_bytes(),
            );
            append_field(statement, "attempt_identity", &attempt.canonical_bytes()?);
            append_field(
                statement,
                "attempt_identity_sha256",
                attempt.sha256()?.as_bytes(),
            );
            append_field(
                statement,
                "capability",
                b"runner_22_23_shared_process_group_sigstop_then_sigcont_only",
            );
            append_field(
                statement,
                "containment_profile_sha256",
                containment.sha256()?.as_bytes(),
            );
            append_field(
                statement,
                "containment.service_parent_absolute_path",
                containment.service_parent_absolute_path.as_bytes(),
            );
            append_field(
                statement,
                "containment.child_relative_name",
                containment.child_relative_name.as_bytes(),
            );
            append_field(
                statement,
                "containment.child_absolute_path",
                containment.child_absolute_path.as_bytes(),
            );
            append_field(statement, "containment.child_delegated", b"false");
            append_u64(statement, "driver_peer.uid", u64::from(driver_peer.uid));
            append_u64(statement, "driver_peer.gid", u64::from(driver_peer.gid));
            append_field(
                statement,
                "driver_peer.executable_sha256",
                driver_peer.executable_sha256.as_bytes(),
            );
            append_field(
                statement,
                "forbidden_operations",
                b"SIGKILL,unregister,delete,reconfigure,ref_mutation,production_mutation,child_delegation",
            );
        }
        AuthorityScopeV8::BreakGlass {
            attempt,
            capability: BreakGlassCapabilityV8::ExactRestorePlanThenAbandonKeepQuarantineAndBarrier,
            recovery_state,
            restore_plan_sha256,
            target_host,
        } => {
            append_field(statement, "scope", b"break_glass");
            append_field(
                statement,
                "machine_id_sha256",
                target_host.machine_id_sha256.as_bytes(),
            );
            append_field(statement, "attempt_identity", &attempt.canonical_bytes()?);
            append_field(
                statement,
                "attempt_identity_sha256",
                attempt.sha256()?.as_bytes(),
            );
            append_field(
                statement,
                "restore_plan_sha256",
                restore_plan_sha256.as_bytes(),
            );
            append_u64(statement, "current_boot_epoch", recovery_state.boot_epoch);
            append_field(
                statement,
                "current_boot_id",
                recovery_state.boot_id.as_bytes(),
            );
            append_field(
                statement,
                "current_journal_tip_sha256",
                recovery_state.journal_tip_sha256.as_bytes(),
            );
            append_field(
                statement,
                "current_runner_snapshot_sha256",
                recovery_state.runner_snapshot_sha256.as_bytes(),
            );
            append_field(
                statement,
                "current_restore_state_sha256",
                recovery_state.restore_state_sha256.as_bytes(),
            );
            append_field(statement, "capability", b"restore_exact_plan_only");
            append_field(statement, "qualification_outcome", b"abandoned");
            append_field(statement, "quarantine", b"permanent");
            append_field(statement, "barrier_release", b"forbidden");
        }
    }
    Ok(())
}

fn append_root_file(statement: &mut Vec<u8>, name: &str, file: &RootFileInstallIdentityV8) {
    append_field(statement, &format!("{name}.path"), file.path.as_bytes());
    append_field(
        statement,
        &format!("{name}.content_sha256"),
        file.content_sha256.as_bytes(),
    );
    append_u64(statement, &format!("{name}.size_bytes"), file.size_bytes);
    append_u64(statement, &format!("{name}.uid"), u64::from(file.uid));
    append_u64(statement, &format!("{name}.gid"), u64::from(file.gid));
    append_u64(statement, &format!("{name}.mode"), u64::from(file.mode));
}

fn append_root_directory(
    statement: &mut Vec<u8>,
    name: &str,
    directory: &RootDirectoryInstallIdentityV2,
) {
    append_field(
        statement,
        &format!("{name}.path"),
        directory.path.as_bytes(),
    );
    append_u64(statement, &format!("{name}.uid"), u64::from(directory.uid));
    append_u64(statement, &format!("{name}.gid"), u64::from(directory.gid));
    append_u64(
        statement,
        &format!("{name}.mode"),
        u64::from(directory.mode),
    );
}

fn append_u64(statement: &mut Vec<u8>, name: &str, value: u64) {
    append_field(statement, name, &value.to_be_bytes());
}

fn append_field(statement: &mut Vec<u8>, name: &str, value: &[u8]) {
    let name_len = name.len() as u64;
    let value_len = value.len() as u64;
    statement.extend_from_slice(&name_len.to_be_bytes());
    statement.extend_from_slice(name.as_bytes());
    statement.extend_from_slice(&value_len.to_be_bytes());
    statement.extend_from_slice(value);
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && !value.bytes().all(|byte| byte == b'0')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
#[path = "authority_tests.rs"]
mod tests;
