use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::AttemptIdentityV8;
use crate::QualificationError;
use crate::invalid;

pub const AUTHORITY_SCHEMA_V8: &str = "hepta_linux_v8_signed_authority_v1";
pub const INSTALL_NAMESPACE_V8: &str = "hepta-linux-v8-install";
pub const ONE_SHOT_RUN_NAMESPACE_V8: &str = "hepta-linux-v8-execution";
pub const BREAK_GLASS_NAMESPACE_V8: &str = "hepta-linux-v8-break-glass";
pub const MAX_AUTHORITY_LIFETIME_SECONDS_V8: u64 = 15 * 60;

pub const ADMISSIOND_INSTALL_PATH_V8: &str = "/usr/local/libexec/hepta-linux-v8-admissiond";
pub const RECOVERY_INSTALL_PATH_V8: &str = "/usr/local/libexec/hepta-linux-v8-recover";
pub const ADMISSIOND_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-admissiond.service";
pub const RECOVERY_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-recover.service";
pub const STATE_ROOT_PATH_V8: &str = "/var/lib/hepta-linux-v8";

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
    OneShotRun {
        attempt: AttemptIdentityV8,
        capability: OneShotRunCapabilityV8,
        driver_peer: DriverPeerBindingV8,
        target_host: TargetHostBindingV8,
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
            || !self.principal.bytes().all(|byte| byte.is_ascii_graphic())
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
    verified_allowed_signers_sha256: String,
    verified_key_fingerprint: String,
    verified_namespace: String,
    verified_principal: String,
    verified_signature_algorithm: AuthoritySignatureAlgorithmV8,
}

impl CryptographicSignatureObservation {
    pub(crate) fn from_verified_sshsig(
        signature_sha256: String,
        signed_statement_sha256: String,
        allowed_signers_sha256: String,
        key_fingerprint: String,
        namespace: String,
        principal: String,
    ) -> Self {
        Self {
            signature_sha256,
            signed_statement_sha256,
            verified_allowed_signers_sha256: allowed_signers_sha256,
            verified_key_fingerprint: key_fingerprint,
            verified_namespace: namespace,
            verified_principal: principal,
            verified_signature_algorithm: AuthoritySignatureAlgorithmV8::OpenSshSshsigEd25519,
        }
    }

    pub(crate) fn exactly_matches(
        &self,
        signature_sha256: &str,
        statement_sha256: &str,
        namespace: &str,
        signer: &AuthoritySignerBindingV8,
    ) -> bool {
        self.signature_sha256 == signature_sha256
            && self.signed_statement_sha256 == statement_sha256
            && self.verified_allowed_signers_sha256 == signer.allowed_signers_sha256
            && self.verified_key_fingerprint == signer.key_fingerprint
            && self.verified_namespace == namespace
            && self.verified_principal == signer.principal
            && self.verified_signature_algorithm == signer.signature_algorithm
    }

    #[cfg(test)]
    pub(crate) fn for_test_only(
        signature_sha256: String,
        signed_statement_sha256: String,
        allowed_signers_sha256: String,
        key_fingerprint: String,
        namespace: String,
        principal: String,
    ) -> Self {
        Self::from_verified_sshsig(
            signature_sha256,
            signed_statement_sha256,
            allowed_signers_sha256,
            key_fingerprint,
            namespace,
            principal,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityScopeKindV8 {
    Install,
    OneShotRun,
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
            Self::OneShotRun { .. } => AuthorityScopeKindV8::OneShotRun,
            Self::BreakGlass { .. } => AuthorityScopeKindV8::BreakGlass,
        }
    }

    fn namespace(&self) -> &'static str {
        match self {
            Self::Install { .. } => INSTALL_NAMESPACE_V8,
            Self::OneShotRun { .. } => ONE_SHOT_RUN_NAMESPACE_V8,
            Self::BreakGlass { .. } => BREAK_GLASS_NAMESPACE_V8,
        }
    }
}

/// Builds the exact bytes that an external SSHSIG verifier must verify.
pub fn canonical_authority_statement_v8(
    challenge: &AuthorityChallengeV8,
) -> Result<Vec<u8>, QualificationError> {
    validate_challenge(challenge)?;
    let mut statement = b"hepta-linux-v8-authority-statement-v1\0".to_vec();
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
    observation: &CryptographicSignatureObservation,
    now_unix_seconds: u64,
    replay_guard: &mut AuthorityReplayGuardV8,
) -> Result<VerifiedAuthorityV8, QualificationError> {
    let statement = canonical_authority_statement_v8(&signed.challenge)?;
    let statement_sha256 = sha256(&statement);
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
    if !observation.exactly_matches(
        &signature_sha256,
        &statement_sha256,
        &signed.challenge.namespace,
        &signed.challenge.signer,
    ) {
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
    })
}

fn validate_challenge(challenge: &AuthorityChallengeV8) -> Result<(), QualificationError> {
    if challenge.schema != AUTHORITY_SCHEMA_V8 {
        return Err(invalid("authority schema is not Linux v8"));
    }
    if challenge.namespace != challenge.scope.namespace() {
        return Err(invalid(
            "authority namespace does not match its closed scope",
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
