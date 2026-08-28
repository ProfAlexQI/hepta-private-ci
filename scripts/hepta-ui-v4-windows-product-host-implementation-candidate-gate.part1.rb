#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "pathname"

root = Pathname.new(__dir__).join("..").expand_path
files = {
  implementation: root.join("apps/hepta-native/src/shared/hepta_windows_product_host_implementation.rs"),
  review: root.join("apps/hepta-native/src/shared/hepta_windows_product_host_integration_review.rs"),
  product_mod: root.join("apps/hepta-native/src/shared/mod.rs"),
  lifecycle: root.join("apps/hepta-native/src/shared/hepta_material_app_lifecycle.rs"),
  harness: root.join("tools/hepta-ui-windows-product-host-implementation/Cargo.toml"),
  harness_lib: root.join("tools/hepta-ui-windows-product-host-implementation/src/lib.rs"),
  contract: root.join("docs/ui/HEPTA_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_V1.json"),
  schema: root.join("docs/ui/schemas/hepta.ui.v4.windows-product-host-implementation-receipt.v1.schema.json"),
  document: root.join("docs/ui/HEPTA_UI_V4_TRANCHE_22_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_2026-08-28.md"),
  workflow: root.join(".github/workflows/hepta-ui-v4-windows-product-host-implementation-candidate.yml")
}.freeze

checks = {}
files.each { |name, path| checks["file.#{name}"] = path.file? && !path.zero? }

implementation_parts = (1..3).map do |index|
  root.join("apps/hepta-native/src/shared/hepta_windows_product_host_implementation/part#{index}.rs")
end
implementation_parts.each_with_index do |path, index|
  checks["file.implementation_part#{index + 1}"] = path.file? && !path.zero?
end
implementation = ([files.fetch(:implementation)] + implementation_parts).map(&:read).join
review = files.fetch(:review).read
product_mod = files.fetch(:product_mod).read
lifecycle = files.fetch(:lifecycle).read
harness = files.fetch(:harness).read
harness_lib = files.fetch(:harness_lib).read
contract = JSON.parse(files.fetch(:contract).read)
schema = JSON.parse(files.fetch(:schema).read)
document = files.fetch(:document).read
workflow = files.fetch(:workflow).read

checks["implementation.feature_contract"] =
  implementation.include?("hepta_ui_windows_system_material_v4") &&
  implementation.include?("HEPTA_WINDOWS_PRODUCT_HOST_FEATURE_DEFAULT_ENABLED: bool = false") &&
  implementation.include?("HEPTA_WINDOWS_PRODUCT_HOST_AUTOMATIC_BINDING_ALLOWED: bool = false") &&
  implementation.include?("cfg!(feature = \"hepta_ui_windows_system_material_v4\")")
checks["implementation.current_state_false"] = %w[
  HEPTA_WINDOWS_PRODUCT_HOST_PRODUCT_WIRED
  HEPTA_WINDOWS_PRODUCT_HOST_LIFECYCLE_WIRED
  HEPTA_WINDOWS_PRODUCT_HOST_RUNTIME_VALIDATED
  HEPTA_WINDOWS_PRODUCT_HOST_PRODUCT_BOUND
  HEPTA_WINDOWS_PRODUCT_HOST_SYSTEM_MATERIAL_BOUND
  HEPTA_WINDOWS_PRODUCT_HOST_DEVICE_VALIDATED
  HEPTA_WINDOWS_PRODUCT_HOST_OPERATOR_ACCEPTANCE
  HEPTA_WINDOWS_PRODUCT_HOST_PRODUCTION_AUTHORITY
  HEPTA_WINDOWS_PRODUCT_HOST_EFFECT_AUTHORITY
  HEPTA_WINDOWS_PRODUCT_HOST_LIVE_ADAPTER_AUTHORITY
  HEPTA_WINDOWS_PRODUCT_HOST_PROMOTION
  HEPTA_WINDOWS_PRODUCT_HOST_RELEASE
].all? { |name| implementation.include?("#{name}: bool = false") }
checks["implementation.review_gate"] =
  implementation.include?("EligibleForImplementationReview") &&
  implementation.include?("remains_review_only") &&
  implementation.include?("grants_no_authority") &&
  implementation.include?("ReviewEnvelopeRejected")
checks["implementation.governance_prerequisites"] = %w[
  ImplementationApprovalMissing
  OperatorAcceptanceMissing
  DeviceValidationMissing
  RollbackDrillMissing
  FeatureDisabled
  TransparencyDisabled
  HighContrast
].all? { |term| implementation.include?(term) }
checks["implementation.explicit_identity"] =
  implementation.include?("root_window_index") &&
  implementation.include?("root_window_generation") &&
  implementation.include?("root_hwnd") &&
  implementation.include?("transient_window_index") &&
  implementation.include?("transient_window_generation") &&
  implementation.include?("transient_hwnd") &&
  implementation.include?("root_hwnd != self.transient_hwnd")
checks["implementation.verified_transaction"] =
  implementation.include?("bind_verified") &&
  implementation.include?("rollback_to_solid_verified") &&
  implementation.include?("BackendBindingReceiptRejected") &&
  implementation.include?("RollbackReceiptRejected") &&
  implementation.include?("root_mica_exact") &&
  implementation.include?("transient_acrylic_exact") &&
  implementation.include?("root_none_exact") &&
  implementation.include?("transient_none_exact")
checks["implementation.lifecycle_cleanup"] =
  implementation.include?("pub fn rollback_to_solid") &&
  implementation.include?("pub fn suspend") &&
  implementation.include?("pub fn shutdown") &&
  implementation.include?("HostShutdown")
checks["implementation.no_automatic_or_system_calls"] =
  !implementation.include?("unsafe") &&
  !implementation.include?("script_mod") &&
  !implementation.include?("DwmSetWindowAttribute") &&
  !implementation.include?("DwmGetWindowAttribute") &&
  !implementation.include?("bind_material_runtime") &&
  !implementation.include?("HeptaPlatformMaterialHost::apply") &&
  !implementation.match?(/impl\s+Drop/)
checks["implementation.tests"] = %w[
  default_build_cannot_activate_product_materials
  explicit_approved_activation_binds_without_granting_authority
  every_governance_prerequisite_is_fail_closed
  invalid_binding_rolls_back_and_never_publishes_bound_state
  explicit_rollback_suspend_and_shutdown_are_unbound
