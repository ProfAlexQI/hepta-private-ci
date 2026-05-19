use hepta_core::{
    ExternalProductionStatus, HeptaError, LocalConfigImportStatus, MemoryRecord, SessionRecord,
};

use super::{
    DoctorCheck, DoctorProviderProbe, DoctorStatus, integrity, intelligence_eval_gate,
    report_bundle, report_input_collectors,
    runtime_metrics::{self, DoctorRuntimeStatInputs},
};
use crate::{
    EventRecord, ModelSelection, RuntimeKernel, RuntimeSnapshot, SessionApprovalState, TurnRecord,
};

pub(super) struct DoctorReportInputs {
    active_session_id: String,
    model_selection: ModelSelection,
    registered_providers: usize,
    registered_tools: usize,
    session_count: usize,
    total_topic_sessions: usize,
    total_topic_graph_edges: usize,
    raw_sessions: Vec<SessionRecord>,
    raw_memories: Vec<MemoryRecord>,
    history: Vec<TurnRecord>,
    events: Vec<EventRecord>,
    active_session_pending_approvals: usize,
    approval_sessions: Vec<SessionApprovalState>,
    provider_probes: Vec<DoctorProviderProbe>,
    snapshot: RuntimeSnapshot,
}

impl DoctorReportInputs {
    pub(super) async fn gather(runtime: &RuntimeKernel) -> Result<Self, HeptaError> {
        let collected = report_input_collectors::collect_runtime_state_inputs(runtime)?;
        let provider_probes = report_input_collectors::collect_provider_probes(runtime).await;

        Ok(Self {
            active_session_id: collected.active_session_id,
            model_selection: collected.model_selection,
            registered_providers: collected.registered_providers,
            registered_tools: collected.registered_tools,
            session_count: collected.session_count,
            total_topic_sessions: collected.total_topic_sessions,
            total_topic_graph_edges: collected.total_topic_graph_edges,
            raw_sessions: collected.raw_sessions,
            raw_memories: collected.raw_memories,
            history: collected.history,
            events: collected.events,
            active_session_pending_approvals: collected.active_session_pending_approvals,
            approval_sessions: collected.approval_sessions,
            provider_probes,
            snapshot: collected.snapshot,
        })
    }

    pub(super) fn into_report_bundle(
        self,
        runtime: &RuntimeKernel,
    ) -> report_bundle::DoctorReportBundle {
        let integrity_checks = self.integrity_checks(runtime);
        let stats = runtime_metrics::collect_runtime_stats(
            runtime,
            &self.active_session_id,
            &self.snapshot,
            DoctorRuntimeStatInputs {
                registered_providers: self.registered_providers,
                registered_tools: self.registered_tools,
                sessions: self.session_count,
                raw_session_records: self.raw_sessions.len(),
                memories: self.raw_memories.len(),
                history_entries: self.history.len(),
                active_session_pending_approvals: self.active_session_pending_approvals,
                approval_scoped_sessions: self.approval_sessions.len(),
                total_topic_sessions: self.total_topic_sessions,
                total_topic_graph_edges: self.total_topic_graph_edges,
            },
        );

        report_bundle::assemble_report_bundle(
            self.model_selection.active,
            self.active_session_id,
            stats,
            self.provider_probes,
            integrity_checks,
        )
    }

    fn integrity_checks(&self, runtime: &RuntimeKernel) -> Vec<DoctorCheck> {
        let mut checks = runtime.session_integrity_checks(
            &self.model_selection.active,
            &self.model_selection.available,
            &self.raw_sessions,
            &self.raw_memories,
            &self.history,
            &self.events,
            &self.approval_sessions,
            &self.snapshot,
        );
        checks.push(intelligence_eval_gate::active_session_replay_eval_check(
            runtime,
            &self.active_session_id,
        ));
        checks.push(
            intelligence_eval_gate::active_session_neuron_lifecycle_check(
                runtime,
                &self.active_session_id,
            ),
        );
        checks.push(local_config_import_check());
        checks.push(external_production_readiness_check());
        checks.push(production_parity_check());
        checks
    }
}

fn local_config_import_check() -> DoctorCheck {
    let manifest_path = std::env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST")
        .or_else(|_| std::env::var("HEPTA_LOCAL_IMPORT_MANIFEST"))
        .unwrap_or_else(|_| ".hepta/local-import/manifest.json".into());
    let status = LocalConfigImportStatus::from_manifest_path(&manifest_path);
    let import_required = std::env::var("HEPTA_REQUIRE_LOCAL_IMPORT").as_deref() == Ok("1");
    let status_kind = if !status.manifest_present && !import_required {
        DoctorStatus::Ok
    } else if status.local_import_complete && status.secret_material_local_only {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    };

    let detail = match status.manifest.as_ref() {
        Some(manifest) => {
            let startup = manifest.startup_config.as_ref();
            let surface = manifest.external_config_surface.as_ref();
            let optional = manifest.optional_config_catalog.as_ref();
            format!(
                "manifest={} config_files={} auth_files={} credential_files={} skill_sources={} skills={} copied_files={} startup_sections={} startup_model_providers={} startup_models={} startup_search_engines={} startup_image_engines={} startup_channels={} config_surface_sections={} config_surface_options={} config_surface_aligned={} optional_catalogs={} optional_source_files={} optional_schema_options={} optional_schema_choices={} optional_bundled_plugins={} optional_setup_surfaces={} optional_channel_catalog={} optional_channel_message_schema={} optional_auth_choices={} optional_auth_flags={} optional_secret_targets={} optional_browser_profiles={} optional_browser_options={} optional_tool_policy_groups={} optional_tool_policy_profiles={} optional_effective_tools={} optional_tool_schemas={} optional_tool_params={} optional_node_commands={} optional_debug_proxy={} optional_chat_commands={} optional_subcli_commands={} optional_skill_catalog={} optional_model_providers={} optional_model_models={} optional_search_providers={} optional_image_models={} optional_video_models={} optional_music_models={} optional_media_providers={} optional_speech_providers={} optional_channels={} optional_acp_agents={} optional_plugins={} optional_tools={} private_policy_local_only={}",
                status.manifest_path,
                manifest.config_file_count,
                manifest.auth_file_count,
                manifest.credential_file_count,
                manifest.skill_source_count,
                manifest.skill_count,
                manifest.copied_file_count,
                startup.map(|item| item.section_count).unwrap_or_default(),
                startup
                    .map(|item| item.model_provider_count)
                    .unwrap_or_default(),
                startup.map(|item| item.model_count).unwrap_or_default(),
                startup
                    .map(|item| item.search_engine_count)
                    .unwrap_or_default(),
                startup
                    .map(|item| item.image_generation_engine_count)
                    .unwrap_or_default(),
                startup
                    .map(|item| item.enabled_external_channel_count)
                    .unwrap_or_default(),
                surface.map(|item| item.section_count).unwrap_or_default(),
                surface.map(|item| item.option_count).unwrap_or_default(),
                surface
                    .map(|item| item.top_level_alignment_complete)
                    .unwrap_or(false),
                optional.map(|item| item.catalog_count).unwrap_or_default(),
                optional
                    .map(|item| item.source_file_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.config_schema_option_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.config_schema_choice_path_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.bundled_plugin_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.setup_surface_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.channel_catalog_entry_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.channel_message_schema_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.provider_auth_choice_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.provider_auth_flag_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.secret_target_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.browser_profile_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.browser_config_option_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.tool_policy_group_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.tool_policy_profile_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.effective_tool_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.tool_schema_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.tool_parameter_option_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.node_command_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.debug_proxy_coverage_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.chat_command_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.subcli_command_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.skill_catalog_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.model_provider_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.model_catalog_model_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.search_engine_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.image_generation_model_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.video_generation_model_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.music_generation_model_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.media_understanding_provider_count)
                    .unwrap_or_default(),
                optional
                    .map(|item| item.speech_provider_count)
                    .unwrap_or_default(),
                optional.map(|item| item.channel_count).unwrap_or_default(),
                optional
                    .map(|item| item.acp_agent_count)
                    .unwrap_or_default(),
                optional.map(|item| item.plugin_count).unwrap_or_default(),
                optional.map(|item| item.tool_count).unwrap_or_default(),
                status.secret_material_local_only,
            )
        }
        None => format!(
            "manifest={} missing_or_unreadable: {}",
            status.manifest_path,
            status.error.as_deref().unwrap_or("unknown error")
        ),
    };

    DoctorCheck {
        name: integrity::LOCAL_CONFIG_IMPORT_READY.into(),
        status: status_kind,
        detail,
    }
}

fn external_production_readiness_check() -> DoctorCheck {
    let manifest_path = std::env::var("HEPTA_EXTERNAL_PRODUCTION_MANIFEST")
        .unwrap_or_else(|_| ".hepta/external-production/manifest.json".into());
    let report = hepta_core::external_production_readiness_report(&manifest_path);
    let require_external = std::env::var("HEPTA_REQUIRE_EXTERNAL_PRODUCTION").as_deref() == Ok("1");
    let status = if report.external_production_ready || !require_external {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    };
    let first_blocker = report
        .requirements
        .iter()
        .find(|requirement| requirement.status != ExternalProductionStatus::Verified)
        .map(|requirement| requirement.id)
        .unwrap_or("none");
    DoctorCheck {
        name: integrity::EXTERNAL_PRODUCTION_READINESS.into(),
        status,
        detail: format!(
            "manifest={} verified={}/{} local_control_plane={} blocked={} first_blocker={} approval_required={}",
            report.manifest_path,
            report.verified_count,
            report.requirement_count,
            report.local_control_plane_coverage_percent,
            report.blocked_count,
            first_blocker,
            report.external_side_effects_require_operator_approval,
        ),
    }
}

fn production_parity_check() -> DoctorCheck {
    let local_manifest_path = std::env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST")
        .or_else(|_| std::env::var("HEPTA_LOCAL_IMPORT_MANIFEST"))
        .unwrap_or_else(|_| ".hepta/local-import/manifest.json".into());
    let external_manifest_path = std::env::var("HEPTA_EXTERNAL_PRODUCTION_MANIFEST")
        .unwrap_or_else(|_| ".hepta/external-production/manifest.json".into());
    let native = hepta_core::hepta_native_absorption_report();
    let local_import = LocalConfigImportStatus::from_manifest_path(&local_manifest_path)
        .redacted_for_product_surface();
    let external = hepta_core::external_production_readiness_report(&external_manifest_path);
    let control_ui = hepta_core::control_ui_report();
    let report =
        hepta_core::production_parity_report(&native, &local_import, &external, &control_ui);
    let require_external = std::env::var("HEPTA_REQUIRE_EXTERNAL_PRODUCTION").as_deref() == Ok("1");
    let status = if report.local_evidence_gated_ready || !require_external {
        DoctorStatus::Ok
    } else {
        DoctorStatus::Fail
    };
    let first_gap = report
        .remaining_gaps
        .first()
        .map(String::as_str)
        .unwrap_or("none");
    DoctorCheck {
        name: integrity::PRODUCTION_PARITY_READY.into(),
        status,
        detail: format!(
            "status={} overall={} complete_dimensions={}/{} baseline_completion={} ahead_dimensions={} local_evidence_ready={} public_ga_ready={} first_gap={}",
            report.status,
            report.overall_completion_percent,
            report.complete_dimension_count,
            report.dimension_count,
            report.baseline_completion_percent,
            report.baseline_surpass_count,
            report.local_evidence_gated_ready,
            report.public_ga_ready,
            first_gap,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_config_import_check_is_redacted_and_non_destructive() {
        let check = local_config_import_check();

        assert_eq!(check.name, integrity::LOCAL_CONFIG_IMPORT_READY);
        assert!(!check.detail.contains("token"));
        assert!(!check.detail.contains("secret"));
    }

    #[test]
    fn external_production_check_is_explicit_and_non_destructive() {
        let check = external_production_readiness_check();

        assert_eq!(check.name, integrity::EXTERNAL_PRODUCTION_READINESS);
        assert!(check.detail.contains("approval_required=true"));
        assert!(!check.detail.contains("token"));
        assert!(!check.detail.contains("secret"));
    }

    #[test]
    fn production_parity_check_is_redacted_and_reports_completion_shape() {
        let check = production_parity_check();

        assert_eq!(check.name, integrity::PRODUCTION_PARITY_READY);
        assert!(check.detail.contains("overall="));
        assert!(check.detail.contains("baseline_completion="));
        assert!(!check.detail.contains("token"));
        assert!(!check.detail.contains("secret"));
    }
}
