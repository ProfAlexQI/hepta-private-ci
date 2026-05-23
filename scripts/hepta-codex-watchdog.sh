#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RELEASE_BIN="${HEPTA_CODEX_RELEASE_BIN:-$REPO_ROOT/codex-rs/target/release/hepta}"
INSTALLED_BIN="${HEPTA_CODEX_INSTALLED_BIN:-$HOME/.local/opt/hepta/bin/hepta}"

release_sha=""
installed_sha=""
if [[ -f "$RELEASE_BIN" ]]; then
  release_sha="$(shasum -a 256 "$RELEASE_BIN" | awk '{print $1}')"
fi
if [[ -f "$INSTALLED_BIN" ]]; then
  installed_sha="$(shasum -a 256 "$INSTALLED_BIN" | awk '{print $1}')"
fi

health_json="$(curl -fsS "$BASE_URL/health")"
route_json="$(curl -fsS "$BASE_URL/api/control-ui-route-parity")"
operator_json="$(curl -fsS "$BASE_URL/api/operator-security")"
owner_json="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff")"
poll_json="$(curl -fsS "$BASE_URL/api/telegram-poll-loop")"
post_json="$(curl -fsS "$BASE_URL/api/native-post-activation-plan")"
stores_json="$(curl -fsS "$BASE_URL/api/native-post-execution-stores")"
adapter_json="$(curl -fsS "$BASE_URL/api/hepta-codex-engine-adapter-boundary")"
core_json="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --arg installed_bin "$INSTALLED_BIN" \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --argjson health "$health_json" \
  --argjson route "$route_json" \
  --argjson operator "$operator_json" \
  --argjson owner "$owner_json" \
  --argjson poll "$poll_json" \
  --argjson post "$post_json" \
  --argjson stores "$stores_json" \
  --argjson adapter "$adapter_json" \
  --argjson core "$core_json" \
  '{
    product:$product,
    runtime:$runtime,
    base_url:$base_url,
    status: (
      if $health.status == "ready"
        and $route.status == "ready"
        and $route.missing_route_count == 0
        and $owner.double_poller_risk == false
        and $poll.external_network_read_by_status == false
        and $poll.external_send_by_status == false
        and $post.status == "ready"
        and $post.real_mutation_performed == false
        and $post.external_side_effects == false
        and $stores.status == "ready"
        and $stores.store_jsonl_valid == true
        and $stores.store_capacity_ok == true
        and $adapter.status == "ready"
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
        and $adapter.adapter_parity_completion_gate_status == "ready_adapter_parity_promoted_full_fusion_pending_binary_package_inversion"
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
        and $adapter.full_fusion_complete == false
        and $adapter.forbidden_real_side_effects.public_ga_claimed == false
        and $adapter.forbidden_real_side_effects.public_release_published == false
        and $adapter.forbidden_real_side_effects.native_post_real_mutation_performed == false
        and $adapter.forbidden_real_side_effects.task_publish_real_mutation_performed == false
        and $adapter.forbidden_real_side_effects.credential_read == false
        and $adapter.forbidden_real_side_effects.model_invoked == false
        and $adapter.forbidden_real_side_effects.external_network_read == false
        and $core.status == "ready"
        and $core.phase == "phase_3_binary_package_inversion"
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
        and $core.full_fusion_complete == false
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
        )
      then "ok" else "failed" end
    ),
    release_sha256:$release_sha,
    installed_sha256:$installed_sha,
    binary_sha_match: ($release_sha != "" and $release_sha == $installed_sha),
    health:$health.status,
    route_count:$route.route_count,
    missing_route_count:$route.missing_route_count,
    operator_security_status:$operator.status,
    security_mode:$operator.security_mode,
    active_owner:$owner.active_owner,
    double_poller_risk:$owner.double_poller_risk,
    telegram_poll_loop_status:$poll.status,
    native_post_activation_enabled:$post.activation_currently_enabled,
    native_post_store_lines:$stores.total_line_count,
    adapter_boundary_status:$adapter.status,
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
      adapter_external_network_read:$adapter.forbidden_real_side_effects.external_network_read
    }
  }')"

printf '%s\n' "$report"

if [[ "$(printf '%s' "$report" | jq -r '.status')" != "ok" ]]; then
  exit 1
fi

echo "Hepta Codex watchdog passed"
