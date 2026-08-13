use super::model::EvidenceProfileV3;
use super::model::ManifestLayerIdV3;
use super::model::ModeManifestFormatV3;

pub(super) const PROFILE_SET: &str = "hepta_vnext_52ec_evidence_profiles_v3_revision_7";

pub(super) const LINUX_TRUST_POLICY_SCHEMA: &str = "hepta_vnext_linux_operator_trust_policy_v7";
pub(super) const LINUX_TRUST_ROOT_ID: &str = "qianqi-existing-github-ed25519-2026";
pub(super) const LINUX_PARENT_TRUST_POLICY_SHA256: &str =
    "7aa71fe6a56a3c5e2bb091bc64e18f2a48f360451b42ad59fb3d7882305f5a49";
pub(super) const LINUX_OPERATOR_PRINCIPAL: &str = "qianqi@hepta-operator";
pub(super) const LINUX_OPERATOR_KEY_FINGERPRINT: &str =
    "SHA256:+eNqmF4lJYlL0besra7M4BSftivEiFsQaTzFkKZKE2E";
pub(super) const LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256: &str =
    "8c87ea612c4c37c8a0c13a1e4bd04d38bcbde49aeaa313e3d181cbecf9eb588d";
pub(super) const LINUX_OPERATOR_ALLOWED_SIGNERS: &str = "qianqi@hepta-operator ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIBzqTTB5U+BgfaVDDRmvdMrdRy/Qu9HTiNTsfG8MMX1b\n";
pub(super) const LINUX_OPERATOR_SIGNATURE_NAMESPACE: &str = "hepta-linux-exact-v5-execution";
pub(super) const LINUX_OPERATOR_ACTION: &str =
    "linux_exact_v5_execute_runner_and_workload_freeze_restore";
pub(super) const LINUX_OPERATOR_AUTHORIZATION_SCOPE: &str =
    "single_linux_exact_v5_direct_launch_runner_and_independent_workload_lifecycle";

pub(super) const GITHUB_PROFILE_SCHEMA: &str = "hepta_vnext_github_hosted_exact_profile_v2";
pub(super) const GITHUB_OUTER_RESULT_SCHEMA: &str =
    "hepta_vnext_github_hosted_exact_outer_result_v2";
pub(super) const GITHUB_PROFILE_NAME: &str = "hepta_vnext_github_hosted_exact_v2";
pub(super) const GITHUB_QUALIFICATION_NONCE: &str = "b79cd07a39b7";
pub(super) const GITHUB_REPOSITORY: &str = "ProfAlexQI/hepta-private-ci";
pub(super) const GITHUB_REPOSITORY_ID: u64 = 1_320_694_176;
pub(super) const GITHUB_WRAPPER_HEAD: &str = "0ef2313ed96751afd64c14d3d28323c1a1454b77";
pub(super) const GITHUB_WRAPPER_TREE: &str = "0d60b09f64a947b15a5f10791ad53bf9724270a3";
pub(super) const GITHUB_WORKFLOW_BLOB: &str = "64f8b0866a59befecb0682abbed07aad1841a932";
pub(super) const GITHUB_WORKFLOW_SHA256: &str =
    "f41d360cd299d5bfe2fc4758e070b2f9959d9022321ef5655b230a7f70a69972";
pub(super) const GITHUB_TRIGGER_REF: &str = "refs/heads/qualification/hosted-52ec-v2-20260813";
pub(super) const GITHUB_TRIGGER_BRANCH: &str = "qualification/hosted-52ec-v2-20260813";

#[derive(Clone, Copy)]
pub(super) struct GithubHostedJobProfileV3 {
    pub artifact_inventory: &'static str,
    pub kind: &'static str,
    pub name: &'static str,
    pub requested_label: &'static str,
    pub runner_os: &'static str,
    pub slug: &'static str,
}

pub(super) const GITHUB_HOSTED_JOBS: [GithubHostedJobProfileV3; 3] = [
    GithubHostedJobProfileV3 {
        artifact_inventory: "FILES.posix.tsv",
        kind: "product",
        name: "Product (ubuntu-24.04)",
        requested_label: "ubuntu-24.04",
        runner_os: "Linux",
        slug: "product-linux",
    },
    GithubHostedJobProfileV3 {
        artifact_inventory: "FILES.ntfs.tsv",
        kind: "product",
        name: "Product (windows-latest)",
        requested_label: "windows-latest",
        runner_os: "Windows",
        slug: "product-windows",
    },
    GithubHostedJobProfileV3 {
        artifact_inventory: "FILES.posix.tsv",
        kind: "generated",
        name: "Generated artifacts and locks (ubuntu-24.04)",
        requested_label: "ubuntu-24.04",
        runner_os: "Linux",
        slug: "generated-linux",
    },
];

pub(super) const MAC_STEPS: [&str; 12] = [
    "preflight",
    "metadata_fmt_diff",
    "script_static_checks",
    "hepta_package_tests",
    "strict_clippy_hepta",
    "product_caller_builds",
    "guardian_baseline_exclusion",
    "upstream_targeted_suites",
    "bazel_validation",
    "freeze_binary",
    "self_tests",
    "postflight",
];

pub(super) const LINUX_STEPS: [&str; 43] = [
    "preflight",
    "locked_metadata",
    "fmt_and_diff",
    "script_static_checks",
    "hepta_package_tests",
    "strict_clippy_hepta",
    "product_caller_builds",
    "strict_clippy_product_callers",
    "guardian_baseline_exclusion",
    "core_steer_input",
    "core_input_queue",
    "core_user_message_admission",
    "core_multi_agent_resume",
    "core_pending_input",
    "core_subagent_notifications",
    "core_mcp_turn_metadata",
    "core_guardian_review",
    "plugin_script_attribution",
    "analytics_plugin_measurement",
    "core_turn_metadata",
    "core_mcp_tool_call",
    "core_review_model",
    "rmcp_test_server_build",
    "core_rmcp_sandbox_meta",
    "login_workload_identity",
    "skills_extension",
    "skills_powershell",
    "core_codex_delegate",
    "core_lazy_mcp",
    "app_server_plugins",
    "tui_sanitizer",
    "core_config_schema",
    "core_config_loader_schema_error",
    "core_config_loader_feature_value",
    "mcp_tool_catalog_cache",
    "config_skills_config",
    "app_server_skills_list",
    "core_mcp_tool_cache",
    "bazel_lock_check",
    "bazel_target_tests",
    "bazel_product_build",
    "freeze_binary",
    "postflight",
];

pub(super) const NIX_STEPS: [&str; 5] = [
    "flake_metadata",
    "flake_check_no_build",
    "default_package_build",
    "output_store_verification",
    "postflight",
];

pub(super) const WINDOWS_STEPS: [&str; 5] = [
    "locked_metadata",
    "bridge_static_contract",
    "portable_hepta_tests",
    "product_caller_compilation",
    "strict_hepta_clippy",
];

pub(super) const MAC_SUITE_COUNTS: [(&str, u64); 29] = [
    ("hepta_packages", 180),
    ("core_steer_input", 5),
    ("core_input_queue", 6),
    ("core_user_message_admission", 7),
    ("core_multi_agent_resume", 1),
    ("core_pending_input", 14),
    ("core_subagent_notifications", 26),
    ("core_mcp_turn_metadata", 10),
    ("core_guardian_review", 9),
    ("plugin_script_attribution", 10),
    ("analytics_plugin_measurement", 2),
    ("core_turn_metadata", 17),
    ("core_mcp_tool_call", 75),
    ("core_review_model", 1),
    ("core_rmcp_sandbox_meta", 4),
    ("login_workload_identity", 8),
    ("skills_extension", 167),
    ("skills_powershell", 2),
    ("core_codex_delegate", 5),
    ("core_lazy_mcp", 4),
    ("app_server_plugins", 138),
    ("tui_sanitizer", 3),
    ("core_config_schema", 3),
    ("core_config_loader_schema_error", 1),
    ("core_config_loader_feature_value", 1),
    ("mcp_tool_catalog_cache", 2),
    ("config_skills_config", 9),
    ("app_server_skills_list", 13),
    ("core_mcp_tool_cache", 6),
];

#[derive(Clone, Copy)]
pub(super) struct LayerProfileV3 {
    pub id: ManifestLayerIdV3,
    pub manifest_path: &'static str,
    pub mode_format: ModeManifestFormatV3,
    pub mode_path: &'static str,
    pub root: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct RequiredArtifactProfileV3 {
    pub layer: ManifestLayerIdV3,
    pub path: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct FrozenReceiptLayerV3 {
    pub entry_count: usize,
    pub manifest_sha256: &'static str,
    pub mode_sha256: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct FrozenReceiptIdentityV3 {
    pub inner: Option<FrozenReceiptLayerV3>,
    pub outer: FrozenReceiptLayerV3,
    pub receipt_root: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct FrozenInventoryIdentityV3 {
    pub row_count: usize,
    pub sha256: &'static str,
    pub size_bytes: usize,
}

#[derive(Clone, Copy)]
pub(super) struct FrozenOriginalIdentityV3 {
    pub entry_count: usize,
    pub extended_metadata_inventory: FrozenInventoryIdentityV3,
    pub hardlink_topology: FrozenInventoryIdentityV3,
    pub manifest_relative_path: &'static str,
    pub manifest_sha256: &'static str,
    pub metadata_inventory: FrozenInventoryIdentityV3,
    pub receipt_root: &'static str,
}

#[derive(Clone, Copy)]
pub(super) struct FrozenPreparedProfileIdentityV3 {
    pub driver_manifest_sha256: &'static str,
    pub driver_mode_sha256: &'static str,
    pub prepared_root: &'static str,
    pub profile_sha256: &'static str,
}

pub(super) const DIRECT_LAYERS: [ManifestLayerIdV3; 1] = [ManifestLayerIdV3::Outer];
pub(super) const NESTED_LAYERS: [ManifestLayerIdV3; 2] =
    [ManifestLayerIdV3::Outer, ManifestLayerIdV3::InnerReceipt];

pub(super) fn gate_profile(gate: &str) -> Option<EvidenceProfileV3> {
    match gate {
        "macos-aarch64" => Some(EvidenceProfileV3::MacExactV6),
        "linux-x86_64" => Some(EvidenceProfileV3::LinuxExactV5),
        "nix-x86_64-linux" => Some(EvidenceProfileV3::NixExactV3),
        "windows-x86_64-native" => Some(EvidenceProfileV3::WindowsNativeV6),
        "github-actions" => Some(EvidenceProfileV3::GithubHostedExactV2),
        _ => None,
    }
}

pub(super) fn prerequisite_profile(id: &str) -> Option<EvidenceProfileV3> {
    match id {
        "portable-inputs" => Some(EvidenceProfileV3::PortableInputsV1),
        "canonical-path-trust" => Some(EvidenceProfileV3::CanonicalPathTrustV2),
        "upstream-cutoff-observation" => Some(EvidenceProfileV3::UpstreamCutoffObservationV1),
        _ => None,
    }
}

pub(super) fn expected_layers(profile: EvidenceProfileV3) -> &'static [ManifestLayerIdV3] {
    match profile {
        EvidenceProfileV3::LinuxExactV5
        | EvidenceProfileV3::NixExactV3
        | EvidenceProfileV3::WindowsNativeV6 => &NESTED_LAYERS,
        EvidenceProfileV3::CanonicalPathTrustV2
        | EvidenceProfileV3::MacExactV6
        | EvidenceProfileV3::PortableInputsV1
        | EvidenceProfileV3::UpstreamCutoffObservationV1
        | EvidenceProfileV3::GithubHostedExactV2 => &DIRECT_LAYERS,
    }
}

pub(super) fn expected_layer_root(layer: ManifestLayerIdV3) -> &'static str {
    match layer {
        ManifestLayerIdV3::Outer => ".",
        ManifestLayerIdV3::InnerReceipt => "receipt",
    }
}

pub(super) fn layer_profile(
    profile: EvidenceProfileV3,
    layer: ManifestLayerIdV3,
) -> Option<LayerProfileV3> {
    use EvidenceProfileV3 as Profile;
    use ManifestLayerIdV3 as Layer;
    use ModeManifestFormatV3 as Modes;

    let direct = |manifest_path, mode_path, mode_format| LayerProfileV3 {
        id: Layer::Outer,
        manifest_path,
        mode_format,
        mode_path,
        root: ".",
    };
    let nested = |id, manifest_path, mode_path, mode_format| LayerProfileV3 {
        id,
        manifest_path,
        mode_format,
        mode_path,
        root: expected_layer_root(id),
    };
    match (profile, layer) {
        (Profile::MacExactV6, Layer::Outer) => Some(direct(
            "SHA256SUMS",
            "MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::PortableInputsV1, Layer::Outer) => Some(direct(
            "INPUTS.sha256",
            "MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::CanonicalPathTrustV2, Layer::Outer)
        | (Profile::UpstreamCutoffObservationV1, Layer::Outer) => Some(direct(
            "SHA256SUMS",
            "MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::LinuxExactV5 | Profile::NixExactV3, Layer::Outer) => Some(nested(
            Layer::Outer,
            "OUTER-SHA256SUMS",
            "OUTER-MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::LinuxExactV5 | Profile::NixExactV3, Layer::InnerReceipt) => Some(nested(
            Layer::InnerReceipt,
            "SHA256SUMS",
            "MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::WindowsNativeV6, Layer::Outer) => Some(nested(
            Layer::Outer,
            "ATTEMPT.sha256",
            "MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        (Profile::WindowsNativeV6, Layer::InnerReceipt) => Some(nested(
            Layer::InnerReceipt,
            "SHA256SUMS",
            "FILES.tsv",
            Modes::WindowsNtfsTypeSizePathTsvV1,
        )),
        (Profile::GithubHostedExactV2, Layer::Outer) => Some(direct(
            "OUTER-SHA256SUMS",
            "OUTER-MODES.tsv",
            Modes::TypedPosixModeSizePathTsvV2,
        )),
        _ => None,
    }
}

pub(super) fn is_unpinned(profile: EvidenceProfileV3) -> bool {
    frozen_receipt_identity(profile).is_none()
}

pub(super) fn authoritative_artifact(
    profile: EvidenceProfileV3,
) -> Option<(ManifestLayerIdV3, &'static str, &'static str)> {
    match profile {
        EvidenceProfileV3::MacExactV6 => Some((
            ManifestLayerIdV3::Outer,
            "qualification-status.txt",
            "hepta_vnext_main_mac_validation_v6",
        )),
        EvidenceProfileV3::LinuxExactV5 => Some((
            ManifestLayerIdV3::InnerReceipt,
            "result.txt",
            "hepta_vnext_linux_exact_result_v3",
        )),
        EvidenceProfileV3::NixExactV3 => Some((
            ManifestLayerIdV3::InnerReceipt,
            "result.txt",
            "hepta_vnext_nix_exact_v3_result_v1",
        )),
        EvidenceProfileV3::WindowsNativeV6 => Some((
            ManifestLayerIdV3::InnerReceipt,
            "result.json",
            "hepta_vnext_windows_native_qualification_v4",
        )),
        EvidenceProfileV3::PortableInputsV1 => Some((
            ManifestLayerIdV3::Outer,
            "GENERATION-COMPLETE.txt",
            "hepta_vnext_portable_generation_v1",
        )),
        EvidenceProfileV3::CanonicalPathTrustV2 => Some((
            ManifestLayerIdV3::Outer,
            "status.txt",
            "hepta_vnext_canonical_path_trust_v2",
        )),
        EvidenceProfileV3::UpstreamCutoffObservationV1 => Some((
            ManifestLayerIdV3::Outer,
            "upstream-cutoff.txt",
            "hepta_vnext_upstream_cutoff_observation_v1",
        )),
        EvidenceProfileV3::GithubHostedExactV2 => Some((
            ManifestLayerIdV3::Outer,
            "OUTER-RESULT.json",
            "hepta_vnext_github_hosted_exact_outer_result_v2",
        )),
    }
}

pub(super) fn outer_verification_artifact(
    profile: EvidenceProfileV3,
) -> Option<(&'static str, &'static str)> {
    match profile {
        EvidenceProfileV3::LinuxExactV5 => Some((
            "LOCAL-VERIFICATION.txt",
            "hepta_vnext_linux_local_verification_v3",
        )),
        EvidenceProfileV3::NixExactV3 => Some((
            "LOCAL-VERIFICATION.txt",
            "hepta_vnext_nix_exact_v3_local_verification_v1",
        )),
        EvidenceProfileV3::WindowsNativeV6 => Some((
            "verification-result.txt",
            "hepta_vnext_windows_native_outer_verification_v3",
        )),
        EvidenceProfileV3::GithubHostedExactV2 => None,
        _ => None,
    }
}

pub(super) fn required_artifacts(
    profile: EvidenceProfileV3,
) -> &'static [RequiredArtifactProfileV3] {
    use ManifestLayerIdV3 as Layer;
    use RequiredArtifactProfileV3 as Artifact;

    match profile {
        EvidenceProfileV3::MacExactV6 => &[
            Artifact {
                layer: Layer::Outer,
                path: "run-mac-exact-v6.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "qualification-status.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "steps.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "test-suite-counts.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "binary.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "hepta",
            },
            Artifact {
                layer: Layer::Outer,
                path: "canary-result.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "mac-exact.log",
            },
        ],
        EvidenceProfileV3::LinuxExactV5 => &[
            Artifact {
                layer: Layer::Outer,
                path: "LOCAL-VERIFICATION.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/DRIVER-SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/DRIVER-MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/run-linux-exact-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/launch-linux-exact-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/verify-host-tools-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/verify-and-seal-copied-receipt-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/finalize-sealed-driver-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/lib-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/.capture-host-tools-remote-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/capture-host-tools-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/STATIC-TESTS.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/expected-step-names.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/expected-suite-names.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/expected-suite-counts.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-tool-roster.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/OBSERVATION.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/HOST-TOOLS.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/HOST-EXECUTABLES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-observation/host-tool-roster.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/NIX-PASS-BINDING.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/NIX-ATTEMPT-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/NIX-ATTEMPT-FULL-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/HOST-TOOLS.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/HOST-EXECUTABLES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/tool-input-binding.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/sanitized-environment-v5.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/resource-watchdog-filter-v5.awk",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/resource-watchdog-static-test-v5.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/verify-resource-watchdog-v5.py",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/acceptance-v7-linux-v5-contract.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/EXECUTION-MODE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-trust-policy/trust-policy.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-trust-policy/allowed_signers",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-trust-policy/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-trust-policy/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/AUTHORITY.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/CHALLENGE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/CHALLENGE.txt.sig",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/ALLOWED-SIGNERS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/operator-authority/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/PROFILE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/NIX-BINDING.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/RUNNER-FREEZE-BINDING.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/WORKLOAD-FREEZE-BINDING.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/host-lock-profile/INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/RUNNER-FREEZE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/RUNNER-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/SERVICE-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/RESTORE-PLAN.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/runner-freeze/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/WORKLOAD-FREEZE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/WORKLOAD-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/WORKLOAD-RESTORE-PLAN.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/NIX-PASS-BINDING.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "driver/workload-freeze/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "execution-authorization/EXECUTION-AUTHORIZATION.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "execution-authorization/EXECUTION-AUTHORIZATION.txt.sig",
            },
            Artifact {
                layer: Layer::Outer,
                path: "execution-authorization/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "execution-authorization/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "legacy-production-preflight.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "legacy-production-postflight.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/RUNNER-RESTORE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/PRE-RESTORE-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/POST-RESTORE-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/SERVICE-RESTORE.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "runner-restore/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/WORKLOAD-RESTORE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/PRE-RESTORE-WORKLOADS.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/POST-RESTORE-WORKLOADS.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/WORKLOAD-RESTORE.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "workload-restore/MODES.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "result.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "steps.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "RUN-STARTED.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "RUN-COMPLETED.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "run-linux-exact-v5.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "lib-v5.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "verify-host-tools-v5.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-suite-counts.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "expected-suite-counts.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "binary.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "hepta",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "environment.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "sanitized-environment-v5.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "sanitized-bootstrap-environment.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "sanitized-build-environment.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "resource-watchdog-filter-v5.awk",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "resource-watchdog-observations.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "resource-watchdog-verification.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "HOST-QUALIFICATION-LOCK.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "verify-resource-watchdog-v5.py",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "bazel-result.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "resource-preflight.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-exclusivity-preflight.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-exclusivity-postflight.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "preflight-git-tree.manifest",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "postflight-git-tree.manifest",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "preflight-git-blobs.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "postflight-git-blobs.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "input-verification.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "tool-input-verification.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "vendor-verification.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "toolchain-binaries.sha256",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-hepta_packages.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_steer_input.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_input_queue.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_user_message_admission.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_multi_agent_resume.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_pending_input.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_subagent_notifications.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_mcp_turn_metadata.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_guardian_review.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-plugin_script_attribution.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-analytics_plugin_measurement.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_turn_metadata.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_mcp_tool_call.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_review_model.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_rmcp_sandbox_meta.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-login_workload_identity.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-skills_extension.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-skills_powershell.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_codex_delegate.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_lazy_mcp.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-app_server_plugins.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-tui_sanitizer.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_config_schema.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_config_loader_schema_error.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_config_loader_feature_value.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-mcp_tool_catalog_cache.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-config_skills_config.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-app_server_skills_list.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-core_mcp_tool_cache.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "NIX-PASS-BINDING.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "NIX-ATTEMPT-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "NIX-ATTEMPT-FULL-INVENTORY.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "HOST-TOOLS.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "HOST-EXECUTABLES.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-tools-preflight.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-tools-postflight.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-executables-preflight.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "host-executables-postflight.tsv",
            },
        ],
        EvidenceProfileV3::NixExactV3 => &[
            Artifact {
                layer: Layer::Outer,
                path: "LOCAL-VERIFICATION.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "result.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "steps.tsv",
            },
        ],
        EvidenceProfileV3::WindowsNativeV6 => &[
            Artifact {
                layer: Layer::Outer,
                path: "verification-result.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "launcher-result.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "result.json",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "steps.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "DRIVERS.sha256",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "INPUTS.sha256",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "input-verification.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "driver-verification.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "run-windows-exact.ps1",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "run-windows-exact.cmd",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "run-candidate-gates.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "launch-via-x230.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "verify-and-seal-attempt.sh",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "candidate-execution-started.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "candidate-execution-completed.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "locked_metadata.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "bridge_static_contract.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "portable_hepta_tests.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "product_caller_compilation.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "strict_hepta_clippy.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "test-suite-counts.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "expected-git-tree.manifest",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "expected-git-blobs.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "preflight.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "preflight-git-tree.manifest",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "preflight-windows-materialized-git-blobs.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "postflight.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "postflight-git-tree.manifest",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "postflight-windows-materialized-git-blobs.tsv",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "resource-preflight.json",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "toolchain-exactness.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "rust-toolchain-source-files.sha256",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "rust-toolchain-fresh-files.sha256",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "tool-inventory.json",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "git-capture-self-test.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "environment-sanitization.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-self-test.txt",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-success-stderr.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-success-stderr.log.stdout.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-success-stderr.log.stderr.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-nonzero.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-nonzero.log.stdout.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-nonzero.log.stderr.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-invocation-error.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-invocation-error.log.stdout.log",
            },
            Artifact {
                layer: Layer::InnerReceipt,
                path: "native-capture-invocation-error.log.stderr.log",
            },
        ],
        EvidenceProfileV3::PortableInputsV1 => &[
            Artifact {
                layer: Layer::Outer,
                path: "GENERATION-COMPLETE.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "candidate-binding.txt",
            },
        ],
        EvidenceProfileV3::CanonicalPathTrustV2 => &[
            Artifact {
                layer: Layer::Outer,
                path: "verify-and-seal.sh",
            },
            Artifact {
                layer: Layer::Outer,
                path: "status.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "t5-volume-info.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "canonical-worktree-status.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "local-canonical-ref.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "remote-candidate-preservation-refs.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "remote-head.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "remote-refs.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "codex-trust-bindings.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "agent-path-instructions.txt",
            },
        ],
        EvidenceProfileV3::UpstreamCutoffObservationV1 => &[Artifact {
            layer: Layer::Outer,
            path: "upstream-cutoff.txt",
        }],
        EvidenceProfileV3::GithubHostedExactV2 => &[
            Artifact {
                layer: Layer::Outer,
                path: "OUTER-RESULT.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "PROFILE.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "capture-started-at.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "capture-completed-at.txt",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/PRE-OBSERVATION.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/SHA256SUMS",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/PROFILE.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/DRIVERS.sha256",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/DRIVER-MODES.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "pre-observation/refs.full.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/run.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/jobs.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/check-suite.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/check-runs.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/artifacts.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/workflow.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/repository.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/actions-permissions.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/wrapper-commit.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/candidate-commit.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/trigger-ref.json",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/workflow.yml",
            },
            Artifact {
                layer: Layer::Outer,
                path: "api/refs.full.post.tsv",
            },
            Artifact {
                layer: Layer::Outer,
                path: "logs/run-logs.http",
            },
            Artifact {
                layer: Layer::Outer,
                path: "logs/run-logs.zip",
            },
        ],
    }
}

pub(super) fn direct_provenance_allowed(profile: EvidenceProfileV3) -> bool {
    !matches!(
        profile,
        EvidenceProfileV3::MacExactV6
            | EvidenceProfileV3::NixExactV3
            | EvidenceProfileV3::PortableInputsV1
    )
}

pub(super) fn frozen_receipt_identity(
    profile: EvidenceProfileV3,
) -> Option<FrozenReceiptIdentityV3> {
    let layer = |entry_count, manifest_sha256, mode_sha256| FrozenReceiptLayerV3 {
        entry_count,
        manifest_sha256,
        mode_sha256,
    };
    match profile {
        EvidenceProfileV3::MacExactV6 => Some(FrozenReceiptIdentityV3 {
            inner: None,
            outer: layer(
                236,
                "1bc706ba581e9b1498ff65890ca429fb9fd328ebe0f4b103657d9d02a2fef10b",
                "c6b12f4e161fd4a350fc51c619a8a845bef513c7a1785b2a65c82ad7199e697a",
            ),
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-mac-exact-reemitted-rev7-prepared-20260813T182053Z",
        }),
        EvidenceProfileV3::PortableInputsV1 => Some(FrozenReceiptIdentityV3 {
            inner: None,
            outer: layer(
                49,
                "2e8f8653e69202471ce52f4ca8b80fa0bf7df15b692378945c553278644a9221",
                "e5f4ba0ad6fdd60b013403659a8fd1e26180b1d3ffc08746d4c92114eff61ea7",
            ),
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-portable-inputs-reemitted-rev7-prepared-20260813T182053Z",
        }),
        EvidenceProfileV3::NixExactV3 => Some(FrozenReceiptIdentityV3 {
            inner: Some(layer(
                103,
                "24e6bf9b8b5bd0134b01ea044582570d45a2085cf019fab43dc3c139b1a45a27",
                "5f6be3e09d9373ba794ea6456d071e8f00716c1a4107ae3e7bbc1f4e37f3d7cd",
            )),
            outer: layer(
                239,
                "f81c84fe01076307c80816914d696cf2a2b234b90847c6294b0e283d2ba55ab2",
                "c2ed5d64444054d8ec52fe04f511bdd58c86ab6481cfcf79c71dc70a5bbb9012",
            ),
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-reemitted-rev7-prepared-20260813T185012Z",
        }),
        EvidenceProfileV3::CanonicalPathTrustV2 => Some(FrozenReceiptIdentityV3 {
            inner: None,
            outer: layer(
                15,
                "2e6501052ef60331593e5bab9950e2329ab0f14b343d63df5ed2d50b28c9271d",
                "7928b302e58c8a512c93c64d44abfddbd79483a34530bff6fda642503f04c81c",
            ),
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-canonical-path-trust-v3-attempt-4-20260813T114508Z",
        }),
        EvidenceProfileV3::UpstreamCutoffObservationV1 => Some(FrozenReceiptIdentityV3 {
            inner: None,
            outer: layer(
                2,
                "31fe6e9d2ae57ae6f0bac8561ad51d90e8d1beba558f2158e541b83b38ad35b0",
                "e7dd8530f92d5ccfb9f19440046902df1c74a731a4a4d9d5eacab3e88837dd6d",
            ),
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-upstream-cutoff-observation-v3-20260813T112118Z",
        }),
        _ => None,
    }
}

pub(super) fn frozen_github_prepared_profile_identity() -> FrozenPreparedProfileIdentityV3 {
    FrozenPreparedProfileIdentityV3 {
        driver_manifest_sha256: "5d18f9923410cd24c0b213bf278b725108086c5e571b5cf489f3c84c5b5b3784",
        driver_mode_sha256: "f9490fe2ae720fe610b852eed1be5bf4c91fc17e74c9cc833c31ab1799058b17",
        prepared_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-github-hosted-exact-v2-prepared-20260813T174255Z",
        profile_sha256: "e9c825e8936577ed1d975df225a7a38a8aa89a21d97230c969b55e8473ef0505",
    }
}

pub(super) fn frozen_linux_driver_identity() -> Option<(&'static str, &'static str)> {
    // The v5 acceptance contract and required-artifact roster are compiled, but
    // the prepared driver still declares WIP_NO_GO/implementation_compatible=false
    // and has no independently sealed terminal driver identity. Do not invent a
    // digest here: execution and aggregate admission remain fail-closed.
    None
}

pub(super) fn frozen_original_identity(
    profile: EvidenceProfileV3,
) -> Option<FrozenOriginalIdentityV3> {
    match profile {
        EvidenceProfileV3::MacExactV6 => Some(FrozenOriginalIdentityV3 {
            entry_count: 114,
            extended_metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 4,
                sha256: "7f2a66f797b273e3332e0d9f26e8290672f025433b587df9cd0a560875b89f76",
                size_bytes: 658,
            },
            hardlink_topology: FrozenInventoryIdentityV3 {
                row_count: 2,
                sha256: "9efaf5f7b046c35ce88af776e723204407514629f96cb8a58bc26e255525c886",
                size_bytes: 220,
            },
            manifest_relative_path: "SHA256SUMS",
            manifest_sha256: "824b5158028fd2d171c7f9b427bc33455705e4d994432926a11d199c02313ca0",
            metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 134,
                sha256: "dbb5dacee129e66dba8f8be7f51979db79ab95d9c02e7323c7750c388aeef4d0",
                size_bytes: 7_674,
            },
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-mac-exact-attempt-2-20260813T052744Z",
        }),
        EvidenceProfileV3::PortableInputsV1 => Some(FrozenOriginalIdentityV3 {
            entry_count: 21,
            extended_metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 0,
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                size_bytes: 0,
            },
            hardlink_topology: FrozenInventoryIdentityV3 {
                row_count: 0,
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                size_bytes: 0,
            },
            manifest_relative_path: "INPUTS.sha256",
            manifest_sha256: "65e9f7e70294c44b5f8e79881af6eebacdcb428452fd47bd8f5dcc54f4fd4bda",
            metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 23,
                sha256: "607d6fb685563e48e9048340115b85210c26aa5aaa1112b3ba91fde005d2207f",
                size_bytes: 1_058,
            },
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-portable-inputs-20260813T051449Z",
        }),
        EvidenceProfileV3::NixExactV3 => Some(FrozenOriginalIdentityV3 {
            entry_count: 116,
            extended_metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 0,
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                size_bytes: 0,
            },
            hardlink_topology: FrozenInventoryIdentityV3 {
                row_count: 0,
                sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
                size_bytes: 0,
            },
            manifest_relative_path: "OUTER-SHA256SUMS",
            manifest_sha256: "55a041bbdaf4bf31d676f20c80fe07737dc5d33f0d7d3dfaed26639e57db93a2",
            metadata_inventory: FrozenInventoryIdentityV3 {
                row_count: 121,
                sha256: "1f5b61ffb693f6eca838abe782668aa291d0d8ab91d21ae36e22514a5a6c7cbd",
                size_bytes: 6_423,
            },
            receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-v3-attempt-3-20260813T065739Z/attempt-52ec08130755",
        }),
        _ => None,
    }
}

pub(super) fn frozen_windows_driver_identity()
-> Option<(&'static str, [(&'static str, &'static str); 5])> {
    // v3/v4/v5, old v6, v6-r2, and v6-r3 are incompatible. v6-r4 fixed
    // ordinal inventory ordering but emits fractional UTC timestamps, which
    // cannot satisfy the exact-second evidence contract. The independently
    // sealed r5 identity below is retained only as an audited static identity:
    // it is not authorized for formal execution because its x230 host boundary
    // lacks the required shared lock and host-wide concurrency evidence. A
    // later driver revision must receive its own independently frozen identity.
    // Fail closed now: retaining the r5 digest as an executable profile would
    // let an accidentally executed r5 PASS enter the aggregate despite the
    // missing host-lock and runner pause/restore boundary.
    None
}
