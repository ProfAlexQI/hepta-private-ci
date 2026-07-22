    #[test]
    fn hepta_first_model_invocation_separate_approval_slice_preflight_endpoint_requires_approval_without_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body)
            .expect("first model invocation separate approval slice preflight route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-separate-approval-slice-preflight --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_provider_router_dry_run_envelope_readback_audit_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_separate_approval_preflight_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_separate_approval_slice_preflight_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_separate_approval_slice_preflight_ready"],
            true
        );
        assert_eq!(
            value["approval_state"],
            "requires_fresh_operator_approval_and_explicit_command"
        );
        assert_eq!(value["fresh_operator_approval_required"], true);
        assert_eq!(value["explicit_command_required"], true);
        assert_eq!(value["single_use_approval_nonce_required"], true);
        assert_eq!(value["operator_identity_session_binding_required"], true);
        assert_eq!(value["approval_packet_preview_constructed"], true);
        assert_eq!(value["approval_packet_preview_redacted"], true);
        assert_eq!(value["approval_packet_readback_audit_performed"], true);
        assert_eq!(value["approval_packet_readback_hash_matched"], true);
        assert_eq!(value["approval_packet_receipt_rendered"], true);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["approval_packet_persisted"], false);
        assert_eq!(value["approval_packet_ledger_recorded"], false);
        assert_eq!(value["approval_packet_filesystem_written"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation preflight audit steps");
        assert_eq!(steps.len(), 4);
        assert_eq!(
            steps[0]["step"],
            "provider_router_dry_run_envelope_source_binding"
        );
        assert_eq!(steps[1]["step"], "approval_packet_preview_and_readback");
        assert_eq!(steps[2]["step"], "fresh_operator_approval_boundary");
        assert_eq!(steps[3]["step"], "invocation_side_effect_denial_check");
        assert_eq!(steps[2]["fresh_operator_approval_required"], true);
        assert_eq!(steps[2]["approval_packet_accepted"], false);
        assert_eq!(steps[3]["provider_invocation_authorized"], false);
        assert_eq!(steps[3]["provider_invoked"], false);
        assert_eq!(steps[3]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation preflight side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_packet_review"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["requires_fresh_operator_approval"],
            true
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_endpoint_blocks_implicit_acceptance_without_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body)
            .expect("first model invocation approval packet review acceptance denial route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-packet-review-acceptance-denial-preflight --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_preflight_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_packet_review_acceptance_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_ready"],
            true
        );
        assert_eq!(
            value["approval_state"],
            "review_surface_rendered_acceptance_denied_until_fresh_artifact_nonce_session_and_explicit_command"
        );
        assert_eq!(value["review_surface_rendered"], true);
        assert_eq!(value["review_surface_redacted"], true);
        assert_eq!(value["review_surface_readback_performed"], true);
        assert_eq!(value["review_surface_readback_hash_matched"], true);
        assert_eq!(value["review_surface_persisted"], false);
        assert_eq!(value["operator_review_recorded"], false);
        assert_eq!(value["operator_review_persisted"], false);
        assert_eq!(value["approval_acceptance_candidate_present"], true);
        assert_eq!(value["approval_acceptance_preconditions_satisfied"], false);
        assert_eq!(value["approval_acceptance_denied"], true);
        assert_eq!(value["approval_packet_review_accepted"], false);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["approval_packet_persisted"], false);
        assert_eq!(value["approval_packet_ledger_recorded"], false);
        assert_eq!(value["approval_packet_filesystem_written"], false);
        assert_eq!(value["approval_acceptance_receipt_rendered"], true);
        assert_eq!(value["approval_acceptance_receipt_persisted"], false);
        assert_eq!(value["approval_acceptance_ledger_recorded"], false);
        assert_eq!(value["approval_acceptance_filesystem_written"], false);
        assert_eq!(
            value["fresh_accepted_operator_approval_artifact_required"],
            true
        );
        assert_eq!(
            value["fresh_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(value["fresh_operator_approval_required"], true);
        assert_eq!(value["explicit_command_required"], true);
        assert_eq!(value["explicit_invocation_command_required"], true);
        assert_eq!(value["explicit_invocation_command_present"], false);
        assert_eq!(value["single_use_approval_nonce_required"], true);
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["operator_identity_session_binding_required"], true);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation approval review audit steps");
        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0]["step"], "approval_preflight_source_binding");
        assert_eq!(
            steps[1]["step"],
            "operator_approval_packet_review_surface_readback"
        );
        assert_eq!(steps[2]["step"], "approval_acceptance_denial_boundary");
        assert_eq!(
            steps[3]["step"],
            "fresh_artifact_nonce_session_command_preconditions"
        );
        assert_eq!(steps[4]["step"], "invocation_side_effect_denial_check");
        assert_eq!(steps[1]["operator_review_recorded"], false);
        assert_eq!(steps[2]["approval_acceptance_denied"], true);
        assert_eq!(steps[2]["approval_packet_accepted"], false);
        assert_eq!(
            steps[3]["fresh_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(steps[3]["explicit_invocation_command_present"], false);
        assert_eq!(steps[4]["provider_invocation_authorized"], false);
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation approval review side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_acceptance_artifact_precondition"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["requires_fresh_accepted_operator_approval_artifact"],
            true
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_endpoint_blocks_missing_artifact_without_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body)
            .expect("first model invocation approval acceptance artifact precondition route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-acceptance-artifact-precondition --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_review_acceptance_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_acceptance_artifact_precondition_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_acceptance_artifact_precondition_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_acceptance_artifact_precondition_ready"],
            true
        );
        assert_eq!(
            value["approval_state"],
            "accepted_artifact_precondition_rendered_but_missing_artifact_nonce_session_and_explicit_command"
        );
        assert_eq!(
            value["accepted_operator_approval_artifact_precondition_rendered"],
            true
        );
        assert_eq!(
            value["accepted_operator_approval_artifact_precondition_readback_performed"],
            true
        );
        assert_eq!(
            value["accepted_operator_approval_artifact_precondition_readback_hash_matched"],
            true
        );
        assert_eq!(value["accepted_operator_approval_artifact_recorded"], false);
        assert_eq!(
            value["accepted_operator_approval_artifact_persisted"],
            false
        );
        assert_eq!(
            value["accepted_operator_approval_artifact_filesystem_read"],
            false
        );
        assert_eq!(
            value["fresh_accepted_operator_approval_artifact_required"],
            true
        );
        assert_eq!(
            value["fresh_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(
            value["fresh_accepted_operator_approval_artifact_verified"],
            false
        );
        assert_eq!(
            value["accepted_operator_approval_artifact_hash_matched"],
            false
        );
        assert_eq!(value["approval_artifact_freshness_window_required"], true);
        assert_eq!(value["approval_artifact_freshness_window_satisfied"], false);
        assert_eq!(value["approval_artifact_replay_denied"], true);
        assert_eq!(value["approval_artifact_supersession_denied"], true);
        assert_eq!(value["stale_operator_approval_artifact_accepted"], false);
        assert_eq!(
            value["superseded_operator_approval_artifact_accepted"],
            false
        );
        assert_eq!(value["approval_artifact_reinstatement_accepted"], false);
        assert_eq!(value["single_use_approval_nonce_required"], true);
        assert_eq!(value["single_use_approval_nonce_present"], false);
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["operator_identity_session_binding_required"], true);
        assert_eq!(value["operator_identity_session_binding_present"], false);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(value["explicit_command_required"], true);
        assert_eq!(value["explicit_invocation_command_required"], true);
        assert_eq!(value["explicit_invocation_command_present"], false);
        assert_eq!(value["explicit_invocation_command_accepted"], false);
        assert_eq!(value["approval_acceptance_candidate_present"], true);
        assert_eq!(value["approval_acceptance_preconditions_satisfied"], false);
        assert_eq!(value["approval_acceptance_denied"], true);
        assert_eq!(value["approval_packet_review_accepted"], false);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["approval_packet_persisted"], false);
        assert_eq!(value["approval_packet_ledger_recorded"], false);
        assert_eq!(value["approval_packet_filesystem_written"], false);
        assert_eq!(value["approval_precondition_receipt_rendered"], true);
        assert_eq!(value["approval_precondition_receipt_persisted"], false);
        assert_eq!(value["operator_approval_recorded"], false);
        assert_eq!(value["operator_consent_recorded"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation approval artifact precondition audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "approval_review_source_binding");
        assert_eq!(
            steps[1]["step"],
            "accepted_artifact_presence_freshness_precondition"
        );
        assert_eq!(
            steps[2]["step"],
            "nonce_session_explicit_command_preconditions"
        );
        assert_eq!(
            steps[3]["step"],
            "approval_artifact_replay_supersession_denial"
        );
        assert_eq!(steps[4]["step"], "acceptance_authorization_denial");
        assert_eq!(steps[5]["step"], "invocation_side_effect_denial_check");
        assert_eq!(
            steps[1]["fresh_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(steps[2]["single_use_approval_nonce_present"], false);
        assert_eq!(steps[2]["operator_identity_session_binding_present"], false);
        assert_eq!(steps[2]["explicit_invocation_command_present"], false);
        assert_eq!(steps[3]["approval_artifact_replay_denied"], true);
        assert_eq!(steps[4]["approval_acceptance_denied"], true);
        assert_eq!(steps[5]["provider_invoked"], false);
        assert_eq!(steps[5]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation approval artifact precondition side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_nonce_session_command_binding_preflight"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["requires_fresh_accepted_operator_approval_artifact"],
            true
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_endpoint_blocks_unbound_nonce_session_command_without_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body)
            .expect("first model invocation approval nonce session command binding route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-nonce-session-command-binding-preflight --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_acceptance_artifact_precondition_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_nonce_session_command_binding_preflight_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_nonce_session_command_binding_preflight_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_nonce_session_command_binding_preflight_ready"],
            true
        );
        assert_eq!(
            value["approval_state"],
            "synthetic_accepted_artifact_fixture_rendered_but_nonce_session_command_not_bound"
        );
        assert_eq!(
            value["synthetic_accepted_operator_approval_artifact_fixture_rendered"],
            true
        );
        assert_eq!(
            value["synthetic_accepted_operator_approval_artifact_fixture_readback_performed"],
            true
        );
        assert_eq!(
            value["synthetic_accepted_operator_approval_artifact_fixture_hash_matched"],
            true
        );
        assert_eq!(
            value["synthetic_accepted_operator_approval_artifact_fixture_persisted"],
            false
        );
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_verified"],
            false
        );
        assert_eq!(value["single_use_approval_nonce_required"], true);
        assert_eq!(value["single_use_approval_nonce_fixture_rendered"], true);
        assert_eq!(value["single_use_approval_nonce_present"], false);
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["single_use_approval_nonce_replay_denied"], true);
        assert_eq!(value["operator_identity_session_binding_required"], true);
        assert_eq!(
            value["operator_identity_session_binding_fixture_rendered"],
            true
        );
        assert_eq!(value["operator_identity_session_binding_present"], false);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(
            value["operator_identity_session_cross_binding_denied"],
            true
        );
        assert_eq!(value["explicit_invocation_command_required"], true);
        assert_eq!(value["explicit_invocation_command_fixture_rendered"], true);
        assert_eq!(value["explicit_invocation_command_present"], false);
        assert_eq!(value["explicit_invocation_command_accepted"], false);
        assert_eq!(value["explicit_invocation_command_replay_denied"], true);
        assert_eq!(
            value["nonce_session_command_binding_candidate_present"],
            true
        );
        assert_eq!(
            value["nonce_session_command_binding_preconditions_satisfied"],
            false
        );
        assert_eq!(value["nonce_session_command_binding_denied"], true);
        assert_eq!(
            value["nonce_session_command_binding_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["nonce_session_command_binding_denial_receipt_rendered"],
            true
        );
        assert_eq!(
            value["nonce_session_command_binding_denial_receipt_persisted"],
            false
        );
        assert_eq!(value["approval_acceptance_denied"], true);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["approval_final_authorization_denied"], true);
        assert_eq!(value["operator_approval_recorded"], false);
        assert_eq!(value["operator_consent_recorded"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation approval nonce/session/command audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "approval_artifact_precondition_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "synthetic_accepted_artifact_fixture_isolation"
        );
        assert_eq!(steps[2]["step"], "single_use_nonce_binding_preflight");
        assert_eq!(
            steps[3]["step"],
            "operator_session_and_explicit_command_binding_preflight"
        );
        assert_eq!(steps[4]["step"], "replay_cross_binding_denial");
        assert_eq!(steps[5]["step"], "invocation_side_effect_denial_check");
        assert_eq!(steps[2]["single_use_approval_nonce_present"], false);
        assert_eq!(steps[3]["operator_identity_session_binding_present"], false);
        assert_eq!(steps[3]["explicit_invocation_command_present"], false);
        assert_eq!(
            steps[4]["operator_identity_session_cross_binding_denied"],
            true
        );
        assert_eq!(steps[5]["provider_invoked"], false);
        assert_eq!(steps[5]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation approval nonce/session/command side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight"
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_endpoint_blocks_authorization_without_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run envelope route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-envelope-preflight --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_nonce_session_command_binding_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_ready"],
            true
        );
        assert_eq!(
            value["authorization_state"],
            "final_authorization_dry_run_envelope_rendered_but_real_preconditions_missing"
        );
        assert_eq!(value["final_authorization_dry_run_envelope_rendered"], true);
        assert_eq!(
            value["final_authorization_dry_run_envelope_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_envelope_persisted"],
            false
        );
        assert_eq!(value["final_authorization_live_envelope_executed"], false);
        assert_eq!(
            value["final_authorization_dry_run_readback_performed"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_readback_hash_matched"],
            true
        );
        assert_eq!(value["final_authorization_denial_receipt_rendered"], true);
        assert_eq!(value["final_authorization_denial_receipt_persisted"], false);
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_verified"],
            false
        );
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(value["explicit_invocation_command_accepted"], false);
        assert_eq!(value["final_authorization_candidate_present"], true);
        assert_eq!(value["final_authorization_preconditions_satisfied"], false);
        assert_eq!(value["final_authorization_denied"], true);
        assert_eq!(value["final_authorization_accepted"], false);
        assert_eq!(value["final_authorization_persisted"], false);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["operator_approval_recorded"], false);
        assert_eq!(value["operator_consent_recorded"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation final authorization dry-run audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "nonce_session_command_source_binding");
        assert_eq!(
            steps[1]["step"],
            "final_authorization_dry_run_envelope_construction"
        );
        assert_eq!(steps[2]["step"], "real_precondition_denial");
        assert_eq!(steps[3]["step"], "provider_model_budget_binding");
        assert_eq!(steps[4]["step"], "dry_run_receipt_non_persistence");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(steps[2]["final_authorization_denied"], true);
        assert_eq!(steps[3]["provider_invocation_budget"], 0);
        assert_eq!(steps[3]["model_invocation_budget"], 0);
        assert_eq!(steps[3]["provider_invoked"], false);
        assert_eq!(steps[3]["model_invoked"], false);
        assert_eq!(
            steps[4]["final_authorization_denial_receipt_persisted"],
            false
        );

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation final authorization dry-run side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence"
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_endpoint_blocks_receipt_persistence_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_envelope_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_state"],
            "final_authorization_dry_run_result_receipt_rendered_but_not_persisted_or_accepted"
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_rendered"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_readback_performed"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_filesystem_written"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ledger_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_delivered"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_exported"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_query_registered"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_observability_recorded"],
            false
        );
        assert_eq!(value["completion_ack_recorded"], false);
        assert_eq!(value["completion_ack_accepted"], false);
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(value["explicit_invocation_command_accepted"], false);
        assert_eq!(value["final_authorization_candidate_present"], true);
        assert_eq!(value["final_authorization_preconditions_satisfied"], false);
        assert_eq!(value["final_authorization_denied"], true);
        assert_eq!(value["final_authorization_accepted"], false);
        assert_eq!(
            value["final_authorization_from_result_receipt_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_result_receipt_accepted"],
            false
        );
        assert_eq!(value["activation_from_result_receipt_allowed"], false);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["operator_approval_recorded"], false);
        assert_eq!(value["operator_consent_recorded"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation final authorization result receipt audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "final_authorization_dry_run_source_binding"
        );
        assert_eq!(steps[1]["step"], "dry_run_result_receipt_shape_rendering");
        assert_eq!(steps[2]["step"], "result_receipt_readback_no_persistence");
        assert_eq!(steps[3]["step"], "receipt_authority_non_promotion");
        assert_eq!(steps[4]["step"], "delivery_export_observability_denial");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["final_authorization_dry_run_result_receipt_persisted"],
            false
        );
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_readback_hash_matched"],
            true
        );
        assert_eq!(
            steps[3]["final_authorization_from_result_receipt_allowed"],
            false
        );
        assert_eq!(steps[3]["provider_invoked"], false);
        assert_eq!(steps[3]["model_invoked"], false);
        assert_eq!(
            steps[4]["final_authorization_dry_run_result_receipt_delivered"],
            false
        );

        let side_effects = value["side_effects"]
            .as_object()
            .expect("first model invocation final authorization result receipt side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial"
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["persists_result_receipt"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["accepts_result_receipt"],
            false
        );
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_endpoint_blocks_replay_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt replay/idempotency route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_no_persistence_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_replay_idempotency_state"],
            "final_authorization_dry_run_result_receipt_replay_duplicate_retry_idempotency_denied"
        );
        assert_eq!(value["replay_idempotency_fixture_count"], 8);
        assert_eq!(value["blocked_replay_idempotency_fixture_count"], 8);
        assert_eq!(value["noop_replay_idempotency_fixture_count"], 8);
        assert_eq!(value["allowed_replay_idempotency_fixture_count"], 0);
        assert_eq!(value["accepted_replay_idempotency_fixture_count"], 0);
        assert_eq!(value["replay_idempotency_performed_count"], 0);
        assert_eq!(value["duplicate_result_receipt_accepted_count"], 0);
        assert_eq!(value["retry_result_receipt_accepted_count"], 0);
        assert_eq!(value["idempotency_state_recorded_count"], 0);
        assert_eq!(value["idempotency_state_persisted_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_allowed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replayed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_performed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_duplicate_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_retry_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_key_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_key_registered"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_state_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_state_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_cache_written"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replay_nonce_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_status_upgrade_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_completed_status_accepted"],
            false
        );
        assert_eq!(value["completion_ack_replay_accepted"], false);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ledger_replay_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_index_replay_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_delivery_replay_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_export_replay_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_query_replay_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_observability_replay_accepted"],
            false
        );
        assert_eq!(value["final_authorization_from_replay_allowed"], false);
        assert_eq!(value["operator_approval_from_replay_accepted"], false);
        assert_eq!(value["activation_from_replay_allowed"], false);
        assert_eq!(
            value["final_authorization_from_result_receipt_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_result_receipt_accepted"],
            false
        );
        assert_eq!(value["activation_from_result_receipt_allowed"], false);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_accepted"],
            false
        );
        assert_eq!(value["completion_ack_recorded"], false);
        assert_eq!(value["completion_ack_accepted"], false);
        assert_eq!(
            value["fresh_live_accepted_operator_approval_artifact_present"],
            false
        );
        assert_eq!(value["single_use_approval_nonce_verified"], false);
        assert_eq!(value["single_use_approval_nonce_consumed"], false);
        assert_eq!(value["operator_identity_session_binding_verified"], false);
        assert_eq!(value["operator_identity_session_bound"], false);
        assert_eq!(value["explicit_invocation_command_accepted"], false);
        assert_eq!(value["final_authorization_candidate_present"], true);
        assert_eq!(value["final_authorization_preconditions_satisfied"], false);
        assert_eq!(value["final_authorization_denied"], true);
        assert_eq!(value["final_authorization_accepted"], false);
        assert_eq!(value["approval_packet_accepted"], false);
        assert_eq!(value["operator_approval_recorded"], false);
        assert_eq!(value["operator_consent_recorded"], false);
        assert_eq!(value["candidate_provider_invocation_requested"], true);
        assert_eq!(value["candidate_model_invocation_requested"], true);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_authorized_from_replay"], false);
        assert_eq!(value["model_invocation_authorized_from_replay"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_value_read"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["provider_router_live_envelope_executed"], false);
        assert_eq!(value["provider_prompt_injection_performed"], false);
        assert_eq!(value["context_injection_performed"], false);
        assert_eq!(value["kg_adapter_read_performed"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["replay_idempotency_fixtures"]
            .as_array()
            .expect("first model invocation result receipt replay fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["replay_idempotency_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["activation_from_replay_allowed"] == false
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"]
            .as_array()
            .expect("first model invocation final authorization result receipt replay audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "result_receipt_no_persistence_source_binding"
        );
        assert_eq!(steps[1]["step"], "replay_duplicate_retry_fixture_denial");
        assert_eq!(steps[2]["step"], "idempotency_state_no_write");
        assert_eq!(steps[3]["step"], "cross_scope_status_ack_replay_denial");
        assert_eq!(steps[4]["step"], "replay_authority_non_promotion");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(steps[1]["blocked_replay_idempotency_fixture_count"], 8);
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_idempotency_cache_written"],
            false
        );
        assert_eq!(
            steps[3]["final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted"],
            false
        );
        assert_eq!(steps[4]["activation_from_replay_allowed"], false);
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt replay side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["registers_idempotency_key"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["writes_idempotency_cache"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_endpoint_blocks_ordering_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt ordering/monotonicity route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-ordering-monotonicity-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_ordering_monotonicity_state"],
            "final_authorization_dry_run_result_receipt_sequence_cursor_monotonicity_denied"
        );
        assert_eq!(value["ordering_monotonicity_fixture_count"], 8);
        assert_eq!(value["blocked_ordering_monotonicity_fixture_count"], 8);
        assert_eq!(value["noop_ordering_monotonicity_fixture_count"], 8);
        assert_eq!(value["allowed_ordering_monotonicity_fixture_count"], 0);
        assert_eq!(value["accepted_ordering_monotonicity_fixture_count"], 0);
        assert_eq!(value["ordering_monotonicity_performed_count"], 0);
        assert_eq!(value["sequence_cursor_recorded_count"], 0);
        assert_eq!(value["sequence_cursor_persisted_count"], 0);
        assert_eq!(value["monotonicity_state_recorded_count"], 0);
        assert_eq!(value["monotonicity_state_persisted_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ordering_allowed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ordered"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ordering_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ordering_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_sequence_cursor_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_sequence_cursor_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_monotonicity_state_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_monotonicity_state_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_duplicate_sequence_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_stale_sequence_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_late_sequence_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_future_sequence_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_timestamp_rollback_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_epoch_rollback_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_same_sequence_override_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_latest_wins_promoted"],
            false
        );
        assert_eq!(value["completion_ack_ordering_accepted"], false);
        assert_eq!(value["final_authorization_from_ordering_allowed"], false);
        assert_eq!(value["operator_approval_from_ordering_accepted"], false);
        assert_eq!(value["activation_from_ordering_allowed"], false);
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(value["provider_invocation_authorized_from_ordering"], false);
        assert_eq!(value["model_invocation_authorized_from_ordering"], false);
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["ordering_monotonicity_fixtures"]
            .as_array()
            .expect("first model invocation result receipt ordering fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["ordering_monotonicity_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["activation_from_ordering_allowed"] == false
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"].as_array().expect(
            "first model invocation final authorization result receipt ordering audit steps",
        );
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "replay_idempotency_source_binding");
        assert_eq!(steps[1]["step"], "ordering_sequence_fixture_denial");
        assert_eq!(steps[2]["step"], "sequence_cursor_no_write");
        assert_eq!(steps[3]["step"], "late_future_rollback_denial");
        assert_eq!(steps[4]["step"], "ordering_authority_non_promotion");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(steps[1]["blocked_ordering_monotonicity_fixture_count"], 8);
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_sequence_cursor_recorded"],
            false
        );
        assert_eq!(
            steps[3]["final_authorization_dry_run_result_receipt_epoch_rollback_accepted"],
            false
        );
        assert_eq!(steps[4]["activation_from_ordering_allowed"], false);
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt ordering side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_sequence_cursor"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["persists_monotonicity_state"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_endpoint_blocks_cancellation_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt cancellation/supersession route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-cancellation-supersession-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_cancellation_supersession_state"],
            "final_authorization_dry_run_result_receipt_cancellation_supersession_replacement_denied"
        );
        assert_eq!(value["cancellation_supersession_fixture_count"], 8);
        assert_eq!(value["blocked_cancellation_supersession_fixture_count"], 8);
        assert_eq!(value["noop_cancellation_supersession_fixture_count"], 8);
        assert_eq!(value["allowed_cancellation_supersession_fixture_count"], 0);
        assert_eq!(value["accepted_cancellation_supersession_fixture_count"], 0);
        assert_eq!(value["cancellation_supersession_performed_count"], 0);
        assert_eq!(value["cancellation_recorded_count"], 0);
        assert_eq!(value["supersession_recorded_count"], 0);
        assert_eq!(value["replacement_receipt_recorded_count"], 0);
        assert_eq!(value["tombstone_recorded_count"], 0);
        assert_eq!(value["delete_marker_recorded_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cancellation_allowed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cancellation_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cancellation_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_cancellation_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_supersession_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_supersession_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replacement_accepted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_replacement_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_tombstone_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_delete_marker_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_latest_replacement_promoted"],
            false
        );
        assert_eq!(value["completion_ack_cancellation_accepted"], false);
        assert_eq!(value["completion_ack_replacement_accepted"], false);
        assert_eq!(value["result_receipt_cancelled_query_registered"], false);
        assert_eq!(value["result_receipt_superseded_export_recorded"], false);
        assert_eq!(
            value["result_receipt_replacement_observability_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_from_cancellation_supersession_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_cancellation_supersession_accepted"],
            false
        );
        assert_eq!(
            value["activation_from_cancellation_supersession_allowed"],
            false
        );
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(
            value["provider_invocation_authorized_from_cancellation_supersession"],
            false
        );
        assert_eq!(
            value["model_invocation_authorized_from_cancellation_supersession"],
            false
        );
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["cancellation_supersession_fixtures"]
            .as_array()
            .expect("first model invocation result receipt cancellation fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["cancellation_supersession_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"].as_array().expect(
            "first model invocation final authorization result receipt cancellation audit steps",
        );
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "ordering_monotonicity_source_binding");
        assert_eq!(steps[1]["step"], "cancellation_supersession_fixture_denial");
        assert_eq!(steps[2]["step"], "replacement_lifecycle_no_write");
        assert_eq!(
            steps[3]["step"],
            "replacement_query_export_observability_denial"
        );
        assert_eq!(
            steps[4]["step"],
            "cancellation_supersession_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_cancellation_supersession_fixture_count"],
            8
        );
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_cancellation_recorded"],
            false
        );
        assert_eq!(steps[3]["result_receipt_superseded_export_recorded"], false);
        assert_eq!(
            steps[4]["activation_from_cancellation_supersession_allowed"],
            false
        );
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt cancellation side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_cancellation"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_supersession"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_replacement"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_endpoint_blocks_audit_evidence_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt audit/immutable evidence route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-audit-immutable-evidence-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_audit_immutable_evidence_state"],
            "final_authorization_dry_run_result_receipt_audit_immutable_evidence_denied"
        );
        assert_eq!(value["audit_immutable_evidence_fixture_count"], 8);
        assert_eq!(value["blocked_audit_immutable_evidence_fixture_count"], 8);
        assert_eq!(value["noop_audit_immutable_evidence_fixture_count"], 8);
        assert_eq!(value["allowed_audit_immutable_evidence_fixture_count"], 0);
        assert_eq!(value["accepted_audit_immutable_evidence_fixture_count"], 0);
        assert_eq!(value["audit_immutable_evidence_performed_count"], 0);
        assert_eq!(value["audit_recorded_count"], 0);
        assert_eq!(value["ledger_written_count"], 0);
        assert_eq!(value["hash_chain_appended_count"], 0);
        assert_eq!(value["immutable_evidence_materialized_count"], 0);
        assert_eq!(value["attestation_signed_count"], 0);
        assert_eq!(value["witness_notarized_count"], 0);
        assert_eq!(value["merkle_root_published_count"], 0);
        assert_eq!(value["evidence_export_recorded_count"], 0);
        assert_eq!(value["external_evidence_sent_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_audit_allowed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_audit_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ledger_written"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_hash_chain_appended"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_immutable_evidence_materialized"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_attestation_signed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_witness_notarized"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_merkle_root_published"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_evidence_export_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_external_evidence_sent"],
            false
        );
        assert_eq!(value["result_receipt_audit_query_registered"], false);
        assert_eq!(
            value["final_authorization_from_audit_immutable_evidence_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_audit_immutable_evidence_accepted"],
            false
        );
        assert_eq!(
            value["activation_from_audit_immutable_evidence_allowed"],
            false
        );
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(
            value["provider_invocation_authorized_from_audit_immutable_evidence"],
            false
        );
        assert_eq!(
            value["model_invocation_authorized_from_audit_immutable_evidence"],
            false
        );
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["audit_immutable_evidence_fixtures"]
            .as_array()
            .expect("first model invocation result receipt audit immutable evidence fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["audit_immutable_evidence_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"].as_array().expect(
            "first model invocation final authorization result receipt audit immutable evidence steps",
        );
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "cancellation_supersession_source_binding");
        assert_eq!(steps[1]["step"], "audit_immutable_evidence_fixture_denial");
        assert_eq!(steps[2]["step"], "ledger_hash_chain_no_write");
        assert_eq!(steps[3]["step"], "attestation_witness_public_proof_denial");
        assert_eq!(
            steps[4]["step"],
            "audit_immutable_evidence_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_audit_immutable_evidence_fixture_count"],
            8
        );
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_ledger_written"],
            false
        );
        assert_eq!(
            steps[3]["final_authorization_dry_run_result_receipt_attestation_signed"],
            false
        );
        assert_eq!(
            steps[4]["activation_from_audit_immutable_evidence_allowed"],
            false
        );
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt audit immutable evidence side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial"
        );
        assert_eq!(value["allowed_next_actions"][0]["records_audit"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["records_immutable_evidence"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["persists_ledger"], false);
        assert_eq!(value["allowed_next_actions"][0]["exports_evidence"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_endpoint_blocks_retention_gc_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt retention/expiry/gc route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_retention_expiry_garbage_collection_state"],
            "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denied"
        );
        assert_eq!(
            value["retention_expiry_garbage_collection_fixture_count"],
            8
        );
        assert_eq!(
            value["blocked_retention_expiry_garbage_collection_fixture_count"],
            8
        );
        assert_eq!(
            value["noop_retention_expiry_garbage_collection_fixture_count"],
            8
        );
        assert_eq!(
            value["allowed_retention_expiry_garbage_collection_fixture_count"],
            0
        );
        assert_eq!(
            value["accepted_retention_expiry_garbage_collection_fixture_count"],
            0
        );
        assert_eq!(
            value["retention_expiry_garbage_collection_performed_count"],
            0
        );
        assert_eq!(value["retention_recorded_count"], 0);
        assert_eq!(value["retention_policy_persisted_count"], 0);
        assert_eq!(value["ttl_scheduled_count"], 0);
        assert_eq!(value["expiry_applied_count"], 0);
        assert_eq!(value["garbage_collection_scan_performed_count"], 0);
        assert_eq!(value["garbage_collection_candidate_recorded_count"], 0);
        assert_eq!(value["delete_marker_recorded_count"], 0);
        assert_eq!(value["archive_recorded_count"], 0);
        assert_eq!(value["compaction_performed_count"], 0);
        assert_eq!(value["retention_export_recorded_count"], 0);
        assert_eq!(value["external_retention_notification_sent_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_retention_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_retention_policy_persisted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_ttl_scheduled"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_expiry_applied"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_garbage_collection_scan_performed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_delete_marker_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_archive_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_compaction_performed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_external_retention_notification_sent"],
            false
        );
        assert_eq!(
            value["final_authorization_from_retention_expiry_garbage_collection_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_retention_expiry_garbage_collection_accepted"],
            false
        );
        assert_eq!(
            value["activation_from_retention_expiry_garbage_collection_allowed"],
            false
        );
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(
            value["provider_invocation_authorized_from_retention_expiry_garbage_collection"],
            false
        );
        assert_eq!(
            value["model_invocation_authorized_from_retention_expiry_garbage_collection"],
            false
        );
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["retention_expiry_garbage_collection_fixtures"]
            .as_array()
            .expect("first model invocation result receipt retention expiry gc fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["retention_expiry_garbage_collection_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"].as_array().expect(
            "first model invocation final authorization result receipt retention expiry gc steps",
        );
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0]["step"], "audit_immutable_evidence_source_binding");
        assert_eq!(
            steps[1]["step"],
            "retention_expiry_garbage_collection_fixture_denial"
        );
        assert_eq!(steps[2]["step"], "ttl_expiry_gc_no_schedule_or_scan");
        assert_eq!(steps[3]["step"], "archive_compaction_delete_marker_denial");
        assert_eq!(
            steps[4]["step"],
            "retention_expiry_garbage_collection_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_retention_expiry_garbage_collection_fixture_count"],
            8
        );
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_ttl_scheduled"],
            false
        );
        assert_eq!(
            steps[3]["final_authorization_dry_run_result_receipt_compaction_performed"],
            false
        );
        assert_eq!(
            steps[4]["activation_from_retention_expiry_garbage_collection_allowed"],
            false
        );
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt retention expiry gc side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial"
        );
        assert_eq!(value["allowed_next_actions"][0]["records_retention"], false);
        assert_eq!(value["allowed_next_actions"][0]["records_expiry"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["records_garbage_collection"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["exports_receipt"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_endpoint_blocks_views_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation approval final authorization dry-run result receipt export/query/observability route json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], "ready");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-export-query-observability-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(
            value["route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(
            value["implemented_route_count"],
            serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_no_provider_model_invocation"
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready"],
            true
        );
        assert_eq!(
            value["result_receipt_export_query_observability_state"],
            "final_authorization_dry_run_result_receipt_export_query_observability_denied"
        );
        assert_eq!(value["export_query_observability_fixture_count"], 8);
        assert_eq!(value["blocked_export_query_observability_fixture_count"], 8);
        assert_eq!(value["noop_export_query_observability_fixture_count"], 8);
        assert_eq!(value["allowed_export_query_observability_fixture_count"], 0);
        assert_eq!(
            value["accepted_export_query_observability_fixture_count"],
            0
        );
        assert_eq!(value["export_query_observability_performed_count"], 0);
        assert_eq!(value["export_materialized_count"], 0);
        assert_eq!(value["export_filesystem_written_count"], 0);
        assert_eq!(value["query_registered_count"], 0);
        assert_eq!(value["query_executed_count"], 0);
        assert_eq!(value["metric_recorded_count"], 0);
        assert_eq!(value["dashboard_published_count"], 0);
        assert_eq!(value["log_recorded_count"], 0);
        assert_eq!(value["trace_recorded_count"], 0);
        assert_eq!(value["alert_emitted_count"], 0);
        assert_eq!(value["slo_recorded_count"], 0);
        assert_eq!(value["external_observability_sent_count"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_matched"],
            true
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_export_materialized"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_export_filesystem_written"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_query_registered"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_query_executed"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_metric_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_dashboard_published"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_log_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_trace_recorded"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_alert_emitted"],
            false
        );
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_external_observability_sent"],
            false
        );
        assert_eq!(
            value["final_authorization_from_export_query_observability_allowed"],
            false
        );
        assert_eq!(
            value["operator_approval_from_export_query_observability_accepted"],
            false
        );
        assert_eq!(
            value["activation_from_export_query_observability_allowed"],
            false
        );
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(
            value["provider_invocation_authorized_from_export_query_observability"],
            false
        );
        assert_eq!(
            value["model_invocation_authorized_from_export_query_observability"],
            false
        );
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);

        let fixtures = value["export_query_observability_fixtures"]
            .as_array()
            .expect("first model invocation result receipt export query observability fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["export_query_observability_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["receipt_noop_confirmed"] == true
        }));

        let steps = value["audit_steps"].as_array().expect(
            "first model invocation final authorization result receipt export query observability steps",
        );
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "retention_expiry_garbage_collection_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "export_query_observability_fixture_denial"
        );
        assert_eq!(steps[2]["step"], "export_query_no_materialization_or_index");
        assert_eq!(steps[3]["step"], "observability_no_metric_log_trace_alert");
        assert_eq!(
            steps[4]["step"],
            "export_query_observability_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_export_query_observability_fixture_count"],
            8
        );
        assert_eq!(
            steps[2]["final_authorization_dry_run_result_receipt_export_materialized"],
            false
        );
        assert_eq!(
            steps[3]["final_authorization_dry_run_result_receipt_metric_recorded"],
            false
        );
        assert_eq!(
            steps[4]["activation_from_export_query_observability_allowed"],
            false
        );
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"].as_object().expect(
            "first model invocation final authorization result receipt export query observability side effects",
        );
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial"
        );
        assert_eq!(value["allowed_next_actions"][0]["exports_receipt"], false);
        assert_eq!(value["allowed_next_actions"][0]["registers_query"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["records_observability"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["delivers_briefing"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_endpoint_blocks_delivery_and_invocation_side_effects()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation final authorization result receipt operator summary briefing json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
        assert_eq!(
            value["implemented_route_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_delivery_no_provider_model_invocation"
        );
        assert_eq!(
            value["result_receipt_operator_facing_summary_briefing_state"],
            "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denied"
        );
        assert_eq!(value["operator_facing_summary_briefing_fixture_count"], 8);
        assert_eq!(
            value["blocked_operator_facing_summary_briefing_fixture_count"],
            8
        );
        assert_eq!(
            value["noop_operator_facing_summary_briefing_fixture_count"],
            8
        );
        assert_eq!(
            value["allowed_operator_facing_summary_briefing_fixture_count"],
            0
        );
        assert_eq!(
            value["accepted_operator_facing_summary_briefing_fixture_count"],
            0
        );
        assert_eq!(value["operator_facing_summary_briefing_performed_count"], 0);
        assert_eq!(value["operator_summary_recorded_count"], 0);
        assert_eq!(value["operator_summary_persisted_count"], 0);
        assert_eq!(value["operator_briefing_recorded_count"], 0);
        assert_eq!(value["operator_briefing_persisted_count"], 0);
        assert_eq!(value["operator_briefing_materialized_count"], 0);
        assert_eq!(value["operator_summary_dashboard_published_count"], 0);
        assert_eq!(value["operator_readback_recorded_count"], 0);
        assert_eq!(value["operator_final_note_recorded_count"], 0);
        assert_eq!(value["operator_final_note_delivered_count"], 0);
        assert_eq!(value["operator_summary_recorded"], false);
        assert_eq!(value["operator_summary_persisted"], false);
        assert_eq!(value["operator_briefing_recorded"], false);
        assert_eq!(value["operator_briefing_persisted"], false);
        assert_eq!(value["operator_briefing_materialized"], false);
        assert_eq!(value["operator_summary_dashboard_published"], false);
        assert_eq!(value["operator_readback_recorded"], false);
        assert_eq!(value["operator_final_note_recorded"], false);
        assert_eq!(value["operator_final_note_delivered"], false);
        assert_eq!(
            value["operator_acknowledgement_from_summary_accepted"],
            false
        );
        assert_eq!(value["activation_from_operator_briefing_allowed"], false);
        assert_eq!(
            value["activation_authority_from_operator_briefing_derived"],
            false
        );
        assert_eq!(value["provider_invocation_authorized"], false);
        assert_eq!(value["model_invocation_authorized"], false);
        assert_eq!(
            value["provider_invocation_authorized_from_operator_briefing"],
            false
        );
        assert_eq!(
            value["model_invocation_authorized_from_operator_briefing"],
            false
        );
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(value["provider_invoked"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["credential_read"], false);
        assert_eq!(value["secret_file_read"], false);
        assert_eq!(value["live_kg_write_performed"], false);
        assert_eq!(value["memory_store_write_performed"], false);
        assert_eq!(value["channel_send_performed"], false);
        assert_eq!(value["telegram_send_performed"], false);
        assert_eq!(value["external_send_performed"], false);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_matched"],
            true
        );

        let fixtures = value["operator_facing_summary_briefing_fixtures"]
            .as_array()
            .expect("operator facing summary briefing fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["operator_facing_summary_briefing_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
        }));

        let steps = value["audit_steps"]
            .as_array()
            .expect("operator facing summary briefing audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "export_query_observability_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "operator_facing_summary_briefing_fixture_denial"
        );
        assert_eq!(steps[2]["step"], "operator_summary_briefing_no_persistence");
        assert_eq!(steps[3]["step"], "operator_summary_briefing_no_delivery");
        assert_eq!(
            steps[4]["step"],
            "operator_summary_briefing_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_operator_facing_summary_briefing_fixture_count"],
            8
        );
        assert_eq!(steps[2]["operator_summary_recorded"], false);
        assert_eq!(steps[3]["operator_final_note_delivered"], false);
        assert_eq!(steps[3]["telegram_send_performed"], false);
        assert_eq!(steps[4]["activation_from_operator_briefing_allowed"], false);
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("operator facing summary briefing side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
        );
        assert_eq!(value["allowed_next_actions"][0]["records_summary"], false);
        assert_eq!(value["allowed_next_actions"][0]["persists_briefing"], false);
        assert_eq!(value["allowed_next_actions"][0]["delivers_briefing"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["accepts_acknowledgement"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_endpoint_blocks_acknowledgement_and_authority()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation final authorization result receipt final operator acknowledgement json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
        assert_eq!(
            value["implemented_route_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_delivery_no_provider_model_invocation"
        );
        assert_eq!(
            value["result_receipt_final_operator_acknowledgement_state"],
            "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denied"
        );
        assert_eq!(value["final_operator_acknowledgement_fixture_count"], 8);
        assert_eq!(
            value["blocked_final_operator_acknowledgement_fixture_count"],
            8
        );
        assert_eq!(
            value["noop_final_operator_acknowledgement_fixture_count"],
            8
        );
        assert_eq!(
            value["allowed_final_operator_acknowledgement_fixture_count"],
            0
        );
        assert_eq!(
            value["accepted_final_operator_acknowledgement_fixture_count"],
            0
        );
        assert_eq!(value["final_operator_acknowledgement_performed_count"], 0);
        assert_eq!(value["final_operator_acknowledgement_accepted_count"], 0);
        assert_eq!(value["final_operator_acknowledgement_recorded_count"], 0);
        assert_eq!(value["final_operator_acknowledgement_persisted_count"], 0);
        assert_eq!(value["final_operator_acknowledgement_delivered_count"], 0);
        assert_eq!(
            value["final_operator_acknowledgement_final_state_promoted_count"],
            0
        );
        assert_eq!(
            value["final_operator_acknowledgement_completion_promoted_count"],
            0
        );
        for key in [
            "final_operator_acknowledgement_allowed",
            "final_operator_acknowledgement_request_accepted",
            "final_operator_acknowledgement_accepted",
            "final_operator_acknowledgement_recorded",
            "final_operator_acknowledgement_persisted",
            "final_operator_acknowledgement_materialized",
            "final_operator_acknowledgement_filesystem_written",
            "final_operator_acknowledgement_delivered",
            "final_operator_acknowledgement_channel_delivery_performed",
            "final_operator_acknowledgement_identity_accepted",
            "final_operator_acknowledgement_signature_accepted",
            "final_operator_acknowledgement_timestamp_accepted",
            "final_operator_acknowledgement_final_state_promoted",
            "final_operator_acknowledgement_completion_promoted",
            "final_operator_acceptance_recorded",
            "final_operator_acceptance_persisted",
            "completion_acknowledgement_recorded",
            "status_acknowledgement_recorded",
            "summary_acknowledgement_recorded",
            "briefing_acknowledgement_recorded",
            "readback_digest_acknowledgement_recorded",
            "dashboard_acknowledgement_recorded",
            "notification_acknowledgement_recorded",
            "channel_acknowledgement_delivered",
            "external_acknowledgement_sent",
            "telegram_acknowledgement_sent",
            "operator_approval_from_acknowledgement_derived",
            "activation_authority_from_acknowledgement_derived",
            "provider_invocation_authorized",
            "model_invocation_authorized",
            "provider_invocation_authorized_from_acknowledgement",
            "model_invocation_authorized_from_acknowledgement",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "provider_router_live_envelope_executed",
            "provider_prompt_injection_performed",
            "context_injection_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "memory_store_write_performed",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "release_artifact_written",
            "public_claim_recorded",
            "public_release_claimed",
            "install_executed",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            assert_eq!(
                value[key], false,
                "final operator acknowledgement field should stay false: {key}"
            );
        }
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_matched"],
            true
        );

        let fixtures = value["final_operator_acknowledgement_fixtures"]
            .as_array()
            .expect("final operator acknowledgement fixtures");
        assert_eq!(fixtures.len(), 8);
        assert!(fixtures.iter().all(|fixture| {
            fixture["final_operator_acknowledgement_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["final_acknowledgement_noop_confirmed"].as_bool() == Some(true)
        }));
        assert_eq!(
            fixtures
                .iter()
                .filter(
                    |fixture| fixture["source_operator_facing_summary_briefing_present"] == false
                )
                .count(),
            1
        );
        assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| fixture["telegram_acknowledgement_requested"] == true)
                .count(),
            1
        );
        assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| fixture["activation_from_acknowledgement_requested"] == true)
                .count(),
            1
        );

        let steps = value["audit_steps"]
            .as_array()
            .expect("final operator acknowledgement audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "operator_facing_summary_briefing_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "final_operator_acknowledgement_fixture_denial"
        );
        assert_eq!(
            steps[2]["step"],
            "final_operator_acknowledgement_no_acceptance_or_persistence"
        );
        assert_eq!(
            steps[3]["step"],
            "final_operator_acknowledgement_no_delivery"
        );
        assert_eq!(
            steps[4]["step"],
            "final_operator_acknowledgement_authority_non_promotion"
        );
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_final_operator_acknowledgement_fixture_count"],
            8
        );
        assert_eq!(steps[2]["final_operator_acknowledgement_accepted"], false);
        assert_eq!(steps[3]["telegram_acknowledgement_sent"], false);
        assert_eq!(
            steps[4]["activation_authority_from_acknowledgement_derived"],
            false
        );

        let side_effects = value["side_effects"]
            .as_object()
            .expect("final operator acknowledgement side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["accepts_acknowledgement"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["claims_public_release"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["writes_release_artifact"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["activates_runtime"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_endpoint_blocks_terminal_decision_public_claim_and_authority()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation final authorization result receipt terminal operator decision public claim json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
        assert_eq!(
            value["implemented_route_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_provider_model_invocation"
        );
        assert_eq!(
            value["result_receipt_terminal_operator_decision_public_claim_state"],
            "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denied"
        );
        assert_eq!(
            value["terminal_operator_decision_public_claim_fixture_count"],
            10
        );
        assert_eq!(
            value["blocked_terminal_operator_decision_public_claim_fixture_count"],
            10
        );
        assert_eq!(
            value["noop_terminal_operator_decision_public_claim_fixture_count"],
            10
        );
        assert_eq!(
            value["allowed_terminal_operator_decision_public_claim_fixture_count"],
            0
        );
        assert_eq!(
            value["accepted_terminal_operator_decision_public_claim_fixture_count"],
            0
        );
        assert_eq!(value["terminal_operator_decision_performed_count"], 0);
        assert_eq!(value["public_claim_promotion_performed_count"], 0);
        assert_eq!(value["terminal_operator_decision_accepted_count"], 0);
        assert_eq!(value["terminal_operator_decision_recorded_count"], 0);
        assert_eq!(value["terminal_operator_decision_persisted_count"], 0);
        assert_eq!(value["terminal_operator_decision_delivered_count"], 0);
        assert_eq!(value["public_claim_recorded_count"], 0);
        assert_eq!(value["public_claim_promoted_count"], 0);
        assert_eq!(value["public_release_published_count"], 0);
        assert_eq!(value["release_artifact_written_count"], 0);
        for key in [
            "terminal_operator_decision_allowed",
            "terminal_operator_decision_request_accepted",
            "terminal_operator_decision_accepted",
            "terminal_operator_decision_recorded",
            "terminal_operator_decision_persisted",
            "terminal_operator_decision_materialized",
            "terminal_operator_decision_filesystem_written",
            "terminal_operator_decision_delivered",
            "terminal_operator_decision_channel_delivery_performed",
            "terminal_operator_decision_identity_accepted",
            "terminal_operator_decision_signature_accepted",
            "terminal_operator_decision_timestamp_accepted",
            "terminal_operator_decision_final_state_promoted",
            "terminal_operator_decision_completion_promoted",
            "public_claim_requested",
            "public_claim_accepted",
            "public_claim_recorded",
            "public_claim_persisted",
            "public_claim_materialized",
            "public_claim_promoted",
            "public_ga_claimed",
            "public_release_claimed",
            "public_release_published",
            "public_distribution_performed",
            "public_artifact_written",
            "release_artifact_written",
            "activation_allowed_by_terminal_operator_decision",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "provider_invocation_authorized",
            "model_invocation_authorized",
            "provider_invocation_authorized_from_terminal_decision",
            "model_invocation_authorized_from_terminal_decision",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "provider_router_live_envelope_executed",
            "provider_prompt_injection_performed",
            "context_injection_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "memory_store_write_performed",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            assert_eq!(
                value[key], false,
                "terminal operator decision/public claim field should stay false: {key}"
            );
        }
        assert_eq!(value["provider_invocation_budget"], 0);
        assert_eq!(value["model_invocation_budget"], 0);
        assert_eq!(
            value["final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_readback_hash_matched"],
            true
        );

        let fixtures = value["terminal_operator_decision_public_claim_fixtures"]
            .as_array()
            .expect("terminal operator decision public claim fixtures");
        assert_eq!(fixtures.len(), 10);
        assert!(fixtures.iter().all(|fixture| {
            fixture["terminal_operator_decision_public_claim_status"]
                .as_str()
                .is_some_and(|status| status.starts_with("blocked_"))
                && fixture["terminal_operator_decision_public_claim_noop_confirmed"].as_bool()
                    == Some(true)
        }));
        assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| fixture["source_final_operator_acknowledgement_present"] == false)
                .count(),
            1
        );
        assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| fixture["public_claim_promotion_requested"] == true)
                .count(),
            1
        );
        assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| fixture["service_restart_decision_requested"] == true)
                .count(),
            1
        );

        let steps = value["audit_steps"]
            .as_array()
            .expect("terminal operator decision public claim audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "final_operator_acknowledgement_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "terminal_operator_decision_public_claim_fixture_denial"
        );
        assert_eq!(
            steps[2]["step"],
            "terminal_operator_decision_no_acceptance_or_persistence"
        );
        assert_eq!(steps[3]["step"], "public_claim_non_promotion");
        assert_eq!(steps[4]["step"], "activation_install_authority_denial");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["blocked_terminal_operator_decision_public_claim_fixture_count"],
            10
        );
        assert_eq!(steps[2]["terminal_operator_decision_accepted"], false);
        assert_eq!(steps[3]["public_claim_promoted"], false);
        assert_eq!(steps[4]["active_binary_mutated"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("terminal operator decision public claim side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["accepts_terminal_decision"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["claims_public_release"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["exposes_public_status"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["writes_release_artifact"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["activates_runtime"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_endpoint_blocks_public_status_exposure_and_authority()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation final authorization result receipt terminal public claim status exposure json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
        assert_eq!(
            value["implemented_route_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_no_status_exposure_no_public_claim_no_release_no_artifact_no_provider_model_invocation"
        );
        assert_eq!(
            value["result_receipt_terminal_public_claim_status_exposure_state"],
            "final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denied"
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_surface_count"],
            18
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_attempt_count"],
            18
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_allowed_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_request_accepted_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_accepted_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_recorded_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_persisted_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_materialized_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_filesystem_written_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_status_exposure_delivered_count"],
            0
        );
        assert_eq!(value["terminal_public_claim_status_exposed_count"], 0);
        assert_eq!(value["public_status_claimed_count"], 0);
        assert_eq!(value["public_release_claimed_count"], 0);
        assert_eq!(value["public_ga_claimed_count"], 0);
        assert_eq!(value["dashboard_status_exposed_count"], 0);
        assert_eq!(value["public_badge_exposed_count"], 0);
        assert_eq!(value["status_endpoint_exposed_count"], 0);
        assert_eq!(value["query_status_exposed_count"], 0);
        assert_eq!(value["export_status_exposed_count"], 0);
        assert_eq!(value["observability_status_exposed_count"], 0);
        assert_eq!(value["release_notes_status_exposed_count"], 0);
        assert_eq!(value["changelog_status_exposed_count"], 0);
        assert_eq!(value["version_tag_status_exposed_count"], 0);
        assert_eq!(value["artifact_availability_status_exposed_count"], 0);
        assert_eq!(value["distribution_queue_status_exposed_count"], 0);
        assert_eq!(value["channel_status_delivered_count"], 0);
        assert_eq!(value["external_status_sent_count"], 0);
        assert_eq!(value["telegram_status_sent_count"], 0);
        assert_eq!(value["release_publication_authority_derived_count"], 0);
        assert_eq!(value["activation_authority_derived_count"], 0);
        assert_eq!(value["live_execution_allowed_count"], 0);
        for key in [
            "terminal_public_claim_status_exposure_accepted",
            "terminal_public_claim_status_exposed",
            "status_endpoint_exposed",
            "query_status_exposed",
            "export_status_exposed",
            "observability_status_exposed",
            "operator_approval_recorded",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_performed",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "external_send_performed",
        ] {
            assert_eq!(
                value[key], false,
                "terminal public claim status exposure field should stay false: {key}"
            );
        }
        assert_eq!(
            value["terminal_public_claim_status_exposure_readback_hash_matched"],
            true
        );

        let surfaces = value["terminal_public_claim_status_exposure_surfaces"]
            .as_array()
            .expect("terminal public claim status exposure surfaces");
        assert_eq!(surfaces.len(), 18);
        assert!(surfaces.iter().all(|surface| {
            surface["public_claim_status_exposure_attempted"].as_bool() == Some(true)
                && surface["public_claim_status_exposure_allowed"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_request_accepted"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_recorded"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_persisted"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_materialized"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_filesystem_written"].as_bool()
                    == Some(false)
                && surface["public_claim_status_exposure_delivered"].as_bool() == Some(false)
                && surface["public_claim_status_exposed"].as_bool() == Some(false)
                && surface["public_status_claimed"].as_bool() == Some(false)
                && surface["public_release_claimed"].as_bool() == Some(false)
                && surface["public_ga_claimed"].as_bool() == Some(false)
                && surface["status_endpoint_exposed"].as_bool() == Some(false)
                && surface["query_status_exposed"].as_bool() == Some(false)
                && surface["export_status_exposed"].as_bool() == Some(false)
                && surface["observability_status_exposed"].as_bool() == Some(false)
                && surface["channel_status_delivered"].as_bool() == Some(false)
                && surface["external_status_sent"].as_bool() == Some(false)
                && surface["telegram_status_sent"].as_bool() == Some(false)
                && surface["release_publication_authority_derived"].as_bool() == Some(false)
                && surface["activation_authority_derived"].as_bool() == Some(false)
                && surface["live_execution_allowed"].as_bool() == Some(false)
                && surface["public_claim_status_exposure_noop_confirmed"].as_bool() == Some(true)
        }));

        let denied =
            value["denied_by_first_model_invocation_terminal_public_claim_status_exposure"]
                .as_array()
                .expect("terminal public claim status exposure denials");
        assert_eq!(denied.len(), 34);
        assert_eq!(
            value["denied_by_first_model_invocation_terminal_public_claim_status_exposure_count"],
            34
        );

        let steps = value["audit_steps"]
            .as_array()
            .expect("terminal public claim status exposure audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "terminal_operator_decision_public_claim_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "terminal_public_claim_status_exposure_fixture_denial"
        );
        assert_eq!(
            steps[2]["step"],
            "public_status_no_recording_or_materialization"
        );
        assert_eq!(steps[3]["step"], "public_status_no_delivery_or_endpoint");
        assert_eq!(steps[4]["step"], "authority_and_invocation_denial");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");
        assert_eq!(
            steps[1]["terminal_public_claim_status_exposure_allowed_count"],
            0
        );
        assert_eq!(
            steps[2]["terminal_public_claim_status_exposure_materialized_count"],
            0
        );
        assert_eq!(steps[3]["status_endpoint_exposed_count"], 0);
        assert_eq!(steps[4]["activation_authority_derived_count"], 0);
        assert_eq!(steps[4]["provider_invoked"], false);
        assert_eq!(steps[4]["model_invoked"], false);

        let side_effects = value["side_effects"]
            .as_object()
            .expect("terminal public claim status exposure side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["accepts_public_status"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["claims_public_release"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["delivers_channel"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["writes_release_artifact"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["activates_runtime"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
    }

    #[test]
    fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_endpoint_blocks_delivery_readback_receipts_and_authority()
     {
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
            &options,
        );
        assert_eq!(status, "200 OK");
        assert_eq!(content_type, "application/json; charset=utf-8");

        let value: serde_json::Value = serde_json::from_str(&body).expect(
            "first model invocation final authorization result receipt terminal public claim delivery readback json",
        );
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(
            value["endpoint"],
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT
        );
        assert_eq!(
            value["source_command"],
            "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-denial --json"
        );
        assert_eq!(
            value["native_gateway_source_command_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
        assert_eq!(
            value["implemented_route_count"],
            NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        );
        assert_eq!(value["missing_route_count"], 0);
        assert_eq!(value["route_count_source_command_accepted"], true);
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_route_enabled"],
            true
        );
        assert_eq!(
            value["first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_ready"],
            true
        );
        assert_eq!(
            value["source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready"],
            true
        );
        assert_eq!(
            value["canary_execution_mode"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_no_delivery_no_readback_no_receipt_no_release_no_channel_no_telegram_no_install"
        );
        assert_eq!(
            value["result_receipt_terminal_public_claim_delivery_readback_state"],
            "final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denied"
        );
        assert_eq!(
            value["source_terminal_public_claim_status_exposure_surface_count"],
            18
        );
        assert_eq!(
            value["source_terminal_public_claim_status_exposed_count"],
            0
        );
        assert_eq!(
            value["terminal_public_claim_delivery_readback_surface_count"],
            18
        );
        assert_eq!(
            value["terminal_public_claim_delivery_readback_attempt_count"],
            18
        );
        assert_eq!(
            value["terminal_public_claim_delivery_readback_denied_count"],
            18
        );
        for key in [
            "terminal_public_claim_delivery_readback_allowed_count",
            "terminal_public_claim_delivery_readback_accepted_count",
            "terminal_public_claim_delivery_readback_recorded_count",
            "terminal_public_claim_delivery_readback_persisted_count",
            "terminal_public_claim_delivery_readback_delivered_count",
            "terminal_public_claim_delivery_readback_status_read_count",
            "public_claim_delivery_recorded_count",
            "public_claim_delivery_persisted_count",
            "status_readback_recorded_count",
            "status_readback_persisted_count",
            "channel_delivery_recorded_count",
            "channel_delivery_persisted_count",
            "channel_status_readback_delivered_count",
            "external_delivery_readback_sent_count",
            "telegram_delivery_readback_sent_count",
            "delivery_receipt_recorded_count",
            "delivery_receipt_persisted_count",
            "readback_receipt_recorded_count",
            "readback_receipt_persisted_count",
            "release_artifact_written_count",
            "public_artifact_written_count",
            "operator_approval_from_delivery_readback_derived_count",
            "release_publication_authority_from_delivery_readback_derived_count",
            "activation_authority_from_delivery_readback_derived_count",
            "download_link_from_delivery_readback_rendered_count",
            "install_command_from_delivery_readback_emitted_count",
            "install_from_delivery_readback_executed_count",
            "service_restart_from_delivery_readback_performed_count",
            "active_binary_from_delivery_readback_mutated_count",
            "memory_store_write_performed_count",
            "live_kg_write_performed_count",
            "provider_invoked_count",
            "model_invoked_count",
            "credential_read_count",
            "secret_file_read_count",
            "external_send_performed_count",
        ] {
            assert_eq!(
                value[key], 0,
                "terminal public claim delivery readback count should stay zero: {key}"
            );
        }

        for key in [
            "terminal_public_claim_delivery_readback_accepted",
            "terminal_public_claim_delivery_readback_recorded",
            "terminal_public_claim_delivery_readback_persisted",
            "terminal_public_claim_delivery_readback_delivered",
            "terminal_public_claim_delivery_readback_status_read",
            "public_claim_delivery_recorded",
            "public_claim_delivery_persisted",
            "status_readback_recorded",
            "status_readback_persisted",
            "channel_delivery_recorded",
            "channel_delivery_persisted",
            "delivery_receipt_recorded",
            "delivery_receipt_persisted",
            "readback_receipt_recorded",
            "readback_receipt_persisted",
            "public_release_claimed",
            "public_ga_claimed",
            "operator_approval_recorded",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_emitted",
            "activation_allowed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
            "release_artifact_written",
            "public_artifact_written",
            "external_send_performed",
            "filesystem_written",
        ] {
            assert_eq!(
                value[key], false,
                "terminal public claim delivery readback field should stay false: {key}"
            );
        }

        let surfaces = value["terminal_public_claim_delivery_readback_surfaces"]
            .as_array()
            .expect("terminal public claim delivery readback surfaces");
        assert_eq!(surfaces.len(), 18);
        assert!(surfaces.iter().all(|surface| {
            surface["terminal_public_claim_delivery_readback_attempted"].as_bool() == Some(true)
                && surface["terminal_public_claim_delivery_readback_noop_confirmed"].as_bool()
                    == Some(true)
                && surface["public_claim_delivery_allowed"].as_bool() == Some(false)
                && surface["status_readback_allowed"].as_bool() == Some(false)
                && surface["channel_delivery_allowed"].as_bool() == Some(false)
                && surface["telegram_delivery_allowed"].as_bool() == Some(false)
                && surface["external_delivery_allowed"].as_bool() == Some(false)
                && surface["delivery_receipt_allowed"].as_bool() == Some(false)
                && surface["readback_receipt_allowed"].as_bool() == Some(false)
                && surface["release_artifact_write_allowed"].as_bool() == Some(false)
                && surface["public_artifact_write_allowed"].as_bool() == Some(false)
                && surface["activation_authority_derivation_allowed"].as_bool() == Some(false)
                && surface["install_restart_active_binary_mutation_allowed"].as_bool()
                    == Some(false)
                && surface["provider_invocation_allowed"].as_bool() == Some(false)
                && surface["model_invocation_allowed"].as_bool() == Some(false)
                && surface["credential_read_allowed"].as_bool() == Some(false)
        }));
        assert_eq!(
            surfaces
                .iter()
                .filter(|surface| surface["telegram_delivery_requested"].as_bool() == Some(true))
                .count(),
            1
        );
        assert_eq!(
            surfaces
                .iter()
                .filter(
                    |surface| surface["public_claim_delivery_requested"].as_bool() == Some(true)
                )
                .count(),
            4
        );
        assert_eq!(
            value["terminal_public_claim_delivery_readback_readback_hash_matched"],
            true
        );

        let denied =
            value["denied_by_first_model_invocation_terminal_public_claim_delivery_readback"]
                .as_array()
                .expect("terminal public claim delivery readback denials");
        assert_eq!(denied.len(), 26);
        assert_eq!(
            value["denied_by_first_model_invocation_terminal_public_claim_delivery_readback_count"],
            26
        );

        let steps = value["audit_steps"]
            .as_array()
            .expect("terminal public claim delivery readback audit steps");
        assert_eq!(steps.len(), 6);
        assert_eq!(
            steps[0]["step"],
            "terminal_public_claim_status_exposure_source_binding"
        );
        assert_eq!(
            steps[1]["step"],
            "terminal_public_claim_delivery_readback_fixture_denial"
        );
        assert_eq!(
            steps[2]["step"],
            "delivery_and_readback_no_recording_or_persistence"
        );
        assert_eq!(
            steps[3]["step"],
            "channel_external_telegram_delivery_denial"
        );
        assert_eq!(steps[4]["step"], "artifact_authority_install_denial");
        assert_eq!(steps[5]["step"], "side_effect_denial_check");

        let side_effects = value["side_effects"]
            .as_object()
            .expect("terminal public claim delivery readback side effects");
        assert!(
            side_effects
                .values()
                .all(|item| item.as_bool() == Some(false))
        );
        assert_eq!(
            value["allowed_next_actions"][0]["action"],
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_release_artifact_publication_denial"
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_public_claim_delivery"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["records_status_readback"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["sends_telegram"], false);
        assert_eq!(
            value["allowed_next_actions"][0]["writes_release_artifact"],
            false
        );
        assert_eq!(
            value["allowed_next_actions"][0]["installs_or_restarts"],
            false
        );
        assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
        assert_eq!(value["allowed_next_actions"][0]["reads_credentials"], false);
    }
