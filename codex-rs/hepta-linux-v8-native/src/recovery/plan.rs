use crate::NativeErrorV8;
use crate::invalid;

use crate::validate_boot_id_v8;
use crate::validate_digest;

/// Durable facts reconstructed before admissiond may start.
///
/// These fields are not caller assertions in production: the eventual
/// recovery scanner must build this value from verified, anchored records.
#[derive(Debug, Eq, PartialEq)]
pub struct RecoveryScanFactsV8 {
    pub(crate) attempt_identity_sha256: String,
    pub(crate) current_boot_id: String,
    pub(crate) current_journal_tip_sha256: String,
    pub(crate) daemon_restart_detected: bool,
    pub(crate) existing_attempt_detected: bool,
    pub(crate) prior_boot_detected: bool,
    pub(crate) incoming_residue_detected: bool,
    pub(crate) unfinished_intent_detected: bool,
    pub(crate) journal_corruption_detected: bool,
    pub(crate) runner_restore_required: bool,
    pub(crate) runner_closure_matches_restore_plan: bool,
    pub(crate) runner_snapshot_sha256: String,
    pub(crate) state_root_device: u64,
    pub(crate) state_root_inode: u64,
    pub(crate) state_root_mode: u32,
    pub(crate) state_root_owner_gid: u32,
    pub(crate) state_root_owner_uid: u32,
    pub(crate) restore_plan_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryDispositionV8 {
    /// No existing attempt or recovery side effect exists. Admission may
    /// begin only through a fresh one-shot authority and nonce claim.
    CleanStartOnly,
    /// Recovery may execute the exact frozen restore plan, but the
    /// qualification remains permanently abandoned and quarantined.
    RestoreThenAbandonAndQuarantine,
    /// Native recovery cannot safely infer an action. Hold all barriers and
    /// require a fresh break-glass authority bound to the current state.
    HoldForExactBreakGlass,
    /// No runner restore is needed, but the interrupted qualification is
    /// still permanently abandoned and quarantined.
    AbandonAndQuarantineWithoutRestore,
}

/// Opaque policy result. There is deliberately no "continue qualification"
/// or ordinary barrier-release disposition.
#[derive(Debug)]
pub struct VerifiedRecoveryPlanV8 {
    attempt_identity_sha256: String,
    disposition: RecoveryDispositionV8,
    journal_tip_sha256: String,
    current_boot_id: String,
    restore_plan_sha256: String,
    runner_snapshot_sha256: String,
    state_root_device: u64,
    state_root_inode: u64,
    state_root_mode: u32,
    state_root_owner_gid: u32,
    state_root_owner_uid: u32,
}

impl VerifiedRecoveryPlanV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn disposition(&self) -> RecoveryDispositionV8 {
        self.disposition
    }

    pub fn journal_tip_sha256(&self) -> &str {
        &self.journal_tip_sha256
    }

    pub fn restore_plan_sha256(&self) -> &str {
        &self.restore_plan_sha256
    }

    pub fn current_boot_id(&self) -> &str {
        &self.current_boot_id
    }

    pub fn runner_snapshot_sha256(&self) -> &str {
        &self.runner_snapshot_sha256
    }

    pub fn state_root_device(&self) -> u64 {
        self.state_root_device
    }

    pub fn state_root_inode(&self) -> u64 {
        self.state_root_inode
    }

    pub fn state_root_mode(&self) -> u32 {
        self.state_root_mode
    }

    pub fn state_root_owner_gid(&self) -> u32 {
        self.state_root_owner_gid
    }

    pub fn state_root_owner_uid(&self) -> u32 {
        self.state_root_owner_uid
    }

    pub fn qualification_abandoned(&self) -> bool {
        self.disposition != RecoveryDispositionV8::CleanStartOnly
    }

    pub fn permanent_quarantine_required(&self) -> bool {
        self.disposition != RecoveryDispositionV8::CleanStartOnly
    }

    pub fn barrier_release_allowed(&self) -> bool {
        false
    }
}

pub fn plan_recovery_v8(
    facts: RecoveryScanFactsV8,
) -> Result<VerifiedRecoveryPlanV8, NativeErrorV8> {
    validate_digest("recovery attempt", &facts.attempt_identity_sha256)?;
    validate_digest("recovery journal tip", &facts.current_journal_tip_sha256)?;
    validate_digest("recovery restore plan", &facts.restore_plan_sha256)?;
    validate_digest("recovery runner snapshot", &facts.runner_snapshot_sha256)?;
    if facts.state_root_device == 0
        || facts.state_root_inode == 0
        || facts.state_root_owner_uid != 0
        || facts.state_root_owner_gid != 0
        || facts.state_root_mode != 0o700
    {
        return Err(invalid(
            "recovery state root must be exact root-owned mode 0700",
        ));
    }
    validate_boot_id_v8(&facts.current_boot_id)?;

    let interrupted = facts.existing_attempt_detected
        || facts.daemon_restart_detected
        || facts.prior_boot_detected
        || facts.incoming_residue_detected
        || facts.unfinished_intent_detected;
    let disposition = if facts.journal_corruption_detected
        || (facts.runner_restore_required && !facts.runner_closure_matches_restore_plan)
    {
        RecoveryDispositionV8::HoldForExactBreakGlass
    } else if facts.runner_restore_required {
        if !interrupted {
            return Err(invalid(
                "runner restore cannot be requested without an interrupted attempt",
            ));
        }
        RecoveryDispositionV8::RestoreThenAbandonAndQuarantine
    } else if interrupted {
        RecoveryDispositionV8::AbandonAndQuarantineWithoutRestore
    } else {
        RecoveryDispositionV8::CleanStartOnly
    };

    Ok(VerifiedRecoveryPlanV8 {
        attempt_identity_sha256: facts.attempt_identity_sha256,
        current_boot_id: facts.current_boot_id,
        disposition,
        journal_tip_sha256: facts.current_journal_tip_sha256,
        restore_plan_sha256: facts.restore_plan_sha256,
        runner_snapshot_sha256: facts.runner_snapshot_sha256,
        state_root_device: facts.state_root_device,
        state_root_inode: facts.state_root_inode,
        state_root_mode: facts.state_root_mode,
        state_root_owner_gid: facts.state_root_owner_gid,
        state_root_owner_uid: facts.state_root_owner_uid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn facts() -> RecoveryScanFactsV8 {
        RecoveryScanFactsV8 {
            attempt_identity_sha256: digest('1'),
            current_boot_id: "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            current_journal_tip_sha256: digest('2'),
            daemon_restart_detected: false,
            existing_attempt_detected: false,
            prior_boot_detected: false,
            incoming_residue_detected: false,
            unfinished_intent_detected: false,
            journal_corruption_detected: false,
            runner_restore_required: false,
            runner_closure_matches_restore_plan: true,
            runner_snapshot_sha256: digest('4'),
            state_root_device: 7,
            state_root_inode: 8,
            state_root_mode: 0o700,
            state_root_owner_gid: 0,
            state_root_owner_uid: 0,
            restore_plan_sha256: digest('3'),
        }
    }

    #[test]
    fn clean_scan_can_only_start_fresh() {
        let plan = plan_recovery_v8(facts()).unwrap();
        assert_eq!(plan.disposition(), RecoveryDispositionV8::CleanStartOnly);
        assert!(!plan.qualification_abandoned());
        assert!(!plan.barrier_release_allowed());
        assert_eq!(plan.state_root_device(), 7);
        assert_eq!(plan.state_root_inode(), 8);
        assert_eq!(plan.runner_snapshot_sha256(), digest('4'));
    }

    #[test]
    fn reboot_or_crash_never_continues_qualification() {
        for mutate in [
            |facts: &mut RecoveryScanFactsV8| facts.existing_attempt_detected = true,
            |facts: &mut RecoveryScanFactsV8| facts.daemon_restart_detected = true,
            |facts: &mut RecoveryScanFactsV8| facts.prior_boot_detected = true,
            |facts: &mut RecoveryScanFactsV8| facts.incoming_residue_detected = true,
            |facts: &mut RecoveryScanFactsV8| facts.unfinished_intent_detected = true,
        ] as [fn(&mut RecoveryScanFactsV8); 5]
        {
            let mut input = facts();
            mutate(&mut input);
            let plan = plan_recovery_v8(input).unwrap();
            assert_eq!(
                plan.disposition(),
                RecoveryDispositionV8::AbandonAndQuarantineWithoutRestore
            );
            assert!(plan.qualification_abandoned());
            assert!(plan.permanent_quarantine_required());
            assert!(!plan.barrier_release_allowed());
        }
    }

    #[test]
    fn exact_restore_is_allowed_only_for_interrupted_matching_closure() {
        let mut exact = facts();
        exact.unfinished_intent_detected = true;
        exact.runner_restore_required = true;
        let plan = plan_recovery_v8(exact).unwrap();
        assert_eq!(
            plan.disposition(),
            RecoveryDispositionV8::RestoreThenAbandonAndQuarantine
        );

        let mut unexplained = facts();
        unexplained.runner_restore_required = true;
        assert!(plan_recovery_v8(unexplained).is_err());

        let mut drift = facts();
        drift.prior_boot_detected = true;
        drift.runner_restore_required = true;
        drift.runner_closure_matches_restore_plan = false;
        assert_eq!(
            plan_recovery_v8(drift).unwrap().disposition(),
            RecoveryDispositionV8::HoldForExactBreakGlass
        );
    }

    #[test]
    fn corruption_never_guesses_a_restore() {
        let mut corrupt = facts();
        corrupt.journal_corruption_detected = true;
        assert_eq!(
            plan_recovery_v8(corrupt).unwrap().disposition(),
            RecoveryDispositionV8::HoldForExactBreakGlass
        );
    }
}
