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
