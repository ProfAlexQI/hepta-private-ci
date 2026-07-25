    #[test]
    fn first_model_invocation_denial_routes_are_data_driven_and_side_effect_free() {
        let endpoints = [
            HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
        ];
        let options = NativeGatewayOptions {
            bind_addr: "127.0.0.1:7373".to_string(),
            with_telegram_plugin: true,
            telegram_plugin_poll_ms: 1500,
        };
        let denied_effects = [
            "provider_invoked",
            "model_invoked",
            "credential_value_read",
            "credential_read",
            "secret_file_read",
            "live_kg_write_performed",
            "memory_store_write_performed",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
        ];

        for endpoint in endpoints {
            let (status, content_type, body) =
                route_native_gateway_request("GET", endpoint, &options);
            assert_eq!(status, "200 OK", "{endpoint}");
            assert_eq!(content_type, "application/json; charset=utf-8", "{endpoint}");
            let value: serde_json::Value =
                serde_json::from_str(&body).expect("first model invocation route json");
            assert_eq!(value["runtime"], "hepta", "{endpoint}");
            assert_eq!(value["status"], "ready", "{endpoint}");
            assert_eq!(value["endpoint"], endpoint, "{endpoint}");
            assert_eq!(
                value["native_gateway_source_command_count"],
                NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
                "{endpoint}"
            );
            assert_eq!(value["missing_route_count"], 0, "{endpoint}");
            assert_eq!(
                value["route_count_source_command_accepted"], true,
                "{endpoint}"
            );
            let observed = denied_effects
                .iter()
                .filter(|field| !value[**field].is_null())
                .map(|field| {
                    assert_eq!(value[*field], false, "{endpoint}: {field}");
                    *field
                })
                .count();
            assert!(observed >= 5, "{endpoint}: denial evidence inventory");
        }
    }
