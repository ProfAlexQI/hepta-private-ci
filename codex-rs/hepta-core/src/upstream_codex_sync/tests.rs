use super::*;

#[test]
fn upstream_codex_sync_lane_is_ready_without_side_effects_or_latest_claims() {
    let report = hepta_upstream_codex_sync_lane_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.lane_id, "upstream-codex-sync-lane");
    assert_eq!(
        report.upstream_repository,
        "https://github.com/openai/codex"
    );
    assert_eq!(report.contract_count, 5);
    assert_eq!(report.ready_contract_count, report.contract_count);
    assert!(report.sync_lane_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_latest_claimed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_runtime_codex_engine_dependency_allowed);
    assert!(report.requires_diff_classification_before_absorption);
    assert!(report.requires_adapter_contract_before_active_runtime);
    assert!(report.requires_release_governance_before_public_claim);
    assert!(report.local_only_audit);
    assert!(report.report_only);
    assert!(!report.mutates_runtime_state);
    assert!(!report.external_network_read);
    assert!(!report.external_send);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_sync_lane_keeps_active_service_dependency_isolation_mandatory() {
    let report = hepta_upstream_codex_sync_lane_report();

    assert_eq!(
        report.active_dependency_isolation_gate,
        "scripts/hepta-active-service-dependency-isolation.sh"
    );
    assert!(report.contracts.iter().any(|contract| {
        contract.id == "compatibility-package-retention-boundary"
            && contract
                .required_gate
                .contains("active hepta-cli cargo tree")
            && !contract.auto_apply_allowed
            && !contract.active_runtime_dependency_allowed
            && !contract.public_release_claim_allowed
    }));
    assert!(report.contracts.iter().any(|contract| {
        contract.id == "provider-credential-security-classification"
            && matches!(contract.risk, HeptaUpstreamCodexSyncRisk::P0Security)
    }));
}

#[test]
fn upstream_codex_snapshot_intake_is_ready_without_default_side_effects() {
    let report = hepta_upstream_codex_snapshot_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.snapshot_lane_id, "upstream-codex-snapshot-intake");
    assert_eq!(
        report.snapshot_gate,
        "scripts/hepta-upstream-codex-snapshot.sh"
    );
    assert_eq!(
        report.sync_lane_gate,
        "scripts/hepta-upstream-codex-sync-lane.sh"
    );
    assert_eq!(
        report.active_dependency_isolation_gate,
        "scripts/hepta-active-service-dependency-isolation.sh"
    );
    assert_eq!(report.risk_class_count, 4);
    assert_eq!(report.ready_risk_class_count, report.risk_class_count);
    assert!(report.snapshot_intake_ready);
    assert!(report.observed_upstream_head_required_before_absorption);
    assert!(report.local_compatibility_head_required);
    assert!(report.diff_range_required_before_absorption);
    assert!(report.diff_inventory_required_before_absorption);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.external_network_read_default);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_snapshot_requires_classification_for_all_risk_classes() {
    let report = hepta_upstream_codex_snapshot_report();

    assert!(report.risk_classes.iter().all(|risk_class| {
        risk_class.classification_required
            && !risk_class.auto_absorb_allowed
            && !risk_class.active_runtime_dependency_allowed
    }));
    assert!(report.risk_classes.iter().any(|risk_class| {
        risk_class.id == "provider-credential-sandbox-security"
            && matches!(risk_class.risk, HeptaUpstreamCodexSyncRisk::P0Security)
    }));
    assert!(report.risk_classes.iter().any(|risk_class| {
        risk_class.id == "runtime-session-tool-mcp-appserver"
            && matches!(risk_class.risk, HeptaUpstreamCodexSyncRisk::P0Runtime)
    }));
    assert!(report.risk_classes.iter().any(|risk_class| {
        risk_class.id == "legacy-cli-tui-compatibility"
            && risk_class
                .hepta_review_surfaces
                .iter()
                .any(|surface| surface.contains("dependency-isolation"))
    }));
}

#[test]
fn upstream_codex_diff_ledger_contract_is_ready_without_fetch_or_merge() {
    let report = hepta_upstream_codex_diff_ledger_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.ledger_id, "upstream-codex-diff-range-ledger");
    assert_eq!(
        report.baseline_upstream_head,
        HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD
    );
    assert_eq!(
        report.target_upstream_head,
        HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD
    );
    assert_eq!(report.target_ref, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF);
    assert_eq!(
        report.diff_ledger_gate,
        "scripts/hepta-upstream-codex-diff-ledger.sh"
    );
    assert_eq!(
        report.candidate_diff_range,
        format!(
            "{}..{}",
            HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD
        )
    );
    assert!(report.commit_inventory_required);
    assert!(report.file_inventory_required);
    assert!(report.risk_bucket_classification_required);
    assert_eq!(report.bucket_count, 4);
    assert_eq!(report.ready_bucket_count, report.bucket_count);
    assert!(report.diff_ledger_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.external_network_read_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_diff_ledger_requires_all_bucket_classifications() {
    let report = hepta_upstream_codex_diff_ledger_report();

    assert!(report.buckets.iter().all(|bucket| {
        bucket.classification_required
            && bucket.bucket_ready
            && !bucket.auto_absorb_allowed
            && !bucket.active_runtime_dependency_allowed
    }));
    assert!(report.buckets.iter().any(|bucket| {
        bucket.id == "provider-credential-sandbox-security"
            && matches!(bucket.risk, HeptaUpstreamCodexSyncRisk::P0Security)
            && bucket.promotion_gate.contains("dependency isolation")
    }));
    assert!(report.buckets.iter().any(|bucket| {
        bucket.id == "runtime-session-tool-mcp-appserver"
            && matches!(bucket.risk, HeptaUpstreamCodexSyncRisk::P0Runtime)
            && bucket.promotion_gate.contains("shadow-replay")
    }));
    assert!(report.buckets.iter().any(|bucket| {
        bucket.id == "legacy-cli-tui-compatibility"
            && bucket
                .promotion_gate
                .contains("active hepta-cli cargo tree")
    }));
    assert!(report.buckets.iter().any(|bucket| {
        bucket.id == "product-doc-release-governance"
            && bucket.promotion_gate.contains("long soak evidence")
    }));
}

#[test]
fn upstream_codex_current_intake_separates_observation_from_absorption() {
    let report = hepta_upstream_codex_current_intake_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.observation_state, "observed");
    assert_eq!(report.classification_state, "classified");
    assert_eq!(report.selected_state, "absorbed");
    assert_eq!(report.remaining_state, "deferred");
    assert_eq!(report.baseline_head, HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD);
    assert_eq!(report.cutoff_ref, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF);
    assert_eq!(report.cutoff_head, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD);
    assert_eq!(report.observed_commit_count, 1821);
    assert_eq!(report.observed_changed_file_count, 3389);
    assert_eq!(report.observed_codex_rs_changed_file_count, 3127);
    assert_eq!(report.selected_absorption_count, 12);
    assert_eq!(report.deferred_decision_count, 20);
    assert!(report.current_intake_ready);
    assert!(!report.full_range_absorption_claimed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_rebase_performed);
    assert!(!report.whole_tree_replacement_performed);
    assert!(!report.cargo_lock_replacement_performed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
}

#[test]
fn upstream_codex_current_intake_tracks_selected_and_deferred_decisions() {
    let report = hepta_upstream_codex_current_intake_report();
    let absorbed: Vec<&HeptaUpstreamCodexCurrentIntakeDecision> = report
        .decisions
        .iter()
        .filter(|decision| {
            decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed
        })
        .collect();
    let deferred: Vec<&HeptaUpstreamCodexCurrentIntakeDecision> = report
        .decisions
        .iter()
        .filter(|decision| {
            decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Deferred
        })
        .collect();

    assert_eq!(absorbed.len(), 12);
    assert_eq!(deferred.len(), 20);
    assert!(absorbed.iter().all(|decision| {
        decision.upstream_commit.is_some()
            && !decision.local_receipts.is_empty()
            && decision.absorption_kind.is_some()
    }));
    assert!(deferred.iter().all(|decision| {
        decision.local_receipts.is_empty() && decision.absorption_kind.is_none()
    }));
    let actual_r2_deferred: Vec<(&str, &str)> = deferred
        .iter()
        .filter(|decision| decision.classification.starts_with("r2_"))
        .map(|decision| {
            (
                decision.classification.as_str(),
                decision
                    .upstream_commit
                    .as_deref()
                    .expect("r2 deferred commit"),
            )
        })
        .collect();
    assert_eq!(
        actual_r2_deferred,
        vec![
            (
                "r2_windows_write_root_acl_integrity",
                "bd92b056ddd91bd7c2ecfea3d8773f7eb5a879a6",
            ),
            (
                "r2_hook_context_spill_limits",
                "e4836f998da166aba456f60d2e74eb79d6e2542b",
            ),
            (
                "r2_session_start_hook_ordering",
                "8c41ed33ce3e39460e7b13b14c35e0c39bb5980d",
            ),
            (
                "r2_approval_rejection_reason_propagation",
                "e52c35b0001ea3e4a1744b99c4250a5b1a09e44d",
            ),
            (
                "r2_history_hook_api_test_alignment",
                "ec3140db1297f3acebec7d6916b329cad3b12693",
            ),
            (
                "r2_paginated_rollout_lineage_resolution",
                "b7e39aa31608b6eaba4f317538a8f82985a9e854",
            ),
            (
                "r2_threadless_mcp_connection_events",
                "19940967bdb5ac04aec5d08ebd465481f1ac964d",
            ),
            (
                "r2_sqlite_test_path_validation",
                "81e89fa5af13012c8313f032a17b11b9a5170d33",
            ),
            (
                "r2_agent_job_storage_migration",
                "687f05cb946d10c96f90dd7ce82e11465c6e20a7",
            ),
            (
                "r2_hook_warning_tui_presentation",
                "cf821e8ec850c6d8380feea0e84859dd8ff54cd0",
            ),
            (
                "r2_connector_metadata_enrichment",
                "60272096bc125ad7bd8ec26508b19d1e0db2874b",
            ),
            (
                "r2_windows_exec_server_sandboxing",
                "35c2278dd5c49daf8a4e44468038aed9be9e866e",
            ),
            (
                "r2_shared_skill_model_migration",
                "56c11cf6586c0579e4e3eca14eefb0916b14c78c",
            ),
            (
                "r2_remote_compaction_history_optimization",
                "fd3c1dc13d0a0941af406e1bc1f697c9d14110ea",
            ),
            (
                "r2_approval_catalog_policy_compatibility",
                "2be7d3bcd9d1aec2780f0a71fe79cbb5afd877a1",
            ),
            (
                "r2_outbound_proxy_route_resolution",
                "c9ef7eff005c3299a5a5f0004c34c6a3eedf2564",
            ),
            (
                "r2_managed_permission_proxy_resolution",
                "88fac6fe108237a105d3203e3508b0d531054312",
            ),
        ]
    );
    assert!(absorbed.iter().any(|decision| {
        decision.upstream_commit.as_deref() == Some("9dbdb4e2c08723e8fc9c18f64d7ccad3dadc03a7")
            && decision.absorption_kind.as_deref() == Some("local_split")
            && decision.local_receipts.len() == 7
    }));
    assert!(absorbed.iter().any(|decision| {
        decision.classification == "mcp_endpoint_ownership"
            && decision.upstream_commit.as_deref()
                == Some("6bf4845b60e0abccd0c64690e9c7591e0efb85d8")
            && decision.absorption_kind.as_deref() == Some("semantic_port")
            && decision.local_receipts == ["f983f4ae7fc7e4b224272990106049f30ee472d7"]
    }));
    assert!(absorbed.iter().any(|decision| {
        decision.classification == "linux_proc_preflight_filesystem_isolation"
            && decision.upstream_commit.as_deref()
                == Some("44481a1c4548d1cc0cc3c95aa03b59ec4cba074a")
            && decision.absorption_kind.as_deref() == Some("semantic_port")
            && decision.local_receipts == ["c62ce9e2d4ee0ccaa85b50098f41198b44ae17e7"]
    }));
}

#[test]
fn upstream_codex_current_intake_preserves_predecessor_cutoff_evidence() {
    let report = hepta_upstream_codex_current_intake_report();

    assert_eq!(
        report.predecessor_manifest_path,
        HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_PATH
    );
    assert_eq!(
        report.predecessor_manifest_sha256,
        HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_SHA256
    );
    assert_eq!(
        report.predecessor_cutoff_ref,
        HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_REF
    );
    assert_eq!(
        report.predecessor_cutoff_head,
        HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_HEAD
    );
    assert!(report.predecessor_cutoff_preserved);
    assert_ne!(report.cutoff_ref, report.predecessor_cutoff_ref);
    assert_ne!(report.cutoff_head, report.predecessor_cutoff_head);

    let history_storage = report
        .decisions
        .iter()
        .find(|decision| decision.classification == "history_storage_efficiency")
        .expect("history storage decision");
    assert_eq!(
        history_storage.upstream_commit.as_deref(),
        Some("45ac251e178416ff5c3022457ad8d2778c0d4549")
    );
    assert_ne!(
        history_storage.upstream_commit.as_deref(),
        Some(report.cutoff_head.as_str())
    );
}

#[test]
fn upstream_codex_current_intake_preserves_historical_receipt_provenance() {
    let report = hepta_upstream_codex_current_intake_report();

    assert_eq!(
        report.historical_receipt_target_head,
        HEPTA_UPSTREAM_CODEX_HISTORICAL_RECEIPT_TARGET_HEAD
    );
    assert_eq!(
        report.historical_receipt_changed_file_count,
        HEPTA_UPSTREAM_CODEX_HISTORICAL_LEDGER_CHANGED_FILE_COUNT
    );
    assert_eq!(
        report.historical_receipt_selected_absorption_count,
        HEPTA_UPSTREAM_CODEX_HISTORICAL_SELECTED_ABSORPTION_COUNT
    );
    assert!(!report.historical_receipt_is_current_freshness_proof);
    assert_ne!(report.cutoff_head, report.historical_receipt_target_head);
}

#[test]
fn upstream_codex_product_governance_absorption_contract_is_ready_and_bounded() {
    let report = hepta_upstream_codex_product_governance_absorption_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.absorption_id,
        "upstream-codex-product-governance-absorption-contract"
    );
    assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
    assert!(matches!(
        report.selected_bucket_risk,
        HeptaUpstreamCodexSyncRisk::P2Product
    ));
    assert_eq!(report.selected_changed_file_count, 22);
    assert!(report.selected_commit_sample_count > 0);
    assert_eq!(
        report.source_ledger_gate,
        "scripts/hepta-upstream-codex-diff-ledger.sh"
    );
    assert_eq!(
        report.absorption_gate,
        "scripts/hepta-upstream-codex-product-governance-absorption.sh"
    );
    assert!(report.selected_as_first_absorption_contract);
    assert!(!report.low_risk_runtime_promotion);
    assert!(report.requires_hepta_translation);
    assert!(!report.raw_upstream_doc_copy_allowed);
    assert!(!report.raw_upstream_package_policy_copy_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.contract_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_product_governance_absorption_tracks_exact_selected_paths() {
    let report = hepta_upstream_codex_product_governance_absorption_report();

    assert_eq!(report.selected_paths.len(), 22);
    assert!(
        report
            .selected_paths
            .iter()
            .all(|path| path.starts_with("codex-rs/"))
    );
    assert!(
        report
            .selected_paths
            .iter()
            .any(|path| path == "codex-rs/README.md")
    );
    assert!(
        report
            .selected_paths
            .iter()
            .any(|path| path == "codex-rs/Cargo.lock")
    );
    assert!(
        report
            .selected_paths
            .iter()
            .any(|path| path.contains("request_plugin_install"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("Hepta release-governance wording"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("active dependency isolation"))
    );
}

#[test]
fn upstream_codex_product_governance_translation_packet_is_ready_and_bounded() {
    let report = hepta_upstream_codex_product_governance_translation_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.translation_id,
        "upstream-codex-product-governance-translation-packet"
    );
    assert_eq!(
        report.translation_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_TRANSLATION.md"
    );
    assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
    assert_eq!(report.selected_changed_file_count, 22);
    assert_eq!(
        report.translated_surface_count,
        report.required_surface_count
    );
    assert_eq!(
        report.source_absorption_gate,
        "scripts/hepta-upstream-codex-product-governance-absorption.sh"
    );
    assert_eq!(
        report.translation_gate,
        "scripts/hepta-upstream-codex-product-governance-translation.sh"
    );
    assert!(report.release_governance_documented);
    assert!(report.package_policy_documented);
    assert!(report.plugin_marketplace_policy_documented);
    assert!(report.sandbox_runtime_policy_documented);
    assert!(report.operator_approval_policy_documented);
    assert!(report.requires_hepta_translation);
    assert!(!report.raw_upstream_doc_copy_allowed);
    assert!(!report.raw_upstream_package_policy_copy_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.translation_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_product_governance_translation_covers_hepta_actions() {
    let report = hepta_upstream_codex_product_governance_translation_report();

    assert!(
        report
            .hepta_actions
            .iter()
            .any(|action| action.contains("packaging governance"))
    );
    assert!(
        report
            .hepta_actions
            .iter()
            .any(|action| action.contains("route/gate language"))
    );
    assert!(
        report
            .hepta_actions
            .iter()
            .any(|action| action.contains("marketplace policy"))
    );
    assert!(
        report
            .hepta_actions
            .iter()
            .any(|action| action.contains("P0 security/runtime"))
    );
    assert!(
        report
            .hepta_actions
            .iter()
            .any(|action| action.contains("long soak"))
    );
}

#[test]
fn upstream_codex_release_governance_promotion_packet_is_ready_but_not_public() {
    let report = hepta_upstream_codex_release_governance_promotion_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.promotion_id,
        "release-governance-claim-promotion-packet"
    );
    assert_eq!(
        report.promotion_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_RELEASE_GOVERNANCE_PROMOTION.md"
    );
    assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
    assert_eq!(report.selected_changed_file_count, 22);
    assert_eq!(
        report.source_translation_gate,
        "scripts/hepta-upstream-codex-product-governance-translation.sh"
    );
    assert_eq!(
        report.promotion_gate,
        "scripts/hepta-upstream-codex-release-governance-promotion.sh"
    );
    assert!(report.release_claim_taxonomy_ready);
    assert!(report.package_install_context_ready);
    assert!(report.plugin_marketplace_policy_ready);
    assert!(report.operator_approval_model_ready);
    assert!(report.watchdog_soak_evidence_ready);
    assert!(report.public_claim_boundary_ready);
    assert!(report.side_effect_boundary_ready);
    assert_eq!(
        report.ready_promotion_condition_count,
        report.required_promotion_condition_count
    );
    assert!(report.promotion_packet_ready);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.upstream_auto_rebase_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_release_governance_promotion_tracks_claim_blockers() {
    let report = hepta_upstream_codex_release_governance_promotion_report();

    assert_eq!(report.promotion_conditions.len(), 7);
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("release claim taxonomy"))
    );
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("watchdog"))
    );
    assert!(
        report
            .remaining_blockers
            .iter()
            .any(|blocker| blocker.contains("public GA claim"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("operator approval"))
    );
}

#[test]
fn upstream_codex_legacy_compatibility_absorption_is_ready_and_bounded() {
    let report = hepta_upstream_codex_legacy_compatibility_absorption_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.absorption_id,
        "upstream-codex-legacy-compatibility-absorption-contract"
    );
    assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
    assert!(matches!(
        report.selected_bucket_risk,
        HeptaUpstreamCodexSyncRisk::P1Compatibility
    ));
    assert_eq!(report.selected_changed_file_count, 128);
    assert_eq!(
        report.source_ledger_gate,
        "scripts/hepta-upstream-codex-diff-ledger.sh"
    );
    assert_eq!(
        report.absorption_gate,
        "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh"
    );
    assert!(report.retained_as_compatibility_snapshot);
    assert!(report.requires_hepta_command_contract);
    assert!(!report.active_cli_tui_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.contract_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_legacy_compatibility_absorption_tracks_required_surfaces() {
    let report = hepta_upstream_codex_legacy_compatibility_absorption_report();

    assert!(
        report
            .sample_surfaces
            .iter()
            .any(|surface| surface.contains("cli"))
    );
    assert!(
        report
            .sample_surfaces
            .iter()
            .any(|surface| surface.contains("tui"))
    );
    assert!(
        report
            .sample_surfaces
            .iter()
            .any(|surface| surface.contains("code-mode"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("Hepta command contracts"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("shadow-replay"))
    );
}

#[test]
fn upstream_codex_legacy_compatibility_replay_packet_is_ready_and_bounded() {
    let report = hepta_upstream_codex_legacy_compatibility_replay_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.replay_id,
        "upstream-codex-legacy-compatibility-replay-packet"
    );
    assert_eq!(
        report.replay_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_REPLAY.md"
    );
    assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
    assert_eq!(report.selected_changed_file_count, 128);
    assert_eq!(
        report.replay_surface_count,
        report.required_replay_surface_count
    );
    assert_eq!(
        report.source_absorption_gate,
        "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh"
    );
    assert_eq!(
        report.replay_gate,
        "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh"
    );
    assert!(report.cli_command_contract_ready);
    assert!(report.tui_presentation_replay_ready);
    assert!(report.code_mode_replay_ready);
    assert!(report.terminal_helper_replay_ready);
    assert!(report.dependency_boundary_ready);
    assert!(!report.active_cli_tui_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.replay_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_legacy_compatibility_replay_tracks_replay_surfaces() {
    let report = hepta_upstream_codex_legacy_compatibility_replay_report();

    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("CLI command"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("TUI"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("code-mode"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("terminal"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("Hepta command contracts"))
    );
}

#[test]
fn upstream_codex_legacy_compatibility_promotion_packet_is_ready_but_not_active() {
    let report = hepta_upstream_codex_legacy_compatibility_promotion_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.promotion_id, "hepta-cli-tui-parity-promotion-packet");
    assert_eq!(
        report.promotion_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_PROMOTION.md"
    );
    assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
    assert_eq!(report.selected_changed_file_count, 128);
    assert_eq!(
        report.source_replay_gate,
        "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh"
    );
    assert_eq!(
        report.promotion_gate,
        "scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh"
    );
    assert!(report.cli_command_contract_parity_ready);
    assert!(report.tui_presentation_parity_ready);
    assert!(report.code_mode_callback_boundary_ready);
    assert!(report.terminal_helper_contract_ready);
    assert!(report.adapter_shadow_replay_ready);
    assert!(report.operator_approval_model_ready);
    assert!(report.side_effect_boundary_ready);
    assert_eq!(
        report.ready_promotion_condition_count,
        report.required_promotion_condition_count
    );
    assert!(report.promotion_packet_ready);
    assert!(!report.active_cli_tui_promotion_allowed);
    assert!(!report.active_tui_presentation_promotion_allowed);
    assert!(!report.active_code_mode_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_legacy_compatibility_promotion_tracks_blockers() {
    let report = hepta_upstream_codex_legacy_compatibility_promotion_report();

    assert_eq!(report.promotion_conditions.len(), 7);
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("CLI command"))
    );
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("TUI presentation"))
    );
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("code-mode"))
    );
    assert!(
        report
            .remaining_blockers
            .iter()
            .any(|blocker| blocker.contains("active CLI/TUI"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("Hepta-native CLI/TUI parity"))
    );
}

#[test]
fn upstream_codex_provider_security_absorption_is_ready_and_bounded() {
    let report = hepta_upstream_codex_provider_security_absorption_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.absorption_id,
        "upstream-codex-provider-security-absorption-contract"
    );
    assert_eq!(
        report.selected_bucket_id,
        "provider-credential-sandbox-security"
    );
    assert!(matches!(
        report.selected_bucket_risk,
        HeptaUpstreamCodexSyncRisk::P0Security
    ));
    assert_eq!(report.selected_changed_file_count, 104);
    assert_eq!(
        report.selected_security_surface_count,
        report.required_security_surface_count
    );
    assert_eq!(
        report.source_ledger_gate,
        "scripts/hepta-upstream-codex-diff-ledger.sh"
    );
    assert_eq!(
        report.absorption_gate,
        "scripts/hepta-upstream-codex-provider-security-absorption.sh"
    );
    assert!(report.p0_security_review_required);
    assert!(report.requires_provider_contract);
    assert!(report.requires_auth_credential_redaction);
    assert!(report.requires_sandbox_exec_replay);
    assert!(report.requires_network_policy_replay);
    assert!(!report.active_provider_promotion_allowed);
    assert!(!report.active_security_policy_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.contract_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_provider_security_absorption_tracks_required_surfaces() {
    let report = hepta_upstream_codex_provider_security_absorption_report();

    assert!(
        report
            .security_surfaces
            .iter()
            .any(|surface| surface.contains("provider"))
    );
    assert!(
        report
            .security_surfaces
            .iter()
            .any(|surface| surface.contains("credential"))
    );
    assert!(
        report
            .security_surfaces
            .iter()
            .any(|surface| surface.contains("sandbox"))
    );
    assert!(
        report
            .security_surfaces
            .iter()
            .any(|surface| surface.contains("network"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("redacted provider contracts"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("network-proxy policy replay"))
    );
}

#[test]
fn upstream_codex_provider_security_replay_packet_is_ready_and_bounded() {
    let report = hepta_upstream_codex_provider_security_replay_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.replay_id,
        "upstream-codex-provider-security-replay-packet"
    );
    assert_eq!(
        report.replay_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_REPLAY.md"
    );
    assert_eq!(
        report.selected_bucket_id,
        "provider-credential-sandbox-security"
    );
    assert_eq!(report.selected_changed_file_count, 104);
    assert_eq!(
        report.replay_surface_count,
        report.required_replay_surface_count
    );
    assert_eq!(
        report.source_absorption_gate,
        "scripts/hepta-upstream-codex-provider-security-absorption.sh"
    );
    assert_eq!(
        report.replay_gate,
        "scripts/hepta-upstream-codex-provider-security-replay.sh"
    );
    assert!(report.redacted_provider_contract_ready);
    assert!(report.auth_credential_redaction_ready);
    assert!(report.approval_policy_replay_ready);
    assert!(report.sandbox_exec_replay_ready);
    assert!(report.network_policy_replay_ready);
    assert!(report.side_effect_boundary_ready);
    assert!(!report.active_provider_promotion_allowed);
    assert!(!report.active_security_policy_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.replay_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_provider_security_replay_tracks_replay_surfaces() {
    let report = hepta_upstream_codex_provider_security_replay_report();

    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("provider"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("credential"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("approval"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("sandbox"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("network"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("credential values"))
    );
}

#[test]
fn upstream_codex_provider_security_promotion_packet_is_ready_but_not_active() {
    let report = hepta_upstream_codex_provider_security_promotion_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.promotion_id,
        "upstream-codex-provider-security-promotion-packet"
    );
    assert_eq!(
        report.promotion_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_PROMOTION.md"
    );
    assert_eq!(
        report.selected_bucket_id,
        "provider-credential-sandbox-security"
    );
    assert_eq!(report.selected_changed_file_count, 104);
    assert_eq!(
        report.source_replay_gate,
        "scripts/hepta-upstream-codex-provider-security-replay.sh"
    );
    assert_eq!(
        report.promotion_gate,
        "scripts/hepta-upstream-codex-provider-security-promotion.sh"
    );
    assert_eq!(
        report.ready_promotion_condition_count,
        report.required_promotion_condition_count
    );
    assert!(report.redacted_provider_contract_ready);
    assert!(report.auth_credential_redaction_ready);
    assert!(report.approval_policy_replay_ready);
    assert!(report.sandbox_exec_replay_ready);
    assert!(report.network_policy_replay_ready);
    assert!(report.operator_approval_model_ready);
    assert!(report.side_effect_boundary_ready);
    assert!(report.promotion_packet_ready);
    assert!(!report.active_provider_promotion_allowed);
    assert!(!report.active_security_policy_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_provider_security_promotion_tracks_blockers() {
    let report = hepta_upstream_codex_provider_security_promotion_report();

    assert_eq!(report.promotion_conditions.len(), 7);
    assert_eq!(report.remaining_blockers.len(), 4);
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("network policy"))
    );
    assert!(
        report
            .remaining_blockers
            .iter()
            .any(|blocker| blocker.contains("credential reads"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("adapter parity"))
    );
}

#[test]
fn upstream_codex_runtime_appserver_absorption_is_ready_and_bounded() {
    let report = hepta_upstream_codex_runtime_appserver_absorption_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.absorption_id,
        "upstream-codex-runtime-appserver-absorption-contract"
    );
    assert_eq!(
        report.selected_bucket_id,
        "runtime-session-tool-mcp-appserver"
    );
    assert!(matches!(
        report.selected_bucket_risk,
        HeptaUpstreamCodexSyncRisk::P0Runtime
    ));
    assert_eq!(report.selected_changed_file_count, 462);
    assert_eq!(
        report.selected_runtime_surface_count,
        report.required_runtime_surface_count
    );
    assert_eq!(
        report.source_ledger_gate,
        "scripts/hepta-upstream-codex-diff-ledger.sh"
    );
    assert_eq!(
        report.absorption_gate,
        "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh"
    );
    assert!(report.p0_runtime_review_required);
    assert!(report.requires_adapter_contract);
    assert!(report.requires_session_thread_replay);
    assert!(report.requires_tool_mcp_replay);
    assert!(report.requires_app_server_protocol_replay);
    assert!(report.requires_exec_hook_replay);
    assert!(!report.active_runtime_promotion_allowed);
    assert!(!report.active_app_server_promotion_allowed);
    assert!(!report.active_tool_mcp_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.contract_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_runtime_appserver_absorption_tracks_required_surfaces() {
    let report = hepta_upstream_codex_runtime_appserver_absorption_report();

    assert!(
        report
            .runtime_surfaces
            .iter()
            .any(|surface| surface.contains("app-server"))
    );
    assert!(
        report
            .runtime_surfaces
            .iter()
            .any(|surface| surface.contains("session"))
    );
    assert!(
        report
            .runtime_surfaces
            .iter()
            .any(|surface| surface.contains("tool"))
    );
    assert!(
        report
            .runtime_surfaces
            .iter()
            .any(|surface| surface.contains("MCP"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("thread-store replay"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("tool and MCP replay"))
    );
}

#[test]
fn upstream_codex_runtime_appserver_replay_packet_is_ready_and_bounded() {
    let report = hepta_upstream_codex_runtime_appserver_replay_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.replay_id,
        "upstream-codex-runtime-appserver-replay-packet"
    );
    assert_eq!(
        report.replay_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_REPLAY.md"
    );
    assert_eq!(
        report.selected_bucket_id,
        "runtime-session-tool-mcp-appserver"
    );
    assert_eq!(report.selected_changed_file_count, 462);
    assert_eq!(
        report.replay_surface_count,
        report.required_replay_surface_count
    );
    assert_eq!(
        report.source_absorption_gate,
        "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh"
    );
    assert_eq!(
        report.replay_gate,
        "scripts/hepta-upstream-codex-runtime-appserver-replay.sh"
    );
    assert!(report.app_server_protocol_replay_ready);
    assert!(report.session_thread_replay_ready);
    assert!(report.tool_mcp_replay_ready);
    assert!(report.exec_hook_replay_ready);
    assert!(report.side_effect_boundary_ready);
    assert!(!report.active_runtime_promotion_allowed);
    assert!(!report.active_app_server_promotion_allowed);
    assert!(!report.active_tool_mcp_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(report.replay_ready);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
}

#[test]
fn upstream_codex_runtime_appserver_replay_tracks_replay_surfaces() {
    let report = hepta_upstream_codex_runtime_appserver_replay_report();

    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("app-server protocol"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("session"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("tool"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("MCP"))
    );
    assert!(
        report
            .replay_surfaces
            .iter()
            .any(|surface| surface.contains("exec-server"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("event-loop promotion"))
    );
}

#[test]
fn upstream_codex_runtime_appserver_promotion_packet_is_ready_but_not_active() {
    let report = hepta_upstream_codex_runtime_appserver_promotion_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.promotion_id,
        "runtime-appserver-route-event-promotion-packet"
    );
    assert_eq!(
        report.promotion_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_PROMOTION.md"
    );
    assert_eq!(
        report.selected_bucket_id,
        "runtime-session-tool-mcp-appserver"
    );
    assert_eq!(report.selected_changed_file_count, 462);
    assert_eq!(
        report.source_replay_gate,
        "scripts/hepta-upstream-codex-runtime-appserver-replay.sh"
    );
    assert_eq!(
        report.promotion_gate,
        "scripts/hepta-upstream-codex-runtime-appserver-promotion.sh"
    );
    assert_eq!(
        report.ready_promotion_condition_count,
        report.required_promotion_condition_count
    );
    assert!(report.app_server_route_event_contract_ready);
    assert!(report.session_thread_lifecycle_contract_ready);
    assert!(report.tool_mcp_request_envelope_ready);
    assert!(report.exec_hook_event_loop_replay_ready);
    assert!(report.adapter_shadow_replay_ready);
    assert!(report.operator_approval_model_ready);
    assert!(report.side_effect_boundary_ready);
    assert!(report.promotion_packet_ready);
    assert!(!report.active_runtime_promotion_allowed);
    assert!(!report.active_app_server_promotion_allowed);
    assert!(!report.active_tool_mcp_promotion_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_runtime_appserver_promotion_tracks_blockers() {
    let report = hepta_upstream_codex_runtime_appserver_promotion_report();

    assert_eq!(report.promotion_conditions.len(), 7);
    assert_eq!(report.remaining_blockers.len(), 4);
    assert!(
        report
            .promotion_conditions
            .iter()
            .any(|condition| condition.contains("route and event contract"))
    );
    assert!(
        report
            .remaining_blockers
            .iter()
            .any(|blocker| blocker.contains("gateway RPC"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("route/event adapter parity"))
    );
}

#[test]
fn upstream_codex_absorption_replay_readiness_is_ready_and_bounded() {
    let report = hepta_upstream_codex_absorption_replay_readiness_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.readiness_id,
        "upstream-codex-absorption-replay-readiness"
    );
    assert_eq!(
        report.readiness_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ABSORPTION_REPLAY_READINESS.md"
    );
    assert_eq!(report.ledger_changed_file_count, 878);
    assert_eq!(report.selected_absorption_changed_file_count, 716);
    assert_eq!(
        report.selected_bucket_count,
        report.required_selected_bucket_count
    );
    assert_eq!(
        report.absorption_contract_ready_count,
        report.required_absorption_contract_ready_count
    );
    assert_eq!(
        report.translation_replay_ready_count,
        report.required_translation_replay_ready_count
    );
    assert_eq!(
        report.p0_replay_ready_count,
        report.required_p0_replay_ready_count
    );
    assert_eq!(
        report.p1_replay_ready_count,
        report.required_p1_replay_ready_count
    );
    assert_eq!(
        report.p2_translation_ready_count,
        report.required_p2_translation_ready_count
    );
    assert!(report.product_governance_translation_ready);
    assert!(report.legacy_compatibility_replay_ready);
    assert!(report.provider_security_replay_ready);
    assert!(report.runtime_appserver_replay_ready);
    assert!(report.all_selected_buckets_absorbed);
    assert!(report.all_required_translation_replay_ready);
    assert!(report.readiness_ready);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_absorption_replay_readiness_tracks_all_closed_gates() {
    let report = hepta_upstream_codex_absorption_replay_readiness_report();

    assert_eq!(report.covered_buckets.len(), 4);
    assert!(
        report
            .covered_buckets
            .iter()
            .any(|bucket| bucket == "product-doc-release-governance")
    );
    assert!(
        report
            .covered_buckets
            .iter()
            .any(|bucket| bucket == "legacy-cli-tui-compatibility")
    );
    assert!(
        report
            .covered_buckets
            .iter()
            .any(|bucket| bucket == "provider-credential-sandbox-security")
    );
    assert!(
        report
            .covered_buckets
            .iter()
            .any(|bucket| bucket == "runtime-session-tool-mcp-appserver")
    );
    assert_eq!(report.closed_gates.len(), 8);
    assert!(
        report
            .closed_gates
            .iter()
            .any(|gate| gate.contains("product-governance-translation"))
    );
    assert!(
        report
            .closed_gates
            .iter()
            .any(|gate| gate.contains("legacy-compatibility-replay"))
    );
    assert!(
        report
            .closed_gates
            .iter()
            .any(|gate| gate.contains("provider-security-replay"))
    );
    assert!(
        report
            .closed_gates
            .iter()
            .any(|gate| gate.contains("runtime-appserver-replay"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("newer upstream Codex range"))
    );
}

#[test]
fn upstream_codex_promotion_readiness_is_decided_but_not_open() {
    let report = hepta_upstream_codex_promotion_readiness_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.decision_id, "upstream-codex-promotion-readiness");
    assert_eq!(
        report.decision_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_READINESS.md"
    );
    assert_eq!(
        report.source_readiness_gate,
        "scripts/hepta-upstream-codex-absorption-replay-readiness.sh"
    );
    assert_eq!(
        report.promotion_readiness_gate,
        "scripts/hepta-upstream-codex-promotion-readiness.sh"
    );
    assert_eq!(
        report.assessed_bucket_count,
        report.required_assessed_bucket_count
    );
    assert_eq!(
        report.absorption_replay_ready_count,
        report.required_absorption_replay_ready_count
    );
    assert_eq!(report.required_surface_promotion_packet_count, 4);
    assert_eq!(report.completed_surface_promotion_packet_count, 4);
    assert_eq!(report.promotable_bucket_count, 0);
    assert_eq!(report.promotion_blocked_bucket_count, 4);
    assert!(report.readiness_source_ready);
    assert!(report.decision_ready);
    assert!(!report.active_promotion_ready);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
}

#[test]
fn upstream_codex_promotion_readiness_blocks_all_selected_buckets() {
    let report = hepta_upstream_codex_promotion_readiness_report();

    assert_eq!(report.decisions.len(), 4);
    assert_eq!(report.promotion_blockers.len(), 4);
    assert!(report.decisions.iter().all(|decision| {
        decision.absorption_replay_ready && !decision.active_promotion_allowed
    }));
    assert!(report.decisions.iter().any(|decision| decision.bucket_id
        == "provider-credential-sandbox-security"
        && decision.risk == HeptaUpstreamCodexSyncRisk::P0Security
        && decision.surface_promotion_packet_ready));
    assert!(report.decisions.iter().any(|decision| decision.bucket_id
        == "product-doc-release-governance"
        && decision.risk == HeptaUpstreamCodexSyncRisk::P2Product
        && decision.surface_promotion_packet_ready));
    assert!(report.decisions.iter().any(|decision| decision.bucket_id
        == "runtime-session-tool-mcp-appserver"
        && decision.risk == HeptaUpstreamCodexSyncRisk::P0Runtime
        && decision.surface_promotion_packet_ready));
    assert!(report.decisions.iter().any(|decision| {
        decision.bucket_id == "legacy-cli-tui-compatibility"
            && decision
                .required_surface_promotion_packet
                .contains("parity")
            && decision.surface_promotion_packet_ready
    }));
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("per-surface promotion packets"))
    );
}

#[test]
fn upstream_codex_promotion_closure_completes_packets_but_denies_activation() {
    let report = hepta_upstream_codex_promotion_closure_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(report.closure_id, "upstream-codex-promotion-closure-denial");
    assert_eq!(
        report.closure_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_CLOSURE.md"
    );
    assert_eq!(
        report.source_promotion_readiness_gate,
        "scripts/hepta-upstream-codex-promotion-readiness.sh"
    );
    assert_eq!(
        report.closure_gate,
        "scripts/hepta-upstream-codex-promotion-closure.sh"
    );
    assert_eq!(report.required_surface_promotion_packet_count, 4);
    assert_eq!(report.completed_surface_promotion_packet_count, 4);
    assert!(report.all_surface_promotion_packets_complete);
    assert_eq!(report.promotable_bucket_count, 0);
    assert_eq!(report.promotion_blocked_bucket_count, 4);
    assert!(!report.active_promotion_ready);
    assert!(report.active_promotion_denial_ready);
    assert!(report.closure_ready);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
}

#[test]
fn upstream_codex_promotion_closure_preserves_side_effect_boundaries() {
    let report = hepta_upstream_codex_promotion_closure_report();

    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert_eq!(report.closure_invariants.len(), 5);
    assert!(report.closure_invariants.iter().any(|invariant| {
        invariant.contains("all four required surface promotion packets are complete")
    }));
    assert!(report.closure_invariants.iter().any(|invariant| {
        invariant.contains("zero selected upstream Codex buckets are promotable")
    }));
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("explicit operator approval before active runtime wiring"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("newer upstream Codex ranges as new snapshot intake"))
    );
}

#[test]
fn upstream_codex_active_wiring_precondition_is_ready_but_not_allowed() {
    let report = hepta_upstream_codex_active_wiring_precondition_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.precondition_id,
        "upstream-codex-active-wiring-precondition"
    );
    assert_eq!(
        report.precondition_packet_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVE_WIRING_PRECONDITION.md"
    );
    assert_eq!(
        report.source_closure_gate,
        "scripts/hepta-upstream-codex-promotion-closure.sh"
    );
    assert_eq!(
        report.active_wiring_precondition_gate,
        "scripts/hepta-upstream-codex-active-wiring-precondition.sh"
    );
    assert!(report.promotion_closure_ready);
    assert!(report.all_surface_promotion_packets_complete);
    assert!(report.active_promotion_denial_ready);
    assert!(report.explicit_operator_approval_required);
    assert!(!report.operator_approval_recorded);
    assert!(report.activation_request_id_required);
    assert!(!report.activation_request_id_present);
    assert!(report.live_dependency_isolation_required);
    assert!(report.watchdog_required);
    assert!(report.browser_smoke_required);
    assert!(report.long_soak_required);
    assert!(report.active_wiring_precondition_ready);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
}

#[test]
fn upstream_codex_active_wiring_precondition_has_no_side_effects() {
    let report = hepta_upstream_codex_active_wiring_precondition_report();

    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .preconditions
            .iter()
            .any(|precondition| { precondition.contains("operator approval record is required") })
    );
    assert!(
        report
            .preconditions
            .iter()
            .any(|precondition| { precondition.contains("activation request id is required") })
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("activation request packet schema"))
    );
}

#[test]
fn upstream_codex_activation_request_packet_schema_is_ready_but_unrecorded() {
    let report = hepta_upstream_codex_activation_request_packet_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.packet_id,
        "upstream-codex-activation-request-packet-schema"
    );
    assert_eq!(
        report.packet_schema_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_REQUEST_PACKET.md"
    );
    assert_eq!(
        report.source_precondition_gate,
        "scripts/hepta-upstream-codex-active-wiring-precondition.sh"
    );
    assert_eq!(
        report.activation_request_packet_gate,
        "scripts/hepta-upstream-codex-activation-request-packet.sh"
    );
    assert!(report.active_wiring_precondition_ready);
    assert!(!report.active_wiring_allowed_by_precondition);
    assert!(report.operator_approval_required);
    assert!(!report.operator_approval_recorded);
    assert!(report.activation_request_id_required);
    assert!(!report.activation_request_id_recorded);
    assert_eq!(report.schema_field_count, 14);
    assert_eq!(
        report.required_schema_field_count,
        report.schema_field_count
    );
    assert_eq!(report.recorded_required_schema_field_count, 0);
    assert!(report.activation_packet_schema_ready);
    assert!(!report.activation_packet_recorded);
    assert!(!report.active_wiring_allowed);
    assert!(report.schema_fields.iter().any(|field| {
        field.name == "activation_request_id" && field.required && !field.recorded
    }));
    assert!(
        report
            .schema_fields
            .iter()
            .any(|field| { field.name == "operator_identity_hash" && field.redacted_or_hashed })
    );
    assert!(
        report
            .schema_fields
            .iter()
            .any(|field| { field.name == "live_dependency_isolation_evidence_id" })
    );
    assert!(
        report
            .schema_fields
            .iter()
            .any(|field| { field.name == "release_artifact_write_decision" })
    );
}

#[test]
fn upstream_codex_activation_request_packet_preserves_denials_and_side_effects() {
    let report = hepta_upstream_codex_activation_request_packet_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .packet_invariants
            .iter()
            .any(|invariant| invariant.contains("no activation packet is recorded"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("concrete activation_request_id"))
    );
}

#[test]
fn upstream_codex_activation_packet_dry_run_blocks_incomplete_fixtures() {
    let report = hepta_upstream_codex_activation_packet_dry_run_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.validator_id,
        "upstream-codex-activation-packet-dry-run-validator"
    );
    assert_eq!(
        report.validator_doc_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_PACKET_DRY_RUN.md"
    );
    assert_eq!(
        report.source_packet_gate,
        "scripts/hepta-upstream-codex-activation-request-packet.sh"
    );
    assert_eq!(
        report.dry_run_validator_gate,
        "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
    );
    assert!(report.activation_packet_schema_ready);
    assert!(!report.activation_packet_recorded);
    assert_eq!(report.required_schema_field_count, 14);
    assert_eq!(report.fixture_count, 3);
    assert_eq!(report.blocked_fixture_count, report.fixture_count);
    assert_eq!(report.allowed_fixture_count, 0);
    assert!(report.dry_run_validator_ready);
    assert!(!report.active_wiring_allowed);

    let empty = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_id == "empty-placeholder")
        .expect("empty placeholder fixture");
    assert_eq!(empty.recorded_required_field_count, 0);
    assert_eq!(empty.missing_required_field_count, 14);
    assert!(!empty.active_wiring_allowed);

    let public_attempt = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_id == "public-claim-attempt-without-evidence")
        .expect("public claim attempt fixture");
    assert!(public_attempt.public_release_claim_requested);
    assert!(public_attempt.release_artifact_write_requested);
    assert!(!public_attempt.public_release_claim_allowed);
    assert!(!public_attempt.release_artifact_write_allowed);
}

#[test]
fn upstream_codex_activation_packet_dry_run_preserves_denials_and_side_effects() {
    let report = hepta_upstream_codex_activation_packet_dry_run_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .validation_invariants
            .iter()
            .any(|invariant| { invariant.contains("dry-run fixtures cannot activate wiring") })
    );
    assert!(
        report
            .validation_invariants
            .iter()
            .any(|invariant| invariant.contains("public release and artifact-write"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("concrete activation packet"))
    );
}

#[test]
fn upstream_codex_activation_evidence_ledger_is_ready_but_empty() {
    let report = hepta_upstream_codex_activation_evidence_ledger_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.ledger_id,
        "upstream-codex-activation-evidence-ledger-checklist"
    );
    assert_eq!(
        report.ledger_doc_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_LEDGER.md"
    );
    assert_eq!(
        report.source_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
    );
    assert_eq!(
        report.evidence_ledger_gate,
        "scripts/hepta-upstream-codex-activation-evidence-ledger.sh"
    );
    assert!(report.dry_run_validator_ready);
    assert!(!report.activation_packet_recorded);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.recorded_evidence_count, 0);
    assert_eq!(report.fresh_evidence_count, 0);
    assert!(report.evidence_ledger_ready);
    assert!(!report.evidence_recorded);
    assert!(!report.active_wiring_allowed);
    assert!(
        report
            .evidence_requirements
            .iter()
            .all(|requirement| requirement.required && !requirement.recorded && !requirement.fresh)
    );
    assert!(report.evidence_requirements.iter().any(|requirement| {
        requirement.id == "live_dependency_isolation_evidence_id"
            && requirement.source_gate == "scripts/hepta-active-service-dependency-isolation.sh"
    }));
    assert!(
        report
            .evidence_requirements
            .iter()
            .any(|requirement| requirement.id == "rollback_plan_id")
    );
}

#[test]
fn upstream_codex_activation_evidence_ledger_preserves_denials_and_side_effects() {
    let report = hepta_upstream_codex_activation_evidence_ledger_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .ledger_invariants
            .iter()
            .any(|invariant| invariant.contains("records no concrete evidence"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("concrete activation request id"))
    );
}

#[test]
fn upstream_codex_activation_readiness_closure_is_ready_and_denied() {
    let report = hepta_upstream_codex_activation_readiness_closure_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.closure_id,
        "upstream-codex-activation-readiness-closure-denial"
    );
    assert_eq!(
        report.closure_doc_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_READINESS_CLOSURE.md"
    );
    assert_eq!(
        report.source_packet_gate,
        "scripts/hepta-upstream-codex-activation-request-packet.sh"
    );
    assert_eq!(
        report.source_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
    );
    assert_eq!(
        report.source_evidence_ledger_gate,
        "scripts/hepta-upstream-codex-activation-evidence-ledger.sh"
    );
    assert_eq!(
        report.activation_readiness_closure_gate,
        "scripts/hepta-upstream-codex-activation-readiness-closure.sh"
    );
    assert!(report.activation_packet_schema_ready);
    assert!(report.dry_run_validator_ready);
    assert!(report.evidence_ledger_ready);
    assert_eq!(report.required_schema_field_count, 14);
    assert_eq!(report.blocked_fixture_count, 3);
    assert_eq!(report.allowed_fixture_count, 0);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.recorded_evidence_count, 0);
    assert_eq!(report.fresh_evidence_count, 0);
    assert!(report.readiness_inputs_ready);
    assert!(report.activation_denied_by_default);
    assert!(report.activation_readiness_closure_ready);
    assert!(!report.operator_approved_activation_ready);
    assert!(!report.activation_packet_recorded);
    assert!(!report.evidence_recorded);
    assert!(!report.active_wiring_allowed);
}

#[test]
fn upstream_codex_activation_readiness_closure_preserves_denials_and_side_effects() {
    let report = hepta_upstream_codex_activation_readiness_closure_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .closure_invariants
            .iter()
            .any(|invariant| invariant.contains("denied"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("fresh live gate evidence"))
    );
}

#[test]
fn upstream_codex_activation_denied_sample_is_full_shaped_but_blocked() {
    let report = hepta_upstream_codex_activation_denied_sample_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.sample_id,
        "upstream-codex-activation-denied-sample-packet"
    );
    assert_eq!(
        report.sample_doc_path,
        "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md"
    );
    assert_eq!(
        report.source_readiness_closure_gate,
        "scripts/hepta-upstream-codex-activation-readiness-closure.sh"
    );
    assert_eq!(
        report.denied_sample_gate,
        "scripts/hepta-upstream-codex-activation-denied-sample.sh"
    );
    assert!(report.activation_readiness_closure_ready);
    assert!(report.sample_packet_shape_complete);
    assert_eq!(report.sample_required_schema_field_count, 14);
    assert_eq!(report.sample_recorded_schema_field_count, 14);
    assert_eq!(report.sample_required_evidence_count, 8);
    assert_eq!(report.sample_fresh_evidence_count, 0);
    assert!(report.sample_operator_approval_field_present);
    assert!(!report.sample_operator_approval_recorded);
    assert!(report.sample_public_release_claim_requested);
    assert!(report.sample_release_artifact_write_requested);
    assert_eq!(report.sample_validation_status, "blocked");
    assert!(report.sample_blocked_reason.contains("not recorded"));
    assert!(!report.active_wiring_allowed);
}

#[test]
fn upstream_codex_activation_denied_sample_preserves_denials_and_side_effects() {
    let report = hepta_upstream_codex_activation_denied_sample_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .sample_invariants
            .iter()
            .any(|invariant| invariant.contains("not approvals"))
    );
    assert!(
        report
            .required_next_gates
            .iter()
            .any(|gate| gate.contains("concrete operator-approved activation packet"))
    );
}

#[test]
fn upstream_codex_activation_evidence_freshness_policy_defines_all_slots() {
    let report = hepta_upstream_codex_activation_evidence_freshness_policy_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.policy_id,
        "upstream-codex-activation-evidence-freshness-policy"
    );
    assert_eq!(
        report.source_denied_sample_gate,
        "scripts/hepta-upstream-codex-activation-denied-sample.sh"
    );
    assert_eq!(
        report.freshness_policy_gate,
        "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh"
    );
    assert!(report.denied_sample_ready);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.policy_entry_count, 8);
    assert_eq!(report.missing_evidence_count, 8);
    assert_eq!(report.fresh_evidence_count, 0);
    assert_eq!(report.expired_evidence_count, 0);
    assert_eq!(report.stale_evidence_count, 0);
    assert!(report.freshness_policy_ready);
    assert!(report.activation_blocked_by_freshness_policy);
    assert!(!report.activation_allowed_by_freshness_policy);
    assert!(!report.active_wiring_allowed);
    assert_eq!(report.freshness_entries.len(), 8);

    let ids: Vec<_> = report
        .freshness_entries
        .iter()
        .map(|entry| entry.evidence_id.as_str())
        .collect();
    assert!(ids.contains(&"activation_request_id"));
    assert!(ids.contains(&"operator_approval_id"));
    assert!(ids.contains(&"operator_identity_hash"));
    assert!(ids.contains(&"live_dependency_isolation_evidence_id"));
    assert!(ids.contains(&"watchdog_evidence_id"));
    assert!(ids.contains(&"browser_smoke_evidence_id"));
    assert!(ids.contains(&"long_soak_evidence_id"));
    assert!(ids.contains(&"rollback_plan_id"));
}

#[test]
fn upstream_codex_activation_evidence_freshness_policy_preserves_denials() {
    let report = hepta_upstream_codex_activation_evidence_freshness_policy_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .freshness_denial_reason
            .contains("evidence slots are absent")
    );
    assert!(report.freshness_entries.iter().all(|entry| {
        entry.required_for_activation
            && !entry.recorded
            && !entry.fresh
            && entry.denial_reason.contains("absent")
    }));
    assert!(
        report
            .policy_invariants
            .iter()
            .any(|invariant| invariant.contains("records no evidence"))
    );
}

#[test]
fn upstream_codex_activation_evidence_binding_record_manifest_defines_schema() {
    let report = hepta_upstream_codex_activation_evidence_binding_record_manifest_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.manifest_id,
        "upstream-codex-activation-evidence-binding-record-manifest"
    );
    assert_eq!(
        report.source_freshness_policy_gate,
        "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh"
    );
    assert_eq!(
        report.binding_manifest_gate,
        "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh"
    );
    assert!(report.freshness_policy_ready);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.binding_record_count, 8);
    assert_eq!(report.missing_binding_record_count, 8);
    assert_eq!(report.recorded_binding_record_count, 0);
    assert_eq!(report.required_record_schema_field_count, 7);
    assert_eq!(report.recorded_record_schema_field_count, 0);
    assert_eq!(report.timestamped_record_count, 0);
    assert_eq!(report.binary_sha_bound_record_count, 0);
    assert_eq!(report.route_or_status_hash_bound_record_count, 0);
    assert_eq!(report.artifact_hash_or_redacted_path_bound_record_count, 0);
    assert_eq!(report.activation_request_id_bound_record_count, 0);
    assert!(report.binding_manifest_ready);
    assert!(report.activation_blocked_by_binding_manifest);
    assert!(!report.activation_allowed_by_binding_manifest);
    assert!(!report.active_wiring_allowed);

    let field_names: Vec<_> = report
        .binding_schema_fields
        .iter()
        .map(|field| field.name.as_str())
        .collect();
    assert!(field_names.contains(&"evidence_record_id"));
    assert!(field_names.contains(&"source_gate"));
    assert!(field_names.contains(&"recorded_at_unix_ms"));
    assert!(field_names.contains(&"active_binary_sha256"));
    assert!(field_names.contains(&"route_or_status_hash"));
    assert!(field_names.contains(&"artifact_sha256_or_redacted_path"));
    assert!(field_names.contains(&"activation_request_id_binding"));
}

#[test]
fn upstream_codex_activation_evidence_binding_record_manifest_preserves_denials() {
    let report = hepta_upstream_codex_activation_evidence_binding_record_manifest_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .binding_denial_reason
            .contains("schema-only and unrecorded")
    );
    assert!(report.binding_records.iter().all(|record| {
        record.required_schema_field_count == 7
            && record.recorded_schema_field_count == 0
            && !record.evidence_recorded
            && !record.timestamp_recorded
            && !record.active_binary_sha_bound
            && !record.route_or_status_hash_bound
            && !record.artifact_hash_or_redacted_path_bound
            && !record.activation_request_id_bound
            && record.binding_denial_reason.contains("not recorded")
    }));
    assert!(
        report
            .binding_invariants
            .iter()
            .any(|invariant| invariant.contains("without recording evidence"))
    );
}

#[test]
fn upstream_codex_activation_evidence_record_denied_fixture_is_full_shaped_but_blocked() {
    let report = hepta_upstream_codex_activation_evidence_record_denied_fixture_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.fixture_id,
        "upstream-codex-activation-evidence-record-denied-fixture"
    );
    assert_eq!(
        report.source_binding_manifest_gate,
        "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh"
    );
    assert_eq!(
        report.denied_fixture_gate,
        "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh"
    );
    assert!(report.binding_manifest_ready);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.fixture_record_count, 8);
    assert_eq!(report.schema_complete_fixture_record_count, 8);
    assert_eq!(report.trusted_fixture_record_count, 0);
    assert_eq!(report.operator_approved_fixture_record_count, 0);
    assert_eq!(report.request_binding_verified_record_count, 0);
    assert_eq!(report.live_gate_hash_verified_record_count, 0);
    assert_eq!(report.artifact_hash_verified_record_count, 0);
    assert_eq!(report.fresh_fixture_record_count, 0);
    assert_eq!(report.blocked_fixture_record_count, 8);
    assert_eq!(report.allowed_fixture_record_count, 0);
    assert!(report.denied_fixture_ready);
    assert!(report.activation_blocked_by_denied_fixture);
    assert!(!report.activation_allowed_by_denied_fixture);
    assert!(!report.active_wiring_allowed);
    assert!(report.fixture_records.iter().all(|record| {
        record.schema_complete
            && record.validation_status == "blocked"
            && record.evidence_record_id.starts_with("fixture-")
            && record.recorded_at_unix_ms == "0"
            && record.active_binary_sha256.contains("placeholder")
            && record.route_or_status_hash.contains("placeholder")
            && record
                .artifact_sha256_or_redacted_path
                .contains("placeholder")
            && record.activation_request_id_binding.contains("placeholder")
    }));
}

#[test]
fn upstream_codex_activation_evidence_record_denied_fixture_preserves_denials() {
    let report = hepta_upstream_codex_activation_evidence_record_denied_fixture_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .fixture_denial_reason
            .contains("placeholders without operator approval")
    );
    assert!(report.fixture_records.iter().all(|record| {
        !record.operator_approved
            && !record.request_binding_verified
            && !record.live_gate_hash_verified
            && !record.artifact_hash_verified
            && !record.freshness_window_satisfied
            && !record.trusted
            && record.denial_reason.contains("placeholder evidence")
    }));
    assert!(
        report
            .fixture_invariants
            .iter()
            .any(|invariant| invariant.contains("not trusted evidence"))
    );
}

#[test]
fn upstream_codex_activation_trusted_evidence_acceptance_matrix_enumerates_checks() {
    let report = hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.matrix_id,
        "upstream-codex-activation-trusted-evidence-acceptance-matrix"
    );
    assert_eq!(
        report.source_denied_fixture_gate,
        "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh"
    );
    assert_eq!(
        report.trusted_acceptance_matrix_gate,
        "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
    );
    assert!(report.source_denied_fixture_ready);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.verification_entry_count, 8);
    assert_eq!(report.schema_complete_verification_entry_count, 8);
    assert_eq!(report.required_verification_count_per_record, 7);
    assert_eq!(report.total_required_verification_count, 56);
    assert_eq!(report.total_satisfied_verification_count, 0);
    assert_eq!(report.operator_approval_verified_record_count, 0);
    assert_eq!(report.request_binding_verified_record_count, 0);
    assert_eq!(report.active_binary_sha_verified_record_count, 0);
    assert_eq!(report.route_or_status_hash_verified_record_count, 0);
    assert_eq!(report.artifact_hash_verified_record_count, 0);
    assert_eq!(report.freshness_window_satisfied_record_count, 0);
    assert_eq!(report.trusted_source_verified_record_count, 0);
    assert_eq!(report.accepted_record_count, 0);
    assert_eq!(report.blocked_record_count, 8);
    assert!(report.trusted_evidence_acceptance_matrix_ready);
    assert!(report.activation_blocked_by_trusted_acceptance_matrix);
    assert!(!report.activation_allowed_by_trusted_acceptance_matrix);
    assert!(!report.active_wiring_allowed);
}

#[test]
fn upstream_codex_activation_trusted_evidence_acceptance_matrix_preserves_denials() {
    let report = hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .acceptance_denial_reason
            .contains("operator approval")
    );
    assert!(report.verification_entries.iter().all(|entry| {
        entry.schema_complete
            && entry.required_verification_count == 7
            && entry.satisfied_verification_count == 0
            && entry.operator_approval_required
            && !entry.operator_approval_verified
            && entry.activation_request_binding_required
            && !entry.activation_request_binding_verified
            && entry.active_binary_sha_required
            && !entry.active_binary_sha_verified
            && entry.route_or_status_hash_required
            && !entry.route_or_status_hash_verified
            && entry.artifact_hash_or_redacted_path_required
            && !entry.artifact_hash_or_redacted_path_verified
            && entry.freshness_window_required
            && !entry.freshness_window_satisfied
            && entry.trusted_source_required
            && !entry.trusted_source_verified
            && !entry.accepted
            && entry.acceptance_status == "blocked"
    }));
    assert!(
        report
            .acceptance_invariants
            .iter()
            .any(|invariant| invariant.contains("not trusted evidence"))
    );
}

#[test]
fn upstream_codex_activation_trusted_record_shape_validator_blocks_partial_trust() {
    let report = hepta_upstream_codex_activation_trusted_record_shape_validator_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.validator_id,
        "upstream-codex-activation-trusted-record-shape-validator"
    );
    assert_eq!(
        report.source_trusted_acceptance_matrix_gate,
        "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
    );
    assert_eq!(
        report.trusted_record_shape_validator_gate,
        "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
    );
    assert!(report.source_trusted_acceptance_matrix_ready);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.fixture_count, 2);
    assert_eq!(report.partial_trusted_fixture_count, 1);
    assert_eq!(report.public_claim_attempt_fixture_count, 1);
    assert_eq!(report.blocked_fixture_count, 2);
    assert_eq!(report.allowed_fixture_count, 0);
    assert_eq!(report.required_verification_count_per_record, 7);
    assert_eq!(report.total_required_verification_count_per_fixture, 56);
    assert_eq!(report.max_satisfied_verification_count, 48);
    assert!(report.trusted_record_shape_validator_ready);
    assert!(report.activation_blocked_by_shape_validator);
    assert!(!report.activation_allowed_by_shape_validator);
    assert!(!report.active_wiring_allowed);

    let partial = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_id == "partial-trusted-records")
        .expect("partial trusted fixture");
    assert_eq!(partial.total_satisfied_verification_count, 32);
    assert_eq!(partial.artifact_hash_verified_record_count, 0);
    assert_eq!(partial.freshness_window_satisfied_record_count, 0);
    assert_eq!(partial.trusted_source_verified_record_count, 0);
    assert_eq!(partial.accepted_record_count, 0);
    assert_eq!(partial.blocked_record_count, 8);
    assert_eq!(partial.validation_status, "blocked");
    assert!(!partial.active_wiring_allowed);
}

#[test]
fn upstream_codex_activation_trusted_record_shape_validator_preserves_public_denials() {
    let report = hepta_upstream_codex_activation_trusted_record_shape_validator_report();

    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .shape_denial_reason
            .contains("partial or public-claim")
    );

    let public_claim = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_id == "public-claim-attempt-with-trusted-shape")
        .expect("public claim fixture");
    assert!(public_claim.public_release_claim_requested);
    assert!(public_claim.release_artifact_write_requested);
    assert_eq!(public_claim.total_satisfied_verification_count, 48);
    assert_eq!(public_claim.artifact_hash_verified_record_count, 8);
    assert_eq!(public_claim.freshness_window_satisfied_record_count, 0);
    assert_eq!(public_claim.trusted_source_verified_record_count, 8);
    assert_eq!(public_claim.validation_status, "blocked");
    assert!(!public_claim.public_release_claim_allowed);
    assert!(!public_claim.release_artifact_write_allowed);
    assert!(
        public_claim
            .denial_reason
            .contains("freshness is incomplete")
    );
    assert!(report.fixtures.iter().all(|fixture| {
        !fixture.active_wiring_allowed
            && !fixture.public_release_claim_allowed
            && !fixture.release_artifact_write_allowed
    }));
    assert!(
        report
            .shape_invariants
            .iter()
            .any(|invariant| invariant.contains("partially verified"))
    );
}

#[test]
fn upstream_codex_activation_evidence_completeness_scoreboard_summarizes_gate_families() {
    let report = hepta_upstream_codex_activation_evidence_completeness_scoreboard_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.scoreboard_id,
        "upstream-codex-activation-evidence-completeness-scoreboard"
    );
    assert_eq!(
        report.source_trusted_record_shape_validator_gate,
        "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
    );
    assert_eq!(
        report.evidence_completeness_scoreboard_gate,
        "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
    );
    assert!(report.source_trusted_record_shape_validator_ready);
    assert_eq!(report.required_gate_family_count, 10);
    assert_eq!(report.ready_gate_family_count, 10);
    assert_eq!(report.activation_blocking_gate_family_count, 10);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.required_trusted_record_count, 8);
    assert_eq!(report.accepted_trusted_record_count, 0);
    assert_eq!(report.fresh_trusted_record_count, 0);
    assert!(report.public_claim_attempt_blocked);
    assert!(report.release_artifact_write_attempt_blocked);
    assert!(report.evidence_completeness_scoreboard_ready);
    assert!(report.activation_blocked_by_scoreboard);
    assert!(!report.activation_allowed_by_scoreboard);
    assert!(!report.active_wiring_allowed);
    assert!(
        report.gate_families.iter().all(|family| {
            family.gate_ready && family.blocks_activation_without_trusted_evidence
        })
    );
}

#[test]
fn upstream_codex_activation_evidence_completeness_scoreboard_preserves_denials() {
    let report = hepta_upstream_codex_activation_evidence_completeness_scoreboard_report();

    assert!(!report.operator_approval_recorded);
    assert!(!report.activation_request_recorded);
    assert!(!report.operator_approved_activation_ready);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .scoreboard_denial_reason
            .contains("no real activation request")
    );
    assert!(
        report
            .scoreboard_invariants
            .iter()
            .any(|invariant| invariant.contains("activation remains denied"))
    );
}

#[test]
fn upstream_codex_activation_evidence_recording_dry_run_receipt_defines_redacted_schema() {
    let report = hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.receipt_id,
        "upstream-codex-activation-evidence-recording-dry-run-receipt"
    );
    assert_eq!(
        report.source_scoreboard_gate,
        "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
    );
    assert_eq!(
        report.evidence_recording_dry_run_receipt_gate,
        "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
    );
    assert!(report.source_scoreboard_ready);
    assert_eq!(report.required_receipt_field_count, 12);
    assert_eq!(report.recorded_receipt_field_count, 0);
    assert_eq!(report.redacted_or_hashed_field_count, 10);
    assert_eq!(report.required_evidence_count, 8);
    assert_eq!(report.required_trusted_record_count, 8);
    assert!(report.receipt_schema_ready);
    assert!(report.evidence_recording_dry_run_ready);
    assert!(report.receipt_fields.iter().all(|field| field.required));
    assert!(report.receipt_fields.iter().all(|field| !field.recorded));
    assert!(
        report
            .receipt_fields
            .iter()
            .any(|field| { field.name == "operator_identity_hash" && field.redacted_or_hashed })
    );
    assert!(report.receipt_fields.iter().any(|field| {
        field.name == "artifact_sha256_or_redacted_path_bundle" && field.redacted_or_hashed
    }));
}

#[test]
fn upstream_codex_activation_evidence_recording_dry_run_receipt_preserves_denials() {
    let report = hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report();

    assert!(!report.operator_approval_recorded);
    assert!(!report.activation_request_recorded);
    assert!(!report.receipt_recorded);
    assert!(!report.real_evidence_recorded);
    assert!(!report.trusted_record_materialized);
    assert_eq!(report.accepted_trusted_record_count, 0);
    assert_eq!(report.fresh_trusted_record_count, 0);
    assert!(report.public_claim_attempt_blocked);
    assert!(report.release_artifact_write_attempt_blocked);
    assert!(report.activation_blocked_by_receipt);
    assert!(!report.activation_allowed_by_receipt);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(report.receipt_denial_reason.contains("schema-only"));
    assert!(
        report
            .receipt_invariants
            .iter()
            .any(|invariant| invariant.contains("no evidence is recorded"))
    );
}

#[test]
fn upstream_codex_activation_evidence_recording_denial_matrix_blocks_attempts() {
    let report = hepta_upstream_codex_activation_evidence_recording_denial_matrix_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.matrix_id,
        "upstream-codex-activation-evidence-recording-denial-matrix"
    );
    assert_eq!(
        report.source_receipt_gate,
        "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
    );
    assert_eq!(
        report.evidence_recording_denial_matrix_gate,
        "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
    );
    assert!(report.source_receipt_gate_ready);
    assert_eq!(report.required_denied_attempt_count, 3);
    assert_eq!(report.denied_receipt_attempt_count, 3);
    assert_eq!(report.allowed_receipt_attempt_count, 0);
    assert_eq!(report.max_recorded_receipt_field_count, 12);
    assert_eq!(report.max_accepted_trusted_record_count, 8);
    assert_eq!(report.max_fresh_trusted_record_count, 8);
    assert_eq!(report.public_claim_attempt_count, 1);
    assert_eq!(report.release_artifact_write_attempt_count, 1);
    assert!(report.no_write_sink_ready);
    assert!(
        report
            .denied_receipt_attempts
            .iter()
            .all(|attempt| attempt.denial_status == "blocked")
    );
    assert!(
        report
            .denied_receipt_attempts
            .iter()
            .any(|attempt| attempt.attempt_kind == "public_claim_release_artifact_attempt")
    );
}

#[test]
fn upstream_codex_activation_evidence_recording_denial_matrix_preserves_no_write_sink() {
    let report = hepta_upstream_codex_activation_evidence_recording_denial_matrix_report();

    assert!(!report.receipt_sink_write_performed);
    assert!(!report.evidence_receipt_persisted);
    assert!(!report.trusted_record_materialized);
    assert!(report.activation_blocked_by_no_write_sink);
    assert!(!report.activation_allowed_by_no_write_sink);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .denied_receipt_attempts
            .iter()
            .all(|attempt| !attempt.receipt_materialized
                && !attempt.workspace_write_allowed
                && !attempt.active_wiring_allowed
                && !attempt.public_release_claim_allowed
                && !attempt.release_artifact_write_allowed)
    );
    assert!(
        report
            .no_write_sink_invariants
            .iter()
            .any(|invariant| invariant.contains("fully shaped without being persisted"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_persistence_command_contract_is_noop_by_default() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.command_contract_id,
        "upstream-codex-activation-evidence-receipt-persistence-command-contract"
    );
    assert_eq!(
        report.source_denial_matrix_gate,
        "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
    );
    assert_eq!(
        report.receipt_persistence_command_contract_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
    );
    assert!(report.source_denial_matrix_ready);
    assert_eq!(report.required_command_field_count, 10);
    assert_eq!(report.recorded_command_field_count, 0);
    assert_eq!(report.redacted_or_hashed_field_count, 9);
    assert!(report.operator_approval_required);
    assert!(report.activation_request_required);
    assert!(!report.operator_approval_recorded);
    assert!(!report.activation_request_recorded);
    assert!(!report.receipt_persistence_command_enabled_by_default);
    assert!(report.receipt_persistence_noop_ready);
    assert!(report.command_fields.iter().all(|field| field.required));
    assert!(report.command_fields.iter().all(|field| !field.recorded));
    assert!(
        report
            .command_fields
            .iter()
            .any(|field| field.name == "receipt_output_path_redacted" && field.redacted_or_hashed)
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_persistence_command_contract_preserves_denials() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report();

    assert!(!report.trusted_record_materialized);
    assert!(!report.receipt_persistence_command_invoked);
    assert!(!report.receipt_persistence_execution_performed);
    assert!(!report.workspace_write_performed);
    assert!(!report.evidence_receipt_persisted);
    assert!(report.activation_blocked_by_persistence_contract);
    assert!(!report.activation_allowed_by_persistence_contract);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .command_contract_invariants
            .iter()
            .any(|invariant| invariant.contains("disabled by default"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_is_noop() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.invocation_dry_run_id,
        "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
    );
    assert_eq!(
        report.source_command_contract_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
    );
    assert_eq!(
        report.receipt_persistence_invocation_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
    );
    assert!(report.source_command_contract_ready);
    assert_eq!(report.required_invocation_fixture_count, 3);
    assert_eq!(report.command_invocation_attempt_count, 3);
    assert_eq!(report.command_invocation_performed_count, 0);
    assert_eq!(report.receipt_persistence_execution_performed_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert_eq!(report.evidence_receipt_persisted_count, 0);
    assert_eq!(report.redacted_output_path_fixture_count, 3);
    assert_eq!(report.payload_hash_bound_fixture_count, 3);
    assert_eq!(report.operator_approved_fixture_count, 3);
    assert_eq!(report.activation_request_bound_fixture_count, 3);
    assert_eq!(report.max_recorded_command_field_count, 10);
    assert_eq!(report.max_accepted_trusted_record_count, 8);
    assert_eq!(report.max_fresh_trusted_record_count, 8);
    assert_eq!(report.public_claim_attempt_count, 1);
    assert_eq!(report.release_artifact_write_attempt_count, 1);
    assert!(!report.receipt_persistence_command_enabled_by_default);
    assert!(report.invocation_dry_run_noop_ready);
}

#[test]
fn upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_blocks_effects() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report();

    assert!(report.activation_blocked_by_invocation_dry_run);
    assert!(!report.activation_allowed_by_invocation_dry_run);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .invocation_fixtures
            .iter()
            .all(|fixture| fixture.command_invocation_requested
                && !fixture.command_invocation_performed
                && !fixture.receipt_persistence_execution_performed
                && !fixture.workspace_write_performed
                && !fixture.evidence_receipt_persisted
                && !fixture.active_wiring_allowed
                && !fixture.public_release_claim_allowed
                && !fixture.release_artifact_write_allowed
                && fixture.dry_run_status == "blocked_noop")
    );
    assert!(
        report
            .invocation_fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == "public_claim_artifact_invocation_attempt")
    );
    assert!(
        report
            .invocation_dry_run_invariants
            .iter()
            .any(|invariant| invariant.contains("request persistence without executing it"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_is_ready() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.no_write_sink_adapter_id,
        "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
    );
    assert_eq!(
        report.source_invocation_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
    );
    assert_eq!(
        report.no_write_sink_adapter_contract_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
    );
    assert!(report.source_invocation_dry_run_ready);
    assert_eq!(report.required_sink_surface_count, 6);
    assert_eq!(report.ready_sink_surface_count, 6);
    assert_eq!(report.side_effect_free_surface_count, 6);
    assert_eq!(report.accepted_invocation_fixture_count, 3);
    assert_eq!(report.rejected_write_fixture_count, 3);
    assert_eq!(report.rejected_public_claim_fixture_count, 1);
    assert_eq!(report.persisted_receipt_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert!(!report.sink_write_path_enabled_by_default);
    assert!(report.sink_accepts_redacted_payload_hash);
    assert!(report.sink_accepts_redacted_output_path);
    assert!(report.sink_requires_operator_approval);
    assert!(report.sink_requires_fresh_trusted_records);
    assert!(report.sink_rejects_public_claim_artifact_write);
    assert!(report.no_write_sink_adapter_ready);
    assert!(report.sink_surfaces.iter().all(|surface| surface.required));
    assert!(report.sink_surfaces.iter().all(|surface| surface.ready));
    assert!(
        report
            .sink_surfaces
            .iter()
            .all(|surface| surface.side_effect_free)
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_blocks_effects() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report();

    assert!(report.activation_blocked_by_no_write_sink_adapter);
    assert!(!report.activation_allowed_by_no_write_sink_adapter);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .no_write_sink_adapter_invariants
            .iter()
            .any(|invariant| invariant.contains("without persisting them"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_write_enable_fixture_is_ready() {
    let report = hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.write_enable_fixture_id,
        "upstream-codex-activation-evidence-receipt-write-enable-fixture"
    );
    assert_eq!(
        report.source_no_write_sink_adapter_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
    );
    assert_eq!(
        report.write_enable_fixture_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
    );
    assert!(report.source_no_write_sink_adapter_ready);
    assert_eq!(report.required_write_enable_fixture_count, 3);
    assert_eq!(report.write_enable_fixture_count, 3);
    assert_eq!(report.blocked_write_enable_fixture_count, 3);
    assert_eq!(report.allowed_write_enable_fixture_count, 0);
    assert_eq!(report.explicit_write_enable_requested_fixture_count, 3);
    assert_eq!(report.operator_approved_fixture_count, 2);
    assert_eq!(report.activation_request_bound_fixture_count, 3);
    assert_eq!(report.fresh_trusted_record_fixture_count, 2);
    assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
    assert_eq!(report.public_claim_attempt_fixture_count, 1);
    assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
    assert_eq!(report.public_artifact_policy_satisfied_fixture_count, 2);
    assert_eq!(report.filesystem_persistence_allowed_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert_eq!(report.evidence_receipt_persisted_count, 0);
    assert!(report.write_enable_fixture_contract_ready);
    assert!(
        report
            .write_enable_fixtures
            .iter()
            .all(|fixture| fixture.explicit_write_enable_requested)
    );
    assert!(
        report
            .write_enable_fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == "public_artifact_write_attempt")
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_write_enable_fixture_blocks_effects() {
    let report = hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report();

    assert!(report.activation_blocked_by_write_enable_fixture);
    assert!(!report.activation_allowed_by_write_enable_fixture);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(report.write_enable_fixtures.iter().all(|fixture| {
        !fixture.filesystem_persistence_allowed
            && !fixture.workspace_write_performed
            && !fixture.evidence_receipt_persisted
    }));
    assert!(
        report
            .write_enable_fixture_invariants
            .iter()
            .any(|invariant| invariant.contains("before any real write path exists"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_materialization_dry_run_is_ready() {
    let report = hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.materialization_dry_run_id,
        "upstream-codex-activation-evidence-receipt-materialization-dry-run"
    );
    assert_eq!(
        report.source_write_enable_fixture_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
    );
    assert_eq!(
        report.materialization_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
    );
    assert!(report.source_write_enable_fixture_ready);
    assert_eq!(report.required_materialization_fixture_count, 3);
    assert_eq!(report.materialization_fixture_count, 3);
    assert_eq!(report.blocked_materialization_fixture_count, 3);
    assert_eq!(report.allowed_materialization_fixture_count, 0);
    assert_eq!(report.explicit_write_enable_requested_fixture_count, 3);
    assert_eq!(report.operator_approved_fixture_count, 2);
    assert_eq!(report.activation_request_bound_fixture_count, 3);
    assert_eq!(report.fresh_trusted_record_fixture_count, 2);
    assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
    assert_eq!(report.payload_hash_planned_fixture_count, 3);
    assert_eq!(report.redacted_output_path_planned_fixture_count, 3);
    assert_eq!(report.deterministic_materialization_plan_count, 3);
    assert_eq!(report.public_claim_attempt_fixture_count, 1);
    assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
    assert_eq!(report.public_artifact_policy_satisfied_fixture_count, 2);
    assert_eq!(report.filesystem_persistence_allowed_count, 0);
    assert_eq!(report.materialization_executed_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert_eq!(report.evidence_receipt_persisted_count, 0);
    assert!(report.materialization_dry_run_ready);
    assert!(report.materialization_fixtures.iter().all(|fixture| {
        fixture.payload_hash_planned
            && fixture.redacted_output_path_planned
            && fixture.deterministic_materialization_plan
    }));
    assert!(
        report
            .materialization_fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == "public_artifact_attempt")
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_materialization_dry_run_blocks_effects() {
    let report = hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report();

    assert!(report.activation_blocked_by_materialization_dry_run);
    assert!(!report.activation_allowed_by_materialization_dry_run);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(report.materialization_fixtures.iter().all(|fixture| {
        !fixture.filesystem_persistence_allowed
            && !fixture.materialization_executed
            && !fixture.workspace_write_performed
            && !fixture.evidence_receipt_persisted
    }));
    assert!(
        report
            .materialization_invariants
            .iter()
            .any(|invariant| invariant.contains("without executing persistence"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_is_ready() {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.filesystem_persistence_approval_packet_id,
        "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
    );
    assert_eq!(
        report.source_materialization_dry_run_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
    );
    assert_eq!(
        report.filesystem_persistence_approval_packet_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
    );
    assert!(report.source_materialization_dry_run_ready);
    assert_eq!(report.required_approval_field_count, 12);
    assert_eq!(report.approval_field_count, 12);
    assert_eq!(report.recorded_approval_field_count, 0);
    assert_eq!(report.redacted_or_hashed_field_count, 10);
    assert_eq!(report.required_for_filesystem_persistence_field_count, 12);
    assert!(report.operator_approval_required);
    assert!(!report.operator_approval_recorded);
    assert!(report.activation_request_required);
    assert!(!report.activation_request_recorded);
    assert!(report.materialization_plan_required);
    assert!(!report.materialization_plan_recorded);
    assert!(report.fresh_trusted_records_required);
    assert!(!report.fresh_trusted_records_recorded);
    assert!(report.active_binary_sha_required);
    assert!(!report.active_binary_sha_recorded);
    assert!(report.public_artifact_policy_required);
    assert!(!report.public_artifact_policy_recorded);
    assert!(report.filesystem_persistence_approval_packet_ready);
    assert!(
        report
            .approval_fields
            .iter()
            .all(|field| field.required_for_filesystem_persistence && !field.recorded_by_default)
    );
    assert!(
        report
            .approval_fields
            .iter()
            .any(|field| field.name == "materialization_plan_id")
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_blocks_effects()
 {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report();

    assert!(report.activation_blocked_by_filesystem_persistence_approval);
    assert!(!report.activation_allowed_by_filesystem_persistence_approval);
    assert!(!report.filesystem_persistence_allowed);
    assert!(!report.filesystem_persistence_execution_performed);
    assert!(!report.workspace_write_performed);
    assert!(!report.evidence_receipt_persisted);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.materialization_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .approval_packet_invariants
            .iter()
            .any(|invariant| invariant.contains("before any workspace write"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_is_ready() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.filesystem_output_path_allowlist_id,
        "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
    );
    assert_eq!(
        report.source_filesystem_persistence_approval_packet_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
    );
    assert_eq!(
        report.filesystem_output_path_allowlist_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
    );
    assert!(report.source_filesystem_persistence_approval_packet_ready);
    assert_eq!(report.required_allowlist_entry_count, 6);
    assert_eq!(report.allowlist_entry_count, 6);
    assert_eq!(report.allowed_output_path_entry_count, 3);
    assert_eq!(report.blocked_output_path_entry_count, 3);
    assert_eq!(report.redacted_output_path_entry_count, 6);
    assert_eq!(report.default_selected_output_path_count, 0);
    assert!(!report.source_tree_path_allowed);
    assert!(!report.home_directory_path_allowed);
    assert!(!report.release_artifact_path_allowed);
    assert!(!report.public_artifact_path_allowed);
    assert!(report.receipt_output_path_allowlist_ready);
    assert!(
        report
            .allowlist_entries
            .iter()
            .all(|entry| entry.requires_operator_approval)
    );
    assert!(
        report
            .allowlist_entries
            .iter()
            .any(|entry| entry.name == "activation_evidence_receipts_root")
    );
    assert!(report.allowlist_entries.iter().any(
        |entry| entry.name == "release_artifact_root" && !entry.allowed_for_receipt_persistence
    ));
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_blocks_effects() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report();

    assert!(report.activation_blocked_by_output_path_allowlist);
    assert!(!report.activation_allowed_by_output_path_allowlist);
    assert!(!report.filesystem_persistence_allowed);
    assert!(!report.filesystem_persistence_execution_performed);
    assert!(!report.workspace_write_performed);
    assert!(!report.evidence_receipt_persisted);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.materialization_execution);
    assert!(!report.filesystem_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .allowlist_invariants
            .iter()
            .any(|invariant| invariant.contains("not filesystem write authority"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_is_ready() {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report(
            );

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.filesystem_output_path_evidence_binding_id,
        "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
    );
    assert_eq!(
        report.source_filesystem_output_path_allowlist_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
    );
    assert_eq!(
        report.filesystem_output_path_evidence_binding_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
    );
    assert!(report.source_filesystem_output_path_allowlist_ready);
    assert_eq!(report.required_path_binding_count, 8);
    assert_eq!(report.path_binding_count, 8);
    assert_eq!(report.allowed_output_path_entry_count, 3);
    assert_eq!(report.selected_output_path_count, 0);
    assert_eq!(report.recorded_path_binding_count, 0);
    assert_eq!(report.fresh_live_evidence_bound_count, 0);
    assert_eq!(report.active_binary_sha_bound_count, 0);
    assert_eq!(report.redacted_or_hashed_binding_count, 8);
    assert_eq!(report.trusted_source_bound_count, 0);
    assert!(!report.source_tree_path_binding_allowed);
    assert!(!report.home_directory_path_binding_allowed);
    assert!(!report.release_artifact_path_binding_allowed);
    assert!(!report.public_artifact_path_binding_allowed);
    assert!(report.output_path_evidence_binding_ready);
    assert!(
        report
            .path_bindings
            .iter()
            .all(|binding| binding.binding_required
                && binding.requires_fresh_live_evidence
                && binding.requires_active_binary_sha
                && !binding.recorded_by_default)
    );
    assert!(report.path_bindings.iter().any(|binding| {
        binding.evidence_id == "watchdog_evidence_id"
            && binding.allowed_output_path_entry_name == "activation_evidence_receipts_root"
    }));
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_blocks_effects()
 {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report(
            );

    assert!(report.activation_blocked_by_output_path_evidence_binding);
    assert!(!report.activation_allowed_by_output_path_evidence_binding);
    assert!(!report.filesystem_persistence_allowed);
    assert!(!report.filesystem_persistence_execution_performed);
    assert!(!report.workspace_write_performed);
    assert!(!report.evidence_receipt_persisted);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.materialization_execution);
    assert!(!report.filesystem_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .binding_invariants
            .iter()
            .any(|invariant| invariant.contains("fresh live evidence binding"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_is_ready() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.filesystem_sink_write_preview_id,
        "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
    );
    assert_eq!(
        report.source_filesystem_output_path_evidence_binding_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
    );
    assert_eq!(
        report.filesystem_sink_write_preview_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
    );
    assert!(report.source_filesystem_output_path_evidence_binding_ready);
    assert_eq!(report.required_preview_fixture_count, 3);
    assert_eq!(report.preview_fixture_count, 3);
    assert_eq!(report.allowed_output_path_entry_count, 3);
    assert_eq!(report.previewed_output_path_count, 3);
    assert_eq!(report.deterministic_payload_hash_count, 3);
    assert_eq!(report.redacted_output_path_preview_count, 3);
    assert_eq!(report.fresh_live_evidence_bound_fixture_count, 3);
    assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
    assert_eq!(report.trusted_source_bound_fixture_count, 3);
    assert_eq!(report.operator_approval_bound_fixture_count, 3);
    assert_eq!(report.blocked_preview_fixture_count, 3);
    assert_eq!(report.allowed_preview_fixture_count, 0);
    assert_eq!(report.public_claim_attempt_fixture_count, 1);
    assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
    assert_eq!(report.filesystem_persistence_allowed_count, 0);
    assert!(report.sink_write_preview_ready);
    assert!(report.preview_fixtures.iter().all(|fixture| {
        fixture.redacted_output_path.starts_with("<redacted:")
            && fixture.deterministic_payload_hash.starts_with("sha256:")
            && fixture.preview_status == "blocked_preview"
    }));
    assert!(report.preview_fixtures.iter().any(|fixture| {
        fixture.fixture_id == "public-artifact-sink-write-preview-attempt"
            && fixture.public_claim_requested
            && fixture.release_artifact_write_requested
    }));
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_blocks_effects() {
    let report =
        hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report();

    assert!(report.activation_blocked_by_sink_write_preview);
    assert!(!report.activation_allowed_by_sink_write_preview);
    assert_eq!(report.filesystem_persistence_allowed_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert_eq!(report.evidence_receipt_persisted_count, 0);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.materialization_execution);
    assert!(!report.filesystem_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .preview_invariants
            .iter()
            .any(|invariant| invariant.contains("not write authority"))
    );
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_is_ready()
 {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report();

    assert_eq!(report.product, "Hepta");
    assert_eq!(report.status, "ready");
    assert_eq!(
        report.filesystem_persistence_execution_denial_matrix_id,
        "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
    );
    assert_eq!(
        report.source_filesystem_sink_write_preview_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
    );
    assert_eq!(
        report.filesystem_persistence_execution_denial_matrix_gate,
        "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh"
    );
    assert!(report.source_filesystem_sink_write_preview_ready);
    assert_eq!(report.required_denial_fixture_count, 4);
    assert_eq!(report.denial_fixture_count, 4);
    assert_eq!(report.source_preview_fixture_count, 3);
    assert_eq!(report.execution_requested_fixture_count, 4);
    assert_eq!(report.future_persistence_approval_slot_count, 4);
    assert_eq!(report.explicit_persistence_approval_id_present_count, 3);
    assert_eq!(report.explicit_persistence_approval_id_missing_count, 1);
    assert_eq!(report.stale_or_missing_fresh_evidence_fixture_count, 1);
    assert_eq!(report.active_binary_sha_bound_fixture_count, 4);
    assert_eq!(report.trusted_source_bound_fixture_count, 4);
    assert_eq!(report.operator_approval_bound_fixture_count, 3);
    assert_eq!(report.workspace_path_attempt_fixture_count, 1);
    assert_eq!(report.public_claim_attempt_fixture_count, 1);
    assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
    assert_eq!(report.blocked_execution_fixture_count, 4);
    assert_eq!(report.allowed_execution_fixture_count, 0);
    assert!(report.execution_denial_matrix_ready);
    assert!(report.denial_fixtures.iter().all(|fixture| {
        fixture.deterministic_payload_hash.starts_with("sha256:")
            && fixture
                .future_persistence_approval_id_slot
                .starts_with("<future:")
            && fixture.execution_status == "blocked_execution"
    }));
    assert!(report.denial_fixtures.iter().any(|fixture| {
        fixture.fixture_id == "public-artifact-execution-attempt"
            && fixture.public_claim_requested
            && fixture.release_artifact_write_requested
    }));
}

#[test]
fn upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_blocks_effects()
 {
    let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report();

    assert!(report.activation_blocked_by_execution_denial_matrix);
    assert!(!report.activation_allowed_by_execution_denial_matrix);
    assert_eq!(report.filesystem_persistence_allowed_count, 0);
    assert_eq!(report.filesystem_persistence_execution_performed_count, 0);
    assert_eq!(report.workspace_write_performed_count, 0);
    assert_eq!(report.evidence_receipt_persisted_count, 0);
    assert!(!report.active_wiring_allowed);
    assert!(!report.active_runtime_code_wiring_allowed);
    assert!(!report.active_runtime_dependency_allowed);
    assert!(!report.active_runtime_auto_rebase_allowed);
    assert!(!report.active_codex_engine_dependency_allowed);
    assert!(!report.public_release_claim_allowed);
    assert!(!report.public_ga_claim_allowed);
    assert!(!report.release_artifact_write_allowed);
    assert!(!report.upstream_fetch_performed);
    assert!(!report.upstream_merge_performed);
    assert!(!report.upstream_checkout_performed);
    assert!(!report.command_invocation_performed);
    assert!(!report.receipt_persistence_execution);
    assert!(!report.materialization_execution);
    assert!(!report.filesystem_persistence_execution);
    assert!(!report.workspace_mutation_default);
    assert!(!report.active_service_restart);
    assert!(!report.credential_value_read);
    assert!(!report.secret_file_read);
    assert!(!report.provider_invoked);
    assert!(!report.channel_delivery_performed);
    assert!(!report.gateway_rpc_performed);
    assert!(!report.public_release_published);
    assert!(
        report
            .denial_invariants
            .iter()
            .any(|invariant| invariant.contains("not write authority"))
    );
}
