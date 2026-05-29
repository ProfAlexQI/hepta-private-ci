#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

RELEASE_ARTIFACT_LOCK_JSON="$(
  capture_json_report \
    "hepta-terminal-release-artifact-non-write-lock-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-release-artifact-non-write-lock-gate.sh
)"

PUBLIC_GA_READINESS_JSON="$(
  capture_json_report \
    "hepta-public-ga-readiness" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-readiness.sh
)"

OPERATOR_PACKET_JSON="$(
  capture_json_report \
    "hepta-public-ga-operator-approval-packet" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-operator-approval-packet.sh
)"

release_artifact_lock_report_sha256="$(sha256_text "$RELEASE_ARTIFACT_LOCK_JSON")"
public_ga_readiness_report_sha256="$(sha256_text "$PUBLIC_GA_READINESS_JSON")"
operator_packet_report_sha256="$(sha256_text "$OPERATOR_PACKET_JSON")"
public_distribution_lock_hash_sha256="$(sha256_text "hepta-terminal-public-distribution-non-publication-lock:index:$release_artifact_lock_report_sha256:$public_ga_readiness_report_sha256:$operator_packet_report_sha256")"
public_distribution_policy_hash_sha256="$(sha256_text "hepta-terminal-public-distribution-non-publication-lock:policy:$release_artifact_lock_report_sha256:$public_ga_readiness_report_sha256:$operator_packet_report_sha256")"
public_distribution_side_effect_hash_sha256="$(sha256_text "hepta-terminal-public-distribution-non-publication-lock:side-effects:$release_artifact_lock_report_sha256:$public_ga_readiness_report_sha256:$operator_packet_report_sha256")"

jq -n -e \
  --argjson release "$RELEASE_ARTIFACT_LOCK_JSON" \
  --argjson public_ga "$PUBLIC_GA_READINESS_JSON" \
  --argjson operator "$OPERATOR_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $release.runtime == "hepta"
    and $release.status == "ready"
    and $release.gate == "hepta_terminal_release_artifact_non_write_lock_gate"
    and $release.release_artifact_non_write_lock_ready == true
    and $release.release_artifact_non_write_denied_by_count == 87
    and $release.release_build_executed == false
    and $release.native_signing_allowed == false
    and $release.native_notarization_allowed == false
    and $release.public_distribution_artifact_write_allowed == false
    and $release.release_artifact_write_allowed == false
    and $release.public_artifact_write_allowed == false
    and $release.public_release_claim_allowed == false
    and $release.public_ga_claim_allowed == false
    and ($release.side_effects | to_entries | all(.value == false))
    and $public_ga.runtime == "hepta"
    and $public_ga.status == "ready"
    and $public_ga.endpoint == "/api/hepta-public-ga-readiness"
    and (
      $public_ga.public_ga_ready == true
      or (
        $public_ga.public_ga_ready == false
        and $public_ga.blocker_count == 1
        and ($public_ga.blockers | length) == 1
        and $public_ga.blockers[0] == "telegram_live_poll_model_send_soak_not_complete"
      )
    )
    and $public_ga.public_ga_claimed == false
    and $public_ga.reports_synchronized == true
    and $public_ga.missing_route_count == 0
    and $public_ga.native_gateway_source_command_count == 69
    and $public_ga.expected_external_blockers.native_packaging_public_distribution_artifact_written == false
    and ($public_ga.side_effects | to_entries | all(.value == false))
    and $operator.runtime == "hepta"
    and $operator.status == "ready"
    and $operator.endpoint == "/api/hepta-public-ga-operator-approval-packet"
    and $operator.approval_packet_ready == true
    and $operator.safe_default_mode == "plan_only_no_live_mutation"
    and $operator.required_operator_approval_count == 8
    and (
      $operator.public_ga_ready == true
      or (
        $operator.public_ga_ready == false
        and $operator.public_ga_blocker_count == 1
      )
    )
    and $operator.reports_synchronized == true
    and $operator.missing_route_count == 0
    and $operator.native_gateway_source_command_count == 69
    and ($operator.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_public_distribution_non_publication_lock_gate" \
  --arg release_artifact_lock_report_sha256 "$release_artifact_lock_report_sha256" \
  --arg public_ga_readiness_report_sha256 "$public_ga_readiness_report_sha256" \
  --arg operator_packet_report_sha256 "$operator_packet_report_sha256" \
  --arg public_distribution_lock_hash_sha256 "$public_distribution_lock_hash_sha256" \
  --arg public_distribution_policy_hash_sha256 "$public_distribution_policy_hash_sha256" \
  --arg public_distribution_side_effect_hash_sha256 "$public_distribution_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson release "$RELEASE_ARTIFACT_LOCK_JSON" \
  --argjson public_ga "$PUBLIC_GA_READINESS_JSON" \
  --argjson operator "$OPERATOR_PACKET_JSON" \
  '
    ([
      "terminal_public_distribution_non_publication_lock_recording_denied",
      "terminal_public_distribution_non_publication_lock_persistence_denied",
      "terminal_public_distribution_non_publication_lock_materialization_denied",
      "terminal_public_distribution_non_publication_lock_filesystem_write_denied",
      "public_distribution_publication_denied",
      "public_ga_claim_denied_by_public_distribution_lock",
      "public_release_claim_denied_by_public_distribution_lock",
      "public_distribution_artifact_write_denied",
      "external_public_claim_denied",
      "operator_approval_required_before_public_distribution",
      "operator_packet_recording_not_present",
      "publication_evidence_persistence_denied"
    ] + $release.denied_by_release_artifact_non_write_lock) as $public_distribution_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_public_distribution_non_publication_lock_schema_version:"terminal_public_distribution_non_publication_lock_v1",
      minimum_required_samples:$min_long_soak_samples,
      public_distribution_non_publication_lock_ready:true,
      public_distribution_non_publication_lock_mode:"schema_only_public_distribution_publication_blocked",
      public_distribution_non_publication_lock_decision:"public_distribution_and_ga_claim_observed_without_publication_artifact_write_or_operator_approval",
      source_release_artifact_lock_gate:$release.gate,
      source_public_ga_readiness_endpoint:$public_ga.endpoint,
      source_operator_packet_endpoint:$operator.endpoint,
      source_release_artifact_lock_report_sha256:$release_artifact_lock_report_sha256,
      source_public_ga_readiness_report_sha256:$public_ga_readiness_report_sha256,
      source_operator_packet_report_sha256:$operator_packet_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_release_artifact_non_write_lock_ready:$release.release_artifact_non_write_lock_ready,
      source_release_artifact_non_write_denied_by_count:$release.release_artifact_non_write_denied_by_count,
      source_release_artifact_lock_family_count:($release.release_artifact_lock_families | length),
      source_active_state_installed_sha256:$release.source_active_state_installed_sha256,
      source_active_state_release_sha256:$release.source_active_state_release_sha256,
      source_native_signing_notarization_deferred:$release.source_native_signing_notarization_deferred,
      source_native_public_distribution_artifact_written:$release.source_native_public_distribution_artifact_written,
      source_release_hardening_live_execution_enabled_count:$release.source_release_hardening_live_execution_enabled_count,
      source_public_ga_ready:$public_ga.public_ga_ready,
      source_public_ga_blocker_count:$public_ga.blocker_count,
      source_public_ga_claimed:$public_ga.public_ga_claimed,
      source_public_ga_reports_synchronized:$public_ga.reports_synchronized,
      source_public_ga_missing_route_count:$public_ga.missing_route_count,
      source_public_ga_native_packaging_public_distribution_artifact_written:$public_ga.expected_external_blockers.native_packaging_public_distribution_artifact_written,
      source_operator_packet_ready:$operator.approval_packet_ready,
      source_operator_packet_safe_default_mode:$operator.safe_default_mode,
      source_operator_packet_required_operator_approval_count:$operator.required_operator_approval_count,
      source_operator_packet_public_ga_ready:$operator.public_ga_ready,
      source_operator_packet_public_ga_blocker_count:$operator.public_ga_blocker_count,
      source_operator_packet_reports_synchronized:$operator.reports_synchronized,
      active_binary_sha_consistent:($release.source_active_state_installed_sha256 == $release.source_active_state_release_sha256),
      release_artifact_write_lock_enforced:true,
      public_distribution_non_publication_enforced:true,
      public_ga_non_claim_enforced:true,
      operator_approval_required:true,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      install_execution_allowed:false,
      release_build_required:false,
      release_build_executed:false,
      active_binary_mutation_allowed:false,
      active_service_restart_allowed:false,
      launchd_restart_allowed:false,
      rollback_execution_allowed:false,
      rollback_restore_allowed:false,
      native_packaging_execution_allowed:false,
      native_signing_allowed:false,
      native_notarization_allowed:false,
      native_stapling_allowed:false,
      public_distribution_publication_allowed:false,
      public_distribution_artifact_write_allowed:false,
      release_artifact_pack_execution_allowed:false,
      recurring_watchdog_install_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      public_release_published:false,
      public_ga_claimed:false,
      external_public_claim_performed:false,
      external_public_distribution_performed:false,
      terminal_public_distribution_non_publication_lock_recorded:false,
      terminal_public_distribution_non_publication_lock_persisted:false,
      terminal_public_distribution_non_publication_lock_materialized:false,
      terminal_public_distribution_non_publication_lock_filesystem_written:false,
      public_distribution_lock_hash_sha256:$public_distribution_lock_hash_sha256,
      public_distribution_policy_hash_sha256:$public_distribution_policy_hash_sha256,
      public_distribution_side_effect_hash_sha256:$public_distribution_side_effect_hash_sha256,
      public_distribution_source_hashes:[
        $release_artifact_lock_report_sha256,
        $public_ga_readiness_report_sha256,
        $operator_packet_report_sha256
      ],
      public_distribution_lock_families:[
        {
          id:"release-artifact-non-write-source",
          ready:true,
          blocked:true,
          denied_by_count:$release.release_artifact_non_write_denied_by_count,
          reason:"release-artifact non-write source denies build, install, signing, notarization, artifact writes, public claims, and live mutation"
        },
        {
          id:"public-ga-readiness-non-claim-boundary",
          ready:true,
          blocked:true,
          public_ga_ready:$public_ga.public_ga_ready,
          public_ga_claimed:$public_ga.public_ga_claimed,
          reason:"public GA readiness is observational and does not publish or claim GA"
        },
        {
          id:"operator-packet-non-approval-boundary",
          ready:true,
          blocked:true,
          approval_packet_ready:$operator.approval_packet_ready,
          operator_approval_recorded:false,
          required_operator_approval_count:$operator.required_operator_approval_count,
          reason:"operator packet is a plan-only checklist, not recorded approval"
        },
        {
          id:"public-distribution-artifact-write-boundary",
          ready:true,
          blocked:true,
          public_distribution_artifact_write_allowed:false,
          release_artifact_write_allowed:false,
          public_artifact_write_allowed:false,
          reason:"public distribution, release, and public artifact writes remain denied"
        },
        {
          id:"publication-external-send-boundary",
          ready:true,
          blocked:true,
          public_release_published:false,
          external_public_claim_performed:false,
          external_public_distribution_performed:false,
          reason:"external publication, external claims, and distribution sends remain denied"
        },
        {
          id:"terminal-public-distribution-lock-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_public_distribution_non_publication_lock_recorded:false,
          terminal_public_distribution_non_publication_lock_persisted:false,
          terminal_public_distribution_non_publication_lock_materialized:false,
          terminal_public_distribution_non_publication_lock_filesystem_written:false,
          reason:"public-distribution non-publication lock is report-only and not persisted or materialized"
        }
      ],
      public_distribution_denied_by_count:($public_distribution_denied | length),
      denied_by_public_distribution_non_publication_lock:$public_distribution_denied,
      side_effects:{
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        coding_agent_spawned:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        runtime_store_mutated:false,
        gateway_event_enqueued:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        active_runtime_auto_rebase_performed:false,
        active_runtime_dependency_mutated:false,
        install_executed:false,
        release_build_executed:false,
        native_packaging_executed:false,
        native_signing_executed:false,
        native_notarization_executed:false,
        native_stapling_executed:false,
        public_distribution_artifact_written:false,
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        recurring_watchdog_installed:false,
        release_artifact_pack_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        external_public_claim_performed:false,
        external_public_distribution_performed:false,
        terminal_public_distribution_non_publication_lock_recorded:false,
        terminal_public_distribution_non_publication_lock_persisted:false,
        terminal_public_distribution_non_publication_lock_materialized:false,
        terminal_public_distribution_non_publication_lock_filesystem_written:false,
        terminal_release_artifact_non_write_lock_recorded:false,
        terminal_release_artifact_non_write_lock_persisted:false,
        terminal_release_artifact_non_write_lock_materialized:false,
        terminal_release_artifact_non_write_lock_filesystem_written:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
        filesystem_written:false,
        workspace_write_performed:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }')"

printf '%s\n' "$report"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .public_distribution_non_publication_lock_ready == true
  and .source_release_artifact_non_write_lock_ready == true
  and .source_release_artifact_non_write_denied_by_count == 87
  and (
    .source_public_ga_ready == true
    or (
      .source_public_ga_ready == false
      and .source_public_ga_blocker_count == 1
    )
  )
  and .source_public_ga_claimed == false
  and .source_operator_packet_ready == true
  and .operator_approval_recorded == false
  and .active_binary_sha_consistent == true
  and .release_artifact_write_lock_enforced == true
  and .public_distribution_non_publication_enforced == true
  and .public_ga_non_claim_enforced == true
  and .public_distribution_artifact_write_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .external_public_claim_performed == false
  and .external_public_distribution_performed == false
  and .terminal_public_distribution_non_publication_lock_recorded == false
  and .terminal_public_distribution_non_publication_lock_persisted == false
  and .terminal_public_distribution_non_publication_lock_materialized == false
  and .terminal_public_distribution_non_publication_lock_filesystem_written == false
  and .public_distribution_denied_by_count == 99
  and (.public_distribution_lock_families | length) == 6
  and (.public_distribution_lock_families | all(.ready == true and .blocked == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta terminal public distribution non-publication lock gate passed"
