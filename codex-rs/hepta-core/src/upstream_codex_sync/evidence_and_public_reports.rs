
impl HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport {
    pub fn native_default() -> Self {
        let denied_sample = HeptaUpstreamCodexActivationDeniedSampleReport::native_default();
        let freshness_entries = default_activation_evidence_freshness_policy_entries();
        let required_evidence_count = denied_sample.sample_required_evidence_count;
        let policy_entry_count = freshness_entries.len();
        let missing_evidence_count = freshness_entries
            .iter()
            .filter(|entry| entry.required_for_activation && !entry.recorded)
            .count();
        let fresh_evidence_count = freshness_entries
            .iter()
            .filter(|entry| entry.required_for_activation && entry.fresh)
            .count();
        let expired_evidence_count = 0;
        let stale_evidence_count = 0;
        let activation_allowed_by_freshness_policy = false;
        let activation_blocked_by_freshness_policy = true;
        let freshness_denial_reason =
            "all required activation evidence slots are absent from the denied sample".to_string();
        let freshness_policy_ready = denied_sample.status == "ready"
            && denied_sample.sample_validation_status == "blocked"
            && policy_entry_count == required_evidence_count
            && missing_evidence_count == required_evidence_count
            && fresh_evidence_count == 0
            && activation_blocked_by_freshness_policy
            && !activation_allowed_by_freshness_policy;

        Self {
            product: "Hepta".into(),
            status: if freshness_policy_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            policy_id: "upstream-codex-activation-evidence-freshness-policy".into(),
            policy_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_FRESHNESS_POLICY.md"
                    .into(),
            upstream_repository: denied_sample.upstream_repository,
            candidate_diff_range: denied_sample.candidate_diff_range,
            source_denied_sample_gate: denied_sample.denied_sample_gate,
            freshness_policy_gate:
                "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh".into(),
            active_dependency_isolation_gate: denied_sample.active_dependency_isolation_gate,
            denied_sample_ready: denied_sample.status == "ready",
            required_evidence_count,
            policy_entry_count,
            missing_evidence_count,
            fresh_evidence_count,
            expired_evidence_count,
            stale_evidence_count,
            freshness_policy_ready,
            activation_blocked_by_freshness_policy,
            activation_allowed_by_freshness_policy,
            freshness_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            freshness_entries,
            policy_invariants: vec![
                "freshness policy defines evidence requirements but records no evidence".into(),
                "missing evidence is a denial reason even when packet shape is complete".into(),
                "freshness is evaluated per evidence slot before active wiring can be reconsidered"
                    .into(),
                "operator approval, public release claims, and artifact writes remain denied"
                    .into(),
            ],
            required_next_gates: vec![
                "bind each required evidence slot to a concrete evidence id".into(),
                "timestamp and hash every live dependency, watchdog, browser, soak, and rollback evidence record"
                    .into(),
                "rerun the denied-sample gate after replacing absence with concrete evidence".into(),
                "rerun clean preflight and live gates before any active wiring decision".into(),
            ],
        }
    }
}

fn activation_evidence_freshness_policy_entry(
    evidence_id: &str,
    source_gate: &str,
    freshness_anchor: &str,
    max_age_policy: &str,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry {
    HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry {
        evidence_id: evidence_id.into(),
        source_gate: source_gate.into(),
        freshness_anchor: freshness_anchor.into(),
        max_age_policy: max_age_policy.into(),
        required_for_activation: true,
        recorded: false,
        fresh: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_freshness_policy_entries()
-> Vec<HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry> {
    vec![
        activation_evidence_freshness_policy_entry(
            "activation_request_id",
            "scripts/hepta-upstream-codex-activation-request-packet.sh",
            "candidate diff range and requested activation scope",
            "same activation request",
            "activation request id is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "operator_approval_id",
            "scripts/hepta-public-ga-operator-approval-packet.sh",
            "explicit operator approval record",
            "same activation request",
            "operator approval id is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "operator_identity_hash",
            "scripts/hepta-public-ga-operator-approval-packet.sh",
            "redacted operator identity bound to approval id",
            "same activation request",
            "operator identity hash is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "live_dependency_isolation_evidence_id",
            "scripts/hepta-active-service-dependency-isolation.sh",
            "active binary sha and live dependency-closure route",
            "30 minutes",
            "live dependency isolation evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "watchdog_evidence_id",
            "scripts/hepta-watchdog.sh",
            "active binary sha and live watchdog route matrix",
            "30 minutes",
            "watchdog evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "browser_smoke_evidence_id",
            "scripts/hepta-browser-visual-smoke.sh",
            "desktop and mobile screenshot hashes",
            "30 minutes",
            "browser smoke evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "long_soak_evidence_id",
            "scripts/hepta-live-soak.sh",
            "24/24 live soak sample report",
            "120 minutes",
            "long soak evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "rollback_plan_id",
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md",
            "candidate diff range and active binary rollback anchor",
            "same activation request",
            "rollback plan id is absent",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport {
    pub fn native_default() -> Self {
        let freshness = HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport::native_default();
        let binding_schema_fields = default_activation_evidence_binding_record_schema_fields();
        let binding_records =
            default_activation_evidence_binding_record_manifest_entries(&freshness);
        let required_evidence_count = freshness.required_evidence_count;
        let binding_record_count = binding_records.len();
        let missing_binding_record_count = binding_records
            .iter()
            .filter(|record| !record.evidence_recorded)
            .count();
        let recorded_binding_record_count = binding_records
            .iter()
            .filter(|record| record.evidence_recorded)
            .count();
        let required_record_schema_field_count = binding_schema_fields.len();
        let recorded_record_schema_field_count = binding_schema_fields
            .iter()
            .filter(|field| field.required && field.recorded)
            .count();
        let timestamped_record_count = binding_records
            .iter()
            .filter(|record| record.timestamp_recorded)
            .count();
        let binary_sha_bound_record_count = binding_records
            .iter()
            .filter(|record| record.active_binary_sha_bound)
            .count();
        let route_or_status_hash_bound_record_count = binding_records
            .iter()
            .filter(|record| record.route_or_status_hash_bound)
            .count();
        let artifact_hash_or_redacted_path_bound_record_count = binding_records
            .iter()
            .filter(|record| record.artifact_hash_or_redacted_path_bound)
            .count();
        let activation_request_id_bound_record_count = binding_records
            .iter()
            .filter(|record| record.activation_request_id_bound)
            .count();
        let activation_allowed_by_binding_manifest = false;
        let activation_blocked_by_binding_manifest = true;
        let binding_denial_reason =
            "all evidence binding records are schema-only and unrecorded".to_string();
        let binding_manifest_ready = freshness.freshness_policy_ready
            && required_evidence_count == 8
            && binding_record_count == required_evidence_count
            && missing_binding_record_count == required_evidence_count
            && recorded_binding_record_count == 0
            && required_record_schema_field_count == 7
            && recorded_record_schema_field_count == 0
            && timestamped_record_count == 0
            && binary_sha_bound_record_count == 0
            && route_or_status_hash_bound_record_count == 0
            && artifact_hash_or_redacted_path_bound_record_count == 0
            && activation_request_id_bound_record_count == 0
            && activation_blocked_by_binding_manifest
            && !activation_allowed_by_binding_manifest;

        Self {
            product: "Hepta".into(),
            status: if binding_manifest_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            manifest_id: "upstream-codex-activation-evidence-binding-record-manifest"
                .into(),
            manifest_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_BINDING_RECORD.md"
                    .into(),
            upstream_repository: freshness.upstream_repository,
            candidate_diff_range: freshness.candidate_diff_range,
            source_freshness_policy_gate: freshness.freshness_policy_gate,
            binding_manifest_gate:
                "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh".into(),
            active_dependency_isolation_gate: freshness.active_dependency_isolation_gate,
            freshness_policy_ready: freshness.freshness_policy_ready,
            required_evidence_count,
            binding_record_count,
            missing_binding_record_count,
            recorded_binding_record_count,
            required_record_schema_field_count,
            recorded_record_schema_field_count,
            timestamped_record_count,
            binary_sha_bound_record_count,
            route_or_status_hash_bound_record_count,
            artifact_hash_or_redacted_path_bound_record_count,
            activation_request_id_bound_record_count,
            binding_manifest_ready,
            activation_blocked_by_binding_manifest,
            activation_allowed_by_binding_manifest,
            binding_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            binding_schema_fields,
            binding_records,
            binding_invariants: vec![
                "binding manifest defines record shape without recording evidence".into(),
                "every evidence record must bind to an activation request id".into(),
                "live evidence records must bind active binary sha and route or status hash"
                    .into(),
                "artifact-bearing records must use artifact hash or redacted artifact path"
                    .into(),
                "schema-only binding records keep active wiring, public release, and artifact writes denied"
                    .into(),
            ],
            required_next_gates: vec![
                "materialize concrete evidence records only after operator approval".into(),
                "populate timestamp, active binary sha, route/status hash, artifact hash or redacted path, and activation request binding for every evidence slot"
                    .into(),
                "rerun freshness policy against recorded evidence before allowing activation review"
                    .into(),
                "rerun clean preflight, live gates, and long soak before any active wiring decision"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_binding_record_schema_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField {
    HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_binding_record_schema_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField> {
    vec![
        activation_evidence_binding_record_schema_field(
            "evidence_record_id",
            false,
            "stable id for the evidence record",
        ),
        activation_evidence_binding_record_schema_field(
            "source_gate",
            false,
            "gate or document that produced the evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "recorded_at_unix_ms",
            false,
            "timestamp used for freshness evaluation",
        ),
        activation_evidence_binding_record_schema_field(
            "active_binary_sha256",
            false,
            "active Hepta binary sha bound to live evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "route_or_status_hash",
            true,
            "hash of the route response or status payload used as evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "artifact_sha256_or_redacted_path",
            true,
            "artifact hash or redacted local path for browser/soak/rollback evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "activation_request_id_binding",
            false,
            "activation request id that this evidence record authorizes",
        ),
    ]
}

fn activation_evidence_binding_record_manifest_entry(
    evidence_id: &str,
    source_gate: &str,
    required_schema_field_count: usize,
) -> HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry {
    HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry {
        evidence_id: evidence_id.into(),
        source_gate: source_gate.into(),
        required_schema_field_count,
        recorded_schema_field_count: 0,
        evidence_recorded: false,
        timestamp_recorded: false,
        active_binary_sha_bound: false,
        route_or_status_hash_bound: false,
        artifact_hash_or_redacted_path_bound: false,
        activation_request_id_bound: false,
        binding_denial_reason: format!("{evidence_id} binding record is not recorded"),
    }
}

fn default_activation_evidence_binding_record_manifest_entries(
    freshness: &HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport,
) -> Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry> {
    let required_schema_field_count =
        default_activation_evidence_binding_record_schema_fields().len();
    freshness
        .freshness_entries
        .iter()
        .map(|entry| {
            activation_evidence_binding_record_manifest_entry(
                &entry.evidence_id,
                &entry.source_gate,
                required_schema_field_count,
            )
        })
        .collect()
}

impl HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport {
    pub fn native_default() -> Self {
        let binding =
            HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport::native_default();
        let fixture_records = default_activation_evidence_record_denied_fixture_entries(&binding);
        let required_evidence_count = binding.required_evidence_count;
        let fixture_record_count = fixture_records.len();
        let schema_complete_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.schema_complete)
            .count();
        let trusted_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.trusted)
            .count();
        let operator_approved_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.operator_approved)
            .count();
        let request_binding_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.request_binding_verified)
            .count();
        let live_gate_hash_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.live_gate_hash_verified)
            .count();
        let artifact_hash_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.artifact_hash_verified)
            .count();
        let fresh_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.freshness_window_satisfied)
            .count();
        let blocked_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.validation_status == "blocked")
            .count();
        let allowed_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.validation_status == "allowed")
            .count();
        let activation_allowed_by_denied_fixture = false;
        let activation_blocked_by_denied_fixture = true;
        let fixture_denial_reason =
            "fixture evidence records are placeholders without operator approval or verified freshness"
                .to_string();
        let denied_fixture_ready = binding.binding_manifest_ready
            && required_evidence_count == 8
            && fixture_record_count == required_evidence_count
            && schema_complete_fixture_record_count == required_evidence_count
            && trusted_fixture_record_count == 0
            && operator_approved_fixture_record_count == 0
            && request_binding_verified_record_count == 0
            && live_gate_hash_verified_record_count == 0
            && artifact_hash_verified_record_count == 0
            && fresh_fixture_record_count == 0
            && blocked_fixture_record_count == required_evidence_count
            && allowed_fixture_record_count == 0
            && activation_blocked_by_denied_fixture
            && !activation_allowed_by_denied_fixture;

        Self {
            product: "Hepta".into(),
            status: if denied_fixture_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            fixture_id: "upstream-codex-activation-evidence-record-denied-fixture".into(),
            fixture_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_DENIED_FIXTURE.md"
                    .into(),
            upstream_repository: binding.upstream_repository,
            candidate_diff_range: binding.candidate_diff_range,
            source_binding_manifest_gate: binding.binding_manifest_gate,
            denied_fixture_gate:
                "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh".into(),
            active_dependency_isolation_gate: binding.active_dependency_isolation_gate,
            binding_manifest_ready: binding.binding_manifest_ready,
            required_evidence_count,
            fixture_record_count,
            schema_complete_fixture_record_count,
            trusted_fixture_record_count,
            operator_approved_fixture_record_count,
            request_binding_verified_record_count,
            live_gate_hash_verified_record_count,
            artifact_hash_verified_record_count,
            fresh_fixture_record_count,
            blocked_fixture_record_count,
            allowed_fixture_record_count,
            denied_fixture_ready,
            activation_blocked_by_denied_fixture,
            activation_allowed_by_denied_fixture,
            fixture_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            fixture_records,
            fixture_invariants: vec![
                "full-shaped placeholder evidence records are not trusted evidence".into(),
                "operator approval must verify every evidence record before activation review"
                    .into(),
                "activation request binding must be verified rather than merely present".into(),
                "live gate and artifact hashes must be verified before freshness can count".into(),
                "denied fixtures keep active wiring, public release, and artifact writes false"
                    .into(),
            ],
            required_next_gates: vec![
                "replace placeholder records with operator-approved evidence records".into(),
                "verify activation request binding and live gate hashes for every record".into(),
                "verify artifact hashes or redacted paths for browser, soak, and rollback records"
                    .into(),
                "rerun freshness policy with trusted recorded evidence before any activation decision"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_record_denied_fixture_entry(
    evidence_id: &str,
    source_gate: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry {
    HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry {
        evidence_id: evidence_id.into(),
        evidence_record_id: format!("fixture-{evidence_id}"),
        source_gate: source_gate.into(),
        recorded_at_unix_ms: "0".into(),
        active_binary_sha256: "placeholder-active-binary-sha256".into(),
        route_or_status_hash: "placeholder-route-or-status-hash".into(),
        artifact_sha256_or_redacted_path: "placeholder-artifact-hash-or-redacted-path".into(),
        activation_request_id_binding: "placeholder-activation-request-id".into(),
        schema_complete: true,
        operator_approved: false,
        request_binding_verified: false,
        live_gate_hash_verified: false,
        artifact_hash_verified: false,
        freshness_window_satisfied: false,
        trusted: false,
        validation_status: "blocked".into(),
        denial_reason:
            "placeholder evidence lacks operator approval, verified binding, trusted hashes, and freshness"
                .into(),
    }
}

fn default_activation_evidence_record_denied_fixture_entries(
    binding: &HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport,
) -> Vec<HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry> {
    binding
        .binding_records
        .iter()
        .map(|record| {
            activation_evidence_record_denied_fixture_entry(
                &record.evidence_id,
                &record.source_gate,
            )
        })
        .collect()
}

impl HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport {
    pub fn native_default() -> Self {
        let denied_fixture =
            HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport::native_default();
        let verification_entries =
            default_activation_trusted_evidence_acceptance_matrix_entries(&denied_fixture);
        let required_evidence_count = denied_fixture.required_evidence_count;
        let verification_entry_count = verification_entries.len();
        let schema_complete_verification_entry_count = verification_entries
            .iter()
            .filter(|entry| entry.schema_complete)
            .count();
        let required_verification_count_per_record = 7;
        let total_required_verification_count = verification_entries
            .iter()
            .map(|entry| entry.required_verification_count)
            .sum();
        let total_satisfied_verification_count = verification_entries
            .iter()
            .map(|entry| entry.satisfied_verification_count)
            .sum();
        let operator_approval_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.operator_approval_verified)
            .count();
        let request_binding_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.activation_request_binding_verified)
            .count();
        let active_binary_sha_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.active_binary_sha_verified)
            .count();
        let route_or_status_hash_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.route_or_status_hash_verified)
            .count();
        let artifact_hash_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.artifact_hash_or_redacted_path_verified)
            .count();
        let freshness_window_satisfied_record_count = verification_entries
            .iter()
            .filter(|entry| entry.freshness_window_satisfied)
            .count();
        let trusted_source_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.trusted_source_verified)
            .count();
        let accepted_record_count = verification_entries
            .iter()
            .filter(|entry| entry.accepted)
            .count();
        let blocked_record_count = verification_entries
            .iter()
            .filter(|entry| entry.acceptance_status == "blocked")
            .count();
        let activation_allowed_by_trusted_acceptance_matrix = false;
        let activation_blocked_by_trusted_acceptance_matrix = true;
        let acceptance_denial_reason =
            "trusted evidence acceptance requires operator approval, request binding, hashes, freshness, and trusted source verification"
                .to_string();
        let trusted_evidence_acceptance_matrix_ready = denied_fixture.denied_fixture_ready
            && required_evidence_count == 8
            && verification_entry_count == required_evidence_count
            && schema_complete_verification_entry_count == required_evidence_count
            && required_verification_count_per_record == 7
            && total_required_verification_count == required_evidence_count * 7
            && total_satisfied_verification_count == 0
            && operator_approval_verified_record_count == 0
            && request_binding_verified_record_count == 0
            && active_binary_sha_verified_record_count == 0
            && route_or_status_hash_verified_record_count == 0
            && artifact_hash_verified_record_count == 0
            && freshness_window_satisfied_record_count == 0
            && trusted_source_verified_record_count == 0
            && accepted_record_count == 0
            && blocked_record_count == required_evidence_count
            && activation_blocked_by_trusted_acceptance_matrix
            && !activation_allowed_by_trusted_acceptance_matrix;

        Self {
            product: "Hepta".into(),
            status: if trusted_evidence_acceptance_matrix_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            matrix_id: "upstream-codex-activation-trusted-evidence-acceptance-matrix".into(),
            matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_EVIDENCE_ACCEPTANCE_MATRIX.md"
                    .into(),
            upstream_repository: denied_fixture.upstream_repository,
            candidate_diff_range: denied_fixture.candidate_diff_range,
            source_denied_fixture_gate: denied_fixture.denied_fixture_gate,
            trusted_acceptance_matrix_gate:
                "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: denied_fixture.active_dependency_isolation_gate,
            source_denied_fixture_ready: denied_fixture.denied_fixture_ready,
            required_evidence_count,
            verification_entry_count,
            schema_complete_verification_entry_count,
            required_verification_count_per_record,
            total_required_verification_count,
            total_satisfied_verification_count,
            operator_approval_verified_record_count,
            request_binding_verified_record_count,
            active_binary_sha_verified_record_count,
            route_or_status_hash_verified_record_count,
            artifact_hash_verified_record_count,
            freshness_window_satisfied_record_count,
            trusted_source_verified_record_count,
            accepted_record_count,
            blocked_record_count,
            trusted_evidence_acceptance_matrix_ready,
            activation_blocked_by_trusted_acceptance_matrix,
            activation_allowed_by_trusted_acceptance_matrix,
            acceptance_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            verification_entries,
            acceptance_invariants: vec![
                "schema-complete fixture records are not trusted evidence".into(),
                "operator approval must be verified for every evidence record".into(),
                "activation request binding, active binary sha, and route/status hash must all verify"
                    .into(),
                "artifact hash or redacted path and freshness window must verify before acceptance"
                    .into(),
                "trusted source verification is required before active wiring can be reconsidered"
                    .into(),
            ],
            required_next_gates: vec![
                "replace placeholders with operator-approved evidence records".into(),
                "bind every evidence record to the activation request id and active binary sha"
                    .into(),
                "verify route/status and artifact hashes for live dependency, watchdog, browser, soak, and rollback evidence"
                    .into(),
                "rerun freshness policy and clean preflight after trusted evidence is recorded"
                    .into(),
            ],
        }
    }
}

fn activation_trusted_evidence_acceptance_matrix_entry(
    record: &HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry,
) -> HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry {
    HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry {
        evidence_id: record.evidence_id.clone(),
        evidence_record_id: record.evidence_record_id.clone(),
        source_gate: record.source_gate.clone(),
        schema_complete: record.schema_complete,
        required_verification_count: 7,
        satisfied_verification_count: 0,
        operator_approval_required: true,
        operator_approval_verified: false,
        activation_request_binding_required: true,
        activation_request_binding_verified: false,
        active_binary_sha_required: true,
        active_binary_sha_verified: false,
        route_or_status_hash_required: true,
        route_or_status_hash_verified: false,
        artifact_hash_or_redacted_path_required: true,
        artifact_hash_or_redacted_path_verified: false,
        freshness_window_required: true,
        freshness_window_satisfied: false,
        trusted_source_required: true,
        trusted_source_verified: false,
        accepted: false,
        acceptance_status: "blocked".into(),
        denial_reason: "trusted evidence acceptance requires all seven verification checks to pass"
            .into(),
    }
}

fn default_activation_trusted_evidence_acceptance_matrix_entries(
    denied_fixture: &HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport,
) -> Vec<HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry> {
    denied_fixture
        .fixture_records
        .iter()
        .map(activation_trusted_evidence_acceptance_matrix_entry)
        .collect()
}

impl HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport {
    pub fn native_default() -> Self {
        let matrix =
            HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport::native_default();
        let fixtures = default_activation_trusted_record_shape_validator_fixtures(&matrix);
        let required_evidence_count = matrix.required_evidence_count;
        let fixture_count = fixtures.len();
        let partial_trusted_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.fixture_kind == "partial_trusted_records")
            .count();
        let public_claim_attempt_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.fixture_kind == "public_claim_attempt")
            .count();
        let blocked_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "blocked")
            .count();
        let allowed_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "allowed")
            .count();
        let required_verification_count_per_record = matrix.required_verification_count_per_record;
        let total_required_verification_count_per_fixture =
            required_evidence_count * required_verification_count_per_record;
        let max_satisfied_verification_count = fixtures
            .iter()
            .map(|fixture| fixture.total_satisfied_verification_count)
            .max()
            .unwrap_or_default();
        let activation_allowed_by_shape_validator = false;
        let activation_blocked_by_shape_validator = true;
        let shape_denial_reason =
            "partial or public-claim trusted-record shapes stay blocked until every record is fresh, bound, trusted, and operator-approved"
                .to_string();
        let trusted_record_shape_validator_ready = matrix.trusted_evidence_acceptance_matrix_ready
            && required_evidence_count == 8
            && fixture_count == 2
            && partial_trusted_fixture_count == 1
            && public_claim_attempt_fixture_count == 1
            && blocked_fixture_count == fixture_count
            && allowed_fixture_count == 0
            && required_verification_count_per_record == 7
            && total_required_verification_count_per_fixture == 56
            && max_satisfied_verification_count < total_required_verification_count_per_fixture
            && fixtures.iter().all(|fixture| {
                !fixture.active_wiring_allowed
                    && !fixture.public_release_claim_allowed
                    && !fixture.release_artifact_write_allowed
            })
            && activation_blocked_by_shape_validator
            && !activation_allowed_by_shape_validator;

        Self {
            product: "Hepta".into(),
            status: if trusted_record_shape_validator_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            validator_id: "upstream-codex-activation-trusted-record-shape-validator".into(),
            validator_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_RECORD_SHAPE_VALIDATOR.md"
                    .into(),
            upstream_repository: matrix.upstream_repository,
            candidate_diff_range: matrix.candidate_diff_range,
            source_trusted_acceptance_matrix_gate: matrix.trusted_acceptance_matrix_gate,
            trusted_record_shape_validator_gate:
                "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
                    .into(),
            active_dependency_isolation_gate: matrix.active_dependency_isolation_gate,
            source_trusted_acceptance_matrix_ready: matrix
                .trusted_evidence_acceptance_matrix_ready,
            required_evidence_count,
            fixture_count,
            partial_trusted_fixture_count,
            public_claim_attempt_fixture_count,
            blocked_fixture_count,
            allowed_fixture_count,
            required_verification_count_per_record,
            total_required_verification_count_per_fixture,
            max_satisfied_verification_count,
            trusted_record_shape_validator_ready,
            activation_blocked_by_shape_validator,
            activation_allowed_by_shape_validator,
            shape_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            fixtures,
            shape_invariants: vec![
                "partially verified evidence records are not trusted evidence".into(),
                "public release claims stay blocked when any evidence record is incomplete"
                    .into(),
                "release artifact writes stay blocked when freshness is missing".into(),
                "active wiring stays false for every trusted-record shape fixture".into(),
                "shape validation is report-only and performs no upstream or runtime mutation"
                    .into(),
            ],
            required_next_gates: vec![
                "record a real operator-approved activation request before replacing fixtures"
                    .into(),
                "verify all seven checks for every required evidence record".into(),
                "rerun clean preflight, live gates, browser smoke, and long soak after evidence recording"
                    .into(),
                "require a separate explicit operator decision before any public claim or artifact write"
                    .into(),
            ],
        }
    }
}

fn activation_trusted_record_shape_validator_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    matrix: &HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport,
    verified_counts: (usize, usize, usize, usize, usize, usize, usize),
    public_release_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture {
    let (
        operator_approval_verified_record_count,
        request_binding_verified_record_count,
        active_binary_sha_verified_record_count,
        route_or_status_hash_verified_record_count,
        artifact_hash_verified_record_count,
        freshness_window_satisfied_record_count,
        trusted_source_verified_record_count,
    ) = verified_counts;
    let evidence_record_count = matrix.required_evidence_count;
    let required_verification_count_per_record = matrix.required_verification_count_per_record;
    let total_required_verification_count =
        evidence_record_count * required_verification_count_per_record;
    let total_satisfied_verification_count = operator_approval_verified_record_count
        + request_binding_verified_record_count
        + active_binary_sha_verified_record_count
        + route_or_status_hash_verified_record_count
        + artifact_hash_verified_record_count
        + freshness_window_satisfied_record_count
        + trusted_source_verified_record_count;

    HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        evidence_record_count,
        schema_complete_record_count: matrix.schema_complete_verification_entry_count,
        required_verification_count_per_record,
        total_required_verification_count,
        total_satisfied_verification_count,
        operator_approval_verified_record_count,
        request_binding_verified_record_count,
        active_binary_sha_verified_record_count,
        route_or_status_hash_verified_record_count,
        artifact_hash_verified_record_count,
        freshness_window_satisfied_record_count,
        trusted_source_verified_record_count,
        accepted_record_count: 0,
        blocked_record_count: evidence_record_count,
        public_release_claim_requested,
        release_artifact_write_requested,
        validation_status: "blocked".into(),
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_trusted_record_shape_validator_fixtures(
    matrix: &HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport,
) -> Vec<HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture> {
    let required = matrix.required_evidence_count;
    vec![
        activation_trusted_record_shape_validator_fixture(
            "partial-trusted-records",
            "partial_trusted_records",
            matrix,
            (required, required, required, required, 0, 0, 0),
            false,
            false,
            "partial trusted-record shape is missing artifact hashes, freshness, and trusted source verification",
        ),
        activation_trusted_record_shape_validator_fixture(
            "public-claim-attempt-with-trusted-shape",
            "public_claim_attempt",
            matrix,
            (
                required, required, required, required, required, 0, required,
            ),
            true,
            true,
            "public release and artifact write attempts remain blocked while freshness is incomplete",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport {
    pub fn native_default() -> Self {
        let shape_validator =
            HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport::native_default();
        let gate_families = default_activation_evidence_completeness_gate_families();
        let required_gate_family_count = gate_families.len();
        let ready_gate_family_count = gate_families
            .iter()
            .filter(|family| family.gate_ready)
            .count();
        let activation_blocking_gate_family_count = gate_families
            .iter()
            .filter(|family| family.blocks_activation_without_trusted_evidence)
            .count();
        let required_evidence_count = shape_validator.required_evidence_count;
        let required_trusted_record_count = required_evidence_count;
        let accepted_trusted_record_count = 0;
        let fresh_trusted_record_count = 0;
        let operator_approval_recorded = false;
        let activation_request_recorded = false;
        let public_claim_attempt_blocked = shape_validator.fixtures.iter().any(|fixture| {
            fixture.fixture_id == "public-claim-attempt-with-trusted-shape"
                && fixture.public_release_claim_requested
                && fixture.release_artifact_write_requested
                && !fixture.public_release_claim_allowed
        });
        let release_artifact_write_attempt_blocked =
            shape_validator.fixtures.iter().any(|fixture| {
                fixture.fixture_id == "public-claim-attempt-with-trusted-shape"
                    && fixture.release_artifact_write_requested
                    && !fixture.release_artifact_write_allowed
            });
        let operator_approved_activation_ready = false;
        let activation_allowed_by_scoreboard = false;
        let activation_blocked_by_scoreboard = true;
        let scoreboard_denial_reason =
            "activation evidence gate families are ready, but no real activation request or fresh trusted evidence records exist"
                .to_string();
        let evidence_completeness_scoreboard_ready = shape_validator
            .trusted_record_shape_validator_ready
            && required_gate_family_count == 10
            && ready_gate_family_count == required_gate_family_count
            && activation_blocking_gate_family_count == required_gate_family_count
            && required_evidence_count == 8
            && required_trusted_record_count == 8
            && accepted_trusted_record_count == 0
            && fresh_trusted_record_count == 0
            && !operator_approval_recorded
            && !activation_request_recorded
            && public_claim_attempt_blocked
            && release_artifact_write_attempt_blocked
            && !operator_approved_activation_ready
            && activation_blocked_by_scoreboard
            && !activation_allowed_by_scoreboard;

        Self {
            product: "Hepta".into(),
            status: if evidence_completeness_scoreboard_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            scoreboard_id: "upstream-codex-activation-evidence-completeness-scoreboard".into(),
            scoreboard_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_COMPLETENESS_SCOREBOARD.md"
                    .into(),
            upstream_repository: shape_validator.upstream_repository,
            candidate_diff_range: shape_validator.candidate_diff_range,
            source_trusted_record_shape_validator_gate: shape_validator
                .trusted_record_shape_validator_gate,
            evidence_completeness_scoreboard_gate:
                "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
                    .into(),
            active_dependency_isolation_gate: shape_validator.active_dependency_isolation_gate,
            source_trusted_record_shape_validator_ready: shape_validator
                .trusted_record_shape_validator_ready,
            required_gate_family_count,
            ready_gate_family_count,
            activation_blocking_gate_family_count,
            required_evidence_count,
            required_trusted_record_count,
            accepted_trusted_record_count,
            fresh_trusted_record_count,
            operator_approval_recorded,
            activation_request_recorded,
            public_claim_attempt_blocked,
            release_artifact_write_attempt_blocked,
            operator_approved_activation_ready,
            evidence_completeness_scoreboard_ready,
            activation_blocked_by_scoreboard,
            activation_allowed_by_scoreboard,
            scoreboard_denial_reason,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            gate_families,
            scoreboard_invariants: vec![
                "all activation evidence gate families can be ready while activation remains denied"
                    .into(),
                "zero accepted trusted records means operator-approved activation is not ready"
                    .into(),
                "public claim and release artifact attempts remain blocked by the scoreboard"
                    .into(),
                "scoreboard readiness does not record evidence or mutate active runtime state".into(),
            ],
            required_next_gates: vec![
                "record a real activation request id and operator approval id".into(),
                "replace fixture evidence with fresh trusted records for all eight required evidence ids"
                    .into(),
                "rerun evidence completeness scoreboard after live gates and long soak".into(),
                "require explicit public-claim and artifact-write approval before external release actions"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_completeness_gate_family(
    gate_id: &str,
    gate_script: &str,
) -> HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily {
    HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily {
        gate_id: gate_id.into(),
        gate_script: gate_script.into(),
        gate_ready: true,
        blocks_activation_without_trusted_evidence: true,
    }
}

fn default_activation_evidence_completeness_gate_families()
-> Vec<HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily> {
    vec![
        activation_evidence_completeness_gate_family(
            "activation-request-packet",
            "scripts/hepta-upstream-codex-activation-request-packet.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-packet-dry-run",
            "scripts/hepta-upstream-codex-activation-packet-dry-run.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-ledger",
            "scripts/hepta-upstream-codex-activation-evidence-ledger.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-readiness-closure",
            "scripts/hepta-upstream-codex-activation-readiness-closure.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-denied-sample",
            "scripts/hepta-upstream-codex-activation-denied-sample.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-freshness-policy",
            "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-binding-record",
            "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-denied-fixture",
            "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-trusted-evidence-acceptance-matrix",
            "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-trusted-record-shape-validator",
            "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport {
    pub fn native_default() -> Self {
        let scoreboard =
            HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport::native_default();
        let receipt_fields = default_activation_evidence_recording_receipt_fields();
        let required_receipt_field_count = receipt_fields.len();
        let recorded_receipt_field_count =
            receipt_fields.iter().filter(|field| field.recorded).count();
        let redacted_or_hashed_field_count = receipt_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let receipt_schema_ready = scoreboard.evidence_completeness_scoreboard_ready
            && required_receipt_field_count == 12
            && recorded_receipt_field_count == 0
            && redacted_or_hashed_field_count >= 8
            && receipt_fields
                .iter()
                .all(|field| field.required && !field.recorded);
        let receipt_recorded = false;
        let real_evidence_recorded = false;
        let trusted_record_materialized = false;
        let evidence_recording_dry_run_ready = receipt_schema_ready
            && !receipt_recorded
            && !real_evidence_recorded
            && !trusted_record_materialized
            && !scoreboard.operator_approval_recorded
            && !scoreboard.activation_request_recorded
            && scoreboard.accepted_trusted_record_count == 0
            && scoreboard.fresh_trusted_record_count == 0
            && scoreboard.public_claim_attempt_blocked
            && scoreboard.release_artifact_write_attempt_blocked;
        let activation_blocked_by_receipt = true;
        let activation_allowed_by_receipt = false;

        Self {
            product: "Hepta".into(),
            status: if evidence_recording_dry_run_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            receipt_id: "upstream-codex-activation-evidence-recording-dry-run-receipt".into(),
            receipt_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DRY_RUN_RECEIPT.md"
                    .into(),
            upstream_repository: scoreboard.upstream_repository,
            candidate_diff_range: scoreboard.candidate_diff_range,
            source_scoreboard_gate: scoreboard.evidence_completeness_scoreboard_gate,
            evidence_recording_dry_run_receipt_gate:
                "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
                    .into(),
            active_dependency_isolation_gate: scoreboard.active_dependency_isolation_gate,
            source_scoreboard_ready: scoreboard.evidence_completeness_scoreboard_ready,
            required_receipt_field_count,
            recorded_receipt_field_count,
            redacted_or_hashed_field_count,
            required_evidence_count: scoreboard.required_evidence_count,
            required_trusted_record_count: scoreboard.required_trusted_record_count,
            accepted_trusted_record_count: scoreboard.accepted_trusted_record_count,
            fresh_trusted_record_count: scoreboard.fresh_trusted_record_count,
            operator_approval_recorded: scoreboard.operator_approval_recorded,
            activation_request_recorded: scoreboard.activation_request_recorded,
            receipt_schema_ready,
            receipt_recorded,
            real_evidence_recorded,
            trusted_record_materialized,
            public_claim_attempt_blocked: scoreboard.public_claim_attempt_blocked,
            release_artifact_write_attempt_blocked: scoreboard
                .release_artifact_write_attempt_blocked,
            evidence_recording_dry_run_ready,
            activation_blocked_by_receipt,
            activation_allowed_by_receipt,
            receipt_denial_reason:
                "recording receipt is schema-only; no real activation request, operator approval, fresh trusted records, or workspace write is present"
                    .into(),
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            receipt_fields,
            receipt_invariants: vec![
                "receipt schema can be ready while no evidence is recorded".into(),
                "all receipt fields remain absent until a real activation request exists".into(),
                "redacted or hashed fields prevent raw operator identity and artifact leakage".into(),
                "dry-run receipt readiness does not permit active wiring or public claims".into(),
            ],
            required_next_gates: vec![
                "bind receipt fields to a real activation request id".into(),
                "record fresh trusted evidence ids only after live gate evidence is captured".into(),
                "write evidence receipts through an explicit operator-approved recording path".into(),
                "rerun scoreboard and receipt gates before any active runtime wiring".into(),
            ],
        }
    }
}

fn activation_evidence_recording_receipt_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordingReceiptField {
    HeptaUpstreamCodexActivationEvidenceRecordingReceiptField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_recording_receipt_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceRecordingReceiptField> {
    vec![
        activation_evidence_recording_receipt_field(
            "evidence_recording_receipt_id",
            true,
            "unique receipt identifier for the dry-run evidence recording packet",
        ),
        activation_evidence_recording_receipt_field(
            "activation_request_id",
            true,
            "binds the receipt to a single operator activation request",
        ),
        activation_evidence_recording_receipt_field(
            "operator_approval_id",
            true,
            "binds the receipt to explicit operator approval",
        ),
        activation_evidence_recording_receipt_field(
            "operator_identity_hash",
            true,
            "records operator identity only as a hash",
        ),
        activation_evidence_recording_receipt_field(
            "accepted_trusted_record_ids",
            true,
            "lists accepted trusted evidence record identifiers",
        ),
        activation_evidence_recording_receipt_field(
            "fresh_trusted_record_ids",
            true,
            "lists trusted evidence records still inside freshness windows",
        ),
        activation_evidence_recording_receipt_field(
            "active_binary_sha256",
            true,
            "binds evidence to the active installed Hepta binary hash",
        ),
        activation_evidence_recording_receipt_field(
            "route_or_status_hash_bundle",
            true,
            "binds evidence to live route and status response hashes",
        ),
        activation_evidence_recording_receipt_field(
            "artifact_sha256_or_redacted_path_bundle",
            true,
            "binds evidence to artifact hashes or redacted local artifact paths",
        ),
        activation_evidence_recording_receipt_field(
            "freshness_window_summary",
            false,
            "summarizes freshness window policy without raw evidence payloads",
        ),
        activation_evidence_recording_receipt_field(
            "rollback_plan_id",
            true,
            "binds activation to an operator-reviewed rollback plan",
        ),
        activation_evidence_recording_receipt_field(
            "public_claim_and_artifact_decision",
            false,
            "records explicit public-claim and release-artifact decisions",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport {
    pub fn native_default() -> Self {
        let receipt =
            HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport::native_default();
        let denied_receipt_attempts =
            default_activation_evidence_recording_denied_receipt_attempts();
        let required_denied_attempt_count = denied_receipt_attempts.len();
        let denied_receipt_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.denial_status == "blocked")
            .count();
        let allowed_receipt_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| {
                attempt.receipt_materialized
                    || attempt.workspace_write_allowed
                    || attempt.active_wiring_allowed
                    || attempt.public_release_claim_allowed
                    || attempt.release_artifact_write_allowed
            })
            .count();
        let max_recorded_receipt_field_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.recorded_receipt_field_count)
            .max()
            .unwrap_or(0);
        let max_accepted_trusted_record_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.accepted_trusted_record_count)
            .max()
            .unwrap_or(0);
        let max_fresh_trusted_record_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.fresh_trusted_record_count)
            .max()
            .unwrap_or(0);
        let public_claim_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.public_claim_requested)
            .count();
        let release_artifact_write_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.release_artifact_write_requested)
            .count();
        let receipt_sink_write_performed = false;
        let evidence_receipt_persisted = false;
        let trusted_record_materialized = false;
        let no_write_sink_ready = receipt.evidence_recording_dry_run_ready
            && required_denied_attempt_count == 3
            && denied_receipt_attempt_count == required_denied_attempt_count
            && allowed_receipt_attempt_count == 0
            && max_recorded_receipt_field_count == receipt.required_receipt_field_count
            && max_accepted_trusted_record_count == receipt.required_trusted_record_count
            && max_fresh_trusted_record_count == receipt.required_trusted_record_count
            && public_claim_attempt_count == 1
            && release_artifact_write_attempt_count == 1
            && !receipt_sink_write_performed
            && !evidence_receipt_persisted
            && !trusted_record_materialized;
        let activation_blocked_by_no_write_sink = true;
        let activation_allowed_by_no_write_sink = false;

        Self {
            product: "Hepta".into(),
            status: if no_write_sink_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            matrix_id: "upstream-codex-activation-evidence-recording-denial-matrix".into(),
            matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DENIAL_MATRIX.md"
                    .into(),
            upstream_repository: receipt.upstream_repository,
            candidate_diff_range: receipt.candidate_diff_range,
            source_receipt_gate: receipt.evidence_recording_dry_run_receipt_gate,
            evidence_recording_denial_matrix_gate:
                "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: receipt.active_dependency_isolation_gate,
            source_receipt_gate_ready: receipt.evidence_recording_dry_run_ready,
            required_denied_attempt_count,
            denied_receipt_attempt_count,
            allowed_receipt_attempt_count,
            max_recorded_receipt_field_count,
            max_accepted_trusted_record_count,
            max_fresh_trusted_record_count,
            public_claim_attempt_count,
            release_artifact_write_attempt_count,
            receipt_sink_write_performed,
            evidence_receipt_persisted,
            trusted_record_materialized,
            no_write_sink_ready,
            activation_blocked_by_no_write_sink,
            activation_allowed_by_no_write_sink,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            denied_receipt_attempts,
            no_write_sink_invariants: vec![
                "denied receipt attempts can be fully shaped without being persisted".into(),
                "receipt sink writes remain false until an explicit operator-approved recording path is opened".into(),
                "public-claim-shaped receipt attempts stay blocked by default".into(),
                "no denied fixture can enable active runtime wiring or release artifact writes".into(),
            ],
            required_next_gates: vec![
                "define an operator-approved receipt persistence command before any workspace write"
                    .into(),
                "bind persisted receipts to fresh trusted record ids and live SHA evidence".into(),
                "rerun denial matrix before accepting any public-claim-shaped receipt".into(),
            ],
        }
    }
}

fn activation_evidence_recording_denied_receipt_attempt(
    attempt_id: &str,
    attempt_kind: &str,
    recorded_receipt_field_count: usize,
    accepted_trusted_record_count: usize,
    fresh_trusted_record_count: usize,
    operator_approval_recorded: bool,
    activation_request_recorded: bool,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt {
    HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt {
        attempt_id: attempt_id.into(),
        attempt_kind: attempt_kind.into(),
        receipt_field_count: 12,
        recorded_receipt_field_count,
        accepted_trusted_record_count,
        fresh_trusted_record_count,
        operator_approval_recorded,
        activation_request_recorded,
        public_claim_requested,
        release_artifact_write_requested,
        receipt_materialized: false,
        workspace_write_allowed: false,
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        denial_status: "blocked".into(),
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_recording_denied_receipt_attempts()
-> Vec<HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt> {
    vec![
        activation_evidence_recording_denied_receipt_attempt(
            "partial-receipt-fields",
            "partial_receipt_fields",
            5,
            3,
            0,
            false,
            true,
            false,
            false,
            "partial receipt fields and stale trusted records cannot be persisted",
        ),
        activation_evidence_recording_denied_receipt_attempt(
            "operator-approved-but-stale-evidence",
            "operator_approved_stale_evidence",
            12,
            8,
            0,
            true,
            true,
            false,
            false,
            "operator approval alone cannot bypass stale trusted evidence",
        ),
        activation_evidence_recording_denied_receipt_attempt(
            "public-claim-release-artifact-attempt",
            "public_claim_release_artifact_attempt",
            12,
            8,
            8,
            true,
            true,
            true,
            true,
            "public release claim and artifact writes require a separate explicit release path",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport {
    pub fn native_default() -> Self {
        let denial_matrix =
            HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport::native_default();
        let command_fields = default_activation_evidence_receipt_persistence_command_fields();
        let required_command_field_count = command_fields.len();
        let recorded_command_field_count =
            command_fields.iter().filter(|field| field.recorded).count();
        let redacted_or_hashed_field_count = command_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let operator_approval_required = true;
        let operator_approval_recorded = false;
        let activation_request_required = true;
        let activation_request_recorded = false;
        let trusted_record_materialized = false;
        let receipt_persistence_command_enabled_by_default = false;
        let receipt_persistence_command_invoked = false;
        let receipt_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let receipt_persistence_noop_ready = denial_matrix.no_write_sink_ready
            && required_command_field_count == 10
            && recorded_command_field_count == 0
            && redacted_or_hashed_field_count >= 8
            && operator_approval_required
            && activation_request_required
            && !operator_approval_recorded
            && !activation_request_recorded
            && !trusted_record_materialized
            && !receipt_persistence_command_enabled_by_default
            && !receipt_persistence_command_invoked
            && !receipt_persistence_execution_performed
            && !workspace_write_performed
            && !evidence_receipt_persisted
            && command_fields
                .iter()
                .all(|field| field.required && !field.recorded);
        let activation_blocked_by_persistence_contract = true;
        let activation_allowed_by_persistence_contract = false;

        Self {
            product: "Hepta".into(),
            status: if receipt_persistence_noop_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            command_contract_id:
                "upstream-codex-activation-evidence-receipt-persistence-command-contract".into(),
            command_contract_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_COMMAND_CONTRACT.md"
                    .into(),
            upstream_repository: denial_matrix.upstream_repository,
            candidate_diff_range: denial_matrix.candidate_diff_range,
            source_denial_matrix_gate: denial_matrix.evidence_recording_denial_matrix_gate,
            receipt_persistence_command_contract_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
                    .into(),
            active_dependency_isolation_gate: denial_matrix.active_dependency_isolation_gate,
            source_denial_matrix_ready: denial_matrix.no_write_sink_ready,
            required_command_field_count,
            recorded_command_field_count,
            redacted_or_hashed_field_count,
            operator_approval_required,
            operator_approval_recorded,
            activation_request_required,
            activation_request_recorded,
            trusted_record_materialized,
            receipt_persistence_command_enabled_by_default,
            receipt_persistence_command_invoked,
            receipt_persistence_execution_performed,
            receipt_persistence_noop_ready,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_persistence_contract,
            activation_allowed_by_persistence_contract,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            command_fields,
            command_contract_invariants: vec![
                "receipt persistence command contract is present but disabled by default".into(),
                "no command invocation can write the workspace without operator approval".into(),
                "activation request and trusted evidence ids are required before persistence".into(),
                "persistence command readiness does not permit active wiring or release claims".into(),
            ],
            required_next_gates: vec![
                "run the redacted receipt persistence invocation dry-run before any real write path"
                    .into(),
                "bind a no-write receipt sink adapter before any persisted receipt path".into(),
                "require live SHA, watchdog, browser smoke, and soak evidence before enabling persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_persistence_command_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_persistence_command_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField> {
    vec![
        activation_evidence_receipt_persistence_command_field(
            "receipt_persistence_command_id",
            true,
            "unique id for an operator-approved persistence command",
        ),
        activation_evidence_receipt_persistence_command_field(
            "activation_request_id",
            true,
            "binds the command to one activation request",
        ),
        activation_evidence_receipt_persistence_command_field(
            "operator_approval_id",
            true,
            "binds the command to explicit operator approval",
        ),
        activation_evidence_receipt_persistence_command_field(
            "operator_identity_hash",
            true,
            "records operator identity only as a hash",
        ),
        activation_evidence_receipt_persistence_command_field(
            "accepted_trusted_record_ids",
            true,
            "lists accepted trusted evidence records to persist",
        ),
        activation_evidence_receipt_persistence_command_field(
            "fresh_trusted_record_ids",
            true,
            "lists trusted evidence records still inside freshness windows",
        ),
        activation_evidence_receipt_persistence_command_field(
            "receipt_payload_hash",
            true,
            "binds the persisted receipt to a redacted payload hash",
        ),
        activation_evidence_receipt_persistence_command_field(
            "receipt_output_path_redacted",
            true,
            "records the intended output path only as a redacted path",
        ),
        activation_evidence_receipt_persistence_command_field(
            "rollback_plan_id",
            true,
            "binds persistence to an operator-reviewed rollback plan",
        ),
        activation_evidence_receipt_persistence_command_field(
            "public_claim_and_artifact_decision",
            false,
            "records explicit public-claim and artifact-write decisions",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport {
    pub fn native_default() -> Self {
        let command_contract =
            HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport::native_default(
            );
        let invocation_fixtures =
            default_activation_evidence_receipt_persistence_invocation_dry_run_fixtures();
        let required_invocation_fixture_count = invocation_fixtures.len();
        let command_invocation_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.command_invocation_requested)
            .count();
        let command_invocation_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.command_invocation_performed)
            .count();
        let receipt_persistence_execution_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_persistence_execution_performed)
            .count();
        let workspace_write_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let redacted_output_path_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_output_path_redacted_recorded)
            .count();
        let payload_hash_bound_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_payload_hash_recorded)
            .count();
        let operator_approved_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_recorded)
            .count();
        let max_recorded_command_field_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.recorded_command_field_count)
            .max()
            .unwrap_or(0);
        let max_accepted_trusted_record_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.accepted_trusted_record_count)
            .max()
            .unwrap_or(0);
        let max_fresh_trusted_record_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.fresh_trusted_record_count)
            .max()
            .unwrap_or(0);
        let public_claim_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let receipt_persistence_command_enabled_by_default = false;
        let invocation_dry_run_noop_ready = command_contract.receipt_persistence_noop_ready
            && required_invocation_fixture_count == 3
            && command_invocation_attempt_count == required_invocation_fixture_count
            && command_invocation_performed_count == 0
            && receipt_persistence_execution_performed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0
            && redacted_output_path_fixture_count == required_invocation_fixture_count
            && payload_hash_bound_fixture_count == required_invocation_fixture_count
            && operator_approved_fixture_count == required_invocation_fixture_count
            && activation_request_bound_fixture_count == required_invocation_fixture_count
            && max_recorded_command_field_count == command_contract.required_command_field_count
            && max_accepted_trusted_record_count == 8
            && max_fresh_trusted_record_count == 8
            && public_claim_attempt_count == 1
            && release_artifact_write_attempt_count == 1
            && !receipt_persistence_command_enabled_by_default
            && invocation_fixtures.iter().all(|fixture| {
                fixture.dry_run_status == "blocked_noop"
                    && !fixture.command_invocation_performed
                    && !fixture.receipt_persistence_execution_performed
                    && !fixture.workspace_write_performed
                    && !fixture.evidence_receipt_persisted
                    && !fixture.active_wiring_allowed
                    && !fixture.public_release_claim_allowed
                    && !fixture.release_artifact_write_allowed
            });
        let activation_blocked_by_invocation_dry_run = true;
        let activation_allowed_by_invocation_dry_run = false;

        Self {
            product: "Hepta".into(),
            status: if invocation_dry_run_noop_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            invocation_dry_run_id:
                "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
                    .into(),
            invocation_dry_run_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_INVOCATION_DRY_RUN.md"
                    .into(),
            upstream_repository: command_contract.upstream_repository,
            candidate_diff_range: command_contract.candidate_diff_range,
            source_command_contract_gate: command_contract
                .receipt_persistence_command_contract_gate,
            receipt_persistence_invocation_dry_run_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
                    .into(),
            active_dependency_isolation_gate: command_contract.active_dependency_isolation_gate,
            source_command_contract_ready: command_contract.receipt_persistence_noop_ready,
            required_invocation_fixture_count,
            command_invocation_attempt_count,
            command_invocation_performed_count,
            receipt_persistence_execution_performed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            redacted_output_path_fixture_count,
            payload_hash_bound_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            max_recorded_command_field_count,
            max_accepted_trusted_record_count,
            max_fresh_trusted_record_count,
            public_claim_attempt_count,
            release_artifact_write_attempt_count,
            receipt_persistence_command_enabled_by_default,
            invocation_dry_run_noop_ready,
            activation_blocked_by_invocation_dry_run,
            activation_allowed_by_invocation_dry_run,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            invocation_fixtures,
            invocation_dry_run_invariants: vec![
                "redacted invocation fixtures can request persistence without executing it".into(),
                "command invocation remains unperformed while the command is disabled by default"
                    .into(),
                "receipt persistence execution and workspace writes stay false for every fixture"
                    .into(),
                "public-claim-shaped invocation fixtures stay blocked by default".into(),
            ],
            required_next_gates: vec![
                "run the no-write receipt sink adapter contract before any persisted receipt path"
                    .into(),
                "require fresh live gate evidence for every invocation fixture".into(),
                "require operator approval before enabling any receipt persistence command".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_persistence_invocation_dry_run_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        recorded_command_field_count: 10,
        redacted_or_hashed_field_count: 9,
        operator_approval_recorded: true,
        activation_request_recorded: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        receipt_payload_hash_recorded: true,
        receipt_output_path_redacted_recorded: true,
        public_claim_requested,
        release_artifact_write_requested,
        command_invocation_requested: true,
        command_invocation_performed: false,
        receipt_persistence_execution_performed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        dry_run_status: "blocked_noop".into(),
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_persistence_invocation_dry_run_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture> {
    vec![
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "redacted-command-shape",
            "redacted_command_shape",
            8,
            false,
            false,
            "fully shaped redacted command remains a no-op while persistence is disabled by default",
        ),
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "stale-evidence-invocation-attempt",
            "stale_evidence_invocation_attempt",
            0,
            false,
            false,
            "stale trusted evidence cannot execute receipt persistence",
        ),
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "public-claim-artifact-invocation-attempt",
            "public_claim_artifact_invocation_attempt",
            8,
            true,
            true,
            "public claim and artifact write requests remain blocked by the no-op dry run",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport {
    pub fn native_default() -> Self {
        let invocation_dry_run =
            HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport::native_default(
            );
        let sink_surfaces = default_activation_evidence_receipt_no_write_sink_adapter_surfaces();
        let required_sink_surface_count = sink_surfaces.len();
        let ready_sink_surface_count = sink_surfaces.iter().filter(|surface| surface.ready).count();
        let side_effect_free_surface_count = sink_surfaces
            .iter()
            .filter(|surface| surface.side_effect_free)
            .count();
        let accepted_invocation_fixture_count = invocation_dry_run.command_invocation_attempt_count;
        let rejected_write_fixture_count = invocation_dry_run.required_invocation_fixture_count;
        let rejected_public_claim_fixture_count = invocation_dry_run.public_claim_attempt_count;
        let persisted_receipt_count = invocation_dry_run.evidence_receipt_persisted_count;
        let workspace_write_performed_count = invocation_dry_run.workspace_write_performed_count;
        let sink_write_path_enabled_by_default = false;
        let sink_accepts_redacted_payload_hash = true;
        let sink_accepts_redacted_output_path = true;
        let sink_requires_operator_approval = true;
        let sink_requires_fresh_trusted_records = true;
        let sink_rejects_public_claim_artifact_write = true;
        let no_write_sink_adapter_ready = invocation_dry_run.invocation_dry_run_noop_ready
            && required_sink_surface_count == 6
            && ready_sink_surface_count == required_sink_surface_count
            && side_effect_free_surface_count == required_sink_surface_count
            && accepted_invocation_fixture_count == 3
            && rejected_write_fixture_count == 3
            && rejected_public_claim_fixture_count == 1
            && persisted_receipt_count == 0
            && workspace_write_performed_count == 0
            && !sink_write_path_enabled_by_default
            && sink_accepts_redacted_payload_hash
            && sink_accepts_redacted_output_path
            && sink_requires_operator_approval
            && sink_requires_fresh_trusted_records
            && sink_rejects_public_claim_artifact_write;
        let activation_blocked_by_no_write_sink_adapter = true;
        let activation_allowed_by_no_write_sink_adapter = false;

        Self {
            product: "Hepta".into(),
            status: if no_write_sink_adapter_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            no_write_sink_adapter_id:
                "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
                    .into(),
            no_write_sink_adapter_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_NO_WRITE_SINK_ADAPTER_CONTRACT.md"
                    .into(),
            upstream_repository: invocation_dry_run.upstream_repository,
            candidate_diff_range: invocation_dry_run.candidate_diff_range,
            source_invocation_dry_run_gate: invocation_dry_run
                .receipt_persistence_invocation_dry_run_gate,
            no_write_sink_adapter_contract_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
                    .into(),
            active_dependency_isolation_gate: invocation_dry_run.active_dependency_isolation_gate,
            source_invocation_dry_run_ready: invocation_dry_run.invocation_dry_run_noop_ready,
            required_sink_surface_count,
            ready_sink_surface_count,
            side_effect_free_surface_count,
            accepted_invocation_fixture_count,
            rejected_write_fixture_count,
            rejected_public_claim_fixture_count,
            persisted_receipt_count,
            workspace_write_performed_count,
            sink_write_path_enabled_by_default,
            sink_accepts_redacted_payload_hash,
            sink_accepts_redacted_output_path,
            sink_requires_operator_approval,
            sink_requires_fresh_trusted_records,
            sink_rejects_public_claim_artifact_write,
            no_write_sink_adapter_ready,
            activation_blocked_by_no_write_sink_adapter,
            activation_allowed_by_no_write_sink_adapter,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            sink_surfaces,
            no_write_sink_adapter_invariants: vec![
                "no-write sink adapter accepts redacted invocation shapes without persisting them"
                    .into(),
                "filesystem persistence remains disabled by default".into(),
                "public-claim and release-artifact requests are rejected by the no-write sink".into(),
                "sink readiness does not permit active runtime wiring or public claims".into(),
            ],
            required_next_gates: vec![
                "add an operator-approved write-enable fixture before any filesystem persistence".into(),
                "bind sink acceptance to fresh live gate evidence and active binary SHA".into(),
                "require release-governance approval before any public artifact path is opened".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_no_write_sink_adapter_surface(
    name: &str,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface {
    HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface {
        name: name.into(),
        required: true,
        ready: true,
        side_effect_free: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_no_write_sink_adapter_surfaces()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface> {
    vec![
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "redacted_invocation_acceptance",
            "accepts redacted invocation fixtures as validation input",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "payload_hash_binding",
            "binds acceptance to a receipt payload hash without reading raw evidence",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "redacted_output_path_binding",
            "tracks intended receipt output paths only as redacted values",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "operator_approval_requirement",
            "requires explicit operator approval before any future write path",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "fresh_trusted_record_requirement",
            "requires fresh trusted evidence before persistence can be enabled",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "public_claim_artifact_rejection",
            "rejects public claim and release artifact requests by default",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport {
    pub fn native_default() -> Self {
        let no_write_sink =
            HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport::native_default(
            );
        let write_enable_fixtures = default_activation_evidence_receipt_write_enable_fixtures();
        let required_write_enable_fixture_count = 3;
        let write_enable_fixture_count = write_enable_fixtures.len();
        let blocked_write_enable_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "blocked")
            .count();
        let allowed_write_enable_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "allowed")
            .count();
        let explicit_write_enable_requested_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_write_enable_requested)
            .count();
        let operator_approved_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_bound)
            .count();
        let fresh_trusted_record_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_trusted_record_count == 8)
            .count();
        let active_binary_sha_bound_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let public_claim_attempt_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let public_artifact_policy_satisfied_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.public_artifact_policy_satisfied)
            .count();
        let filesystem_persistence_allowed_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let workspace_write_performed_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let write_enable_fixture_contract_ready = no_write_sink.no_write_sink_adapter_ready
            && write_enable_fixture_count == required_write_enable_fixture_count
            && blocked_write_enable_fixture_count == required_write_enable_fixture_count
            && allowed_write_enable_fixture_count == 0
            && explicit_write_enable_requested_fixture_count == 3
            && operator_approved_fixture_count == 2
            && activation_request_bound_fixture_count == 3
            && fresh_trusted_record_fixture_count == 2
            && active_binary_sha_bound_fixture_count == 3
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && public_artifact_policy_satisfied_fixture_count == 2
            && filesystem_persistence_allowed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_write_enable_fixture = true;
        let activation_allowed_by_write_enable_fixture = false;

        Self {
            product: "Hepta".into(),
            status: if write_enable_fixture_contract_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            write_enable_fixture_id:
                "upstream-codex-activation-evidence-receipt-write-enable-fixture".into(),
            write_enable_fixture_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_WRITE_ENABLE_FIXTURE.md"
                    .into(),
            upstream_repository: no_write_sink.upstream_repository,
            candidate_diff_range: no_write_sink.candidate_diff_range,
            source_no_write_sink_adapter_gate: no_write_sink.no_write_sink_adapter_contract_gate,
            write_enable_fixture_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
                    .into(),
            active_dependency_isolation_gate: no_write_sink.active_dependency_isolation_gate,
            source_no_write_sink_adapter_ready: no_write_sink.no_write_sink_adapter_ready,
            required_write_enable_fixture_count,
            write_enable_fixture_count,
            blocked_write_enable_fixture_count,
            allowed_write_enable_fixture_count,
            explicit_write_enable_requested_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            fresh_trusted_record_fixture_count,
            active_binary_sha_bound_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            public_artifact_policy_satisfied_fixture_count,
            filesystem_persistence_allowed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            write_enable_fixture_contract_ready,
            activation_blocked_by_write_enable_fixture,
            activation_allowed_by_write_enable_fixture,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            write_enable_fixtures,
            write_enable_fixture_invariants: vec![
                "explicit write-enable requests are modeled before any real write path exists"
                    .into(),
                "operator approval alone is insufficient without fresh trusted records".into(),
                "fresh trusted records are insufficient without operator approval".into(),
                "public-claim or release-artifact requests keep filesystem persistence blocked"
                    .into(),
            ],
            required_next_gates: vec![
                "bind write-enable fixtures to fresh live gate evidence and active binary SHA"
                    .into(),
                "add a redacted receipt materialization dry run before filesystem writes".into(),
                "require release-governance approval before public artifact persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_write_enable_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    operator_approval_recorded: bool,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    public_artifact_policy_satisfied: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        explicit_write_enable_requested: true,
        operator_approval_recorded,
        activation_request_bound: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        active_binary_sha_bound: true,
        public_claim_requested,
        release_artifact_write_requested,
        public_artifact_policy_satisfied,
        validation_status: "blocked".into(),
        filesystem_persistence_allowed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_write_enable_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture> {
    vec![
        activation_evidence_receipt_write_enable_fixture(
            "write-enable-without-operator-approval",
            "missing_operator_approval",
            false,
            8,
            false,
            false,
            true,
            "explicit write-enable request is blocked because operator approval is absent",
        ),
        activation_evidence_receipt_write_enable_fixture(
            "operator-approved-stale-evidence-write-enable",
            "operator_approved_stale_evidence",
            true,
            0,
            false,
            false,
            true,
            "operator approval is blocked because trusted records are not fresh",
        ),
        activation_evidence_receipt_write_enable_fixture(
            "public-artifact-write-enable-attempt",
            "public_artifact_write_attempt",
            true,
            8,
            true,
            true,
            false,
            "public claim and release artifact requests require separate release-governance approval",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport {
    pub fn native_default() -> Self {
        let write_enable =
            HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport::native_default();
        let materialization_fixtures =
            default_activation_evidence_receipt_materialization_dry_run_fixtures();
        let required_materialization_fixture_count = 3;
        let materialization_fixture_count = materialization_fixtures.len();
        let blocked_materialization_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.dry_run_status == "blocked_dry_run")
            .count();
        let allowed_materialization_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.dry_run_status == "allowed")
            .count();
        let explicit_write_enable_requested_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_write_enable_requested)
            .count();
        let operator_approved_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_bound)
            .count();
        let fresh_trusted_record_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_trusted_record_count == 8)
            .count();
        let active_binary_sha_bound_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let payload_hash_planned_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.payload_hash_planned)
            .count();
        let redacted_output_path_planned_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.redacted_output_path_planned)
            .count();
        let deterministic_materialization_plan_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.deterministic_materialization_plan)
            .count();
        let public_claim_attempt_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let public_artifact_policy_satisfied_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.public_artifact_policy_satisfied)
            .count();
        let filesystem_persistence_allowed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let materialization_executed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.materialization_executed)
            .count();
        let workspace_write_performed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let materialization_dry_run_ready = write_enable.write_enable_fixture_contract_ready
            && materialization_fixture_count == required_materialization_fixture_count
            && blocked_materialization_fixture_count == required_materialization_fixture_count
            && allowed_materialization_fixture_count == 0
            && explicit_write_enable_requested_fixture_count == 3
            && operator_approved_fixture_count == 2
            && activation_request_bound_fixture_count == 3
            && fresh_trusted_record_fixture_count == 2
            && active_binary_sha_bound_fixture_count == 3
            && payload_hash_planned_fixture_count == 3
            && redacted_output_path_planned_fixture_count == 3
            && deterministic_materialization_plan_count == 3
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && public_artifact_policy_satisfied_fixture_count == 2
            && filesystem_persistence_allowed_count == 0
            && materialization_executed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_materialization_dry_run = true;
        let activation_allowed_by_materialization_dry_run = false;

        Self {
            product: "Hepta".into(),
            status: if materialization_dry_run_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            materialization_dry_run_id:
                "upstream-codex-activation-evidence-receipt-materialization-dry-run".into(),
            materialization_dry_run_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_MATERIALIZATION_DRY_RUN.md"
                    .into(),
            upstream_repository: write_enable.upstream_repository,
            candidate_diff_range: write_enable.candidate_diff_range,
            source_write_enable_fixture_gate: write_enable.write_enable_fixture_gate,
            materialization_dry_run_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
                    .into(),
            active_dependency_isolation_gate: write_enable.active_dependency_isolation_gate,
            source_write_enable_fixture_ready: write_enable.write_enable_fixture_contract_ready,
            required_materialization_fixture_count,
            materialization_fixture_count,
            blocked_materialization_fixture_count,
            allowed_materialization_fixture_count,
            explicit_write_enable_requested_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            fresh_trusted_record_fixture_count,
            active_binary_sha_bound_fixture_count,
            payload_hash_planned_fixture_count,
            redacted_output_path_planned_fixture_count,
            deterministic_materialization_plan_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            public_artifact_policy_satisfied_fixture_count,
            filesystem_persistence_allowed_count,
            materialization_executed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            materialization_dry_run_ready,
            activation_blocked_by_materialization_dry_run,
            activation_allowed_by_materialization_dry_run,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            materialization_fixtures,
            materialization_invariants: vec![
                "redacted receipt materialization is planned without executing persistence".into(),
                "payload hashes and redacted output paths are deterministic dry-run fields".into(),
                "write-enable requests still cannot cross the filesystem boundary".into(),
                "public-claim or release-artifact requests keep materialization blocked".into(),
            ],
            required_next_gates: vec![
                "bind materialization dry runs to fresh live evidence records".into(),
                "add a filesystem persistence approval packet before any workspace write".into(),
                "require release-governance approval before public artifact persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_materialization_dry_run_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    operator_approval_recorded: bool,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    public_artifact_policy_satisfied: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        explicit_write_enable_requested: true,
        operator_approval_recorded,
        activation_request_bound: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        active_binary_sha_bound: true,
        payload_hash_planned: true,
        redacted_output_path_planned: true,
        deterministic_materialization_plan: true,
        public_claim_requested,
        release_artifact_write_requested,
        public_artifact_policy_satisfied,
        dry_run_status: "blocked_dry_run".into(),
        filesystem_persistence_allowed: false,
        materialization_executed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_materialization_dry_run_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture> {
    vec![
        activation_evidence_receipt_materialization_dry_run_fixture(
            "materialization-without-operator-approval",
            "missing_operator_approval",
            false,
            8,
            false,
            false,
            true,
            "materialization dry run is blocked because operator approval is absent",
        ),
        activation_evidence_receipt_materialization_dry_run_fixture(
            "operator-approved-stale-materialization",
            "operator_approved_stale_evidence",
            true,
            0,
            false,
            false,
            true,
            "materialization dry run is blocked because trusted records are not fresh",
        ),
        activation_evidence_receipt_materialization_dry_run_fixture(
            "public-artifact-materialization-attempt",
            "public_artifact_attempt",
            true,
            8,
            true,
            true,
            false,
            "public claim and release artifact requests require separate release-governance approval",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport {
    pub fn native_default() -> Self {
        let materialization =
            HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport::native_default(
            );
        let approval_fields =
            default_activation_evidence_receipt_filesystem_persistence_approval_fields();
        let required_approval_field_count = 12;
        let approval_field_count = approval_fields.len();
        let recorded_approval_field_count = approval_fields
            .iter()
            .filter(|field| field.recorded_by_default)
            .count();
        let redacted_or_hashed_field_count = approval_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let required_for_filesystem_persistence_field_count = approval_fields
            .iter()
            .filter(|field| field.required_for_filesystem_persistence)
            .count();
        let operator_approval_required = true;
        let operator_approval_recorded = false;
        let activation_request_required = true;
        let activation_request_recorded = false;
        let materialization_plan_required = true;
        let materialization_plan_recorded = false;
        let fresh_trusted_records_required = true;
        let fresh_trusted_records_recorded = false;
        let active_binary_sha_required = true;
        let active_binary_sha_recorded = false;
        let public_artifact_policy_required = true;
        let public_artifact_policy_recorded = false;
        let filesystem_persistence_approval_packet_ready = materialization
            .materialization_dry_run_ready
            && approval_field_count == required_approval_field_count
            && recorded_approval_field_count == 0
            && redacted_or_hashed_field_count == 10
            && required_for_filesystem_persistence_field_count == required_approval_field_count
            && operator_approval_required
            && !operator_approval_recorded
            && activation_request_required
            && !activation_request_recorded
            && materialization_plan_required
            && !materialization_plan_recorded
            && fresh_trusted_records_required
            && !fresh_trusted_records_recorded
            && active_binary_sha_required
            && !active_binary_sha_recorded
            && public_artifact_policy_required
            && !public_artifact_policy_recorded;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_filesystem_persistence_approval = true;
        let activation_allowed_by_filesystem_persistence_approval = false;

        Self {
            product: "Hepta".into(),
            status: if filesystem_persistence_approval_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_persistence_approval_packet_id:
                "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
                    .into(),
            filesystem_persistence_approval_packet_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_APPROVAL_PACKET.md"
                    .into(),
            upstream_repository: materialization.upstream_repository,
            candidate_diff_range: materialization.candidate_diff_range,
            source_materialization_dry_run_gate: materialization.materialization_dry_run_gate,
            filesystem_persistence_approval_packet_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
                    .into(),
            active_dependency_isolation_gate: materialization.active_dependency_isolation_gate,
            source_materialization_dry_run_ready: materialization.materialization_dry_run_ready,
            required_approval_field_count,
            approval_field_count,
            recorded_approval_field_count,
            redacted_or_hashed_field_count,
            required_for_filesystem_persistence_field_count,
            operator_approval_required,
            operator_approval_recorded,
            activation_request_required,
            activation_request_recorded,
            materialization_plan_required,
            materialization_plan_recorded,
            fresh_trusted_records_required,
            fresh_trusted_records_recorded,
            active_binary_sha_required,
            active_binary_sha_recorded,
            public_artifact_policy_required,
            public_artifact_policy_recorded,
            filesystem_persistence_approval_packet_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_filesystem_persistence_approval,
            activation_allowed_by_filesystem_persistence_approval,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            approval_fields,
            approval_packet_invariants: vec![
                "filesystem persistence requires a complete approval packet before any workspace write"
                    .into(),
                "approval packet fields are schema-only and unrecorded by default".into(),
                "materialization plans are not execution authority".into(),
                "public claim and release artifact decisions stay denied without release-governance approval"
                    .into(),
            ],
            required_next_gates: vec![
                "add a filesystem output path allowlist before any receipt write".into(),
                "bind approval packets to fresh live evidence and active binary SHA".into(),
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport {
    pub fn native_default() -> Self {
        let approval =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport::native_default();
        let allowlist_entries =
            default_activation_evidence_receipt_filesystem_output_path_allowlist_entries();
        let required_allowlist_entry_count = 6;
        let allowlist_entry_count = allowlist_entries.len();
        let allowed_output_path_entry_count = allowlist_entries
            .iter()
            .filter(|entry| entry.allowed_for_receipt_persistence)
            .count();
        let blocked_output_path_entry_count =
            allowlist_entry_count.saturating_sub(allowed_output_path_entry_count);
        let redacted_output_path_entry_count = allowlist_entries
            .iter()
            .filter(|entry| entry.redacted_path.starts_with("<redacted:"))
            .count();
        let default_selected_output_path_count = 0;
        let source_tree_path_allowed = false;
        let home_directory_path_allowed = false;
        let release_artifact_path_allowed = false;
        let public_artifact_path_allowed = false;
        let receipt_output_path_allowlist_ready = approval
            .filesystem_persistence_approval_packet_ready
            && allowlist_entry_count == required_allowlist_entry_count
            && allowed_output_path_entry_count == 3
            && blocked_output_path_entry_count == 3
            && redacted_output_path_entry_count == required_allowlist_entry_count
            && default_selected_output_path_count == 0
            && !source_tree_path_allowed
            && !home_directory_path_allowed
            && !release_artifact_path_allowed
            && !public_artifact_path_allowed;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_output_path_allowlist = true;
        let activation_allowed_by_output_path_allowlist = false;

        Self {
            product: "Hepta".into(),
            status: if receipt_output_path_allowlist_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_output_path_allowlist_id:
                "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
                    .into(),
            filesystem_output_path_allowlist_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_ALLOWLIST.md"
                    .into(),
            upstream_repository: approval.upstream_repository,
            candidate_diff_range: approval.candidate_diff_range,
            source_filesystem_persistence_approval_packet_gate: approval
                .filesystem_persistence_approval_packet_gate,
            filesystem_output_path_allowlist_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
                    .into(),
            active_dependency_isolation_gate: approval.active_dependency_isolation_gate,
            source_filesystem_persistence_approval_packet_ready: approval
                .filesystem_persistence_approval_packet_ready,
            required_allowlist_entry_count,
            allowlist_entry_count,
            allowed_output_path_entry_count,
            blocked_output_path_entry_count,
            redacted_output_path_entry_count,
            default_selected_output_path_count,
            source_tree_path_allowed,
            home_directory_path_allowed,
            release_artifact_path_allowed,
            public_artifact_path_allowed,
            receipt_output_path_allowlist_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_output_path_allowlist,
            activation_allowed_by_output_path_allowlist,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            allowlist_entries,
            allowlist_invariants: vec![
                "receipt output paths must match an allowlisted redacted root before any filesystem persistence"
                    .into(),
                "source tree, home directory, release artifact, and public artifact paths are not receipt persistence targets"
                    .into(),
                "no output path is selected by default".into(),
                "allowlist readiness is not filesystem write authority".into(),
            ],
            required_next_gates: vec![
                "bind allowlisted output paths to fresh live evidence and active binary SHA".into(),
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport {
    pub fn native_default() -> Self {
        let allowlist =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport::native_default();
        let path_bindings =
            default_activation_evidence_receipt_filesystem_output_path_evidence_bindings();
        let required_path_binding_count = 8;
        let path_binding_count = path_bindings.len();
        let allowed_output_path_entry_count = allowlist.allowed_output_path_entry_count;
        let selected_output_path_count = 0;
        let recorded_path_binding_count = path_bindings
            .iter()
            .filter(|binding| binding.recorded_by_default)
            .count();
        let fresh_live_evidence_bound_count = 0;
        let active_binary_sha_bound_count = 0;
        let redacted_or_hashed_binding_count = path_bindings
            .iter()
            .filter(|binding| binding.redacted_or_hashed)
            .count();
        let trusted_source_bound_count = 0;
        let source_tree_path_binding_allowed = false;
        let home_directory_path_binding_allowed = false;
        let release_artifact_path_binding_allowed = false;
        let public_artifact_path_binding_allowed = false;
        let output_path_evidence_binding_ready = allowlist.receipt_output_path_allowlist_ready
            && path_binding_count == required_path_binding_count
            && allowed_output_path_entry_count == 3
            && selected_output_path_count == 0
            && recorded_path_binding_count == 0
            && fresh_live_evidence_bound_count == 0
            && active_binary_sha_bound_count == 0
            && redacted_or_hashed_binding_count == required_path_binding_count
            && trusted_source_bound_count == 0
            && path_bindings.iter().all(|binding| {
                binding.binding_required
                    && binding.requires_fresh_live_evidence
                    && binding.requires_active_binary_sha
            })
            && !source_tree_path_binding_allowed
            && !home_directory_path_binding_allowed
            && !release_artifact_path_binding_allowed
            && !public_artifact_path_binding_allowed;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_output_path_evidence_binding = true;
        let activation_allowed_by_output_path_evidence_binding = false;

        Self {
            product: "Hepta".into(),
            status: if output_path_evidence_binding_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_output_path_evidence_binding_id:
                "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
                    .into(),
            filesystem_output_path_evidence_binding_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_EVIDENCE_BINDING.md"
                    .into(),
            upstream_repository: allowlist.upstream_repository,
            candidate_diff_range: allowlist.candidate_diff_range,
            source_filesystem_output_path_allowlist_gate: allowlist
                .filesystem_output_path_allowlist_gate,
            filesystem_output_path_evidence_binding_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
                    .into(),
            active_dependency_isolation_gate: allowlist.active_dependency_isolation_gate,
            source_filesystem_output_path_allowlist_ready: allowlist
                .receipt_output_path_allowlist_ready,
            required_path_binding_count,
            path_binding_count,
            allowed_output_path_entry_count,
            selected_output_path_count,
            recorded_path_binding_count,
            fresh_live_evidence_bound_count,
            active_binary_sha_bound_count,
            redacted_or_hashed_binding_count,
            trusted_source_bound_count,
            source_tree_path_binding_allowed,
            home_directory_path_binding_allowed,
            release_artifact_path_binding_allowed,
            public_artifact_path_binding_allowed,
            output_path_evidence_binding_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_output_path_evidence_binding,
            activation_allowed_by_output_path_evidence_binding,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            path_bindings,
            binding_invariants: vec![
                "allowlisted receipt output paths require fresh live evidence binding before destination selection"
                    .into(),
                "active binary SHA binding is required before any filesystem persistence".into(),
                "path evidence binding is schema-only and unrecorded by default".into(),
                "source tree, home directory, release artifact, and public artifact paths remain blocked"
                    .into(),
            ],
            required_next_gates: vec![
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
                "bind the sink preview to a deterministic redacted payload hash".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport {
    pub fn native_default() -> Self {
        let binding =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport::native_default();
        let preview_fixtures =
            default_activation_evidence_receipt_filesystem_sink_write_preview_fixtures();
        let required_preview_fixture_count = 3;
        let preview_fixture_count = preview_fixtures.len();
        let allowed_output_path_entry_count = binding.allowed_output_path_entry_count;
        let previewed_output_path_count = preview_fixtures.len();
        let deterministic_payload_hash_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.deterministic_payload_hash.starts_with("sha256:"))
            .count();
        let redacted_output_path_preview_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.redacted_output_path.starts_with("<redacted:"))
            .count();
        let fresh_live_evidence_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_live_evidence_bound)
            .count();
        let active_binary_sha_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let trusted_source_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.trusted_source_bound)
            .count();
        let operator_approval_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_bound)
            .count();
        let blocked_preview_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.preview_status == "blocked_preview")
            .count();
        let allowed_preview_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.preview_status == "allowed")
            .count();
        let public_claim_attempt_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let filesystem_persistence_allowed_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let workspace_write_performed_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let sink_write_preview_ready = binding.output_path_evidence_binding_ready
            && preview_fixture_count == required_preview_fixture_count
            && allowed_output_path_entry_count == 3
            && previewed_output_path_count == required_preview_fixture_count
            && deterministic_payload_hash_count == required_preview_fixture_count
            && redacted_output_path_preview_count == required_preview_fixture_count
            && fresh_live_evidence_bound_fixture_count == required_preview_fixture_count
            && active_binary_sha_bound_fixture_count == required_preview_fixture_count
            && trusted_source_bound_fixture_count == required_preview_fixture_count
            && operator_approval_bound_fixture_count == required_preview_fixture_count
            && blocked_preview_fixture_count == required_preview_fixture_count
            && allowed_preview_fixture_count == 0
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && filesystem_persistence_allowed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_sink_write_preview = true;
        let activation_allowed_by_sink_write_preview = false;

        Self {
            product: "Hepta".into(),
            status: if sink_write_preview_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_sink_write_preview_id:
                "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
                    .into(),
            filesystem_sink_write_preview_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_SINK_WRITE_PREVIEW.md"
                    .into(),
            upstream_repository: binding.upstream_repository,
            candidate_diff_range: binding.candidate_diff_range,
            source_filesystem_output_path_evidence_binding_gate: binding
                .filesystem_output_path_evidence_binding_gate,
            filesystem_sink_write_preview_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
                    .into(),
            active_dependency_isolation_gate: binding.active_dependency_isolation_gate,
            source_filesystem_output_path_evidence_binding_ready: binding
                .output_path_evidence_binding_ready,
            required_preview_fixture_count,
            preview_fixture_count,
            allowed_output_path_entry_count,
            previewed_output_path_count,
            deterministic_payload_hash_count,
            redacted_output_path_preview_count,
            fresh_live_evidence_bound_fixture_count,
            active_binary_sha_bound_fixture_count,
            trusted_source_bound_fixture_count,
            operator_approval_bound_fixture_count,
            blocked_preview_fixture_count,
            allowed_preview_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            filesystem_persistence_allowed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            sink_write_preview_ready,
            activation_blocked_by_sink_write_preview,
            activation_allowed_by_sink_write_preview,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            preview_fixtures,
            preview_invariants: vec![
                "sink write previews can select only redacted allowlisted roots".into(),
                "deterministic payload hashes are preview evidence, not write authority".into(),
                "filesystem persistence remains disabled until a separate execution gate exists"
                    .into(),
                "public release and artifact attempts stay blocked by release governance".into(),
            ],
            required_next_gates: vec![
                "add a filesystem persistence execution-denial matrix before any workspace write"
                    .into(),
                "bind preview payload hashes to a future explicit persistence approval id".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport {
    pub fn native_default() -> Self {
        let preview =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport::native_default();
        let denial_fixtures =
            default_activation_evidence_receipt_filesystem_persistence_execution_denial_fixtures();
        let required_denial_fixture_count = 4;
        let denial_fixture_count = denial_fixtures.len();
        let source_preview_fixture_count = preview.preview_fixture_count;
        let execution_requested_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_requested)
            .count();
        let future_persistence_approval_slot_count = denial_fixtures
            .iter()
            .filter(|fixture| {
                fixture
                    .future_persistence_approval_id_slot
                    .starts_with("<future:")
            })
            .count();
        let explicit_persistence_approval_id_present_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_persistence_approval_id_present)
            .count();
        let explicit_persistence_approval_id_missing_count =
            denial_fixture_count - explicit_persistence_approval_id_present_count;
        let stale_or_missing_fresh_evidence_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| !fixture.fresh_live_evidence_bound)
            .count();
        let active_binary_sha_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let trusted_source_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.trusted_source_bound)
            .count();
        let operator_approval_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_bound)
            .count();
        let workspace_path_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_path_requested)
            .count();
        let public_claim_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let blocked_execution_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_status == "blocked_execution")
            .count();
        let allowed_execution_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_status == "allowed")
            .count();
        let filesystem_persistence_allowed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let filesystem_persistence_execution_performed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_execution_performed)
            .count();
        let workspace_write_performed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let execution_denial_matrix_ready = preview.sink_write_preview_ready
            && denial_fixture_count == required_denial_fixture_count
            && source_preview_fixture_count == 3
            && execution_requested_fixture_count == required_denial_fixture_count
            && future_persistence_approval_slot_count == required_denial_fixture_count
            && explicit_persistence_approval_id_present_count == 3
            && explicit_persistence_approval_id_missing_count == 1
            && stale_or_missing_fresh_evidence_fixture_count == 1
            && active_binary_sha_bound_fixture_count == required_denial_fixture_count
            && trusted_source_bound_fixture_count == required_denial_fixture_count
            && operator_approval_bound_fixture_count == 3
            && workspace_path_attempt_fixture_count == 1
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && blocked_execution_fixture_count == required_denial_fixture_count
            && allowed_execution_fixture_count == 0
            && filesystem_persistence_allowed_count == 0
            && filesystem_persistence_execution_performed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_execution_denial_matrix = true;
        let activation_allowed_by_execution_denial_matrix = false;

        Self {
            product: "Hepta".into(),
            status: if execution_denial_matrix_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_persistence_execution_denial_matrix_id:
                "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
                    .into(),
            filesystem_persistence_execution_denial_matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_EXECUTION_DENIAL_MATRIX.md"
                    .into(),
            upstream_repository: preview.upstream_repository,
            candidate_diff_range: preview.candidate_diff_range,
            source_filesystem_sink_write_preview_gate: preview.filesystem_sink_write_preview_gate,
            filesystem_persistence_execution_denial_matrix_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: preview.active_dependency_isolation_gate,
            source_filesystem_sink_write_preview_ready: preview.sink_write_preview_ready,
            required_denial_fixture_count,
            denial_fixture_count,
            source_preview_fixture_count,
            execution_requested_fixture_count,
            future_persistence_approval_slot_count,
            explicit_persistence_approval_id_present_count,
            explicit_persistence_approval_id_missing_count,
            stale_or_missing_fresh_evidence_fixture_count,
            active_binary_sha_bound_fixture_count,
            trusted_source_bound_fixture_count,
            operator_approval_bound_fixture_count,
            workspace_path_attempt_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            blocked_execution_fixture_count,
            allowed_execution_fixture_count,
            filesystem_persistence_allowed_count,
            filesystem_persistence_execution_performed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            execution_denial_matrix_ready,
            activation_blocked_by_execution_denial_matrix,
            activation_allowed_by_execution_denial_matrix,
            active_wiring_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            denial_fixtures,
            denial_invariants: vec![
                "preview payload hashes are bound to future persistence approval slots, not write authority"
                    .into(),
                "missing approval id, stale evidence, workspace path attempts, and public artifact attempts all deny execution"
                    .into(),
                "filesystem persistence execution remains disabled by default".into(),
                "no workspace write or evidence receipt persistence occurs in the denial matrix".into(),
            ],
            required_next_gates: vec![
                "add a receipt persistence executor dry-run that consumes the denial matrix without writing"
                    .into(),
                "require explicit persistence approval id materialization before any filesystem write"
                    .into(),
                "keep public artifact writes behind release-governance approval".into(),
            ],
        }
    }
}

fn filesystem_persistence_approval_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField {
        name: name.into(),
        redacted_or_hashed,
        required_for_filesystem_persistence: true,
        recorded_by_default: false,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_persistence_approval_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField> {
    vec![
        filesystem_persistence_approval_field(
            "filesystem_persistence_approval_id",
            false,
            "unique operator-reviewed approval packet identifier",
        ),
        filesystem_persistence_approval_field(
            "activation_request_id",
            true,
            "binds the persistence request to the activation request packet",
        ),
        filesystem_persistence_approval_field(
            "operator_approval_id",
            true,
            "binds persistence to explicit operator approval",
        ),
        filesystem_persistence_approval_field(
            "operator_identity_hash",
            true,
            "records a redacted operator identity binding",
        ),
        filesystem_persistence_approval_field(
            "materialization_plan_id",
            true,
            "binds the write decision to a deterministic dry-run materialization plan",
        ),
        filesystem_persistence_approval_field(
            "receipt_payload_hash",
            true,
            "binds the approved write to the redacted receipt payload hash",
        ),
        filesystem_persistence_approval_field(
            "redacted_output_path",
            true,
            "records the intended output path without exposing private filesystem details",
        ),
        filesystem_persistence_approval_field(
            "accepted_trusted_record_ids",
            true,
            "binds persistence to accepted trusted evidence records",
        ),
        filesystem_persistence_approval_field(
            "fresh_trusted_record_ids",
            true,
            "binds persistence to fresh trusted evidence records",
        ),
        filesystem_persistence_approval_field(
            "active_binary_sha256",
            true,
            "binds persistence to the active Hepta binary under verification",
        ),
        filesystem_persistence_approval_field(
            "rollback_plan_id",
            true,
            "binds persistence to an operator-visible rollback plan",
        ),
        filesystem_persistence_approval_field(
            "public_claim_and_artifact_decision",
            false,
            "keeps public release claims and artifact writes separately approved",
        ),
    ]
}

fn filesystem_output_path_allowlist_entry(
    name: &str,
    redacted_path: &str,
    allowed_for_receipt_persistence: bool,
    blocked_for_public_artifact: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry {
        name: name.into(),
        redacted_path: redacted_path.into(),
        allowed_for_receipt_persistence,
        blocked_for_public_artifact,
        requires_operator_approval: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_output_path_allowlist_entries()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry> {
    vec![
        filesystem_output_path_allowlist_entry(
            "activation_evidence_receipts_root",
            "<redacted:hepta-activation-evidence-receipts>",
            true,
            true,
            "bounded local receipt sink for operator-approved activation evidence",
        ),
        filesystem_output_path_allowlist_entry(
            "activation_evidence_dry_run_root",
            "<redacted:hepta-activation-evidence-dry-run>",
            true,
            true,
            "bounded local dry-run sink for receipt write previews",
        ),
        filesystem_output_path_allowlist_entry(
            "activation_evidence_operator_packet_root",
            "<redacted:hepta-operator-activation-packets>",
            true,
            true,
            "bounded local operator packet sink for redacted evidence references",
        ),
        filesystem_output_path_allowlist_entry(
            "source_tree_root",
            "<redacted:hepta-source-tree>",
            false,
            true,
            "source tree paths are not receipt persistence targets",
        ),
        filesystem_output_path_allowlist_entry(
            "home_directory_root",
            "<redacted:home-directory>",
            false,
            true,
            "home directory paths are never direct receipt persistence targets",
        ),
        filesystem_output_path_allowlist_entry(
            "release_artifact_root",
            "<redacted:release-artifact-root>",
            false,
            true,
            "release artifact paths require separate release-governance approval",
        ),
    ]
}

fn filesystem_output_path_evidence_binding(
    evidence_id: &str,
    allowed_output_path_entry_name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding {
        evidence_id: evidence_id.into(),
        allowed_output_path_entry_name: allowed_output_path_entry_name.into(),
        binding_required: true,
        recorded_by_default: false,
        redacted_or_hashed,
        requires_fresh_live_evidence: true,
        requires_active_binary_sha: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_output_path_evidence_bindings()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding> {
    vec![
        filesystem_output_path_evidence_binding(
            "activation_request_id",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to the activation request packet",
        ),
        filesystem_output_path_evidence_binding(
            "operator_approval_id",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to explicit operator approval",
        ),
        filesystem_output_path_evidence_binding(
            "operator_identity_hash",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to a redacted operator identity",
        ),
        filesystem_output_path_evidence_binding(
            "live_dependency_isolation_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh live dependency isolation evidence",
        ),
        filesystem_output_path_evidence_binding(
            "watchdog_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh watchdog evidence",
        ),
        filesystem_output_path_evidence_binding(
            "browser_smoke_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh browser visual smoke evidence",
        ),
        filesystem_output_path_evidence_binding(
            "long_soak_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh long-soak evidence",
        ),
        filesystem_output_path_evidence_binding(
            "rollback_plan_id",
            "activation_evidence_dry_run_root",
            true,
            "binds the selected output root to the rollback plan before any persistence",
        ),
    ]
}

fn filesystem_sink_write_preview_fixture(
    fixture_id: &str,
    allowed_output_path_entry_name: &str,
    redacted_output_path: &str,
    deterministic_payload_hash: &str,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture {
        fixture_id: fixture_id.into(),
        allowed_output_path_entry_name: allowed_output_path_entry_name.into(),
        redacted_output_path: redacted_output_path.into(),
        deterministic_payload_hash: deterministic_payload_hash.into(),
        fresh_live_evidence_bound: true,
        active_binary_sha_bound: true,
        trusted_source_bound: true,
        operator_approval_bound: true,
        public_claim_requested,
        release_artifact_write_requested,
        preview_status: "blocked_preview".into(),
        filesystem_persistence_allowed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
    }
}

fn default_activation_evidence_receipt_filesystem_sink_write_preview_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture> {
    vec![
        filesystem_sink_write_preview_fixture(
            "receipt-root-sink-write-preview",
            "activation_evidence_receipts_root",
            "<redacted:hepta-activation-evidence-receipts/receipt-preview.json>",
            "sha256:preview-receipt-root-payload",
            false,
            false,
        ),
        filesystem_sink_write_preview_fixture(
            "dry-run-root-sink-write-preview",
            "activation_evidence_dry_run_root",
            "<redacted:hepta-activation-evidence-dry-run/receipt-preview.json>",
            "sha256:preview-dry-run-root-payload",
            false,
            false,
        ),
        filesystem_sink_write_preview_fixture(
            "public-artifact-sink-write-preview-attempt",
            "activation_evidence_operator_packet_root",
            "<redacted:hepta-operator-activation-packets/public-artifact-attempt.json>",
            "sha256:preview-public-artifact-attempt-payload",
            true,
            true,
        ),
    ]
}

struct FilesystemPersistenceExecutionDenialFixtureSpec<'a> {
    fixture_id: &'a str,
    source_preview_fixture_id: &'a str,
    deterministic_payload_hash: &'a str,
    future_persistence_approval_id_slot: &'a str,
    explicit_persistence_approval_id_present: bool,
    fresh_live_evidence_bound: bool,
    operator_approval_bound: bool,
    workspace_path_requested: bool,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &'a str,
}

fn filesystem_persistence_execution_denial_fixture(
    spec: FilesystemPersistenceExecutionDenialFixtureSpec<'_>,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture {
        fixture_id: spec.fixture_id.into(),
        source_preview_fixture_id: spec.source_preview_fixture_id.into(),
        deterministic_payload_hash: spec.deterministic_payload_hash.into(),
        future_persistence_approval_id_slot: spec.future_persistence_approval_id_slot.into(),
        execution_requested: true,
        explicit_persistence_approval_id_present: spec.explicit_persistence_approval_id_present,
        fresh_live_evidence_bound: spec.fresh_live_evidence_bound,
        active_binary_sha_bound: true,
        trusted_source_bound: true,
        operator_approval_bound: spec.operator_approval_bound,
        workspace_path_requested: spec.workspace_path_requested,
        public_claim_requested: spec.public_claim_requested,
        release_artifact_write_requested: spec.release_artifact_write_requested,
        denial_reason: spec.denial_reason.into(),
        execution_status: "blocked_execution".into(),
        filesystem_persistence_allowed: false,
        filesystem_persistence_execution_performed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
    }
}

fn default_activation_evidence_receipt_filesystem_persistence_execution_denial_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture> {
    vec![
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "missing-persistence-approval-id-execution-attempt",
                source_preview_fixture_id: "receipt-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-receipt-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:receipt-root>",
                explicit_persistence_approval_id_present: false,
                fresh_live_evidence_bound: true,
                operator_approval_bound: false,
                workspace_path_requested: false,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "explicit persistence approval id is absent",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "stale-live-evidence-execution-attempt",
                source_preview_fixture_id: "dry-run-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-dry-run-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:dry-run-root>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: false,
                operator_approval_bound: true,
                workspace_path_requested: false,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "fresh live evidence binding is stale or missing",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "workspace-path-execution-attempt",
                source_preview_fixture_id: "receipt-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-receipt-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:workspace-path>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: true,
                operator_approval_bound: true,
                workspace_path_requested: true,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "workspace path write is outside the receipt sink authority",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "public-artifact-execution-attempt",
                source_preview_fixture_id: "public-artifact-sink-write-preview-attempt",
                deterministic_payload_hash: "sha256:preview-public-artifact-attempt-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:public-artifact>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: true,
                operator_approval_bound: true,
                workspace_path_requested: false,
                public_claim_requested: true,
                release_artifact_write_requested: true,
                denial_reason: "public release and artifact writes require separate release governance",
            },
        ),
    ]
}

fn activation_request_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationRequestPacketField {
    HeptaUpstreamCodexActivationRequestPacketField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_request_packet_fields() -> Vec<HeptaUpstreamCodexActivationRequestPacketField>
{
    vec![
        activation_request_field(
            "activation_request_id",
            false,
            "unique request id binding the activation review",
        ),
        activation_request_field(
            "operator_approval_id",
            false,
            "explicit approval record for the requested activation",
        ),
        activation_request_field(
            "operator_identity_hash",
            true,
            "hashed operator identity without exposing private account details",
        ),
        activation_request_field(
            "approved_bucket_ids",
            false,
            "upstream Codex diff buckets approved for activation consideration",
        ),
        activation_request_field(
            "approved_surface_ids",
            false,
            "Hepta surfaces approved for active wiring consideration",
        ),
        activation_request_field(
            "requested_runtime_wiring_scope",
            false,
            "bounded active runtime code path requested for wiring",
        ),
        activation_request_field(
            "requested_dependency_change_set",
            false,
            "explicit dependency changes requested for the active service",
        ),
        activation_request_field(
            "live_dependency_isolation_evidence_id",
            false,
            "fresh active-service dependency isolation evidence",
        ),
        activation_request_field(
            "watchdog_evidence_id",
            false,
            "fresh watchdog evidence for the requested activation",
        ),
        activation_request_field(
            "browser_smoke_evidence_id",
            false,
            "fresh browser visual smoke evidence",
        ),
        activation_request_field(
            "long_soak_evidence_id",
            false,
            "fresh long-soak evidence for the requested activation",
        ),
        activation_request_field(
            "rollback_plan_id",
            false,
            "rollback anchor for the requested active wiring",
        ),
        activation_request_field(
            "public_release_claim_decision",
            false,
            "explicit decision that public release claims remain separately gated",
        ),
        activation_request_field(
            "release_artifact_write_decision",
            false,
            "explicit decision that release artifact writes remain separately gated",
        ),
    ]
}

fn sync_contract(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    title: &str,
    upstream_scope: &[&str],
    hepta_surfaces: &[&str],
    required_gate: &str,
) -> HeptaUpstreamCodexSyncContract {
    HeptaUpstreamCodexSyncContract {
        id: id.into(),
        risk,
        title: title.into(),
        upstream_scope: upstream_scope.iter().map(|value| (*value).into()).collect(),
        hepta_surfaces: hepta_surfaces.iter().map(|value| (*value).into()).collect(),
        required_gate: required_gate.into(),
        auto_apply_allowed: false,
        active_runtime_dependency_allowed: false,
        public_release_claim_allowed: false,
        contract_ready: true,
    }
}

fn default_upstream_codex_sync_contracts() -> Vec<HeptaUpstreamCodexSyncContract> {
    vec![
        sync_contract(
            "snapshot-and-diff-intake",
            HeptaUpstreamCodexSyncRisk::Guardrail,
            "Fetch upstream only into an explicit snapshot/diff lane",
            &[
                "upstream remote metadata",
                "release tags",
                "commit range summary",
                "file-level diff inventory",
            ],
            &[
                "codex-rs compatibility snapshot",
                "docs/architecture/HEPTA_UPSTREAM_CODEX_SYNC_LANE.md",
            ],
            "scripts/hepta-upstream-codex-snapshot.sh and scripts/hepta-upstream-codex-diff-ledger.sh must record the observed upstream head and classified diff range before any absorption patch",
        ),
        sync_contract(
            "provider-credential-security-classification",
            HeptaUpstreamCodexSyncRisk::P0Security,
            "Classify provider, credential, approval, sandbox, and network deltas first",
            &[
                "model provider",
                "credential and auth flows",
                "approval policy",
                "sandbox and exec",
                "network/proxy behavior",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel policy gates",
                "hepta-gateway read-only reports",
            ],
            "security/provider diffs require adapter contract review before active runtime wiring",
        ),
        sync_contract(
            "runtime-session-tool-contract-classification",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            "Classify runtime, session, thread-store, tool, MCP, and app-server deltas",
            &[
                "runtime event loop",
                "session and thread store",
                "tool invocation",
                "MCP server/client",
                "app-server protocol",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "/api/hepta-core-fusion-readiness",
                "/api/hepta-engine-dependency-closure",
            ],
            "adapter behavior-equivalence and shadow-replay gates must pass before promotion",
        ),
        sync_contract(
            "compatibility-package-retention-boundary",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            "Keep Codex compatibility as an intake surface, not the active service engine",
            &[
                "codex-cli",
                "codex-core",
                "codex-exec",
                "codex-state",
                "codex-mcp",
                "codex-app-server",
                "codex-sandboxing",
                "codex-plugin",
                "codex-model-provider",
                "codex-protocol",
                "codex-tui",
            ],
            &[
                "hepta-cli --bin hepta",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "active hepta-cli cargo tree must remain free of tracked Codex engine crates",
        ),
        sync_contract(
            "release-governance-no-public-claim",
            HeptaUpstreamCodexSyncRisk::P2Product,
            "Require governance evidence before claiming an upstream-sync release",
            &[
                "changelog",
                "release notes",
                "operator packet",
                "long soak evidence",
                "watchdog evidence",
            ],
            &[
                "public GA readiness",
                "operator approval packet",
                "watchdog",
                "live soak",
            ],
            "no public release claim until long-cycle soak and governance packet pass",
        ),
    ]
}

pub fn hepta_upstream_codex_sync_lane_report() -> HeptaUpstreamCodexSyncLaneReport {
    HeptaUpstreamCodexSyncLaneReport::native_default()
}

fn snapshot_risk_class(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    upstream_path_hints: &[&str],
    hepta_review_surfaces: &[&str],
    required_action: &str,
) -> HeptaUpstreamCodexSnapshotRiskClass {
    HeptaUpstreamCodexSnapshotRiskClass {
        id: id.into(),
        risk,
        upstream_path_hints: upstream_path_hints
            .iter()
            .map(|value| (*value).into())
            .collect(),
        hepta_review_surfaces: hepta_review_surfaces
            .iter()
            .map(|value| (*value).into())
            .collect(),
        required_action: required_action.into(),
        auto_absorb_allowed: false,
        active_runtime_dependency_allowed: false,
        classification_required: true,
    }
}

fn default_upstream_codex_snapshot_risk_classes() -> Vec<HeptaUpstreamCodexSnapshotRiskClass> {
    vec![
        snapshot_risk_class(
            "provider-credential-sandbox-security",
            HeptaUpstreamCodexSyncRisk::P0Security,
            &[
                "providers",
                "auth",
                "login",
                "credentials",
                "approval_policy",
                "sandbox",
                "exec",
                "network",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel security policy reports",
                "operator approval packet",
            ],
            "classify as P0 before any adapter or active runtime wiring",
        ),
        snapshot_risk_class(
            "runtime-session-tool-mcp-appserver",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            &[
                "runtime",
                "session",
                "thread",
                "tool",
                "mcp",
                "app-server",
                "protocol",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "adapter behavior-equivalence gate",
                "shadow replay gate",
            ],
            "require contract tests and replay evidence before promotion",
        ),
        snapshot_risk_class(
            "legacy-cli-tui-compatibility",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            &["cli", "tui", "codex-cli", "codex-tui", "legacy command"],
            &[
                "codex-cli compatibility package",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "retain only as compatibility intake unless Hepta contracts absorb it",
        ),
        snapshot_risk_class(
            "product-doc-release-governance",
            HeptaUpstreamCodexSyncRisk::P2Product,
            &[
                "docs",
                "changelog",
                "release",
                "install",
                "package metadata",
            ],
            &[
                "public GA readiness gate",
                "release-hardening status",
                "operator approval packet",
                "long-cycle soak evidence",
            ],
            "gate release claims on governance evidence, not on upstream freshness alone",
        ),
    ]
}

pub fn hepta_upstream_codex_snapshot_report() -> HeptaUpstreamCodexSnapshotReport {
    HeptaUpstreamCodexSnapshotReport::native_default()
}

fn diff_ledger_bucket(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    upstream_path_hints: &[&str],
    hepta_review_surfaces: &[&str],
    required_action: &str,
    promotion_gate: &str,
) -> HeptaUpstreamCodexDiffLedgerBucket {
    HeptaUpstreamCodexDiffLedgerBucket {
        id: id.into(),
        risk,
        upstream_path_hints: upstream_path_hints
            .iter()
            .map(|value| (*value).into())
            .collect(),
        hepta_review_surfaces: hepta_review_surfaces
            .iter()
            .map(|value| (*value).into())
            .collect(),
        required_action: required_action.into(),
        promotion_gate: promotion_gate.into(),
        auto_absorb_allowed: false,
        active_runtime_dependency_allowed: false,
        classification_required: true,
        bucket_ready: true,
    }
}

fn default_upstream_codex_diff_ledger_buckets() -> Vec<HeptaUpstreamCodexDiffLedgerBucket> {
    vec![
        diff_ledger_bucket(
            "provider-credential-sandbox-security",
            HeptaUpstreamCodexSyncRisk::P0Security,
            &[
                "codex-rs/codex-api",
                "codex-rs/model-provider",
                "codex-rs/login",
                "codex-rs/config",
                "codex-rs/*sandbox*",
                "codex-rs/exec",
                "codex-rs/network-proxy",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel security policy reports",
                "operator approval packet",
            ],
            "classify security/auth/provider/sandbox paths before any active adapter wiring",
            "security/provider diffs require adapter contract review and dependency isolation",
        ),
        diff_ledger_bucket(
            "runtime-session-tool-mcp-appserver",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            &[
                "codex-rs/app-server*",
                "codex-rs/core/src/session",
                "codex-rs/core/src/tools",
                "codex-rs/codex-mcp",
                "codex-rs/mcp-server",
                "codex-rs/thread-store",
                "codex-rs/hooks",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "adapter behavior-equivalence gate",
                "adapter shadow replay gate",
            ],
            "classify runtime/session/tool/MCP/app-server paths before promotion",
            "adapter behavior-equivalence and shadow-replay gates must pass",
        ),
        diff_ledger_bucket(
            "legacy-cli-tui-compatibility",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            &[
                "codex-rs/cli",
                "codex-rs/tui",
                "codex-rs/code-mode",
                "codex-rs/terminal-*",
            ],
            &[
                "codex-cli compatibility package",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "retain CLI/TUI deltas as compatibility intake unless Hepta contracts absorb them",
            "active hepta-cli cargo tree must stay free of tracked Codex engine crates",
        ),
        diff_ledger_bucket(
            "product-doc-release-governance",
            HeptaUpstreamCodexSyncRisk::P2Product,
            &[
                "README",
                "docs",
                "package",
                "release",
                "Cargo.lock",
                "Cargo.toml",
            ],
            &[
                "public GA readiness gate",
                "release-hardening status",
                "operator approval packet",
                "long-cycle soak evidence",
            ],
            "separate product/release deltas from runtime claims",
            "release claims require governance packet, watchdog, and long soak evidence",
        ),
    ]
}

pub fn hepta_upstream_codex_diff_ledger_report() -> HeptaUpstreamCodexDiffLedgerReport {
    HeptaUpstreamCodexDiffLedgerReport::native_default()
}

pub fn hepta_upstream_codex_current_intake_report() -> HeptaUpstreamCodexCurrentIntakeReport {
    HeptaUpstreamCodexCurrentIntakeReport::native_default()
}

fn default_product_governance_selected_paths() -> Vec<String> {
    [
        "codex-rs/Cargo.lock",
        "codex-rs/Cargo.toml",
        "codex-rs/README.md",
        "codex-rs/app-server/README.md",
        "codex-rs/app-server/tests/suite/v2/plugin_install.rs",
        "codex-rs/app-server/tests/suite/v2/plugin_uninstall.rs",
        "codex-rs/core-plugins/src/remote/remote_installed_plugin_sync.rs",
        "codex-rs/core/README.md",
        "codex-rs/core/src/tools/handlers/list_available_plugins_to_install.rs",
        "codex-rs/core/src/tools/handlers/list_available_plugins_to_install_spec.rs",
        "codex-rs/core/src/tools/handlers/request_plugin_install.rs",
        "codex-rs/core/src/tools/handlers/request_plugin_install_spec.rs",
        "codex-rs/core/tests/suite/request_plugin_install.rs",
        "codex-rs/docs/protocol_v1.md",
        "codex-rs/exec-server/README.md",
        "codex-rs/install-context/Cargo.toml",
        "codex-rs/install-context/src/lib.rs",
        "codex-rs/linux-sandbox/README.md",
        "codex-rs/network-proxy/README.md",
        "codex-rs/skills/src/assets/samples/plugin-creator/references/installing-and-updating.md",
        "codex-rs/tools/README.md",
        "codex-rs/utils/pty/README.md",
    ]
    .iter()
    .map(|path| (*path).into())
    .collect()
}

pub fn hepta_upstream_codex_product_governance_absorption_report()
-> HeptaUpstreamCodexProductGovernanceAbsorptionReport {
    HeptaUpstreamCodexProductGovernanceAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_product_governance_translation_report()
-> HeptaUpstreamCodexProductGovernanceTranslationReport {
    HeptaUpstreamCodexProductGovernanceTranslationReport::native_default()
}

pub fn hepta_upstream_codex_release_governance_promotion_report()
-> HeptaUpstreamCodexReleaseGovernancePromotionReport {
    HeptaUpstreamCodexReleaseGovernancePromotionReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_absorption_report()
-> HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport {
    HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_replay_report()
-> HeptaUpstreamCodexLegacyCompatibilityReplayReport {
    HeptaUpstreamCodexLegacyCompatibilityReplayReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_promotion_report()
-> HeptaUpstreamCodexLegacyCompatibilityPromotionReport {
    HeptaUpstreamCodexLegacyCompatibilityPromotionReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_absorption_report()
-> HeptaUpstreamCodexProviderSecurityAbsorptionReport {
    HeptaUpstreamCodexProviderSecurityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_replay_report()
-> HeptaUpstreamCodexProviderSecurityReplayReport {
    HeptaUpstreamCodexProviderSecurityReplayReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_promotion_report()
-> HeptaUpstreamCodexProviderSecurityPromotionReport {
    HeptaUpstreamCodexProviderSecurityPromotionReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_absorption_report()
-> HeptaUpstreamCodexRuntimeAppServerAbsorptionReport {
    HeptaUpstreamCodexRuntimeAppServerAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_replay_report()
-> HeptaUpstreamCodexRuntimeAppServerReplayReport {
    HeptaUpstreamCodexRuntimeAppServerReplayReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_promotion_report()
-> HeptaUpstreamCodexRuntimeAppServerPromotionReport {
    HeptaUpstreamCodexRuntimeAppServerPromotionReport::native_default()
}

pub fn hepta_upstream_codex_absorption_replay_readiness_report()
-> HeptaUpstreamCodexAbsorptionReplayReadinessReport {
    HeptaUpstreamCodexAbsorptionReplayReadinessReport::native_default()
}

pub fn hepta_upstream_codex_promotion_readiness_report()
-> HeptaUpstreamCodexPromotionReadinessReport {
    HeptaUpstreamCodexPromotionReadinessReport::native_default()
}

pub fn hepta_upstream_codex_promotion_closure_report() -> HeptaUpstreamCodexPromotionClosureReport {
    HeptaUpstreamCodexPromotionClosureReport::native_default()
}

pub fn hepta_upstream_codex_active_wiring_precondition_report()
-> HeptaUpstreamCodexActiveWiringPreconditionReport {
    HeptaUpstreamCodexActiveWiringPreconditionReport::native_default()
}

pub fn hepta_upstream_codex_activation_request_packet_report()
-> HeptaUpstreamCodexActivationRequestPacketReport {
    HeptaUpstreamCodexActivationRequestPacketReport::native_default()
}

pub fn hepta_upstream_codex_activation_packet_dry_run_report()
-> HeptaUpstreamCodexActivationPacketDryRunReport {
    HeptaUpstreamCodexActivationPacketDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_ledger_report()
-> HeptaUpstreamCodexActivationEvidenceLedgerReport {
    HeptaUpstreamCodexActivationEvidenceLedgerReport::native_default()
}

pub fn hepta_upstream_codex_activation_readiness_closure_report()
-> HeptaUpstreamCodexActivationReadinessClosureReport {
    HeptaUpstreamCodexActivationReadinessClosureReport::native_default()
}

pub fn hepta_upstream_codex_activation_denied_sample_report()
-> HeptaUpstreamCodexActivationDeniedSampleReport {
    HeptaUpstreamCodexActivationDeniedSampleReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_freshness_policy_report()
-> HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport {
    HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_binding_record_manifest_report()
-> HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport {
    HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_record_denied_fixture_report()
-> HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport {
    HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport::native_default()
}

pub fn hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report()
-> HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport {
    HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport::native_default()
}

pub fn hepta_upstream_codex_activation_trusted_record_shape_validator_report()
-> HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport {
    HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_completeness_scoreboard_report()
-> HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport {
    HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report()
-> HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport {
    HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_recording_denial_matrix_report()
-> HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport {
    HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport {
    HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport {
    HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport {
    HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport::native_default()
}
