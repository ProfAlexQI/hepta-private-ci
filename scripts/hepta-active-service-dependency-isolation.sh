#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
RUN_LIVE="${HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE:-0}"
REQUIRE_LIVE="${HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_REQUIRE_LIVE:-0}"

FORBIDDEN_CODEX_ENGINE_CRATES=(
  codex-cli
  codex-core
  codex-exec
  codex-state
  codex-mcp
  codex-app-server
  codex-sandboxing
  codex-plugin
  codex-model-provider
  codex-protocol
  codex-tui
)

tree_output="$(cargo tree --offline --manifest-path "$MANIFEST" -p hepta-cli --edges normal --prefix none)"
package_names="$(awk '{print $1}' <<<"$tree_output")"
metadata="$(cargo metadata --offline --manifest-path "$MANIFEST" --no-deps --format-version 1)"
hepta_binary_owners="$(
  jq -c '[
    .packages[]
    | select(any(.targets[]; (.kind | index("bin")) != null and .name == "hepta"))
    | .name
  ] | sort' <<<"$metadata"
)"
compatibility_binary_present="$(
  jq -r 'any(
    .packages[];
    .name == "codex-cli"
    and any(.targets[]; (.kind | index("bin")) != null and .name == "hepta-codex-compat")
  )' <<<"$metadata"
)"
official_entrypoint_ready=false
if [[ "$hepta_binary_owners" == '["hepta-cli"]' && "$compatibility_binary_present" == "true" ]]; then
  official_entrypoint_ready=true
fi

found_forbidden=()
for crate in "${FORBIDDEN_CODEX_ENGINE_CRATES[@]}"; do
  if grep -Fxq "$crate" <<<"$package_names"; then
    found_forbidden+=("$crate")
  fi
done

live_dependency_json="null"
live_core_json="null"
live_status="skipped"
live_ready=false
if [[ "$RUN_LIVE" == "1" || "$REQUIRE_LIVE" == "1" ]]; then
  if live_dependency_json="$(curl -fsS "$BASE_URL/api/hepta-engine-dependency-closure")" \
    && live_core_json="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"; then
    if jq -e '
      .status == "ready"
      and .phase == "phase_5_engine_dependency_closure"
      and .closure_gate_ready == true
      and .closure_gate_status == "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
      and .full_fusion_complete == true
      and .remaining_direct_dependency_count == 0
      and .adapter_retained_dependency_count == 0
      and (.blockers | length) == 0
      and (.surfaces | all(.direct_dependency_retained == false))
      and (.surfaces | all(.blocks_full_fusion == false))
    ' <<<"$live_dependency_json" >/dev/null \
      && jq -e '
        .status == "ready"
        and .active_binary_package == "hepta-cli"
        and .active_binary_target == "hepta"
        and .phase_5_engine_dependency_closure_gate_ready == true
        and .phase_5_engine_dependency_closure_remaining_dependency_count == 0
        and .full_fusion_complete == true
      ' <<<"$live_core_json" >/dev/null; then
      live_status="ready"
      live_ready=true
    else
      live_status="failed"
      live_ready=false
    fi
  else
    live_dependency_json="null"
    live_core_json="null"
    live_status="unavailable"
    live_ready=false
  fi
fi

forbidden_json="$(
  printf '%s\n' "${FORBIDDEN_CODEX_ENGINE_CRATES[@]}" | jq -R . | jq -s .
)"
found_json="$(
  if ((${#found_forbidden[@]} == 0)); then
    printf '[]\n'
  else
    printf '%s\n' "${found_forbidden[@]}" | jq -R . | jq -s .
  fi
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg manifest "$MANIFEST" \
    --arg package "hepta-cli" \
    --arg binary "hepta" \
    --arg base_url "$BASE_URL" \
    --arg live_status "$live_status" \
    --argjson forbidden "$forbidden_json" \
    --argjson found "$found_json" \
    --argjson hepta_binary_owners "$hepta_binary_owners" \
    --argjson compatibility_binary_present "$compatibility_binary_present" \
    --argjson official_entrypoint_ready "$official_entrypoint_ready" \
    --argjson live_ready "$live_ready" \
    --argjson live_dependency "$live_dependency_json" \
    --argjson live_core "$live_core_json" \
    '{
      product:$product,
      runtime:$runtime,
      status:(if ($found | length) == 0 and $official_entrypoint_ready and ($live_status != "failed") then "ready" else "blocked" end),
      gate:"hepta_active_service_dependency_isolation_gate",
      manifest:$manifest,
      active_binary_package:$package,
      active_binary_target:$binary,
      official_entrypoint_ready:$official_entrypoint_ready,
      official_binary_owners:$hepta_binary_owners,
      compatibility_binary_target:"hepta-codex-compat",
      compatibility_binary_present:$compatibility_binary_present,
      tracked_forbidden_codex_engine_crates:$forbidden,
      found_forbidden_codex_engine_crates:$found,
      local_cargo_tree_isolated:(($found | length) == 0),
      live_check_status:$live_status,
      live_check_ready:$live_ready,
      live_engine_dependency_closure:(
        if $live_dependency == null then null else {
          status:$live_dependency.status,
          closure_gate_ready:$live_dependency.closure_gate_ready,
          closure_gate_status:$live_dependency.closure_gate_status,
          full_fusion_complete:$live_dependency.full_fusion_complete,
          remaining_direct_dependency_count:$live_dependency.remaining_direct_dependency_count,
          adapter_retained_dependency_count:$live_dependency.adapter_retained_dependency_count,
          blocker_count:($live_dependency.blockers | length)
        } end
      ),
      live_core_fusion_readiness:(
        if $live_core == null then null else {
          status:$live_core.status,
          active_binary_package:$live_core.active_binary_package,
          active_binary_target:$live_core.active_binary_target,
          phase_5_engine_dependency_closure_gate_ready:$live_core.phase_5_engine_dependency_closure_gate_ready,
          phase_5_engine_dependency_closure_remaining_dependency_count:$live_core.phase_5_engine_dependency_closure_remaining_dependency_count,
          full_fusion_complete:$live_core.full_fusion_complete
        } end
      ),
      side_effects:{
        filesystem_written:false,
        gateway_mutation_performed:false,
        public_release_published:false,
        credential_read:false,
        model_invoked:false,
        external_send_performed:false
      }
    }'
)"

printf '%s\n' "$report"

if ((${#found_forbidden[@]} > 0)); then
  echo "active hepta-cli package still depends on forbidden Codex engine crates: ${found_forbidden[*]}" >&2
  exit 1
fi

if [[ "$official_entrypoint_ready" != "true" ]]; then
  echo "official hepta binary ownership is ambiguous or compatibility target is missing" >&2
  exit 1
fi

if [[ "$REQUIRE_LIVE" == "1" && "$live_ready" != "true" ]]; then
  echo "live Hepta dependency-closure route is not ready (status: $live_status)" >&2
  exit 1
fi

if [[ "$live_status" == "failed" ]]; then
  echo "live Hepta dependency-closure route returned a failing contract" >&2
  exit 1
fi

echo "Hepta active service dependency isolation gate passed"
