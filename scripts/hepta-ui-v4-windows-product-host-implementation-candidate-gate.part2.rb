  rollback_failure_is_rejected_and_authority_constants_remain_false
].all? { |term| implementation.include?(term) }

checks["product.not_registered"] =
  !product_mod.include?("hepta_windows_product_host_implementation") &&
  !lifecycle.include?("HeptaWindowsProductMaterialHost") &&
  !lifecycle.include?("hepta_windows_product_host_implementation")
checks["review.remains_review_only"] =
  review.include?("HEPTA_WINDOWS_REVIEW_PRODUCT_HOST_MAY_BIND: bool = false") &&
  review.include?("eligible_for_implementation_review")

checks["harness.feature_default_off"] =
  harness.include?("default = []") &&
  harness.include?("hepta_ui_windows_system_material_v4 = []") &&
  harness_lib.include?("#![forbid(unsafe_code)]") &&
  harness_lib.include?("hepta_windows_product_host_implementation.rs") &&
  harness_lib.include?("hepta_windows_product_host_integration_review.rs")

checks["contract.source_only"] =
  contract.dig("qualification", "sourceImplemented") == true &&
  contract.dig("feature", "isolatedHarnessDefaultEnabled") == false &&
  contract.dig("feature", "productCargoFeatureDeclared") == false &&
  contract.dig("feature", "productModuleRegistered") == false &&
  contract.dig("feature", "productLifecycleWired") == false &&
  contract.dig("feature", "automaticBindingAllowed") == false &&
  contract.fetch("qualification").reject { |key, _| key == "sourceImplemented" }.values.none? &&
  contract.fetch("authority").values.none?
checks["contract.prerequisites"] = %w[
  reviewReceiptRequired
  implementationApprovalRequired
  operatorAcceptanceRequired
  physicalDeviceValidationRequired
  rollbackDrillRequired
  transparencyPreferenceRecheckRequired
  highContrastRecheckRequired
  explicitActivationOnly
].all? { |key| contract.dig("implementation", key) == true }

checks["schema.statuses"] = schema.dig("properties", "status", "enum") == %w[
  PASS_WINDOWS_PRODUCT_HOST_ACTIVATION
  PASS_WINDOWS_PRODUCT_HOST_ROLLBACK
  FAIL_WINDOWS_PRODUCT_HOST_ACTIVATION
]
checks["schema.feature_default_off"] =
  schema.dig("$defs", "feature", "properties", "name", "const") ==
    "hepta_ui_windows_system_material_v4" &&
  schema.dig("$defs", "feature", "properties", "defaultEnabled", "const") == false &&
  schema.dig("$defs", "feature", "properties", "automaticBindingAllowed", "const") == false
checks["schema.authority_false"] =
  schema.dig("$defs", "authority", "properties").values.all? { |entry| entry["const"] == false }
checks["schema.fail_closed"] =
  schema.dig("properties", "failures", "maxItems") == 64 &&
  schema.dig("properties", "failures", "items", "maxLength") == 4096

checks["workflow.read_only"] =
  workflow.include?("permissions:") && workflow.include?("contents: read") &&
  !workflow.match?(/^\s*(?:deploy|publish|promotion|release)\s*:/i)
checks["workflow.exact_checkout"] =
  workflow.include?("github.event.pull_request.head.sha || github.sha") &&
  workflow.include?("git rev-parse HEAD")
checks["workflow.source_gate"] =
  workflow.include?("hepta-ui-v4-windows-product-host-implementation-candidate-gate") &&
  workflow.include?("PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_SOURCE_ONLY")
checks["workflow.dual_feature_compile"] =
  workflow.include?("--no-default-features") &&
  workflow.include?("--features hepta_ui_windows_system_material_v4") &&
  workflow.include?("ubuntu-latest") && workflow.include?("windows-latest") &&
  workflow.include?("macos-latest")
checks["workflow.no_product_activation"] =
  !workflow.include?("DwmSetWindowAttribute") &&
  !workflow.include?("cargo run") &&
  !workflow.match?(/^\s*(?:bind|activate-product|deploy|release)\s*:/i)

checks["document.boundary"] =
  document.include?("default disabled") &&
  document.include?("productCargoFeatureDeclared=false") &&
  document.include?("productLifecycleWired=false") &&
  document.include?("production=false") &&
  document.include?("release=false")

failures = checks.reject { |_key, value| value }.keys
puts JSON.pretty_generate(
  schema: "hepta.ui.v4.windows-product-host-implementation-candidate-source-gate.v1",
  status: failures.empty? ? "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_SOURCE_ONLY" :
    "FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_SOURCE",
  source_implemented: failures.empty?,
  hosted_source_validated: false,
  default_feature_compile_validated: false,
  enabled_feature_compile_validated: false,
  enabled_feature_tests_validated: false,
  review_envelope_validated: false,
  product_cargo_feature_declared: false,
  product_module_registered: false,
  product_lifecycle_wired: false,
  product_bound: false,
  transient_system_material_bound: false,
  complete_profile_bound: false,
  system_material_bound: false,
  native_product_runtime: false,
  device_validation: false,
  production_authority: false,
  effect_authority: false,
  live_adapter_authority: false,
  promotion: false,
  release: false,
  checks: checks,
  failures: failures
)
exit(failures.empty? ? 0 : 1)
