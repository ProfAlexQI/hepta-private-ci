use crate::manifest::PluginManifestToolDeclarations;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub const PLUGIN_MANIFEST_V1_VALIDATOR_SCHEMA_VERSION: &str = "plugin_manifest_v1_validator_v1";
pub const PLUGIN_MANIFEST_V1_MIGRATION_DRY_RUN_SCHEMA_VERSION: &str =
    "plugin_manifest_v1_migration_dry_run_v1";
pub const PLUGIN_MANIFEST_V1_SIGNATURE_TRUST_DRY_RUN_SCHEMA_VERSION: &str =
    "plugin_manifest_v1_signature_trust_dry_run_v1";
pub const PLUGIN_MANIFEST_V1_INSTALL_CACHE_FIXTURE_DRY_RUN_SCHEMA_VERSION: &str =
    "plugin_manifest_v1_install_cache_fixture_dry_run_v1";
pub const PLUGIN_MANIFEST_V1_SANDBOX_ENFORCEMENT_DRY_RUN_SCHEMA_VERSION: &str =
    "plugin_manifest_v1_sandbox_enforcement_dry_run_v1";

const REQUIRED_TIMEOUT_MS: u64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginManifestV1ValidationErrorKind {
    NoCandidateDeclarations,
    InvalidCandidateId,
    MissingToolSchema,
    IncompleteToolSchema,
    MissingPermission,
    InvalidPermission,
    MissingActivationEvent,
    InvalidActivationEvent,
    MissingToolPolicy,
    IncompleteToolPolicy,
    InvalidToolPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1ValidationError {
    pub candidate_tool_id: Option<String>,
    pub kind: PluginManifestV1ValidationErrorKind,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1ValidationReport {
    pub schema_version: &'static str,
    pub candidate_count: usize,
    pub valid_candidate_count: usize,
    pub invalid_candidate_count: usize,
    pub error_count: usize,
    pub errors: Vec<PluginManifestV1ValidationError>,
    pub manifest_v1_valid: bool,
    pub manifest_schema_write_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1MigrationDryRunPhase {
    pub phase_id: &'static str,
    pub sequence: u8,
    pub summary: &'static str,
    pub candidate_count: usize,
    pub ready: bool,
    pub migration_written: bool,
    pub manifest_rewrite_allowed: bool,
    pub manifest_schema_write_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1MigrationDryRunReport {
    pub schema_version: &'static str,
    pub target_manifest_schema_version: &'static str,
    pub status: &'static str,
    pub candidate_count: usize,
    pub phase_count: usize,
    pub ready_phase_count: usize,
    pub validation_error_count: usize,
    pub manifest_v1_valid: bool,
    pub migration_dry_run_ready: bool,
    pub migration_written_count: usize,
    pub phases: Vec<PluginManifestV1MigrationDryRunPhase>,
    pub validation_errors: Vec<PluginManifestV1ValidationError>,
    pub manifest_rewrite_allowed: bool,
    pub manifest_schema_write_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PluginManifestV1SignatureTrustDryRunInput {
    pub candidate_tool_id: String,
    pub signature_artifact_present: bool,
    pub trust_root_present: bool,
    pub operator_evidence_present: bool,
    pub operator_acceptance_present: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1SignatureTrustDryRunEntry {
    pub candidate_tool_id: String,
    pub signature_boundary_ready: bool,
    pub signature_artifact_present: bool,
    pub signature_verification_ready: bool,
    pub signature_verified: bool,
    pub signature_acceptance_allowed: bool,
    pub trust_boundary_ready: bool,
    pub trust_root_present: bool,
    pub trust_root_accepted: bool,
    pub trust_root_acceptance_allowed: bool,
    pub operator_evidence_required: bool,
    pub operator_evidence_present: bool,
    pub operator_acceptance_required: bool,
    pub operator_acceptance_present: bool,
    pub install_cache_boundary_ready: bool,
    pub install_cache_materialized: bool,
    pub install_cache_materialization_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1SignatureTrustDryRunReport {
    pub schema_version: &'static str,
    pub status: &'static str,
    pub candidate_count: usize,
    pub signature_boundary_ready_count: usize,
    pub trust_boundary_ready_count: usize,
    pub install_cache_boundary_ready_count: usize,
    pub signature_artifact_present_count: usize,
    pub trust_root_present_count: usize,
    pub signature_verification_ready_count: usize,
    pub signature_verified_count: usize,
    pub operator_evidence_required_count: usize,
    pub operator_evidence_present_count: usize,
    pub operator_acceptance_required_count: usize,
    pub operator_acceptance_present_count: usize,
    pub install_cache_materialized_count: usize,
    pub validation_error_count: usize,
    pub manifest_v1_valid: bool,
    pub signature_trust_dry_run_ready: bool,
    pub entries: Vec<PluginManifestV1SignatureTrustDryRunEntry>,
    pub validation_errors: Vec<PluginManifestV1ValidationError>,
    pub signature_acceptance_allowed: bool,
    pub trust_root_acceptance_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub install_cache_materialization_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1InstallCacheFixtureDryRunEntry {
    pub candidate_tool_id: String,
    pub cache_key: String,
    pub cache_path: String,
    pub artifact_digest: String,
    pub idempotency_key: String,
    pub rollback_plan_id: String,
    pub denial_receipt_id: String,
    pub noop_preflight_ready: bool,
    pub idempotency_key_stable: bool,
    pub rollback_uninstall_noop_ready: bool,
    pub dynamic_activation_boundary_ready: bool,
    pub signature_artifact_present: bool,
    pub trust_root_present: bool,
    pub operator_evidence_present: bool,
    pub operator_acceptance_present: bool,
    pub signature_verified: bool,
    pub trust_root_accepted: bool,
    pub install_cache_materialized: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub fixture_written: bool,
    pub denial_receipt_persisted: bool,
    pub runtime_event_log_written: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1InstallCacheFixtureDryRunReport {
    pub schema_version: &'static str,
    pub status: &'static str,
    pub candidate_count: usize,
    pub fixture_entry_count: usize,
    pub noop_preflight_ready_count: usize,
    pub stable_cache_key_count: usize,
    pub stable_idempotency_key_count: usize,
    pub rollback_uninstall_noop_ready_count: usize,
    pub dynamic_activation_boundary_ready_count: usize,
    pub denial_receipt_projected_count: usize,
    pub install_cache_materialized_count: usize,
    pub plugin_installed_count: usize,
    pub plugin_cache_mutated_count: usize,
    pub fixture_written_count: usize,
    pub denial_receipt_persisted_count: usize,
    pub runtime_event_log_written_count: usize,
    pub validation_error_count: usize,
    pub manifest_v1_valid: bool,
    pub signature_trust_dry_run_ready: bool,
    pub install_cache_fixture_dry_run_ready: bool,
    pub entries: Vec<PluginManifestV1InstallCacheFixtureDryRunEntry>,
    pub validation_errors: Vec<PluginManifestV1ValidationError>,
    pub install_cache_materialization_allowed: bool,
    pub plugin_install_allowed: bool,
    pub plugin_cache_mutation_allowed: bool,
    pub fixture_write_allowed: bool,
    pub denial_receipt_persistence_allowed: bool,
    pub runtime_event_log_write_allowed: bool,
    pub dynamic_activation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1SandboxEnforcementDryRunEntry {
    pub candidate_tool_id: String,
    pub network_none_enforced: bool,
    pub filesystem_read_only_enforced: bool,
    pub connector_scope_enforced: bool,
    pub manual_activation_only_enforced: bool,
    pub approval_ledger_policy_enforced: bool,
    pub credential_boundary_enforced: bool,
    pub transport_boundary_enforced: bool,
    pub runtime_persistence_boundary_enforced: bool,
    pub permission_granted: bool,
    pub network_access_allowed: bool,
    pub credential_read_allowed: bool,
    pub connector_start_allowed: bool,
    pub runtime_mutation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
    pub blockers: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginManifestV1SandboxEnforcementDryRunReport {
    pub schema_version: &'static str,
    pub status: &'static str,
    pub candidate_count: usize,
    pub sandbox_rule_count: usize,
    pub enforced_rule_count: usize,
    pub network_none_enforced_count: usize,
    pub filesystem_read_only_enforced_count: usize,
    pub connector_scope_enforced_count: usize,
    pub manual_activation_only_enforced_count: usize,
    pub approval_ledger_policy_enforced_count: usize,
    pub credential_boundary_enforced_count: usize,
    pub transport_boundary_enforced_count: usize,
    pub runtime_persistence_boundary_enforced_count: usize,
    pub validation_error_count: usize,
    pub manifest_v1_valid: bool,
    pub sandbox_enforcement_dry_run_ready: bool,
    pub entries: Vec<PluginManifestV1SandboxEnforcementDryRunEntry>,
    pub validation_errors: Vec<PluginManifestV1ValidationError>,
    pub permission_grant_allowed: bool,
    pub connector_start_allowed: bool,
    pub network_access_allowed: bool,
    pub credential_read_allowed: bool,
    pub runtime_mutation_allowed: bool,
    pub tool_registry_registration_allowed: bool,
    pub tool_invocation_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CandidateKind {
    Mcp,
    Connector,
}

pub fn validate_plugin_manifest_v1_tool_declarations(
    declarations: &PluginManifestToolDeclarations,
) -> PluginManifestV1ValidationReport {
    let candidate_ids = declarations.declared_candidate_ids();
    let schemas_by_id = declarations
        .tool_schemas
        .iter()
        .map(|declaration| (declaration.candidate_tool_id.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let permissions_by_id = declarations
        .permission_declarations
        .iter()
        .map(|declaration| (declaration.candidate_tool_id.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let activation_events_by_id = declarations
        .activation_event_declarations
        .iter()
        .map(|declaration| (declaration.candidate_tool_id.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();
    let policies_by_id = declarations
        .tool_policies
        .iter()
        .map(|declaration| (declaration.candidate_tool_id.as_str(), declaration))
        .collect::<BTreeMap<_, _>>();

    let mut errors = Vec::new();
    let mut valid_candidate_ids = BTreeSet::new();

    if candidate_ids.is_empty() {
        errors.push(error(
            None,
            PluginManifestV1ValidationErrorKind::NoCandidateDeclarations,
            "manifest v1 requires at least one declared tool candidate",
        ));
    }

    for candidate_id in &candidate_ids {
        let error_count_before_candidate = errors.len();
        let Some(candidate_kind) = candidate_kind(candidate_id) else {
            errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::InvalidCandidateId,
                "candidate id must use preview:mcp:<plugin>@<marketplace>:<tool> or preview:connector:<plugin>@<marketplace>:<tool>",
            ));
            continue;
        };

        match schemas_by_id.get(candidate_id.as_str()) {
            Some(schema)
                if schema.input_schema_declared
                    && schema.output_schema_declared
                    && schema.input_schema_is_object
                    && schema.output_schema_is_object => {}
            Some(_) => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::IncompleteToolSchema,
                "tool schema must declare object inputSchema and object outputSchema",
            )),
            None => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::MissingToolSchema,
                "candidate is missing a toolSchemas entry",
            )),
        }

        match permissions_by_id.get(candidate_id.as_str()) {
            Some(permission) if permission_valid_for_candidate(candidate_kind, permission) => {}
            Some(_) => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::InvalidPermission,
                "permission must keep network none and declare the correct local read-only scope",
            )),
            None => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::MissingPermission,
                "candidate is missing a permissions entry",
            )),
        }

        match activation_events_by_id.get(candidate_id.as_str()) {
            Some(activation_event)
                if activation_event.activation_event_declared
                    && activation_event.manual_activation_only => {}
            Some(_) => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::InvalidActivationEvent,
                "activationEvents must be manual-only for plugin manifest v1",
            )),
            None => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::MissingActivationEvent,
                "candidate is missing an activationEvents entry",
            )),
        }

        match policies_by_id.get(candidate_id.as_str()) {
            Some(policy) if !tool_policy_complete(policy) => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::IncompleteToolPolicy,
                "tool policy must declare approval, ledger, and timeout",
            )),
            Some(policy) if tool_policy_valid_for_candidate(candidate_kind, policy) => {}
            Some(_) => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::InvalidToolPolicy,
                "tool policy must use the expected approval kind, required ledger, and 30000 ms timeout",
            )),
            None => errors.push(error(
                Some(candidate_id),
                PluginManifestV1ValidationErrorKind::MissingToolPolicy,
                "candidate is missing a toolPolicies entry",
            )),
        }

        if errors.len() == error_count_before_candidate {
            valid_candidate_ids.insert(candidate_id.clone());
        }
    }

    let candidate_count = candidate_ids.len();
    let valid_candidate_count = valid_candidate_ids.len();
    let error_count = errors.len();

    PluginManifestV1ValidationReport {
        schema_version: PLUGIN_MANIFEST_V1_VALIDATOR_SCHEMA_VERSION,
        candidate_count,
        valid_candidate_count,
        invalid_candidate_count: candidate_count.saturating_sub(valid_candidate_count),
        error_count,
        manifest_v1_valid: error_count == 0,
        errors,
        manifest_schema_write_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
    }
}

pub fn plan_plugin_manifest_v1_migration_dry_run(
    declarations: &PluginManifestToolDeclarations,
) -> PluginManifestV1MigrationDryRunReport {
    let validation_report = validate_plugin_manifest_v1_tool_declarations(declarations);
    let candidate_count = validation_report.candidate_count;
    let manifest_v1_valid = validation_report.manifest_v1_valid;
    let phases = vec![
        migration_phase(
            "schema_version_header_v1",
            1,
            "declare the manifest/permission/activation/tool-policy contract as v1 before any manifest rewrite",
            candidate_count,
            manifest_v1_valid,
        ),
        migration_phase(
            "candidate_id_canonicalization",
            2,
            "keep candidate ids stable across manifest, permissions, activation events, and tool policies",
            candidate_count,
            manifest_v1_valid,
        ),
        migration_phase(
            "permission_normalization",
            3,
            "normalize network none plus filesystem read-only or local connector scopes before any grant",
            candidate_count,
            manifest_v1_valid,
        ),
        migration_phase(
            "activation_manual_only",
            4,
            "keep first activation path manual-only until operator approval and trust acceptance exist",
            candidate_count,
            manifest_v1_valid,
        ),
        migration_phase(
            "tool_policy_approval_ledger_timeout",
            5,
            "require approval, ledger, and timeout policy before ToolRegistry registration",
            candidate_count,
            manifest_v1_valid,
        ),
    ];
    let ready_phase_count = phases.iter().filter(|phase| phase.ready).count();
    let migration_written_count = phases
        .iter()
        .filter(|phase| phase.migration_written)
        .count();
    let validation_error_count = validation_report.error_count;
    let migration_dry_run_ready =
        manifest_v1_valid && ready_phase_count == phases.len() && migration_written_count == 0;

    PluginManifestV1MigrationDryRunReport {
        schema_version: PLUGIN_MANIFEST_V1_MIGRATION_DRY_RUN_SCHEMA_VERSION,
        target_manifest_schema_version: "v1",
        status: if migration_dry_run_ready {
            "ready"
        } else {
            "blocked"
        },
        candidate_count,
        phase_count: phases.len(),
        ready_phase_count,
        validation_error_count,
        manifest_v1_valid,
        migration_dry_run_ready,
        migration_written_count,
        phases,
        validation_errors: validation_report.errors,
        manifest_rewrite_allowed: false,
        manifest_schema_write_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
    }
}

pub fn plan_plugin_manifest_v1_signature_trust_dry_run(
    declarations: &PluginManifestToolDeclarations,
    inputs: &[PluginManifestV1SignatureTrustDryRunInput],
) -> PluginManifestV1SignatureTrustDryRunReport {
    let validation_report = validate_plugin_manifest_v1_tool_declarations(declarations);
    let inputs_by_id = inputs
        .iter()
        .map(|input| (input.candidate_tool_id.as_str(), input))
        .collect::<BTreeMap<_, _>>();

    let entries = if validation_report.manifest_v1_valid {
        declarations
            .declared_candidate_ids()
            .into_iter()
            .map(|candidate_tool_id| {
                let input = inputs_by_id.get(candidate_tool_id.as_str()).copied();
                signature_trust_entry(&candidate_tool_id, input)
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let candidate_count = entries.len();
    let signature_boundary_ready_count =
        count_entries(&entries, |entry| entry.signature_boundary_ready);
    let trust_boundary_ready_count = count_entries(&entries, |entry| entry.trust_boundary_ready);
    let install_cache_boundary_ready_count =
        count_entries(&entries, |entry| entry.install_cache_boundary_ready);
    let signature_artifact_present_count =
        count_entries(&entries, |entry| entry.signature_artifact_present);
    let trust_root_present_count = count_entries(&entries, |entry| entry.trust_root_present);
    let signature_verification_ready_count =
        count_entries(&entries, |entry| entry.signature_verification_ready);
    let signature_verified_count = count_entries(&entries, |entry| entry.signature_verified);
    let operator_evidence_required_count =
        count_entries(&entries, |entry| entry.operator_evidence_required);
    let operator_evidence_present_count =
        count_entries(&entries, |entry| entry.operator_evidence_present);
    let operator_acceptance_required_count =
        count_entries(&entries, |entry| entry.operator_acceptance_required);
    let operator_acceptance_present_count =
        count_entries(&entries, |entry| entry.operator_acceptance_present);
    let install_cache_materialized_count =
        count_entries(&entries, |entry| entry.install_cache_materialized);
    let validation_error_count = validation_report.error_count;
    let all_boundaries_ready = candidate_count > 0
        && signature_boundary_ready_count == candidate_count
        && trust_boundary_ready_count == candidate_count
        && install_cache_boundary_ready_count == candidate_count;
    let all_side_effects_closed = entries.iter().all(|entry| {
        !entry.signature_verified
            && !entry.signature_acceptance_allowed
            && !entry.trust_root_accepted
            && !entry.trust_root_acceptance_allowed
            && !entry.install_cache_materialized
            && !entry.install_cache_materialization_allowed
            && !entry.plugin_install_allowed
            && !entry.plugin_cache_mutation_allowed
            && !entry.dynamic_activation_allowed
            && !entry.tool_registry_registration_allowed
            && !entry.tool_invocation_allowed
            && !entry.live_execution_allowed
    });
    let signature_trust_dry_run_ready =
        validation_report.manifest_v1_valid && all_boundaries_ready && all_side_effects_closed;

    PluginManifestV1SignatureTrustDryRunReport {
        schema_version: PLUGIN_MANIFEST_V1_SIGNATURE_TRUST_DRY_RUN_SCHEMA_VERSION,
        status: if signature_trust_dry_run_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        candidate_count,
        signature_boundary_ready_count,
        trust_boundary_ready_count,
        install_cache_boundary_ready_count,
        signature_artifact_present_count,
        trust_root_present_count,
        signature_verification_ready_count,
        signature_verified_count,
        operator_evidence_required_count,
        operator_evidence_present_count,
        operator_acceptance_required_count,
        operator_acceptance_present_count,
        install_cache_materialized_count,
        validation_error_count,
        manifest_v1_valid: validation_report.manifest_v1_valid,
        signature_trust_dry_run_ready,
        entries,
        validation_errors: validation_report.errors,
        signature_acceptance_allowed: false,
        trust_root_acceptance_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        install_cache_materialization_allowed: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
    }
}

pub fn plan_plugin_manifest_v1_install_cache_fixture_dry_run(
    declarations: &PluginManifestToolDeclarations,
    signature_trust_inputs: &[PluginManifestV1SignatureTrustDryRunInput],
) -> PluginManifestV1InstallCacheFixtureDryRunReport {
    let signature_trust_report =
        plan_plugin_manifest_v1_signature_trust_dry_run(declarations, signature_trust_inputs);
    let entries = if signature_trust_report.manifest_v1_valid
        && signature_trust_report.signature_trust_dry_run_ready
    {
        signature_trust_report
            .entries
            .iter()
            .map(install_cache_fixture_entry)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let candidate_count = signature_trust_report.candidate_count;
    let fixture_entry_count = entries.len();
    let noop_preflight_ready_count =
        count_install_cache_entries(&entries, |entry| entry.noop_preflight_ready);
    let stable_cache_key_count =
        count_install_cache_entries(&entries, |entry| !entry.cache_key.is_empty());
    let stable_idempotency_key_count =
        count_install_cache_entries(&entries, |entry| entry.idempotency_key_stable);
    let rollback_uninstall_noop_ready_count =
        count_install_cache_entries(&entries, |entry| entry.rollback_uninstall_noop_ready);
    let dynamic_activation_boundary_ready_count =
        count_install_cache_entries(&entries, |entry| entry.dynamic_activation_boundary_ready);
    let denial_receipt_projected_count =
        count_install_cache_entries(&entries, |entry| !entry.denial_receipt_id.is_empty());
    let install_cache_materialized_count =
        count_install_cache_entries(&entries, |entry| entry.install_cache_materialized);
    let plugin_installed_count =
        count_install_cache_entries(&entries, |entry| entry.plugin_installed);
    let plugin_cache_mutated_count =
        count_install_cache_entries(&entries, |entry| entry.plugin_cache_mutated);
    let fixture_written_count =
        count_install_cache_entries(&entries, |entry| entry.fixture_written);
    let denial_receipt_persisted_count =
        count_install_cache_entries(&entries, |entry| entry.denial_receipt_persisted);
    let runtime_event_log_written_count =
        count_install_cache_entries(&entries, |entry| entry.runtime_event_log_written);
    let fixture_paths_ready = fixture_entry_count > 0
        && noop_preflight_ready_count == fixture_entry_count
        && stable_cache_key_count == fixture_entry_count
        && stable_idempotency_key_count == fixture_entry_count
        && rollback_uninstall_noop_ready_count == fixture_entry_count
        && dynamic_activation_boundary_ready_count == fixture_entry_count
        && denial_receipt_projected_count == fixture_entry_count;
    let all_side_effects_closed = entries.iter().all(|entry| {
        !entry.signature_verified
            && !entry.trust_root_accepted
            && !entry.install_cache_materialized
            && !entry.plugin_installed
            && !entry.plugin_cache_mutated
            && !entry.fixture_written
            && !entry.denial_receipt_persisted
            && !entry.runtime_event_log_written
            && !entry.dynamic_activation_allowed
            && !entry.tool_registry_registration_allowed
            && !entry.tool_invocation_allowed
            && !entry.live_execution_allowed
    });
    let install_cache_fixture_dry_run_ready = signature_trust_report.manifest_v1_valid
        && signature_trust_report.signature_trust_dry_run_ready
        && fixture_paths_ready
        && install_cache_materialized_count == 0
        && plugin_installed_count == 0
        && plugin_cache_mutated_count == 0
        && fixture_written_count == 0
        && denial_receipt_persisted_count == 0
        && runtime_event_log_written_count == 0
        && all_side_effects_closed;

    PluginManifestV1InstallCacheFixtureDryRunReport {
        schema_version: PLUGIN_MANIFEST_V1_INSTALL_CACHE_FIXTURE_DRY_RUN_SCHEMA_VERSION,
        status: if install_cache_fixture_dry_run_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        candidate_count,
        fixture_entry_count,
        noop_preflight_ready_count,
        stable_cache_key_count,
        stable_idempotency_key_count,
        rollback_uninstall_noop_ready_count,
        dynamic_activation_boundary_ready_count,
        denial_receipt_projected_count,
        install_cache_materialized_count,
        plugin_installed_count,
        plugin_cache_mutated_count,
        fixture_written_count,
        denial_receipt_persisted_count,
        runtime_event_log_written_count,
        validation_error_count: signature_trust_report.validation_error_count,
        manifest_v1_valid: signature_trust_report.manifest_v1_valid,
        signature_trust_dry_run_ready: signature_trust_report.signature_trust_dry_run_ready,
        install_cache_fixture_dry_run_ready,
        entries,
        validation_errors: signature_trust_report.validation_errors,
        install_cache_materialization_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        fixture_write_allowed: false,
        denial_receipt_persistence_allowed: false,
        runtime_event_log_write_allowed: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
    }
}

pub fn plan_plugin_manifest_v1_sandbox_enforcement_dry_run(
    declarations: &PluginManifestToolDeclarations,
) -> PluginManifestV1SandboxEnforcementDryRunReport {
    let validation_report = validate_plugin_manifest_v1_tool_declarations(declarations);
    let entries = if validation_report.manifest_v1_valid {
        declarations
            .declared_candidate_ids()
            .into_iter()
            .filter_map(|candidate_tool_id| {
                candidate_kind(&candidate_tool_id)
                    .map(|kind| sandbox_enforcement_entry(&candidate_tool_id, kind))
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let candidate_count = entries.len();
    let sandbox_rule_count = 8;
    let network_none_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.network_none_enforced);
    let filesystem_read_only_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.filesystem_read_only_enforced);
    let connector_scope_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.connector_scope_enforced);
    let manual_activation_only_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.manual_activation_only_enforced);
    let approval_ledger_policy_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.approval_ledger_policy_enforced);
    let credential_boundary_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.credential_boundary_enforced);
    let transport_boundary_enforced_count =
        count_sandbox_entries(&entries, |entry| entry.transport_boundary_enforced);
    let runtime_persistence_boundary_enforced_count = count_sandbox_entries(&entries, |entry| {
        entry.runtime_persistence_boundary_enforced
    });
    let enforced_rule_count =
        usize::from(candidate_count > 0 && network_none_enforced_count == candidate_count)
            + usize::from(
                candidate_count > 0
                    && filesystem_read_only_enforced_count + connector_scope_enforced_count
                        == candidate_count,
            )
            + usize::from(
                candidate_count > 0 && manual_activation_only_enforced_count == candidate_count,
            )
            + usize::from(
                candidate_count > 0 && approval_ledger_policy_enforced_count == candidate_count,
            )
            + usize::from(
                candidate_count > 0 && credential_boundary_enforced_count == candidate_count,
            )
            + usize::from(
                candidate_count > 0 && transport_boundary_enforced_count == candidate_count,
            )
            + usize::from(
                candidate_count > 0
                    && runtime_persistence_boundary_enforced_count == candidate_count,
            )
            + usize::from(
                candidate_count > 0
                    && entries.iter().all(|entry| {
                        !entry.permission_granted
                            && !entry.network_access_allowed
                            && !entry.credential_read_allowed
                            && !entry.connector_start_allowed
                            && !entry.runtime_mutation_allowed
                            && !entry.tool_registry_registration_allowed
                            && !entry.tool_invocation_allowed
                            && !entry.live_execution_allowed
                    }),
            );
    let all_side_effects_closed = entries.iter().all(|entry| {
        !entry.permission_granted
            && !entry.network_access_allowed
            && !entry.credential_read_allowed
            && !entry.connector_start_allowed
            && !entry.runtime_mutation_allowed
            && !entry.tool_registry_registration_allowed
            && !entry.tool_invocation_allowed
            && !entry.live_execution_allowed
    });
    let sandbox_enforcement_dry_run_ready = validation_report.manifest_v1_valid
        && candidate_count > 0
        && enforced_rule_count == sandbox_rule_count
        && all_side_effects_closed;

    PluginManifestV1SandboxEnforcementDryRunReport {
        schema_version: PLUGIN_MANIFEST_V1_SANDBOX_ENFORCEMENT_DRY_RUN_SCHEMA_VERSION,
        status: if sandbox_enforcement_dry_run_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        candidate_count,
        sandbox_rule_count,
        enforced_rule_count,
        network_none_enforced_count,
        filesystem_read_only_enforced_count,
        connector_scope_enforced_count,
        manual_activation_only_enforced_count,
        approval_ledger_policy_enforced_count,
        credential_boundary_enforced_count,
        transport_boundary_enforced_count,
        runtime_persistence_boundary_enforced_count,
        validation_error_count: validation_report.error_count,
        manifest_v1_valid: validation_report.manifest_v1_valid,
        sandbox_enforcement_dry_run_ready,
        entries,
        validation_errors: validation_report.errors,
        permission_grant_allowed: false,
        connector_start_allowed: false,
        network_access_allowed: false,
        credential_read_allowed: false,
        runtime_mutation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
    }
}

fn candidate_kind(candidate_id: &str) -> Option<CandidateKind> {
    parse_candidate_kind(candidate_id, "preview:mcp:", CandidateKind::Mcp).or_else(|| {
        parse_candidate_kind(candidate_id, "preview:connector:", CandidateKind::Connector)
    })
}

fn parse_candidate_kind(
    candidate_id: &str,
    prefix: &str,
    candidate_kind: CandidateKind,
) -> Option<CandidateKind> {
    let rest = candidate_id.strip_prefix(prefix)?;
    let (plugin_id, tool_name) = rest.rsplit_once(':')?;
    let (plugin_name, marketplace_name) = plugin_id.split_once('@')?;
    if plugin_name.is_empty()
        || marketplace_name.is_empty()
        || tool_name.is_empty()
        || tool_name.contains(':')
    {
        return None;
    }
    Some(candidate_kind)
}

fn permission_valid_for_candidate(
    candidate_kind: CandidateKind,
    permission: &crate::manifest::PluginManifestPermissionDeclaration,
) -> bool {
    match candidate_kind {
        CandidateKind::Mcp => {
            permission.network_declared
                && permission.network_none
                && permission.filesystem_read_only
                && !permission.connector_declared
        }
        CandidateKind::Connector => {
            permission.network_declared
                && permission.network_none
                && permission.connector_declared
                && !permission.filesystem_read_only
        }
    }
}

fn tool_policy_complete(policy: &crate::manifest::PluginManifestToolPolicyDeclaration) -> bool {
    policy.approval_policy_declared
        && policy.ledger_policy_declared
        && policy.timeout_policy_declared
}

fn tool_policy_valid_for_candidate(
    candidate_kind: CandidateKind,
    policy: &crate::manifest::PluginManifestToolPolicyDeclaration,
) -> bool {
    let expected_approval_kind = match candidate_kind {
        CandidateKind::Mcp => "onUse",
        CandidateKind::Connector => "install",
    };
    policy.approval_kind.as_deref() == Some(expected_approval_kind)
        && policy.ledger_required == Some(true)
        && policy.timeout_ms == Some(REQUIRED_TIMEOUT_MS)
}

fn signature_trust_entry(
    candidate_tool_id: &str,
    input: Option<&PluginManifestV1SignatureTrustDryRunInput>,
) -> PluginManifestV1SignatureTrustDryRunEntry {
    let signature_artifact_present = input.is_some_and(|input| input.signature_artifact_present);
    let trust_root_present = input.is_some_and(|input| input.trust_root_present);
    let operator_evidence_present = input.is_some_and(|input| input.operator_evidence_present);
    let operator_acceptance_present = input.is_some_and(|input| input.operator_acceptance_present);
    let signature_verification_ready = signature_artifact_present && trust_root_present;
    let mut blockers = Vec::new();
    if !signature_artifact_present {
        blockers.push("signature_artifact_missing");
    }
    if !trust_root_present {
        blockers.push("trust_root_missing");
    }
    if signature_verification_ready {
        blockers.push("signature_verification_not_executed");
    }
    if !operator_evidence_present {
        blockers.push("operator_evidence_missing");
    }
    if !operator_acceptance_present {
        blockers.push("operator_acceptance_missing");
    }
    blockers.push("signature_acceptance_disabled");
    blockers.push("trust_root_acceptance_disabled");
    blockers.push("install_cache_materialization_disabled");
    blockers.push("plugin_install_disabled");
    blockers.push("dynamic_activation_disabled");
    blockers.push("tool_registry_registration_disabled");
    blockers.push("tool_invocation_disabled");
    blockers.push("live_execution_disabled");

    PluginManifestV1SignatureTrustDryRunEntry {
        candidate_tool_id: candidate_tool_id.to_string(),
        signature_boundary_ready: true,
        signature_artifact_present,
        signature_verification_ready,
        signature_verified: false,
        signature_acceptance_allowed: false,
        trust_boundary_ready: true,
        trust_root_present,
        trust_root_accepted: false,
        trust_root_acceptance_allowed: false,
        operator_evidence_required: true,
        operator_evidence_present,
        operator_acceptance_required: true,
        operator_acceptance_present,
        install_cache_boundary_ready: true,
        install_cache_materialized: false,
        install_cache_materialization_allowed: false,
        plugin_install_allowed: false,
        plugin_cache_mutation_allowed: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
        blockers,
    }
}

fn count_entries(
    entries: &[PluginManifestV1SignatureTrustDryRunEntry],
    predicate: impl Fn(&PluginManifestV1SignatureTrustDryRunEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn count_install_cache_entries(
    entries: &[PluginManifestV1InstallCacheFixtureDryRunEntry],
    predicate: impl Fn(&PluginManifestV1InstallCacheFixtureDryRunEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn count_sandbox_entries(
    entries: &[PluginManifestV1SandboxEnforcementDryRunEntry],
    predicate: impl Fn(&PluginManifestV1SandboxEnforcementDryRunEntry) -> bool,
) -> usize {
    entries.iter().filter(|entry| predicate(entry)).count()
}

fn migration_phase(
    phase_id: &'static str,
    sequence: u8,
    summary: &'static str,
    candidate_count: usize,
    ready: bool,
) -> PluginManifestV1MigrationDryRunPhase {
    PluginManifestV1MigrationDryRunPhase {
        phase_id,
        sequence,
        summary,
        candidate_count,
        ready,
        migration_written: false,
        manifest_rewrite_allowed: false,
        manifest_schema_write_allowed: false,
        plugin_cache_mutation_allowed: false,
    }
}

fn sandbox_enforcement_entry(
    candidate_tool_id: &str,
    candidate_kind: CandidateKind,
) -> PluginManifestV1SandboxEnforcementDryRunEntry {
    let filesystem_read_only_enforced = candidate_kind == CandidateKind::Mcp;
    let connector_scope_enforced = candidate_kind == CandidateKind::Connector;
    PluginManifestV1SandboxEnforcementDryRunEntry {
        candidate_tool_id: candidate_tool_id.to_string(),
        network_none_enforced: true,
        filesystem_read_only_enforced,
        connector_scope_enforced,
        manual_activation_only_enforced: true,
        approval_ledger_policy_enforced: true,
        credential_boundary_enforced: true,
        transport_boundary_enforced: true,
        runtime_persistence_boundary_enforced: true,
        permission_granted: false,
        network_access_allowed: false,
        credential_read_allowed: false,
        connector_start_allowed: false,
        runtime_mutation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
        blockers: vec![
            "permission_grant_disabled",
            "network_access_disabled",
            "credential_read_disabled",
            "connector_start_disabled",
            "runtime_mutation_disabled",
            "tool_registry_registration_disabled",
            "tool_invocation_disabled",
            "live_execution_disabled",
        ],
    }
}

fn install_cache_fixture_entry(
    signature_entry: &PluginManifestV1SignatureTrustDryRunEntry,
) -> PluginManifestV1InstallCacheFixtureDryRunEntry {
    let stable_fragment = stable_candidate_fragment(&signature_entry.candidate_tool_id);
    let mut blockers = Vec::new();
    if !signature_entry.signature_artifact_present {
        blockers.push("signature_artifact_missing");
    }
    if !signature_entry.trust_root_present {
        blockers.push("trust_root_missing");
    }
    if !signature_entry.signature_verified {
        blockers.push("signature_not_verified");
    }
    if !signature_entry.trust_root_accepted {
        blockers.push("trust_root_not_accepted");
    }
    if !signature_entry.operator_evidence_present {
        blockers.push("operator_evidence_missing");
    }
    if !signature_entry.operator_acceptance_present {
        blockers.push("operator_acceptance_missing");
    }
    blockers.push("install_cache_materialization_disabled");
    blockers.push("plugin_install_disabled");
    blockers.push("plugin_cache_mutation_disabled");
    blockers.push("fixture_write_disabled");
    blockers.push("denial_receipt_persistence_disabled");
    blockers.push("runtime_event_log_write_disabled");
    blockers.push("dynamic_activation_disabled");
    blockers.push("tool_registry_registration_disabled");
    blockers.push("tool_invocation_disabled");
    blockers.push("live_execution_disabled");

    PluginManifestV1InstallCacheFixtureDryRunEntry {
        candidate_tool_id: signature_entry.candidate_tool_id.clone(),
        cache_key: format!("plugin-v1-cache-key:{stable_fragment}"),
        cache_path: format!(".hepta/plugin-install-cache/dry-run/{stable_fragment}"),
        artifact_digest: format!("dry-run-artifact-digest:{stable_fragment}"),
        idempotency_key: format!("plugin-v1-install-idempotency:{stable_fragment}"),
        rollback_plan_id: format!("plugin-v1-rollback-uninstall-noop:{stable_fragment}"),
        denial_receipt_id: format!("plugin-v1-install-denial:{stable_fragment}"),
        noop_preflight_ready: true,
        idempotency_key_stable: true,
        rollback_uninstall_noop_ready: true,
        dynamic_activation_boundary_ready: true,
        signature_artifact_present: signature_entry.signature_artifact_present,
        trust_root_present: signature_entry.trust_root_present,
        operator_evidence_present: signature_entry.operator_evidence_present,
        operator_acceptance_present: signature_entry.operator_acceptance_present,
        signature_verified: false,
        trust_root_accepted: false,
        install_cache_materialized: false,
        plugin_installed: false,
        plugin_cache_mutated: false,
        fixture_written: false,
        denial_receipt_persisted: false,
        runtime_event_log_written: false,
        dynamic_activation_allowed: false,
        tool_registry_registration_allowed: false,
        tool_invocation_allowed: false,
        live_execution_allowed: false,
        blockers,
    }
}

fn stable_candidate_fragment(candidate_tool_id: &str) -> String {
    candidate_tool_id
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn error(
    candidate_tool_id: Option<&str>,
    kind: PluginManifestV1ValidationErrorKind,
    message: &str,
) -> PluginManifestV1ValidationError {
    PluginManifestV1ValidationError {
        candidate_tool_id: candidate_tool_id.map(str::to_string),
        kind,
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest_tool_declarations::resolve_tool_declarations;
    use serde_json::Value as JsonValue;
    use serde_json::json;

    #[test]
    fn plugin_manifest_v1_validator_accepts_hepta_fixture_contract() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = validate_plugin_manifest_v1_tool_declarations(&declarations);

        assert_eq!(
            report.schema_version,
            PLUGIN_MANIFEST_V1_VALIDATOR_SCHEMA_VERSION
        );
        assert!(report.manifest_v1_valid);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.valid_candidate_count, 2);
        assert_eq!(report.invalid_candidate_count, 0);
        assert_eq!(report.error_count, 0);
        assert!(report.errors.is_empty());
        assert!(!report.manifest_schema_write_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn plugin_manifest_v1_validator_rejects_missing_tool_schema() {
        let manifest = json!({
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "none"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "manual" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "onUse" },
                    "ledger": { "required": true },
                    "timeoutMs": 30000
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = validate_plugin_manifest_v1_tool_declarations(&declarations);

        assert!(!report.manifest_v1_valid);
        assert_eq!(report.candidate_count, 1);
        assert_eq!(report.valid_candidate_count, 0);
        assert!(report.errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::MissingToolSchema
                && error.candidate_tool_id.as_deref()
                    == Some("preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp")
        }));
        assert!(!report.plugin_install_allowed);
        assert!(!report.tool_registry_registration_allowed);
    }

    #[test]
    fn plugin_manifest_v1_validator_rejects_permission_activation_and_policy_drift() {
        let manifest = json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "local"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "install" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": false },
                    "timeoutMs": 10
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = validate_plugin_manifest_v1_tool_declarations(&declarations);

        assert!(!report.manifest_v1_valid);
        assert_eq!(report.candidate_count, 1);
        assert_eq!(report.valid_candidate_count, 0);
        assert_eq!(report.invalid_candidate_count, 1);
        assert!(has_error_kind(
            &report,
            PluginManifestV1ValidationErrorKind::InvalidPermission
        ));
        assert!(has_error_kind(
            &report,
            PluginManifestV1ValidationErrorKind::InvalidActivationEvent
        ));
        assert!(has_error_kind(
            &report,
            PluginManifestV1ValidationErrorKind::InvalidToolPolicy
        ));
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn plugin_manifest_v1_validator_rejects_missing_permission() {
        let manifest = json!({
            "toolSchemas": {
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "activationEvents": {
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": [
                    { "type": "manual" }
                ]
            },
            "toolPolicies": {
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": true },
                    "timeoutMs": 30000
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = validate_plugin_manifest_v1_tool_declarations(&declarations);

        assert!(!report.manifest_v1_valid);
        assert!(has_error_kind(
            &report,
            PluginManifestV1ValidationErrorKind::MissingPermission
        ));
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.tool_invocation_allowed);
    }

    #[test]
    fn plugin_manifest_v1_migration_dry_run_plans_fixture_without_writes() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_migration_dry_run(&declarations);

        assert_eq!(
            report.schema_version,
            PLUGIN_MANIFEST_V1_MIGRATION_DRY_RUN_SCHEMA_VERSION
        );
        assert_eq!(report.target_manifest_schema_version, "v1");
        assert_eq!(report.status, "ready");
        assert!(report.manifest_v1_valid);
        assert!(report.migration_dry_run_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.phase_count, 5);
        assert_eq!(report.ready_phase_count, 5);
        assert_eq!(report.validation_error_count, 0);
        assert_eq!(report.migration_written_count, 0);
        assert_eq!(
            report
                .phases
                .iter()
                .map(|phase| phase.phase_id)
                .collect::<Vec<_>>(),
            vec![
                "schema_version_header_v1",
                "candidate_id_canonicalization",
                "permission_normalization",
                "activation_manual_only",
                "tool_policy_approval_ledger_timeout",
            ]
        );
        assert!(report.phases.iter().all(|phase| {
            phase.ready
                && !phase.migration_written
                && !phase.manifest_rewrite_allowed
                && !phase.manifest_schema_write_allowed
                && !phase.plugin_cache_mutation_allowed
        }));
        assert!(!report.manifest_rewrite_allowed);
        assert!(!report.manifest_schema_write_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn plugin_manifest_v1_migration_dry_run_blocks_permission_activation_and_policy_drift() {
        let manifest = json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "local"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "install" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": false },
                    "timeoutMs": 10
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_migration_dry_run(&declarations);

        assert_eq!(report.status, "blocked");
        assert!(!report.manifest_v1_valid);
        assert!(!report.migration_dry_run_ready);
        assert_eq!(report.phase_count, 5);
        assert_eq!(report.ready_phase_count, 0);
        assert_eq!(report.migration_written_count, 0);
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidPermission
            })
        );
        assert!(report.validation_errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::InvalidActivationEvent
        }));
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidToolPolicy
            })
        );
        assert!(report.phases.iter().all(|phase| {
            !phase.ready
                && !phase.migration_written
                && !phase.manifest_rewrite_allowed
                && !phase.manifest_schema_write_allowed
                && !phase.plugin_cache_mutation_allowed
        }));
        assert!(!report.manifest_rewrite_allowed);
        assert!(!report.manifest_schema_write_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn plugin_manifest_v1_migration_dry_run_blocks_empty_manifest_without_writes() {
        let manifest = json!({});
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_migration_dry_run(&declarations);

        assert_eq!(report.status, "blocked");
        assert_eq!(report.candidate_count, 0);
        assert_eq!(report.phase_count, 5);
        assert_eq!(report.ready_phase_count, 0);
        assert_eq!(report.migration_written_count, 0);
        assert!(report.validation_errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::NoCandidateDeclarations
        }));
        assert!(!report.manifest_rewrite_allowed);
        assert!(!report.manifest_schema_write_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.live_execution_allowed);
    }

    #[test]
    fn plugin_manifest_v1_signature_trust_dry_run_projects_missing_materials_without_writes() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_signature_trust_dry_run(&declarations, &[]);

        assert_eq!(
            report.schema_version,
            PLUGIN_MANIFEST_V1_SIGNATURE_TRUST_DRY_RUN_SCHEMA_VERSION
        );
        assert_eq!(report.status, "ready_blocked");
        assert!(report.manifest_v1_valid);
        assert!(report.signature_trust_dry_run_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.signature_boundary_ready_count, 2);
        assert_eq!(report.trust_boundary_ready_count, 2);
        assert_eq!(report.install_cache_boundary_ready_count, 2);
        assert_eq!(report.signature_artifact_present_count, 0);
        assert_eq!(report.trust_root_present_count, 0);
        assert_eq!(report.signature_verification_ready_count, 0);
        assert_eq!(report.signature_verified_count, 0);
        assert_eq!(report.operator_evidence_required_count, 2);
        assert_eq!(report.operator_evidence_present_count, 0);
        assert_eq!(report.operator_acceptance_required_count, 2);
        assert_eq!(report.operator_acceptance_present_count, 0);
        assert_eq!(report.install_cache_materialized_count, 0);
        assert_eq!(report.validation_error_count, 0);
        assert!(report.entries.iter().all(|entry| {
            entry.signature_boundary_ready
                && entry.trust_boundary_ready
                && entry.install_cache_boundary_ready
                && entry.blockers.contains(&"signature_artifact_missing")
                && entry.blockers.contains(&"trust_root_missing")
                && entry.blockers.contains(&"operator_evidence_missing")
                && entry.blockers.contains(&"operator_acceptance_missing")
                && !entry.signature_verified
                && !entry.signature_acceptance_allowed
                && !entry.trust_root_accepted
                && !entry.trust_root_acceptance_allowed
                && !entry.install_cache_materialized
                && !entry.install_cache_materialization_allowed
                && !entry.plugin_install_allowed
                && !entry.plugin_cache_mutation_allowed
                && !entry.dynamic_activation_allowed
                && !entry.tool_registry_registration_allowed
                && !entry.tool_invocation_allowed
                && !entry.live_execution_allowed
        }));
        assert_signature_trust_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_signature_trust_dry_run_keeps_install_closed_with_materials_present() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);
        let inputs = hepta_signature_trust_inputs(
            /*signature_artifact_present*/ true, /*trust_root_present*/ true,
            /*operator_evidence_present*/ true, /*operator_acceptance_present*/ true,
        );

        let report = plan_plugin_manifest_v1_signature_trust_dry_run(&declarations, &inputs);

        assert_eq!(report.status, "ready_blocked");
        assert!(report.signature_trust_dry_run_ready);
        assert_eq!(report.signature_artifact_present_count, 2);
        assert_eq!(report.trust_root_present_count, 2);
        assert_eq!(report.signature_verification_ready_count, 2);
        assert_eq!(report.signature_verified_count, 0);
        assert_eq!(report.operator_evidence_present_count, 2);
        assert_eq!(report.operator_acceptance_present_count, 2);
        assert_eq!(report.install_cache_materialized_count, 0);
        assert!(report.entries.iter().all(|entry| {
            entry.signature_artifact_present
                && entry.trust_root_present
                && entry.signature_verification_ready
                && entry.operator_evidence_present
                && entry.operator_acceptance_present
                && entry
                    .blockers
                    .contains(&"signature_verification_not_executed")
                && entry
                    .blockers
                    .contains(&"install_cache_materialization_disabled")
                && !entry.signature_verified
                && !entry.trust_root_accepted
                && !entry.plugin_install_allowed
                && !entry.tool_registry_registration_allowed
                && !entry.tool_invocation_allowed
        }));
        assert_signature_trust_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_signature_trust_dry_run_blocks_invalid_manifest() {
        let manifest = json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "local"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "install" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": false },
                    "timeoutMs": 10
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_signature_trust_dry_run(&declarations, &[]);

        assert_eq!(report.status, "blocked");
        assert!(!report.manifest_v1_valid);
        assert!(!report.signature_trust_dry_run_ready);
        assert_eq!(report.candidate_count, 0);
        assert_eq!(report.validation_error_count, 3);
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidPermission
            })
        );
        assert!(report.validation_errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::InvalidActivationEvent
        }));
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidToolPolicy
            })
        );
        assert_signature_trust_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_install_cache_fixture_dry_run_projects_fixture_without_writes() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_install_cache_fixture_dry_run(&declarations, &[]);

        assert_eq!(
            report.schema_version,
            PLUGIN_MANIFEST_V1_INSTALL_CACHE_FIXTURE_DRY_RUN_SCHEMA_VERSION
        );
        assert_eq!(report.status, "ready_blocked");
        assert!(report.manifest_v1_valid);
        assert!(report.signature_trust_dry_run_ready);
        assert!(report.install_cache_fixture_dry_run_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.fixture_entry_count, 2);
        assert_eq!(report.noop_preflight_ready_count, 2);
        assert_eq!(report.stable_cache_key_count, 2);
        assert_eq!(report.stable_idempotency_key_count, 2);
        assert_eq!(report.rollback_uninstall_noop_ready_count, 2);
        assert_eq!(report.dynamic_activation_boundary_ready_count, 2);
        assert_eq!(report.denial_receipt_projected_count, 2);
        assert_eq!(report.install_cache_materialized_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.plugin_cache_mutated_count, 0);
        assert_eq!(report.fixture_written_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert_eq!(report.runtime_event_log_written_count, 0);
        assert!(report.entries.iter().all(|entry| {
            entry
                .cache_path
                .starts_with(".hepta/plugin-install-cache/dry-run/")
                && entry.cache_key.starts_with("plugin-v1-cache-key:")
                && entry
                    .artifact_digest
                    .starts_with("dry-run-artifact-digest:")
                && entry
                    .idempotency_key
                    .starts_with("plugin-v1-install-idempotency:")
                && entry
                    .rollback_plan_id
                    .starts_with("plugin-v1-rollback-uninstall-noop:")
                && entry
                    .denial_receipt_id
                    .starts_with("plugin-v1-install-denial:")
                && entry.noop_preflight_ready
                && entry.idempotency_key_stable
                && entry.rollback_uninstall_noop_ready
                && entry.dynamic_activation_boundary_ready
                && entry.blockers.contains(&"signature_artifact_missing")
                && entry.blockers.contains(&"trust_root_missing")
                && entry
                    .blockers
                    .contains(&"install_cache_materialization_disabled")
                && !entry.install_cache_materialized
                && !entry.plugin_installed
                && !entry.plugin_cache_mutated
                && !entry.fixture_written
                && !entry.denial_receipt_persisted
                && !entry.runtime_event_log_written
        }));
        assert_install_cache_fixture_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_install_cache_fixture_dry_run_keeps_materialization_closed_with_materials()
     {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);
        let inputs = hepta_signature_trust_inputs(
            /*signature_artifact_present*/ true, /*trust_root_present*/ true,
            /*operator_evidence_present*/ true, /*operator_acceptance_present*/ true,
        );

        let report = plan_plugin_manifest_v1_install_cache_fixture_dry_run(&declarations, &inputs);

        assert_eq!(report.status, "ready_blocked");
        assert!(report.install_cache_fixture_dry_run_ready);
        assert_eq!(report.fixture_entry_count, 2);
        assert_eq!(report.install_cache_materialized_count, 0);
        assert_eq!(report.plugin_installed_count, 0);
        assert_eq!(report.denial_receipt_persisted_count, 0);
        assert!(report.entries.iter().all(|entry| {
            entry.signature_artifact_present
                && entry.trust_root_present
                && entry.operator_evidence_present
                && entry.operator_acceptance_present
                && !entry.signature_verified
                && !entry.trust_root_accepted
                && entry.blockers.contains(&"signature_not_verified")
                && entry.blockers.contains(&"trust_root_not_accepted")
                && entry.blockers.contains(&"plugin_install_disabled")
                && !entry.install_cache_materialized
                && !entry.plugin_installed
                && !entry.tool_registry_registration_allowed
                && !entry.tool_invocation_allowed
        }));
        assert_install_cache_fixture_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_install_cache_fixture_dry_run_blocks_invalid_manifest() {
        let manifest = json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "local"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "install" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": false },
                    "timeoutMs": 10
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_install_cache_fixture_dry_run(&declarations, &[]);

        assert_eq!(report.status, "blocked");
        assert!(!report.manifest_v1_valid);
        assert!(!report.signature_trust_dry_run_ready);
        assert!(!report.install_cache_fixture_dry_run_ready);
        assert_eq!(report.fixture_entry_count, 0);
        assert_eq!(report.validation_error_count, 3);
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidPermission
            })
        );
        assert!(report.validation_errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::InvalidActivationEvent
        }));
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidToolPolicy
            })
        );
        assert_install_cache_fixture_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_sandbox_enforcement_dry_run_covers_fixture_rules() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_sandbox_enforcement_dry_run(&declarations);

        assert_eq!(
            report.schema_version,
            PLUGIN_MANIFEST_V1_SANDBOX_ENFORCEMENT_DRY_RUN_SCHEMA_VERSION
        );
        assert_eq!(report.status, "ready_blocked");
        assert!(report.manifest_v1_valid);
        assert!(report.sandbox_enforcement_dry_run_ready);
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.sandbox_rule_count, 8);
        assert_eq!(report.enforced_rule_count, 8);
        assert_eq!(report.network_none_enforced_count, 2);
        assert_eq!(report.filesystem_read_only_enforced_count, 1);
        assert_eq!(report.connector_scope_enforced_count, 1);
        assert_eq!(report.manual_activation_only_enforced_count, 2);
        assert_eq!(report.approval_ledger_policy_enforced_count, 2);
        assert_eq!(report.credential_boundary_enforced_count, 2);
        assert_eq!(report.transport_boundary_enforced_count, 2);
        assert_eq!(report.runtime_persistence_boundary_enforced_count, 2);
        assert!(report.entries.iter().all(|entry| {
            entry.network_none_enforced
                && (entry.filesystem_read_only_enforced ^ entry.connector_scope_enforced)
                && entry.manual_activation_only_enforced
                && entry.approval_ledger_policy_enforced
                && entry.credential_boundary_enforced
                && entry.transport_boundary_enforced
                && entry.runtime_persistence_boundary_enforced
                && entry.blockers.contains(&"permission_grant_disabled")
                && entry.blockers.contains(&"credential_read_disabled")
                && entry.blockers.contains(&"runtime_mutation_disabled")
                && !entry.permission_granted
                && !entry.network_access_allowed
                && !entry.credential_read_allowed
                && !entry.connector_start_allowed
                && !entry.runtime_mutation_allowed
                && !entry.tool_registry_registration_allowed
                && !entry.tool_invocation_allowed
                && !entry.live_execution_allowed
        }));
        assert_sandbox_enforcement_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_sandbox_enforcement_dry_run_keeps_runtime_boundaries_closed() {
        let manifest = hepta_fixture_manifest();
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_sandbox_enforcement_dry_run(&declarations);

        assert!(report.sandbox_enforcement_dry_run_ready);
        assert_eq!(report.enforced_rule_count, report.sandbox_rule_count);
        assert!(!report.permission_grant_allowed);
        assert!(!report.connector_start_allowed);
        assert!(!report.network_access_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.runtime_mutation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
        assert_sandbox_enforcement_boundaries_closed(&report);
    }

    #[test]
    fn plugin_manifest_v1_sandbox_enforcement_dry_run_blocks_invalid_manifest() {
        let manifest = json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "local"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "install" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": false },
                    "timeoutMs": 10
                }
            }
        });
        let declarations = declarations_from_manifest(&manifest);

        let report = plan_plugin_manifest_v1_sandbox_enforcement_dry_run(&declarations);

        assert_eq!(report.status, "blocked");
        assert!(!report.manifest_v1_valid);
        assert!(!report.sandbox_enforcement_dry_run_ready);
        assert_eq!(report.candidate_count, 0);
        assert_eq!(report.enforced_rule_count, 0);
        assert_eq!(report.validation_error_count, 3);
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidPermission
            })
        );
        assert!(report.validation_errors.iter().any(|error| {
            error.kind == PluginManifestV1ValidationErrorKind::InvalidActivationEvent
        }));
        assert!(
            report.validation_errors.iter().any(|error| {
                error.kind == PluginManifestV1ValidationErrorKind::InvalidToolPolicy
            })
        );
        assert_sandbox_enforcement_boundaries_closed(&report);
    }

    fn declarations_from_manifest(manifest: &JsonValue) -> PluginManifestToolDeclarations {
        resolve_tool_declarations(
            manifest.get("toolSchemas"),
            manifest.get("permissions"),
            manifest.get("activationEvents"),
            manifest.get("toolPolicies"),
        )
    }

    fn has_error_kind(
        report: &PluginManifestV1ValidationReport,
        kind: PluginManifestV1ValidationErrorKind,
    ) -> bool {
        report.errors.iter().any(|error| error.kind == kind)
    }

    fn assert_signature_trust_boundaries_closed(
        report: &PluginManifestV1SignatureTrustDryRunReport,
    ) {
        assert!(!report.signature_acceptance_allowed);
        assert!(!report.trust_root_acceptance_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    fn assert_install_cache_fixture_boundaries_closed(
        report: &PluginManifestV1InstallCacheFixtureDryRunReport,
    ) {
        assert!(!report.install_cache_materialization_allowed);
        assert!(!report.plugin_install_allowed);
        assert!(!report.plugin_cache_mutation_allowed);
        assert!(!report.fixture_write_allowed);
        assert!(!report.denial_receipt_persistence_allowed);
        assert!(!report.runtime_event_log_write_allowed);
        assert!(!report.dynamic_activation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    fn assert_sandbox_enforcement_boundaries_closed(
        report: &PluginManifestV1SandboxEnforcementDryRunReport,
    ) {
        assert!(!report.permission_grant_allowed);
        assert!(!report.connector_start_allowed);
        assert!(!report.network_access_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.runtime_mutation_allowed);
        assert!(!report.tool_registry_registration_allowed);
        assert!(!report.tool_invocation_allowed);
        assert!(!report.live_execution_allowed);
    }

    fn hepta_signature_trust_inputs(
        signature_artifact_present: bool,
        trust_root_present: bool,
        operator_evidence_present: bool,
        operator_acceptance_present: bool,
    ) -> Vec<PluginManifestV1SignatureTrustDryRunInput> {
        vec![
            PluginManifestV1SignatureTrustDryRunInput {
                candidate_tool_id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
                    .to_string(),
                signature_artifact_present,
                trust_root_present,
                operator_evidence_present,
                operator_acceptance_present,
            },
            PluginManifestV1SignatureTrustDryRunInput {
                candidate_tool_id:
                    "preview:connector:hepta-system@hepta-local:hepta_system_local_app".to_string(),
                signature_artifact_present,
                trust_root_present,
                operator_evidence_present,
                operator_acceptance_present,
            },
        ]
    }

    fn hepta_fixture_manifest() -> JsonValue {
        json!({
            "toolSchemas": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                },
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": {
                    "inputSchema": { "type": "object" },
                    "outputSchema": { "type": "object" }
                }
            },
            "permissions": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "filesystem": "read-only",
                    "network": "none"
                },
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": {
                    "connector": "hepta-local",
                    "network": "none"
                }
            },
            "activationEvents": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": [
                    { "type": "manual" }
                ],
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": [
                    { "type": "manual" }
                ]
            },
            "toolPolicies": {
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp": {
                    "approval": { "kind": "onUse" },
                    "ledger": { "required": true },
                    "timeoutMs": 30000
                },
                "preview:connector:hepta-system@hepta-local:hepta_system_local_app": {
                    "approval": { "kind": "install" },
                    "ledger": { "required": true },
                    "timeoutMs": 30000
                }
            }
        })
    }
}
