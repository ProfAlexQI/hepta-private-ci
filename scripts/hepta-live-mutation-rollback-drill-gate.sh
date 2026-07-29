#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
SOURCE_MODE="${HEPTA_MEMORY_INTELLIGENCE_SOURCE_MODE:-live_endpoint}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$REPO_ROOT/codex-rs/target/release/hepta}}"
INSTALLED_BIN="${HEPTA_LIVE_MUTATION_INSTALLED_BIN:-${HEPTA_INSTALLED_BIN:-${HEPTA_CODEX_INSTALLED_BIN:-$HOME/.local/opt/hepta/bin/hepta}}}"
BACKUP_ROOT="${HEPTA_BACKUP_ROOT:-$HOME/.openclaw/workspace/backups}"
SERVICE_LABEL="${HEPTA_LAUNCHD_LABEL:-ai.hepta.gateway}"
SERVICE_TARGET="${HEPTA_LAUNCHD_TARGET:-gui/$(id -u)/$SERVICE_LABEL}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

if [[ -z "${HEPTA_RELEASE_BIN:-}${HEPTA_CODEX_RELEASE_BIN:-}" && ! -f "$RELEASE_BIN" && -f "$INSTALLED_BIN" ]]; then
  RELEASE_BIN="$INSTALLED_BIN"
fi

release_sha=""
installed_sha=""
rollback_sha=""
release_size=0
installed_size=0
rollback_size=0

if [[ -f "$RELEASE_BIN" ]]; then
  release_sha="$(shasum -a 256 "$RELEASE_BIN" | awk '{print $1}')"
  release_size="$(wc -c <"$RELEASE_BIN" | tr -d ' ')"
fi
if [[ -f "$INSTALLED_BIN" ]]; then
  installed_sha="$(shasum -a 256 "$INSTALLED_BIN" | awk '{print $1}')"
  installed_size="$(wc -c <"$INSTALLED_BIN" | tr -d ' ')"
fi

rollback_candidates="$(
  find "$BACKUP_ROOT" -maxdepth 2 -type f -name hepta.previous 2>/dev/null \
    | grep '/hepta-active-binary-' \
    | sort || true
)"
rollback_backup_count="$(printf '%s\n' "$rollback_candidates" | sed '/^$/d' | wc -l | tr -d ' ')"
latest_rollback_backup="$(printf '%s\n' "$rollback_candidates" | sed '/^$/d' | tail -n 1)"

rollback_backup_executable=false
if [[ -n "$latest_rollback_backup" && -f "$latest_rollback_backup" ]]; then
  rollback_sha="$(shasum -a 256 "$latest_rollback_backup" | awk '{print $1}')"
  rollback_size="$(wc -c <"$latest_rollback_backup" | tr -d ' ')"
  if [[ -x "$latest_rollback_backup" ]]; then
    rollback_backup_executable=true
  fi
fi

installed_dir="$(dirname "$INSTALLED_BIN")"
installed_dir_present=false
installed_dir_writable=false
if [[ -d "$installed_dir" ]]; then
  installed_dir_present=true
  if [[ -w "$installed_dir" ]]; then
    installed_dir_writable=true
  fi
fi

case "$SOURCE_MODE" in
  live_endpoint)
    [[ -z "${HEPTA_ROUTE_PARITY_NATIVE_REPORT_FIXTURE:-}" ]] || {
      echo "HEPTA_ROUTE_PARITY_NATIVE_REPORT_FIXTURE requires offline_fixture source mode" >&2
      exit 2
    }
    MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
    PACKET_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-operator-approval-packet")"
    RELEASE_JSON="$(curl -fsS "$BASE_URL/api/hepta-release-hardening-status-gate")"
    CORE_JSON="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"
    DEPENDENCY_JSON="$(curl -fsS "$BASE_URL/api/hepta-engine-dependency-closure")"
    ;;
  offline_fixture)
    source "$REPO_ROOT/scripts/lib/hepta-route-parity-native-report-fixture.sh"
    hepta_load_route_parity_native_reports
    MEMORY_JSON="$(jq -c '.memory_capability_absorption_inventory' <<<"$HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON")"
    PACKET_JSON="$(jq -c '.public_ga_operator_approval_packet' <<<"$HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON")"
    RELEASE_JSON="$(jq -c '.release_hardening_status_gate' <<<"$HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON")"
    CORE_JSON="$(jq -c '.core_fusion_readiness' <<<"$HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON")"
    DEPENDENCY_JSON="$(jq -c '.engine_dependency_closure' <<<"$HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON")"
    ;;
  *)
    echo "HEPTA_MEMORY_INTELLIGENCE_SOURCE_MODE must be live_endpoint or offline_fixture" >&2
    exit 2
    ;;
esac

jq -n -e \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --arg rollback_sha "$rollback_sha" \
  --arg latest_rollback_backup "$latest_rollback_backup" \
  --argjson release_size "$release_size" \
  --argjson installed_size "$installed_size" \
  --argjson rollback_size "$rollback_size" \
  --argjson rollback_backup_count "$rollback_backup_count" \
  --argjson rollback_backup_executable "$rollback_backup_executable" \
  --argjson installed_dir_present "$installed_dir_present" \
  --argjson installed_dir_writable "$installed_dir_writable" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_JSON" \
  --argjson packet "$PACKET_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson core "$CORE_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  '
    $release_sha != ""
    and $installed_sha != ""
    and $rollback_sha != ""
    and $release_sha == $installed_sha
    and $rollback_sha != $installed_sha
    and $latest_rollback_backup != ""
    and $release_size > 0
    and $installed_size > 0
    and $rollback_size > 0
    and $rollback_backup_count >= 1
    and $rollback_backup_executable == true
    and $installed_dir_present == true
    and $installed_dir_writable == true
    and $min_long_soak_samples >= 24
    and $memory.runtime == "hepta"
    and ($memory.status == "attention" or $memory.status == "ready")
    and $memory.surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.live_mutation_enabled_count == 0
    and ($memory.side_effects | to_entries | all(.value == false))
    and $packet.runtime == "hepta"
    and $packet.status == "ready"
    and $packet.approval_packet_ready == true
    and $packet.safe_default_mode == "plan_only_no_live_mutation"
    and $packet.irreversible_actions_blocked_by_default == true
    and ($packet.side_effects | to_entries | all(.value == false))
    and $release.runtime == "hepta"
    and ($release.status == "attention" or $release.status == "ready")
    and $release.release_hardening_status_gate_ready == true
    and $release.live_execution_enabled_count == 0
    and $release.side_effects.launchd_mutated == false
    and $release.side_effects.release_artifact_written == false
    and ($release.side_effects | to_entries | all(.value == false))
    and $core.runtime == "hepta"
    and $core.status == "ready"
    and $core.full_fusion_complete == true
    and $core.installed_service_binary != ""
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.full_fusion_complete == true
    and $dependency.remaining_direct_dependency_count == 0
    and ($dependency.blockers | length) == 0
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_rollback_drill_gate" \
  --arg release_bin "$RELEASE_BIN" \
  --arg installed_bin "$INSTALLED_BIN" \
  --arg installed_dir "$installed_dir" \
  --arg latest_rollback_backup "$latest_rollback_backup" \
  --arg service_label "$SERVICE_LABEL" \
  --arg service_target "$SERVICE_TARGET" \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --arg rollback_sha "$rollback_sha" \
  --argjson release_size "$release_size" \
  --argjson installed_size "$installed_size" \
  --argjson rollback_size "$rollback_size" \
  --argjson rollback_backup_count "$rollback_backup_count" \
  --argjson rollback_backup_executable "$rollback_backup_executable" \
  --argjson installed_dir_present "$installed_dir_present" \
  --argjson installed_dir_writable "$installed_dir_writable" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_JSON" \
  --argjson packet "$PACKET_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson core "$CORE_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    drill_mode:"dry_run_no_restore_no_restart",
    rollback_plan_ready:true,
    rollback_execution_enabled:false,
    operator_approval_required_before_execution:true,
    release_installed_sha_match:($release_sha != "" and $release_sha == $installed_sha),
    rollback_would_change_installed_binary:($rollback_sha != "" and $rollback_sha != $installed_sha),
    release_bin:$release_bin,
    installed_bin:$installed_bin,
    installed_dir:$installed_dir,
    installed_dir_present:$installed_dir_present,
    installed_dir_writable:$installed_dir_writable,
    latest_rollback_backup:$latest_rollback_backup,
    rollback_backup_count:$rollback_backup_count,
    rollback_backup_executable:$rollback_backup_executable,
    release_sha:$release_sha,
    installed_sha:$installed_sha,
    rollback_sha:$rollback_sha,
    release_size:$release_size,
    installed_size:$installed_size,
    rollback_size:$rollback_size,
    service_label:$service_label,
    service_target:$service_target,
    minimum_long_soak_required_samples:$min_long_soak_samples,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    live_execution_enabled_count:$release.live_execution_enabled_count,
    safe_default_mode:$packet.safe_default_mode,
    core_full_fusion_complete:$core.full_fusion_complete,
    remaining_direct_dependency_count:$dependency.remaining_direct_dependency_count,
    rollback_dry_run_commands:[
      ("cp " + ($latest_rollback_backup | @sh) + " " + ($installed_bin | @sh)),
      ("chmod +x " + ($installed_bin | @sh)),
      ("launchctl kickstart -k " + ($service_target | @sh)),
      "scripts/hepta-watchdog.sh",
      ("HEPTA_SOAK_SAMPLES=" + ($min_long_soak_samples|tostring) + " HEPTA_SOAK_INTERVAL_SECONDS=5 scripts/hepta-live-soak.sh")
    ],
    required_before_execution:[
      "explicit_operator_approval_id",
      "current_installed_binary_backup_created_after_approval",
      "rollback_commands_reviewed",
      "single_surface_activation_scope",
      "post_restore_watchdog",
      "post_restore_minimum_24_sample_soak",
      "side_effect_receipt_with_no_secret_values"
    ],
    local_reads:{
      backup_file_read:true,
      release_file_read:true,
      installed_file_read:true
    },
    side_effects:{
      backup_file_written:false,
      release_file_written:false,
      installed_file_read_revealed_secret:false,
      filesystem_written:false,
      installed_binary_replaced:false,
      launchd_mutated:false,
      service_restarted:false,
      gateway_mutation_performed:false,
      external_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false
    }
  }')"

printf '%s\n' "$report"
echo "Hepta live mutation rollback drill gate passed"
