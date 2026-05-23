#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"

PACKET_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-operator-approval-packet")"
GA_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-readiness")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "ready"
  and .compatibility_mode == "native_public_ga_operator_approval_packet"
  and .side_effect_free == true
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 65
  and .missing_route_count == 0
  and .approval_packet_ready == true
  and .safe_default_mode == "plan_only_no_live_mutation"
  and .irreversible_actions_blocked_by_default == true
  and (.public_ga_ready == false or .public_ga_ready == true)
  and (.public_ga_blocker_count == 8 or .public_ga_blocker_count == 0)
  and .required_operator_approval_count == 8
  and (
    .public_ga_ready == true
    or (
      (.blockers | index("gateway_replacement_not_ready")) != null
      and (.blockers | index("telegram_owner_handoff_not_operator_approved")) != null
      and (.blockers | index("telegram_live_poll_model_send_soak_not_complete")) != null
      and (.blockers | index("native_post_real_activation_not_operator_approved")) != null
      and (.blockers | index("credentialed_provider_live_smoke_not_operator_approved")) != null
      and (.blockers | index("channel_live_delivery_not_operator_approved")) != null
      and (.blockers | index("release_artifact_pack_not_operator_approved")) != null
      and (.blockers | index("external_public_release_not_operator_approved")) != null
    )
  )
  and .side_effects.public_release_published == false
  and .side_effects.release_artifact_written == false
  and .side_effects.launchd_mutated == false
  and .side_effects.credential_read == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
' <<<"$PACKET_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson packet "$PACKET_JSON" \
  --argjson ga "$GA_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-public-ga-operator-approval-packet",
    approval_packet_ready:$packet.approval_packet_ready,
    safe_default_mode:$packet.safe_default_mode,
    current_hepta_codex_script_total:$packet.current_hepta_codex_script_total,
    native_gateway_source_command_count:$packet.native_gateway_source_command_count,
    route_count:$packet.route_count,
    missing_route_count:$packet.missing_route_count,
    public_ga_ready:$packet.public_ga_ready,
    public_ga_blocker_count:$packet.public_ga_blocker_count,
    required_operator_approval_count:$packet.required_operator_approval_count,
    reports_synchronized: (
      $packet.current_hepta_codex_script_total == $ga.current_hepta_codex_script_total
      and $packet.native_gateway_source_command_count == $ga.native_gateway_source_command_count
      and $packet.missing_route_count == $ga.missing_route_count
      and $packet.public_ga_ready == $ga.public_ga_ready
      and $packet.public_ga_blocker_count == $ga.blocker_count
    ),
    side_effects:$packet.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "operator approval packet and public GA report are out of sync" >&2
  exit 1
fi

echo "Hepta Codex public GA operator approval packet passed"
