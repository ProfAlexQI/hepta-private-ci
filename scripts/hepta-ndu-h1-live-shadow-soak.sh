#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

samples="${HEPTA_NDU_H1_SOAK_SAMPLES:-24}"
case "$samples" in
  ''|*[!0-9]*) echo "HEPTA_NDU_H1_SOAK_SAMPLES must be a positive integer" >&2; exit 2 ;;
esac
if (( samples < 1 )); then
  echo "HEPTA_NDU_H1_SOAK_SAMPLES must be greater than zero" >&2
  exit 2
fi

artifact_dir="${HEPTA_NDU_H1_SOAK_ARTIFACT_DIR:-}"
if [[ -z "$artifact_dir" ]]; then
  artifact_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ndu-h1-live-shadow-soak.XXXXXX")"
else
  mkdir -p "$artifact_dir"
  artifact_dir="$(cd "$artifact_dir" && pwd)"
fi
fixture_dir="$artifact_dir/runtime"
mkdir -p "$fixture_dir"
chmod 700 "$artifact_dir" "$fixture_dir"

runtime_database="$fixture_dir/outcomes.sqlite3"
runtime_state_database="$fixture_dir/runtime-state.json"
runtime_key_file="$fixture_dir/integrity.key"
preference_database="$fixture_dir/preferences.sqlite3"
preference_integrity_key_file="$fixture_dir/preference-integrity.key"
preference_auth_key_file="$fixture_dir/preference-auth.key"
anchor_file="$fixture_dir/monotonic.anchor"
anchor_key_file="$fixture_dir/anchor.key"
journal_file="$artifact_dir/ndu-h1-shadow.jsonl"
kill_switch_file="$fixture_dir/ndu-h1.kill"
server_log="$artifact_dir/server.log"
receipt_file="$artifact_dir/receipt.json"

write_key() {
  local file="$1"
  local value="$2"
  (umask 077; printf '%s' "$value" >"$file")
  chmod 600 "$file"
}
write_key "$runtime_key_file" '000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f'
write_key "$preference_integrity_key_file" '202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f'
write_key "$preference_auth_key_file" '404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f'
write_key "$anchor_key_file" '606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f'

release_bin="${HEPTA_RELEASE_BIN:-}"
if [[ -z "$release_bin" ]]; then
  target_dir="${CARGO_TARGET_DIR:-/Users/qianqi/.openclaw/tmp/cargo-targets/hepta-main-backend}"
  rust_toolchain="${HEPTA_RUST_TOOLCHAIN:-1.95.0}"
  rustc_bin="$(rustup which --toolchain "$rust_toolchain" rustc)"
  RUSTC="$rustc_bin" cargo build --manifest-path codex-rs/Cargo.toml \
    -p hepta-cli --bin hepta --target-dir "$target_dir"
  release_bin="$target_dir/debug/hepta"
fi
if [[ ! -x "$release_bin" ]]; then
  echo "Hepta binary is not executable: $release_bin" >&2
  exit 1
fi
release_bin="$(cd "$(dirname "$release_bin")" && pwd)/$(basename "$release_bin")"

bind_addr="${HEPTA_NDU_H1_SOAK_ADDR:-}"
if [[ -z "$bind_addr" ]]; then
  for port in 7391 7392 7393 7394 7395; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      bind_addr="127.0.0.1:$port"
      break
    fi
  done
fi
if [[ -z "$bind_addr" ]]; then
  echo "no free loopback port found for NDU H1 soak" >&2
  exit 1
fi
base_url="http://$bind_addr"
server_pid=''

stop_server() {
  if [[ -n "$server_pid" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    for _ in {1..40}; do
      if ! kill -0 "$server_pid" 2>/dev/null; then
        wait "$server_pid" 2>/dev/null || true
        server_pid=''
        return
      fi
      sleep 0.1
    done
    kill -9 "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  server_pid=''
}
trap stop_server EXIT

start_server() {
  local outcome_mode='bootstrap-new'
  local preference_mode='bootstrap-new'
  [[ -e "$runtime_database" ]] && outcome_mode='open-existing'
  [[ -e "$preference_database" ]] && preference_mode='open-existing'
  : >"$server_log"
  HEPTA_RUNTIME_OUTCOME_DATABASE="$runtime_database" \
  HEPTA_RUNTIME_STATE_DATABASE="$runtime_state_database" \
  HEPTA_RUNTIME_INTEGRITY_KEY_FILE="$runtime_key_file" \
  HEPTA_RUNTIME_OUTCOME_MODE="$outcome_mode" \
  HEPTA_PREFERENCE_DATABASE="$preference_database" \
  HEPTA_PREFERENCE_INTEGRITY_KEY_FILE="$preference_integrity_key_file" \
  HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE="$preference_auth_key_file" \
  HEPTA_PREFERENCE_STORE_MODE="$preference_mode" \
  HEPTA_MONOTONIC_ANCHOR_FILE="$anchor_file" \
  HEPTA_MONOTONIC_ANCHOR_KEY_FILE="$anchor_key_file" \
  HEPTA_NDU_H1_SHADOW_ENABLED=1 \
  HEPTA_NDU_H1_JOURNAL="$journal_file" \
  HEPTA_NDU_H1_KILL_SWITCH_FILE="$kill_switch_file" \
  HEPTA_NDU_H1_TENANT_SCOPE_HASH='soak-tenant-v1' \
  HEPTA_NDU_H1_CONSENT_SCOPE_HASH='soak-consent-v1' \
  HEPTA_NDU_H1_REVOCATION_SNAPSHOT_HASH='soak-revocation-v1' \
  HEPTA_NDU_H1_MODEL_HASH='soak-model-v1' \
  HEPTA_NDU_H1_SCORER_CONFIG_HASH='soak-scorer-v1' \
  HEPTA_NDU_H1_INITIAL_STATE_HASH='soak-initial-v1' \
  HEPTA_NDU_H1_MAX_EVENTS="$((samples + 16))" \
  "$release_bin" --serve-ui "$bind_addr" --without-telegram-plugin >"$server_log" 2>&1 &
  server_pid="$!"
  local deadline=$((SECONDS + 120))
  until curl -fsS "$base_url/api/ndu/h1/status" >/dev/null 2>&1; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "NDU H1 soak server exited during startup" >&2
      tail -n 120 "$server_log" >&2 || true
      return 1
    fi
    if (( SECONDS >= deadline )); then
      echo "timed out waiting for NDU H1 soak server" >&2
      tail -n 120 "$server_log" >&2 || true
      return 1
    fi
    sleep 0.25
  done
}

status_json() {
  curl -fsS "$base_url/api/ndu/h1/status"
}

run_canary() {
  curl -fsS -H 'Content-Type: application/json' \
    --data '{"dry_run":true}' "$base_url/api/actions/runtime-kernel-canary"
}

start_server
initial_status="$(status_json)"
jq -e '.enabled and .ready and .accepting_observations and .shadow_only and
  (.production_authority_granted | not) and .observed_event_count == 0' \
  <<<"$initial_status" >/dev/null

for ((sample = 1; sample <= samples; sample++)); do
  run_canary | jq -e '.status == "succeeded" and .external_network_requested == false and
    .external_side_effects == false and .live_surface_expanded == false' >/dev/null
done
live_status="$(status_json)"
jq -e --argjson samples "$samples" '.ready and .shadow_only and
  (.production_authority_granted | not) and .observed_event_count == $samples and
  .recorded_count == $samples and .replay_count == 0 and .rejected_count == 0 and
  .evaluation.observed_event_count == $samples and
  (.evaluation.arms | length) == 4 and
  (.evaluation.arms | all(.sample_count == $samples and .feasibility_violation_count == 0)) and
  (.evaluation.promotion_eligible | not) and
  (.evaluation.production_authority_granted | not)' <<<"$live_status" >/dev/null

stop_server
start_server
recovered_status="$(status_json)"
jq -e --argjson samples "$samples" '.ready and .observed_event_count == $samples and
  .recorded_count == 0 and .replay_count == 0 and .rejected_count == 0 and
  .evaluation.observed_event_count == $samples and
  (.evaluation.arms | all(.sample_count == $samples))' <<<"$recovered_status" >/dev/null

: >"$kill_switch_file"
kill_switch_status="$(status_json)"
jq -e '.kill_switch_active and (.ready | not) and (.accepting_observations | not) and
  .shadow_only and (.production_authority_granted | not)' <<<"$kill_switch_status" >/dev/null
run_canary | jq -e '.status == "succeeded" and .external_side_effects == false' >/dev/null
rejected_status="$(status_json)"
jq -e --argjson samples "$samples" '.kill_switch_active and
  .observed_event_count == $samples and .rejected_count == 1 and
  .last_error == "kill_switch_active" and (.production_authority_granted | not)' \
  <<<"$rejected_status" >/dev/null

stop_server
rm -f "$kill_switch_file"
start_server
run_canary | jq -e '.status == "succeeded" and .external_side_effects == false' >/dev/null
resumed_status="$(status_json)"
jq -e --argjson samples "$samples" '.ready and .accepting_observations and
  .observed_event_count == ($samples + 1) and .recorded_count == 1 and
  .rejected_count == 0 and .evaluation.observed_event_count == ($samples + 1) and
  (.evaluation.arms | all(.sample_count == ($samples + 1))) and
  .shadow_only and (.production_authority_granted | not)' <<<"$resumed_status" >/dev/null
stop_server

binary_sha256="$(shasum -a 256 "$release_bin" | awk '{print $1}')"
journal_sha256="$(shasum -a 256 "$journal_file" | awk '{print $1}')"
jq -n \
  --arg artifact_dir "$artifact_dir" \
  --arg binary "$release_bin" \
  --arg binary_sha256 "$binary_sha256" \
  --arg journal "$journal_file" \
  --arg journal_sha256 "$journal_sha256" \
  --argjson samples "$samples" \
  --argjson initial "$initial_status" \
  --argjson live "$live_status" \
  --argjson recovered "$recovered_status" \
  --argjson kill_switch "$kill_switch_status" \
  --argjson rejected "$rejected_status" \
  --argjson resumed "$resumed_status" \
  '{schema:"hepta_ndu_h1_live_shadow_soak_v1",status:"ready",artifact_dir:$artifact_dir,
    isolated_loopback:true,active_service_mutated:false,telegram_plugin_enabled:false,
    production_authority_granted:false,sample_count:$samples,binary:$binary,
    binary_sha256:$binary_sha256,journal:$journal,journal_sha256:$journal_sha256,
    initial:$initial,live:$live,recovered:$recovered,kill_switch:$kill_switch,
    rejected:$rejected,resumed:$resumed}' | tee "$receipt_file"
