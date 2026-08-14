//! Production-path opener for the first Linux v8 executable milestone.
//!
//! This establishes that the process is root, opens the fixed root without
//! following path components, verifies its exact identity, and acquires a
//! never-replaced process-lifetime singleton lock. Its only bridge beyond
//! preflight is a read-only durable-state classifier. It grants no admission,
//! recovery effect, signal, runner, barrier, or activation right.

#[cfg(all(test, target_os = "linux"))]
use std::path::Path;

use serde::Serialize;

use codex_hepta_linux_qualification_v8::JournalEffectV8;

use crate::FileIdentityV8;
use crate::NativeErrorV8;
use crate::PRODUCTION_STATE_ROOT_V8;
use crate::TrustedStateRootV8;
use crate::invalid;
use crate::observe_boot_id_v8;
#[cfg(target_os = "linux")]
use crate::open_production_trusted_state_root_v8;
use crate::read_active_attempt_for_recovery_v8;
use crate::scan_durable_journal_events_v8;

#[derive(Debug)]
pub struct ProductionRuntimePreflightV8 {
    trusted_state_root: TrustedStateRootV8,
}

impl ProductionRuntimePreflightV8 {
    pub fn root_identity(&self) -> FileIdentityV8 {
        self.trusted_state_root.identity()
    }

    pub fn lock_identity(&self) -> FileIdentityV8 {
        self.trusted_state_root.lock_identity()
    }

    pub fn revalidate(&self) -> Result<(), NativeErrorV8> {
        self.trusted_state_root.revalidate()
    }

    /// Produces a read-only plan from the exact active-attempt binding and
    /// typed journal replay. Every returned disposition remains NO_AUTHORITY;
    /// malformed, unsupported, or changing state returns an error instead of
    /// guessing an activation or recovery effect.
    pub fn classify_activation_recovery(
        &mut self,
    ) -> Result<RuntimeActivationRecoveryPlanV8, NativeErrorV8> {
        self.trusted_state_root.revalidate()?;
        let root = self.trusted_state_root.identity();
        let root_binding_sha256 = self.trusted_state_root.binding_sha256().to_string();
        let machine_id_sha256 = self.trusted_state_root.machine_id_sha256().to_string();
        let current_boot_before = observe_boot_id_v8()?.to_string();
        let active_and_replay = {
            let (state_root, _, state_root_lock) = self.trusted_state_root.split_for_store_v8();
            match read_active_attempt_for_recovery_v8(state_root, state_root_lock)? {
                None => None,
                Some(active) => {
                    if active.machine_id_sha256() != machine_id_sha256 {
                        return Err(invalid(
                            "active attempt machine binding differs from the trusted state root",
                        ));
                    }
                    let replay = scan_durable_journal_events_v8(
                        state_root,
                        state_root_lock,
                        active.attempt_identity_sha256(),
                    )?;
                    Some((active, replay))
                }
            }
        };
        self.trusted_state_root.revalidate()?;
        let current_boot_after = observe_boot_id_v8()?.to_string();
        if current_boot_before != current_boot_after {
            return Err(invalid(
                "kernel boot identity changed during activation/recovery classification",
            ));
        }

        let mut plan = RuntimeActivationRecoveryPlanV8::new_read_only_v8(
            RuntimeActivationRecoveryDispositionV8::NoActiveAttemptHoldForFreshAuthority,
            root_binding_sha256,
            root.device(),
            root.inode(),
            machine_id_sha256,
            current_boot_after.clone(),
        );
        if let Some((active, replay)) = active_and_replay {
            if replay.current_boot_id() != current_boot_after {
                return Err(invalid(
                    "typed journal replay and bridge observe different kernel boots",
                ));
            }
            let pending_effect = replay.pending_effect().map(|pending| pending.effect());
            plan.disposition = classify_replayed_activation_recovery_v8(
                replay.journal().incoming_residue_detected(),
                replay.qualification_abandoned(),
                replay.boot_recovery_detected()
                    || replay.current_boot_mismatch_detected()
                    || active.boot_id() != current_boot_after,
                pending_effect,
            )?;
            plan.attempt_identity_sha256 = Some(active.attempt_identity_sha256().to_string());
            plan.active_attempt_record_sha256 = Some(active.record_sha256().to_string());
            plan.active_boot_id = Some(active.boot_id().to_string());
            plan.restore_plan_sha256 = Some(active.restore_plan_sha256().to_string());
            plan.barrier_generation = Some(active.barrier_generation());
            plan.journal_tip_sha256 = Some(replay.journal().tip_sha256().to_string());
            plan.pending_effect = pending_effect
                .map(journal_effect_name_v8)
                .map(str::to_string);
            plan.qualification_abandoned = replay.qualification_abandoned();
        }
        self.trusted_state_root.revalidate()?;
        Ok(plan)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeActivationRecoveryDispositionV8 {
    NoActiveAttemptHoldForFreshAuthority,
    ActiveCurrentBootHoldNoAuthority,
    PendingRunnerStopHoldForRecovery,
    PendingRunnerRestoreHoldForRecovery,
    PriorBootAbandonAndQuarantine,
    InterruptedPublicationHoldForExactRecovery,
    TerminalAbandonedQuarantine,
}

/// Serializable read-only bridge result. Boolean authority fields are
/// deliberately frozen false; this type is evidence for a later authority
/// boundary and cannot be consumed as an effect token.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeActivationRecoveryPlanV8 {
    pub schema: String,
    pub disposition: RuntimeActivationRecoveryDispositionV8,
    pub state_root_binding_sha256: String,
    pub state_root_device: u64,
    pub state_root_inode: u64,
    pub machine_id_sha256: String,
    pub current_boot_id: String,
    pub attempt_identity_sha256: Option<String>,
    pub active_attempt_record_sha256: Option<String>,
    pub active_boot_id: Option<String>,
    pub restore_plan_sha256: Option<String>,
    pub barrier_generation: Option<u64>,
    pub journal_tip_sha256: Option<String>,
    pub pending_effect: Option<String>,
    pub qualification_abandoned: bool,
    pub activation_allowed: bool,
    pub recovery_effect_allowed: bool,
    pub barrier_release_allowed: bool,
    pub authority: String,
}

impl RuntimeActivationRecoveryPlanV8 {
    fn new_read_only_v8(
        disposition: RuntimeActivationRecoveryDispositionV8,
        state_root_binding_sha256: String,
        state_root_device: u64,
        state_root_inode: u64,
        machine_id_sha256: String,
        current_boot_id: String,
    ) -> Self {
        Self {
            schema: "hepta-linux-v8-activation-recovery-plan-v1".to_string(),
            disposition,
            state_root_binding_sha256,
            state_root_device,
            state_root_inode,
            machine_id_sha256,
            current_boot_id,
            attempt_identity_sha256: None,
            active_attempt_record_sha256: None,
            active_boot_id: None,
            restore_plan_sha256: None,
            barrier_generation: None,
            journal_tip_sha256: None,
            pending_effect: None,
            qualification_abandoned: false,
            activation_allowed: false,
            recovery_effect_allowed: false,
            barrier_release_allowed: false,
            authority: "read-only-classification-no-authority".to_string(),
        }
    }
}

fn classify_replayed_activation_recovery_v8(
    incoming_residue_detected: bool,
    qualification_abandoned: bool,
    prior_boot_detected: bool,
    pending_effect: Option<JournalEffectV8>,
) -> Result<RuntimeActivationRecoveryDispositionV8, NativeErrorV8> {
    if qualification_abandoned {
        return Ok(RuntimeActivationRecoveryDispositionV8::TerminalAbandonedQuarantine);
    }
    if incoming_residue_detected {
        return Ok(
            RuntimeActivationRecoveryDispositionV8::InterruptedPublicationHoldForExactRecovery,
        );
    }
    if prior_boot_detected {
        return Ok(RuntimeActivationRecoveryDispositionV8::PriorBootAbandonAndQuarantine);
    }
    match pending_effect {
        None => Ok(RuntimeActivationRecoveryDispositionV8::ActiveCurrentBootHoldNoAuthority),
        Some(JournalEffectV8::RunnerStop) => {
            Ok(RuntimeActivationRecoveryDispositionV8::PendingRunnerStopHoldForRecovery)
        }
        Some(JournalEffectV8::RunnerRestore) => {
            Ok(RuntimeActivationRecoveryDispositionV8::PendingRunnerRestoreHoldForRecovery)
        }
        Some(JournalEffectV8::CandidateExecution) => Err(invalid(
            "typed candidate-execution evidence remains no-authority until descriptor-bound recovery assessment is frozen",
        )),
        Some(
            JournalEffectV8::CandidateRelay
            | JournalEffectV8::PostRestoreSnapshot
            | JournalEffectV8::BarrierRelease,
        ) => Err(invalid(
            "activation/recovery bridge encountered an effect without a frozen V2 semantic schema and backend",
        )),
    }
}

fn journal_effect_name_v8(effect: JournalEffectV8) -> &'static str {
    match effect {
        JournalEffectV8::RunnerStop => "runner_stop",
        JournalEffectV8::CandidateExecution => "candidate_execution",
        JournalEffectV8::CandidateRelay => "candidate_relay",
        JournalEffectV8::RunnerRestore => "runner_restore",
        JournalEffectV8::PostRestoreSnapshot => "post_restore_snapshot",
        JournalEffectV8::BarrierRelease => "barrier_release",
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimePreflightReportV8 {
    pub schema: String,
    pub component: String,
    pub state_root: String,
    pub state_root_device: u64,
    pub state_root_inode: u64,
    pub singleton_lock_device: u64,
    pub singleton_lock_inode: u64,
    pub admissiond_cgroup_root: String,
    pub authority: String,
}

pub fn open_production_runtime_preflight_v8() -> Result<ProductionRuntimePreflightV8, NativeErrorV8>
{
    #[cfg(not(target_os = "linux"))]
    {
        Err(invalid(
            "Linux v8 production runtime preflight requires Linux",
        ))
    }
    #[cfg(target_os = "linux")]
    {
        // SAFETY: geteuid/getegid have no pointer arguments or preconditions.
        let uid = unsafe { libc::geteuid() };
        // SAFETY: see above.
        let gid = unsafe { libc::getegid() };
        if uid != 0 || gid != 0 {
            return Err(invalid(
                "Linux v8 production runtime preflight requires uid 0 and gid 0",
            ));
        }
        let trusted_state_root = open_production_trusted_state_root_v8()?;
        trusted_state_root.revalidate()?;
        Ok(ProductionRuntimePreflightV8 { trusted_state_root })
    }
}

#[cfg(all(test, target_os = "linux"))]
fn open_runtime_preflight_at_v8(
    path: &Path,
    _expected_uid: u32,
    _expected_gid: u32,
) -> Result<ProductionRuntimePreflightV8, NativeErrorV8> {
    let trusted_state_root = crate::open_test_trusted_state_root_v8(path)?;
    trusted_state_root.revalidate()?;
    Ok(ProductionRuntimePreflightV8 { trusted_state_root })
}

pub fn run_runtime_preflight_v8(
    component: &str,
    arguments: &[String],
) -> Result<String, NativeErrorV8> {
    if component != "admissiond" && component != "recover" {
        return Err(invalid("unknown Linux v8 runtime bridge component"));
    }
    if arguments == ["--classify"] {
        let mut preflight = open_production_runtime_preflight_v8()?;
        let plan = preflight.classify_activation_recovery()?;
        return serde_json::to_string(&plan)
            .map_err(|error| invalid(format!("encode runtime activation/recovery plan: {error}")));
    }
    if arguments != ["--preflight"] {
        return Err(invalid(format!(
            "{component} accepts only read-only --preflight or --classify; run authority is not implemented"
        )));
    }
    let preflight = open_production_runtime_preflight_v8()?;
    preflight.revalidate()?;
    let root = preflight.root_identity();
    let lock = preflight.lock_identity();
    let report = RuntimePreflightReportV8 {
        schema: "hepta-linux-v8-runtime-preflight-v1".to_string(),
        component: component.to_string(),
        state_root: PRODUCTION_STATE_ROOT_V8.to_string(),
        state_root_device: root.device(),
        state_root_inode: root.inode(),
        singleton_lock_device: lock.device(),
        singleton_lock_inode: lock.inode(),
        admissiond_cgroup_root: crate::ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8.to_string(),
        authority: "preflight-only-no-run-authority".to_string(),
    };
    serde_json::to_string(&report)
        .map_err(|error| invalid(format!("encode runtime preflight report: {error}")))
}

/// Long-lived, preflight-only owner for the delegated admissiond service
/// cgroup. It exposes no candidate, signal, recovery, or run operation. The
/// unpublished production state-root profile keeps this boundary fail-closed.
pub fn run_admissiond_guardian_v8(arguments: &[String]) -> Result<(), NativeErrorV8> {
    if arguments != ["--guardian"] {
        return Err(invalid(
            "admissiond guardian accepts only the exact --guardian invocation",
        ));
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(invalid("Linux v8 admissiond guardian requires Linux"))
    }
    #[cfg(target_os = "linux")]
    {
        let preflight = open_production_runtime_preflight_v8()?;
        preflight.revalidate()?;
        loop {
            // SAFETY: pause has no pointer arguments. SIGTERM and SIGINT keep
            // their default terminating disposition; a caught signal merely
            // wakes the loop and forces a complete trust revalidation.
            unsafe { libc::pause() };
            preflight.revalidate()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_os = "linux")]
    use crate::STATE_ROOT_LOCK_LEAF_V8;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    #[cfg(target_os = "linux")]
    fn install_test_state_lock(root: &Path) {
        use std::os::unix::fs::OpenOptionsExt as _;
        use std::os::unix::fs::PermissionsExt as _;

        for name in [
            crate::ATTEMPTS_DIRECTORY_V8,
            crate::INSTALL_EPOCH_DIRECTORY_V8,
            crate::JOURNAL_DIRECTORY_V8,
            crate::NONCE_CLAIMS_DIRECTORY_V8,
            crate::QUARANTINE_DIRECTORY_V8,
        ] {
            let directory = root.join(name);
            std::fs::create_dir(&directory).expect("install exact state directory");
            std::fs::set_permissions(directory, std::fs::Permissions::from_mode(0o700))
                .expect("set exact state directory mode");
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(root.join(STATE_ROOT_LOCK_LEAF_V8))
            .expect("install exact state lock");
        file.sync_all().expect("fsync state lock");
    }

    #[test]
    fn any_non_preflight_invocation_is_fail_closed() {
        assert!(run_runtime_preflight_v8("admissiond", &[]).is_err());
        assert!(run_runtime_preflight_v8("recover", &["--run".to_string()]).is_err());
        assert!(run_runtime_preflight_v8("other", &["--preflight".to_string()]).is_err());
    }

    #[test]
    fn read_only_dispositions_never_turn_candidate_or_untyped_effects_into_authority() {
        assert_eq!(
            classify_replayed_activation_recovery_v8(false, false, false, None).unwrap(),
            RuntimeActivationRecoveryDispositionV8::ActiveCurrentBootHoldNoAuthority
        );
        assert_eq!(
            classify_replayed_activation_recovery_v8(
                false,
                false,
                false,
                Some(JournalEffectV8::RunnerStop),
            )
            .unwrap(),
            RuntimeActivationRecoveryDispositionV8::PendingRunnerStopHoldForRecovery
        );
        assert_eq!(
            classify_replayed_activation_recovery_v8(
                false,
                false,
                false,
                Some(JournalEffectV8::RunnerRestore),
            )
            .unwrap(),
            RuntimeActivationRecoveryDispositionV8::PendingRunnerRestoreHoldForRecovery
        );
        for unsupported in [
            JournalEffectV8::CandidateExecution,
            JournalEffectV8::CandidateRelay,
            JournalEffectV8::PostRestoreSnapshot,
            JournalEffectV8::BarrierRelease,
        ] {
            assert!(
                classify_replayed_activation_recovery_v8(false, false, false, Some(unsupported),)
                    .is_err()
            );
        }
        assert_eq!(
            classify_replayed_activation_recovery_v8(
                false,
                false,
                true,
                Some(JournalEffectV8::RunnerRestore),
            )
            .unwrap(),
            RuntimeActivationRecoveryDispositionV8::PriorBootAbandonAndQuarantine
        );
    }

    #[test]
    fn terminal_abandonment_precedes_incoming_and_every_plan_is_no_authority() {
        assert_eq!(
            classify_replayed_activation_recovery_v8(false, true, false, None).unwrap(),
            RuntimeActivationRecoveryDispositionV8::TerminalAbandonedQuarantine,
            "terminal abandonment must be selected without incoming residue"
        );
        assert_eq!(
            classify_replayed_activation_recovery_v8(true, true, true, None).unwrap(),
            RuntimeActivationRecoveryDispositionV8::TerminalAbandonedQuarantine,
            "incoming residue must not mask an already terminal abandonment"
        );
        assert_eq!(
            classify_replayed_activation_recovery_v8(true, false, false, None).unwrap(),
            RuntimeActivationRecoveryDispositionV8::InterruptedPublicationHoldForExactRecovery
        );

        for disposition in [
            RuntimeActivationRecoveryDispositionV8::NoActiveAttemptHoldForFreshAuthority,
            RuntimeActivationRecoveryDispositionV8::ActiveCurrentBootHoldNoAuthority,
            RuntimeActivationRecoveryDispositionV8::PendingRunnerStopHoldForRecovery,
            RuntimeActivationRecoveryDispositionV8::PendingRunnerRestoreHoldForRecovery,
            RuntimeActivationRecoveryDispositionV8::PriorBootAbandonAndQuarantine,
            RuntimeActivationRecoveryDispositionV8::InterruptedPublicationHoldForExactRecovery,
            RuntimeActivationRecoveryDispositionV8::TerminalAbandonedQuarantine,
        ] {
            let plan = RuntimeActivationRecoveryPlanV8::new_read_only_v8(
                disposition,
                digest('1'),
                7,
                8,
                digest('2'),
                "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            );
            assert!(!plan.activation_allowed);
            assert!(!plan.recovery_effect_allowed);
            assert!(!plan.barrier_release_allowed);
            let serialized = serde_json::to_value(&plan).expect("serialize read-only plan");
            assert_eq!(serialized["activation_allowed"], false);
            assert_eq!(serialized["recovery_effect_allowed"], false);
            assert_eq!(serialized["barrier_release_allowed"], false);
            assert_eq!(
                serialized["authority"],
                "read-only-classification-no-authority"
            );
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn absent_root_and_second_writer_fail_closed() {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;

        let temporary = tempfile::tempdir().expect("tempdir");
        let absent = temporary.path().join("absent");
        // SAFETY: geteuid/getegid have no arguments or preconditions.
        let uid = unsafe { libc::geteuid() };
        // SAFETY: see above.
        let gid = unsafe { libc::getegid() };
        assert!(open_runtime_preflight_at_v8(&absent, uid, gid).is_err());

        let root = temporary.path().join("root");
        std::fs::create_dir(&root).expect("create root");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
            .expect("set root mode");
        let metadata = std::fs::metadata(&root).expect("root metadata");
        assert!(open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid()).is_err());
        assert!(!root.join(STATE_ROOT_LOCK_LEAF_V8).exists());
        install_test_state_lock(&root);
        let first = open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid())
            .expect("first writer");
        assert!(open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid()).is_err());
        first.revalidate().expect("first writer remains valid");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn replaced_lock_name_invalidates_live_writer() {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;

        let temporary = tempfile::tempdir().expect("tempdir");
        let root = temporary.path().join("root");
        std::fs::create_dir(&root).expect("create root");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
            .expect("set root mode");
        let metadata = std::fs::metadata(&root).expect("root metadata");
        install_test_state_lock(&root);
        let preflight = open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid())
            .expect("open preflight");
        std::fs::remove_file(root.join(STATE_ROOT_LOCK_LEAF_V8)).expect("unlink lock name");
        std::fs::write(root.join(STATE_ROOT_LOCK_LEAF_V8), b"").expect("replace lock name");
        std::fs::set_permissions(
            root.join(STATE_ROOT_LOCK_LEAF_V8),
            std::fs::Permissions::from_mode(0o600),
        )
        .expect("set replacement mode");
        assert!(preflight.revalidate().is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn machine_id_digest_mismatch_is_exact_fail_closed_and_no_authority() {
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;

        let temporary = tempfile::tempdir().expect("tempdir");
        let root = temporary.path().join("root");
        std::fs::create_dir(&root).expect("create root");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
            .expect("set root mode");
        install_test_state_lock(&root);
        let metadata = std::fs::metadata(&root).expect("root metadata");
        let mut preflight = open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid())
            .expect("open read-only bridge");

        let before_injection = preflight
            .classify_activation_recovery()
            .expect("classify absent active attempt");
        assert_eq!(
            before_injection.disposition,
            RuntimeActivationRecoveryDispositionV8::NoActiveAttemptHoldForFreshAuthority
        );
        assert!(!before_injection.activation_allowed);
        assert!(!before_injection.recovery_effect_allowed);
        assert!(!before_injection.barrier_release_allowed);
        assert_eq!(
            before_injection.authority,
            "read-only-classification-no-authority"
        );

        let trusted_machine_id_sha256 =
            preflight.trusted_state_root.machine_id_sha256().to_string();
        let mismatched_machine_id_sha256 = if trusted_machine_id_sha256 == digest('f') {
            digest('e')
        } else {
            digest('f')
        };
        assert_ne!(mismatched_machine_id_sha256, trusted_machine_id_sha256);
        let request = crate::ActiveAttemptRequestV8::new(
            digest('1'),
            7,
            crate::observe_boot_id_v8().unwrap().to_string(),
            mismatched_machine_id_sha256,
            digest('2'),
        )
        .unwrap();
        let (state_root, _, state_root_lock) = preflight.trusted_state_root.split_for_store_v8();
        let active = match crate::publish_active_attempt_durably_v8(
            state_root,
            state_root_lock,
            &request,
            &digest('3'),
        )
        .unwrap()
        {
            crate::ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            crate::ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        drop(active);

        let error = preflight.classify_activation_recovery().unwrap_err();
        match error {
            NativeErrorV8::Invalid(message) => assert_eq!(
                message,
                "active attempt machine binding differs from the trusted state root"
            ),
            other => panic!("unexpected machine-id mismatch error: {other}"),
        }
        preflight
            .revalidate()
            .expect("machine-id mismatch emits no authority and preserves descriptor binding");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn classifier_binds_absence_then_existing_journal_without_granting_authority() {
        use codex_hepta_linux_qualification_v8::JournalEventV8;
        use std::os::unix::fs::MetadataExt as _;
        use std::os::unix::fs::PermissionsExt as _;

        const ZERO_SHA256: &str =
            "0000000000000000000000000000000000000000000000000000000000000000";

        let temporary = tempfile::tempdir().expect("tempdir");
        let root = temporary.path().join("root");
        std::fs::create_dir(&root).expect("create root");
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
            .expect("set root mode");
        install_test_state_lock(&root);
        let attempt = digest('1');
        std::fs::create_dir_all(
            root.join(crate::ATTEMPTS_DIRECTORY_V8)
                .join(&attempt)
                .join("journal"),
        )
        .expect("create exact attempt journal");
        let metadata = std::fs::metadata(&root).expect("root metadata");
        let mut preflight = open_runtime_preflight_at_v8(&root, metadata.uid(), metadata.gid())
            .expect("open read-only bridge");

        let absent = preflight
            .classify_activation_recovery()
            .expect("classify absent active attempt");
        assert_eq!(
            absent.disposition,
            RuntimeActivationRecoveryDispositionV8::NoActiveAttemptHoldForFreshAuthority
        );
        assert!(absent.attempt_identity_sha256.is_none());
        assert!(!absent.activation_allowed);
        assert!(!absent.recovery_effect_allowed);
        assert!(!absent.barrier_release_allowed);

        let boot_id = crate::observe_boot_id_v8().unwrap().to_string();
        let machine_id_sha256 = preflight.trusted_state_root.machine_id_sha256().to_string();
        let request = crate::ActiveAttemptRequestV8::new(
            attempt.clone(),
            7,
            boot_id.clone(),
            machine_id_sha256,
            digest('2'),
        )
        .unwrap();
        let (state_root, _, state_root_lock) = preflight.trusted_state_root.split_for_store_v8();
        let active = match crate::publish_active_attempt_durably_v8(
            state_root,
            state_root_lock,
            &request,
            &digest('3'),
        )
        .unwrap()
        {
            crate::ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            crate::ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        let opened = crate::DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            boot_id,
            1,
            ZERO_SHA256.to_string(),
            crate::encode_durable_journal_event_v8(&JournalEventV8::AttemptOpened {
                authority_manifest_sha256: digest('4'),
            })
            .unwrap(),
        )
        .unwrap();
        let opened_record_sha256 = opened.record_sha256().unwrap();
        crate::append_journal_record_durably_v8(
            state_root,
            state_root_lock,
            &active,
            &opened,
            &digest('5'),
        )
        .unwrap();
        drop(active);

        let existing = preflight
            .classify_activation_recovery()
            .expect("classify exact existing journal");
        assert_eq!(
            existing.disposition,
            RuntimeActivationRecoveryDispositionV8::ActiveCurrentBootHoldNoAuthority
        );
        assert_eq!(
            existing.attempt_identity_sha256.as_deref(),
            Some(attempt.as_str())
        );
        assert_eq!(
            existing.journal_tip_sha256.as_deref(),
            Some(opened_record_sha256.as_str())
        );
        assert!(!existing.activation_allowed);
        assert!(!existing.recovery_effect_allowed);
        assert!(!existing.barrier_release_allowed);
        preflight
            .revalidate()
            .expect("bridge remains descriptor-bound");
    }
}
