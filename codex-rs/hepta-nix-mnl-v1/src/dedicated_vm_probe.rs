use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;

use crate::ClosedAuthorityV1;
use crate::ClosedRunPlanDispositionV1;
use crate::InvalidatedNixSandboxContractInspectionV1;
use crate::NIX_CLOSED_RUN_PLAN_SCHEMA;
use crate::NIX_SANDBOX_REQUALIFICATION_SCHEMA;
use crate::NIX_SANDBOX_REQUALIFICATION_SCHEMA_VERSION;
use crate::NixMnlError;
use crate::invalid;

pub const NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_SCHEMA: &str =
    "hepta_nix_mnl_dedicated_vm_kvm_development_probe_v1";
pub const NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_SCHEMA_VERSION: u32 = 1;
pub const MAX_NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_BYTES: usize = 64 * 1024;

const MAX_NIX_DEDICATED_VM_ARTIFACT_BYTES: u64 = 256 * 1024 * 1024;
const MAX_NIX_DEDICATED_VM_QEMU_VERSION_BYTES: usize = 256;
const MAX_NIX_DEDICATED_VM_RAW_CAPTURE_BYTES: u64 = 1024 * 1024 * 1024;

const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const POSITIVE_SERIAL_SHA256: &str =
    "98eee34684137e9a06b1de0fdfde566ad803d93f49fe0e6604f6bdcf789a0822";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmProbeDispositionV1 {
    DevelopmentProbeOnlyNoQualification,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmBackendV1 {
    QemuSystemX8664PcKvm,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmAccelerationV1 {
    HardwareKvmOnlyNoTcgFallback,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmObservationOriginV1 {
    CallerSuppliedCanonicalDevelopmentMaterialNoTrustedCollector,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmForbiddenHostFallbackV1 {
    HostInitialNamespaceCapSysAdmin,
    HostPrivilegedContainerWrapper,
    HostSoftwareEmulationTcg,
    HostUnconfinedLsmFallback,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmGuestIsolationAxisV1 {
    Ipc,
    Mount,
    Network,
    Pid,
    User,
    Uts,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmGuestAxisDispositionV1 {
    GuestLocalAxesUnexercisedNotHostFallback,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmArtifactLocatorPolicyV1 {
    OpaqueRoleOnlyNoWirePaths,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmArtifactRoleV1 {
    BootAssemblySource,
    LinkerScript,
    ProbeHarness,
    PositiveBootSectorImage,
    NegativeInvalidSignatureBootSectorImage,
    QemuSystemX8664Backend,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmBootMediaFormatV1 {
    Raw,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmBootMediaInterfaceV1 {
    Floppy,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmBootOrderV1 {
    FloppyDriveAOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmDeviceRoleV1 {
    GuestReadOnlyRawFloppyBootMedia,
    GuestIsaSerialConsole,
    GuestIsaDebugExit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NixDedicatedVmObservedDeviceTypeV1 {
    CharacterDevice,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmQemuSandboxV1 {
    pub elevate_privileges_denied: bool,
    pub enabled: bool,
    pub obsolete_denied: bool,
    pub resource_control_denied: bool,
    pub spawn_denied: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmRequestedConfigurationV1 {
    pub boot_drive_count: u32,
    pub boot_media_format: NixDedicatedVmBootMediaFormatV1,
    pub boot_media_interface: NixDedicatedVmBootMediaInterfaceV1,
    pub boot_media_read_only: bool,
    pub boot_order: NixDedicatedVmBootOrderV1,
    pub cpu_model: String,
    pub debug_exit_iobase: u32,
    pub debug_exit_iosize: u32,
    pub display_enabled: bool,
    pub machine: String,
    pub memory_bytes: u64,
    pub monitor_enabled: bool,
    pub network_device_count: u32,
    pub no_reboot: bool,
    pub no_shutdown: bool,
    pub no_user_config: bool,
    pub nodefaults: bool,
    pub qemu_sandbox: NixDedicatedVmQemuSandboxV1,
    pub serial_device_count: u32,
    pub vcpu_count: u32,
    pub writable_block_device_count: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmProbeClaimsV1 {
    pub acceleration: NixDedicatedVmAccelerationV1,
    pub artifact_locator_policy: NixDedicatedVmArtifactLocatorPolicyV1,
    pub backend: NixDedicatedVmBackendV1,
    pub forbidden_host_fallbacks: Vec<NixDedicatedVmForbiddenHostFallbackV1>,
    pub guest_axis_disposition: NixDedicatedVmGuestAxisDispositionV1,
    pub guest_isolation_axes: Vec<NixDedicatedVmGuestIsolationAxisV1>,
    pub requested_configuration: NixDedicatedVmRequestedConfigurationV1,
    pub required_artifacts: Vec<NixDedicatedVmArtifactRoleV1>,
    pub required_devices: Vec<NixDedicatedVmDeviceRoleV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmObservedArtifactV1 {
    pub byte_count: u64,
    pub mode: u32,
    pub post_observation_sha256: String,
    pub pre_observation_sha256: String,
    pub role: NixDedicatedVmArtifactRoleV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmObservedKvmDeviceV1 {
    pub device_type: NixDedicatedVmObservedDeviceTypeV1,
    pub gid: u32,
    pub inode: u64,
    pub mode: u32,
    pub mount_id: u64,
    pub rdev_major: u32,
    pub rdev_minor: u32,
    pub uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmHostObservationV1 {
    pub architecture: String,
    pub boot_id_sha256: String,
    pub cgroup_identity_sha256: String,
    pub cpu_identity_sha256: String,
    pub egid: u32,
    pub euid: u32,
    pub host_identity_sha256: String,
    pub kernel_release_sha256: String,
    pub kvm_api_version: u32,
    pub kvm_device: NixDedicatedVmObservedKvmDeviceV1,
    pub kvm_extension_count: u32,
    pub kvm_extensions_sha256: String,
    pub lsm_identity_sha256: String,
    pub mount_namespace_inode: u64,
    pub pid_namespace_inode: u64,
    pub user_namespace_inode: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmBackendObservationV1 {
    pub argv_count: u32,
    pub argv_sha256: String,
    pub artifact_sha256: String,
    pub environment_count: u32,
    pub environment_sha256: String,
    pub process_cgroup_identity_sha256: String,
    pub process_egid: u32,
    pub process_euid: u32,
    pub process_lsm_identity_sha256: String,
    pub process_mount_namespace_inode: u64,
    pub process_pid_namespace_inode: u64,
    pub process_user_namespace_inode: u64,
    pub version_byte_count: u64,
    pub version_bytes: String,
    pub version_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmDeviceObservationV1 {
    pub boot_drive_count: u32,
    pub boot_media_format: NixDedicatedVmBootMediaFormatV1,
    pub boot_media_interface: NixDedicatedVmBootMediaInterfaceV1,
    pub boot_media_read_only: bool,
    pub boot_order: NixDedicatedVmBootOrderV1,
    pub debug_exit_iobase: u32,
    pub debug_exit_iosize: u32,
    pub network_device_count: u32,
    pub observed_devices: Vec<NixDedicatedVmDeviceRoleV1>,
    pub serial_device_count: u32,
    pub writable_block_device_count: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmNegativeOutcomeV1 {
    pub exit_code: i32,
    pub serial_byte_count: u64,
    pub serial_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmOutcomeObservationV1 {
    pub negative_boot_signature_hex: String,
    pub negative_outcomes: Vec<NixDedicatedVmNegativeOutcomeV1>,
    pub positive_boot_signature_hex: String,
    pub positive_exit_code: i32,
    pub positive_serial_byte_count: u64,
    pub positive_serial_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmRawCaptureObservationV1 {
    pub entry_count: u32,
    pub manifest_byte_count: u64,
    pub manifest_sha256: String,
    pub total_byte_count: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmProbeObservationsV1 {
    pub artifacts: Vec<NixDedicatedVmObservedArtifactV1>,
    pub backend: NixDedicatedVmBackendObservationV1,
    pub devices: NixDedicatedVmDeviceObservationV1,
    pub effective_configuration_observed: bool,
    pub effective_topology_observed: bool,
    pub host: NixDedicatedVmHostObservationV1,
    pub outcomes: NixDedicatedVmOutcomeObservationV1,
    pub raw_capture: NixDedicatedVmRawCaptureObservationV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmProbeParentV1 {
    pub invalidated_sandbox_envelope_byte_count: u64,
    pub invalidated_sandbox_envelope_schema: String,
    pub invalidated_sandbox_envelope_schema_version: u32,
    pub invalidated_sandbox_envelope_sha256: String,
    pub v3_closed_plan_byte_count: u64,
    pub v3_closed_plan_disposition: ClosedRunPlanDispositionV1,
    pub v3_closed_plan_schema: String,
    pub v3_closed_plan_schema_version: u32,
    pub v3_closed_plan_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NixDedicatedVmKvmDevelopmentProbeEnvelopeV1 {
    pub authority: ClosedAuthorityV1,
    pub claims: NixDedicatedVmProbeClaimsV1,
    pub disposition: NixDedicatedVmProbeDispositionV1,
    pub launch_authorized: bool,
    pub observation_origin: NixDedicatedVmObservationOriginV1,
    pub observations: NixDedicatedVmProbeObservationsV1,
    pub parent: NixDedicatedVmProbeParentV1,
    pub pass_authorized: bool,
    pub qualification_observed: bool,
    pub receipt_acceptance_authorized: bool,
    pub replay_publication_authorized: bool,
    pub schema: String,
    pub schema_version: u32,
    pub vm_launch_authorized: bool,
}

/// Opaque inspection of caller-supplied development material.
///
/// The token proves only that the canonical bytes match the fixed negative
/// probe contract and the retained N5c parent. It does not make any reported
/// observation authoritative and cannot be converted into execution power.
#[derive(Debug)]
pub struct InspectedNixDedicatedVmKvmDevelopmentProbeV1 {
    canonical_bytes: Vec<u8>,
    envelope: NixDedicatedVmKvmDevelopmentProbeEnvelopeV1,
    envelope_sha256: String,
}

impl InspectedNixDedicatedVmKvmDevelopmentProbeV1 {
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn envelope(&self) -> &NixDedicatedVmKvmDevelopmentProbeEnvelopeV1 {
        &self.envelope
    }

    pub fn envelope_sha256(&self) -> &str {
        &self.envelope_sha256
    }

    pub const fn authorizes_live(&self) -> bool {
        false
    }

    pub const fn capability_available(&self) -> bool {
        false
    }

    pub const fn effective_configuration_trusted(&self) -> bool {
        false
    }

    pub const fn effective_topology_trusted(&self) -> bool {
        false
    }

    pub const fn launch_grant_available(&self) -> bool {
        false
    }

    pub const fn launch_performed(&self) -> bool {
        false
    }

    pub const fn pass_authorized(&self) -> bool {
        false
    }

    pub const fn qualification_observed(&self) -> bool {
        false
    }

    pub const fn receipt_acceptance_authorized(&self) -> bool {
        false
    }

    pub const fn receipt_accepted(&self) -> bool {
        false
    }

    pub const fn replay_publication_available(&self) -> bool {
        false
    }

    pub const fn replay_publication_authorized(&self) -> bool {
        false
    }

    pub const fn trusted_collector_observed(&self) -> bool {
        false
    }

    pub const fn vm_launch_authorized(&self) -> bool {
        false
    }

    pub const fn vm_qualified(&self) -> bool {
        false
    }
}

pub fn derive_nix_dedicated_vm_kvm_development_probe_envelope(
    parent: &InvalidatedNixSandboxContractInspectionV1,
    observations: NixDedicatedVmProbeObservationsV1,
) -> Result<NixDedicatedVmKvmDevelopmentProbeEnvelopeV1, NixMnlError> {
    validate_observations(&observations)?;
    Ok(NixDedicatedVmKvmDevelopmentProbeEnvelopeV1 {
        authority: ClosedAuthorityV1::exact(),
        claims: exact_claims(),
        disposition: NixDedicatedVmProbeDispositionV1::DevelopmentProbeOnlyNoQualification,
        launch_authorized: false,
        observation_origin:
            NixDedicatedVmObservationOriginV1::CallerSuppliedCanonicalDevelopmentMaterialNoTrustedCollector,
        observations,
        parent: derive_parent(parent)?,
        pass_authorized: false,
        qualification_observed: false,
        receipt_acceptance_authorized: false,
        replay_publication_authorized: false,
        schema: NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_SCHEMA.to_string(),
        schema_version: NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_SCHEMA_VERSION,
        vm_launch_authorized: false,
    })
}

pub fn inspect_canonical_nix_dedicated_vm_kvm_development_probe(
    parent: &InvalidatedNixSandboxContractInspectionV1,
    bytes: &[u8],
) -> Result<InspectedNixDedicatedVmKvmDevelopmentProbeV1, NixMnlError> {
    if bytes.is_empty() || bytes.len() > MAX_NIX_DEDICATED_VM_KVM_DEVELOPMENT_PROBE_BYTES {
        return Err(invalid(
            "Nix dedicated-VM development probe byte length is outside its bound",
        ));
    }
    let envelope: NixDedicatedVmKvmDevelopmentProbeEnvelopeV1 = serde_json::from_slice(bytes)
        .map_err(|error| {
            invalid(format!(
                "Nix dedicated-VM development probe is malformed: {error}"
            ))
        })?;
    let canonical = serde_json::to_vec(&envelope)?;
    if canonical != bytes {
        return Err(invalid(
            "Nix dedicated-VM development probe is not exact canonical JSON",
        ));
    }
    let expected = derive_nix_dedicated_vm_kvm_development_probe_envelope(
        parent,
        envelope.observations.clone(),
    )?;
    if envelope != expected {
        return Err(invalid(
            "Nix dedicated-VM development probe differs from its exact closed model",
        ));
    }
    Ok(InspectedNixDedicatedVmKvmDevelopmentProbeV1 {
        canonical_bytes: canonical,
        envelope,
        envelope_sha256: sha256_hex(bytes),
    })
}

fn exact_claims() -> NixDedicatedVmProbeClaimsV1 {
    NixDedicatedVmProbeClaimsV1 {
        acceleration: NixDedicatedVmAccelerationV1::HardwareKvmOnlyNoTcgFallback,
        artifact_locator_policy: NixDedicatedVmArtifactLocatorPolicyV1::OpaqueRoleOnlyNoWirePaths,
        backend: NixDedicatedVmBackendV1::QemuSystemX8664PcKvm,
        forbidden_host_fallbacks: vec![
            NixDedicatedVmForbiddenHostFallbackV1::HostInitialNamespaceCapSysAdmin,
            NixDedicatedVmForbiddenHostFallbackV1::HostPrivilegedContainerWrapper,
            NixDedicatedVmForbiddenHostFallbackV1::HostSoftwareEmulationTcg,
            NixDedicatedVmForbiddenHostFallbackV1::HostUnconfinedLsmFallback,
        ],
        guest_axis_disposition:
            NixDedicatedVmGuestAxisDispositionV1::GuestLocalAxesUnexercisedNotHostFallback,
        guest_isolation_axes: vec![
            NixDedicatedVmGuestIsolationAxisV1::Ipc,
            NixDedicatedVmGuestIsolationAxisV1::Mount,
            NixDedicatedVmGuestIsolationAxisV1::Network,
            NixDedicatedVmGuestIsolationAxisV1::Pid,
            NixDedicatedVmGuestIsolationAxisV1::User,
            NixDedicatedVmGuestIsolationAxisV1::Uts,
        ],
        requested_configuration: NixDedicatedVmRequestedConfigurationV1 {
            boot_drive_count: 1,
            boot_media_format: NixDedicatedVmBootMediaFormatV1::Raw,
            boot_media_interface: NixDedicatedVmBootMediaInterfaceV1::Floppy,
            boot_media_read_only: true,
            boot_order: NixDedicatedVmBootOrderV1::FloppyDriveAOnly,
            cpu_model: "host".to_string(),
            debug_exit_iobase: 0xf4,
            debug_exit_iosize: 4,
            display_enabled: false,
            machine: "pc".to_string(),
            memory_bytes: 67_108_864,
            monitor_enabled: false,
            network_device_count: 0,
            no_reboot: true,
            no_shutdown: true,
            no_user_config: true,
            nodefaults: true,
            qemu_sandbox: NixDedicatedVmQemuSandboxV1 {
                elevate_privileges_denied: true,
                enabled: true,
                obsolete_denied: true,
                resource_control_denied: true,
                spawn_denied: true,
            },
            serial_device_count: 1,
            vcpu_count: 1,
            writable_block_device_count: 0,
        },
        required_artifacts: exact_artifact_roles(),
        required_devices: exact_device_roles(),
    }
}

fn derive_parent(
    parent: &InvalidatedNixSandboxContractInspectionV1,
) -> Result<NixDedicatedVmProbeParentV1, NixMnlError> {
    let envelope = parent.envelope();
    if envelope.schema != NIX_SANDBOX_REQUALIFICATION_SCHEMA
        || envelope.schema_version != NIX_SANDBOX_REQUALIFICATION_SCHEMA_VERSION
        || envelope.inspected_closed_plan_schema != NIX_CLOSED_RUN_PLAN_SCHEMA
        || envelope.inspected_closed_plan_schema_version != 3
        || envelope.inspected_closed_plan_disposition
            != ClosedRunPlanDispositionV1::FreshSandboxBuildInspectionOnlyNoLaunchAuthority
    {
        return Err(invalid(
            "Nix dedicated-VM development probe parent lacks the exact retained N5c/V3 identity",
        ));
    }
    Ok(NixDedicatedVmProbeParentV1 {
        invalidated_sandbox_envelope_byte_count: u64::try_from(parent.canonical_bytes().len())
            .map_err(|_| invalid("N5c envelope byte count does not fit u64"))?,
        invalidated_sandbox_envelope_schema: envelope.schema.clone(),
        invalidated_sandbox_envelope_schema_version: envelope.schema_version,
        invalidated_sandbox_envelope_sha256: parent.envelope_sha256().to_string(),
        v3_closed_plan_byte_count: envelope.inspected_closed_plan_byte_count,
        v3_closed_plan_disposition: envelope.inspected_closed_plan_disposition,
        v3_closed_plan_schema: envelope.inspected_closed_plan_schema.clone(),
        v3_closed_plan_schema_version: envelope.inspected_closed_plan_schema_version,
        v3_closed_plan_sha256: envelope.inspected_closed_plan_sha256.clone(),
    })
}

fn validate_observations(
    observations: &NixDedicatedVmProbeObservationsV1,
) -> Result<(), NixMnlError> {
    if observations.effective_configuration_observed || observations.effective_topology_observed {
        return Err(invalid(
            "caller-supplied VM development material cannot establish effective configuration or topology",
        ));
    }
    validate_artifacts(&observations.artifacts)?;
    validate_host(&observations.host)?;
    validate_backend(
        &observations.backend,
        &observations.host,
        &observations.artifacts,
    )?;
    validate_devices(&observations.devices)?;
    validate_outcomes(&observations.outcomes)?;
    if observations.raw_capture.entry_count == 0
        || observations.raw_capture.manifest_byte_count == 0
        || observations.raw_capture.manifest_byte_count > observations.raw_capture.total_byte_count
        || !is_lower_hex_sha256(&observations.raw_capture.manifest_sha256)
        || observations.raw_capture.manifest_sha256 == EMPTY_SHA256
        || observations.raw_capture.total_byte_count == 0
        || observations.raw_capture.total_byte_count > MAX_NIX_DEDICATED_VM_RAW_CAPTURE_BYTES
    {
        return Err(invalid(
            "Nix dedicated-VM raw-capture manifest observation is incomplete",
        ));
    }
    Ok(())
}

fn validate_artifacts(artifacts: &[NixDedicatedVmObservedArtifactV1]) -> Result<(), NixMnlError> {
    if artifacts.len() != exact_artifact_roles().len() {
        return Err(invalid(
            "Nix dedicated-VM artifact observation roster is incomplete",
        ));
    }
    for (artifact, expected_role) in artifacts.iter().zip(exact_artifact_roles()) {
        if artifact.role != expected_role
            || artifact.byte_count == 0
            || artifact.byte_count > MAX_NIX_DEDICATED_VM_ARTIFACT_BYTES
            || artifact.mode == 0
            || artifact.mode > 0o777
            || !is_lower_hex_sha256(&artifact.pre_observation_sha256)
            || artifact.pre_observation_sha256 == EMPTY_SHA256
            || artifact.post_observation_sha256 != artifact.pre_observation_sha256
        {
            return Err(invalid(
                "Nix dedicated-VM artifact observation differs from its exact role/identity contract",
            ));
        }
        if matches!(
            artifact.role,
            NixDedicatedVmArtifactRoleV1::PositiveBootSectorImage
                | NixDedicatedVmArtifactRoleV1::NegativeInvalidSignatureBootSectorImage
        ) && artifact.byte_count != 512
        {
            return Err(invalid(
                "Nix dedicated-VM boot-sector artifact is not exactly 512 bytes",
            ));
        }
    }
    let positive = &artifacts[3];
    let negative = &artifacts[4];
    if positive.pre_observation_sha256 == negative.pre_observation_sha256 {
        return Err(invalid(
            "positive and invalid-signature boot sectors must have distinct identities",
        ));
    }
    Ok(())
}

fn validate_host(host: &NixDedicatedVmHostObservationV1) -> Result<(), NixMnlError> {
    for digest in [
        &host.boot_id_sha256,
        &host.cgroup_identity_sha256,
        &host.cpu_identity_sha256,
        &host.host_identity_sha256,
        &host.kernel_release_sha256,
        &host.kvm_extensions_sha256,
        &host.lsm_identity_sha256,
    ] {
        if !is_lower_hex_sha256(digest) {
            return Err(invalid(
                "Nix dedicated-VM host observation contains a malformed identity digest",
            ));
        }
    }
    let device = &host.kvm_device;
    if host.architecture != "x86_64"
        || host.kvm_api_version != 12
        || host.kvm_extension_count == 0
        || host.kvm_extensions_sha256 == EMPTY_SHA256
        || host.mount_namespace_inode == 0
        || host.pid_namespace_inode == 0
        || host.user_namespace_inode == 0
        || device.device_type != NixDedicatedVmObservedDeviceTypeV1::CharacterDevice
        || device.inode == 0
        || device.gid == 0
        || device.mode != 0o660
        || device.mount_id == 0
        || device.rdev_major != 10
        || device.rdev_minor != 232
        || device.uid != 0
    {
        return Err(invalid(
            "Nix dedicated-VM host/KVM observation differs from the exact development contract",
        ));
    }
    Ok(())
}

fn validate_backend(
    backend: &NixDedicatedVmBackendObservationV1,
    host: &NixDedicatedVmHostObservationV1,
    artifacts: &[NixDedicatedVmObservedArtifactV1],
) -> Result<(), NixMnlError> {
    let version_byte_count = u64::try_from(backend.version_bytes.len())
        .map_err(|_| invalid("QEMU version byte count does not fit u64"))?;
    if backend.argv_count == 0
        || !is_lower_hex_sha256(&backend.argv_sha256)
        || backend.argv_sha256 == EMPTY_SHA256
        || backend.artifact_sha256 != artifacts[5].pre_observation_sha256
        || !is_lower_hex_sha256(&backend.environment_sha256)
        || (backend.environment_count == 0) != (backend.environment_sha256 == EMPTY_SHA256)
        || backend.process_cgroup_identity_sha256 != host.cgroup_identity_sha256
        || backend.process_egid != host.egid
        || backend.process_euid != host.euid
        || backend.process_lsm_identity_sha256 != host.lsm_identity_sha256
        || backend.process_mount_namespace_inode != host.mount_namespace_inode
        || backend.process_pid_namespace_inode != host.pid_namespace_inode
        || backend.process_user_namespace_inode != host.user_namespace_inode
        || backend.version_bytes.is_empty()
        || backend.version_bytes.len() > MAX_NIX_DEDICATED_VM_QEMU_VERSION_BYTES
        || backend.version_bytes.contains(['\n', '\r', '\0'])
        || backend.version_byte_count != version_byte_count
        || backend.version_sha256 != sha256_hex(backend.version_bytes.as_bytes())
    {
        return Err(invalid(
            "Nix dedicated-VM QEMU backend observation differs from the exact host/artifact contract",
        ));
    }
    Ok(())
}

fn validate_devices(devices: &NixDedicatedVmDeviceObservationV1) -> Result<(), NixMnlError> {
    if devices.boot_drive_count != 1
        || devices.boot_media_format != NixDedicatedVmBootMediaFormatV1::Raw
        || devices.boot_media_interface != NixDedicatedVmBootMediaInterfaceV1::Floppy
        || !devices.boot_media_read_only
        || devices.boot_order != NixDedicatedVmBootOrderV1::FloppyDriveAOnly
        || devices.debug_exit_iobase != 0xf4
        || devices.debug_exit_iosize != 4
        || devices.network_device_count != 0
        || devices.observed_devices != exact_device_roles()
        || devices.serial_device_count != 1
        || devices.writable_block_device_count != 0
    {
        return Err(invalid(
            "Nix dedicated-VM device observation differs from the exact floppy-only configuration",
        ));
    }
    Ok(())
}

fn validate_outcomes(outcomes: &NixDedicatedVmOutcomeObservationV1) -> Result<(), NixMnlError> {
    if outcomes.negative_boot_signature_hex != "0000"
        || outcomes.negative_outcomes.len() != 3
        || outcomes.positive_boot_signature_hex != "55aa"
        || outcomes.positive_exit_code != 33
        || outcomes.positive_serial_byte_count != 23
        || outcomes.positive_serial_sha256 != POSITIVE_SERIAL_SHA256
        || outcomes.negative_outcomes.iter().any(|outcome| {
            outcome.exit_code != 124
                || outcome.serial_byte_count != 0
                || outcome.serial_sha256 != EMPTY_SHA256
        })
    {
        return Err(invalid(
            "Nix dedicated-VM positive/negative development outcomes differ from the exact probe contract",
        ));
    }
    Ok(())
}

fn exact_artifact_roles() -> Vec<NixDedicatedVmArtifactRoleV1> {
    vec![
        NixDedicatedVmArtifactRoleV1::BootAssemblySource,
        NixDedicatedVmArtifactRoleV1::LinkerScript,
        NixDedicatedVmArtifactRoleV1::ProbeHarness,
        NixDedicatedVmArtifactRoleV1::PositiveBootSectorImage,
        NixDedicatedVmArtifactRoleV1::NegativeInvalidSignatureBootSectorImage,
        NixDedicatedVmArtifactRoleV1::QemuSystemX8664Backend,
    ]
}

fn exact_device_roles() -> Vec<NixDedicatedVmDeviceRoleV1> {
    vec![
        NixDedicatedVmDeviceRoleV1::GuestReadOnlyRawFloppyBootMedia,
        NixDedicatedVmDeviceRoleV1::GuestIsaSerialConsole,
        NixDedicatedVmDeviceRoleV1::GuestIsaDebugExit,
    ]
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}
