//! Exact, fail-closed installation planning for the Linux v8 runtime bridge.
//!
//! Planning is read-only and may run without privilege. Execution accepts only
//! a fresh canonical plan whose digest, host probes, source ELF files, fixed
//! destinations, and fixed unit bytes still match. It installs files but never
//! enables or starts either unit.

#[cfg(target_os = "linux")]
use std::collections::BTreeSet;
#[cfg(any(target_os = "linux", all(test, unix)))]
use std::fs;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(all(unix, not(target_os = "linux")))]
use std::fs::OpenOptions;
#[cfg(unix)]
use std::io::Read;
#[cfg(target_os = "linux")]
use std::io::Seek;
#[cfg(target_os = "linux")]
use std::io::SeekFrom;
#[cfg(target_os = "linux")]
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use codex_hepta_linux_qualification_v8::AuthorityReplayGuardV8;
use codex_hepta_linux_qualification_v8::ExactRootInstallInventoryV2;
use codex_hepta_linux_qualification_v8::INSTALL_NAMESPACE_V2;
use codex_hepta_linux_qualification_v8::InstallStateDispositionV2;
use codex_hepta_linux_qualification_v8::InstallTargetHostBindingV2;
use codex_hepta_linux_qualification_v8::RootDirectoryInstallIdentityV2;
use codex_hepta_linux_qualification_v8::RootFileInstallIdentityV8;
use codex_hepta_linux_qualification_v8::RootStateIdentityV8;
use codex_hepta_linux_qualification_v8::SignedAuthorityV8;
use codex_hepta_linux_qualification_v8::VerifiedAuthorityV8;
use codex_hepta_linux_qualification_v8::verify_signed_authority_v8;

use crate::NativeErrorV8;
use crate::invalid;

#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::ffi::OsStr;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd as _;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd as _;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt as _;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt as _;
#[cfg(all(unix, not(target_os = "linux")))]
use std::os::unix::fs::OpenOptionsExt as _;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt as _;

pub const INSTALL_PLAN_SCHEMA_V8: &str = "hepta-linux-v8-install-plan-v2";
pub const INSTALL_RESULT_SCHEMA_V8: &str = "hepta-linux-v8-install-result-v2";
pub const INSTALL_PLAN_FRESH_SECONDS_V8: u64 = 300;
pub const PRODUCTION_STATE_ROOT_V8: &str = "/var/lib/hepta-linux-v8";
pub const PRODUCTION_BIN_DIRECTORY_V8: &str = "/usr/local/libexec/hepta-linux-v8";
pub const PRODUCTION_STATE_LOCK_PATH_V8: &str = "/var/lib/hepta-linux-v8/state.lock";
pub const RECOVER_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-recover.service";
pub const ADMISSIOND_UNIT_PATH_V8: &str = "/etc/systemd/system/hepta-linux-v8-admissiond.service";
pub const INSTALLER_NAMESPACE_LEASE_PATH_V2: &str = "/run/hepta-linux-v8-installer.lock";
pub const INSTALL_NONCE_CLAIM_SCHEMA_V2: &str = "hepta-linux-v8-install-nonce-claim-v2";

pub const RECOVER_UNIT_V8: &str = "[Unit]\n\
Description=Hepta Linux v8 fail-closed recovery preflight\n\
Before=hepta-linux-v8-admissiond.service\n\
\n\
[Service]\n\
Type=oneshot\n\
User=root\n\
Group=root\n\
ExecStart=/usr/local/libexec/hepta-linux-v8/hepta-linux-v8-recover --preflight\n\
RemainAfterExit=yes\n\
Restart=on-failure\n\
KillMode=control-group\n\
ProtectControlGroups=yes\n\
UMask=0077\n\
NoNewPrivileges=yes\n\
PrivateTmp=yes\n\
ProtectHome=yes\n\
ProtectSystem=strict\n\
ReadWritePaths=/var/lib/hepta-linux-v8\n\
ProtectKernelTunables=yes\n\
ProtectKernelModules=yes\n\
ProtectKernelLogs=yes\n\
ProtectClock=yes\n\
RestrictSUIDSGID=yes\n\
LockPersonality=yes\n\
MemoryDenyWriteExecute=yes\n\
RestrictRealtime=yes\n\
SystemCallArchitectures=native\n";

pub const ADMISSIOND_UNIT_V8: &str = "[Unit]\n\
Description=Hepta Linux v8 admission preflight (no run authority)\n\
Requires=hepta-linux-v8-recover.service\n\
After=hepta-linux-v8-recover.service\n\
\n\
[Service]\n\
Type=simple\n\
User=root\n\
Group=root\n\
ExecStart=/usr/local/libexec/hepta-linux-v8/hepta-linux-v8-admissiond --guardian\n\
Restart=always\n\
RestartSec=1s\n\
KillMode=control-group\n\
Slice=system.slice\n\
Delegate=yes\n\
ProtectControlGroups=no\n\
UMask=0077\n\
NoNewPrivileges=yes\n\
PrivateTmp=yes\n\
ProtectHome=yes\n\
ProtectSystem=strict\n\
ReadWritePaths=/var/lib/hepta-linux-v8\n\
ProtectKernelTunables=yes\n\
ProtectKernelModules=yes\n\
ProtectKernelLogs=yes\n\
ProtectClock=yes\n\
RestrictSUIDSGID=yes\n\
LockPersonality=yes\n\
MemoryDenyWriteExecute=yes\n\
RestrictRealtime=yes\n\
SystemCallArchitectures=native\n";

#[cfg(unix)]
const ELF_MAGIC: &[u8; 4] = b"\x7fELF";
const MAX_INSTALL_ELF_BYTES_V8: u64 = 128 * 1024 * 1024;
pub const MAX_INSTALL_PLAN_BYTES_V8: usize = 1024 * 1024;
pub const MAX_INSTALL_AUTHORITY_BYTES_V8: usize = 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallPrerequisitesV8 {
    pub target_linux: bool,
    pub machine_id_sha256: String,
    pub boot_id: String,
    pub cgroup_v2: bool,
    pub systemd: bool,
    pub openat2: bool,
    pub pidfd_open: bool,
    pub execveat: bool,
    pub renameat2_noreplace: bool,
    pub target_filesystem_sha256: String,
}

impl InstallPrerequisitesV8 {
    pub fn require_all(&self) -> Result<(), NativeErrorV8> {
        let missing = [
            ("target_linux", self.target_linux),
            ("machine_id", !self.machine_id_sha256.is_empty()),
            ("boot_id", !self.boot_id.is_empty()),
            ("cgroup_v2", self.cgroup_v2),
            ("systemd", self.systemd),
            ("openat2", self.openat2),
            ("pidfd_open", self.pidfd_open),
            ("execveat", self.execveat),
            ("renameat2_noreplace", self.renameat2_noreplace),
            (
                "target_filesystem",
                !self.target_filesystem_sha256.is_empty(),
            ),
        ]
        .into_iter()
        .filter_map(|(name, present)| (!present).then_some(name))
        .collect::<Vec<_>>();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(invalid(format!(
                "Linux v8 install prerequisites are absent: {}",
                missing.join(",")
            )))
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallArtifactV8 {
    pub name: String,
    pub source_path: String,
    pub destination_path: String,
    pub sha256: String,
    pub size: u64,
    pub mode: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallUnitV8 {
    pub name: String,
    pub destination_path: String,
    pub sha256: String,
    pub mode: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallStateFileV8 {
    pub name: String,
    pub destination_path: String,
    pub sha256: String,
    pub size: u64,
    pub mode: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallDirectoryV8 {
    pub name: String,
    pub destination_path: String,
    pub mode: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallActivationV8 {
    pub enable: bool,
    pub start: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallPlanV8 {
    pub schema: String,
    pub generated_unix_seconds: u64,
    pub fresh_for_seconds: u64,
    pub state_root: String,
    pub state_root_mode: u32,
    pub binary_directory: String,
    pub binary_directory_mode: u32,
    pub admissiond_cgroup_root: String,
    pub file_inventory_count: u32,
    pub directory_inventory_count: u32,
    pub state_root_layout_manifest_sha256: String,
    pub state_disposition: InstallStateDispositionV2,
    pub state_lock: InstallStateFileV8,
    pub state_directories: Vec<InstallDirectoryV8>,
    pub prerequisites: InstallPrerequisitesV8,
    pub artifacts: Vec<InstallArtifactV8>,
    pub units: Vec<InstallUnitV8>,
    pub activation: InstallActivationV8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallResultV8 {
    pub schema: String,
    pub plan_sha256: String,
    pub authority_statement_sha256: String,
    pub authority_signature_sha256: String,
    pub installed_files: u32,
    pub installed_directories: u32,
    pub enabled: bool,
    pub started: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct InstallNonceClaimV2 {
    authority_nonce: String,
    authority_signature_sha256: String,
    authority_statement_sha256: String,
    boot_id: String,
    install_plan_sha256: String,
    machine_id_sha256: String,
    namespace: String,
    schema: String,
    state_disposition: InstallStateDispositionV2,
}

impl InstallNonceClaimV2 {
    fn from_verified_v8(
        verified: &VerifiedFreshInstallPlanV8,
        authority: &VerifiedInstallAuthorityV8,
    ) -> Result<Self, NativeErrorV8> {
        let claim = Self {
            authority_nonce: authority.authority.authority_nonce().to_string(),
            authority_signature_sha256: authority.signature_sha256().to_string(),
            authority_statement_sha256: authority.statement_sha256().to_string(),
            boot_id: verified.plan.prerequisites.boot_id.clone(),
            install_plan_sha256: verified.plan_sha256.clone(),
            machine_id_sha256: verified.plan.prerequisites.machine_id_sha256.clone(),
            namespace: authority.authority.namespace().to_string(),
            schema: INSTALL_NONCE_CLAIM_SCHEMA_V2.to_string(),
            state_disposition: verified.plan.state_disposition.clone(),
        };
        claim.validate_v8()?;
        Ok(claim)
    }

    fn validate_v8(&self) -> Result<(), NativeErrorV8> {
        if self.schema != INSTALL_NONCE_CLAIM_SCHEMA_V2
            || self.namespace != INSTALL_NAMESPACE_V2
            || self.state_disposition != InstallStateDispositionV2::FreshEmpty
        {
            return Err(invalid("install nonce claim policy is not exact v2"));
        }
        for digest in [
            &self.authority_nonce,
            &self.authority_signature_sha256,
            &self.authority_statement_sha256,
            &self.install_plan_sha256,
            &self.machine_id_sha256,
        ] {
            validate_sha256_text_v8(digest)?;
        }
        validate_boot_id_text_v8(&self.boot_id)
    }

    fn canonical_bytes_v8(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate_v8()?;
        serde_json::to_vec(self)
            .map_err(|error| invalid(format!("encode canonical install nonce claim: {error}")))
    }

    fn sha256_v8(&self) -> Result<String, NativeErrorV8> {
        Ok(sha256_hex_v8(&self.canonical_bytes_v8()?))
    }

    fn leaf_v8(&self) -> Result<std::ffi::OsString, NativeErrorV8> {
        let relative = crate::nonce_claim_relative_path_v8(&self.authority_nonce)?;
        Path::new(&relative)
            .file_name()
            .map(std::ffi::OsStr::to_os_string)
            .ok_or_else(|| invalid("install nonce claim has no exact leaf"))
    }
}

/// Opaque proof that exact canonical plan bytes matched the caller-supplied
/// digest and were inside the compiled freshness window. There is no public
/// constructor, so installation cannot bypass plan parsing and freshness.
#[derive(Debug)]
pub struct VerifiedFreshInstallPlanV8 {
    plan: InstallPlanV8,
    plan_sha256: String,
}

/// Exact signed InstallV2 authority already verified against the plan, target
/// machine, boot, and production trust profile. Construction is private.
#[derive(Debug)]
pub struct VerifiedInstallAuthorityV8 {
    authority: VerifiedAuthorityV8,
    plan_sha256: String,
    valid_before_unix_seconds: u64,
}

/// A linkat publication occurred (or its outcome cannot safely be collapsed
/// to "before effect"). The retained descriptors are intentionally private;
/// dropping this token never unlinks or repairs the published name, while the
/// durable nonce claim remains the recovery root.
#[cfg(target_os = "linux")]
#[must_use = "a linked install effect must be durably resolved or quarantined"]
#[derive(Debug)]
pub struct InstallLinkIssuedOrUncertainV2 {
    claim_sha256: String,
    failure: String,
    linked_file: File,
    parent: InstallDirectoryAnchorV8,
    plan_sha256: String,
    state_disposition: InstallStateDispositionV2,
    target: PathBuf,
}

#[cfg(target_os = "linux")]
impl InstallLinkIssuedOrUncertainV2 {
    pub fn claim_sha256(&self) -> &str {
        &self.claim_sha256
    }

    pub fn failure(&self) -> &str {
        &self.failure
    }

    pub fn plan_sha256(&self) -> &str {
        &self.plan_sha256
    }

    pub fn state_disposition(&self) -> &InstallStateDispositionV2 {
        &self.state_disposition
    }

    pub fn target(&self) -> &Path {
        &self.target
    }

    pub fn revalidate_retained_descriptors_v8(&mut self) -> Result<(), NativeErrorV8> {
        self.parent.revalidate_named_v8()?;
        let mut linked = self.linked_file.try_clone()?;
        let expected = self.linked_file.metadata()?;
        linked.seek(SeekFrom::Start(0))?;
        let after = linked.metadata()?;
        if expected.dev() != after.dev()
            || expected.ino() != after.ino()
            || expected.nlink() != after.nlink()
        {
            return Err(invalid("retained linked install descriptor changed"));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum InstallExecutionFailureV2 {
    Native(NativeErrorV8),
    #[cfg(target_os = "linux")]
    LinkIssuedOrUncertain(InstallLinkIssuedOrUncertainV2),
}

impl std::fmt::Display for InstallExecutionFailureV2 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(error) => error.fmt(formatter),
            #[cfg(target_os = "linux")]
            Self::LinkIssuedOrUncertain(obligation) => write!(
                formatter,
                "Linux v8 install publication for {} is issued or uncertain: {}",
                obligation.target.display(),
                obligation.failure
            ),
        }
    }
}

impl std::error::Error for InstallExecutionFailureV2 {}

impl From<NativeErrorV8> for InstallExecutionFailureV2 {
    fn from(error: NativeErrorV8) -> Self {
        Self::Native(error)
    }
}

impl From<std::io::Error> for InstallExecutionFailureV2 {
    fn from(error: std::io::Error) -> Self {
        Self::Native(error.into())
    }
}

impl VerifiedInstallAuthorityV8 {
    pub fn statement_sha256(&self) -> &str {
        self.authority.statement_sha256()
    }

    pub fn signature_sha256(&self) -> &str {
        self.authority.detached_signature_sha256()
    }
}

impl VerifiedFreshInstallPlanV8 {
    pub fn plan(&self) -> &InstallPlanV8 {
        &self.plan
    }

    pub fn plan_sha256(&self) -> &str {
        &self.plan_sha256
    }
}

pub fn current_unix_seconds_v8() -> Result<u64, NativeErrorV8> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| invalid(format!("system clock predates the Unix epoch: {error}")))
}

pub fn parse_signed_install_authority_bytes_v8(
    bytes: &[u8],
) -> Result<SignedAuthorityV8, NativeErrorV8> {
    if bytes.is_empty() || bytes.len() > MAX_INSTALL_AUTHORITY_BYTES_V8 {
        return Err(invalid(
            "signed InstallV2 authority exceeds its exact byte bound",
        ));
    }
    serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("decode signed InstallV2 authority: {error}")))
}

pub fn read_bounded_regular_absolute_v8(
    path: &Path,
    maximum_bytes: usize,
) -> Result<Vec<u8>, NativeErrorV8> {
    if maximum_bytes == 0 {
        return Err(invalid("bounded input maximum must be non-zero"));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        Err(invalid("anchored bounded input reading requires Linux"))
    }
    #[cfg(target_os = "linux")]
    {
        let (parent_path, leaf) = absolute_parent_and_leaf_v8(path)?;
        let parent = open_absolute_directory_anchored_v8(&parent_path)?;
        let mut file = try_open_regular_beneath_v8(&parent, &leaf, false)?
            .ok_or_else(|| invalid("bounded input file is absent"))?;
        let before = file.metadata()?;
        if !before.file_type().is_file()
            || before.nlink() != 1
            || before.mode() & 0o7022 != 0
            || usize::try_from(before.len())
                .ok()
                .is_none_or(|size| size > maximum_bytes)
        {
            return Err(invalid(
                "bounded input is not one non-shared regular file within its byte limit",
            ));
        }
        file.seek(SeekFrom::Start(0))?;
        let mut bytes = Vec::with_capacity(usize::try_from(before.len()).unwrap_or(maximum_bytes));
        (&mut file)
            .take(
                u64::try_from(maximum_bytes)
                    .unwrap_or(u64::MAX)
                    .saturating_add(1),
            )
            .read_to_end(&mut bytes)?;
        let after = file.metadata()?;
        if bytes.len() > maximum_bytes
            || before.dev() != after.dev()
            || before.ino() != after.ino()
            || before.len() != after.len()
            || before.mode() != after.mode()
            || before.uid() != after.uid()
            || before.gid() != after.gid()
            || before.nlink() != after.nlink()
            || before.mtime() != after.mtime()
            || before.mtime_nsec() != after.mtime_nsec()
            || before.ctime() != after.ctime()
            || before.ctime_nsec() != after.ctime_nsec()
        {
            return Err(invalid("bounded input changed during descriptor read"));
        }
        Ok(bytes)
    }
}

pub fn observe_install_prerequisites_v8() -> InstallPrerequisitesV8 {
    observe_install_prerequisites_impl_v8()
}

pub fn build_install_plan_v8(
    artifact_directory: &Path,
    generated_unix_seconds: u64,
) -> Result<InstallPlanV8, NativeErrorV8> {
    let prerequisites = observe_install_prerequisites_v8();
    prerequisites.require_all()?;
    build_install_plan_with_prerequisites_v8(
        artifact_directory,
        generated_unix_seconds,
        prerequisites,
    )
}

fn build_install_plan_with_prerequisites_v8(
    artifact_directory: &Path,
    generated_unix_seconds: u64,
    prerequisites: InstallPrerequisitesV8,
) -> Result<InstallPlanV8, NativeErrorV8> {
    prerequisites.require_all()?;
    let artifact_directory = absolute_lexical_path_v8(artifact_directory)?;
    let artifacts = [
        "hepta-linux-v8-admissiond",
        "hepta-linux-v8-recover",
        "hepta-linux-v8ctl",
    ]
    .into_iter()
    .map(|name| {
        let source = artifact_directory.join(name);
        let bytes = read_verified_elf_v8(&source)?;
        Ok(InstallArtifactV8 {
            name: name.to_string(),
            source_path: path_utf8_v8(&source)?.to_string(),
            destination_path: format!("{PRODUCTION_BIN_DIRECTORY_V8}/{name}"),
            sha256: sha256_hex_v8(&bytes),
            size: u64::try_from(bytes.len())
                .map_err(|_| invalid("install artifact length does not fit u64"))?,
            mode: 0o555,
        })
    })
    .collect::<Result<Vec<_>, NativeErrorV8>>()?;
    let units = vec![
        InstallUnitV8 {
            name: "hepta-linux-v8-recover.service".to_string(),
            destination_path: RECOVER_UNIT_PATH_V8.to_string(),
            sha256: sha256_hex_v8(RECOVER_UNIT_V8.as_bytes()),
            mode: 0o444,
        },
        InstallUnitV8 {
            name: "hepta-linux-v8-admissiond.service".to_string(),
            destination_path: ADMISSIOND_UNIT_PATH_V8.to_string(),
            sha256: sha256_hex_v8(ADMISSIOND_UNIT_V8.as_bytes()),
            mode: 0o444,
        },
    ];
    let state_lock = InstallStateFileV8 {
        name: crate::STATE_ROOT_LOCK_LEAF_V8.to_string(),
        destination_path: PRODUCTION_STATE_LOCK_PATH_V8.to_string(),
        sha256: sha256_hex_v8(b""),
        size: 0,
        mode: 0o600,
    };
    let state_directories = [
        crate::ATTEMPTS_DIRECTORY_V8,
        crate::INSTALL_EPOCH_DIRECTORY_V8,
        crate::JOURNAL_DIRECTORY_V8,
        crate::NONCE_CLAIMS_DIRECTORY_V8,
        crate::QUARANTINE_DIRECTORY_V8,
    ]
    .into_iter()
    .map(|name| InstallDirectoryV8 {
        name: name.to_string(),
        destination_path: format!("{PRODUCTION_STATE_ROOT_V8}/{name}"),
        mode: 0o700,
    })
    .collect();
    Ok(InstallPlanV8 {
        schema: INSTALL_PLAN_SCHEMA_V8.to_string(),
        generated_unix_seconds,
        fresh_for_seconds: INSTALL_PLAN_FRESH_SECONDS_V8,
        state_root: PRODUCTION_STATE_ROOT_V8.to_string(),
        state_root_mode: 0o700,
        binary_directory: PRODUCTION_BIN_DIRECTORY_V8.to_string(),
        binary_directory_mode: 0o755,
        admissiond_cgroup_root: crate::ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8.to_string(),
        file_inventory_count: 6,
        directory_inventory_count: 7,
        state_root_layout_manifest_sha256: crate::state_root_layout_manifest_sha256_v8(),
        state_disposition: InstallStateDispositionV2::FreshEmpty,
        state_lock,
        state_directories,
        prerequisites,
        artifacts,
        units,
        activation: InstallActivationV8 {
            enable: false,
            start: false,
        },
    })
}

pub fn canonical_install_plan_bytes_v8(plan: &InstallPlanV8) -> Result<Vec<u8>, NativeErrorV8> {
    validate_install_plan_shape_v8(plan)?;
    serde_json::to_vec(plan)
        .map_err(|error| invalid(format!("encode canonical Linux v8 install plan: {error}")))
}

pub fn install_plan_sha256_v8(plan: &InstallPlanV8) -> Result<String, NativeErrorV8> {
    Ok(sha256_hex_v8(&canonical_install_plan_bytes_v8(plan)?))
}

pub fn canonical_install_result_bytes_v8(
    result: &InstallResultV8,
) -> Result<Vec<u8>, NativeErrorV8> {
    if result.schema != INSTALL_RESULT_SCHEMA_V8
        || result.installed_files != 6
        || result.installed_directories != 7
        || result.enabled
        || result.started
    {
        return Err(invalid(
            "install result differs from the compiled files-only effect",
        ));
    }
    for digest in [
        &result.plan_sha256,
        &result.authority_statement_sha256,
        &result.authority_signature_sha256,
    ] {
        validate_sha256_text_v8(digest)?;
    }
    serde_json::to_vec(result)
        .map_err(|error| invalid(format!("encode canonical Linux v8 install result: {error}")))
}

pub fn parse_fresh_install_plan_v8(
    bytes: &[u8],
    expected_sha256: &str,
    now_unix_seconds: u64,
) -> Result<VerifiedFreshInstallPlanV8, NativeErrorV8> {
    validate_sha256_text_v8(expected_sha256)?;
    if sha256_hex_v8(bytes) != expected_sha256 {
        return Err(invalid(
            "install plan digest differs from the exact supplied digest",
        ));
    }
    let plan: InstallPlanV8 = serde_json::from_slice(bytes)
        .map_err(|error| invalid(format!("decode Linux v8 install plan: {error}")))?;
    let canonical = canonical_install_plan_bytes_v8(&plan)?;
    if canonical != bytes {
        return Err(invalid("install plan bytes are not exact canonical JSON"));
    }
    if plan.generated_unix_seconds > now_unix_seconds.saturating_add(5) {
        return Err(invalid("install plan generation time is in the future"));
    }
    let age = now_unix_seconds.saturating_sub(plan.generated_unix_seconds);
    if age > plan.fresh_for_seconds {
        return Err(invalid(format!(
            "install plan is stale: age {age}s exceeds {}s",
            plan.fresh_for_seconds
        )));
    }
    Ok(VerifiedFreshInstallPlanV8 {
        plan,
        plan_sha256: expected_sha256.to_string(),
    })
}

fn require_fresh_at_execution_v8(
    plan: &InstallPlanV8,
    now_unix_seconds: u64,
) -> Result<(), NativeErrorV8> {
    if plan.generated_unix_seconds > now_unix_seconds.saturating_add(5) {
        return Err(invalid("install plan generation time is in the future"));
    }
    let age = now_unix_seconds.saturating_sub(plan.generated_unix_seconds);
    if age > INSTALL_PLAN_FRESH_SECONDS_V8 {
        return Err(invalid(format!(
            "install plan expired before root mutation: age {age}s"
        )));
    }
    Ok(())
}

pub fn require_root_install_v8(
    effective_uid: u32,
    effective_gid: u32,
) -> Result<(), NativeErrorV8> {
    if effective_uid == 0 && effective_gid == 0 {
        Ok(())
    } else {
        Err(invalid(
            "install --execute requires effective uid 0 and effective gid 0",
        ))
    }
}

pub fn exact_install_inventory_v2(
    plan: &InstallPlanV8,
) -> Result<ExactRootInstallInventoryV2, NativeErrorV8> {
    validate_install_plan_shape_v8(plan)?;
    let artifact = |name: &str| {
        plan.artifacts
            .iter()
            .find(|artifact| artifact.name == name)
            .ok_or_else(|| invalid(format!("install artifact {name} is absent")))
            .map(|artifact| RootFileInstallIdentityV8 {
                content_sha256: artifact.sha256.clone(),
                gid: 0,
                mode: artifact.mode,
                path: artifact.destination_path.clone(),
                size_bytes: artifact.size,
                uid: 0,
            })
    };
    let unit = |name: &str| {
        plan.units
            .iter()
            .find(|unit| unit.name == name)
            .ok_or_else(|| invalid(format!("install unit {name} is absent")))
            .and_then(|unit| {
                let bytes = unit_bytes_for_inventory_v8(name)?;
                Ok(RootFileInstallIdentityV8 {
                    content_sha256: unit.sha256.clone(),
                    gid: 0,
                    mode: unit.mode,
                    path: unit.destination_path.clone(),
                    size_bytes: u64::try_from(bytes.len())
                        .map_err(|_| invalid("unit byte length does not fit u64"))?,
                    uid: 0,
                })
            })
    };
    let directory = |name: &str| {
        plan.state_directories
            .iter()
            .find(|directory| directory.name == name)
            .ok_or_else(|| invalid(format!("state directory {name} is absent")))
            .map(|directory| RootDirectoryInstallIdentityV2 {
                gid: 0,
                mode: directory.mode,
                path: directory.destination_path.clone(),
                uid: 0,
            })
    };
    Ok(ExactRootInstallInventoryV2 {
        ctl_binary: artifact("hepta-linux-v8ctl")?,
        admissiond_binary: artifact("hepta-linux-v8-admissiond")?,
        recovery_binary: artifact("hepta-linux-v8-recover")?,
        admissiond_unit: unit("hepta-linux-v8-admissiond.service")?,
        recovery_unit: unit("hepta-linux-v8-recover.service")?,
        binary_directory: RootDirectoryInstallIdentityV2 {
            gid: 0,
            mode: plan.binary_directory_mode,
            path: plan.binary_directory.clone(),
            uid: 0,
        },
        state_root: RootStateIdentityV8 {
            gid: 0,
            layout_manifest_sha256: plan.state_root_layout_manifest_sha256.clone(),
            mode: plan.state_root_mode,
            path: plan.state_root.clone(),
            uid: 0,
        },
        attempts_directory: directory(crate::ATTEMPTS_DIRECTORY_V8)?,
        install_epoch_directory: directory(crate::INSTALL_EPOCH_DIRECTORY_V8)?,
        journal_directory: directory(crate::JOURNAL_DIRECTORY_V8)?,
        nonce_claims_directory: directory(crate::NONCE_CLAIMS_DIRECTORY_V8)?,
        quarantine_directory: directory(crate::QUARANTINE_DIRECTORY_V8)?,
        state_lock: RootFileInstallIdentityV8 {
            content_sha256: plan.state_lock.sha256.clone(),
            gid: 0,
            mode: plan.state_lock.mode,
            path: plan.state_lock.destination_path.clone(),
            size_bytes: plan.state_lock.size,
            uid: 0,
        },
    })
}

pub fn verify_install_authority_for_plan_v8(
    signed: &SignedAuthorityV8,
    verified_plan: &VerifiedFreshInstallPlanV8,
) -> Result<VerifiedInstallAuthorityV8, NativeErrorV8> {
    let now_unix_seconds = current_unix_seconds_v8()?;
    let mut replay = AuthorityReplayGuardV8::default();
    let authority = verify_signed_authority_v8(signed, now_unix_seconds, &mut replay)
        .map_err(|error| invalid(format!("verify signed InstallV2 authority: {error}")))?;
    let (
        authorized_plan_sha256,
        authorized_state_disposition,
        authorized_inventory,
        authorized_host,
    ) = authority
        .authorized_install_v2()
        .ok_or_else(|| invalid("signed authority is not the closed InstallV2 scope"))?;
    let expected_inventory = exact_install_inventory_v2(verified_plan.plan())?;
    let expected_host = InstallTargetHostBindingV2 {
        boot_id: verified_plan.plan().prerequisites.boot_id.clone(),
        machine_id_sha256: verified_plan.plan().prerequisites.machine_id_sha256.clone(),
    };
    if authorized_plan_sha256 != verified_plan.plan_sha256()
        || authorized_state_disposition != &verified_plan.plan().state_disposition
        || authorized_inventory != &expected_inventory
        || authorized_host != &expected_host
    {
        return Err(invalid(
            "signed InstallV2 authority differs from the exact plan, state disposition, inventory, or target boot",
        ));
    }
    Ok(VerifiedInstallAuthorityV8 {
        authority,
        plan_sha256: verified_plan.plan_sha256.clone(),
        valid_before_unix_seconds: signed.challenge.expires_at_unix_seconds,
    })
}

fn unit_bytes_for_inventory_v8(name: &str) -> Result<&'static [u8], NativeErrorV8> {
    match name {
        "hepta-linux-v8-recover.service" => Ok(RECOVER_UNIT_V8.as_bytes()),
        "hepta-linux-v8-admissiond.service" => Ok(ADMISSIOND_UNIT_V8.as_bytes()),
        _ => Err(invalid("unknown fixed Linux v8 unit")),
    }
}

pub fn execute_install_plan_v8(
    verified: &VerifiedFreshInstallPlanV8,
    authority: &VerifiedInstallAuthorityV8,
) -> Result<InstallResultV8, InstallExecutionFailureV2> {
    let plan = &verified.plan;
    #[cfg(not(target_os = "linux"))]
    {
        let _ = plan;
        let _ = (&authority.plan_sha256, authority.valid_before_unix_seconds);
        Err(invalid("Linux v8 installation is supported only on Linux").into())
    }
    #[cfg(target_os = "linux")]
    {
        let now_unix_seconds = current_unix_seconds_v8()?;
        require_fresh_at_execution_v8(plan, now_unix_seconds)?;
        if authority.plan_sha256 != verified.plan_sha256
            || now_unix_seconds >= authority.valid_before_unix_seconds
        {
            return Err(
                invalid("InstallV2 authority is stale or bound to a different exact plan").into(),
            );
        }
        // SAFETY: geteuid/getegid have no arguments and no preconditions.
        let effective_uid = unsafe { libc::geteuid() };
        // SAFETY: see above.
        let effective_gid = unsafe { libc::getegid() };
        require_root_install_v8(effective_uid, effective_gid)?;
        validate_install_plan_shape_v8(plan)?;
        let observed = observe_install_prerequisites_v8();
        observed.require_all()?;
        if observed != plan.prerequisites {
            return Err(invalid("host prerequisite observation changed after install-plan").into());
        }
        let artifact_bytes = read_all_artifacts_exact_v8(&plan.artifacts)?;

        // Re-observe the wall-clock deadline after reading every source and
        // immediately before the first root mutation. A parsed proof cannot be
        // retained past either the plan or authority deadline.
        let mutation_now = current_unix_seconds_v8()?;
        require_fresh_at_execution_v8(plan, mutation_now)?;
        if mutation_now >= authority.valid_before_unix_seconds {
            return Err(invalid("InstallV2 authority expired before mutation").into());
        }
        let mutation_observed = observe_install_prerequisites_v8();
        mutation_observed.require_all()?;
        if mutation_observed != plan.prerequisites {
            return Err(invalid(
                "full host and target-filesystem observation changed immediately before mutation",
            )
            .into());
        }
        let (
            authorized_plan_sha256,
            authorized_state_disposition,
            authorized_inventory,
            authorized_host,
        ) = authority
            .authority
            .authorized_install_v2()
            .ok_or_else(|| invalid("verified authority lost its InstallV2 scope"))?;
        let expected_inventory = exact_install_inventory_v2(plan)?;
        let observed_host = InstallTargetHostBindingV2 {
            boot_id: mutation_observed.boot_id,
            machine_id_sha256: mutation_observed.machine_id_sha256,
        };
        if authorized_plan_sha256 != verified.plan_sha256()
            || authorized_state_disposition != &plan.state_disposition
            || authorized_inventory != &expected_inventory
            || authorized_host != &observed_host
        {
            return Err(invalid(
                "InstallV2 authority no longer matches the exact mutation inventory",
            )
            .into());
        }

        let _claim = InstallNonceClaimV2::from_verified_v8(verified, authority)?;
        execute_anchored_install_v2(plan, &artifact_bytes)?;
        Ok(InstallResultV8 {
            schema: INSTALL_RESULT_SCHEMA_V8.to_string(),
            plan_sha256: verified.plan_sha256.clone(),
            authority_statement_sha256: authority.statement_sha256().to_string(),
            authority_signature_sha256: authority.signature_sha256().to_string(),
            installed_files: plan.file_inventory_count,
            installed_directories: plan.directory_inventory_count,
            enabled: false,
            started: false,
        })
    }
}

fn validate_install_plan_shape_v8(plan: &InstallPlanV8) -> Result<(), NativeErrorV8> {
    if plan.schema != INSTALL_PLAN_SCHEMA_V8
        || plan.fresh_for_seconds != INSTALL_PLAN_FRESH_SECONDS_V8
        || plan.state_root != PRODUCTION_STATE_ROOT_V8
        || plan.state_root_mode != 0o700
        || plan.binary_directory != PRODUCTION_BIN_DIRECTORY_V8
        || plan.binary_directory_mode != 0o755
        || plan.admissiond_cgroup_root != crate::ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8
        || plan.file_inventory_count != 6
        || plan.directory_inventory_count != 7
        || plan.state_root_layout_manifest_sha256 != crate::state_root_layout_manifest_sha256_v8()
        || plan.state_disposition != InstallStateDispositionV2::FreshEmpty
        || plan.activation.enable
        || plan.activation.start
    {
        return Err(invalid(
            "install plan differs from the compiled fixed policy",
        ));
    }
    plan.prerequisites.require_all()?;
    validate_sha256_text_v8(&plan.prerequisites.machine_id_sha256)?;
    validate_sha256_text_v8(&plan.prerequisites.target_filesystem_sha256)?;
    validate_boot_id_text_v8(&plan.prerequisites.boot_id)?;
    if Path::new(PRODUCTION_STATE_ROOT_V8).join(crate::STATE_ROOT_LOCK_LEAF_V8)
        != Path::new(PRODUCTION_STATE_LOCK_PATH_V8)
        || plan.state_lock.name != crate::STATE_ROOT_LOCK_LEAF_V8
        || plan.state_lock.destination_path != PRODUCTION_STATE_LOCK_PATH_V8
        || plan.state_lock.sha256 != sha256_hex_v8(b"")
        || plan.state_lock.size != 0
        || plan.state_lock.mode != 0o600
    {
        return Err(invalid(
            "install state lock differs from the compiled exact inventory",
        ));
    }
    if plan.artifacts.len() != 3 || plan.units.len() != 2 || plan.state_directories.len() != 5 {
        return Err(invalid("install plan inventory cardinality is not exact"));
    }
    for (directory, expected_name) in plan.state_directories.iter().zip([
        crate::ATTEMPTS_DIRECTORY_V8,
        crate::INSTALL_EPOCH_DIRECTORY_V8,
        crate::JOURNAL_DIRECTORY_V8,
        crate::NONCE_CLAIMS_DIRECTORY_V8,
        crate::QUARANTINE_DIRECTORY_V8,
    ]) {
        if directory.name != expected_name
            || directory.destination_path != format!("{PRODUCTION_STATE_ROOT_V8}/{expected_name}")
            || directory.mode != 0o700
        {
            return Err(invalid(
                "install state directory differs from the closed-world layout",
            ));
        }
    }
    for (artifact, expected_name) in plan.artifacts.iter().zip([
        "hepta-linux-v8-admissiond",
        "hepta-linux-v8-recover",
        "hepta-linux-v8ctl",
    ]) {
        validate_sha256_text_v8(&artifact.sha256)?;
        if artifact.name != expected_name
            || artifact.destination_path != format!("{PRODUCTION_BIN_DIRECTORY_V8}/{expected_name}")
            || artifact.mode != 0o555
            || artifact.size < 4
            || artifact.size > MAX_INSTALL_ELF_BYTES_V8
            || absolute_lexical_path_v8(Path::new(&artifact.source_path)).is_err()
        {
            return Err(invalid("install artifact differs from the exact inventory"));
        }
    }
    let expected_units = [
        (
            "hepta-linux-v8-recover.service",
            RECOVER_UNIT_PATH_V8,
            RECOVER_UNIT_V8,
        ),
        (
            "hepta-linux-v8-admissiond.service",
            ADMISSIOND_UNIT_PATH_V8,
            ADMISSIOND_UNIT_V8,
        ),
    ];
    for (unit, (name, destination, bytes)) in plan.units.iter().zip(expected_units) {
        if unit.name != name
            || unit.destination_path != destination
            || unit.mode != 0o444
            || unit.sha256 != sha256_hex_v8(bytes.as_bytes())
        {
            return Err(invalid(
                "install unit differs from the compiled exact bytes",
            ));
        }
    }
    Ok(())
}

#[cfg(any(target_os = "linux", all(test, unix)))]
fn read_all_artifacts_exact_v8(
    artifacts: &[InstallArtifactV8],
) -> Result<Vec<Vec<u8>>, NativeErrorV8> {
    let mut exact = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let bytes = read_verified_elf_v8(Path::new(&artifact.source_path))?;
        let size = u64::try_from(bytes.len())
            .map_err(|_| invalid("install artifact length does not fit u64"))?;
        if size != artifact.size || sha256_hex_v8(&bytes) != artifact.sha256 {
            return Err(invalid(format!(
                "install artifact changed after planning: {}",
                artifact.name
            )));
        }
        exact.push(bytes);
    }
    Ok(exact)
}

fn read_verified_elf_v8(path: &Path) -> Result<Vec<u8>, NativeErrorV8> {
    #[cfg(not(unix))]
    {
        let _ = path;
        return Err(invalid("ELF artifact verification requires a Unix host"));
    }
    #[cfg(target_os = "linux")]
    {
        let maximum = usize::try_from(MAX_INSTALL_ELF_BYTES_V8)
            .map_err(|_| invalid("ELF byte limit does not fit usize"))?;
        let bytes = read_bounded_regular_absolute_v8(path, maximum)?;
        if bytes.len() < 4 || bytes.get(..4) != Some(ELF_MAGIC.as_slice()) {
            return Err(invalid(format!(
                "install artifact is not one bounded ELF file: {}",
                path.display()
            )));
        }
        Ok(bytes)
    }
    #[cfg(all(unix, not(target_os = "linux")))]
    {
        let mut options = OpenOptions::new();
        options
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
        let mut file = options.open(path)?;
        let before = file.metadata()?;
        if !before.file_type().is_file()
            || before.nlink() != 1
            || before.len() < 4
            || before.len() > MAX_INSTALL_ELF_BYTES_V8
        {
            return Err(invalid(format!(
                "install artifact is not an exact single-link bounded regular file: {}",
                path.display()
            )));
        }
        let capacity = usize::try_from(before.len())
            .map_err(|_| invalid("install artifact size does not fit usize"))?;
        let mut bytes = Vec::with_capacity(capacity);
        file.read_to_end(&mut bytes)?;
        let after = file.metadata()?;
        if before.dev() != after.dev()
            || before.ino() != after.ino()
            || before.len() != after.len()
            || before.mtime() != after.mtime()
            || before.mtime_nsec() != after.mtime_nsec()
            || u64::try_from(bytes.len()).ok() != Some(before.len())
            || bytes.get(..4) != Some(ELF_MAGIC.as_slice())
        {
            return Err(invalid(format!(
                "install artifact changed during read or is not ELF: {}",
                path.display()
            )));
        }
        Ok(bytes)
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct InstallDirectoryAnchorV8 {
    descriptor: OwnedFd,
    display: PathBuf,
}

#[cfg(target_os = "linux")]
impl InstallDirectoryAnchorV8 {
    fn revalidate_named_v8(&self) -> Result<(), NativeErrorV8> {
        let retained = InstallFileIdentityV2::from_fd_v8(self.descriptor.as_raw_fd())?;
        let named = open_absolute_directory_anchored_v8(&self.display)?;
        let named = InstallFileIdentityV2::from_fd_v8(named.descriptor.as_raw_fd())?;
        if !retained.same_stable_directory_v8(named) {
            return Err(invalid(format!(
                "install directory pathname was renamed or recreated: {}",
                self.display.display()
            )));
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InstallFileIdentityV2 {
    device: u64,
    inode: u64,
    uid: u32,
    gid: u32,
    mode: u32,
    link_count: u64,
    size: u64,
}

#[cfg(target_os = "linux")]
impl InstallFileIdentityV2 {
    fn from_fd_v8(fd: libc::c_int) -> Result<Self, NativeErrorV8> {
        let stat = stat_fd_v8(fd)?;
        Ok(Self {
            device: stat.st_dev,
            inode: stat.st_ino,
            uid: stat.st_uid,
            gid: stat.st_gid,
            mode: stat.st_mode,
            link_count: u64::from(stat.st_nlink),
            size: stat.st_size.try_into().unwrap_or(u64::MAX),
        })
    }

    fn same_stable_directory_v8(self, other: Self) -> bool {
        self.device == other.device
            && self.inode == other.inode
            && self.uid == other.uid
            && self.gid == other.gid
            && self.mode == other.mode
            && self.link_count > 0
            && other.link_count > 0
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
struct InstallerNamespaceLeaseV2 {
    descriptor: File,
    identity: InstallFileIdentityV2,
    leaf: Box<OsStr>,
    owner_pid: libc::pid_t,
    parent: InstallDirectoryAnchorV8,
    parent_identity: InstallFileIdentityV2,
}

#[cfg(target_os = "linux")]
impl InstallerNamespaceLeaseV2 {
    fn acquire_production_v8() -> Result<Self, NativeErrorV8> {
        Self::acquire_at_v8(
            Path::new("/run"),
            OsStr::new("hepta-linux-v8-installer.lock"),
            ROOT_INSTALL_OWNER_V8,
        )
    }

    fn acquire_at_v8(
        parent_path: &Path,
        leaf: &OsStr,
        expected_owner: InstallOwnerV8,
    ) -> Result<Self, NativeErrorV8> {
        let parent = open_absolute_directory_anchored_v8(parent_path)?;
        require_secure_parent_v8(&parent, expected_owner)?;
        let parent_identity = InstallFileIdentityV2::from_fd_v8(parent.descriptor.as_raw_fd())?;
        let leaf_c = cstring_leaf_v8(leaf)?;
        let mut how: libc::open_how = unsafe { std::mem::zeroed() };
        how.flags = u64::try_from(
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
        .map_err(|_| invalid("invalid installer namespace lease flags"))?;
        how.mode = 0o600;
        how.resolve = install_resolve_flags_v8() | libc::RESOLVE_NO_XDEV;
        // SAFETY: arguments remain live and a successful descriptor is owned.
        let raw = unsafe {
            libc::syscall(
                libc::SYS_openat2,
                parent.descriptor.as_raw_fd(),
                leaf_c.as_ptr(),
                &how as *const libc::open_how,
                std::mem::size_of::<libc::open_how>(),
            )
        };
        let (descriptor, created) = if raw >= 0 {
            let raw = libc::c_int::try_from(raw)
                .map_err(|_| invalid("openat2 returned invalid installer lease descriptor"))?;
            // SAFETY: successful openat2 returned one unique descriptor.
            (unsafe { OwnedFd::from_raw_fd(raw) }, true)
        } else {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::EEXIST) {
                return Err(error.into());
            }
            (open_regular_beneath_fd_v8(&parent, leaf, true)?, false)
        };
        if created {
            // The mode passed to O_EXCL is still filtered through umask. Only
            // this newly created fd may be normalized; an existing leaf is
            // exact-only and is never repaired.
            // SAFETY: descriptor is live and mode is scalar.
            if unsafe { libc::fchmod(descriptor.as_raw_fd(), 0o600) } != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
        }
        let identity = InstallFileIdentityV2::from_fd_v8(descriptor.as_raw_fd())?;
        if identity.mode & libc::S_IFMT != libc::S_IFREG
            || identity.uid != expected_owner.uid
            || identity.gid != expected_owner.gid
            || identity.mode & 0o7777 != 0o600
            || identity.link_count != 1
            || identity.size != 0
        {
            return Err(invalid(
                "installer namespace lease is not one exact empty 0600 owner-bound file",
            ));
        }
        sync_fd_v8(descriptor.as_raw_fd())?;
        if created {
            sync_fd_v8(parent.descriptor.as_raw_fd())?;
        }
        // SAFETY: flock receives one live open file description.
        if unsafe { libc::flock(descriptor.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let lease = Self {
            descriptor: File::from(descriptor),
            identity,
            leaf: leaf.into(),
            // SAFETY: getpid has no arguments.
            owner_pid: unsafe { libc::getpid() },
            parent,
            parent_identity,
        };
        lease.revalidate_v8()?;
        Ok(lease)
    }

    fn revalidate_v8(&self) -> Result<(), NativeErrorV8> {
        // SAFETY: getpid has no arguments.
        if unsafe { libc::getpid() } != self.owner_pid {
            return Err(invalid(
                "installer namespace lease was inherited across fork",
            ));
        }
        let retained_parent =
            InstallFileIdentityV2::from_fd_v8(self.parent.descriptor.as_raw_fd())?;
        if !retained_parent.same_stable_directory_v8(self.parent_identity) {
            return Err(invalid(
                "retained installer lease parent changed or was unlinked",
            ));
        }
        let named_parent = open_absolute_directory_anchored_v8(&self.parent.display)?;
        let named_parent_identity =
            InstallFileIdentityV2::from_fd_v8(named_parent.descriptor.as_raw_fd())?;
        if !named_parent_identity.same_stable_directory_v8(self.parent_identity) {
            return Err(invalid(
                "fixed installer lease parent pathname was renamed or recreated",
            ));
        }
        let retained = InstallFileIdentityV2::from_fd_v8(self.descriptor.as_raw_fd())?;
        let named = open_regular_beneath_fd_v8(&self.parent, &self.leaf, true)?;
        let named = InstallFileIdentityV2::from_fd_v8(named.as_raw_fd())?;
        if retained != self.identity || named != self.identity {
            return Err(invalid(
                "installer namespace lease name no longer binds the flocked inode",
            ));
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
trait InstallMutationGuardV2 {
    fn claim_sha256_v8(&self) -> &str;
    fn plan_sha256_v8(&self) -> &str;
    fn state_disposition_v8(&self) -> &InstallStateDispositionV2;
    fn revalidate_v8(&self) -> Result<(), NativeErrorV8>;
}

#[cfg(target_os = "linux")]
struct InstallBootstrapGuardV2<'a> {
    claim_sha256: &'a str,
    lease: &'a InstallerNamespaceLeaseV2,
    plan_sha256: &'a str,
    state_disposition: &'a InstallStateDispositionV2,
    state_lock: &'a crate::StateRootLockV8,
    state_root: &'a crate::DirectoryAnchorV8,
    state_root_path: &'a Path,
}

#[cfg(target_os = "linux")]
impl InstallMutationGuardV2 for InstallBootstrapGuardV2<'_> {
    fn claim_sha256_v8(&self) -> &str {
        self.claim_sha256
    }

    fn plan_sha256_v8(&self) -> &str {
        self.plan_sha256
    }

    fn state_disposition_v8(&self) -> &InstallStateDispositionV2 {
        self.state_disposition
    }

    fn revalidate_v8(&self) -> Result<(), NativeErrorV8> {
        self.lease.revalidate_v8()?;
        self.state_root.revalidate_identity()?;
        self.state_lock.revalidate_for_root(self.state_root)?;
        let named = crate::DirectoryAnchorV8::open(self.state_root_path)?;
        if !named
            .identity()
            .matches_stable_directory(self.state_root.identity())
        {
            return Err(invalid(
                "fixed install state-root pathname was renamed or recreated",
            ));
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
struct InstallTransactionV2 {
    claim_bytes: Vec<u8>,
    claim_identity: crate::FileIdentityV8,
    claim_relative_path: PathBuf,
    claim_sha256: String,
    lease: InstallerNamespaceLeaseV2,
    plan_sha256: String,
    state_disposition: InstallStateDispositionV2,
    state_lock: crate::StateRootLockV8,
    state_root: crate::DirectoryAnchorV8,
    state_root_path: PathBuf,
}

#[cfg(target_os = "linux")]
impl InstallMutationGuardV2 for InstallTransactionV2 {
    fn claim_sha256_v8(&self) -> &str {
        &self.claim_sha256
    }

    fn plan_sha256_v8(&self) -> &str {
        &self.plan_sha256
    }

    fn state_disposition_v8(&self) -> &InstallStateDispositionV2 {
        &self.state_disposition
    }

    fn revalidate_v8(&self) -> Result<(), NativeErrorV8> {
        self.lease.revalidate_v8()?;
        self.state_root.revalidate_identity()?;
        self.state_lock.revalidate_for_root(&self.state_root)?;
        let named = crate::DirectoryAnchorV8::open(&self.state_root_path)?;
        if !named
            .identity()
            .matches_stable_directory(self.state_root.identity())
        {
            return Err(invalid(
                "fixed install state-root pathname was renamed or recreated",
            ));
        }
        let claim = self
            .state_root
            .open_regular_readonly_beneath(&self.claim_relative_path)?;
        if claim.identity() != self.claim_identity
            || claim.read_all(u64::try_from(self.claim_bytes.len()).unwrap_or(u64::MAX) + 1)?
                != self.claim_bytes
        {
            return Err(invalid(
                "durable install nonce claim changed after transaction binding",
            ));
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
enum InstallFilePublishFailureV2 {
    BeforeLink(NativeErrorV8),
    LinkIssuedOrUncertain(InstallLinkIssuedOrUncertainV2),
}

#[cfg(target_os = "linux")]
impl From<NativeErrorV8> for InstallFilePublishFailureV2 {
    fn from(error: NativeErrorV8) -> Self {
        Self::BeforeLink(error)
    }
}

#[cfg(target_os = "linux")]
impl From<std::io::Error> for InstallFilePublishFailureV2 {
    fn from(error: std::io::Error) -> Self {
        Self::BeforeLink(error.into())
    }
}

#[cfg(target_os = "linux")]
fn publish_tmpfile_noreplace_v2<G: InstallMutationGuardV2>(
    path: &Path,
    expected_bytes: &[u8],
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
    guard: &G,
) -> Result<(), InstallFilePublishFailureV2> {
    guard.revalidate_v8()?;
    let (parent_path, final_leaf) = absolute_parent_and_leaf_v8(path)?;
    let parent = open_absolute_directory_anchored_v8(&parent_path)?;
    require_secure_parent_v8(&parent, expected_owner)?;
    parent.revalidate_named_v8()?;

    if let Some(mut existing) = try_open_regular_beneath_v8(&parent, &final_leaf, false)? {
        // EEXIST retry is accepted only while the same signed plan, durable
        // claim, and state disposition are still live under both locks.
        guard.revalidate_v8()?;
        validate_exact_open_file_v8(
            &mut existing,
            expected_bytes,
            expected_mode,
            expected_owner,
            &path.display().to_string(),
        )?;
        sync_fd_v8(parent.descriptor.as_raw_fd())?;
        parent.revalidate_named_v8()?;
        guard.revalidate_v8()?;
        return Ok(());
    }

    // SAFETY: openat receives a live directory descriptor and static ".".
    let raw = unsafe {
        libc::openat(
            parent.descriptor.as_raw_fd(),
            c".".as_ptr(),
            libc::O_TMPFILE | libc::O_RDWR | libc::O_CLOEXEC,
            0o600,
        )
    };
    if raw < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: successful openat returned one uniquely owned descriptor.
    let mut temporary = File::from(unsafe { OwnedFd::from_raw_fd(raw) });
    // SAFETY: temporary is live and the mode is scalar.
    if unsafe { libc::fchmod(temporary.as_raw_fd(), expected_mode) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    temporary.write_all(expected_bytes)?;
    temporary.sync_all()?;
    validate_exact_unlinked_tmpfile_v2(
        &mut temporary,
        expected_bytes,
        expected_mode,
        expected_owner,
    )?;
    guard.revalidate_v8()?;
    parent.revalidate_named_v8()?;

    let final_c = cstring_leaf_v8(&final_leaf)?;
    // SAFETY: AT_EMPTY_PATH binds the empty source to temporary's retained fd;
    // the final parent/name are live and linkat is no-replace by definition.
    let linked = unsafe {
        libc::linkat(
            temporary.as_raw_fd(),
            c"".as_ptr(),
            parent.descriptor.as_raw_fd(),
            final_c.as_ptr(),
            libc::AT_EMPTY_PATH,
        )
    };
    if linked != 0 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EEXIST) {
            return Err(error.into());
        }
        guard.revalidate_v8()?;
        let mut existing = try_open_regular_beneath_v8(&parent, &final_leaf, false)?
            .ok_or_else(|| invalid("linkat EEXIST target disappeared"))?;
        validate_exact_open_file_v8(
            &mut existing,
            expected_bytes,
            expected_mode,
            expected_owner,
            &path.display().to_string(),
        )?;
        sync_fd_v8(parent.descriptor.as_raw_fd())?;
        parent.revalidate_named_v8()?;
        guard.revalidate_v8()?;
        return Ok(());
    }

    let post_link = (|| -> Result<(), NativeErrorV8> {
        sync_fd_v8(parent.descriptor.as_raw_fd())?;
        parent.revalidate_named_v8()?;
        guard.revalidate_v8()?;
        let mut named = try_open_regular_beneath_v8(&parent, &final_leaf, false)?
            .ok_or_else(|| invalid("linked install target disappeared"))?;
        validate_exact_open_file_v8(
            &mut named,
            expected_bytes,
            expected_mode,
            expected_owner,
            &path.display().to_string(),
        )?;
        let retained = InstallFileIdentityV2::from_fd_v8(temporary.as_raw_fd())?;
        let named_identity = InstallFileIdentityV2::from_fd_v8(named.as_raw_fd())?;
        if retained != named_identity || retained.link_count != 1 {
            return Err(invalid(
                "linked install name differs from the retained O_TMPFILE inode",
            ));
        }
        Ok(())
    })();
    if let Err(error) = post_link {
        return Err(InstallFilePublishFailureV2::LinkIssuedOrUncertain(
            InstallLinkIssuedOrUncertainV2 {
                claim_sha256: guard.claim_sha256_v8().to_string(),
                failure: error.to_string(),
                linked_file: temporary,
                parent,
                plan_sha256: guard.plan_sha256_v8().to_string(),
                state_disposition: guard.state_disposition_v8().clone(),
                target: path.to_path_buf(),
            },
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn validate_exact_unlinked_tmpfile_v2(
    file: &mut File,
    expected_bytes: &[u8],
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let before = file.metadata()?;
    if !before.file_type().is_file()
        || before.uid() != expected_owner.uid
        || before.gid() != expected_owner.gid
        || before.mode() & 0o7777 != expected_mode
        || before.nlink() != 0
        || usize::try_from(before.len()).ok() != Some(expected_bytes.len())
    {
        return Err(invalid("anonymous install tmpfile identity is not exact"));
    }
    file.seek(SeekFrom::Start(0))?;
    let mut observed = Vec::with_capacity(expected_bytes.len());
    (&mut *file)
        .take(u64::try_from(expected_bytes.len()).unwrap_or(u64::MAX) + 1)
        .read_to_end(&mut observed)?;
    let after = file.metadata()?;
    if observed != expected_bytes
        || before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.mode() != after.mode()
        || before.uid() != after.uid()
        || before.gid() != after.gid()
        || before.nlink() != after.nlink()
        || before.len() != after.len()
    {
        return Err(invalid(
            "anonymous install tmpfile changed during validation",
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct InstallOwnerV8 {
    uid: u32,
    gid: u32,
}

#[cfg(target_os = "linux")]
const ROOT_INSTALL_OWNER_V8: InstallOwnerV8 = InstallOwnerV8 { uid: 0, gid: 0 };

#[cfg(target_os = "linux")]
fn execute_anchored_install_v2(
    plan: &InstallPlanV8,
    artifact_bytes: &[Vec<u8>],
) -> Result<(), NativeErrorV8> {
    ensure_exact_directory_anchored_v8(
        Path::new(&plan.state_root),
        plan.state_root_mode,
        ROOT_INSTALL_OWNER_V8,
    )?;
    for directory in &plan.state_directories {
        ensure_exact_directory_anchored_v8(
            Path::new(&directory.destination_path),
            directory.mode,
            ROOT_INSTALL_OWNER_V8,
        )?;
    }
    install_atomic_file_anchored_v8(
        Path::new(&plan.state_lock.destination_path),
        b"",
        plan.state_lock.mode,
        ROOT_INSTALL_OWNER_V8,
    )?;
    ensure_exact_directory_anchored_v8(
        Path::new(&plan.binary_directory),
        plan.binary_directory_mode,
        ROOT_INSTALL_OWNER_V8,
    )?;
    for (artifact, bytes) in plan.artifacts.iter().zip(artifact_bytes) {
        install_atomic_file_anchored_v8(
            Path::new(&artifact.destination_path),
            bytes,
            artifact.mode,
            ROOT_INSTALL_OWNER_V8,
        )?;
    }
    for unit in &plan.units {
        install_atomic_file_anchored_v8(
            Path::new(&unit.destination_path),
            unit_bytes_for_inventory_v8(&unit.name)?,
            unit.mode,
            ROOT_INSTALL_OWNER_V8,
        )?;
    }
    verify_final_install_inventory_v8(plan, artifact_bytes, ROOT_INSTALL_OWNER_V8)
}

#[cfg(target_os = "linux")]
fn ensure_exact_directory_anchored_v8(
    path: &Path,
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let (parent_path, leaf) = absolute_parent_and_leaf_v8(path)?;
    let parent = open_absolute_directory_anchored_v8(&parent_path)?;
    require_secure_parent_v8(&parent, expected_owner)?;
    let leaf_c = cstring_leaf_v8(&leaf)?;
    // SAFETY: the parent descriptor and leaf remain live and mkdirat retains neither.
    if unsafe {
        libc::mkdirat(
            parent.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            expected_mode,
        )
    } != 0
    {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EEXIST) {
            return Err(error.into());
        }
    }
    let child = open_directory_beneath_v8(&parent, &leaf)?;
    require_exact_directory_v8(&child, expected_mode, expected_owner)?;
    sync_fd_v8(child.descriptor.as_raw_fd())?;
    sync_fd_v8(parent.descriptor.as_raw_fd())
}

#[cfg(target_os = "linux")]
fn install_atomic_file_anchored_v8(
    path: &Path,
    expected_bytes: &[u8],
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let (parent_path, final_leaf) = absolute_parent_and_leaf_v8(path)?;
    let parent = open_absolute_directory_anchored_v8(&parent_path)?;
    require_secure_parent_v8(&parent, expected_owner)?;
    let digest = sha256_hex_v8(expected_bytes);
    let stage_leaf = std::ffi::OsString::from(format!(
        ".{}.{}.installing",
        final_leaf.to_string_lossy(),
        digest
    ));

    if let Some(mut final_file) = try_open_regular_beneath_v8(&parent, &final_leaf, false)? {
        validate_exact_open_file_v8(
            &mut final_file,
            expected_bytes,
            expected_mode,
            expected_owner,
            &path.display().to_string(),
        )?;
        cleanup_recoverable_stage_v8(
            &parent,
            &stage_leaf,
            expected_bytes.len(),
            expected_mode,
            expected_owner,
        )?;
        // An exact final name may be the survivor of a crash after rename but
        // before the original parent fsync. Retrying always crosses that
        // durability boundary, even when there is no stale stage to unlink.
        return sync_fd_v8(parent.descriptor.as_raw_fd());
    }

    let mut stage = open_or_create_recoverable_stage_v8(
        &parent,
        &stage_leaf,
        expected_bytes.len(),
        expected_mode,
        expected_owner,
    )?;
    stage.set_len(0)?;
    stage.seek(SeekFrom::Start(0))?;
    stage.write_all(expected_bytes)?;
    stage.set_permissions(fs::Permissions::from_mode(expected_mode))?;
    stage.sync_all()?;
    validate_exact_open_file_v8(
        &mut stage,
        expected_bytes,
        expected_mode,
        expected_owner,
        &format!("{} (staged)", path.display()),
    )?;

    let stage_c = cstring_leaf_v8(&stage_leaf)?;
    let final_c = cstring_leaf_v8(&final_leaf)?;
    // SAFETY: all descriptors and C strings remain live for the syscall.
    let renamed = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            parent.descriptor.as_raw_fd(),
            stage_c.as_ptr(),
            parent.descriptor.as_raw_fd(),
            final_c.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if renamed != 0 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EEXIST) {
            return Err(error.into());
        }
        let mut final_file = try_open_regular_beneath_v8(&parent, &final_leaf, false)?
            .ok_or_else(|| invalid("renameat2 reported EEXIST but final target is absent"))?;
        validate_exact_open_file_v8(
            &mut final_file,
            expected_bytes,
            expected_mode,
            expected_owner,
            &path.display().to_string(),
        )?;
        cleanup_recoverable_stage_v8(
            &parent,
            &stage_leaf,
            expected_bytes.len(),
            expected_mode,
            expected_owner,
        )?;
    }
    sync_fd_v8(parent.descriptor.as_raw_fd())?;
    let mut final_file = try_open_regular_beneath_v8(&parent, &final_leaf, false)?
        .ok_or_else(|| invalid("atomically published install target disappeared"))?;
    validate_exact_open_file_v8(
        &mut final_file,
        expected_bytes,
        expected_mode,
        expected_owner,
        &path.display().to_string(),
    )
}

#[cfg(target_os = "linux")]
fn open_or_create_recoverable_stage_v8(
    parent: &InstallDirectoryAnchorV8,
    leaf: &std::ffi::OsStr,
    maximum_size: usize,
    final_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<File, NativeErrorV8> {
    let leaf_c = cstring_leaf_v8(leaf)?;
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(
        libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
    )
    .map_err(|_| invalid("invalid staged install open flags"))?;
    how.mode = 0o600;
    how.resolve = install_resolve_flags_v8();
    // SAFETY: arguments remain live and a successful descriptor is uniquely owned.
    let raw = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            parent.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    let descriptor = if raw >= 0 {
        let raw = libc::c_int::try_from(raw)
            .map_err(|_| invalid("openat2 returned invalid staged descriptor"))?;
        // SAFETY: openat2 returned a unique owned descriptor.
        unsafe { OwnedFd::from_raw_fd(raw) }
    } else {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EEXIST) {
            return Err(error.into());
        }
        open_regular_beneath_fd_v8(parent, leaf, true)?
    };
    let file = File::from(descriptor);
    require_recoverable_stage_identity_v8(&file, maximum_size, final_mode, expected_owner)?;
    Ok(file)
}

#[cfg(target_os = "linux")]
fn cleanup_recoverable_stage_v8(
    parent: &InstallDirectoryAnchorV8,
    leaf: &std::ffi::OsStr,
    maximum_size: usize,
    final_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let Some(stage) = try_open_regular_beneath_v8(parent, leaf, true)? else {
        return Ok(());
    };
    require_recoverable_stage_identity_v8(&stage, maximum_size, final_mode, expected_owner)?;
    let leaf_c = cstring_leaf_v8(leaf)?;
    // SAFETY: descriptor and leaf remain live; unlinkat retains neither.
    if unsafe { libc::unlinkat(parent.descriptor.as_raw_fd(), leaf_c.as_ptr(), 0) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    sync_fd_v8(parent.descriptor.as_raw_fd())
}

#[cfg(target_os = "linux")]
fn require_recoverable_stage_identity_v8(
    file: &File,
    maximum_size: usize,
    final_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let metadata = file.metadata()?;
    let observed_mode = metadata.mode() & 0o7777;
    if !metadata.file_type().is_file()
        || metadata.uid() != expected_owner.uid
        || metadata.gid() != expected_owner.gid
        || (observed_mode != 0o600 && observed_mode != final_mode)
        || metadata.nlink() != 1
        || metadata.len() > u64::try_from(maximum_size).unwrap_or(u64::MAX)
    {
        return Err(invalid(
            "staged install leaf is not one recoverable root-owned partial file",
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn validate_exact_open_file_v8(
    file: &mut File,
    expected_bytes: &[u8],
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
    label: &str,
) -> Result<(), NativeErrorV8> {
    let before = file.metadata()?;
    if !before.file_type().is_file()
        || before.uid() != expected_owner.uid
        || before.gid() != expected_owner.gid
        || before.mode() & 0o7777 != expected_mode
        || before.nlink() != 1
        || usize::try_from(before.len()).ok() != Some(expected_bytes.len())
    {
        return Err(invalid(format!(
            "install target identity differs from exact policy: {label}"
        )));
    }
    file.seek(SeekFrom::Start(0))?;
    let limit = u64::try_from(expected_bytes.len())
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut observed = Vec::with_capacity(expected_bytes.len());
    (&mut *file).take(limit).read_to_end(&mut observed)?;
    let after = file.metadata()?;
    if observed != expected_bytes
        || before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.len() != after.len()
        || before.mode() != after.mode()
        || before.uid() != after.uid()
        || before.gid() != after.gid()
        || before.nlink() != after.nlink()
        || before.mtime() != after.mtime()
        || before.mtime_nsec() != after.mtime_nsec()
        || before.ctime() != after.ctime()
        || before.ctime_nsec() != after.ctime_nsec()
    {
        return Err(invalid(format!(
            "install target changed during bounded descriptor read: {label}"
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_final_install_inventory_v8(
    plan: &InstallPlanV8,
    artifact_bytes: &[Vec<u8>],
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let state_root = open_absolute_directory_anchored_v8(Path::new(&plan.state_root))?;
    require_exact_directory_v8(&state_root, plan.state_root_mode, expected_owner)?;
    verify_exact_directory_roster_v8(
        Path::new(&plan.state_root),
        &[
            crate::ATTEMPTS_DIRECTORY_V8,
            crate::INSTALL_EPOCH_DIRECTORY_V8,
            crate::JOURNAL_DIRECTORY_V8,
            crate::NONCE_CLAIMS_DIRECTORY_V8,
            crate::QUARANTINE_DIRECTORY_V8,
            crate::STATE_ROOT_LOCK_LEAF_V8,
        ],
    )?;
    let binary_directory = open_absolute_directory_anchored_v8(Path::new(&plan.binary_directory))?;
    require_exact_directory_v8(
        &binary_directory,
        plan.binary_directory_mode,
        expected_owner,
    )?;
    verify_exact_directory_roster_v8(
        Path::new(&plan.binary_directory),
        &[
            "hepta-linux-v8-admissiond",
            "hepta-linux-v8-recover",
            "hepta-linux-v8ctl",
        ],
    )?;
    for directory in &plan.state_directories {
        let opened = open_absolute_directory_anchored_v8(Path::new(&directory.destination_path))?;
        require_exact_directory_v8(&opened, directory.mode, expected_owner)?;
    }
    verify_exact_file_path_v8(
        Path::new(&plan.state_lock.destination_path),
        b"",
        plan.state_lock.mode,
        expected_owner,
    )?;
    for (artifact, bytes) in plan.artifacts.iter().zip(artifact_bytes) {
        verify_exact_file_path_v8(
            Path::new(&artifact.destination_path),
            bytes,
            artifact.mode,
            expected_owner,
        )?;
    }
    for unit in &plan.units {
        verify_exact_file_path_v8(
            Path::new(&unit.destination_path),
            unit_bytes_for_inventory_v8(&unit.name)?,
            unit.mode,
            expected_owner,
        )?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_exact_directory_roster_v8(path: &Path, expected: &[&str]) -> Result<(), NativeErrorV8> {
    let anchor = crate::DirectoryAnchorV8::open(path)?;
    let names = anchor
        .list_leaf_names_bounded(expected.len())?
        .into_iter()
        .map(|name| {
            name.into_string()
                .map_err(|_| invalid("installed directory contains a non-UTF-8 leaf"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = expected
        .iter()
        .map(|name| (*name).to_string())
        .collect::<BTreeSet<_>>();
    if names != expected {
        return Err(invalid(format!(
            "installed directory roster is not exact: {}",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_exact_file_path_v8(
    path: &Path,
    expected_bytes: &[u8],
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let (parent_path, leaf) = absolute_parent_and_leaf_v8(path)?;
    let parent = open_absolute_directory_anchored_v8(&parent_path)?;
    let mut file = try_open_regular_beneath_v8(&parent, &leaf, false)?
        .ok_or_else(|| invalid(format!("installed file disappeared: {}", path.display())))?;
    validate_exact_open_file_v8(
        &mut file,
        expected_bytes,
        expected_mode,
        expected_owner,
        &path.display().to_string(),
    )
}

#[cfg(target_os = "linux")]
fn open_absolute_directory_anchored_v8(
    path: &Path,
) -> Result<InstallDirectoryAnchorV8, NativeErrorV8> {
    let path = absolute_lexical_path_v8(path)?;
    if path == Path::new("/") {
        return Err(invalid("install anchor may not be filesystem root"));
    }
    let root_c = c"/";
    // SAFETY: static path remains live and open retains no pointer.
    let root_raw = unsafe {
        libc::open(
            root_c.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if root_raw < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: successful open returned a unique descriptor.
    let root = unsafe { OwnedFd::from_raw_fd(root_raw) };
    let relative = path
        .strip_prefix(Path::new("/"))
        .map_err(|_| invalid("absolute install anchor is not below root"))?;
    let relative_c = CString::new(relative.as_os_str().as_bytes())
        .map_err(|_| invalid("install anchor contains NUL"))?;
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)
        .map_err(|_| invalid("invalid install directory open flags"))?;
    how.resolve = install_resolve_flags_v8();
    // SAFETY: pointers remain live; successful descriptor is uniquely owned.
    let raw = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            root.as_raw_fd(),
            relative_c.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let raw = libc::c_int::try_from(raw)
        .map_err(|_| invalid("openat2 returned invalid directory descriptor"))?;
    // SAFETY: openat2 returned a unique descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(raw) };
    Ok(InstallDirectoryAnchorV8 {
        descriptor,
        display: path,
    })
}

#[cfg(target_os = "linux")]
fn open_directory_beneath_v8(
    parent: &InstallDirectoryAnchorV8,
    leaf: &std::ffi::OsStr,
) -> Result<InstallDirectoryAnchorV8, NativeErrorV8> {
    let leaf_c = cstring_leaf_v8(leaf)?;
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)
        .map_err(|_| invalid("invalid child directory open flags"))?;
    how.resolve = install_resolve_flags_v8() | libc::RESOLVE_NO_XDEV;
    // SAFETY: arguments remain live and successful descriptor is uniquely owned.
    let raw = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            parent.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let raw = libc::c_int::try_from(raw)
        .map_err(|_| invalid("openat2 returned invalid child directory descriptor"))?;
    // SAFETY: openat2 returned a unique descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(raw) };
    Ok(InstallDirectoryAnchorV8 {
        descriptor,
        display: parent.display.join(leaf),
    })
}

#[cfg(target_os = "linux")]
fn try_open_regular_beneath_v8(
    parent: &InstallDirectoryAnchorV8,
    leaf: &std::ffi::OsStr,
    writable: bool,
) -> Result<Option<File>, NativeErrorV8> {
    match open_regular_beneath_fd_v8(parent, leaf, writable) {
        Ok(descriptor) => Ok(Some(File::from(descriptor))),
        Err(NativeErrorV8::Io(error)) if error.raw_os_error() == Some(libc::ENOENT) => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(target_os = "linux")]
fn open_regular_beneath_fd_v8(
    parent: &InstallDirectoryAnchorV8,
    leaf: &std::ffi::OsStr,
    writable: bool,
) -> Result<OwnedFd, NativeErrorV8> {
    let leaf_c = cstring_leaf_v8(leaf)?;
    let access = if writable {
        libc::O_RDWR
    } else {
        libc::O_RDONLY
    };
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(access | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NONBLOCK)
        .map_err(|_| invalid("invalid install file open flags"))?;
    how.resolve = install_resolve_flags_v8() | libc::RESOLVE_NO_XDEV;
    // SAFETY: pointers remain live; successful descriptor is uniquely owned.
    let raw = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            parent.descriptor.as_raw_fd(),
            leaf_c.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let raw = libc::c_int::try_from(raw)
        .map_err(|_| invalid("openat2 returned invalid install file descriptor"))?;
    // SAFETY: openat2 returned a unique descriptor.
    Ok(unsafe { OwnedFd::from_raw_fd(raw) })
}

#[cfg(target_os = "linux")]
fn require_secure_parent_v8(
    parent: &InstallDirectoryAnchorV8,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let stat = stat_fd_v8(parent.descriptor.as_raw_fd())?;
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR
        || stat.st_uid != expected_owner.uid
        || stat.st_gid != expected_owner.gid
        || stat.st_mode & 0o022 != 0
        || stat.st_nlink == 0
    {
        return Err(invalid(format!(
            "install parent is not a linked root-owned non-writable directory: {}",
            parent.display.display()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_exact_directory_v8(
    directory: &InstallDirectoryAnchorV8,
    expected_mode: u32,
    expected_owner: InstallOwnerV8,
) -> Result<(), NativeErrorV8> {
    let stat = stat_fd_v8(directory.descriptor.as_raw_fd())?;
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR
        || stat.st_uid != expected_owner.uid
        || stat.st_gid != expected_owner.gid
        || stat.st_mode & 0o7777 != expected_mode
        || stat.st_nlink == 0
    {
        return Err(invalid(format!(
            "install directory identity is not root:root mode={expected_mode:04o}: {}",
            directory.display.display()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn stat_fd_v8(fd: libc::c_int) -> Result<libc::stat, NativeErrorV8> {
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: fd is live and stat points to writable storage.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(stat)
}

#[cfg(target_os = "linux")]
fn sync_fd_v8(fd: libc::c_int) -> Result<(), NativeErrorV8> {
    // SAFETY: fsync receives one live descriptor.
    if unsafe { libc::fsync(fd) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn absolute_parent_and_leaf_v8(
    path: &Path,
) -> Result<(PathBuf, std::ffi::OsString), NativeErrorV8> {
    let path = absolute_lexical_path_v8(path)?;
    let parent = path
        .parent()
        .ok_or_else(|| invalid("install target has no parent"))?
        .to_path_buf();
    let leaf = path
        .file_name()
        .ok_or_else(|| invalid("install target has no leaf"))?
        .to_os_string();
    cstring_leaf_v8(&leaf)?;
    Ok((parent, leaf))
}

#[cfg(target_os = "linux")]
fn cstring_leaf_v8(leaf: &std::ffi::OsStr) -> Result<CString, NativeErrorV8> {
    let bytes = leaf.as_bytes();
    if bytes.is_empty() || bytes == b"." || bytes == b".." || bytes.contains(&b'/') {
        return Err(invalid("install path leaf is not one exact component"));
    }
    CString::new(bytes).map_err(|_| invalid("install path leaf contains NUL"))
}

#[cfg(target_os = "linux")]
const fn install_resolve_flags_v8() -> u64 {
    libc::RESOLVE_BENEATH | libc::RESOLVE_NO_SYMLINKS | libc::RESOLVE_NO_MAGICLINKS
}

fn absolute_lexical_path_v8(path: &Path) -> Result<PathBuf, NativeErrorV8> {
    if !path.is_absolute() {
        return Err(invalid("install path must be absolute"));
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(invalid("install path may not contain '..'"));
    }
    let canonical = path.components().collect::<PathBuf>();
    if canonical.as_os_str() != path.as_os_str() {
        return Err(invalid(
            "install path must be a canonical absolute lexical path",
        ));
    }
    Ok(canonical)
}

fn path_utf8_v8(path: &Path) -> Result<&str, NativeErrorV8> {
    path.to_str()
        .ok_or_else(|| invalid("install path is not valid UTF-8"))
}

fn validate_sha256_text_v8(value: &str) -> Result<(), NativeErrorV8> {
    if value.len() == 64
        && !value.bytes().all(|byte| byte == b'0')
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        Ok(())
    } else {
        Err(invalid(
            "SHA-256 must be 64 lowercase hexadecimal characters",
        ))
    }
}

fn validate_boot_id_text_v8(value: &str) -> Result<(), NativeErrorV8> {
    if value.len() != 36 {
        return Err(invalid("boot id is not a canonical lowercase UUID"));
    }
    for (index, byte) in value.as_bytes().iter().copied().enumerate() {
        let separator = matches!(index, 8 | 13 | 18 | 23);
        let invalid_byte = if separator {
            byte != b'-'
        } else {
            !(byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        };
        if invalid_byte {
            return Err(invalid("boot id is not a canonical lowercase UUID"));
        }
    }
    if value
        .as_bytes()
        .iter()
        .copied()
        .filter(|byte| *byte != b'-')
        .all(|byte| byte == b'0')
    {
        return Err(invalid("boot id must not be the nil UUID"));
    }
    Ok(())
}

fn sha256_hex_v8(bytes: &[u8]) -> String {
    let digest = sha2::Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(target_os = "linux")]
fn observe_install_target_filesystem_v8() -> Result<String, NativeErrorV8> {
    const FS_IOC_GETFLAGS_V8: libc::c_ulong = 0x8008_6601;
    let mut binding = b"hepta_linux_v8_install_target_filesystem_v2\0".to_vec();
    for path in [
        "/run",
        "/var",
        "/var/lib",
        "/usr",
        "/usr/local",
        "/usr/local/libexec",
        "/etc",
        "/etc/systemd",
        "/etc/systemd/system",
    ] {
        let anchor = open_absolute_directory_anchored_v8(Path::new(path))?;
        let stat = stat_fd_v8(anchor.descriptor.as_raw_fd())?;
        if stat.st_mode & libc::S_IFMT != libc::S_IFDIR
            || stat.st_uid != 0
            || stat.st_gid != 0
            || stat.st_mode & 0o022 != 0
            || stat.st_nlink == 0
        {
            return Err(invalid(format!(
                "install target ancestor is not linked root:root and non-writable: {path}"
            )));
        }
        // ACLs and other inherited policy are part of the namespace trust
        // boundary. Until an independently published allow-list exists, any
        // xattr is rejected instead of silently inheriting it.
        // SAFETY: flistxattr with a null buffer and zero length is the kernel
        // size query for one live descriptor.
        let xattr_bytes =
            unsafe { libc::flistxattr(anchor.descriptor.as_raw_fd(), std::ptr::null_mut(), 0) };
        if xattr_bytes < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if xattr_bytes != 0 {
            return Err(invalid(format!(
                "install target ancestor has unapproved xattrs: {path}"
            )));
        }
        let mut filesystem: libc::statfs = unsafe { std::mem::zeroed() };
        // SAFETY: filesystem points to writable storage and fd remains live.
        if unsafe { libc::fstatfs(anchor.descriptor.as_raw_fd(), &mut filesystem) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let mut statvfs: libc::statvfs = unsafe { std::mem::zeroed() };
        // SAFETY: statvfs points to writable storage and fd remains live.
        if unsafe { libc::fstatvfs(anchor.descriptor.as_raw_fd(), &mut statvfs) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if statvfs.f_flag & u64::try_from(libc::ST_RDONLY).unwrap_or(u64::MAX) != 0 {
            return Err(invalid(format!(
                "install target filesystem is read-only: {path}"
            )));
        }
        let filesystem_type = filesystem.f_type as u32;
        let inode_flags_allow_mask =
            crate::trusted_inode_flag_allow_mask_v8(filesystem_type).ok_or_else(|| {
                invalid(format!(
                    "install target filesystem type 0x{filesystem_type:08x} has no compiled inode-flag policy: {path}"
                ))
            })?;
        let mut inode_flags: libc::c_int = 0;
        // SAFETY: ioctl receives a live directory descriptor and a writable
        // C int matching the stable FS_IOC_GETFLAGS UAPI.
        if unsafe {
            libc::ioctl(
                anchor.descriptor.as_raw_fd(),
                FS_IOC_GETFLAGS_V8,
                &mut inode_flags,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        let inode_flags = inode_flags as u32;
        if inode_flags & !inode_flags_allow_mask != 0 {
            return Err(invalid(format!(
                "install target ancestor inode flags 0x{inode_flags:08x} exceed filesystem 0x{filesystem_type:08x} allow-mask 0x{inode_flags_allow_mask:08x}: {path}"
            )));
        }
        let mount_id = mount_id_for_fd_v8(anchor.descriptor.as_raw_fd())?;
        // Repeat the empty roster query after all other metadata reads so an
        // ACL/xattr transition during the observation cannot be accepted.
        // SAFETY: null/zero is an exact roster-size query for the live fd.
        let xattr_bytes_after =
            unsafe { libc::flistxattr(anchor.descriptor.as_raw_fd(), std::ptr::null_mut(), 0) };
        if xattr_bytes_after < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if xattr_bytes_after != 0 {
            return Err(invalid(format!(
                "install target ancestor gained an unapproved xattr or ACL: {path}"
            )));
        }
        append_install_binding_field_v8(&mut binding, "path", path.as_bytes());
        append_install_binding_u64_v8(&mut binding, "device", stat.st_dev);
        append_install_binding_u64_v8(&mut binding, "inode", stat.st_ino);
        append_install_binding_u64_v8(&mut binding, "uid", u64::from(stat.st_uid));
        append_install_binding_u64_v8(&mut binding, "gid", u64::from(stat.st_gid));
        append_install_binding_u64_v8(&mut binding, "mode", u64::from(stat.st_mode & 0o7777));
        append_install_binding_u64_v8(&mut binding, "mount_id", mount_id);
        append_install_binding_u64_v8(&mut binding, "filesystem_type", u64::from(filesystem_type));
        append_install_binding_u64_v8(
            &mut binding,
            "inode_flags_allow_mask",
            u64::from(inode_flags_allow_mask),
        );
        append_install_binding_u64_v8(&mut binding, "inode_flags", u64::from(inode_flags));
    }
    Ok(sha256_hex_v8(&binding))
}

#[cfg(target_os = "linux")]
fn mount_id_for_fd_v8(fd: libc::c_int) -> Result<u64, NativeErrorV8> {
    let mut statx: libc::statx = unsafe { std::mem::zeroed() };
    // SAFETY: empty pathname plus AT_EMPTY_PATH binds statx to the live fd;
    // statx points to writable storage and the kernel retains no pointers.
    let result = unsafe {
        libc::statx(
            fd,
            c"".as_ptr(),
            libc::AT_EMPTY_PATH | libc::AT_NO_AUTOMOUNT,
            libc::STATX_BASIC_STATS | libc::STATX_MNT_ID,
            &mut statx,
        )
    };
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if statx.stx_mask & libc::STATX_MNT_ID == 0 || statx.stx_mnt_id == 0 {
        return Err(invalid("install target mount id is unavailable"));
    }
    Ok(statx.stx_mnt_id)
}

#[cfg(target_os = "linux")]
fn append_install_binding_field_v8(bytes: &mut Vec<u8>, name: &str, value: &[u8]) {
    bytes.extend_from_slice(&(name.len() as u64).to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

#[cfg(target_os = "linux")]
fn append_install_binding_u64_v8(bytes: &mut Vec<u8>, name: &str, value: u64) {
    append_install_binding_field_v8(bytes, name, &value.to_be_bytes());
}

#[cfg(target_os = "linux")]
fn observe_install_prerequisites_impl_v8() -> InstallPrerequisitesV8 {
    let machine_id_sha256 = crate::observe_machine_id_v8()
        .map(|observation| observation.machine_id_sha256().to_string())
        .unwrap_or_default();
    let boot_id = crate::observe_boot_id_v8()
        .map(|observation| observation.to_string())
        .unwrap_or_default();
    InstallPrerequisitesV8 {
        target_linux: true,
        machine_id_sha256,
        boot_id,
        cgroup_v2: probe_cgroup_v2_mount_v8(),
        systemd: Path::new("/run/systemd/system").is_dir()
            && fs::read_to_string("/proc/1/comm")
                .map(|name| name.trim() == "systemd")
                .unwrap_or(false),
        openat2: probe_openat2_v8(),
        pidfd_open: probe_pidfd_v8(),
        execveat: probe_execveat_v8(),
        renameat2_noreplace: probe_renameat2_noreplace_v8(),
        target_filesystem_sha256: observe_install_target_filesystem_v8().unwrap_or_default(),
    }
}

#[cfg(not(target_os = "linux"))]
fn observe_install_prerequisites_impl_v8() -> InstallPrerequisitesV8 {
    InstallPrerequisitesV8 {
        target_linux: false,
        machine_id_sha256: String::new(),
        boot_id: String::new(),
        cgroup_v2: false,
        systemd: false,
        openat2: false,
        pidfd_open: false,
        execveat: false,
        renameat2_noreplace: false,
        target_filesystem_sha256: String::new(),
    }
}

#[cfg(target_os = "linux")]
fn probe_cgroup_v2_mount_v8() -> bool {
    let path = c"/sys/fs/cgroup";
    // SAFETY: zero is a valid initial statfs buffer, populated on success.
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: path is a static C string and stat points to writable storage.
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return false;
    }
    stat.f_type == libc::CGROUP2_SUPER_MAGIC
        && Path::new("/sys/fs/cgroup/cgroup.controllers").is_file()
}

#[cfg(target_os = "linux")]
fn probe_openat2_v8() -> bool {
    // SAFETY: all-zero open_how is the kernel baseline and is initialized below.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags =
        u64::try_from(libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC).unwrap_or_default();
    let current = c".";
    // SAFETY: AT_FDCWD and the static C string are valid; the kernel retains no pointers.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            libc::AT_FDCWD,
            current.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if fd < 0 {
        return false;
    }
    let Ok(fd) = libc::c_int::try_from(fd) else {
        return false;
    };
    // SAFETY: a successful syscall returned an owned descriptor used once here.
    unsafe { libc::close(fd) };
    true
}

#[cfg(target_os = "linux")]
fn probe_pidfd_v8() -> bool {
    // SAFETY: getpid has no arguments or preconditions.
    let pid = unsafe { libc::getpid() };
    // SAFETY: scalar-only pidfd_open probe; the returned fd is closed below.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0_u32) };
    if fd < 0 {
        return false;
    }
    let Ok(fd) = libc::c_int::try_from(fd) else {
        return false;
    };
    // SAFETY: a successful syscall returned an owned descriptor used once here.
    unsafe { libc::close(fd) };
    true
}

#[cfg(target_os = "linux")]
fn probe_execveat_v8() -> bool {
    let empty_path = c"";
    let argument = c"hepta-linux-v8-probe";
    let argv = [argument.as_ptr(), std::ptr::null()];
    let envp = [std::ptr::null::<libc::c_char>()];
    // SAFETY: every pointer references live NUL-terminated storage. The
    // deliberately invalid descriptor must be rejected with EBADF before any
    // executable lookup or process-image mutation.
    let result = unsafe {
        libc::syscall(
            libc::SYS_execveat,
            -1_i32,
            empty_path.as_ptr(),
            argv.as_ptr(),
            envp.as_ptr(),
            libc::AT_EMPTY_PATH,
        )
    };
    result == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EBADF)
}

#[cfg(target_os = "linux")]
fn probe_renameat2_noreplace_v8() -> bool {
    let source = c"hepta-linux-v8-probe-source";
    let destination = c"hepta-linux-v8-probe-destination";
    // SAFETY: both pointers reference live NUL-terminated storage. Invalid
    // directory descriptors must be rejected with EBADF before either name is
    // resolved or mutated; the exact flag is therefore exercised harmlessly.
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            -1_i32,
            source.as_ptr(),
            -1_i32,
            destination.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    result == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EBADF)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[cfg(unix)]
    fn all_prerequisites_v8() -> InstallPrerequisitesV8 {
        InstallPrerequisitesV8 {
            target_linux: true,
            machine_id_sha256: "11".repeat(32),
            boot_id: "01234567-89ab-cdef-8123-456789abcdef".to_string(),
            cgroup_v2: true,
            systemd: true,
            openat2: true,
            pidfd_open: true,
            execveat: true,
            renameat2_noreplace: true,
            target_filesystem_sha256: "22".repeat(32),
        }
    }

    #[cfg(unix)]
    fn write_fake_artifacts_v8(directory: &Path) {
        for name in [
            "hepta-linux-v8-admissiond",
            "hepta-linux-v8-recover",
            "hepta-linux-v8ctl",
        ] {
            fs::write(directory.join(name), b"\x7fELF-test-fixture").expect("write fixture");
        }
    }

    #[cfg(unix)]
    #[test]
    fn plan_is_exact_canonical_and_tamper_fails() {
        let temporary = tempfile::tempdir().expect("tempdir");
        write_fake_artifacts_v8(temporary.path());
        let plan = build_install_plan_with_prerequisites_v8(
            temporary.path(),
            1000,
            all_prerequisites_v8(),
        )
        .expect("build plan");
        let bytes = canonical_install_plan_bytes_v8(&plan).expect("canonical plan");
        let digest = install_plan_sha256_v8(&plan).expect("plan digest");
        let parsed =
            parse_fresh_install_plan_v8(&bytes, &digest, 1001).expect("parse exact fresh plan");
        assert_eq!(parsed.plan(), &plan);
        let inventory = exact_install_inventory_v2(&plan).expect("exact install-v2 inventory");
        assert_eq!(
            inventory.ctl_binary.path,
            codex_hepta_linux_qualification_v8::CTL_INSTALL_PATH_V2
        );
        assert_eq!(
            inventory.admissiond_binary.path,
            codex_hepta_linux_qualification_v8::ADMISSIOND_INSTALL_PATH_V2
        );
        assert_eq!(
            inventory.recovery_binary.path,
            codex_hepta_linux_qualification_v8::RECOVERY_INSTALL_PATH_V2
        );
        assert_eq!(
            inventory.state_lock.path,
            codex_hepta_linux_qualification_v8::STATE_LOCK_PATH_V2
        );
        assert_eq!(inventory.state_lock.content_sha256, sha256_hex_v8(b""));

        let mut tampered = bytes.clone();
        let last = tampered.last_mut().expect("non-empty plan");
        *last = b' ';
        assert!(parse_fresh_install_plan_v8(&tampered, &digest, 1001).is_err());
        assert!(parse_fresh_install_plan_v8(&bytes, &digest, 1400).is_err());

        let mut policy_tamper = plan.clone();
        policy_tamper.units[0].mode = 0o600;
        assert!(canonical_install_plan_bytes_v8(&policy_tamper).is_err());
        let mut activation_tamper = plan;
        activation_tamper.activation.start = true;
        assert!(canonical_install_plan_bytes_v8(&activation_tamper).is_err());
    }

    #[test]
    fn non_root_install_is_rejected() {
        assert!(require_root_install_v8(1, 0).is_err());
        assert!(require_root_install_v8(0, 1).is_err());
        assert!(require_root_install_v8(u32::MAX, u32::MAX).is_err());
        assert!(require_root_install_v8(0, 0).is_ok());
    }

    #[test]
    fn install_result_binds_authority_and_cannot_claim_activation() {
        let result = InstallResultV8 {
            schema: INSTALL_RESULT_SCHEMA_V8.to_string(),
            plan_sha256: "11".repeat(32),
            authority_statement_sha256: "22".repeat(32),
            authority_signature_sha256: "33".repeat(32),
            installed_files: 6,
            installed_directories: 7,
            enabled: false,
            started: false,
        };
        assert!(canonical_install_result_bytes_v8(&result).is_ok());
        let mut activated = result;
        activated.started = true;
        assert!(canonical_install_result_bytes_v8(&activated).is_err());
    }

    #[test]
    fn fixed_units_encode_dependency_and_runtime_hardening() {
        assert!(RECOVER_UNIT_V8.contains("Before=hepta-linux-v8-admissiond.service"));
        assert!(ADMISSIOND_UNIT_V8.contains("After=hepta-linux-v8-recover.service"));
        for unit in [RECOVER_UNIT_V8, ADMISSIOND_UNIT_V8] {
            assert!(unit.contains("KillMode=control-group"));
            assert!(unit.contains("UMask=0077"));
            assert!(unit.contains("ProtectSystem=strict"));
            assert!(unit.contains("PrivateTmp=yes"));
            assert!(unit.contains("MemoryDenyWriteExecute=yes"));
        }
        assert!(RECOVER_UNIT_V8.contains("Restart=on-failure"));
        assert!(ADMISSIOND_UNIT_V8.contains("Restart=always"));
        assert!(ADMISSIOND_UNIT_V8.contains("--guardian"));
        assert!(!RECOVER_UNIT_V8.contains("Delegate=yes"));
        assert!(RECOVER_UNIT_V8.contains("ProtectControlGroups=yes"));
        assert!(ADMISSIOND_UNIT_V8.contains("Delegate=yes"));
        assert!(ADMISSIOND_UNIT_V8.contains("ProtectControlGroups=no"));
        assert!(ADMISSIOND_UNIT_V8.contains("Slice=system.slice"));
        assert!(RECOVER_UNIT_V8.contains("/usr/local/libexec/hepta-linux-v8/"));
        assert!(ADMISSIOND_UNIT_V8.contains("/usr/local/libexec/hepta-linux-v8/"));
        assert_eq!(
            crate::ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8,
            "/sys/fs/cgroup/system.slice/hepta-linux-v8-admissiond.service"
        );
        assert_eq!(
            crate::ADMISSIOND_CGROUP_ABSOLUTE_PATH_V8,
            codex_hepta_linux_qualification_v8::ADMISSIOND_SERVICE_CGROUP_PARENT_V2
        );
    }

    #[cfg(unix)]
    #[test]
    fn planned_elf_replacement_is_detected() {
        let temporary = tempfile::tempdir().expect("tempdir");
        write_fake_artifacts_v8(temporary.path());
        let plan = build_install_plan_with_prerequisites_v8(
            temporary.path(),
            1000,
            all_prerequisites_v8(),
        )
        .expect("build plan");
        fs::write(
            temporary.path().join("hepta-linux-v8-recover"),
            b"\x7fELF-replaced-fixture",
        )
        .expect("replace fixture");
        assert!(read_all_artifacts_exact_v8(&plan.artifacts).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn atomic_install_recovers_partial_stage_and_rejects_wrong_final_identity() {
        let temporary = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o700))
            .expect("secure parent mode");
        let owner = current_owner_v8();
        let target = temporary.path().join("target");

        let stage = install_stage_path_v8(&target, b"exact");
        fs::write(&stage, b"ex").expect("write simulated crash partial");
        fs::set_permissions(&stage, fs::Permissions::from_mode(0o600)).expect("partial mode");
        install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner)
            .expect("recover partial and publish");
        assert_eq!(fs::read(&target).expect("installed bytes"), b"exact");
        assert!(!stage.exists());

        fs::write(&stage, b"e").expect("write stale exact-name partial");
        fs::set_permissions(&stage, fs::Permissions::from_mode(0o600)).expect("stale stage mode");
        install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner)
            .expect("exact reentry cleans stale stage");
        assert!(!stage.exists());
        assert!(install_atomic_file_anchored_v8(&target, b"changed", 0o600, owner).is_err());

        let state_lock = temporary.path().join("state.lock");
        install_atomic_file_anchored_v8(&state_lock, b"", 0o600, owner)
            .expect("install empty state lock");
        install_atomic_file_anchored_v8(&state_lock, b"", 0o600, owner)
            .expect("exact empty state-lock retry");
        fs::write(&state_lock, b"replacement").expect("tamper state lock");
        assert!(install_atomic_file_anchored_v8(&state_lock, b"", 0o600, owner).is_err());

        fs::set_permissions(&target, fs::Permissions::from_mode(0o644)).expect("change mode");
        assert!(install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner).is_err());
        fs::set_permissions(&target, fs::Permissions::from_mode(0o600)).expect("restore mode");

        let hardlink = temporary.path().join("hardlink");
        fs::hard_link(&target, &hardlink).expect("create hardlink");
        assert!(install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner).is_err());
        fs::remove_file(&hardlink).expect("remove hardlink");
        fs::remove_file(&target).expect("remove final");

        let victim = temporary.path().join("victim");
        fs::write(&victim, b"victim").expect("write victim");
        std::os::unix::fs::symlink(&victim, &target).expect("create final symlink");
        assert!(install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner).is_err());
        assert_eq!(fs::read(&victim).expect("victim unchanged"), b"victim");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn atomic_install_rejects_unrecoverable_stage_and_insecure_parent() {
        let temporary = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o700))
            .expect("secure parent mode");
        let owner = current_owner_v8();
        let target = temporary.path().join("target");
        let stage = install_stage_path_v8(&target, b"exact");
        fs::write(&stage, b"ex").expect("write partial");
        fs::set_permissions(&stage, fs::Permissions::from_mode(0o644)).expect("wrong stage mode");
        assert!(install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner).is_err());
        assert!(!target.exists());
        assert_eq!(fs::read(&stage).expect("rejected stage retained"), b"ex");

        fs::remove_file(&stage).expect("remove wrong stage");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o720))
            .expect("make parent group-writable");
        assert!(install_atomic_file_anchored_v8(&target, b"exact", 0o600, owner).is_err());
        assert!(!target.exists());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn anchored_directory_creation_accepts_exact_retry_and_rejects_symlinks() {
        let temporary = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o700))
            .expect("secure parent mode");
        let owner = current_owner_v8();
        let directory = temporary.path().join("state");
        ensure_exact_directory_anchored_v8(&directory, 0o700, owner).expect("create directory");
        ensure_exact_directory_anchored_v8(&directory, 0o700, owner).expect("exact retry");
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o755))
            .expect("tamper directory mode");
        assert!(ensure_exact_directory_anchored_v8(&directory, 0o700, owner).is_err());

        let directory_target = temporary.path().join("directory-target");
        std::fs::create_dir(&directory_target).expect("create directory target");
        fs::set_permissions(&directory_target, fs::Permissions::from_mode(0o700))
            .expect("directory target mode");
        let directory_symlink = temporary.path().join("directory-symlink");
        std::os::unix::fs::symlink(&directory_target, &directory_symlink)
            .expect("create directory symlink");
        assert!(ensure_exact_directory_anchored_v8(&directory_symlink, 0o700, owner).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn bounded_anchored_input_rejects_symlinks_hardlinks_fifo_and_oversize() {
        let temporary = tempfile::tempdir().expect("tempdir");
        fs::set_permissions(temporary.path(), fs::Permissions::from_mode(0o700))
            .expect("secure parent mode");
        let input = temporary.path().join("input.json");
        fs::write(&input, b"exact").expect("write input");
        fs::set_permissions(&input, fs::Permissions::from_mode(0o600)).expect("input mode");
        assert_eq!(
            read_bounded_regular_absolute_v8(&input, 5).expect("bounded exact read"),
            b"exact"
        );
        assert!(read_bounded_regular_absolute_v8(&input, 4).is_err());

        let hardlink = temporary.path().join("hardlink.json");
        fs::hard_link(&input, &hardlink).expect("hardlink input");
        assert!(read_bounded_regular_absolute_v8(&input, 5).is_err());
        fs::remove_file(&hardlink).expect("remove hardlink");

        let symlink = temporary.path().join("symlink.json");
        std::os::unix::fs::symlink(&input, &symlink).expect("symlink input");
        assert!(read_bounded_regular_absolute_v8(&symlink, 5).is_err());

        let real_parent = temporary.path().join("real-parent");
        fs::create_dir(&real_parent).expect("real intermediate directory");
        fs::write(real_parent.join("nested.json"), b"nested").expect("nested input");
        let linked_parent = temporary.path().join("linked-parent");
        std::os::unix::fs::symlink(&real_parent, &linked_parent).expect("intermediate symlink");
        assert!(read_bounded_regular_absolute_v8(&linked_parent.join("nested.json"), 6).is_err());

        let fifo = temporary.path().join("input.fifo");
        let fifo_c = CString::new(fifo.as_os_str().as_bytes()).expect("fifo C path");
        // SAFETY: fifo_c is one live NUL-terminated path and mode is scalar.
        assert_eq!(unsafe { libc::mkfifo(fifo_c.as_ptr(), 0o600) }, 0);
        assert!(read_bounded_regular_absolute_v8(&fifo, 5).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn final_directory_roster_is_closed_world() {
        let temporary = tempfile::tempdir().expect("tempdir");
        fs::write(temporary.path().join("a"), b"a").expect("write a");
        fs::write(temporary.path().join("b"), b"b").expect("write b");
        verify_exact_directory_roster_v8(temporary.path(), &["a", "b"]).expect("exact roster");
        fs::write(temporary.path().join("extra"), b"extra").expect("write extra");
        assert!(verify_exact_directory_roster_v8(temporary.path(), &["a", "b"]).is_err());
    }

    #[cfg(target_os = "linux")]
    fn current_owner_v8() -> InstallOwnerV8 {
        // SAFETY: geteuid/getegid have no arguments and no preconditions.
        let uid = unsafe { libc::geteuid() };
        // SAFETY: see above.
        let gid = unsafe { libc::getegid() };
        InstallOwnerV8 { uid, gid }
    }

    #[cfg(target_os = "linux")]
    fn install_stage_path_v8(target: &Path, bytes: &[u8]) -> PathBuf {
        let leaf = target
            .file_name()
            .and_then(|name| name.to_str())
            .expect("UTF-8 target leaf");
        target
            .parent()
            .expect("target parent")
            .join(format!(".{leaf}.{}.installing", sha256_hex_v8(bytes)))
    }
}
