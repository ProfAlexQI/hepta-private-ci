use codex_hepta_mnl_trust_v1::PreparedPreRunReplayClaimV1;
use codex_hepta_mnl_trust_v1::ReplayPlatformScopeV1;

use crate::DurableReplayPublicationInspectionV1;
use crate::ReplayStoreAnchorV1;
use crate::ReplayStoreErrorV1;
use crate::ReplayStoreResultV1;
use crate::boot_id::FixedProcfsBootIdSourceV1;
use crate::clock::ClockSampleV1;
use crate::clock::validate_realtime_window;
use crate::clock::validate_sample_sequence;
use crate::error::invalid;
use crate::publish_pre_run_claim_once;

pub const PRODUCTION_WALL_CLOCK_SUPERVISOR_POLICY_AVAILABLE: bool = false;

#[derive(Debug)]
struct WallClockSupervisorPolicyV1 {
    platform_scope: ReplayPlatformScopeV1,
}

#[derive(Clone, Debug)]
pub(crate) struct PreRunClockClaimBindingV1 {
    boot_id_sha256: String,
    expires_at_unix_seconds: u64,
    full_binding_sha256: String,
    generation_epoch_id: String,
    maximum_lifetime_seconds: u64,
    not_before_unix_seconds: u64,
    platform_scope: ReplayPlatformScopeV1,
    profile_id: String,
    replay_slot_sha256: String,
    run_identity_sha256: String,
    store_identity_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClockBootObservationV1 {
    pub(crate) boot_id_sha256: String,
    pub(crate) sample: ClockSampleV1,
}

pub(crate) trait ClockBootSourceV1 {
    fn observe(&mut self) -> ReplayStoreResultV1<ClockBootObservationV1>;
    fn revalidate(&self) -> ReplayStoreResultV1<()>;
}

#[derive(Debug)]
pub(crate) struct LinuxClockBootSourceV1 {
    boot: FixedProcfsBootIdSourceV1,
}

/// Non-authorizing observation of one indivisible clock-before, durable
/// replay publication, clock-after sequence.
///
/// This value is intentionally not a delayed launch grant. N5 must perform
/// its own post-claim time check and immediate spawn inside one state machine.
#[derive(Debug)]
pub struct PreRunClockPublicationInspectionV1 {
    boot_id_sha256: String,
    full_binding_sha256: String,
    generation_epoch_id: String,
    post_publication: ClockSampleV1,
    pre_publication: ClockSampleV1,
    profile_id: String,
    publication: DurableReplayPublicationInspectionV1,
    replay_slot_sha256: String,
    run_identity_sha256: String,
}

impl PreRunClockPublicationInspectionV1 {
    pub fn boot_id_sha256(&self) -> &str {
        &self.boot_id_sha256
    }

    pub fn full_binding_sha256(&self) -> &str {
        &self.full_binding_sha256
    }

    pub fn generation_epoch_id(&self) -> &str {
        &self.generation_epoch_id
    }

    pub fn post_boottime_after_seconds(&self) -> i64 {
        self.post_publication.boottime_after.tv_sec
    }

    pub fn post_realtime_seconds(&self) -> i64 {
        self.post_publication.realtime.tv_sec
    }

    pub fn pre_boottime_before_seconds(&self) -> i64 {
        self.pre_publication.boottime_before.tv_sec
    }

    pub fn pre_realtime_seconds(&self) -> i64 {
        self.pre_publication.realtime.tv_sec
    }

    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn replay_slot_sha256(&self) -> &str {
        &self.replay_slot_sha256
    }

    pub fn run_identity_sha256(&self) -> &str {
        &self.run_identity_sha256
    }

    pub fn retained_descriptor_count(&self) -> usize {
        self.publication.retained_descriptor_count()
    }

    pub const fn exact_publication_read_back_observed(&self) -> bool {
        true
    }

    pub const fn wall_clock_publication_sequence_observed(&self) -> bool {
        true
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn crash_reboot_qualified(&self) -> bool {
        false
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }
}

pub fn require_production_wall_clock_supervisor_policy() -> ReplayStoreResultV1<()> {
    compiled_production_wall_clock_supervisor_policy()
        .map(|_| ())
        .ok_or(ReplayStoreErrorV1::Blocked(
            "compiled production wall-clock supervisor policy is absent",
        ))
}

/// The only effectful N4a entrypoint. Its internal order cannot be rearranged
/// by a caller. Production remains blocked before procfs, clocks, or replay
/// storage are touched because the compiled supervisor policy is absent.
pub fn inspect_pre_run_clock_publication_sequence(
    store: &ReplayStoreAnchorV1,
    claim: &PreparedPreRunReplayClaimV1,
) -> ReplayStoreResultV1<PreRunClockPublicationInspectionV1> {
    let policy = compiled_production_wall_clock_supervisor_policy().ok_or(
        ReplayStoreErrorV1::Blocked("compiled production wall-clock supervisor policy is absent"),
    )?;
    let binding = PreRunClockClaimBindingV1::from_prepared(claim);
    if binding.platform_scope != policy.platform_scope {
        return Err(invalid(
            "prepared replay platform differs from the compiled clock policy",
        ));
    }
    let mut source = LinuxClockBootSourceV1::open()?;
    inspect_sequence_with_source_and_publisher(&binding, &mut source, || {
        publish_pre_run_claim_once(store, claim)
    })
}

impl PreRunClockClaimBindingV1 {
    fn from_prepared(claim: &PreparedPreRunReplayClaimV1) -> Self {
        Self {
            boot_id_sha256: claim.boot_id_sha256().to_string(),
            expires_at_unix_seconds: claim.expires_at_unix_seconds(),
            full_binding_sha256: claim.full_binding_sha256().to_string(),
            generation_epoch_id: claim.generation_epoch_id().to_string(),
            maximum_lifetime_seconds: claim.maximum_lifetime_seconds(),
            not_before_unix_seconds: claim.not_before_unix_seconds(),
            platform_scope: claim.platform_scope(),
            profile_id: claim.profile_id().to_string(),
            replay_slot_sha256: claim.replay_slot_sha256().to_string(),
            run_identity_sha256: claim.run_identity_sha256().to_string(),
            store_identity_sha256: claim.pre_run_replay_store_identity_sha256().to_string(),
        }
    }

    fn validate(&self) -> ReplayStoreResultV1<()> {
        if self.platform_scope != ReplayPlatformScopeV1::Nix {
            return Err(invalid(
                "N4a clock-publication sequence accepts only the Nix platform scope",
            ));
        }
        let lifetime = self
            .expires_at_unix_seconds
            .checked_sub(self.not_before_unix_seconds)
            .ok_or_else(|| invalid("signed wall-clock window is reversed"))?;
        if lifetime == 0 || lifetime > self.maximum_lifetime_seconds {
            return Err(invalid(
                "signed wall-clock window differs from its retained lifetime bound",
            ));
        }
        for (value, label) in [
            (&self.boot_id_sha256, "boot identity"),
            (&self.full_binding_sha256, "full replay binding"),
            (&self.replay_slot_sha256, "replay slot"),
            (&self.run_identity_sha256, "run identity"),
            (&self.store_identity_sha256, "replay-store identity"),
        ] {
            validate_sha256(value, label)?;
        }
        if self.generation_epoch_id.is_empty() || self.profile_id.is_empty() {
            return Err(invalid(
                "clock-publication binding lacks profile or generation identity",
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn for_tests(
        boot_id_sha256: String,
        not_before_unix_seconds: u64,
        expires_at_unix_seconds: u64,
        platform_scope: ReplayPlatformScopeV1,
        replay_slot_sha256: String,
        full_binding_sha256: String,
        store_identity_sha256: String,
    ) -> Self {
        Self {
            boot_id_sha256,
            expires_at_unix_seconds,
            full_binding_sha256,
            generation_epoch_id: "test-generation-v1".to_string(),
            maximum_lifetime_seconds: expires_at_unix_seconds
                .saturating_sub(not_before_unix_seconds),
            not_before_unix_seconds,
            platform_scope,
            profile_id: "test-profile-v1".to_string(),
            replay_slot_sha256,
            run_identity_sha256: "7".repeat(64),
            store_identity_sha256,
        }
    }
}

impl LinuxClockBootSourceV1 {
    pub(crate) fn open() -> ReplayStoreResultV1<Self> {
        Ok(Self {
            boot: FixedProcfsBootIdSourceV1::open()?,
        })
    }
}

impl ClockBootSourceV1 for LinuxClockBootSourceV1 {
    fn observe(&mut self) -> ReplayStoreResultV1<ClockBootObservationV1> {
        let boot_before = self.boot.observe_sha256()?;
        let sample = ClockSampleV1::from_system()?;
        let boot_after = self.boot.observe_sha256()?;
        if boot_before != boot_after {
            return Err(ReplayStoreErrorV1::RaceDetected(
                "kernel boot identity changed across one clock sample".to_string(),
            ));
        }
        Ok(ClockBootObservationV1 {
            boot_id_sha256: boot_after,
            sample,
        })
    }

    fn revalidate(&self) -> ReplayStoreResultV1<()> {
        self.boot.revalidate()
    }
}

pub(crate) fn inspect_sequence_with_source_and_publisher<Source, Publish>(
    binding: &PreRunClockClaimBindingV1,
    source: &mut Source,
    publish: Publish,
) -> ReplayStoreResultV1<PreRunClockPublicationInspectionV1>
where
    Source: ClockBootSourceV1,
    Publish: FnOnce() -> ReplayStoreResultV1<DurableReplayPublicationInspectionV1>,
{
    binding.validate()?;
    let pre_publication = source.observe()?;
    validate_observation(binding, &pre_publication)?;
    source.revalidate()?;

    // The publisher can fail after the no-replace rename or directory fsync
    // boundary. Its error type intentionally does not expose a stage that a
    // caller could use for recovery, so every failed publication attempt is
    // conservatively terminal and uncertain for this one-shot slot.
    let publication = publish().map_err(after_publication)?;
    if publication.replay_slot_sha256() != binding.replay_slot_sha256
        || publication.full_binding_sha256() != binding.full_binding_sha256
        || publication.store_identity_sha256() != binding.store_identity_sha256
        || !publication.exact_publication_read_back_observed()
    {
        return Err(uncertain(
            "durable publication differs from the prepared clock binding",
        ));
    }

    let post_publication = source.observe().map_err(after_publication)?;
    source.revalidate().map_err(after_publication)?;
    validate_observation(binding, &post_publication).map_err(after_publication)?;
    validate_sample_sequence(pre_publication.sample, post_publication.sample)
        .map_err(after_publication)?;

    Ok(PreRunClockPublicationInspectionV1 {
        boot_id_sha256: post_publication.boot_id_sha256,
        full_binding_sha256: binding.full_binding_sha256.clone(),
        generation_epoch_id: binding.generation_epoch_id.clone(),
        post_publication: post_publication.sample,
        pre_publication: pre_publication.sample,
        profile_id: binding.profile_id.clone(),
        publication,
        replay_slot_sha256: binding.replay_slot_sha256.clone(),
        run_identity_sha256: binding.run_identity_sha256.clone(),
    })
}

fn validate_observation(
    binding: &PreRunClockClaimBindingV1,
    observation: &ClockBootObservationV1,
) -> ReplayStoreResultV1<()> {
    if observation.boot_id_sha256 != binding.boot_id_sha256 {
        return Err(invalid(
            "observed kernel boot identity differs from the signed replay profile",
        ));
    }
    validate_realtime_window(
        observation.sample,
        binding.not_before_unix_seconds,
        binding.expires_at_unix_seconds,
    )
}

fn compiled_production_wall_clock_supervisor_policy() -> Option<WallClockSupervisorPolicyV1> {
    None
}

fn after_publication(error: ReplayStoreErrorV1) -> ReplayStoreErrorV1 {
    uncertain(error.to_string())
}

fn uncertain(message: impl Into<String>) -> ReplayStoreErrorV1 {
    ReplayStoreErrorV1::UncertainAfterPublicationAttempt(message.into())
}

fn validate_sha256(value: &str, label: &str) -> ReplayStoreResultV1<()> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid(format!("{label} is not canonical SHA-256")));
    }
    Ok(())
}

#[cfg(test)]
impl ClockBootObservationV1 {
    pub(crate) fn for_tests(boot_id_sha256: String, sample: ClockSampleV1) -> Self {
        Self {
            boot_id_sha256,
            sample,
        }
    }
}
