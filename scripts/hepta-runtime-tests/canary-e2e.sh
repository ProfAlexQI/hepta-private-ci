#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: canary-e2e.sh --binary PATH [--source-state-root DIR]" >&2
  exit 64
}

binary=""
source_state_root=""
while (( $# > 0 )); do
  case "$1" in
    --binary) shift; [[ $# -gt 0 ]] || usage; binary="$1" ;;
    --source-state-root) shift; [[ $# -gt 0 ]] || usage; source_state_root="$1" ;;
    *) usage ;;
  esac
  shift
done
[[ -x "$binary" && -f "$binary" && ! -L "$binary" ]] || usage
binary="$(realpath "$binary")"
if [[ -n "$source_state_root" ]]; then
  source_state_root="$(realpath "$source_state_root")"
  [[ "$source_state_root" == /* && "$source_state_root" != "/" ]] || usage
fi
if lsof -nP -iTCP:17373 -sTCP:LISTEN 2>/dev/null | grep -q .; then
  echo "isolated canary port 17373 is already in use" >&2
  exit 1
fi

tmp_root="${TMPDIR:-/tmp}"
root="$(mktemp -d "${tmp_root%/}/hepta-vnext-canary.XXXXXX")"
server_pid=""
cleanup() {
  if [[ -n "$server_pid" ]]; then
    kill "$server_pid" >/dev/null 2>&1 || true
    wait "$server_pid" >/dev/null 2>&1 || true
  fi
  chmod -R u+w "$root" 2>/dev/null || true
  rm -rf "$root"
}
trap cleanup EXIT
state_root="$root/state"
runtime_root="$state_root/runtime-v2"
keys_root="$runtime_root/keys"
install_root="$root/install"
release="$root/release"
launch_agents="$root/LaunchAgents"
inventory_tree() {
  local tree_root="$1" output="$2"
  [[ -d "$tree_root" && ! -L "$tree_root" ]] || return 1
  (
    cd "$tree_root"
    find . -print | LC_ALL=C sort | while IFS= read -r item; do
      [[ ! -L "$item" ]] || { echo "symlink rejected in inventory: $item" >&2; exit 1; }
      if [[ -f "$item" ]]; then
        printf '%s|file|%s|%s\n' "$item" "$(stat -f '%Lp|%u|%g|%z|%m|%c' "$item")" "$(shasum -a 256 "$item" | awk '{print $1}')"
      elif [[ -d "$item" ]]; then
        printf '%s|directory|%s\n' "$item" "$(stat -f '%Lp|%u|%g|%z|%m|%c' "$item")"
      else
        echo "unsupported state entry: $item" >&2
        exit 1
      fi
    done
  ) >"$output"
}

payload_inventory() {
  local tree_root="$1" output="$2"
  (
    cd "$tree_root"
    find . -type f -print | LC_ALL=C sort | while IFS= read -r item; do
      printf '%s|%s|%s|%s\n' "$item" "$(stat -f '%Lp' "$item")" "$(stat -f '%z' "$item")" "$(shasum -a 256 "$item" | awk '{print $1}')"
    done
  ) >"$output"
}

live_before=""; live_after=""; live_payload=""; copy_payload=""
mkdir -p "$state_root" "$launch_agents"
chmod 0700 "$state_root" "$launch_agents"
if [[ -n "$source_state_root" ]]; then
  source_runtime="$source_state_root/runtime-v2"
  [[ -d "$source_runtime" && ! -L "$source_runtime" ]] || {
    echo "source runtime-v2 is missing or unsafe" >&2
    exit 1
  }
  live_before="$root/live-before.inventory"
  live_after="$root/live-after.inventory"
  live_payload="$root/live.payload"
  copy_payload="$root/copy.payload"
  inventory_tree "$source_runtime" "$live_before"
  cp -pR "$source_runtime" "$state_root/runtime-v2"
  inventory_tree "$source_runtime" "$root/live-after-copy.inventory"
  diff -u "$live_before" "$root/live-after-copy.inventory" >/dev/null || {
    echo "source state changed while making the private copy" >&2
    exit 1
  }
  payload_inventory "$source_runtime" "$live_payload"
  payload_inventory "$runtime_root" "$copy_payload"
  diff -u "$live_payload" "$copy_payload" >/dev/null || {
    echo "private state copy differs from source bytes or modes" >&2
    exit 1
  }
else
  mkdir -p "$keys_root"
  chmod 0700 "$runtime_root" "$keys_root"
  for key in runtime-integrity.key preference-integrity.key preference-ingress-auth.key; do
    printf '%064d\n' 0 >"$keys_root/$key"
    chmod 0600 "$keys_root/$key"
  done
  for database in outcomes.sqlite3 preferences.sqlite3; do
    sqlite3 "$runtime_root/$database" >/dev/null <<'SQL'
PRAGMA journal_mode=DELETE;
CREATE TABLE hepta_v2_schema (singleton INTEGER PRIMARY KEY, version INTEGER NOT NULL);
INSERT INTO hepta_v2_schema VALUES (1, 5);
CREATE TABLE hepta_v2_write_lock (singleton INTEGER PRIMARY KEY, generation INTEGER NOT NULL);
INSERT INTO hepta_v2_write_lock VALUES (1, 0);
CREATE TABLE hepta_v2_integrity (singleton INTEGER PRIMARY KEY, algorithm TEXT NOT NULL, key_id TEXT NOT NULL);
INSERT INTO hepta_v2_integrity VALUES (1, 'hmac-sha256-v1', 'sha256:4a75c5baf4bd27a70e3a28856ec5ff1e54c91c7c9bb8d3151b1a9aae279ff4bc');
CREATE TABLE hepta_v2_outcome_records (receipt_id TEXT PRIMARY KEY, attempt_id TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_outcome_intents (attempt_id TEXT PRIMARY KEY, receipt_id TEXT NOT NULL, state TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_execution_intents (attempt_id TEXT PRIMARY KEY, idempotency_key TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_execution_effect_acks (attempt_id TEXT PRIMARY KEY, idempotency_key TEXT NOT NULL, effect_plan_hash TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_preference_genesis (preference_id TEXT NOT NULL, subject_id TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_preference_heads (preference_id TEXT NOT NULL, subject_id TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
CREATE TABLE hepta_v2_preference_transitions (sequence INTEGER PRIMARY KEY, transition_id TEXT NOT NULL, evidence_id TEXT NOT NULL, receipt_id TEXT NOT NULL, preference_id TEXT NOT NULL, subject_id TEXT NOT NULL, payload_json TEXT NOT NULL, storage_hash TEXT NOT NULL);
SQL
    chmod 0600 "$runtime_root/$database"
  done
  printf '%s\n' '{"payload":{"version":1,"generation":0,"snapshot":{"sessions":[],"memories":[],"transcripts":[]}},"integrity_tag":"hmac-sha256:5fec32ebcd9fa7ee6c5b30ede59ba942a0e9760123594318611bf7a258e71b8b"}' >"$runtime_root/runtime-state.json"
  chmod 0600 "$runtime_root/runtime-state.json"
fi

before_inventory="$root/state-before.inventory"
after_inventory="$root/state-after.inventory"
inventory_tree "$runtime_root" "$before_inventory"

"$binary" --serve-ui 127.0.0.1:17373 --state-root "$state_root" \
  >"$root/gateway.stdout" 2>"$root/gateway.stderr" &
server_pid=$!
ready=false
for _ in $(seq 1 300); do
  if curl --fail --silent --max-time 1 http://127.0.0.1:17373/healthz >/dev/null 2>&1; then
    ready=true
    break
  fi
  if ! kill -0 "$server_pid" >/dev/null 2>&1; then
    sed -n '1,160p' "$root/gateway.stderr" >&2
    echo "canary gateway exited before readiness" >&2
    exit 1
  fi
  sleep 0.1
done
[[ "$ready" == "true" ]] || {
  sed -n '1,160p' "$root/gateway.stderr" >&2
  echo "canary gateway did not become ready" >&2
  exit 1
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
"$script_dir/hepta-immutable-release-tree" materialize \
  --binary "$binary" \
  --destination "$release" \
  --state-root "$state_root" \
  --install-root "$install_root" \
  --gateway-label ai.hepta.vnext.canary \
  --watchdog-label ai.hepta.vnext.canary.watchdog \
  --listen-port 17373 \
  --source-commit canary-e2e >/dev/null
"$release/scripts/hepta-generation-pointer" initialize \
  --install-root "$install_root" \
  --manifest "$release/manifest.json" >/dev/null
"$release/scripts/hepta-install-live-gateway" \
  --manifest "$release/manifest.json" \
  --launch-agent-root "$launch_agents" >/dev/null
"$release/scripts/hepta-install-live-watchdog" \
  --manifest "$release/manifest.json" \
  --launch-agent-root "$launch_agents" >/dev/null
watchdog="$("$release/scripts/hepta-watchdog.sh" --manifest "$release/manifest.json")"
soak="$("$release/scripts/hepta-live-soak.sh" --manifest "$release/manifest.json" --samples 3 --interval-seconds 0)"
jq -e '.status == "ready" and .authority_all_closed == true' <<<"$watchdog" >/dev/null
jq -e '.status == "ready" and .passed == 3 and .failed == 0 and .authority_all_closed == true' <<<"$soak" >/dev/null

inventory_tree "$runtime_root" "$after_inventory"
diff -u "$before_inventory" "$after_inventory" >/dev/null || {
  echo "read-only canary changed its state fixture" >&2
  exit 1
}
source_state_copied=false
source_unchanged=true
if [[ -n "$source_state_root" ]]; then
  source_state_copied=true
  inventory_tree "$source_runtime" "$live_after"
  diff -u "$live_before" "$live_after" >/dev/null || {
    echo "source state changed during the private-copy canary" >&2
    exit 1
  }
fi
jq -cn \
  --arg binary_sha256 "$(shasum -a 256 "$binary" | awk '{print $1}')" \
  --arg manifest_sha256 "$(shasum -a 256 "$release/manifest.json" | awk '{print $1}')" \
  --argjson source_state_copied "$source_state_copied" \
  --argjson source_unchanged "$source_unchanged" \
  '{schema:"hepta_vnext_runtime_canary_e2e_v1",status:"ready",listen_addr:"127.0.0.1:17373",binary_sha256:$binary_sha256,manifest_sha256:$manifest_sha256,schema_v5_open_existing:true,keyed_integrity_verified:true,immutable_query_only:true,requires_empty_wal:true,source_state_copied:$source_state_copied,source_unchanged:$source_unchanged,copy_unchanged:true,metadata_and_hash_checked:true,installer_dry_run:true,authority_all_closed:true,production_service_changed:false}'
