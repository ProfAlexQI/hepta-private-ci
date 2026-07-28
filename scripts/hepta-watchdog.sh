#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
source "$REPO_ROOT/scripts/lib/hepta-watchdog-release-evidence-v1.sh"
source "$REPO_ROOT/scripts/lib/hepta-watchdog-product-boundary-v1.sh"

RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$REPO_ROOT/codex-rs/target/release/hepta}}"
INSTALLED_BIN="${HEPTA_INSTALLED_BIN:-${HEPTA_CODEX_INSTALLED_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
WATCHDOG_MODE="${HEPTA_WATCHDOG_MODE:-deployment-consistency}"
CANDIDATE_MANIFEST="${HEPTA_CANDIDATE_MANIFEST:-${HEPTA_RELEASE_MANIFEST:-${HEPTA_CODEX_RELEASE_MANIFEST:-}}}"
INSTALLED_RECEIPT="${HEPTA_INSTALLED_RECEIPT:-${HEPTA_INSTALLED_MANIFEST:-${HEPTA_CODEX_INSTALLED_MANIFEST:-$INSTALLED_BIN.manifest}}}"
EXPECTED_SOURCE_COMMIT="${HEPTA_EXPECTED_SOURCE_COMMIT:-}"
PRODUCT_BOUNDARY="${HEPTA_PRODUCT_BOUNDARY:-$REPO_ROOT/docs/decisions/hepta-product-boundary-v1.json}"

usage() {
  cat >&2 <<'EOF'
usage: scripts/hepta-watchdog.sh [--mode MODE] [--candidate-manifest PATH] [--installed-receipt PATH] [--expected-source-commit SHA]

Modes:
  deployment-consistency  Validate active health, a source-bound candidate,
                          a source-bound installed receipt, and byte equality.
                          This is the fail-closed default.
  active-health            Probe only the running service. This explicit mode
                          makes no artifact or deployment claim.
  candidate-artifact       Validate only candidate artifact provenance.
  deployed-receipt         Validate only the installed/deployed receipt.
EOF
  exit 2
}

while (( $# > 0 )); do
  case "$1" in
    --mode)
      shift
      [[ $# -gt 0 ]] || usage
      WATCHDOG_MODE="$1"
      ;;
    --candidate-manifest)
      shift
      [[ $# -gt 0 ]] || usage
      CANDIDATE_MANIFEST="$1"
      ;;
    --installed-receipt)
      shift
      [[ $# -gt 0 ]] || usage
      INSTALLED_RECEIPT="$1"
      ;;
    --expected-source-commit)
      shift
      [[ $# -gt 0 ]] || usage
      EXPECTED_SOURCE_COMMIT="$1"
      ;;
    -h|--help) usage ;;
    *) usage ;;
  esac
  shift
done

if [[ -z "$CANDIDATE_MANIFEST" ]]; then
  if [[ "$(basename "$(dirname "$RELEASE_BIN")")" == "bin" ]]; then
    CANDIDATE_MANIFEST="$(dirname "$(dirname "$RELEASE_BIN")")/manifest.json"
  else
    CANDIDATE_MANIFEST="$RELEASE_BIN.manifest"
  fi
fi

if [[ -n "$EXPECTED_SOURCE_COMMIT" && ! "$EXPECTED_SOURCE_COMMIT" =~ ^[0-9a-f]{40}$ ]]; then
  echo "expected source commit must be a full Git SHA" >&2
  exit 2
fi

require_active_health=false
require_candidate_artifact=false
require_deployed_receipt=false
require_deployment_match=false
case "$WATCHDOG_MODE" in
  deployment-consistency)
    require_active_health=true
    require_candidate_artifact=true
    require_deployed_receipt=true
    require_deployment_match=true
    ;;
  active-health)
    require_active_health=true
    ;;
  candidate-artifact)
    require_candidate_artifact=true
    ;;
  deployed-receipt)
    require_deployed_receipt=true
    ;;
  *)
    echo "unsupported Hepta watchdog mode: $WATCHDOG_MODE" >&2
    usage
    ;;
esac

release_evidence_bundle="$(
  hepta_watchdog_release_evidence_bundle \
    "$REPO_ROOT" \
    "$require_candidate_artifact" \
    "$RELEASE_BIN" \
    "$CANDIDATE_MANIFEST" \
    "$require_deployed_receipt" \
    "$INSTALLED_BIN" \
    "$INSTALLED_RECEIPT" \
    "$EXPECTED_SOURCE_COMMIT" \
    "$require_deployment_match"
)"
release_sha="$(jq -r '.release_sha256' <<<"$release_evidence_bundle")"
installed_sha="$(jq -r '.installed_sha256' <<<"$release_evidence_bundle")"
binary_sha_match="$(jq -r '.binary_sha_match' <<<"$release_evidence_bundle")"
candidate_evidence="$(jq -c '.candidate' <<<"$release_evidence_bundle")"
deployed_evidence="$(jq -c '.deployed' <<<"$release_evidence_bundle")"
release_evidence_failure_reasons="$(jq -c '.failure_reasons' <<<"$release_evidence_bundle")"
release_evidence_ready="$(jq -r '.ready' <<<"$release_evidence_bundle")"

if [[ "$require_active_health" != "true" || "$release_evidence_ready" != "true" ]]; then
  evidence_report="$(
    hepta_watchdog_release_evidence_report \
      "$BASE_URL" \
      "$WATCHDOG_MODE" \
      "$require_active_health" \
      "$require_candidate_artifact" \
      "$require_deployed_receipt" \
      "$require_deployment_match" \
      "$release_evidence_bundle"
  )"
  printf '%s\n' "$evidence_report"
  if [[ "$(jq -r '.status' <<<"$evidence_report")" != "ok" ]]; then
    exit 1
  fi
  echo "Hepta watchdog passed"
  exit 0
fi

health_json="$(curl -fsS "$BASE_URL/health")"
route_manifest_bin=""
route_manifest_json=""
for candidate_bin in "$RELEASE_BIN" "$INSTALLED_BIN"; do
  [[ -x "$candidate_bin" ]] || continue
  candidate_manifest_json="$("$candidate_bin" manifest 2>/dev/null || true)"
  if jq -e '
    .route_effect_gate_manifest.schema_version == "hepta_route_effect_gate_manifest_v1"
    and (.route_effect_gate_manifest.entries | type == "array")
  ' >/dev/null 2>&1 <<<"$candidate_manifest_json"; then
    route_manifest_bin="$candidate_bin"
    route_manifest_json="$candidate_manifest_json"
    break
  fi
done
if [[ -z "$route_manifest_bin" ]]; then
  echo "Hepta watchdog could not load the generated route/effect/gate manifest" >&2
  exit 1
fi
mapfile -t generated_watchdog_probe_paths < <(
  jq -r '
    .route_effect_gate_manifest.entries[]
    | select(.watchdog_probe == true and .method == "GET")
    | .path_pattern
  ' <<<"$route_manifest_json"
)
generated_watchdog_probe_failures='[]'
for probe_path in "${generated_watchdog_probe_paths[@]}"; do
  if ! curl -fsS "$BASE_URL$probe_path" >/dev/null; then
    generated_watchdog_probe_failures="$(
      jq -c --arg path "$probe_path" '. + [$path]' <<<"$generated_watchdog_probe_failures"
    )"
  fi
done
route_manifest_schema="$(
  jq -r '.route_effect_gate_manifest.schema_version' <<<"$route_manifest_json"
)"
route_manifest_sha256="$(
  jq -r '.route_effect_gate_manifest.sha256' <<<"$route_manifest_json"
)"
route_json="$(curl -fsS "$BASE_URL/api/control-ui-route-parity")"
operator_json="$(curl -fsS "$BASE_URL/api/operator-security")"
owner_json="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff")"
poll_json="$(curl -fsS "$BASE_URL/api/telegram-poll-loop")"
post_json="$(curl -fsS "$BASE_URL/api/native-post-activation-plan")"
stores_json="$(curl -fsS "$BASE_URL/api/native-post-execution-stores")"
adapter_json="$(curl -fsS "$BASE_URL/api/hepta-engine-adapter-boundary")"
adapter_alias_json="$(curl -fsS "$BASE_URL/api/hepta-codex-engine-adapter-boundary")"
core_json="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"
closure_json="$(curl -fsS "$BASE_URL/api/hepta-name-repository-closure")"
dependency_json="$(curl -fsS "$BASE_URL/api/hepta-engine-dependency-closure")"
product_boundary_json="$(jq -c . "$PRODUCT_BOUNDARY")"
native_post_contract_json="$(
  hepta_watchdog_native_post_contract_json "$product_boundary_json" "$post_json"
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg installed_bin "$INSTALLED_BIN" \
  --arg watchdog_mode "$WATCHDOG_MODE" \
  --arg route_manifest_bin "$route_manifest_bin" \
  --arg route_manifest_schema "$route_manifest_schema" \
  --arg route_manifest_sha256 "$route_manifest_sha256" \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --argjson release_evidence_ready "$release_evidence_ready" \
  --argjson candidate_required "$require_candidate_artifact" \
  --argjson deployed_required "$require_deployed_receipt" \
  --argjson deployment_match_required "$require_deployment_match" \
  --argjson binary_sha_match "$binary_sha_match" \
  --argjson candidate_evidence "$candidate_evidence" \
  --argjson deployed_evidence "$deployed_evidence" \
  --argjson release_evidence_failure_reasons "$release_evidence_failure_reasons" \
  --argjson health "$health_json" \
  --argjson generated_watchdog_probe_count "${#generated_watchdog_probe_paths[@]}" \
  --argjson generated_watchdog_probe_failures "$generated_watchdog_probe_failures" \
  --argjson route "$route_json" \
  --argjson operator "$operator_json" \
  --argjson owner "$owner_json" \
  --argjson poll "$poll_json" \
  --argjson post "$post_json" \
  --argjson native_post_contract "$native_post_contract_json" \
  --argjson stores "$stores_json" \
  --argjson adapter "$adapter_json" \
  --argjson adapter_alias "$adapter_alias_json" \
  --argjson core "$core_json" \
  --argjson closure "$closure_json" \
  --argjson dependency "$dependency_json" \
  '
    ($operator.telegram_production_readiness_status // {}) as $production
    | (
        $operator.status == "attention"
        and $operator.legacy_owner_coexistence_ready == true
        and $operator.attention_reason == "telegram_replacement_not_requested"
        and $owner.active_owner == "legacy_openclaw"
        and $owner.double_poller_risk == false
        and $owner.hepta_poll_loop_armed == false
        and $poll.status == "gated"
        and $post.activation_currently_enabled == false
      ) as $legacy_owner_attention_state_known
    | (
        $operator.status == "ready"
        and $operator.security_mode == "active_replacement_ready"
        and $owner.active_owner == "parallel_bots"
        and $owner.hepta_parallel_bot_ready == true
        and $owner.hepta_poll_loop_armed == true
        and $poll.status == "armed"
        and $post.activation_currently_enabled == true
        and $post.single_handler_scope_ready == true
      ) as $active_replacement_state_known
    | (
        $operator.status == "attention"
        and $operator.attention_reason == "security_gate_not_ready"
        and $production.status == "attention"
        and $production.attention_budget_ok == false
        and ($production.readiness_blockers | length) == 1
        and ($production.readiness_blockers[0] == "attention_budget_exceeded")
        and $production.recent_bot_api_ok == true
        and $production.observation_ready == true
        and $production.observation_fresh == true
        and $production.poll_loop_armed == true
        and $production.cursor_ready == true
        and $production.delivery_ledger_ready == true
      ) as $attention_budget_exceeded_state_known
    | (
        $legacy_owner_attention_state_known
        and $production.status == "gated"
        and $production.attention_budget_ok == true
        and $production.recent_bot_api_ok == true
        and $production.observation_ready == false
        and $production.observation_fresh == false
        and $production.poll_loop_armed == false
        and $production.cursor_ready == true
        and $production.delivery_ledger_ready == true
        and ($production.readiness_blockers | index("poll_loop_not_armed")) != null
        and ($production.readiness_blockers | index("production_guards_not_ready")) != null
        and ($production.readiness_blockers | index("observation_min_poll_iterations")) != null
        and ($production.readiness_blockers | index("observation_stale")) != null
      ) as $warming_observation_budget_state_known
    | {
    product:$product,
    runtime:$runtime,
    base_url:$base_url,
    watchdog_mode:$watchdog_mode,
    status: (
      if $release_evidence_ready == true
        and $health.status == "ready"
        and $route_manifest_schema == "hepta_route_effect_gate_manifest_v1"
        and ($route_manifest_sha256 | length) == 64
        and $generated_watchdog_probe_count >= 12
        and ($generated_watchdog_probe_failures | length) == 0
        and $route.status == "ready"
        and $route.missing_route_count == 0
        and $owner.double_poller_risk == false
        and $poll.external_network_read_by_status == false
        and $poll.external_send_by_status == false
        and $native_post_contract.ready == true
        and $post.real_mutation_performed == false
        and $post.external_side_effects == false
        and $stores.status == "ready"
        and $stores.store_jsonl_valid == true
        and $stores.store_capacity_ok == true
        and $adapter.status == "ready"
        and $adapter.source_command == "/hepta-engine-adapter-boundary --json"
        and $adapter.canonical_endpoint == "/api/hepta-engine-adapter-boundary"
        and $adapter.canonical_source_command == "/hepta-engine-adapter-boundary --json"
        and $adapter.transition_alias_endpoint == "/api/hepta-codex-engine-adapter-boundary"
        and $adapter.transition_alias_source_command == "/hepta-codex-engine-adapter-boundary --json"
        and $adapter.hepta_named_route_alias_ready == true
        and $adapter.transition_alias_retained == true
        and $adapter_alias.status == "ready"
        and $adapter_alias.canonical_endpoint == $adapter.canonical_endpoint
        and $adapter_alias.transition_alias_endpoint == $adapter.transition_alias_endpoint
        and $adapter_alias.source_command == $adapter.source_command
        and $adapter_alias.boundary_ready == true
        and $adapter.boundary_ready == true
        and ($adapter.surfaces | length) >= 6
        and ($adapter.surfaces | all(.typed_request_response_envelope_ready == true))
        and ($adapter.surfaces | all(.typed_adapter_parity_gate_ready == true))
        and ($adapter.surfaces | all(.live_mutation_allowed == false))
        and ($adapter.parity_evidence | length) >= 6
        and ($adapter.parity_evidence | all(.evidence_ready == true))
        and ($adapter.parity_evidence | all(.compatibility_dispatch_checked == true))
        and ($adapter.parity_evidence | all(.behavior_equivalence_checked == true))
        and ($adapter.parity_evidence | all(.observable_behavior_preserved == true))
        and ($adapter.parity_evidence | all(.live_mutation_blocked == true))
        and ($adapter.parity_evidence | all(.forbidden_side_effects_blocked == true))
        and $adapter.adapter_parity_complete == true
        and $adapter.adapter_parity_promotion_ready == true
        and $adapter.adapter_parity_completion_gate == "adapter_behavior_equivalence_to_parity_completion_gate"
        and $adapter.adapter_parity_completion_gate_ready == true
        and $adapter.adapter_parity_completion_gate_status == "ready_adapter_parity_promoted_active_hepta_service_dependency_closure_complete"
        and $adapter.adapter_parity_completion_gate_allows_promotion == true
        and $adapter.adapter_shadow_replay_required_surface_count == ($adapter.surfaces | length)
        and $adapter.adapter_shadow_replay_covered_surface_count == ($adapter.surfaces | length)
        and $adapter.adapter_shadow_replay_remaining_surface_count == (($adapter.surfaces | length) - $adapter.adapter_shadow_replay_covered_surface_count)
        and (
          $adapter.parity_evidence
          | map(select(
              .shadow_replay_checked == true
              and .shadow_replay_observable_match == true
              and .shadow_replay_side_effect_free == true
            ))
          | length
        ) == ($adapter.surfaces | length)
        and ($adapter.adapter_parity_promotion_criteria | length) >= 6
        and ($adapter.adapter_parity_promotion_blockers | length) == 0
        and $adapter.full_fusion_complete == true
        and $adapter.forbidden_real_side_effects.public_ga_claimed == false
        and $adapter.forbidden_real_side_effects.public_release_published == false
        and $adapter.forbidden_real_side_effects.native_post_real_mutation_performed == false
        and $adapter.forbidden_real_side_effects.task_publish_real_mutation_performed == false
        and $adapter.forbidden_real_side_effects.credential_read == false
        and $adapter.forbidden_real_side_effects.model_invoked == false
        and $adapter.forbidden_real_side_effects.external_network_read == false
        and $core.status == "ready"
        and $core.phase == "phase_5_engine_dependency_closure"
        and $core.phase_2_engine_adapter_boundary_ready == true
        and $core.phase_3_binary_package_inversion_ready == true
        and $core.binary_package_inversion_gate == "hepta_first_class_binary_package_inversion_gate"
        and $core.binary_package_inversion_gate_ready == true
        and $core.binary_package_inversion_gate_status == "ready_hepta_cli_release_package_ownership_active"
        and ($core.binary_package_inversion_criteria | length) >= 6
        and ($core.binary_package_inversion_blockers | length) == 0
        and $core.active_binary_package == "hepta-cli"
        and $core.active_binary_target == "hepta"
        and $core.intended_binary_package == "hepta-cli"
        and $core.intended_binary_target == "hepta"
        and $core.installed_service_binary == $installed_bin
        and $core.full_fusion_complete == true
        and $core.phase_4_name_repository_closure_gate == "hepta_name_repository_closure_gate"
        and $core.phase_4_name_repository_closure_gate_ready == true
        and $core.phase_4_name_repository_closure_gate_status == "ready_phase_4_transition_names_closed"
        and $core.phase_4_name_repository_closure_remaining_surface_count == 0
        and ($core.phase_4_name_repository_closure_blockers | length) == 0
        and $core.phase_4_name_repository_closure_ready == true
        and $core.phase_5_engine_dependency_closure_gate == "hepta_engine_dependency_closure_gate"
        and $core.phase_5_engine_dependency_closure_gate_ready == true
        and $core.phase_5_engine_dependency_closure_gate_status == "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
        and $core.phase_5_engine_dependency_closure_remaining_dependency_count == $dependency.remaining_direct_dependency_count
        and $core.phase_5_engine_dependency_closure_remaining_dependency_count == 0
        and ($core.phase_5_engine_dependency_closure_blockers | length) == 0
        and $closure.status == "ready"
        and $closure.phase == "phase_4_name_repository_closure"
        and $closure.closure_gate == "hepta_name_repository_closure_gate"
        and $closure.closure_gate_ready == true
        and $closure.closure_gate_status == "ready_phase_4_transition_names_closed"
        and $closure.phase_4_name_repository_closure_ready == true
        and $closure.full_fusion_complete == true
        and $closure.transition_surface_count >= 6
        and $closure.closed_transition_surface_count == $closure.transition_surface_count
        and $closure.remaining_transition_surface_count == 0
        and ($closure.blockers | length) == 0
        and ($closure.surfaces | map(select(.blocks_full_fusion == true)) | length) == $closure.remaining_transition_surface_count
        and ($closure.surfaces | map(select(.surface_id == "active_release_binary_package" and .closure_state == "closed" and .blocks_full_fusion == false)) | length) == 1
        and ($closure.surfaces | map(select(.surface_id == "runtime_report_strings" and .current_name == "hepta" and .target_name == "hepta" and .closure_state == "closed" and .blocks_full_fusion == false)) | length) == 1
        and ($closure.surfaces | map(select(.surface_id == "engine_adapter_boundary_route" and .closure_state == "alias_active" and .blocks_full_fusion == false)) | length) == 1
        and ($closure.surfaces | map(select(.surface_id == "release_gate_script_family" and .closure_state == "alias_active" and .blocks_full_fusion == false)) | length) == 1
        and ($closure.surfaces | map(select(.surface_id == "core_fusion_route_document" and .closure_state == "alias_active" and .blocks_full_fusion == false)) | length) == 1
        and ($closure.surfaces | map(select(.surface_id == "workspace_repository_directory" and .current_name == "/Users/qianqi/.openclaw/workspace/Hepta" and .closure_state == "closed" and .blocks_full_fusion == false)) | length) == 1
        and $closure.forbidden_real_side_effects.public_ga_claimed == false
        and $closure.forbidden_real_side_effects.public_release_published == false
        and $closure.forbidden_real_side_effects.gateway_mutation_performed == false
        and $closure.forbidden_real_side_effects.credential_read == false
        and $closure.forbidden_real_side_effects.model_invoked == false
        and $dependency.status == "ready"
        and $dependency.phase == "phase_5_engine_dependency_closure"
        and $dependency.closure_gate == "hepta_engine_dependency_closure_gate"
        and $dependency.closure_gate_ready == true
        and $dependency.closure_gate_status == "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
        and $dependency.full_fusion_complete == true
        and $dependency.direct_dependency_count >= 10
        and $dependency.adapter_retained_dependency_count == 0
        and $dependency.remaining_direct_dependency_count == 0
        and $dependency.closed_direct_dependency_count == $dependency.direct_dependency_count
        and ($dependency.surfaces | length) == $dependency.direct_dependency_count
        and ($dependency.surfaces | all(.closure_state == "closed_active_hepta_service_binary_isolated"))
        and ($dependency.surfaces | all(.direct_dependency_retained == false))
        and ($dependency.surfaces | all(.compatibility_adapter_required == false))
        and ($dependency.surfaces | all(.typed_adapter_parity_ready == true))
        and ($dependency.surfaces | all(.blocks_full_fusion == false))
        and ($dependency.surfaces | map(select(.dependency_crate == "codex-core" and .adapter_surface_id == "tool_invocation" and .target_owner == "hepta-kernel")) | length) == 1
        and ($dependency.surfaces | map(select(.dependency_crate == "codex-tui" and .adapter_surface_id == "legacy_tui_cli" and .target_owner == "hepta-runtime")) | length) == 1
        and ($dependency.blockers | length) == 0
        and $dependency.forbidden_real_side_effects.public_release_published == false
        and $dependency.forbidden_real_side_effects.gateway_mutation_performed == false
        and $dependency.forbidden_real_side_effects.credential_read == false
        and $dependency.forbidden_real_side_effects.model_invoked == false
        and (
          (
            $operator.status == "attention"
            and $operator.legacy_owner_coexistence_ready == true
            and $operator.attention_reason == "telegram_replacement_not_requested"
            and $owner.active_owner == "legacy_openclaw"
            and $owner.hepta_poll_loop_armed == false
            and $poll.status == "gated"
            and $post.activation_currently_enabled == false
          )
          or (
            $operator.status == "ready"
            and $operator.security_mode == "active_replacement_ready"
            and $owner.active_owner == "parallel_bots"
            and $owner.hepta_parallel_bot_ready == true
            and $owner.hepta_poll_loop_armed == true
            and $poll.status == "armed"
            and $post.activation_currently_enabled == true
            and $post.single_handler_scope_ready == true
          )
          or (
            $operator.status == "attention"
            and $operator.attention_reason == "security_gate_not_ready"
            and $operator.telegram_production_readiness_status.status == "attention"
            and $operator.telegram_production_readiness_status.attention_budget_ok == false
            and ($operator.telegram_production_readiness_status.readiness_blockers | length) == 1
            and ($operator.telegram_production_readiness_status.readiness_blockers[0] == "attention_budget_exceeded")
            and $operator.telegram_production_readiness_status.recent_bot_api_ok == true
            and $operator.telegram_production_readiness_status.observation_ready == true
            and $operator.telegram_production_readiness_status.observation_fresh == true
            and $operator.telegram_production_readiness_status.poll_loop_armed == true
            and $operator.telegram_production_readiness_status.cursor_ready == true
            and $operator.telegram_production_readiness_status.delivery_ledger_ready == true
            and $owner.active_owner == "parallel_bots"
            and $owner.hepta_parallel_bot_ready == true
            and $owner.hepta_poll_loop_armed == true
            and $owner.double_poller_risk == false
            and $poll.status == "armed"
            and $post.activation_currently_enabled == true
            and $post.single_handler_scope_ready == true
          )
        )
      then "ok" else "failed" end
    ),
    release_sha256:$release_sha,
    installed_sha256:$installed_sha,
    binary_sha_match:$binary_sha_match,
    active_health:{required:true,status:(if $health.status == "ready" then "ready" else "failed" end)},
    route_effect_gate_manifest:{
      schema_version:$route_manifest_schema,
      sha256:$route_manifest_sha256,
      source_binary:$route_manifest_bin,
      generated_watchdog_probe_count:$generated_watchdog_probe_count,
      generated_watchdog_probe_failures:$generated_watchdog_probe_failures
    },
    candidate_artifact:{required:$candidate_required,evidence:$candidate_evidence},
    deployed_receipt:{required:$deployed_required,evidence:$deployed_evidence},
    deployment_consistency_required:$deployment_match_required,
    health:$health.status,
    route_count:$route.route_count,
    missing_route_count:$route.missing_route_count,
    operator_security_status:$operator.status,
    operator_security_attention_state_known:(
      $legacy_owner_attention_state_known
      or $active_replacement_state_known
      or $attention_budget_exceeded_state_known
    ),
    telegram_production_readiness_state_known:(
      $active_replacement_state_known
      or $attention_budget_exceeded_state_known
      or $warming_observation_budget_state_known
    ),
    telegram_production_readiness_classification:(
      if $active_replacement_state_known
      then "ready"
      elif $attention_budget_exceeded_state_known
      then "attention_budget_exceeded"
      elif $warming_observation_budget_state_known
      then "warming_observation_budget"
      else "unknown"
      end
    ),
    operator_security_attention_budget_known: (
      $attention_budget_exceeded_state_known
      or $warming_observation_budget_state_known
    ),
    telegram_production_attention_budget_ok:$production.attention_budget_ok,
    security_mode:$operator.security_mode,
    active_owner:$owner.active_owner,
    double_poller_risk:$owner.double_poller_risk,
    telegram_poll_loop_status:$poll.status,
    native_post_activation_enabled:$post.activation_currently_enabled,
    native_post_contract_ready:$native_post_contract.ready,
    native_post_contract_mode:$native_post_contract.mode,
    product_role:$native_post_contract.product_role,
    native_post_store_lines:$stores.total_line_count,
    adapter_boundary_status:$adapter.status,
    adapter_canonical_endpoint:$adapter.canonical_endpoint,
    adapter_transition_alias_endpoint:$adapter.transition_alias_endpoint,
    adapter_alias_status:$adapter_alias.status,
    adapter_surface_count:($adapter.surfaces | length),
    adapter_parity_evidence_count:($adapter.parity_evidence | length),
    adapter_parity_evidence_ready_count:($adapter.parity_evidence | map(select(.evidence_ready == true)) | length),
    adapter_behavior_equivalence_checked_count:($adapter.parity_evidence | map(select(.behavior_equivalence_checked == true)) | length),
    adapter_observable_behavior_preserved_count:($adapter.parity_evidence | map(select(.observable_behavior_preserved == true)) | length),
    adapter_typed_envelope_ready_count:($adapter.surfaces | map(select(.typed_request_response_envelope_ready == true)) | length),
    adapter_typed_parity_gate_ready_count:($adapter.surfaces | map(select(.typed_adapter_parity_gate_ready == true)) | length),
    adapter_parity_complete:$adapter.adapter_parity_complete,
    adapter_parity_promotion_ready:$adapter.adapter_parity_promotion_ready,
    adapter_parity_completion_gate_ready:$adapter.adapter_parity_completion_gate_ready,
    adapter_parity_completion_gate_status:$adapter.adapter_parity_completion_gate_status,
    adapter_parity_completion_gate_allows_promotion:$adapter.adapter_parity_completion_gate_allows_promotion,
    adapter_shadow_replay_required_surface_count:$adapter.adapter_shadow_replay_required_surface_count,
    adapter_shadow_replay_covered_surface_count:$adapter.adapter_shadow_replay_covered_surface_count,
    adapter_shadow_replay_remaining_surface_count:$adapter.adapter_shadow_replay_remaining_surface_count,
    adapter_shadow_replay_ready_count:(
      $adapter.parity_evidence
      | map(select(
          .shadow_replay_checked == true
          and .shadow_replay_observable_match == true
          and .shadow_replay_side_effect_free == true
        ))
      | length
    ),
    adapter_parity_promotion_blocker_count:($adapter.adapter_parity_promotion_blockers | length),
    full_fusion_complete:$adapter.full_fusion_complete,
    core_fusion_phase:$core.phase,
    phase_3_binary_package_inversion_ready:$core.phase_3_binary_package_inversion_ready,
    binary_package_inversion_gate_status:$core.binary_package_inversion_gate_status,
    binary_package_inversion_blocker_count:($core.binary_package_inversion_blockers | length),
    active_binary_package:$core.active_binary_package,
    intended_binary_package:$core.intended_binary_package,
    installed_service_binary:$core.installed_service_binary,
    phase_4_name_repository_closure_gate_status:$core.phase_4_name_repository_closure_gate_status,
    phase_4_name_repository_closure_ready:$core.phase_4_name_repository_closure_ready,
    phase_4_name_repository_closure_remaining_surface_count:$core.phase_4_name_repository_closure_remaining_surface_count,
    name_repository_closure_status:$closure.status,
    name_repository_closure_gate_status:$closure.closure_gate_status,
    name_repository_closure_remaining_surface_count:$closure.remaining_transition_surface_count,
    name_repository_closure_blocker_count:($closure.blockers | length),
    phase_5_engine_dependency_closure_gate_status:$core.phase_5_engine_dependency_closure_gate_status,
    phase_5_engine_dependency_closure_ready:$core.phase_5_engine_dependency_closure_gate_ready,
    phase_5_engine_dependency_closure_remaining_dependency_count:$core.phase_5_engine_dependency_closure_remaining_dependency_count,
    engine_dependency_closure_status:$dependency.status,
    engine_dependency_closure_gate_status:$dependency.closure_gate_status,
    engine_dependency_closure_remaining_dependency_count:$dependency.remaining_direct_dependency_count,
    engine_dependency_closure_blocker_count:($dependency.blockers | length),
    side_effects:{
      telegram_read_by_status:$poll.external_network_read_by_status,
      telegram_send_by_status:$poll.external_send_by_status,
      native_post_real_mutation:$post.real_mutation_performed,
      native_post_external_side_effects:$post.external_side_effects,
      adapter_public_ga_claimed:$adapter.forbidden_real_side_effects.public_ga_claimed,
      adapter_public_release_published:$adapter.forbidden_real_side_effects.public_release_published,
      adapter_native_post_real_mutation:$adapter.forbidden_real_side_effects.native_post_real_mutation_performed,
      adapter_task_publish_real_mutation:$adapter.forbidden_real_side_effects.task_publish_real_mutation_performed,
      adapter_credential_read:$adapter.forbidden_real_side_effects.credential_read,
      adapter_model_invoked:$adapter.forbidden_real_side_effects.model_invoked,
      adapter_external_network_read:$adapter.forbidden_real_side_effects.external_network_read,
      closure_public_ga_claimed:$closure.forbidden_real_side_effects.public_ga_claimed,
      closure_public_release_published:$closure.forbidden_real_side_effects.public_release_published,
      closure_gateway_mutation:$closure.forbidden_real_side_effects.gateway_mutation_performed,
      closure_credential_read:$closure.forbidden_real_side_effects.credential_read,
      closure_model_invoked:$closure.forbidden_real_side_effects.model_invoked,
      dependency_public_release_published:$dependency.forbidden_real_side_effects.public_release_published,
      dependency_gateway_mutation:$dependency.forbidden_real_side_effects.gateway_mutation_performed,
      dependency_credential_read:$dependency.forbidden_real_side_effects.credential_read,
      dependency_model_invoked:$dependency.forbidden_real_side_effects.model_invoked
    }
  } as $report
  | $report + {
      failure_reasons:(
        $release_evidence_failure_reasons
        + (if $report.status == "ok" then [] else ["active_health_contract_failed"] end)
      )
    }')"

printf '%s\n' "$report"

if [[ "$(printf '%s' "$report" | jq -r '.status')" != "ok" ]]; then
  exit 1
fi

echo "Hepta watchdog passed"
