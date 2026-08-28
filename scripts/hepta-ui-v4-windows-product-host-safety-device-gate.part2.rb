# frozen_string_literal: true

module HeptaWindowsSafetyDeviceGate
  def self.run
    data = context
    paths = data.fetch(:paths)
    checks = {}
    paths.reject { |name, _| %i[host_parts device_parts].include?(name) }.each do |name, path|
      checks["file.#{name}"] = path.file? && !path.zero?
    end
    checks["file.host_parts"] = data.fetch(:host_parts).length == 4 && data.fetch(:host_parts).all? { |path| path.file? && !path.zero? }
    checks["file.device_parts"] = data.fetch(:device_parts).length == 4 && data.fetch(:device_parts).all? { |path| path.file? && !path.zero? }
    checks.merge!(host_checks(data))

    device = data.fetch(:device)
    checks["device.real_windows_and_dwm"] = %w[
      RegisterClassExW CreateWindowExW DestroyWindow
      DwmSetWindowAttribute DwmGetWindowAttribute
      DWMSBT_MAINWINDOW DWMSBT_TRANSIENTWINDOW DWMSBT_NONE
    ].all? { |term| device.include?(term) }
    checks["device.complete_drills"] = %w[
      high_contrast transparency_allowed suspend shutdown
      PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_DEVICE_DRILL
    ].all? { |term| device.include?(term) }
    checks["device.no_product_runtime"] =
      !device.include?("matrix_sdk") && !device.include?("tokio::") &&
      !device.include?("HeptaPlatformMaterialHost")

    contract = data.fetch(:contract)
    checks["contract.boundary"] =
      contract.dig("safetyHost", "physicalDeviceValidationIsOutput") == true &&
      contract.dig("safetyHost", "rollbackDrillValidationIsOutput") == true &&
      contract.dig("safetyHost", "identityRetainedOnRollbackFailure") == true &&
      contract.dig("deviceProducer", "manualOnly") == true &&
      contract.fetch("qualification").reject { |key, _| key == "sourceImplemented" }.values.none? &&
      contract.fetch("authority").values.none?

    schema = data.fetch(:device_schema)
    checks["schema.still_fail_closed"] =
      schema.dig("properties", "status", "const") ==
        "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_DEVICE_DRILL" &&
      %w[productCargoFeatureDeclared productModuleRegistered productLifecycleWired automaticBindingAllowed implementationApproved productHostMayBind productBound transientSystemMaterialBound completeProfileBound systemMaterialBound nativeProductRuntime deviceValidation].all? do |key|
        schema.dig("$defs", "qualification", "properties", key, "const") == false
      end

    workflow = data.fetch(:workflow)
    checks["workflow.manual_physical"] =
      workflow.include?("workflow_dispatch:") &&
      workflow.include?("runs-on: [self-hosted, Windows, X64, hepta-ui-dwm]") &&
      workflow.include?("review_envelope_artifact_id") &&
      workflow.include?("implementation_review_approved") &&
      workflow.include?("operator_accepted")
    checks["workflow.compile_matrix"] =
      %w[ubuntu-latest windows-latest macos-latest].all? { |os| workflow.include?(os) } &&
      workflow.include?("default-off") && workflow.include?("explicit-feature")
    checks["workflow.read_only"] =
      workflow.include?("contents: read") && workflow.include?("actions: read") &&
      !workflow.match?(/^\s*(?:deploy|publish|promotion|release)\s*:/i)

    checks["product.not_wired"] =
      !data.fetch(:product_manifest).include?("hepta_ui_windows_system_material_v4") &&
      !data.fetch(:product_mod).include?("hepta_windows_product_host_qualification_host") &&
      !data.fetch(:product_lifecycle).include?("HeptaWindowsQualificationHost")

    failures = checks.reject { |_key, value| value }.keys
    puts JSON.pretty_generate(
      schema: "hepta.ui.v4.windows-product-host-safety-device-source-gate.v1",
      status: failures.empty? ? "PASS_WINDOWS_PRODUCT_HOST_SAFETY_DEVICE_SOURCE_ONLY" :
        "FAIL_WINDOWS_PRODUCT_HOST_SAFETY_DEVICE_SOURCE",
      source_implemented: failures.empty?,
      hosted_compile_validated: false,
      review_envelope_validated: false,
      physical_device_drill_validated: false,
      rollback_drill_validated: false,
      product_cargo_feature_declared: false,
      product_module_registered: false,
      product_lifecycle_wired: false,
      product_host_may_bind: false,
      product_bound: false,
      system_material_binding: false,
      production_authority: false,
      effect_authority: false,
      live_adapter_authority: false,
      promotion: false,
      release: false,
      checks: checks,
      failures: failures
    )
    exit(failures.empty? ? 0 : 1)
  end
end
