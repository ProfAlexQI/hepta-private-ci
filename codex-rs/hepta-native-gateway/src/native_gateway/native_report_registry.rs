use super::*;

pub(super) type NativeReportResponse = (&'static str, &'static str, String);

type NativeReportRenderer = for<'a> fn(NativeReportContext<'a>) -> NativeReportResponse;

struct NativeReportContext<'a> {
    options: &'a NativeGatewayOptions,
    telegram_plugin: NativeTelegramPluginStatus,
}

#[derive(Clone, Copy)]
struct NativeReportDefinition {
    renderer: NativeReportRenderer,
}

const fn report(renderer: NativeReportRenderer) -> NativeReportDefinition {
    NativeReportDefinition { renderer }
}

const NATIVE_REPORT_DEFINITIONS: &[NativeReportDefinition] = &[
    report(|_context| {
        (
            "200 OK",
            "text/html; charset=utf-8",
            hepta_core::control_ui::control_ui_index_html(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "text/css; charset=utf-8",
            hepta_core::control_ui::CONTROL_UI_STYLES_CSS.to_string(),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "text/html; charset=utf-8",
            index_html(context.options, &context.telegram_plugin),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&HealthResponse {
                product: "Hepta",
                runtime: "hepta",
                status: "ready",
            }),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            watchdog_state::watchdog_state_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_gateway_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_control_ui_audit_json(
                NativeControlUiAuditSurface::ControlUi,
                context.options,
                &context.telegram_plugin,
            ),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_control_ui_audit_json(
                NativeControlUiAuditSurface::UiContractAudit,
                context.options,
                &context.telegram_plugin,
            ),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_control_ui_audit_json(
                NativeControlUiAuditSurface::GatewayDispatch,
                context.options,
                &context.telegram_plugin,
            ),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_control_ui_audit_json(
                NativeControlUiAuditSurface::UiActionPlanGatewayDispatch,
                context.options,
                &context.telegram_plugin,
            ),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_control_ui_audit_json(
                NativeControlUiAuditSurface::ExternalAgentBenchmark,
                context.options,
                &context.telegram_plugin,
            ),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&gateway_replacement_readiness(
                context.options,
                &context.telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&gateway_live_activation_plan(
                context.options,
                &context.telegram_plugin,
            )),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&control_ui_route_parity_report()),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_merge_completion_report(context.options)),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_cli_command_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_provider_metadata_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_runtime_session_dry_run_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_context_recall_worker_scheduler_handoff_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_channel_adapter_status_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_local_tooling_content_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_systems_tool_registry_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_systems_workflow_definition_registry_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_memory_capability_absorption_inventory_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_memory_intelligence_kg_full_enablement_runtime_readiness_report()),
        )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_readiness_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_separation_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_memory_intelligence_kg_activation_truth_index_report()),
        )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_acceptance_receipt_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_readback_audit_receipt_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_receipt_audit_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report(),
        ),
    )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_intelligence_bounded_context_attachment_preview_readback_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_kg_read_only_adapter_shadow_rank_canary_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_provider_router_dry_run_envelope_readback_audit_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(
                &hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report(),
            ),
        )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_full_live_activation_closure_index_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(
                &hepta_memory_live_mutation_operator_write_approval_packet_boundary_report(),
            ),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(
                &hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report(),
            ),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(
                &hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report(
                ),
            ),
        )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_report(
            ),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_preflight_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_report(),
        ),
    )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_upstream_codex_latest_multisurface_absorption_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_first_model_invocation_separate_approval_slice_preflight_report()),
        )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
        "200 OK",
        "application/json; charset=utf-8",
        json_or_error(
            &hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_report(),
        ),
    )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_first_model_positive_approval_packet_boundary_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_scoped_memory_canary_durable_receipt_boundary_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_release_hardening_status_gate_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_provider_channel_dry_run_plan_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_native_packaging_gate_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_legacy_compatibility_closure_report()),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_public_ga_operator_approval_packet_report(
                context.options,
                &context.telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_public_ga_readiness_report(
                context.options,
                &context.telegram_plugin,
            )),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_gateway::hepta_core_fusion_readiness_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_gateway::hepta_name_repository_closure_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_gateway::hepta_engine_dependency_closure_report()),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&hepta_gateway::hepta_codex_engine_adapter_boundary_report()),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            operator_snapshot_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            operator_console_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            operator_security_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_post_execution_readiness_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_post_execution_stores_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_post_activation_plan_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_post_rollout_evidence_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_post_gray_release_evidence_json(),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_sessions_json("/sessions --json", "native_sessions_inventory"),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_sessions_json("/session-activity --json", "native_session_activity"),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_transcript_json(None),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_approvals_json(),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_policy_json(context.options, &context.telegram_plugin),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_events_json(NativeEventSurface::Events, None),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_events_json(NativeEventSurface::EventsReport, None),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_events_json(NativeEventSurface::Activity, None),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_runtime_audit_json(NativeRuntimeAuditSurface::SubagentObservatory),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayLedger),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_runtime_audit_json(NativeRuntimeAuditSurface::GatewayRetryDeadLetter),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_runtime_audit_json(NativeRuntimeAuditSurface::MultiAgentRuntime),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_config_json(context.options),
        )
    }),
    report(|_context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            native_optional_configs_json(),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&context.telegram_plugin),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_model_turn_plan_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_model_bridge_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_send_plan_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_drain_once_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_poll_loop_status(
                context.options.with_telegram_plugin,
                context.options.telegram_plugin_poll_ms,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_live_soak_status(
                context.options.with_telegram_plugin,
                context.options.telegram_plugin_poll_ms,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_production_readiness_status(
                context.options.with_telegram_plugin,
                context.options.telegram_plugin_poll_ms,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_delivery_ledger_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&telegram_owner_handoff_status(context.options)),
        )
    }),
    report(|context| {
        (
            "200 OK",
            "application/json; charset=utf-8",
            json_or_error(&native_telegram::telegram_cursor_status(
                context.options.with_telegram_plugin,
            )),
        )
    }),
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
    }
}
