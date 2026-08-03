# frozen_string_literal: true

require "digest"
require "json"

module HeptaPreflightRuntimeCompanion
  BUILD_SCHEMA_V1 = "hepta_build_provenance_v1"
  BUILD_SCHEMA_V2 = "hepta_build_provenance_v2"
  FINAL_SCHEMA_V1 = "hepta_preflight_final_receipt_v1"
  FINAL_SCHEMA_V2 = "hepta_preflight_final_receipt_v2"
  SHA256_PATTERN = /\A[0-9a-f]{64}\z/

  module_function

  def validate!(provenance, final_receipt)
    case provenance["schema_version"]
    when BUILD_SCHEMA_V1
      raise "invalid legacy preflight final receipt schema" unless final_receipt["schema"] ==
        FINAL_SCHEMA_V1
      nil
    when BUILD_SCHEMA_V2
      raise "invalid runtime-bound preflight final receipt schema" unless final_receipt["schema"] ==
        FINAL_SCHEMA_V2
      runtime_companions = provenance["runtime_companions"]
      raise "invalid runtime companion provenance" unless runtime_companions.is_a?(Hash) &&
        runtime_companions["bound"] == true
      artifacts = runtime_companions["artifacts"]
      raise "invalid runtime companion artifact set" unless artifacts.is_a?(Array) &&
        artifacts.length == 1
      artifact = artifacts.fetch(0)
      raise "invalid runtime companion artifact" unless artifact.keys.sort == %w[id name sha256] &&
        artifact["id"] == "code-mode-host" &&
        ["codex-code-mode-host", "codex-code-mode-host.exe"].include?(artifact["name"]) &&
        artifact["sha256"]&.match?(SHA256_PATTERN)
      aggregate = Digest::SHA256.hexdigest(
        "#{artifact.fetch("id")}\t#{artifact.fetch("name")}\t#{artifact.fetch("sha256")}\n"
      )
      raise "invalid runtime companion aggregate" unless runtime_companions["aggregate_sha256"] ==
        aggregate
      raise "preflight final receipt/runtime companion drifted" unless final_receipt[
        "runtime_companions_sha256"
      ] == aggregate
      aggregate
    else
      raise "invalid preflight provenance schema"
    end
  end

  def source_binding(runtime_companions_sha256)
    return {} unless runtime_companions_sha256

    {"runtime_companions_sha256" => runtime_companions_sha256}
  end

  def self_test!(root:, provenance:, final_receipt:, pack:, verify:)
    runtime_artifact_sha256 = "c" * 64
    runtime_artifact = {
      "id" => "code-mode-host",
      "name" => "codex-code-mode-host",
      "sha256" => runtime_artifact_sha256
    }
    runtime_companions_sha256 = Digest::SHA256.hexdigest(
      "code-mode-host\tcodex-code-mode-host\t#{runtime_artifact_sha256}\n"
    )
    runtime_provenance = provenance.merge(
      "schema_version" => BUILD_SCHEMA_V2,
      "runtime_companions" => {
        "bound" => true,
        "aggregate_sha256" => runtime_companions_sha256,
        "artifacts" => [runtime_artifact]
      }
    )
    runtime_provenance_text = JSON.generate(runtime_provenance)
    runtime_final_receipt = final_receipt.merge(
      "schema" => FINAL_SCHEMA_V2,
      "runtime_companions_sha256" => runtime_companions_sha256,
      "build_provenance_sha256" => Digest::SHA256.hexdigest(runtime_provenance_text)
    )
    runtime_log = [
      "[hepta-preflight] metadata",
      "[hepta-preflight] release",
      "[hepta-preflight-provenance] #{runtime_provenance_text}",
      "[hepta-preflight-final] #{JSON.generate(runtime_final_receipt)}",
      "Hepta preflight passed"
    ].join("\n") + "\n"
    runtime_log_path = File.join(root, "runtime-preflight.log")
    runtime_output = File.join(root, "runtime-bound")
    File.binwrite(runtime_log_path, runtime_log)
    runtime_summary = pack.call(runtime_log_path, runtime_output)
    verify.call(File.join(runtime_output, "preflight-summary.json"))
    raise "runtime companion aggregate was not summary-bound" unless runtime_summary.dig(
      "source", "runtime_companions_sha256"
    ) == runtime_companions_sha256

    drifted_runtime_log = runtime_log.sub(
      %Q{"runtime_companions_sha256":"#{runtime_companions_sha256}"},
      %Q{"runtime_companions_sha256":"#{"d" * 64}"}
    )
    drifted_runtime_path = File.join(root, "drifted-runtime-preflight.log")
    File.binwrite(drifted_runtime_path, drifted_runtime_log)
    runtime_drift_rejected = false
    begin
      pack.call(drifted_runtime_path, File.join(root, "drifted-runtime"))
    rescue StandardError
      runtime_drift_rejected = true
    end
    raise "runtime companion receipt drift was accepted" unless runtime_drift_rejected
  end
end
