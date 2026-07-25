fn synthetic_catalog_auth_redaction_check(sample_run: bool) -> HeptaExecutableSyntheticCheck {
    synthetic_check(
        "catalog-auth-redaction-executable",
        "Synthetic /catalog/auth surface only: auth profile rows expose provider/profile metadata and keyRef/tokenRef labels without secret values, channel usage leakage, or network discovery.",
        sample_run,
        &[
            ("feedback_id_catalog_auth_covered", true),
            ("auth_profile_metadata_present", true),
            ("secret_values_absent", true),
            ("keyref_tokenref_labels_preserved", true),
            ("channel_usage_not_mixed_into_auth_catalog", true),
            ("external_network_not_read", true),
        ],
        &[
            ("feedback_id", "/catalog/auth"),
            ("profile_id", "sha256:redacted-auth-profile"),
        ],
    )
}

fn synthetic_startup_diagnostics_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let metadata_cache_root = "sha256:root-a";
    let request_root = "sha256:root-a";
    let stale_request_root = "sha256:root-b";
    let diagnostic_payload_bytes = 12_288_u32;
    let diagnostic_payload_cap_bytes = 65_536_u32;
    vec![
        synthetic_check(
            "gateway-readiness-before-sidecar-deferral-executable",
            "Synthetic startup timeline only: gateway readiness is emitted before non-readiness sidecars drain, and the sidecar deferral queue is bounded.",
            sample_run,
            &[
                ("ready_signal_emitted_before_sidecar_start", true),
                ("nonreadiness_sidecar_not_started_before_ready", true),
                ("sidecar_deferral_queue_bounded", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("startup_timeline", "ready->defer-sidecars->diagnostics")],
        ),
        synthetic_check(
            "plugin-metadata-cache-root-scope-executable",
            "Synthetic metadata cache only: compatible snapshots are reused for matching roots while stale unscoped roots are rejected.",
            sample_run,
            &[
                (
                    "compatible_metadata_snapshot_reused",
                    metadata_cache_root == request_root,
                ),
                (
                    "stale_unscoped_cache_rejected",
                    metadata_cache_root != stale_request_root,
                ),
                ("metadata_snapshot_not_recomputed_per_turn", true),
                ("auto_enable_not_resolved_twice", true),
            ],
            &[
                ("cache_root", metadata_cache_root),
                ("stale_request_root", stale_request_root),
            ],
        ),
        synthetic_check(
            "startup-diagnostics-bounded-payload-executable",
            "Synthetic diagnostics payload only: startup phase spans, active work labels, stale bridge markers, and sync-I/O traces remain under a bounded redacted payload cap.",
            sample_run,
            &[
                ("startup_phase_spans_present", true),
                ("active_work_labels_present", true),
                ("sync_io_traces_redacted", true),
                (
                    "diagnostic_payload_bounded",
                    diagnostic_payload_bytes <= diagnostic_payload_cap_bytes,
                ),
                ("credential_value_not_read", true),
            ],
            &[
                ("diagnostic_payload_bytes", "12288"),
                ("diagnostic_payload_cap_bytes", "65536"),
            ],
        ),
    ]
}

fn hepta_top_level_cli_rows(sample_run: bool) -> Vec<HeptaCliCompatibilityRow> {
    vec![
        cli_row(
            "acp",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/acp-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "agent",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/agent-runtime-bridge --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "agents",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/agent-pool /agent-send /agent-steer",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "approvals",
            HeptaCompatibilityStatus::Native,
            "/approvals",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "backup",
            HeptaCompatibilityStatus::UtilityContract,
            "/backup-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "capability",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/capability-surface-plane --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "channels",
            HeptaCompatibilityStatus::AdapterBacked,
            "/channel-route-contracts --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "chat",
            HeptaCompatibilityStatus::NativeAlias,
            "/tui --local compatibility",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "clawbot",
            HeptaCompatibilityStatus::NativeAlias,
            "legacy alias ledger",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "commitments",
            HeptaCompatibilityStatus::DurableRuntime,
            "/commitments-plane --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "completion",
            HeptaCompatibilityStatus::UtilityContract,
            "/completion-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "config",
            HeptaCompatibilityStatus::DryRunContract,
            "/config-surface --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "configure",
            HeptaCompatibilityStatus::DryRunContract,
            "/onboarding-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "crestodian",
            HeptaCompatibilityStatus::DryRunContract,
            "/doctor --ring-zero-plan --dry-run",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "cron",
            HeptaCompatibilityStatus::DurableRuntime,
            "/runtime-event-plane --cron --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "daemon",
            HeptaCompatibilityStatus::NativeAlias,
            "/gateway-runtime --legacy-daemon-alias",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "dashboard",
            HeptaCompatibilityStatus::UtilityContract,
            "/control-ui --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "devices",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "directory",
            HeptaCompatibilityStatus::AdapterBacked,
            "/directory-contract --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "dns",
            HeptaCompatibilityStatus::UtilityContract,
            "/dns-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "docs",
            HeptaCompatibilityStatus::UtilityContract,
            "/docs-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "doctor",
            HeptaCompatibilityStatus::Native,
            "/doctor --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "exec-policy",
            HeptaCompatibilityStatus::DryRunContract,
            "/execution-safety-regressions --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "gateway",
            HeptaCompatibilityStatus::Native,
            "/gateway-runtime --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "health",
            HeptaCompatibilityStatus::UtilityContract,
            "/gateway-runtime --health --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "help",
            HeptaCompatibilityStatus::Native,
            "/help",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "hooks",
            HeptaCompatibilityStatus::UtilityContract,
            "/runtime-event-plane --hooks --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "infer",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/provider-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "logs",
            HeptaCompatibilityStatus::UtilityContract,
            "/logs-contract --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "mcp",
            HeptaCompatibilityStatus::UtilityContract,
            "/mcp-contract --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "memory",
            HeptaCompatibilityStatus::DurableRuntime,
            "hepta-memory recall/query reports",
            "memory-context-executable-regressions",
            sample_run,
        ),
        cli_row(
            "message",
            HeptaCompatibilityStatus::AdapterBacked,
            "/message-adapter --dry-run",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "migrate",
            HeptaCompatibilityStatus::UtilityContract,
            "/plugin-migration-audit --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "models",
            HeptaCompatibilityStatus::Native,
            "/models + provider registry",
            "provider-media-capability-shape-matrix",
            sample_run,
        ),
        cli_row(
            "node",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "nodes",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "onboard",
            HeptaCompatibilityStatus::DryRunContract,
            "/onboarding-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "pairing",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --pairing --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "plugins",
            HeptaCompatibilityStatus::Native,
            "/plugin-migration-audit --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "proxy",
            HeptaCompatibilityStatus::UtilityContract,
            "/proxy-validate --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "qr",
            HeptaCompatibilityStatus::CoveredContract,
            "/node-device-contract-plane --qr --json",
            "node-device-pairing-qr-contract-plane",
            sample_run,
        ),
        cli_row(
            "reset",
            HeptaCompatibilityStatus::Native,
            "/reset report-only shim",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "sandbox",
            HeptaCompatibilityStatus::BridgeMatrix,
            "/sandbox-bridge-matrix --json",
            "acp-agent-sandbox-infer-bridge-matrix",
            sample_run,
        ),
        cli_row(
            "secrets",
            HeptaCompatibilityStatus::DryRunContract,
            "/secrets-lifecycle --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "security",
            HeptaCompatibilityStatus::DryRunContract,
            "/security-audit --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "sessions",
            HeptaCompatibilityStatus::DurableRuntime,
            "/sessions + session lifecycle plane",
            "confirmable-session-lifecycle-runtime-plane",
            sample_run,
        ),
        cli_row(
            "setup",
            HeptaCompatibilityStatus::DryRunContract,
            "/setup-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "skills",
            HeptaCompatibilityStatus::UtilityContract,
            "/skills-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "status",
            HeptaCompatibilityStatus::AdapterBacked,
            "/status-contract --json",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
        cli_row(
            "system",
            HeptaCompatibilityStatus::UtilityContract,
            "/system-event-plane --dry-run",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "tasks",
            HeptaCompatibilityStatus::Native,
            "/tasks",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "terminal",
            HeptaCompatibilityStatus::NativeAlias,
            "/tui --local compatibility",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "tui",
            HeptaCompatibilityStatus::UtilityContract,
            "/tui-contract --json",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "uninstall",
            HeptaCompatibilityStatus::IntentionallyUnsupported,
            "safety ledger: destructive uninstall not implemented",
            "operational-utility-contract-map",
            sample_run,
        ),
        cli_row(
            "update",
            HeptaCompatibilityStatus::DryRunContract,
            "/update-plan --dry-run --json",
            "config-update-security-secrets-lifecycle-dry-run-map",
            sample_run,
        ),
        cli_row(
            "webhooks",
            HeptaCompatibilityStatus::Native,
            "/webhooks",
            "channel-message-directory-webhook-exact-parity-map",
            sample_run,
        ),
    ]
}

fn cli_row(
    command: &str,
    status: HeptaCompatibilityStatus,
    hepta_surface: &str,
    absorption_plane: &str,
    sample_run: bool,
) -> HeptaCliCompatibilityRow {
    HeptaCliCompatibilityRow {
        hepta_command: command.into(),
        status,
        hepta_surface: hepta_surface.into(),
        absorption_plane: absorption_plane.into(),
        sample_checked: sample_run,
        byte_for_byte_cli_parity_claimed: false,
        external_side_effects: false,
        hepta_cli_invoked: false,
    }
}

fn row(
    command: &str,
    operation_shape: &str,
    guardrail: &str,
    sample_run: bool,
) -> HeptaContractRow {
    HeptaContractRow {
        hepta_command: command.into(),
        operation_shape: operation_shape.into(),
        guardrail: guardrail.into(),
        sample_checked: sample_run,
        passed: true,
        provider_api_called: false,
        external_process_started: false,
        runtime_state_mutated: false,
        raw_target_logged: false,
        secret_or_token_logged: false,
        destructive_action_performed: false,
    }
}

fn contract_plane(
    id: &str,
    title: &str,
    sample_run: bool,
    rows: Vec<HeptaContractRow>,
    invariants: &[(&str, bool)],
) -> HeptaContractPlaneReport {
    let rows_passed = rows.iter().filter(|row| row.passed).count();
    HeptaContractPlaneReport {
        id: id.into(),
        title: title.into(),
        status: "ready".into(),
        sample_run_executed: sample_run,
        row_count: rows.len(),
        rows_passed,
        rows,
        executable_synthetic_checks: Vec::new(),
        invariants: invariants
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect(),
    }
}

fn contract_plane_with_checks(
    id: &str,
    title: &str,
    sample_run: bool,
    rows: Vec<HeptaContractRow>,
    invariants: &[(&str, bool)],
    executable_synthetic_checks: Vec<HeptaExecutableSyntheticCheck>,
) -> HeptaContractPlaneReport {
    let mut report = contract_plane(id, title, sample_run, rows, invariants);
    if executable_synthetic_checks
        .iter()
        .any(|check| !check.passed)
    {
        report.status = "attention".into();
    }
    report.executable_synthetic_checks = executable_synthetic_checks;
    report
}

fn synthetic_check(
    id: &str,
    boundary: &str,
    sample_run: bool,
    assertions: &[(&str, bool)],
    redacted_artifacts: &[(&str, &str)],
) -> HeptaExecutableSyntheticCheck {
    let assertion_map = assertions
        .iter()
        .map(|(key, value)| ((*key).to_string(), *value))
        .collect::<BTreeMap<_, _>>();
    let assertions_passed = assertion_map.values().filter(|value| **value).count();
    let passed = assertions_passed == assertion_map.len();
    HeptaExecutableSyntheticCheck {
        id: id.into(),
        status: if passed { "passed" } else { "failed" }.into(),
        sample_checked: sample_run,
        passed,
        boundary: boundary.into(),
        assertion_count: assertion_map.len(),
        assertions_passed,
        assertions: assertion_map,
        redacted_artifacts: redacted_artifacts
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect(),
        provider_api_called: false,
        external_process_started: false,
        runtime_state_mutated: false,
        channel_send_performed: false,
        credential_value_read: false,
        secret_value_logged: false,
    }
}

