use super::*;

pub(super) type NativeReportResponse = (&'static str, &'static str, String);

type NativeReportRenderer = for<'a> fn(NativeReportContext<'a>) -> NativeReportResponse;

struct NativeReportContext<'a> {
    options: &'a NativeGatewayOptions,
    telegram_plugin: NativeTelegramPluginStatus,
}

#[derive(Clone, Copy)]
struct NativeReportDefinition {
    key: &'static str,
    renderer: NativeReportRenderer,
}

const fn report(key: &'static str, renderer: NativeReportRenderer) -> NativeReportDefinition {
    NativeReportDefinition { key, renderer }
}

const NATIVE_REPORT_JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";

macro_rules! json_report {
    ($key:literal, $context:ident => $body:expr) => {
        report($key, |$context| {
            ("200 OK", NATIVE_REPORT_JSON_CONTENT_TYPE, $body)
        })
    };
}

const NATIVE_REPORT_DEFINITIONS: &[NativeReportDefinition] = &[
    report("native_report_000", |_context| {
        (
            "200 OK",
            "text/html; charset=utf-8",
            hepta_core::control_ui::control_ui_index_html(),
        )
    }),
    report("native_report_001", |_context| {
        (
            "200 OK",
            "text/css; charset=utf-8",
            hepta_core::control_ui::CONTROL_UI_STYLES_CSS.to_string(),
        )
    }),
    report("native_report_002", |context| {
        (
            "200 OK",
            "text/html; charset=utf-8",
            index_html(context.options, &context.telegram_plugin),
        )
    }),
    json_report!("native_report_003", _context => json_or_error(&HealthResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
    })),
    json_report!("native_report_004", context => watchdog_state::watchdog_state_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_005", context => native_gateway_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_006", context => native_control_ui_audit_json(
        NativeControlUiAuditSurface::ControlUi,
        context.options,
        &context.telegram_plugin,
    )),
    json_report!("native_report_007", context => native_control_ui_audit_json(
        NativeControlUiAuditSurface::UiContractAudit,
        context.options,
        &context.telegram_plugin,
    )),
    json_report!("native_report_008", context => native_control_ui_audit_json(
        NativeControlUiAuditSurface::GatewayDispatch,
        context.options,
        &context.telegram_plugin,
    )),
    json_report!("native_report_009", context => native_control_ui_audit_json(
        NativeControlUiAuditSurface::UiActionPlanGatewayDispatch,
        context.options,
        &context.telegram_plugin,
    )),
    json_report!("native_report_010", context => native_control_ui_audit_json(
        NativeControlUiAuditSurface::ExternalAgentBenchmark,
        context.options,
        &context.telegram_plugin,
    )),
    json_report!("native_report_011", context => json_or_error(&gateway_replacement_readiness(
        context.options,
        &context.telegram_plugin,
    ))),
    json_report!("native_report_012", context => json_or_error(&gateway_live_activation_plan(
        context.options,
        &context.telegram_plugin,
    ))),
    json_report!("native_report_013", _context => json_or_error(&control_ui_route_parity_report())),
    json_report!("native_report_014", context => json_or_error(&hepta_merge_completion_report(context.options))),
    json_report!("native_report_015", _context => json_or_error(&hepta_cli_command_inventory_report())),
    json_report!("native_report_016", _context => json_or_error(&hepta_provider_metadata_inventory_report())),
    json_report!("native_report_017", _context => json_or_error(&hepta_runtime_session_dry_run_inventory_report())),
    json_report!("native_report_018", _context => json_or_error(&hepta_context_recall_worker_scheduler_handoff_report())),
    json_report!("native_report_019", _context => json_or_error(&hepta_channel_adapter_status_inventory_report())),
    json_report!("native_report_020", _context => json_or_error(&hepta_local_tooling_content_inventory_report())),
    json_report!("native_report_021", _context => json_or_error(&hepta_systems_tool_registry_inventory_report())),
    json_report!("native_report_022", _context => json_or_error(&hepta_systems_workflow_definition_registry_report())),
    json_report!("native_report_023", _context => json_or_error(&hepta_memory_capability_absorption_inventory_report())),
    json_report!("native_report_024", _context => json_or_error(&hepta_memory_intelligence_kg_full_enablement_runtime_readiness_report())),
    json_report!("native_report_025", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_readiness_report(),
    )),
    json_report!("native_report_026", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_report(),
    )),
    json_report!("native_report_027", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_report(),
    )),
    json_report!("native_report_028", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_report(),
    )),
    json_report!("native_report_029", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_separation_report(),
    )),
    json_report!("native_report_030", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_report(),
    )),
    json_report!("native_report_031", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report(),
    )),
    json_report!("native_report_032", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report(),
    )),
    json_report!("native_report_033", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report(),
    )),
    json_report!("native_report_034", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_report(),
    )),
    json_report!("native_report_035", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report(),
    )),
    json_report!("native_report_036", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report(),
    )),
    json_report!("native_report_037", _context => json_or_error(&hepta_memory_intelligence_kg_activation_truth_index_report())),
    json_report!("native_report_038", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_report(),
    )),
    json_report!("native_report_039", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report(),
    )),
    json_report!("native_report_040", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report(),
    )),
    json_report!("native_report_041", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report(),
    )),
    json_report!("native_report_042", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_043", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_044", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_045", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_046", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_047", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_048", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_049", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_050", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
    )),
    json_report!("native_report_051", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_report(),
    )),
    json_report!("native_report_052", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_report(),
    )),
    json_report!("native_report_053", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_report(),
    )),
    json_report!("native_report_054", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_report(),
    )),
    json_report!("native_report_055", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_report(),
    )),
    json_report!("native_report_056", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_receipt_audit_lane_report(),
    )),
    json_report!("native_report_057", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_report(),
    )),
    json_report!("native_report_058", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_report(),
    )),
    json_report!("native_report_059", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_report(),
    )),
    json_report!("native_report_060", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_report(),
    )),
    json_report!("native_report_061", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_report(),
    )),
    json_report!("native_report_062", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_report(),
    )),
    json_report!("native_report_063", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_report(),
    )),
    json_report!("native_report_064", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_report(),
    )),
    json_report!("native_report_065", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_report(),
    )),
    json_report!("native_report_066", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_report(),
    )),
    json_report!("native_report_067", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_068", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_069", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_070", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_071", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_072", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_073", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_074", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_075", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
    )),
    json_report!("native_report_076", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report(),
    )),
    json_report!("native_report_077", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report(),
    )),
    json_report!("native_report_078", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_079", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report(),
    )),
    json_report!("native_report_080", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_report(),
    )),
    json_report!("native_report_081", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_report(),
    )),
    json_report!("native_report_082", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_report(),
    )),
    json_report!("native_report_083", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_report(),
    )),
    json_report!("native_report_084", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_report(),
    )),
    json_report!("native_report_085", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_086", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_087", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_088", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_089", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_090", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_091", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report(),
    )),
    json_report!("native_report_092", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report(),
    )),
    json_report!("native_report_093", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report(),
    )),
    json_report!("native_report_094", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report(),
    )),
    json_report!("native_report_095", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report(),
    )),
    json_report!("native_report_096", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report(),
    )),
    json_report!("native_report_097", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_098", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_099", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_100", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_101", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_102", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_103", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_104", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_105", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_report(),
    )),
    json_report!("native_report_106", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_report(),
    )),
    json_report!("native_report_107", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_report(),
    )),
    json_report!("native_report_108", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_report(),
    )),
    json_report!("native_report_109", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_report(),
    )),
    json_report!("native_report_110", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_report(),
    )),
    json_report!("native_report_111", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_report(),
    )),
    json_report!("native_report_112", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_report(),
    )),
    json_report!("native_report_113", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_report(),
    )),
    json_report!("native_report_114", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_report(),
    )),
    json_report!("native_report_115", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_116", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_117", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_118", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_119", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_120", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_121", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_122", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_123", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_report(),
    )),
    json_report!("native_report_124", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_report(),
    )),
    json_report!("native_report_125", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_report(),
    )),
    json_report!("native_report_126", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_report(),
    )),
    json_report!("native_report_127", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_report(),
    )),
    json_report!("native_report_128", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_report(),
    )),
    json_report!("native_report_129", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_130", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_131", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_report(),
    )),
    json_report!("native_report_132", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_133", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_report(),
    )),
    json_report!("native_report_134", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_135", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_136", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_report(),
    )),
    json_report!("native_report_137", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_report(),
    )),
    json_report!("native_report_138", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_report(),
    )),
    json_report!("native_report_139", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_report(),
    )),
    json_report!("native_report_140", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_report(),
    )),
    json_report!("native_report_141", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_142", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_143", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_144", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_report(),
    )),
    json_report!("native_report_145", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_146", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_147", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_148", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_149", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report(),
    )),
    json_report!("native_report_150", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report(),
    )),
    json_report!("native_report_151", _context => json_or_error(
        &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report(),
    )),
    json_report!("native_report_152", _context => json_or_error(
        &hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report(),
    )),
    json_report!("native_report_153", _context => json_or_error(&hepta_intelligence_bounded_context_attachment_preview_readback_report())),
    json_report!("native_report_154", _context => hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_json()),
    json_report!("native_report_155", _context => json_or_error(&hepta_kg_read_only_adapter_shadow_rank_canary_report())),
    json_report!("native_report_156", _context => json_or_error(&hepta_provider_router_dry_run_envelope_readback_audit_report())),
    json_report!("native_report_157", _context => json_or_error(
        &hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report(),
    )),
    json_report!("native_report_158", _context => json_or_error(
        &hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_report(),
    )),
    json_report!("native_report_159", _context => json_or_error(&hepta_full_live_activation_closure_index_report())),
    json_report!("native_report_160", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_approval_packet_boundary_report(),
    )),
    json_report!("native_report_161", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report(),
    )),
    json_report!("native_report_162", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report(
        ),
    )),
    json_report!("native_report_163", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report(
        ),
    )),
    json_report!("native_report_164", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report(
        ),
    )),
    json_report!("native_report_165", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report(
        ),
    )),
    json_report!("native_report_166", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report(
        ),
    )),
    json_report!("native_report_167", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report(
        ),
    )),
    json_report!("native_report_168", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report(
        ),
    )),
    json_report!("native_report_169", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report(
        ),
    )),
    json_report!("native_report_170", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report(
        ),
    )),
    json_report!("native_report_171", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report(
        ),
    )),
    json_report!("native_report_172", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_report(
        ),
    )),
    json_report!("native_report_173", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_report(
        ),
    )),
    json_report!("native_report_174", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_report(
        ),
    )),
    json_report!("native_report_175", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_report(
        ),
    )),
    json_report!("native_report_176", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report(
        ),
    )),
    json_report!("native_report_177", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report(
        ),
    )),
    json_report!("native_report_178", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report(
        ),
    )),
    json_report!("native_report_179", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_report(
        ),
    )),
    json_report!("native_report_180", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_report(),
    )),
    json_report!("native_report_181", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_report(),
    )),
    json_report!("native_report_182", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_report(),
    )),
    json_report!("native_report_183", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_report(),
    )),
    json_report!("native_report_184", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_report(),
    )),
    json_report!("native_report_185", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_report(),
    )),
    json_report!("native_report_186", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_report(),
    )),
    json_report!("native_report_187", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_report(),
    )),
    json_report!("native_report_188", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_report(),
    )),
    json_report!("native_report_189", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_report(),
    )),
    json_report!("native_report_190", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_report(),
    )),
    json_report!("native_report_191", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_report(),
    )),
    json_report!("native_report_192", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_report(),
    )),
    json_report!("native_report_193", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_report(),
    )),
    json_report!("native_report_194", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_report(),
    )),
    json_report!("native_report_195", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report(),
    )),
    json_report!("native_report_196", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report(),
    )),
    json_report!("native_report_197", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report(),
    )),
    json_report!("native_report_198", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report(),
    )),
    json_report!("native_report_199", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_preflight_boundary_report(),
    )),
    json_report!("native_report_200", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report(),
    )),
    json_report!("native_report_201", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_report(),
    )),
    json_report!("native_report_202", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report(),
    )),
    json_report!("native_report_203", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report(),
    )),
    json_report!("native_report_204", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report(),
    )),
    json_report!("native_report_205", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report(),
    )),
    json_report!("native_report_206", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report(),
    )),
    json_report!("native_report_207", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report(),
    )),
    json_report!("native_report_208", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report(),
    )),
    json_report!("native_report_209", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report(),
    )),
    json_report!("native_report_210", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report(),
    )),
    json_report!("native_report_211", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report(),
    )),
    json_report!("native_report_212", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report(),
    )),
    json_report!("native_report_213", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report(),
    )),
    json_report!("native_report_214", _context => json_or_error(
        &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_report(),
    )),
    json_report!("native_report_215", _context => json_or_error(&hepta_upstream_codex_latest_multisurface_absorption_report())),
    json_report!("native_report_216", _context => json_or_error(&hepta_first_model_invocation_separate_approval_slice_preflight_report())),
    json_report!("native_report_217", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_report(),
    )),
    json_report!("native_report_218", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_report(),
    )),
    json_report!("native_report_219", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_report(),
    )),
    json_report!("native_report_220", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_report(),
    )),
    json_report!("native_report_221", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_report(),
    )),
    json_report!("native_report_222", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_report(),
    )),
    json_report!("native_report_223", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_report(),
    )),
    json_report!("native_report_224", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_report(),
    )),
    json_report!("native_report_225", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_report(),
    )),
    json_report!("native_report_226", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_report(),
    )),
    json_report!("native_report_227", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_report(),
    )),
    json_report!("native_report_228", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
    )),
    json_report!("native_report_229", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
    )),
    json_report!("native_report_230", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
    )),
    json_report!("native_report_231", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_report(),
    )),
    json_report!("native_report_232", _context => json_or_error(
        &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_report(),
    )),
    json_report!("native_report_233", _context => json_or_error(&hepta_first_model_positive_approval_packet_boundary_report())),
    json_report!("native_report_234", _context => json_or_error(&hepta_scoped_memory_canary_durable_receipt_boundary_report())),
    json_report!("native_report_235", _context => json_or_error(&hepta_release_hardening_status_gate_report())),
    json_report!("native_report_236", _context => json_or_error(&hepta_provider_channel_dry_run_plan_report())),
    json_report!("native_report_237", _context => json_or_error(&hepta_native_packaging_gate_report())),
    json_report!("native_report_238", _context => json_or_error(&hepta_legacy_compatibility_closure_report())),
    json_report!("native_report_239", context => json_or_error(&hepta_public_ga_operator_approval_packet_report(
        context.options,
        &context.telegram_plugin,
    ))),
    json_report!("native_report_240", context => json_or_error(&hepta_public_ga_readiness_report(
        context.options,
        &context.telegram_plugin,
    ))),
    json_report!("native_report_241", _context => json_or_error(&hepta_gateway::hepta_core_fusion_readiness_report())),
    json_report!("native_report_242", _context => json_or_error(&hepta_gateway::hepta_name_repository_closure_report())),
    json_report!("native_report_243", _context => json_or_error(&hepta_gateway::hepta_engine_dependency_closure_report())),
    json_report!("native_report_244", _context => json_or_error(&hepta_gateway::hepta_codex_engine_adapter_boundary_report())),
    json_report!("native_report_245", context => operator_snapshot_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_246", context => operator_console_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_247", context => operator_security_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_248", _context => native_post_execution_readiness_json()),
    json_report!("native_report_249", _context => native_post_execution_stores_json()),
    json_report!("native_report_250", _context => native_post_activation_plan_json()),
    json_report!("native_report_251", _context => native_post_rollout_evidence_json()),
    json_report!("native_report_252", _context => native_post_gray_release_evidence_json()),
    json_report!("native_report_253", _context => native_sessions_json("/sessions --json", "native_sessions_inventory")),
    json_report!("native_report_254", _context => native_sessions_json("/session-activity --json", "native_session_activity")),
    json_report!("native_report_255", _context => native_transcript_json(None)),
    json_report!("native_report_256", _context => native_approvals_json()),
    json_report!("native_report_257", context => native_policy_json(context.options, &context.telegram_plugin)),
    json_report!("native_report_258", _context => native_events_json(NativeEventSurface::Events, None)),
    json_report!("native_report_259", _context => native_events_json(NativeEventSurface::EventsReport, None)),
    json_report!("native_report_260", _context => native_events_json(NativeEventSurface::Activity, None)),
    json_report!("native_report_261", _context => native_runtime_audit_json(NativeRuntimeAuditSurface::SubagentObservatory)),
    json_report!("native_report_262", _context => native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayLedger)),
    json_report!("native_report_263", _context => native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayRetryDeadLetter)),
    json_report!("native_report_264", _context => native_runtime_audit_json(NativeRuntimeAuditSurface::MultiAgentRuntime)),
    json_report!("native_report_265", context => native_config_json(context.options)),
    json_report!("native_report_266", _context => native_optional_configs_json()),
    json_report!("native_report_267", context => json_or_error(&context.telegram_plugin)),
    json_report!("native_report_268", context => json_or_error(&native_telegram::telegram_model_turn_plan_status(
        context.options.with_telegram_plugin,
    ))),
    json_report!("native_report_269", context => json_or_error(&native_telegram::telegram_model_bridge_status(
        context.options.with_telegram_plugin,
    ))),
    json_report!("native_report_270", context => json_or_error(&native_telegram::telegram_send_plan_status(
        context.options.with_telegram_plugin,
    ))),
    json_report!("native_report_271", context => json_or_error(&native_telegram::telegram_drain_once_status(
        context.options.with_telegram_plugin,
    ))),
    json_report!("native_report_272", context => json_or_error(&native_telegram::telegram_poll_loop_status(
        context.options.with_telegram_plugin,
        context.options.telegram_plugin_poll_ms,
    ))),
    json_report!("native_report_273", context => json_or_error(&native_telegram::telegram_live_soak_status(
        context.options.with_telegram_plugin,
        context.options.telegram_plugin_poll_ms,
    ))),
    json_report!("native_report_274", context => json_or_error(&native_telegram::telegram_production_readiness_status(
        context.options.with_telegram_plugin,
        context.options.telegram_plugin_poll_ms,
    ))),
    json_report!("native_report_275", context => json_or_error(&native_telegram::telegram_delivery_ledger_status(
        context.options.with_telegram_plugin,
    ))),
    json_report!("native_report_276", context => json_or_error(&telegram_owner_handoff_status(context.options))),
    json_report!("native_report_277", context => json_or_error(&native_telegram::telegram_cursor_status(
        context.options.with_telegram_plugin,
    ))),
];

pub(super) fn render_registered_native_report(
    report_id: crate::route_definition::NativeReportId,
    path: &str,
    options: &NativeGatewayOptions,
    telegram_plugin: NativeTelegramPluginStatus,
) -> Option<NativeReportResponse> {
    if crate::route_registry::native_report_id(path) != Some(report_id) {
        return None;
    }
    let definition = NATIVE_REPORT_DEFINITIONS.get(usize::from(report_id.0))?;
    if crate::route_registry::native_report_key(report_id) != Some(definition.key) {
        return None;
    }
    Some((definition.renderer)(NativeReportContext {
        options,
        telegram_plugin,
    }))
}

pub(super) fn render_registered_evidence_report(
    renderer_key: &str,
    options: &NativeGatewayOptions,
    telegram_plugin: NativeTelegramPluginStatus,
) -> Option<NativeReportResponse> {
    let definition = NATIVE_REPORT_DEFINITIONS
        .iter()
        .find(|definition| definition.key == renderer_key)?;
    Some((definition.renderer)(NativeReportContext {
        options,
        telegram_plugin,
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn native_report_registry_paths_are_unique_and_typed() {
        let paths = crate::route_registry::registered_native_report_paths().collect::<Vec<_>>();
        let unique = paths.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique.len(), paths.len());
        assert_eq!(paths.len(), 285);
        assert!(paths.iter().all(|path| path.starts_with('/')));
        assert!(
            paths
                .iter()
                .all(|path| crate::route_registry::native_report_id(path).is_some())
        );
        let report_ids = paths
            .iter()
            .filter_map(|path| crate::route_registry::native_report_id(path))
            .map(|report_id| report_id.0)
            .collect::<HashSet<_>>();
        assert_eq!(report_ids.len(), NATIVE_REPORT_DEFINITIONS.len());
        assert!(
            report_ids
                .iter()
                .all(|report_id| usize::from(*report_id) < NATIVE_REPORT_DEFINITIONS.len())
        );
        assert!(
            NATIVE_REPORT_DEFINITIONS
                .iter()
                .enumerate()
                .all(|(index, definition)| {
                    crate::route_registry::native_report_key(NativeReportId(index as u16))
                        == Some(definition.key)
                })
        );
    }
}
