#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
usage: hepta-live-soak.sh [--manifest PATH] [--url http://127.0.0.1:PORT] [--samples N] [--interval-seconds N]

Defaults to the production-compatible loopback URL. Pass a canary manifest to
bind the isolated state root and port (recommended port: 17373).
EOF
  exit 64
}

manifest=""; live_url=""; samples="${HEPTA_SOAK_SAMPLES:-12}"; interval="${HEPTA_SOAK_INTERVAL_SECONDS:-1}"
while (( $# > 0 )); do
  case "$1" in
    --manifest) shift; [[ $# -gt 0 ]] || usage; manifest="$1" ;;
    --url) shift; [[ $# -gt 0 ]] || usage; live_url="$1" ;;
    --samples) shift; [[ $# -gt 0 ]] || usage; samples="$1" ;;
    --interval-seconds) shift; [[ $# -gt 0 ]] || usage; interval="$1" ;;
    *) usage ;;
  esac
  shift
done
if ! [[ "$samples" =~ ^[0-9]+$ ]] || ! (( samples >= 1 && samples <= 10000 )); then
  echo "--samples must be from 1 through 10000" >&2
  exit 1
fi
[[ "$interval" =~ ^[0-9]+([.][0-9]+)?$ ]] || {
  echo "--interval-seconds must be a non-negative number" >&2
  exit 1
}
state_root=""
if [[ -n "$manifest" ]]; then
  manifest="$(realpath "$manifest")"
  release="$(dirname "$manifest")"
  verify_relative="$(jq -r '.watchdog.verify_tool' "$manifest")"
  [[ "$verify_relative" == "scripts/hepta-immutable-release-tree" ]] || {
    echo "manifest declares an unsupported verifier" >&2
    exit 1
  }
  "$release/$verify_relative" verify --manifest "$manifest" >/dev/null
  state_root="$(jq -r '.runtime.state_root' "$manifest")"
  listen_port="$(jq -r '.runtime.listen_port' "$manifest")"
  live_url="${live_url:-http://127.0.0.1:$listen_port}"
  [[ "$live_url" == "http://127.0.0.1:$listen_port" ]] || {
    echo "soak URL differs from manifest-bound loopback port" >&2
    exit 1
  }
else
  live_url="${live_url:-http://127.0.0.1:7373}"
  [[ "$live_url" =~ ^http://127[.]0[.]0[.]1:[0-9]+$ ]] || {
    echo "soak requires an explicit IPv4 loopback URL" >&2
    exit 1
  }
fi

started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
passed=0
for (( sample = 1; sample <= samples; sample++ )); do
  health="$(curl --fail --silent --show-error --max-time 5 "$live_url/healthz")"
  runtime="$(curl --fail --silent --show-error --max-time 5 "$live_url/api/hepta/runtime")"
  jq -e '.product == "hepta" and .status == "ok"' <<<"$health" >/dev/null
  jq_filter='
    .schema == "hepta_vnext_live_runtime_status_v1"
    and .status == "ready"
    and .state.schema_version == 5
    and .state.open_mode == "immutable-query-only-open-existing"
    and .state.integrity_verification == "hmac-sha256-v1-key-id-and-row-macs-verified"
    and ([.authority[]] | all(. == false))
  '
  if [[ -n "$state_root" ]]; then
    jq -e --arg state_root "$state_root" "($jq_filter) and .state_root == \$state_root" <<<"$runtime" >/dev/null
  else
    jq -e "$jq_filter" <<<"$runtime" >/dev/null
  fi
  passed=$((passed + 1))
  if (( sample < samples )); then
    sleep "$interval"
  fi
done
finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
jq -cn \
  --arg manifest "$manifest" \
  --arg live_url "$live_url" \
  --arg state_root "$state_root" \
  --arg started_at "$started_at" \
  --arg finished_at "$finished_at" \
  --argjson samples "$samples" \
  --argjson passed "$passed" \
  '{schema:"hepta_vnext_live_soak_v1",status:"ready",manifest:(if $manifest=="" then null else $manifest end),live_url:$live_url,state_root:(if $state_root=="" then null else $state_root end),started_at:$started_at,finished_at:$finished_at,samples:$samples,passed:$passed,failed:($samples-$passed),schema_v5_open_existing:true,keyed_integrity_verified:true,immutable_query_only:true,authority_all_closed:true,service_changed:false}'
