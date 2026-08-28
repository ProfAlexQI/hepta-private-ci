# frozen_string_literal: true

require "json"
require "pathname"
require "yaml"

module HeptaWindowsSafetyDeviceGate
  def self.context
    root = Pathname.new(__dir__).join("..").expand_path
    paths = {
      host: root.join("apps/hepta-native/src/shared/hepta_windows_product_host_qualification_host.rs"),
      host_parts: root.join("apps/hepta-native/src/shared/hepta_windows_product_host_qualification_host"),
      safety_manifest: root.join("tools/hepta-ui-windows-product-host-safety/Cargo.toml"),
      safety_lib: root.join("tools/hepta-ui-windows-product-host-safety/src/lib.rs"),
      device_manifest: root.join("tools/hepta-ui-windows-product-host-device-drill/Cargo.toml"),
      device_main: root.join("tools/hepta-ui-windows-product-host-device-drill/src/main.rs"),
      device_parts: root.join("tools/hepta-ui-windows-product-host-device-drill/src/windows_drill"),
      contract: root.join("docs/ui/HEPTA_WINDOWS_PRODUCT_HOST_SAFETY_DEVICE_CLOSURE_V1.json"),
      device_schema: root.join("docs/ui/schemas/hepta.ui.v4.windows-product-host-device-drill.v1.schema.json"),
      workflow: root.join(".github/workflows/hepta-ui-v4-windows-product-host-safety-device-closure.yml"),
      document: root.join("docs/ui/HEPTA_UI_V4_TRANCHE_24_PRODUCT_HOST_SAFETY_DEVICE_CLOSURE_2026-08-28.md"),
      product_manifest: root.join("apps/hepta-native/Cargo.toml"),
      product_mod: root.join("apps/hepta-native/src/shared/mod.rs"),
      product_lifecycle: root.join("apps/hepta-native/src/shared/hepta_material_app_lifecycle.rs")
    }.freeze
    host_parts = Dir.glob(paths.fetch(:host_parts).join("part*.rs").to_s).sort.map { |path| Pathname.new(path) }
    device_parts = Dir.glob(paths.fetch(:device_parts).join("part*.rs").to_s).sort.map { |path| Pathname.new(path) }
    {
      paths: paths,
      host_parts: host_parts,
      device_parts: device_parts,
      host: ([paths.fetch(:host)] + host_parts).map(&:read).join("\n"),
      device: ([paths.fetch(:device_main)] + device_parts).map(&:read).join("\n"),
      safety_manifest: paths.fetch(:safety_manifest).read,
      device_manifest: paths.fetch(:device_manifest).read,
      contract: JSON.parse(paths.fetch(:contract).read),
      device_schema: JSON.parse(paths.fetch(:device_schema).read),
      workflow: paths.fetch(:workflow).read,
      product_manifest: paths.fetch(:product_manifest).read,
      product_mod: paths.fetch(:product_mod).read,
      product_lifecycle: paths.fetch(:product_lifecycle).read
    }
  end

  def self.host_checks(data)
    host = data.fetch(:host)
    {
      "host.feature_default_off" =>
        host.include?("HEPTA_WINDOWS_QUALIFICATION_FEATURE_DEFAULT_ENABLED: bool = false") &&
        data.fetch(:safety_manifest).include?("default = []") &&
        data.fetch(:device_manifest).include?("default = []"),
      "host.no_circular_device_prerequisite" =>
        !host.match?(/struct HeptaWindowsQualificationApproval[\s\S]*physical_device_validated/) &&
        !host.match?(/struct HeptaWindowsQualificationApproval[\s\S]*rollback_drill_validated/),
      "host.sealed_review" =>
        host.include?("HeptaWindowsProductHostReviewSeal") &&
        host.include?("ReviewDigestMismatch") &&
        host.include?("ImplementationCandidateMismatch") &&
        host.include?("request.approval.review_binding_digest != request.review_seal.binding_digest"),
      "host.rollback_failure_retains_identity" =>
        host.include?("RejectedUnsafe") &&
        host.include?("rollback_required: bool") &&
        host.include?("active_identity: Option") &&
        host.include?("failed_bind_with_failed_rollback_cannot_suspend_or_shutdown_as_unbound") &&
        host.include?("rollback_failure_retains_identity_and_blocks_false_safe_state"),
      "host.safe_lifecycle" =>
        host.include?("if self.rollback_required {\n            self.rollback_to_solid()?;") &&
        host.include?("QualificationEvidenceUnavailable") &&
        host.include?("debug_assert!(!self.rollback_required)"),
      "host.never_product" => %w[
        product_host_may_bind product_bound transient_system_material_bound
        complete_profile_bound system_material_bound native_product_runtime
      ].all? { |term| host.include?("#{term}: false") },
      "host.authority_false" => %w[
        production_authority effect_authority live_adapter_authority promotion release
      ].all? { |term| host.include?("#{term}: false") }
    }
  end
end
