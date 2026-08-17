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

/// Exact final collector-receipt inode referenced by one lifecycle
/// observation. Historical records omit this projection; prepared-manifest
/// V3 restart records must carry it. The receipt JSON cannot contain this
/// binding itself because the final inode identity exists only after the
/// receipt has been renamed, reopened, and replayed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CollectorReceiptFileBindingV3 {
    birthtime_nanoseconds: i64,
    birthtime_seconds: i64,
    canonical_sha256: String,
    ctime_nanoseconds: i64,
    ctime_seconds: i64,
    dev: u64,
    final_basename: String,
    flags: u32,
    generation: u32,
    gid: u32,
    inode: u64,
    mode: u32,
    mtime_nanoseconds: i64,
    mtime_seconds: i64,
    nlink: u64,
    root_after: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
    root_generation_ordinal: u32,
    size: u64,
    uid: u32,
}

impl CollectorReceiptFileBindingV3 {
    pub(crate) fn from_retained_collector(
        _seal: crate::mac_disposable_reconciliation_collector::CollectorReceiptFileBindingSealV3,
        canonical_sha256: String,
        final_basename: String,
        binding: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        root_after: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        root_generation_ordinal: u32,
    ) -> Self {
        Self {
            birthtime_nanoseconds: binding.birthtime_nanoseconds,
            birthtime_seconds: binding.birthtime_seconds,
            canonical_sha256,
            ctime_nanoseconds: binding.ctime_nanoseconds,
            ctime_seconds: binding.ctime_seconds,
            dev: binding.dev,
            final_basename,
            flags: binding.flags,
            generation: binding.generation,
            gid: binding.gid,
            inode: binding.inode,
            mode: binding.mode,
            mtime_nanoseconds: binding.mtime_nanoseconds,
            mtime_seconds: binding.mtime_seconds,
            nlink: binding.nlink,
            root_after,
            root_generation_ordinal,
            size: binding.size,
            uid: binding.uid,
        }
    }

    pub(crate) fn canonical_sha256(&self) -> &str {
        &self.canonical_sha256
    }

    pub(crate) fn final_basename(&self) -> &str {
        &self.final_basename
    }

    pub(crate) fn exact_binding(&self) -> crate::mac_iomedia_identity::FilesystemObjectBindingV3 {
        crate::mac_iomedia_identity::FilesystemObjectBindingV3 {
            birthtime_nanoseconds: self.birthtime_nanoseconds,
            birthtime_seconds: self.birthtime_seconds,
            ctime_nanoseconds: self.ctime_nanoseconds,
            ctime_seconds: self.ctime_seconds,
            dev: self.dev,
            flags: self.flags,
            generation: self.generation,
            gid: self.gid,
            inode: self.inode,
            mode: self.mode,
            mtime_nanoseconds: self.mtime_nanoseconds,
            mtime_seconds: self.mtime_seconds,
            nlink: self.nlink,
            size: self.size,
            uid: self.uid,
        }
    }

    pub(crate) const fn root_after(
        &self,
    ) -> crate::mac_iomedia_identity::FilesystemObjectBindingV3 {
        self.root_after
    }

    pub(crate) const fn root_generation_ordinal(&self) -> u32 {
        self.root_generation_ordinal
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        canonical_sha256: String,
        final_basename: String,
        binding: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        root_after: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        root_generation_ordinal: u32,
    ) -> Self {
        Self {
            birthtime_nanoseconds: binding.birthtime_nanoseconds,
            birthtime_seconds: binding.birthtime_seconds,
            canonical_sha256,
            ctime_nanoseconds: binding.ctime_nanoseconds,
            ctime_seconds: binding.ctime_seconds,
            dev: binding.dev,
            final_basename,
            flags: binding.flags,
            generation: binding.generation,
            gid: binding.gid,
            inode: binding.inode,
            mode: binding.mode,
            mtime_nanoseconds: binding.mtime_nanoseconds,
            mtime_seconds: binding.mtime_seconds,
            nlink: binding.nlink,
            root_after,
            root_generation_ordinal,
            size: binding.size,
            uid: binding.uid,
        }
    }
}

/// Durable lifecycle link for one retained collector observation in the
/// terminal namespace-absence lineage.  This is a serializable projection,
/// not a capability: production append remains sealed by the lifecycle store.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TerminalCollectorLineageV3 {
    pub(crate) collector_receipt_sha256: String,
    pub(crate) lifecycle_record_sha256: String,
    pub(crate) lifecycle_sequence: u32,
    pub(crate) observation_sha256: String,
}

impl TerminalCollectorLineageV3 {
    pub(crate) fn new(
        collector_receipt_sha256: String,
        lifecycle_record_sha256: String,
        lifecycle_sequence: u32,
        observation_sha256: String,
    ) -> Result<Self, LifecycleErrorV2> {
        let value = Self {
            collector_receipt_sha256,
            lifecycle_record_sha256,
            lifecycle_sequence,
            observation_sha256,
        };
        validate_terminal_collector_lineage_v3(&value, "terminal collector lineage")?;
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TerminalLatestZeroKindV3 {
    FirstSnapshot,
    PostEject,
}

/// Exact V2 restart-start and V3 restart-admission lineage carried forward to
/// the terminal FreshAbsence observation.  The admission digest is checked
/// against its retained sidecar by the lifecycle store; the reducer also
/// cross-binds the V2 start record, prepared manifest, operation, boot, and
/// restart epoch that it can independently replay.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TerminalRestartAdmissionLineageV3 {
    pub(crate) prepared_manifest_sha256: String,
    pub(crate) prepared_profile_sha256: String,
    pub(crate) process_epoch_sha256: String,
    pub(crate) restart_admission_sha256: String,
    pub(crate) restart_started_lifecycle_record_sha256: String,
    pub(crate) restart_started_lifecycle_sequence: u32,
}

impl TerminalRestartAdmissionLineageV3 {
    pub(crate) fn new(
        prepared_manifest_sha256: String,
        prepared_profile_sha256: String,
        process_epoch_sha256: String,
        restart_admission_sha256: String,
        restart_started_lifecycle_record_sha256: String,
        restart_started_lifecycle_sequence: u32,
    ) -> Result<Self, LifecycleErrorV2> {
        let value = Self {
            prepared_manifest_sha256,
            prepared_profile_sha256,
            process_epoch_sha256,
            restart_admission_sha256,
            restart_started_lifecycle_record_sha256,
            restart_started_lifecycle_sequence,
        };
        validate_terminal_restart_admission_lineage_v3(&value)?;
        Ok(value)
    }
}

/// Exact retained evidence for the two crash-distinct backing-absence cases.
/// A live unlink proves the original retained inode reached `nlink == 0`;
/// recovery proves only that the prepared canonical basename is absent.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "binding", rename_all = "snake_case")]
pub(crate) enum TerminalBackingAbsenceEvidenceV3 {
    LiveUnlinked(crate::mac_iomedia_identity::UnlinkedBackingBindingV3),
    RecoveredPathAbsent(crate::mac_iomedia_identity::BackingPathAbsenceBindingV3),
}

/// Exact retained artifact-root endpoints around the externally observed
/// removal of the sole prepared backing basename.  Endpoint equality does not
/// claim that no transient third-party namespace churn occurred between them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TerminalArtifactNamespaceDeltaV3 {
    pub(crate) after_entries: Vec<String>,
    pub(crate) after_root: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
    pub(crate) authority: DisposableAuthorityV2,
    pub(crate) backing_basename: String,
    pub(crate) before_entries: Vec<String>,
    pub(crate) before_root: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
}

impl TerminalArtifactNamespaceDeltaV3 {
    pub(crate) fn from_retained_endpoints(
        backing_basename: String,
        before_root: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        before_entries: Vec<String>,
        after_root: crate::mac_iomedia_identity::FilesystemObjectBindingV3,
        after_entries: Vec<String>,
    ) -> Result<Self, LifecycleErrorV2> {
        let value = Self {
            after_entries,
            after_root,
            authority: DisposableAuthorityV2::none(),
            backing_basename,
            before_entries,
            before_root,
        };
        validate_terminal_artifact_namespace_delta_v3(&value)?;
        Ok(value)
    }
}

/// Exact predecessor-only closure material embedded in the terminal
/// FreshAbsence observation.  It deliberately contains no digest or lifecycle
/// record identity for that FreshAbsence record itself; those values do not
/// exist until after this projection has been canonicalized and persisted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TerminalFreshAbsenceBindingV3 {
    pub(crate) artifact_evidence_sha256: String,
    pub(crate) artifact_namespace_delta: TerminalArtifactNamespaceDeltaV3,
    pub(crate) authority: DisposableAuthorityV2,
    pub(crate) backing_absence: TerminalBackingAbsenceEvidenceV3,
    pub(crate) boot_session_uuid: String,
    pub(crate) collector_policy_sha256: String,
    pub(crate) first: TerminalCollectorLineageV3,
    pub(crate) fresh_collector_receipt_sha256: String,
    pub(crate) fresh_iomedia_evidence_sha256: String,
    pub(crate) fresh_mount_evidence_sha256: String,
    pub(crate) fresh_receipt_root_generation: u32,
    pub(crate) latest: TerminalCollectorLineageV3,
    pub(crate) latest_zero_kind: TerminalLatestZeroKindV3,
    pub(crate) operation_nonce: String,
    pub(crate) prepared_backing_exact_sha256: String,
    pub(crate) restart: TerminalRestartAdmissionLineageV3,
    pub(crate) restart_epoch_nonce: String,
}

impl TerminalFreshAbsenceBindingV3 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_retained_projection(
        operation_nonce: String,
        boot_session_uuid: String,
        restart_epoch_nonce: String,
        collector_policy_sha256: String,
        prepared_backing_exact_sha256: String,
        restart: TerminalRestartAdmissionLineageV3,
        first: TerminalCollectorLineageV3,
        latest_zero_kind: TerminalLatestZeroKindV3,
        latest: TerminalCollectorLineageV3,
        backing_absence: TerminalBackingAbsenceEvidenceV3,
        artifact_namespace_delta: TerminalArtifactNamespaceDeltaV3,
        artifact_evidence_sha256: String,
        fresh_collector_receipt_sha256: String,
        fresh_receipt_root_generation: u32,
        fresh_iomedia_evidence_sha256: String,
        fresh_mount_evidence_sha256: String,
    ) -> Result<Self, LifecycleErrorV2> {
        let value = Self {
            artifact_evidence_sha256,
            artifact_namespace_delta,
            authority: DisposableAuthorityV2::none(),
            backing_absence,
            boot_session_uuid,
            collector_policy_sha256,
            first,
            fresh_collector_receipt_sha256,
            fresh_iomedia_evidence_sha256,
            fresh_mount_evidence_sha256,
            fresh_receipt_root_generation,
            latest,
            latest_zero_kind,
            operation_nonce,
            prepared_backing_exact_sha256,
            restart,
            restart_epoch_nonce,
        };
        validate_terminal_fresh_absence_binding_shape_v3(&value)?;
        Ok(value)
    }
}

/// Final closure over the already-durable terminal FreshAbsence record.  This
/// record points only backwards: Fresh receipt -> FreshAbsence V2 record ->
/// TerminalAbsenceProved V2, avoiding a hash self-reference.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TerminalNamespaceClosureV3 {
    pub(crate) authority: DisposableAuthorityV2,
    pub(crate) fresh_absence_collector_receipt_sha256: String,
    pub(crate) fresh_absence_lifecycle_record_sha256: String,
    pub(crate) fresh_absence_lifecycle_sequence: u32,
    pub(crate) fresh_absence_sha256: String,
    pub(crate) terminal_binding_sha256: String,
}

impl TerminalNamespaceClosureV3 {
    pub(crate) fn from_retained_fresh(
        binding: &TerminalFreshAbsenceBindingV3,
        fresh_absence_sha256: String,
        fresh_absence_lifecycle_record_sha256: String,
        fresh_absence_lifecycle_sequence: u32,
    ) -> Result<Self, LifecycleErrorV2> {
        let value = Self {
            authority: DisposableAuthorityV2::none(),
            fresh_absence_collector_receipt_sha256: binding.fresh_collector_receipt_sha256.clone(),
            fresh_absence_lifecycle_record_sha256,
            fresh_absence_lifecycle_sequence,
            fresh_absence_sha256,
            terminal_binding_sha256: terminal_fresh_absence_binding_sha256(binding)?,
        };
        validate_terminal_namespace_closure_shape_v3(&value)?;
        Ok(value)
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) collector_receipt_file: Option<CollectorReceiptFileBindingV3>,
    /// Full canonical current-boot IOMedia inventory expected after the one
    /// admitted reconciliation match is absent.  Restart observations must
    /// bind this to the first snapshot; historical forward observations omit
    /// it and retain the V2 baseline-summary rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_expected_absence_inventory_sha256: Option<String>,
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
    /// Exact Stage-E namespace-absence lineage. Historical and forward-flow
    /// observations omit it byte-for-byte; prepared-manifest restart
    /// completion requires it and a matching terminal closure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) terminal_binding_v3: Option<TerminalFreshAbsenceBindingV3>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) collector_receipt_file: Option<CollectorReceiptFileBindingV3>,
    /// Full canonical current-boot IOMedia inventory expected after the exact
    /// match is absent.  Zero and Unique snapshots require this binding;
    /// Ambiguous snapshots deliberately cannot predict one exact absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_expected_absence_inventory_sha256: Option<String>,
    pub iomedia_evidence_sha256: String,
    pub match_result: ReconciliationMatchV2,
    pub monotonic_after_nanoseconds: u64,
    pub monotonic_before_nanoseconds: u64,
    pub mount_evidence_sha256: String,
    pub mountpoint_underlying_sha256: String,
    pub operation_nonce: String,
    pub restart_epoch_nonce: String,
}

/// Exact durable collector evidence produced after a reconciliation effect.
///
/// Historical V2 records omit this side binding.  A live V3 restart must bind
/// every post-effect observation to the same operation/restart epoch, the
/// first reconciliation snapshot that admitted the target, and the newly
/// retained collector receipt that proves the post-effect state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PostEffectCollectorBindingV3 {
    boot_session_uuid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    collector_receipt_file: Option<CollectorReceiptFileBindingV3>,
    collector_receipt_sha256: String,
    first_reconciliation_snapshot_sha256: String,
    observation_sha256: String,
    operation_nonce: String,
    restart_epoch_nonce: String,
}

impl PostEffectCollectorBindingV3 {
    pub(crate) fn from_retained_collector(
        _seal: crate::mac_disposable_reconciliation_collector::PostEffectCollectorBindingSealV3,
        boot_session_uuid: String,
        collector_receipt_file: CollectorReceiptFileBindingV3,
        collector_receipt_sha256: String,
        first_reconciliation_snapshot_sha256: String,
        observation_sha256: String,
        operation_nonce: String,
        restart_epoch_nonce: String,
    ) -> Self {
        Self {
            boot_session_uuid,
            collector_receipt_file: Some(collector_receipt_file),
            collector_receipt_sha256,
            first_reconciliation_snapshot_sha256,
            observation_sha256,
            operation_nonce,
            restart_epoch_nonce,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(
        boot_session_uuid: String,
        collector_receipt_file: Option<CollectorReceiptFileBindingV3>,
        collector_receipt_sha256: String,
        first_reconciliation_snapshot_sha256: String,
        observation_sha256: String,
        operation_nonce: String,
        restart_epoch_nonce: String,
    ) -> Self {
        Self {
            boot_session_uuid,
            collector_receipt_file,
            collector_receipt_sha256,
            first_reconciliation_snapshot_sha256,
            observation_sha256,
            operation_nonce,
            restart_epoch_nonce,
        }
    }

    pub(crate) fn boot_session_uuid(&self) -> &str {
        &self.boot_session_uuid
    }

    pub(crate) fn collector_receipt_sha256(&self) -> &str {
        &self.collector_receipt_sha256
    }

    pub(crate) fn collector_receipt_file(&self) -> Option<&CollectorReceiptFileBindingV3> {
        self.collector_receipt_file.as_ref()
    }

    pub(crate) fn first_reconciliation_snapshot_sha256(&self) -> &str {
        &self.first_reconciliation_snapshot_sha256
    }

    pub(crate) fn observation_sha256(&self) -> &str {
        &self.observation_sha256
    }

    pub(crate) fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub(crate) fn restart_epoch_nonce(&self) -> &str {
        &self.restart_epoch_nonce
    }
}

/// Durable binding from the first lifecycle record to the fixed prepared
/// collector sidecar.  The digest binds its canonical bytes while the full
/// inode identity rejects same-byte replacement before restart admission.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedCollectorManifestBindingV3 {
    pub birthtime_nanoseconds: i64,
    pub birthtime_seconds: i64,
    pub dev: u64,
    pub generation: u32,
    pub inode: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) receipt_root_initial: Option<crate::mac_iomedia_identity::FilesystemObjectBindingV3>,
    pub sha256: String,
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
    OperationPreparedWithManifestV3 {
        baseline_inventory_sha256: String,
        backing_identity_sha256: String,
        boot_session_uuid: String,
        collector_policy_sha256: String,
        mountpoint_underlying_sha256: String,
        prepared_manifest: PreparedCollectorManifestBindingV3,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        collector: Option<PostEffectCollectorBindingV3>,
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
        /// Historical records legitimately omit this projection. New
        /// prepared-manifest V3 records may only reach the reducer through a
        /// sealed retained collector observation and must carry the exact
        /// next receipt-root generation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        collector: Option<PostEffectCollectorBindingV3>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        closure_v3: Option<TerminalNamespaceClosureV3>,
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
    pub prepared_manifest: Option<PreparedCollectorManifestBindingV3>,
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

#[derive(Clone, Copy)]
struct ReducerRecordContext<'a> {
    current_record_sha256: &'a str,
    previous_record_sha256: Option<&'a str>,
    sequence: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleProcessModeV2 {
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
    collector_receipt_root_generation:
        Option<(u32, crate::mac_iomedia_identity::FilesystemObjectBindingV3)>,
    fresh_absence_sha256: Option<String>,
    fresh_absence_record_sha256: Option<String>,
    fresh_absence_record_sequence: Option<u32>,
    fresh_absence_terminal_binding_v3: Option<TerminalFreshAbsenceBindingV3>,
    first_terminal_collector_lineage_v3: Option<TerminalCollectorLineageV3>,
    epoch_snapshot_seen: bool,
    epoch_snapshot_sha256: Option<String>,
    epoch_snapshot_expected_absence_sha256: Option<String>,
    epoch_snapshot_was_zero: bool,
    is_replay: bool,
    last_effect_id: u64,
    latest_terminal_zero_kind_v3: Option<TerminalLatestZeroKindV3>,
    latest_terminal_zero_lineage_v3: Option<TerminalCollectorLineageV3>,
    manual: bool,
    mode: Mode,
    mountpoint_underlying_sha256: Option<String>,
    operation_nonce: String,
    prepared_manifest: Option<PreparedCollectorManifestBindingV3>,
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
    restart_started_lifecycle_record_sha256: Option<String>,
    restart_started_lifecycle_sequence: Option<u32>,
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
            collector_receipt_root_generation: None,
            fresh_absence_sha256: None,
            fresh_absence_record_sha256: None,
            fresh_absence_record_sequence: None,
            fresh_absence_terminal_binding_v3: None,
            first_terminal_collector_lineage_v3: None,
            epoch_snapshot_seen: false,
            epoch_snapshot_sha256: None,
            epoch_snapshot_expected_absence_sha256: None,
            epoch_snapshot_was_zero: false,
            is_replay: mode == Mode::Replay,
            last_effect_id: 0,
            latest_terminal_zero_kind_v3: None,
            latest_terminal_zero_lineage_v3: None,
            manual: false,
            mode,
            mountpoint_underlying_sha256: None,
            operation_nonce: operation_nonce.to_string(),
            prepared_manifest: None,
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
            restart_started_lifecycle_record_sha256: None,
            restart_started_lifecycle_sequence: None,
            restart_monotonic_nanoseconds: 0,
            terminal: None,
        }
    }

    fn apply(
        &mut self,
        event: &DisposableLifecycleEventV2,
        record: ReducerRecordContext<'_>,
    ) -> Result<(), LifecycleErrorV2> {
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
                    | DisposableLifecycleEventV2::TerminalAbsenceProved {
                        disposition: TerminalDispositionV2::Completed,
                        closure_v3: Some(_),
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
            DisposableLifecycleEventV2::OperationPreparedWithManifestV3 {
                baseline_inventory_sha256,
                backing_identity_sha256,
                boot_session_uuid,
                collector_policy_sha256,
                mountpoint_underlying_sha256,
                prepared_manifest,
            } => {
                require_digest(baseline_inventory_sha256, "baseline inventory")?;
                require_digest(backing_identity_sha256, "backing identity")?;
                require_digest(collector_policy_sha256, "collector policy")?;
                require_digest(mountpoint_underlying_sha256, "mountpoint identity")?;
                require_digest(&prepared_manifest.sha256, "prepared collector manifest")?;
                require_uuid(boot_session_uuid)?;
                if prepared_manifest.dev == 0
                    || prepared_manifest.inode == 0
                    || !(0..1_000_000_000).contains(&prepared_manifest.birthtime_nanoseconds)
                    || prepared_manifest
                        .receipt_root_initial
                        .as_ref()
                        .is_some_and(|binding| {
                            validate_collector_receipt_root_binding_v3(binding).is_err()
                        })
                    || self.phase != Phase::Empty
                {
                    return Err(invalid(
                        "OperationPreparedWithManifestV3 has malformed identity or is not first",
                    ));
                }
                self.baseline_inventory_sha256 = Some(baseline_inventory_sha256.clone());
                self.backing_identity_sha256 = Some(backing_identity_sha256.clone());
                self.boot_session_uuid = Some(boot_session_uuid.clone());
                self.collector_policy_sha256 = Some(collector_policy_sha256.clone());
                self.mountpoint_underlying_sha256 = Some(mountpoint_underlying_sha256.clone());
                self.collector_receipt_root_generation = prepared_manifest
                    .receipt_root_initial
                    .map(|receipt_root_initial| (0, receipt_root_initial));
                self.prepared_manifest = Some(prepared_manifest.clone());
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
                collector,
            } => {
                require_digest(mount_absence_sha256, "mount absence")?;
                self.validate_post_effect_collector_binding(
                    collector.as_ref(),
                    mount_absence_sha256,
                )?;
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
                collector,
            } => {
                require_digest(iomedia_absence_sha256, "IOMedia absence")?;
                self.validate_post_effect_collector_binding(
                    collector.as_ref(),
                    iomedia_absence_sha256,
                )?;
                self.observe_after_callback(*effect_id, EffectKind::Eject, Phase::Ejected)?;
                if let Some(collector) = collector {
                    let latest = TerminalCollectorLineageV3 {
                        collector_receipt_sha256: collector.collector_receipt_sha256.clone(),
                        lifecycle_record_sha256: record.current_record_sha256.to_string(),
                        lifecycle_sequence: record.sequence,
                        observation_sha256: iomedia_absence_sha256.clone(),
                    };
                    validate_terminal_collector_lineage_v3(
                        &latest,
                        "post-eject terminal collector lineage",
                    )?;
                    self.latest_terminal_zero_kind_v3 = Some(TerminalLatestZeroKindV3::PostEject);
                    self.latest_terminal_zero_lineage_v3 = Some(latest);
                }
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
                self.epoch_snapshot_expected_absence_sha256 = None;
                self.epoch_snapshot_was_zero = false;
                self.first_terminal_collector_lineage_v3 = None;
                self.latest_terminal_zero_kind_v3 = None;
                self.latest_terminal_zero_lineage_v3 = None;
                self.restart_epoch_boot_session_uuid = Some(boot_session_uuid.clone());
                self.restart_epoch_collector_policy_sha256 = Some(collector_policy_sha256.clone());
                self.restart_epoch_nonces
                    .insert(restart_epoch_nonce.clone());
                self.restart_epoch_nonce = Some(restart_epoch_nonce.clone());
                self.restart_monotonic_nanoseconds = *monotonic_nanoseconds;
                self.fresh_absence_sha256 = None;
                self.fresh_absence_record_sha256 = None;
                self.fresh_absence_record_sequence = None;
                self.fresh_absence_terminal_binding_v3 = None;
                self.restart_started_lifecycle_record_sha256 =
                    Some(record.current_record_sha256.to_string());
                self.restart_started_lifecycle_sequence = Some(record.sequence);
            }
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot } => {
                self.apply_reconciliation_snapshot(snapshot, record)?;
            }
            DisposableLifecycleEventV2::FreshAbsenceObserved { observation } => {
                let next_root = self.validate_fresh_absence(observation, record)?;
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
                self.fresh_absence_record_sha256 = Some(record.current_record_sha256.to_string());
                self.fresh_absence_record_sequence = Some(record.sequence);
                self.fresh_absence_terminal_binding_v3 = observation.terminal_binding_v3.clone();
                self.collector_receipt_root_generation = next_root;
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
                closure_v3,
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
                    TerminalDispositionV2::Completed if self.mode == Mode::RestartReconcileOnly => {
                        let eligible_phase = if self.epoch_snapshot_was_zero {
                            self.phase == Phase::Prepared
                        } else {
                            self.phase == Phase::Ejected
                        };
                        if self.prepared_manifest.is_none()
                            || !self.restart_epoch_open
                            || !self.epoch_snapshot_seen
                            || !eligible_phase
                        {
                            return Err(invalid(
                                "restart completion requires one prepared-manifest exact namespace-absence lineage",
                            ));
                        }
                        let closure = closure_v3.as_ref().ok_or_else(|| {
                            invalid(
                                "prepared-manifest restart completion omitted its exact terminal closure",
                            )
                        })?;
                        self.validate_terminal_namespace_closure_v3(
                            closure,
                            fresh_absence_sha256,
                            record,
                        )?;
                    }
                    TerminalDispositionV2::Completed => {
                        if self.phase != Phase::Ejected || closure_v3.is_some() {
                            return Err(invalid(
                                "forward completed terminal requires the uninterrupted full eject flow without restart closure data",
                            ));
                        }
                    }
                    TerminalDispositionV2::Aborted
                        if self.mode != Mode::RestartReconcileOnly
                            || !self.restart_epoch_open
                            || !self.epoch_snapshot_seen
                            || (self.epoch_snapshot_was_zero && self.phase != Phase::Prepared)
                            || (!self.epoch_snapshot_was_zero && self.phase != Phase::Ejected)
                            || closure_v3.is_some() =>
                    {
                        return Err(invalid(
                            "aborted terminal requires a fresh restart reconciliation closure",
                        ));
                    }
                    TerminalDispositionV2::Aborted => {}
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
        record: ReducerRecordContext<'_>,
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
        validate_collector_receipt_file_binding_v3(
            snapshot.collector_receipt_file.as_ref(),
            &snapshot.collector_receipt_sha256,
            self.collector_receipt_root_generation.is_some(),
        )?;
        let next_root =
            self.next_collector_receipt_root_generation(snapshot.collector_receipt_file.as_ref())?;
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
        match (
            snapshot.match_result,
            snapshot
                .current_expected_absence_inventory_sha256
                .as_deref(),
        ) {
            (ReconciliationMatchV2::Zero, Some(expected))
                if expected == snapshot.iomedia_evidence_sha256 =>
            {
                require_digest(expected, "Zero expected-absence inventory")?;
            }
            (ReconciliationMatchV2::Unique { .. }, Some(expected))
                if expected != snapshot.iomedia_evidence_sha256 =>
            {
                require_digest(expected, "Unique expected-absence inventory")?;
            }
            (ReconciliationMatchV2::Ambiguous { .. }, None) => {}
            _ => {
                return Err(invalid(
                    "reconciliation match differs from its current-boot expected absence",
                ));
            }
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
        let snapshot_sha256 = reconciliation_snapshot_sha256(snapshot)?;
        let first_lineage = TerminalCollectorLineageV3 {
            collector_receipt_sha256: snapshot.collector_receipt_sha256.clone(),
            lifecycle_record_sha256: record.current_record_sha256.to_string(),
            lifecycle_sequence: record.sequence,
            observation_sha256: snapshot_sha256.clone(),
        };
        validate_terminal_collector_lineage_v3(&first_lineage, "first terminal collector lineage")?;
        self.epoch_snapshot_seen = true;
        self.epoch_snapshot_sha256 = Some(snapshot_sha256);
        self.epoch_snapshot_expected_absence_sha256 =
            snapshot.current_expected_absence_inventory_sha256.clone();
        self.epoch_snapshot_was_zero = matches!(snapshot.match_result, ReconciliationMatchV2::Zero);
        self.reconciliation_receipts
            .insert(snapshot.collector_receipt_sha256.clone());
        self.collector_receipt_root_generation = next_root;
        self.first_terminal_collector_lineage_v3 = Some(first_lineage.clone());
        match snapshot.match_result {
            ReconciliationMatchV2::Zero => {
                self.latest_terminal_zero_kind_v3 = Some(TerminalLatestZeroKindV3::FirstSnapshot);
                self.latest_terminal_zero_lineage_v3 = Some(first_lineage);
                self.phase = Phase::Prepared;
            }
            ReconciliationMatchV2::Unique { mounted } => {
                self.latest_terminal_zero_kind_v3 = None;
                self.latest_terminal_zero_lineage_v3 = None;
                self.phase = if mounted {
                    Phase::Mounted
                } else {
                    Phase::Attached
                };
            }
            ReconciliationMatchV2::Ambiguous { matching_objects } => {
                self.latest_terminal_zero_kind_v3 = None;
                self.latest_terminal_zero_lineage_v3 = None;
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
        record: ReducerRecordContext<'_>,
    ) -> Result<
        Option<(u32, crate::mac_iomedia_identity::FilesystemObjectBindingV3)>,
        LifecycleErrorV2,
    > {
        validate_fresh_absence_shape(observation)?;
        validate_collector_receipt_file_binding_v3(
            observation.collector_receipt_file.as_ref(),
            &observation.collector_receipt_sha256,
            self.collector_receipt_root_generation.is_some(),
        )?;
        let next_root = self
            .next_collector_receipt_root_generation(observation.collector_receipt_file.as_ref())?;
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
                    || self.epoch_snapshot_expected_absence_sha256.as_ref()
                        != observation
                            .current_expected_absence_inventory_sha256
                            .as_ref()
                    || observation
                        .current_expected_absence_inventory_sha256
                        .as_ref()
                        != Some(&observation.iomedia_evidence_sha256)
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
                    || observation
                        .current_expected_absence_inventory_sha256
                        .is_some()
                    || observation.restart_epoch_nonce.is_some()
                    || observation.reconciliation_snapshot_sha256.is_some()))
        {
            return Err(invalid(
                "fresh absence differs from prepared operation bindings",
            ));
        }
        if let Some(binding) = observation.terminal_binding_v3.as_ref() {
            self.validate_terminal_fresh_absence_binding_v3(binding, observation, record)?;
        }
        Ok(next_root)
    }

    fn validate_terminal_fresh_absence_binding_v3(
        &self,
        binding: &TerminalFreshAbsenceBindingV3,
        observation: &FreshAbsenceObservationV2,
        record: ReducerRecordContext<'_>,
    ) -> Result<(), LifecycleErrorV2> {
        validate_terminal_fresh_absence_binding_shape_v3(binding)?;
        let prepared_manifest = self.prepared_manifest.as_ref().ok_or_else(|| {
            invalid("terminal FreshAbsence binding requires a prepared-manifest lifecycle")
        })?;
        let first = self
            .first_terminal_collector_lineage_v3
            .as_ref()
            .ok_or_else(|| invalid("terminal FreshAbsence binding lost its first snapshot"))?;
        let latest = self
            .latest_terminal_zero_lineage_v3
            .as_ref()
            .ok_or_else(|| invalid("terminal FreshAbsence binding has no latest zero state"))?;
        let latest_kind = self.latest_terminal_zero_kind_v3.ok_or_else(|| {
            invalid("terminal FreshAbsence binding has no latest zero state kind")
        })?;
        let restart_started_sha256 = self
            .restart_started_lifecycle_record_sha256
            .as_deref()
            .ok_or_else(|| invalid("terminal FreshAbsence binding lost restart-start record"))?;
        let restart_started_sequence = self
            .restart_started_lifecycle_sequence
            .ok_or_else(|| invalid("terminal FreshAbsence binding lost restart-start sequence"))?;
        let fresh_receipt_file = observation.collector_receipt_file.as_ref().ok_or_else(|| {
            invalid("terminal FreshAbsence binding requires its exact retained receipt inode")
        })?;
        let backing_prepared_sha256 = match &binding.backing_absence {
            TerminalBackingAbsenceEvidenceV3::LiveUnlinked(evidence) => {
                &evidence.prepared_backing_sha256
            }
            TerminalBackingAbsenceEvidenceV3::RecoveredPathAbsent(evidence) => {
                &evidence.prepared_backing_sha256
            }
        };
        if self.mode != Mode::RestartReconcileOnly
            || !self.restart_epoch_open
            || binding.operation_nonce != self.operation_nonce
            || self.restart_epoch_boot_session_uuid.as_ref() != Some(&binding.boot_session_uuid)
            || self.restart_epoch_nonce.as_ref() != Some(&binding.restart_epoch_nonce)
            || self.restart_epoch_collector_policy_sha256.as_ref()
                != Some(&binding.collector_policy_sha256)
            || backing_prepared_sha256 != &binding.prepared_backing_exact_sha256
            || binding.restart.prepared_manifest_sha256 != prepared_manifest.sha256
            || binding.restart.restart_started_lifecycle_record_sha256 != restart_started_sha256
            || binding.restart.restart_started_lifecycle_sequence != restart_started_sequence
            || &binding.first != first
            || binding.latest_zero_kind != latest_kind
            || &binding.latest != latest
            || binding.restart.restart_started_lifecycle_sequence
                >= binding.first.lifecycle_sequence
            || binding.first.lifecycle_sequence > binding.latest.lifecycle_sequence
            || binding.latest.lifecycle_sequence >= record.sequence
            || binding.fresh_collector_receipt_sha256 != observation.collector_receipt_sha256
            || binding.fresh_receipt_root_generation != fresh_receipt_file.root_generation_ordinal
            || binding.artifact_evidence_sha256 != observation.artifact_evidence_sha256
            || binding.fresh_iomedia_evidence_sha256 != observation.iomedia_evidence_sha256
            || binding.fresh_mount_evidence_sha256 != observation.mount_evidence_sha256
        {
            return Err(invalid(
                "terminal FreshAbsence binding differs from its exact retained restart lineage",
            ));
        }
        Ok(())
    }

    fn validate_terminal_namespace_closure_v3(
        &self,
        closure: &TerminalNamespaceClosureV3,
        fresh_absence_sha256: &str,
        record: ReducerRecordContext<'_>,
    ) -> Result<(), LifecycleErrorV2> {
        validate_terminal_namespace_closure_shape_v3(closure)?;
        let binding = self
            .fresh_absence_terminal_binding_v3
            .as_ref()
            .ok_or_else(|| invalid("terminal closure has no retained FreshAbsence binding"))?;
        let binding_sha256 = terminal_fresh_absence_binding_sha256(binding)?;
        if closure.fresh_absence_sha256 != fresh_absence_sha256
            || closure.fresh_absence_collector_receipt_sha256
                != binding.fresh_collector_receipt_sha256
            || closure.fresh_absence_lifecycle_record_sha256
                != self
                    .fresh_absence_record_sha256
                    .as_deref()
                    .unwrap_or_default()
            || Some(closure.fresh_absence_lifecycle_sequence) != self.fresh_absence_record_sequence
            || closure.terminal_binding_sha256 != binding_sha256
            || record.previous_record_sha256
                != Some(closure.fresh_absence_lifecycle_record_sha256.as_str())
            || closure.fresh_absence_lifecycle_sequence.checked_add(1) != Some(record.sequence)
        {
            return Err(invalid(
                "terminal namespace closure differs from its exact durable FreshAbsence predecessor",
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

    fn validate_post_effect_collector_binding(
        &mut self,
        binding: Option<&PostEffectCollectorBindingV3>,
        observation_sha256: &str,
    ) -> Result<(), LifecycleErrorV2> {
        match (self.mode, binding) {
            (Mode::RestartReconcileOnly, Some(binding)) => {
                require_uuid(&binding.boot_session_uuid)?;
                require_nonce(&binding.restart_epoch_nonce)?;
                require_nonce(&binding.operation_nonce)?;
                validate_collector_receipt_file_binding_v3(
                    binding.collector_receipt_file.as_ref(),
                    &binding.collector_receipt_sha256,
                    self.collector_receipt_root_generation.is_some(),
                )?;
                let next_root = self.next_collector_receipt_root_generation(
                    binding.collector_receipt_file.as_ref(),
                )?;
                for (value, label) in [
                    (
                        &binding.collector_receipt_sha256,
                        "post-effect collector receipt",
                    ),
                    (
                        &binding.first_reconciliation_snapshot_sha256,
                        "post-effect first reconciliation snapshot",
                    ),
                    (&binding.observation_sha256, "post-effect observation"),
                ] {
                    require_digest(value, label)?;
                }
                if binding.operation_nonce != self.operation_nonce
                    || self.restart_epoch_boot_session_uuid.as_ref()
                        != Some(&binding.boot_session_uuid)
                    || self.restart_epoch_nonce.as_ref() != Some(&binding.restart_epoch_nonce)
                    || self.epoch_snapshot_sha256.as_ref()
                        != Some(&binding.first_reconciliation_snapshot_sha256)
                    || binding.observation_sha256 != observation_sha256
                    || self
                        .reconciliation_receipts
                        .contains(&binding.collector_receipt_sha256)
                {
                    return Err(invalid(
                        "post-effect collector binding differs from the active restart lineage",
                    ));
                }
                self.reconciliation_receipts
                    .insert(binding.collector_receipt_sha256.clone());
                self.collector_receipt_root_generation = next_root;
                Ok(())
            }
            (Mode::RestartReconcileOnly, None)
                if self.collector_receipt_root_generation.is_some() =>
            {
                Err(invalid(
                    "V3 restart post-effect observation requires retained collector evidence",
                ))
            }
            (Mode::RestartReconcileOnly, None) => Ok(()),
            (Mode::FreshProcess | Mode::Replay, None) => Ok(()),
            (Mode::FreshProcess | Mode::Replay, Some(_)) => Err(invalid(
                "forward or historical replay cannot self-report restart collector evidence",
            )),
        }
    }

    fn next_collector_receipt_root_generation(
        &self,
        binding: Option<&CollectorReceiptFileBindingV3>,
    ) -> Result<
        Option<(u32, crate::mac_iomedia_identity::FilesystemObjectBindingV3)>,
        LifecycleErrorV2,
    > {
        let Some(binding) = binding else {
            return if self.collector_receipt_root_generation.is_some() {
                Err(invalid(
                    "prepared-manifest V3 lifecycle omitted its next receipt-root generation",
                ))
            } else {
                Ok(None)
            };
        };
        let (prior_ordinal, prior_root) = self
            .collector_receipt_root_generation
            .as_ref()
            .ok_or_else(|| {
                invalid("collector receipt-root generation has no durable initial binding")
            })?;
        if prior_ordinal.checked_add(1) != Some(binding.root_generation_ordinal)
            || !same_collector_receipt_root_object_v3(prior_root, &binding.root_after)
            || prior_root.nlink.checked_add(1) != Some(binding.root_after.nlink)
        {
            return Err(invalid(
                "collector receipt-root full binding is not the exact next durable generation",
            ));
        }
        Ok(Some((binding.root_generation_ordinal, binding.root_after)))
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

    #[cfg(test)]
    pub fn append_with<F>(
        &mut self,
        event: DisposableLifecycleEventV2,
        persist: F,
    ) -> Result<String, LifecycleErrorV2>
    where
        F: FnOnce(&DisposableLifecycleRecordV2, &[u8]) -> std::io::Result<()>,
    {
        self.append_with_inner(event, persist)
    }

    pub(crate) fn append_with_sealed<F>(
        &mut self,
        _seal: crate::mac_disposable_lifecycle_store::LifecycleStoreAppendSealV3,
        event: DisposableLifecycleEventV2,
        persist: F,
    ) -> Result<String, LifecycleErrorV2>
    where
        F: FnOnce(&DisposableLifecycleRecordV2, &[u8]) -> std::io::Result<()>,
    {
        self.append_with_inner(event, persist)
    }

    fn append_with_inner<F>(
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
        let digest = sha256(&bytes);
        let mut next_reducer = self.reducer.clone();
        next_reducer.apply(
            &record.event,
            ReducerRecordContext {
                current_record_sha256: &digest,
                previous_record_sha256: record.previous_record_sha256.as_deref(),
                sequence: record.sequence,
            },
        )?;
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

    /// Return the exact first-snapshot bindings only when this replayed
    /// restart lifecycle is ready to collect FreshAbsence.  Zero may proceed
    /// directly from Prepared; Unique must first complete its exact cleanup
    /// sequence through Ejected.  Ambiguous snapshots never carry an exact
    /// expected-absence inventory.
    pub(crate) fn restart_fresh_absence_binding(&self) -> Result<(&str, &str), LifecycleErrorV2> {
        let reducer = &self.reducer;
        let eligible_phase = if reducer.epoch_snapshot_was_zero {
            reducer.phase == Phase::Prepared
        } else {
            reducer.phase == Phase::Ejected
        };
        if self.persistence_uncertain
            || reducer.mode != Mode::RestartReconcileOnly
            || !reducer.restart_epoch_open
            || !reducer.epoch_snapshot_seen
            || !eligible_phase
            || reducer.pending.is_some()
            || reducer.callback_seen
            || reducer.callback_succeeded
            || reducer.manual
            || reducer.quarantined
            || reducer.terminal.is_some()
        {
            return Err(invalid(
                "restart lifecycle is not ready for exact FreshAbsence collection",
            ));
        }
        let snapshot = reducer
            .epoch_snapshot_sha256
            .as_deref()
            .ok_or_else(|| invalid("restart lifecycle lost its first snapshot digest"))?;
        let expected = reducer
            .epoch_snapshot_expected_absence_sha256
            .as_deref()
            .ok_or_else(|| invalid("restart snapshot has no exact expected absence"))?;
        Ok((snapshot, expected))
    }

    pub fn operation_nonce(&self) -> &str {
        &self.operation_nonce
    }

    pub fn record_count(&self) -> usize {
        self.records
    }

    pub fn terminal_record_sha256(&self) -> Option<&str> {
        self.previous_record_sha256.as_deref()
    }

    pub fn process_mode(&self) -> LifecycleProcessModeV2 {
        match self.reducer.mode {
            Mode::FreshProcess => LifecycleProcessModeV2::FreshProcess,
            Mode::Replay => LifecycleProcessModeV2::Replay,
            Mode::RestartReconcileOnly => LifecycleProcessModeV2::RestartReconcileOnly,
        }
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
        prepared_manifest: journal.reducer.prepared_manifest,
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

pub(crate) fn terminal_fresh_absence_binding_sha256(
    binding: &TerminalFreshAbsenceBindingV3,
) -> Result<String, LifecycleErrorV2> {
    canonical_json(binding)
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

/// Replay the exact lifecycle chain and return its closed-world collector
/// receipt inode roster. Legacy records legitimately contribute no entry;
/// prepared-manifest V3 records are rejected by the reducer if any reference
/// is absent.
pub(crate) fn collector_receipt_file_roster_v3(
    records: &[Vec<u8>],
) -> Result<Vec<CollectorReceiptFileBindingV3>, LifecycleErrorV2> {
    let _ = replay(records)?;
    let mut result = Vec::new();
    for bytes in records {
        let record: DisposableLifecycleRecordV2 = serde_json::from_slice(bytes)
            .map_err(|error| LifecycleErrorV2::Serialization(error.to_string()))?;
        let binding = match &record.event {
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved { snapshot } => {
                snapshot.collector_receipt_file.as_ref()
            }
            DisposableLifecycleEventV2::FreshAbsenceObserved { observation } => {
                observation.collector_receipt_file.as_ref()
            }
            DisposableLifecycleEventV2::UnmountObserved { collector, .. }
            | DisposableLifecycleEventV2::EjectObserved { collector, .. } => collector
                .as_ref()
                .and_then(PostEffectCollectorBindingV3::collector_receipt_file),
            _ => None,
        };
        if let Some(binding) = binding {
            result.push(binding.clone());
        }
    }
    if result.iter().enumerate().any(|(index, binding)| {
        usize::try_from(binding.root_generation_ordinal).ok() != Some(index + 1)
    }) || result.iter().enumerate().any(|(index, binding)| {
        result.iter().skip(index + 1).any(|other| {
            binding.final_basename == other.final_basename
                || (binding.dev, binding.inode) == (other.dev, other.inode)
        })
    }) || result.windows(2).any(|pair| {
        !same_collector_receipt_root_object_v3(&pair[0].root_after, &pair[1].root_after)
            || pair[0].root_after.nlink.checked_add(1) != Some(pair[1].root_after.nlink)
    }) {
        return Err(invalid(
            "lifecycle collector receipt references are not one ordered exact root generation chain",
        ));
    }
    Ok(result)
}

/// Replay both lifecycle chains and prove that `after` contains either the
/// exact same collector-receipt lineage or exactly one appended generation.
///
/// The returned binding is the sole appended receipt, when one exists. This
/// deliberately compares the complete retained bindings rather than only
/// their digest or generation ordinal.
pub(crate) fn exact_collector_receipt_append_v3(
    before: &[Vec<u8>],
    after: &[Vec<u8>],
) -> Result<Option<CollectorReceiptFileBindingV3>, LifecycleErrorV2> {
    // Fresh S1 admission has no predecessor record yet.  Model that exact
    // pre-operation state as an empty receipt lineage while still requiring
    // `after` to be a fully replayable lifecycle.  This does not admit a
    // collector as record one: replaying `after` enforces the reducer's
    // OperationPrepared-first transition.
    let before_roster = if before.is_empty() {
        Vec::new()
    } else {
        collector_receipt_file_roster_v3(before)?
    };
    let after_roster = collector_receipt_file_roster_v3(after)?;
    let maximum_after_len = before_roster
        .len()
        .checked_add(1)
        .ok_or_else(|| invalid("collector receipt lineage length overflowed"))?;
    if after_roster.len() < before_roster.len()
        || after_roster.len() > maximum_after_len
        || !after_roster.starts_with(&before_roster)
    {
        return Err(invalid(
            "collector receipt lineage is not an exact zero-or-one append",
        ));
    }
    Ok(after_roster.get(before_roster.len()).cloned())
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
        let current_record_sha256 = sha256(bytes);
        reducer.apply(
            &record.event,
            ReducerRecordContext {
                current_record_sha256: &current_record_sha256,
                previous_record_sha256: record.previous_record_sha256.as_deref(),
                sequence: record.sequence,
            },
        )?;
        previous = Some(current_record_sha256);
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

fn validate_terminal_collector_lineage_v3(
    lineage: &TerminalCollectorLineageV3,
    label: &str,
) -> Result<(), LifecycleErrorV2> {
    for (value, suffix) in [
        (&lineage.collector_receipt_sha256, "collector receipt"),
        (&lineage.lifecycle_record_sha256, "lifecycle record"),
        (&lineage.observation_sha256, "observation"),
    ] {
        require_digest(value, &format!("{label} {suffix}"))?;
    }
    if lineage.lifecycle_sequence == 0 {
        return Err(invalid(format!("{label} sequence is zero")));
    }
    Ok(())
}

fn validate_terminal_restart_admission_lineage_v3(
    lineage: &TerminalRestartAdmissionLineageV3,
) -> Result<(), LifecycleErrorV2> {
    for (value, label) in [
        (
            &lineage.prepared_manifest_sha256,
            "terminal prepared manifest",
        ),
        (
            &lineage.prepared_profile_sha256,
            "terminal prepared profile",
        ),
        (&lineage.process_epoch_sha256, "terminal process epoch"),
        (
            &lineage.restart_admission_sha256,
            "terminal restart admission",
        ),
        (
            &lineage.restart_started_lifecycle_record_sha256,
            "terminal restart-start lifecycle record",
        ),
    ] {
        require_digest(value, label)?;
    }
    if lineage.restart_started_lifecycle_sequence == 0 {
        return Err(invalid(
            "terminal restart-admission lineage has zero lifecycle sequence",
        ));
    }
    Ok(())
}

fn validate_terminal_artifact_namespace_delta_v3(
    delta: &TerminalArtifactNamespaceDeltaV3,
) -> Result<(), LifecycleErrorV2> {
    if delta.authority.any()
        || delta.backing_basename.is_empty()
        || delta.backing_basename.as_bytes().contains(&0)
        || delta.backing_basename.as_bytes().contains(&b'/')
        || delta.before_entries != [delta.backing_basename.clone()]
        || !delta.after_entries.is_empty()
        || validate_collector_receipt_root_binding_v3(&delta.before_root).is_err()
        || validate_collector_receipt_root_binding_v3(&delta.after_root).is_err()
        || !same_collector_receipt_root_object_v3(&delta.before_root, &delta.after_root)
    {
        return Err(invalid(
            "terminal artifact namespace delta is malformed, non-exact, or grants authority",
        ));
    }
    let before_entries = u64::try_from(delta.before_entries.len())
        .map_err(|_| invalid("terminal artifact roster length overflowed"))?;
    let after_entries = u64::try_from(delta.after_entries.len())
        .map_err(|_| invalid("terminal artifact roster length overflowed"))?;
    if delta.before_root.nlink.checked_sub(before_entries)
        != delta.after_root.nlink.checked_sub(after_entries)
    {
        return Err(invalid(
            "terminal artifact root link count does not match its exact roster delta",
        ));
    }
    Ok(())
}

fn validate_terminal_backing_absence_evidence_v3(
    evidence: &TerminalBackingAbsenceEvidenceV3,
) -> Result<(), LifecycleErrorV2> {
    match evidence {
        TerminalBackingAbsenceEvidenceV3::LiveUnlinked(binding) => {
            crate::mac_iomedia_identity::validate_unlinked_backing_binding_v3(binding).map_err(
                |error| invalid(format!("terminal live backing absence is invalid: {error}")),
            )
        }
        TerminalBackingAbsenceEvidenceV3::RecoveredPathAbsent(binding) => {
            crate::mac_iomedia_identity::validate_backing_path_absence_binding_v3(binding).map_err(
                |error| {
                    invalid(format!(
                        "terminal recovered backing absence is invalid: {error}"
                    ))
                },
            )
        }
    }
}

fn validate_terminal_fresh_absence_binding_shape_v3(
    binding: &TerminalFreshAbsenceBindingV3,
) -> Result<(), LifecycleErrorV2> {
    if binding.authority.any() {
        return Err(invalid(
            "terminal FreshAbsence binding grants disposable authority",
        ));
    }
    require_nonce(&binding.operation_nonce)?;
    require_uuid(&binding.boot_session_uuid)?;
    require_nonce(&binding.restart_epoch_nonce)?;
    for (value, label) in [
        (
            &binding.artifact_evidence_sha256,
            "terminal artifact evidence",
        ),
        (
            &binding.collector_policy_sha256,
            "terminal collector policy",
        ),
        (
            &binding.fresh_collector_receipt_sha256,
            "terminal FreshAbsence collector receipt",
        ),
        (
            &binding.fresh_iomedia_evidence_sha256,
            "terminal FreshAbsence IOMedia evidence",
        ),
        (
            &binding.fresh_mount_evidence_sha256,
            "terminal FreshAbsence mount evidence",
        ),
        (
            &binding.prepared_backing_exact_sha256,
            "terminal exact prepared backing",
        ),
    ] {
        require_digest(value, label)?;
    }
    if binding.fresh_receipt_root_generation == 0 {
        return Err(invalid(
            "terminal FreshAbsence receipt-root generation is zero",
        ));
    }
    validate_terminal_restart_admission_lineage_v3(&binding.restart)?;
    validate_terminal_collector_lineage_v3(&binding.first, "terminal first lineage")?;
    validate_terminal_collector_lineage_v3(&binding.latest, "terminal latest-zero lineage")?;
    validate_terminal_backing_absence_evidence_v3(&binding.backing_absence)?;
    validate_terminal_artifact_namespace_delta_v3(&binding.artifact_namespace_delta)?;

    let (prepared_backing_sha256, canonical_path) = match &binding.backing_absence {
        TerminalBackingAbsenceEvidenceV3::LiveUnlinked(evidence) => (
            &evidence.prepared_backing_sha256,
            evidence.canonical_path.as_str(),
        ),
        TerminalBackingAbsenceEvidenceV3::RecoveredPathAbsent(evidence) => (
            &evidence.prepared_backing_sha256,
            evidence.canonical_path.as_str(),
        ),
    };
    let backing_basename = std::path::Path::new(canonical_path)
        .file_name()
        .and_then(|name| name.to_str());
    let latest_shape_valid = match binding.latest_zero_kind {
        TerminalLatestZeroKindV3::FirstSnapshot => binding.latest == binding.first,
        TerminalLatestZeroKindV3::PostEject => {
            binding.latest.lifecycle_sequence > binding.first.lifecycle_sequence
                && binding.latest.collector_receipt_sha256 != binding.first.collector_receipt_sha256
        }
    };
    if prepared_backing_sha256 != &binding.prepared_backing_exact_sha256
        || backing_basename != Some(binding.artifact_namespace_delta.backing_basename.as_str())
        || binding.restart.restart_started_lifecycle_sequence >= binding.first.lifecycle_sequence
        || !latest_shape_valid
        || binding.fresh_collector_receipt_sha256 == binding.latest.collector_receipt_sha256
    {
        return Err(invalid(
            "terminal FreshAbsence binding has inconsistent predecessor lineage",
        ));
    }
    Ok(())
}

fn validate_terminal_namespace_closure_shape_v3(
    closure: &TerminalNamespaceClosureV3,
) -> Result<(), LifecycleErrorV2> {
    if closure.authority.any() {
        return Err(invalid("terminal namespace closure grants authority"));
    }
    for (value, label) in [
        (
            &closure.fresh_absence_collector_receipt_sha256,
            "terminal closure FreshAbsence collector receipt",
        ),
        (
            &closure.fresh_absence_lifecycle_record_sha256,
            "terminal closure FreshAbsence lifecycle record",
        ),
        (
            &closure.fresh_absence_sha256,
            "terminal closure FreshAbsence observation",
        ),
        (&closure.terminal_binding_sha256, "terminal closure binding"),
    ] {
        require_digest(value, label)?;
    }
    if closure.fresh_absence_lifecycle_sequence == 0 {
        return Err(invalid(
            "terminal namespace closure has zero FreshAbsence lifecycle sequence",
        ));
    }
    Ok(())
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
    if let Some(digest) = &observation.current_expected_absence_inventory_sha256 {
        require_digest(digest, "absence current-boot expected inventory")?;
    }
    validate_collector_receipt_file_binding_v3(
        observation.collector_receipt_file.as_ref(),
        &observation.collector_receipt_sha256,
        false,
    )?;
    if let Some(binding) = observation.terminal_binding_v3.as_ref() {
        validate_terminal_fresh_absence_binding_shape_v3(binding)?;
    }
    if observation.restart_epoch_nonce.is_some()
        != observation.reconciliation_snapshot_sha256.is_some()
        || observation.restart_epoch_nonce.is_some()
            != observation
                .current_expected_absence_inventory_sha256
                .is_some()
    {
        return Err(invalid(
            "fresh absence restart epoch, snapshot, and current-boot expected bindings must be present together",
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
        || (observation
            .current_expected_absence_inventory_sha256
            .as_ref()
            .is_some_and(|expected| expected != &observation.iomedia_evidence_sha256))
        || (observation
            .current_expected_absence_inventory_sha256
            .is_none()
            && observation.baseline_inventory_sha256 != observation.post_inventory_sha256)
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

pub(crate) fn validate_collector_receipt_file_binding_v3(
    binding: Option<&CollectorReceiptFileBindingV3>,
    expected_sha256: &str,
    required: bool,
) -> Result<(), LifecycleErrorV2> {
    require_digest(expected_sha256, "collector receipt")?;
    let Some(binding) = binding else {
        return if required {
            Err(invalid(
                "prepared-manifest V3 collector observation omits its exact receipt inode",
            ))
        } else {
            Ok(())
        };
    };
    require_digest(&binding.canonical_sha256, "collector receipt file")?;
    if binding.canonical_sha256 != expected_sha256
        || binding.final_basename != format!("collector-{expected_sha256}.json")
        || binding.dev == 0
        || binding.inode == 0
        || binding.mode & libc::S_IFMT as u32 != libc::S_IFREG as u32
        || binding.mode & 0o7777 != 0o600
        || binding.flags != 0
        || binding.nlink != 1
        || binding.root_generation_ordinal == 0
        || validate_collector_receipt_root_binding_v3(&binding.root_after).is_err()
        || binding.root_after.dev != binding.dev
        || binding.root_after.uid != binding.uid
        || binding.root_after.gid != binding.gid
        || binding.size == 0
    {
        return Err(invalid(
            "collector receipt file digest, basename, inode, type, or immutable metadata is invalid",
        ));
    }
    Ok(())
}

fn validate_collector_receipt_root_binding_v3(
    binding: &crate::mac_iomedia_identity::FilesystemObjectBindingV3,
) -> Result<(), LifecycleErrorV2> {
    if binding.dev == 0
        || binding.inode == 0
        || binding.mode & libc::S_IFMT as u32 != libc::S_IFDIR as u32
        || binding.mode & 0o7777 != 0o700
        || binding.flags != 0
        || binding.nlink == 0
    {
        return Err(invalid(
            "collector receipt root full binding is not a private stable directory",
        ));
    }
    Ok(())
}

pub(crate) fn same_collector_receipt_root_object_v3(
    before: &crate::mac_iomedia_identity::FilesystemObjectBindingV3,
    after: &crate::mac_iomedia_identity::FilesystemObjectBindingV3,
) -> bool {
    before.birthtime_nanoseconds == after.birthtime_nanoseconds
        && before.birthtime_seconds == after.birthtime_seconds
        && before.dev == after.dev
        && before.flags == after.flags
        && before.generation == after.generation
        && before.gid == after.gid
        && before.inode == after.inode
        && before.mode == after.mode
        && before.uid == after.uid
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
