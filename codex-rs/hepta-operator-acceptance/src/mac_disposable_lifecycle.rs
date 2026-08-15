//! Pure, no-authority lifecycle records for a disposable macOS disk-image run.
//!
//! This module deliberately contains no Disk Arbitration, mount, subprocess,
//! or privilege code.  It only validates the durable order of intent and
//! observation records.  An issued effect is always treated as uncertain
//! until a separate observation is durably appended.  Replaying a journal
//! after restart never resumes the forward flow.

use crate::durable::canonical_json;
use crate::mac_apfs_barrier_fixture::AttachmentObligationRecordV1;
use crate::mac_apfs_barrier_fixture::ObligationDispositionV1;
use crate::mac_apfs_barrier_fixture::replay_attachment_obligation_records;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

pub const LIFECYCLE_RECORD_SCHEMA_V2: &str = "hepta_mac_disposable_lifecycle_record_v2";
const HISTORICAL_RECORD_SCHEMA_V1: &str = "hepta_mac_attachment_obligation_record_v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisposableAuthorityV2 {
    pub cutover_authority: bool,
    pub deletion_authority: bool,
    pub privileged_effect_authority: bool,
    pub production_authority: bool,
}

impl DisposableAuthorityV2 {
    pub const fn none() -> Self {
        Self {
            cutover_authority: false,
            deletion_authority: false,
            privileged_effect_authority: false,
            production_authority: false,
        }
    }

    pub const fn any(&self) -> bool {
        self.cutover_authority
            || self.deletion_authority
            || self.privileged_effect_authority
            || self.production_authority
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectPurposeV2 {
    ForwardFlow,
    Reconciliation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackOutcomeV2 {
    Succeeded,
    Failed,
    DeadlineExpired,
    ChannelLost,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDispositionV2 {
    Completed,
    Aborted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FreshAbsenceObservationV2 {
    pub artifact_evidence_sha256: String,
    pub baseline_inventory_sha256: String,
    pub backing_identity_sha256: String,
    pub boot_session_uuid: String,
    pub collector_policy_sha256: String,
    pub collector_receipt_sha256: String,
    pub iomedia_evidence_sha256: String,
    pub monotonic_after_nanoseconds: u64,
    pub monotonic_before_nanoseconds: u64,
    pub mount_evidence_sha256: String,
    pub mountpoint_underlying_sha256: String,
    pub no_matching_iomedia: bool,
    pub no_nested_mounts: bool,
    pub operation_nonce: String,
    pub operation_artifacts_absent: bool,
    pub post_inventory_sha256: String,
    /// Exact canonical digest of the one reconciliation snapshot admitted in
    /// this restart epoch.  Fresh-process observations must leave this empty.
    pub reconciliation_snapshot_sha256: Option<String>,
    /// Restart epoch that owns the observation.  Fresh-process observations
    /// must leave this empty.
    pub restart_epoch_nonce: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ReconciliationMatchV2 {
    Zero,
    Unique { mounted: bool },
    Ambiguous { matching_objects: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconciliationSnapshotV2 {
    pub backing_identity_sha256: String,
    pub boot_session_uuid: String,
    pub collector_policy_sha256: String,
    pub collector_receipt_sha256: String,
    pub iomedia_evidence_sha256: String,
    pub match_result: ReconciliationMatchV2,
    pub monotonic_after_nanoseconds: u64,
    pub monotonic_before_nanoseconds: u64,
    pub mount_evidence_sha256: String,
    pub mountpoint_underlying_sha256: String,
    pub operation_nonce: String,
    pub restart_epoch_nonce: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum DisposableLifecycleEventV2 {
    OperationPrepared {
        baseline_inventory_sha256: String,
        backing_identity_sha256: String,
        boot_session_uuid: String,
        collector_policy_sha256: String,
        mountpoint_underlying_sha256: String,
    },
    CreateIssuedOrUncertain {
        effect_id: u64,
    },
    CreateObserved {
        effect_id: u64,
        image_identity_sha256: String,
    },
    AttachIssuedOrUncertain {
        effect_id: u64,
    },
    AttachObserved {
        effect_id: u64,
        topology_sha256: String,
    },
    MountIssuedOrUncertain {
        effect_id: u64,
    },
    MountObserved {
        effect_id: u64,
        mount_observation_sha256: String,
    },
    UnmountIssuedOrUncertain {
        effect_id: u64,
        purpose: EffectPurposeV2,
    },
    UnmountCallbackObserved {
        effect_id: u64,
        outcome: CallbackOutcomeV2,
    },
    UnmountObserved {
        effect_id: u64,
        mount_absence_sha256: String,
    },
    EjectIssuedOrUncertain {
        effect_id: u64,
        purpose: EffectPurposeV2,
    },
    EjectCallbackObserved {
        effect_id: u64,
        outcome: CallbackOutcomeV2,
    },
    EjectObserved {
        effect_id: u64,
        iomedia_absence_sha256: String,
    },
    RestartReconciliationStarted {
        boot_session_uuid: String,
        collector_policy_sha256: String,
        monotonic_nanoseconds: u64,
        restart_epoch_nonce: String,
    },
    ReconciliationSnapshotObserved {
        snapshot: ReconciliationSnapshotV2,
    },
    FreshAbsenceObserved {
        observation: FreshAbsenceObservationV2,
    },
    ManualIntervention {
        reason_sha256: String,
    },
    Quarantined {
        reason_sha256: String,
    },
    TerminalAbsenceProved {
        disposition: TerminalDispositionV2,
        fresh_absence_sha256: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DisposableLifecycleRecordV2 {
    pub authority: DisposableAuthorityV2,
    pub event: DisposableLifecycleEventV2,
    pub operation_nonce: String,
    pub previous_record_sha256: Option<String>,
    pub schema: String,
    pub schema_version: u32,
    pub sequence: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleDispositionV2 {
    Outstanding,
    ManualIntervention,
    Quarantined,
    TerminalCompleted,
    TerminalAborted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleInspectionV2 {
    pub authority: DisposableAuthorityV2,
    pub blocks_new_operations: bool,
    pub disposition: LifecycleDispositionV2,
    pub last_effect_id: u64,
    pub operation_nonce: String,
    pub records: usize,
    pub restart_forward_flow_authority: bool,
    pub terminal_record_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoricalLifecycleV1Classification {
    ActiveBlocking,
    QuarantinedBlocking,
    Completed,
}

impl HistoricalLifecycleV1Classification {
    pub const fn blocks_new_operations(self) -> bool {
        match self {
            Self::ActiveBlocking | Self::QuarantinedBlocking | Self::Completed => true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LifecycleDispatchV2 {
    V2(LifecycleInspectionV2),
    HistoricalV1(HistoricalLifecycleV1Classification),
}

impl LifecycleDispatchV2 {
    pub const fn blocks_new_operations(&self) -> bool {
        match self {
            Self::V2(inspection) => inspection.blocks_new_operations,
            Self::HistoricalV1(classification) => classification.blocks_new_operations(),
        }
    }
}

#[derive(Debug, Error)]
pub enum LifecycleErrorV2 {
    #[error("invalid disposable lifecycle: {0}")]
    Invalid(String),
    #[error("disposable lifecycle append was not persisted: {0}")]
    Persistence(String),
    #[error("disposable lifecycle JSON failed: {0}")]
    Serialization(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Empty,
    Prepared,
    Created,
    Attached,
    Mounted,
    Ejected,
    Terminal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EffectKind {
    Create,
    Attach,
    Mount,
    Unmount,
    Eject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    FreshProcess,
    Replay,
    RestartReconcileOnly,
}

#[derive(Clone, Debug)]
struct Reducer {
    backing_identity_sha256: Option<String>,
    baseline_inventory_sha256: Option<String>,
    boot_session_uuid: Option<String>,
    callback_seen: bool,
    callback_succeeded: bool,
    collector_policy_sha256: Option<String>,
    fresh_absence_sha256: Option<String>,
    epoch_snapshot_seen: bool,
    epoch_snapshot_sha256: Option<String>,
    epoch_snapshot_was_zero: bool,
    is_replay: bool,
    last_effect_id: u64,
    manual: bool,
    mode: Mode,
    mountpoint_underlying_sha256: Option<String>,
    operation_nonce: String,
    pending: Option<(u64, EffectKind)>,
    phase: Phase,
    quarantined: bool,
    reconciliation_after_nanoseconds: u64,
    reconciliation_boot_session_uuid: Option<String>,
    reconciliation_receipts: std::collections::BTreeSet<String>,
    restart_epoch_open: bool,
    restart_epoch_nonces: std::collections::BTreeSet<String>,
    restart_epoch_nonce: Option<String>,
    restart_epoch_boot_session_uuid: Option<String>,
    restart_epoch_collector_policy_sha256: Option<String>,
    restart_monotonic_nanoseconds: u64,
    terminal: Option<TerminalDispositionV2>,
}

impl Reducer {
    fn new(mode: Mode, operation_nonce: &str) -> Self {
        Self {
            backing_identity_sha256: None,
            baseline_inventory_sha256: None,
            boot_session_uuid: None,
            callback_seen: false,
            callback_succeeded: false,
            collector_policy_sha256: None,
            fresh_absence_sha256: None,
            epoch_snapshot_seen: false,
            epoch_snapshot_sha256: None,
            epoch_snapshot_was_zero: false,
            is_replay: mode == Mode::Replay,
            last_effect_id: 0,
            manual: false,
            mode,
            mountpoint_underlying_sha256: None,
            operation_nonce: operation_nonce.to_string(),
            pending: None,
            phase: Phase::Empty,
            quarantined: false,
            reconciliation_after_nanoseconds: 0,
            reconciliation_boot_session_uuid: None,
            reconciliation_receipts: std::collections::BTreeSet::new(),
            restart_epoch_open: false,
            restart_epoch_nonces: std::collections::BTreeSet::new(),
            restart_epoch_nonce: None,
            restart_epoch_boot_session_uuid: None,
            restart_epoch_collector_policy_sha256: None,
            restart_monotonic_nanoseconds: 0,
            terminal: None,
        }
    }

    fn apply(&mut self, event: &DisposableLifecycleEventV2) -> Result<(), LifecycleErrorV2> {
        if self.phase == Phase::Terminal {
            return Err(invalid("record follows the unique terminal record"));
        }
        if self.mode == Mode::RestartReconcileOnly
            && !matches!(
                event,
                DisposableLifecycleEventV2::RestartReconciliationStarted { .. }
                    | DisposableLifecycleEventV2::ReconciliationSnapshotObserved { .. }
                    | DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
                        purpose: EffectPurposeV2::Reconciliation,
                        ..
                    }
                    | DisposableLifecycleEventV2::UnmountCallbackObserved { .. }
                    | DisposableLifecycleEventV2::UnmountObserved { .. }
                    | DisposableLifecycleEventV2::EjectIssuedOrUncertain {
                        purpose: EffectPurposeV2::Reconciliation,
                        ..
                    }
                    | DisposableLifecycleEventV2::EjectCallbackObserved { .. }
                    | DisposableLifecycleEventV2::EjectObserved { .. }
                    | DisposableLifecycleEventV2::FreshAbsenceObserved { .. }
                    | DisposableLifecycleEventV2::ManualIntervention { .. }
                    | DisposableLifecycleEventV2::Quarantined { .. }
                    | DisposableLifecycleEventV2::TerminalAbsenceProved {
                        disposition: TerminalDispositionV2::Aborted,
                        ..
                    }
            )
        {
            return Err(invalid(
                "restart may reconcile or abort, never resume forward flow",
            ));
        }
        if self.mode == Mode::RestartReconcileOnly
            && !self.restart_epoch_open
            && !matches!(
                event,
                DisposableLifecycleEventV2::RestartReconciliationStarted { .. }
            )
        {
            return Err(invalid(
                "restart reconciliation requires a durable process-epoch record",
            ));
        }
        if self.fresh_absence_sha256.is_some()
            && !matches!(
                event,
                DisposableLifecycleEventV2::RestartReconciliationStarted { .. }
                    | DisposableLifecycleEventV2::ManualIntervention { .. }
                    | DisposableLifecycleEventV2::Quarantined { .. }
                    | DisposableLifecycleEventV2::TerminalAbsenceProved { .. }
            )
        {
            return Err(invalid(
                "fresh absence must be followed only by disposition records",
            ));
        }
        match event {
            DisposableLifecycleEventV2::OperationPrepared {
                baseline_inventory_sha256,
                backing_identity_sha256,
                boot_session_uuid,
                collector_policy_sha256,
                mountpoint_underlying_sha256,
            } => {
                require_digest(baseline_inventory_sha256, "baseline inventory")?;
                require_digest(backing_identity_sha256, "backing identity")?;
                require_digest(collector_policy_sha256, "collector policy")?;
                require_digest(mountpoint_underlying_sha256, "mountpoint identity")?;
                require_uuid(boot_session_uuid)?;
                if self.phase != Phase::Empty {
                    return Err(invalid("OperationPrepared is not the first record"));
                }
                self.baseline_inventory_sha256 = Some(baseline_inventory_sha256.clone());
                self.backing_identity_sha256 = Some(backing_identity_sha256.clone());
                self.boot_session_uuid = Some(boot_session_uuid.clone());
                self.collector_policy_sha256 = Some(collector_policy_sha256.clone());
                self.mountpoint_underlying_sha256 = Some(mountpoint_underlying_sha256.clone());
                self.phase = Phase::Prepared;
            }
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id } => {
                self.issue(*effect_id, EffectKind::Create, Phase::Prepared)?;
            }
            DisposableLifecycleEventV2::CreateObserved {
                effect_id,
                image_identity_sha256,
            } => {
                require_digest(image_identity_sha256, "created image identity")?;
                self.observe(*effect_id, EffectKind::Create, Phase::Created)?;
            }
            DisposableLifecycleEventV2::AttachIssuedOrUncertain { effect_id } => {
                self.issue(*effect_id, EffectKind::Attach, Phase::Created)?;
            }
            DisposableLifecycleEventV2::AttachObserved {
                effect_id,
                topology_sha256,
            } => {
                require_digest(topology_sha256, "attached topology")?;
                self.observe(*effect_id, EffectKind::Attach, Phase::Attached)?;
            }
            DisposableLifecycleEventV2::MountIssuedOrUncertain { effect_id } => {
                self.issue(*effect_id, EffectKind::Mount, Phase::Attached)?;
            }
            DisposableLifecycleEventV2::MountObserved {
                effect_id,
                mount_observation_sha256,
            } => {
                require_digest(mount_observation_sha256, "mount observation")?;
                self.observe(*effect_id, EffectKind::Mount, Phase::Mounted)?;
            }
            DisposableLifecycleEventV2::UnmountIssuedOrUncertain { effect_id, purpose } => {
                self.require_purpose(*purpose)?;
                self.issue(*effect_id, EffectKind::Unmount, Phase::Mounted)?;
            }
            DisposableLifecycleEventV2::UnmountCallbackObserved { effect_id, outcome } => {
                self.callback(*effect_id, EffectKind::Unmount, *outcome)?;
            }
            DisposableLifecycleEventV2::UnmountObserved {
                effect_id,
                mount_absence_sha256,
            } => {
                require_digest(mount_absence_sha256, "mount absence")?;
                self.observe_after_callback(*effect_id, EffectKind::Unmount, Phase::Attached)?;
            }
            DisposableLifecycleEventV2::EjectIssuedOrUncertain { effect_id, purpose } => {
                self.require_purpose(*purpose)?;
                self.issue(*effect_id, EffectKind::Eject, Phase::Attached)?;
            }
            DisposableLifecycleEventV2::EjectCallbackObserved { effect_id, outcome } => {
                self.callback(*effect_id, EffectKind::Eject, *outcome)?;
            }
            DisposableLifecycleEventV2::EjectObserved {
                effect_id,
                iomedia_absence_sha256,
            } => {
                require_digest(iomedia_absence_sha256, "IOMedia absence")?;
                self.observe_after_callback(*effect_id, EffectKind::Eject, Phase::Ejected)?;
            }
            DisposableLifecycleEventV2::RestartReconciliationStarted {
                boot_session_uuid,
                collector_policy_sha256,
                monotonic_nanoseconds,
                restart_epoch_nonce,
            } => {
                require_uuid(boot_session_uuid)?;
                require_digest(collector_policy_sha256, "restart collector policy")?;
                require_nonce(restart_epoch_nonce)?;
                let same_boot =
                    self.restart_epoch_boot_session_uuid.as_ref() == Some(boot_session_uuid);
                if self.mode == Mode::FreshProcess
                    || (self.restart_epoch_open && !self.is_replay)
                    || *monotonic_nanoseconds == 0
                    || (same_boot && *monotonic_nanoseconds <= self.restart_monotonic_nanoseconds)
                    || self.restart_epoch_nonces.contains(restart_epoch_nonce)
                {
                    return Err(invalid(
                        "restart reconciliation epoch is stale or duplicated",
                    ));
                }
                if !same_boot {
                    self.reconciliation_after_nanoseconds = 0;
                    self.reconciliation_boot_session_uuid = None;
                    self.restart_monotonic_nanoseconds = 0;
                }
                self.mode = Mode::RestartReconcileOnly;
                self.restart_epoch_open = true;
                self.epoch_snapshot_seen = false;
                self.epoch_snapshot_sha256 = None;
                self.epoch_snapshot_was_zero = false;
                self.restart_epoch_boot_session_uuid = Some(boot_session_uuid.clone());
                self.restart_epoch_collector_policy_sha256 = Some(collector_policy_sha256.clone());
                self.restart_epoch_nonces
                    .insert(restart_epoch_nonce.clone());
                self.restart_epoch_nonce = Some(restart_epoch_nonce.clone());
                self.restart_monotonic_nanoseconds = *monotonic_nanoseconds;
                self.fresh_absence_sha256 = None;
            }
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot } => {
                self.apply_reconciliation_snapshot(snapshot)?;
            }
            DisposableLifecycleEventV2::FreshAbsenceObserved { observation } => {
                self.validate_fresh_absence(observation)?;
                if self.phase == Phase::Empty
                    || self.fresh_absence_sha256.is_some()
                    || self.pending.is_some()
                    || self.callback_seen
                    || self.callback_succeeded
                    || self.manual
                    || self.quarantined
                {
                    return Err(invalid(
                        "fresh absence observation is out of order or an effect remains unresolved",
                    ));
                }
                if self.mode == Mode::RestartReconcileOnly {
                    if !self.epoch_snapshot_seen
                        || (self.epoch_snapshot_was_zero && self.phase != Phase::Prepared)
                        || (!self.epoch_snapshot_was_zero && self.phase != Phase::Ejected)
                    {
                        return Err(invalid(
                            "restart fresh absence does not follow its exact reconciliation state",
                        ));
                    }
                } else if self.phase != Phase::Ejected {
                    return Err(invalid(
                        "forward fresh absence requires the fully observed eject flow",
                    ));
                }
                self.fresh_absence_sha256 = Some(fresh_absence_sha256(observation)?);
            }
            DisposableLifecycleEventV2::ManualIntervention { reason_sha256 } => {
                require_digest(reason_sha256, "manual-intervention reason")?;
                self.manual = true;
            }
            DisposableLifecycleEventV2::Quarantined { reason_sha256 } => {
                require_digest(reason_sha256, "quarantine reason")?;
                if !self.manual {
                    return Err(invalid("Quarantined requires ManualIntervention"));
                }
                self.quarantined = true;
            }
            DisposableLifecycleEventV2::TerminalAbsenceProved {
                disposition,
                fresh_absence_sha256,
            } => {
                require_digest(fresh_absence_sha256, "terminal fresh absence")?;
                if self.fresh_absence_sha256.as_ref() != Some(fresh_absence_sha256) {
                    return Err(invalid(
                        "terminal does not bind the fresh absence observation",
                    ));
                }
                if self.pending.is_some()
                    || self.callback_seen
                    || self.callback_succeeded
                    || self.manual
                    || self.quarantined
                {
                    return Err(invalid(
                        "terminal cannot close an unresolved or quarantined lifecycle",
                    ));
                }
                match disposition {
                    TerminalDispositionV2::Completed
                        if self.mode == Mode::RestartReconcileOnly
                            || self.phase != Phase::Ejected =>
                    {
                        return Err(invalid(
                            "completed terminal requires the uninterrupted full eject flow",
                        ));
                    }
                    TerminalDispositionV2::Aborted
                        if self.mode != Mode::RestartReconcileOnly
                            || !self.restart_epoch_open
                            || !self.epoch_snapshot_seen
                            || (self.epoch_snapshot_was_zero && self.phase != Phase::Prepared)
                            || (!self.epoch_snapshot_was_zero && self.phase != Phase::Ejected) =>
                    {
                        return Err(invalid(
                            "aborted terminal requires a fresh restart reconciliation closure",
                        ));
                    }
                    TerminalDispositionV2::Completed | TerminalDispositionV2::Aborted => {}
                }
                self.terminal = Some(*disposition);
                self.phase = Phase::Terminal;
                self.pending = None;
                self.restart_epoch_open = false;
            }
        }
        Ok(())
    }

    fn issue(
        &mut self,
        effect_id: u64,
        kind: EffectKind,
        required_phase: Phase,
    ) -> Result<(), LifecycleErrorV2> {
        if self.phase != required_phase
            || self.pending.is_some()
            || self.manual
            || self.quarantined
            || effect_id != self.last_effect_id.checked_add(1).unwrap_or(0)
        {
            return Err(invalid("effect id, phase, or admission state is invalid"));
        }
        self.last_effect_id = effect_id;
        self.pending = Some((effect_id, kind));
        self.callback_seen = false;
        self.callback_succeeded = false;
        Ok(())
    }

    fn observe(
        &mut self,
        effect_id: u64,
        kind: EffectKind,
        next_phase: Phase,
    ) -> Result<(), LifecycleErrorV2> {
        if self.pending != Some((effect_id, kind)) || self.manual || self.quarantined {
            return Err(invalid("effect observation has no exact pending effect"));
        }
        self.pending = None;
        self.callback_seen = false;
        self.callback_succeeded = false;
        self.phase = next_phase;
        Ok(())
    }

    fn observe_after_callback(
        &mut self,
        effect_id: u64,
        kind: EffectKind,
        next_phase: Phase,
    ) -> Result<(), LifecycleErrorV2> {
        if !self.callback_seen || !self.callback_succeeded {
            return Err(invalid(
                "unmount/eject observation requires one successful callback observation",
            ));
        }
        self.observe(effect_id, kind, next_phase)
    }

    fn callback(
        &mut self,
        effect_id: u64,
        kind: EffectKind,
        outcome: CallbackOutcomeV2,
    ) -> Result<(), LifecycleErrorV2> {
        if self.pending != Some((effect_id, kind)) || self.callback_seen {
            return Err(invalid(
                "callback observation is missing, stale, or duplicated",
            ));
        }
        self.callback_seen = true;
        self.callback_succeeded = outcome == CallbackOutcomeV2::Succeeded;
        Ok(())
    }

    fn apply_reconciliation_snapshot(
        &mut self,
        snapshot: &ReconciliationSnapshotV2,
    ) -> Result<(), LifecycleErrorV2> {
        if self.mode != Mode::RestartReconcileOnly
            || self.epoch_snapshot_seen
            || snapshot.operation_nonce != self.operation_nonce
            || self.restart_epoch_nonce.as_ref() != Some(&snapshot.restart_epoch_nonce)
            || self.restart_epoch_boot_session_uuid.as_ref() != Some(&snapshot.boot_session_uuid)
            || self.restart_epoch_collector_policy_sha256.as_ref()
                != Some(&snapshot.collector_policy_sha256)
            || snapshot.monotonic_before_nanoseconds == 0
            || snapshot.monotonic_after_nanoseconds < snapshot.monotonic_before_nanoseconds
            || snapshot.monotonic_before_nanoseconds <= self.restart_monotonic_nanoseconds
            || snapshot.monotonic_after_nanoseconds <= self.restart_monotonic_nanoseconds
            || snapshot.monotonic_before_nanoseconds <= self.reconciliation_after_nanoseconds
            || self
                .reconciliation_receipts
                .contains(&snapshot.collector_receipt_sha256)
        {
            return Err(invalid(
                "reconciliation snapshot identity or window is invalid",
            ));
        }
        require_uuid(&snapshot.boot_session_uuid)?;
        require_nonce(&snapshot.restart_epoch_nonce)?;
        for (value, label) in [
            (
                &snapshot.backing_identity_sha256,
                "reconciliation backing identity",
            ),
            (
                &snapshot.collector_policy_sha256,
                "reconciliation collector policy",
            ),
            (
                &snapshot.collector_receipt_sha256,
                "reconciliation collector receipt",
            ),
            (
                &snapshot.iomedia_evidence_sha256,
                "reconciliation IOMedia evidence",
            ),
            (
                &snapshot.mount_evidence_sha256,
                "reconciliation mount evidence",
            ),
            (
                &snapshot.mountpoint_underlying_sha256,
                "reconciliation mountpoint identity",
            ),
        ] {
            require_digest(value, label)?;
        }
        if self.backing_identity_sha256.as_ref() != Some(&snapshot.backing_identity_sha256)
            || self.mountpoint_underlying_sha256.as_ref()
                != Some(&snapshot.mountpoint_underlying_sha256)
        {
            return Err(invalid(
                "reconciliation snapshot differs from prepared bindings",
            ));
        }
        self.pending = None;
        self.callback_seen = false;
        self.callback_succeeded = false;
        self.reconciliation_after_nanoseconds = snapshot.monotonic_after_nanoseconds;
        self.reconciliation_boot_session_uuid = Some(snapshot.boot_session_uuid.clone());
        self.epoch_snapshot_seen = true;
        self.epoch_snapshot_sha256 = Some(reconciliation_snapshot_sha256(snapshot)?);
        self.epoch_snapshot_was_zero = matches!(snapshot.match_result, ReconciliationMatchV2::Zero);
        self.reconciliation_receipts
            .insert(snapshot.collector_receipt_sha256.clone());
        match snapshot.match_result {
            ReconciliationMatchV2::Zero => {
                self.phase = Phase::Prepared;
            }
            ReconciliationMatchV2::Unique { mounted } => {
                self.phase = if mounted {
                    Phase::Mounted
                } else {
                    Phase::Attached
                };
            }
            ReconciliationMatchV2::Ambiguous { matching_objects } => {
                if matching_objects < 2 {
                    return Err(invalid(
                        "ambiguous snapshot must contain at least two matches",
                    ));
                }
                self.manual = true;
            }
        }
        Ok(())
    }

    fn validate_fresh_absence(
        &self,
        observation: &FreshAbsenceObservationV2,
    ) -> Result<(), LifecycleErrorV2> {
        validate_fresh_absence_shape(observation)?;
        if observation.operation_nonce != self.operation_nonce
            || self.baseline_inventory_sha256.as_ref()
                != Some(&observation.baseline_inventory_sha256)
            || self.backing_identity_sha256.as_ref() != Some(&observation.backing_identity_sha256)
            || self.mountpoint_underlying_sha256.as_ref()
                != Some(&observation.mountpoint_underlying_sha256)
            || (self.mode == Mode::RestartReconcileOnly
                && (!self.epoch_snapshot_seen
                    || self.restart_epoch_nonce.as_ref()
                        != observation.restart_epoch_nonce.as_ref()
                    || self.epoch_snapshot_sha256.as_ref()
                        != observation.reconciliation_snapshot_sha256.as_ref()
                    || self.restart_epoch_boot_session_uuid.as_ref()
                        != Some(&observation.boot_session_uuid)
                    || self.restart_epoch_collector_policy_sha256.as_ref()
                        != Some(&observation.collector_policy_sha256)
                    || self.reconciliation_boot_session_uuid.as_ref()
                        != Some(&observation.boot_session_uuid)
                    || observation.monotonic_before_nanoseconds
                        <= self.reconciliation_after_nanoseconds
                    || (!self.epoch_snapshot_was_zero && self.phase != Phase::Ejected)))
            || (self.mode != Mode::RestartReconcileOnly
                && (self.boot_session_uuid.as_ref() != Some(&observation.boot_session_uuid)
                    || self.collector_policy_sha256.as_ref()
                        != Some(&observation.collector_policy_sha256)
                    || observation.restart_epoch_nonce.is_some()
                    || observation.reconciliation_snapshot_sha256.is_some()))
        {
            return Err(invalid(
                "fresh absence differs from prepared operation bindings",
            ));
        }
        Ok(())
    }

    fn require_purpose(&self, purpose: EffectPurposeV2) -> Result<(), LifecycleErrorV2> {
        let expected = if self.mode == Mode::RestartReconcileOnly {
            EffectPurposeV2::Reconciliation
        } else {
            EffectPurposeV2::ForwardFlow
        };
        if purpose != expected {
            return Err(invalid(
                "effect purpose does not match process lifecycle mode",
            ));
        }
        if purpose == EffectPurposeV2::Reconciliation
            && (!self.restart_epoch_open
                || !self.epoch_snapshot_seen
                || self.epoch_snapshot_was_zero)
        {
            return Err(invalid(
                "reconciliation effect requires one exact current-epoch unique target",
            ));
        }
        Ok(())
    }

    fn disposition(&self) -> LifecycleDispositionV2 {
        match self.terminal {
            Some(TerminalDispositionV2::Completed) => LifecycleDispositionV2::TerminalCompleted,
            Some(TerminalDispositionV2::Aborted) => LifecycleDispositionV2::TerminalAborted,
            None if self.quarantined => LifecycleDispositionV2::Quarantined,
            None if self.manual => LifecycleDispositionV2::ManualIntervention,
            None => LifecycleDispositionV2::Outstanding,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DisposableLifecycleJournalV2 {
    operation_nonce: String,
    persistence_uncertain: bool,
    previous_record_sha256: Option<String>,
    records: usize,
    reducer: Reducer,
    sequence: u32,
}

impl DisposableLifecycleJournalV2 {
    pub fn new(operation_nonce: &str) -> Result<Self, LifecycleErrorV2> {
        require_nonce(operation_nonce)?;
        Ok(Self {
            operation_nonce: operation_nonce.to_string(),
            persistence_uncertain: false,
            previous_record_sha256: None,
            records: 0,
            reducer: Reducer::new(Mode::FreshProcess, operation_nonce),
            sequence: 1,
        })
    }

    pub fn resume_for_reconciliation(records: &[Vec<u8>]) -> Result<Self, LifecycleErrorV2> {
        let mut journal = replay(records)?;
        journal.reducer.mode = Mode::RestartReconcileOnly;
        journal.reducer.is_replay = false;
        journal.reducer.restart_epoch_open = false;
        Ok(journal)
    }

    pub fn append_with<F>(
        &mut self,
        event: DisposableLifecycleEventV2,
        persist: F,
    ) -> Result<String, LifecycleErrorV2>
    where
        F: FnOnce(&DisposableLifecycleRecordV2, &[u8]) -> std::io::Result<()>,
    {
        if self.persistence_uncertain {
            return Err(invalid(
                "journal persistence is issued-or-uncertain; descriptor replay is required",
            ));
        }
        let next_sequence = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| invalid("record sequence overflows before persistence"))?;
        let next_records = self
            .records
            .checked_add(1)
            .ok_or_else(|| invalid("record count overflows before persistence"))?;
        let mut next_reducer = self.reducer.clone();
        next_reducer.apply(&event)?;
        let record = DisposableLifecycleRecordV2 {
            authority: DisposableAuthorityV2::none(),
            event,
            operation_nonce: self.operation_nonce.clone(),
            previous_record_sha256: self.previous_record_sha256.clone(),
            schema: LIFECYCLE_RECORD_SCHEMA_V2.to_string(),
            schema_version: 2,
            sequence: self.sequence,
        };
        let bytes = canonical_record(&record)?;
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| persist(&record, &bytes))) {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                self.persistence_uncertain = true;
                return Err(LifecycleErrorV2::Persistence(error.to_string()));
            }
            Err(_) => {
                self.persistence_uncertain = true;
                return Err(LifecycleErrorV2::Persistence(
                    "persistence callback panicked after the write became issued-or-uncertain"
                        .to_string(),
                ));
            }
        }
        let digest = sha256(&bytes);
        self.previous_record_sha256 = Some(digest.clone());
        self.sequence = next_sequence;
        self.records = next_records;
        self.reducer = next_reducer;
        Ok(digest)
    }

    pub fn disposition(&self) -> LifecycleDispositionV2 {
        if self.persistence_uncertain {
            LifecycleDispositionV2::Outstanding
        } else {
            self.reducer.disposition()
        }
    }

    pub fn persistence_uncertain(&self) -> bool {
        self.persistence_uncertain
    }

    pub fn last_effect_id(&self) -> u64 {
        self.reducer.last_effect_id
    }
}

pub fn inspect_lifecycle_v2(
    records: &[Vec<u8>],
) -> Result<LifecycleInspectionV2, LifecycleErrorV2> {
    let journal = replay(records)?;
    let terminal_record_sha256 = journal
        .previous_record_sha256
        .clone()
        .ok_or_else(|| invalid("lifecycle contains no records"))?;
    let disposition = journal.reducer.disposition();
    Ok(LifecycleInspectionV2 {
        authority: DisposableAuthorityV2::none(),
        blocks_new_operations: !matches!(
            disposition,
            LifecycleDispositionV2::TerminalCompleted | LifecycleDispositionV2::TerminalAborted
        ),
        disposition,
        last_effect_id: journal.reducer.last_effect_id,
        operation_nonce: journal.operation_nonce,
        records: journal.records,
        restart_forward_flow_authority: false,
        terminal_record_sha256,
    })
}

pub fn dispatch_lifecycle_records(
    records: &[Vec<u8>],
) -> Result<LifecycleDispatchV2, LifecycleErrorV2> {
    let first = records
        .first()
        .ok_or_else(|| invalid("lifecycle contains no records"))?;
    let value: serde_json::Value = serde_json::from_slice(first)
        .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))?;
    match value.get("schema").and_then(serde_json::Value::as_str) {
        Some(LIFECYCLE_RECORD_SCHEMA_V2) => {
            inspect_lifecycle_v2(records).map(LifecycleDispatchV2::V2)
        }
        Some(HISTORICAL_RECORD_SCHEMA_V1) => {
            classify_historical_v1(records).map(LifecycleDispatchV2::HistoricalV1)
        }
        _ => Err(invalid("unknown lifecycle schema")),
    }
}

pub fn fresh_absence_sha256(
    observation: &FreshAbsenceObservationV2,
) -> Result<String, LifecycleErrorV2> {
    canonical_json(observation)
        .map(|bytes| sha256(&bytes))
        .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))
}

pub fn reconciliation_snapshot_sha256(
    snapshot: &ReconciliationSnapshotV2,
) -> Result<String, LifecycleErrorV2> {
    canonical_json(snapshot)
        .map(|bytes| sha256(&bytes))
        .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))
}

fn replay(records: &[Vec<u8>]) -> Result<DisposableLifecycleJournalV2, LifecycleErrorV2> {
    if records.is_empty() {
        return Err(invalid("lifecycle contains no records"));
    }
    let first: DisposableLifecycleRecordV2 = serde_json::from_slice(&records[0])
        .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))?;
    require_nonce(&first.operation_nonce)?;
    let mut reducer = Reducer::new(Mode::Replay, &first.operation_nonce);
    let mut operation_nonce = None;
    let mut previous = None;
    for (index, bytes) in records.iter().enumerate() {
        let record: DisposableLifecycleRecordV2 = serde_json::from_slice(bytes)
            .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))?;
        if canonical_record(&record)? != *bytes
            || record.schema != LIFECYCLE_RECORD_SCHEMA_V2
            || record.schema_version != 2
            || record.authority.any()
            || record.sequence != (index + 1) as u32
            || record.previous_record_sha256 != previous
        {
            return Err(invalid(
                "record differs from its canonical hash-chain envelope",
            ));
        }
        require_nonce(&record.operation_nonce)?;
        if operation_nonce
            .as_ref()
            .is_some_and(|nonce| nonce != &record.operation_nonce)
        {
            return Err(invalid("operation nonce changed within lifecycle"));
        }
        operation_nonce = Some(record.operation_nonce.clone());
        reducer.apply(&record.event)?;
        previous = Some(sha256(bytes));
    }
    let records_len = records.len();
    let sequence = u32::try_from(records_len)
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| invalid("record sequence overflowed"))?;
    Ok(DisposableLifecycleJournalV2 {
        operation_nonce: operation_nonce.expect("nonempty lifecycle"),
        persistence_uncertain: false,
        previous_record_sha256: previous,
        records: records_len,
        reducer,
        sequence,
    })
}

fn classify_historical_v1(
    records: &[Vec<u8>],
) -> Result<HistoricalLifecycleV1Classification, LifecycleErrorV2> {
    let decoded = records
        .iter()
        .map(|bytes| {
            serde_json::from_slice::<AttachmentObligationRecordV1>(bytes)
                .map(|record| (record, bytes.clone()))
                .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let verification =
        replay_attachment_obligation_records(&decoded, "00000000-0000-0000-0000-000000000000")
            .map_err(|error| invalid(format!("frozen historical v1 replay failed: {error}")))?;
    match verification.disposition {
        ObligationDispositionV1::Active | ObligationDispositionV1::ReconcileRequired => {
            Ok(HistoricalLifecycleV1Classification::ActiveBlocking)
        }
        ObligationDispositionV1::Quarantined => {
            Ok(HistoricalLifecycleV1Classification::QuarantinedBlocking)
        }
        ObligationDispositionV1::Reconciled => Ok(HistoricalLifecycleV1Classification::Completed),
    }
}

pub(crate) fn validate_fresh_absence_shape(
    observation: &FreshAbsenceObservationV2,
) -> Result<(), LifecycleErrorV2> {
    require_nonce(&observation.operation_nonce)?;
    require_uuid(&observation.boot_session_uuid)?;
    if let Some(nonce) = &observation.restart_epoch_nonce {
        require_nonce(nonce)?;
    }
    if let Some(digest) = &observation.reconciliation_snapshot_sha256 {
        require_digest(digest, "absence reconciliation snapshot")?;
    }
    if observation.restart_epoch_nonce.is_some()
        != observation.reconciliation_snapshot_sha256.is_some()
    {
        return Err(invalid(
            "fresh absence restart epoch and snapshot bindings must be present together",
        ));
    }
    for (value, label) in [
        (
            &observation.artifact_evidence_sha256,
            "absence artifact evidence",
        ),
        (&observation.baseline_inventory_sha256, "absence baseline"),
        (
            &observation.backing_identity_sha256,
            "absence backing identity",
        ),
        (
            &observation.collector_policy_sha256,
            "absence collector policy",
        ),
        (
            &observation.collector_receipt_sha256,
            "absence collector receipt",
        ),
        (
            &observation.iomedia_evidence_sha256,
            "absence IOMedia evidence",
        ),
        (&observation.mount_evidence_sha256, "absence mount evidence"),
        (
            &observation.mountpoint_underlying_sha256,
            "absence mountpoint identity",
        ),
        (&observation.post_inventory_sha256, "absence post inventory"),
    ] {
        require_digest(value, label)?;
    }
    if observation.monotonic_before_nanoseconds == 0
        || observation.monotonic_after_nanoseconds < observation.monotonic_before_nanoseconds
        || observation.baseline_inventory_sha256 != observation.post_inventory_sha256
        || !observation.no_matching_iomedia
        || !observation.no_nested_mounts
        || !observation.operation_artifacts_absent
    {
        return Err(invalid(
            "fresh absence does not prove exact baseline restoration",
        ));
    }
    Ok(())
}

fn canonical_record(record: &DisposableLifecycleRecordV2) -> Result<Vec<u8>, LifecycleErrorV2> {
    canonical_json(record).map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))
}

fn require_nonce(value: &str) -> Result<(), LifecycleErrorV2> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid(
            "operation nonce is not 64 lowercase hexadecimal characters",
        ));
    }
    Ok(())
}

fn require_digest(value: &str, label: &str) -> Result<(), LifecycleErrorV2> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid(format!(
            "{label} is not a lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

fn require_uuid(value: &str) -> Result<(), LifecycleErrorV2> {
    let bytes = value.as_bytes();
    if bytes.len() != 36
        || !bytes
            .iter()
            .any(|byte| byte.is_ascii_hexdigit() && *byte != b'0')
        || !bytes.iter().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => *byte == b'-',
            _ => byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase(),
        })
    {
        return Err(invalid(
            "boot session UUID is not canonical lowercase and non-nil",
        ));
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn invalid(message: impl Into<String>) -> LifecycleErrorV2 {
    LifecycleErrorV2::Invalid(message.into())
}

#[cfg(test)]
#[path = "mac_disposable_lifecycle_tests.rs"]
mod tests;
