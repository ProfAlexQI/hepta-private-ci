def hepta_ui_explicit_false:
  type == "boolean" and . == false;

def hepta_ui_source_ready:
  . as $c |
  (
    $c.binding_stable == true
    and $c.binding_after.repository_worktree_clean == true
    and $c.sync_exit_code == 0
    and $c.product_exit_code == 0
    and $c.token_exit_code == 0
    and $c.feature_exit_code == 0
    and $c.mobile_exit_code == 0
    and $c.sync_bound == true
    and $c.product_bound == true
    and $c.token_bound == true
    and $c.feature_bound == true
    and $c.package_bound == true
    and $c.mobile_bound == true
    and $c.sync.status == "ready"
    and $c.sync.path_ledger_ready == true
    and $c.product.status == "ready"
    and $c.tokens.status == "ready"
    and $c.feature.feature_matrix_ready == true
    and $c.package.static_package_contract_ready == true
    and $c.mobile.status == "source_contract_ready"
    and $c.mobile.mobile_source_contract_ready == true
    and ($c.mobile.hard_boundaries.android_emulator_visual_verified | hepta_ui_explicit_false)
    and ($c.mobile.hard_boundaries.android_emulator_rotation_verified | hepta_ui_explicit_false)
    and ($c.mobile.hard_boundaries.android_emulator_ime_verified | hepta_ui_explicit_false)
  );

def hepta_ui_browser_ready:
  . as $c |
  (
    ($c.browser.schema_version // null) == 1
    and ($c.browser.kind // "") == "hepta-control-ui-browser-smoke-current-wrapper"
    and ($c.browser.producer // "") == "scripts/hepta-ui-current-readiness.sh"
    and ($c.browser.original_receipt_valid // false) == true
    and ($c.browser.browser_child_exit_code // -1) == 0
    and ($c.browser.source_binding.head // "") == $c.binding_after.head
    and ($c.browser.source_binding.head_tree // "") == $c.binding_after.head_tree
    and ($c.browser.source_binding.source_fingerprint // "") == $c.binding_after.source_fingerprint
    and ($c.browser.browser_smoke_ready // false) == true
  );

def hepta_ui_promotion_independent_verifiers_ready:
  . as $c |
  (
    $c.window_verifier_executed == true
    and $c.window_exit_code == 0
    and $c.window_receipt.ready == true
    and $c.window_receipt.source_stable_during_run == true
    and $c.window_receipt.independent_verifier_ready == true
    and $c.window_receipt.scope == "unauthenticated_local_macos_product_shell"
    and ($c.window_receipt.run_nonce // "") == $c.run_nonce
    and ($c.window_receipt.package.current_source_local_package_ready // false) == true
    and ($c.window_receipt.package.visual_capture_binary_is_exact_packaged_executable // false) == true
    and ($c.window_receipt.package.visual_capture_binary_is_separate_developer_diagnostics_build | hepta_ui_explicit_false)
    and ($c.window_receipt.package.report_path // "") == $c.package_report_path
    and ($c.window_receipt.package.report_sha256 // "") == $c.package_report_sha256
    and ($c.window_receipt.package.app_path // "") == ($c.package.artifact.path // "")
    and ($c.package.artifact.path // "") == $c.expected_package_app_path
    and ($c.window_receipt.package.binary_path // "") == $c.package_binary_path
    and ($c.window_receipt.package.binary_sha256 // "") == $c.package_binary_actual_sha256
    and ($c.window_receipt.package.bundle_fingerprint_sha256 // "") == $c.package_bundle_actual_sha256
    and $c.package_artifact_hash_valid == true
    and ($c.window_receipt.automation.no_remote // false) == true
    and ($c.window_receipt.automation.host_kind // "") == "local"
    and ($c.window_receipt.automation.host_source // "") == "forced_local_services"
    and ($c.window_receipt.automation.application_process.identity_safe_termination_confirmed // false) == true
    and ($c.window_receipt.host_window.title // "") == "Hepta"
    and ($c.window_receipt.host_window.exact_title_match_count // 0) == 1
    and ($c.window_receipt.host_window.bounds_within_tolerance // false) == true
    and ($c.window_receipt.host_window.minimum_capture_size_ready // false) == true
    and ($c.window_receipt.isolation.home_isolated // false) == true
    and ($c.window_receipt.isolation.real_product_data_path_denied // false) == true
    and ($c.window_receipt.isolation.real_product_cache_path_denied // false) == true
    and ($c.window_receipt.isolation.keychain_services_denied // false) == true
    and ($c.window_receipt.isolation.network_denied_by_sandbox // false) == true
    and ($c.window_receipt.isolation.force_login_argument // false) == true
  );

def hepta_ui_readiness_truth:
  . as $c |
  ($c | hepta_ui_source_ready) as $source_ready |
  ($c | hepta_ui_browser_ready) as $browser_ready |
  ($c | hepta_ui_promotion_independent_verifiers_ready) as $promotion_ready |
  (
    $source_ready
    and $c.package_exit_code == 0
    and $c.package.status == "ready"
    and $c.package.local_package_ready == true
    and $browser_ready
    and $c.window_receipt.ready == true
    and $promotion_ready
  ) as $local_ready |
  {
    source: $source_ready,
    browser: $browser_ready,
    promotion: $promotion_ready,
    local: $local_ready
  };

def hepta_ui_live_chain_bound($matrix; $bridge):
  (
    $matrix.ready == true
    and $bridge.ready == true
    and ($matrix.live_chain_binding.sequence_verified // false) == true
    and ($bridge.live_chain_binding.parent_signature_verified // false) == true
    and ($bridge.live_chain_binding.session_match_verified // false) == true
    and ($bridge.live_chain_binding.run_match_verified // false) == true
    and ($bridge.live_chain_binding.sequence_verified // false) == true
    and ($matrix.live_chain_binding.run_identifier_sha256 // "") != ""
    and ($matrix.live_chain_binding.run_identifier_sha256 // "") == ($bridge.live_chain_binding.run_identifier_sha256 // "")
    and ($matrix.live_chain_binding.session_identifier_sha256 // "") != ""
    and ($matrix.live_chain_binding.session_identifier_sha256 // "") == ($bridge.live_chain_binding.session_identifier_sha256 // "")
    and ($matrix.input_receipt.sha256 // "") == ($bridge.live_chain_binding.matrix_attestation_sha256 // "")
    and ($matrix.attestation_signature.sha256 // "") == ($bridge.live_chain_binding.matrix_signature_sha256 // "")
    and ($matrix.attestation_signature.trusted_public_key_sha256 // "") == ($bridge.live_chain_binding.matrix_trusted_public_key_sha256 // "")
    and ($matrix.artifact.expected_sha256 // "") == ($bridge.live_chain_binding.matrix_evidence_manifest_sha256 // "")
    and ($matrix.attestation_signature.expected_producer // "") == ($bridge.live_chain_binding.matrix_producer // "")
  );

def hepta_ui_product_promotion_truth($local_ready; $mobile; $matrix; $bridge; $device; $accessibility; $release):
  (
    $local_ready
    and ($mobile.hard_boundaries.ios_accessibility_update_consumed // false) == true
    and ($mobile.hard_boundaries.android_accessibility_update_consumed // false) == true
    and ($mobile.hard_boundaries.android_secure_session_persistence_ready // false) == true
    and hepta_ui_live_chain_bound($matrix; $bridge)
    and $device.ready == true
    and $device.independent_verifier_ready == true
    and $accessibility.ready == true
    and $accessibility.independent_verifier_ready == true
  ) as $full_ready |
  (
    $release.ready == true
    and $release.source_stable_during_run == true
    and $release.independent_verifier_ready == true
    and $release.signed == true
    and $release.notarized == true
    and $release.stapled == true
  ) as $release_ready |
  {
    full: $full_ready,
    mobile_full: (
      ($mobile.hard_boundaries.ios_accessibility_update_consumed // false) == true
      and ($mobile.hard_boundaries.android_accessibility_update_consumed // false) == true
      and ($mobile.hard_boundaries.android_secure_session_persistence_ready // false) == true
      and $device.ready == true
      and $device.independent_verifier_ready == true
      and $accessibility.ready == true
      and $accessibility.independent_verifier_ready == true
    ),
    release_independent: $release_ready,
    ga: ($full_ready and $release_ready)
  };

def hepta_ui_invalidate_claim_tree:
  walk(
    if type == "boolean" then false
    elif type == "object" then
      (if has("status") then .status = "not_ready" else . end)
      | (if has("reported_status") then .reported_status = "not_ready" else . end)
    else .
    end
  );

def hepta_ui_invalidate_derived_claims($reason):
  .source_stable_during_run = false
  | .current_head_active_truth_ready = false
  | .readiness = {source:false, local_demo:false, full_product:false, public_ga:false}
  | .promotion_trust_policy |= (
      .loaded_from_exact_head_blob = false
      | .worktree_matches_head = false
      | .index_flags_clear = false
      | .contract_ready = false
      | .configured_profiles = []
    )
  | .promotion_receipts |= map(
      hepta_ui_invalidate_claim_tree
      | .reason = $reason
    )
  | .gates |= hepta_ui_invalidate_claim_tree
  | .hard_boundaries |= hepta_ui_invalidate_claim_tree
  | .status = (if .report_only then "report_complete" else "not_ready" end)
  | .blockers = ((.blockers + [$reason]) | unique);
