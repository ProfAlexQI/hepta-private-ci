#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.cross-agent-visibility-20260619}"
REPORT_PATH="${HEPTA_UI_CROSS_AGENT_VISIBILITY_REPORT_PATH:-$READINESS_DIR/ui-cross-agent-visibility-gate.json}"
VISIBILITY_DIR="${HEPTA_UI_CROSS_AGENT_VISIBILITY_DIR:-$READINESS_DIR/cross-agent-visibility}"
VISIBILITY_MARKDOWN_PATH="$VISIBILITY_DIR/cross-agent-visibility.md"

AGENTS_LIST_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_AGENTS_LIST_PATH:-}"
SESSIONS_LIST_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_SESSIONS_LIST_PATH:-}"
SESSIONS_SEND_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_SESSIONS_SEND_PATH:-}"
CONFIG_PATCH_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_CONFIG_PATCH_PATH:-}"
SESSIONS_VISIBILITY_SCHEMA_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_SESSIONS_VISIBILITY_SCHEMA_PATH:-}"
AGENT_TO_AGENT_SCHEMA_EVIDENCE_PATH="${HEPTA_UI_CROSS_AGENT_AGENT_TO_AGENT_SCHEMA_PATH:-}"

OPENCLAW_CONFIG_PATH="${HEPTA_UI_OPENCLAW_CONFIG_PATH:-/Users/qianqi/.openclaw/openclaw.json}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI cross-agent visibility gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required cross-agent visibility input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

capture_command() {
  local output_path="$1"
  shift

  set +e
  "$@" >"$output_path.stdout" 2>"$output_path.stderr"
  local rc=$?
  set -e

  jq -n \
    --arg command "$*" \
    --arg stdout_path "$output_path.stdout" \
    --arg stderr_path "$output_path.stderr" \
    --arg stdout "$(cat "$output_path.stdout")" \
    --arg stderr "$(cat "$output_path.stderr")" \
    --argjson exit_code "$rc" \
    '{
      command:$command,
      exit_code:$exit_code,
      stdout_path:$stdout_path,
      stderr_path:$stderr_path,
      stdout:$stdout,
      stderr:$stderr
    }' >"$output_path.json"
}

require_command jq
require_command shasum
require_command openclaw
require_command security
require_command xcrun

require_report "$AGENTS_LIST_EVIDENCE_PATH"
require_report "$SESSIONS_LIST_EVIDENCE_PATH"
require_report "$SESSIONS_SEND_EVIDENCE_PATH"
require_report "$CONFIG_PATCH_EVIDENCE_PATH"
require_report "$SESSIONS_VISIBILITY_SCHEMA_EVIDENCE_PATH"
require_report "$AGENT_TO_AGENT_SCHEMA_EVIDENCE_PATH"
require_report "$OPENCLAW_CONFIG_PATH"

rm -rf "$VISIBILITY_DIR"
mkdir -p "$VISIBILITY_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-cross-agent-visibility.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/cross-agent-visibility-draft.json"
REPORT_TMP="$TMP_DIR/cross-agent-visibility-report.json"
MARKDOWN_TMP="$TMP_DIR/cross-agent-visibility.md"
LOCAL_TOOLS_CONFIG_PATH="$TMP_DIR/local-tools-config.json"
LOCAL_ALL_AGENT_SESSIONS_PATH="$VISIBILITY_DIR/openclaw-sessions-all-agents.json"
SIGNING_IDENTITY_PATH="$VISIBILITY_DIR/codesigning-identities"
NOTARYTOOL_VERSION_PATH="$VISIBILITY_DIR/notarytool-version"
STAPLER_VERSION_PATH="$VISIBILITY_DIR/stapler-version"
PACKAGER_VERSION_PATH="$VISIBILITY_DIR/cargo-packager-version"
trap 'rm -rf "$TMP_DIR"' EXIT

jq '.tools // {}' "$OPENCLAW_CONFIG_PATH" >"$LOCAL_TOOLS_CONFIG_PATH"

set +e
openclaw sessions --json --all-agents --limit 100 >"$LOCAL_ALL_AGENT_SESSIONS_PATH" 2>"$LOCAL_ALL_AGENT_SESSIONS_PATH.stderr"
local_sessions_rc=$?
set -e
if [[ "$local_sessions_rc" -ne 0 ]]; then
  jq -n \
    --arg stderr "$(cat "$LOCAL_ALL_AGENT_SESSIONS_PATH.stderr")" \
    --argjson exit_code "$local_sessions_rc" \
    '{error:"openclaw sessions --all-agents failed", exit_code:$exit_code, stderr:$stderr}' \
    >"$LOCAL_ALL_AGENT_SESSIONS_PATH"
fi

capture_command "$SIGNING_IDENTITY_PATH" security find-identity -v -p codesigning
capture_command "$NOTARYTOOL_VERSION_PATH" xcrun notarytool --version
capture_command "$STAPLER_VERSION_PATH" xcrun stapler --version
if command -v cargo-packager >/dev/null 2>&1; then
  capture_command "$PACKAGER_VERSION_PATH" cargo-packager --version
else
  jq -n \
    '{command:"cargo-packager --version", exit_code:127, stdout:"", stderr:"cargo-packager not found"}' \
    >"$PACKAGER_VERSION_PATH.json"
fi

agents_list_sha="$(file_sha256 "$AGENTS_LIST_EVIDENCE_PATH")"
sessions_list_sha="$(file_sha256 "$SESSIONS_LIST_EVIDENCE_PATH")"
sessions_send_sha="$(file_sha256 "$SESSIONS_SEND_EVIDENCE_PATH")"
config_patch_sha="$(file_sha256 "$CONFIG_PATCH_EVIDENCE_PATH")"
sessions_visibility_schema_sha="$(file_sha256 "$SESSIONS_VISIBILITY_SCHEMA_EVIDENCE_PATH")"
agent_to_agent_schema_sha="$(file_sha256 "$AGENT_TO_AGENT_SCHEMA_EVIDENCE_PATH")"
openclaw_config_sha="$(file_sha256 "$OPENCLAW_CONFIG_PATH")"
local_tools_config_sha="$(file_sha256 "$LOCAL_TOOLS_CONFIG_PATH")"
local_all_agent_sessions_sha="$(file_sha256 "$LOCAL_ALL_AGENT_SESSIONS_PATH")"
signing_identity_sha="$(file_sha256 "$SIGNING_IDENTITY_PATH.json")"
notarytool_sha="$(file_sha256 "$NOTARYTOOL_VERSION_PATH.json")"
stapler_sha="$(file_sha256 "$STAPLER_VERSION_PATH.json")"
packager_sha="$(file_sha256 "$PACKAGER_VERSION_PATH.json")"

env_present_json="$TMP_DIR/env-present.json"
jq -n \
  --argjson apple_id "$([[ -n "${APPLE_ID:-}" ]] && printf true || printf false)" \
  --argjson apple_password "$([[ -n "${APPLE_APP_SPECIFIC_PASSWORD:-}" ]] && printf true || printf false)" \
  --argjson apple_team "$([[ -n "${APPLE_TEAM_ID:-}" ]] && printf true || printf false)" \
  --argjson ac_username "$([[ -n "${AC_USERNAME:-}" ]] && printf true || printf false)" \
  --argjson ac_password "$([[ -n "${AC_PASSWORD:-}" ]] && printf true || printf false)" \
  --argjson notary_profile "$([[ -n "${NOTARYTOOL_PROFILE:-}" ]] && printf true || printf false)" \
  --argjson packager_key "$([[ -n "${CARGO_PACKAGER_SIGN_PRIVATE_KEY:-}" ]] && printf true || printf false)" \
  --argjson packager_cert "$([[ -n "${CARGO_PACKAGER_SIGN_CERTIFICATE:-}" ]] && printf true || printf false)" \
  '{
    APPLE_ID:$apple_id,
    APPLE_APP_SPECIFIC_PASSWORD:$apple_password,
    APPLE_TEAM_ID:$apple_team,
    AC_USERNAME:$ac_username,
    AC_PASSWORD:$ac_password,
    NOTARYTOOL_PROFILE:$notary_profile,
    CARGO_PACKAGER_SIGN_PRIVATE_KEY:$packager_key,
    CARGO_PACKAGER_SIGN_CERTIFICATE:$packager_cert
  }' >"$env_present_json"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_cross_agent_visibility_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg visibility_dir "$VISIBILITY_DIR" \
  --arg visibility_markdown_path "$VISIBILITY_MARKDOWN_PATH" \
  --arg agents_list_path "$AGENTS_LIST_EVIDENCE_PATH" \
  --arg sessions_list_path "$SESSIONS_LIST_EVIDENCE_PATH" \
  --arg sessions_send_path "$SESSIONS_SEND_EVIDENCE_PATH" \
  --arg config_patch_path "$CONFIG_PATCH_EVIDENCE_PATH" \
  --arg sessions_visibility_schema_path "$SESSIONS_VISIBILITY_SCHEMA_EVIDENCE_PATH" \
  --arg agent_to_agent_schema_path "$AGENT_TO_AGENT_SCHEMA_EVIDENCE_PATH" \
  --arg openclaw_config_path "$OPENCLAW_CONFIG_PATH" \
  --arg local_tools_config_path "$LOCAL_TOOLS_CONFIG_PATH" \
  --arg local_all_agent_sessions_path "$LOCAL_ALL_AGENT_SESSIONS_PATH" \
  --arg signing_identity_path "$SIGNING_IDENTITY_PATH.json" \
  --arg notarytool_version_path "$NOTARYTOOL_VERSION_PATH.json" \
  --arg stapler_version_path "$STAPLER_VERSION_PATH.json" \
  --arg packager_version_path "$PACKAGER_VERSION_PATH.json" \
  --arg agents_list_sha "$agents_list_sha" \
  --arg sessions_list_sha "$sessions_list_sha" \
  --arg sessions_send_sha "$sessions_send_sha" \
  --arg config_patch_sha "$config_patch_sha" \
  --arg sessions_visibility_schema_sha "$sessions_visibility_schema_sha" \
  --arg agent_to_agent_schema_sha "$agent_to_agent_schema_sha" \
  --arg openclaw_config_sha "$openclaw_config_sha" \
  --arg local_tools_config_sha "$local_tools_config_sha" \
  --arg local_all_agent_sessions_sha "$local_all_agent_sessions_sha" \
  --arg signing_identity_sha "$signing_identity_sha" \
  --arg notarytool_sha "$notarytool_sha" \
  --arg stapler_sha "$stapler_sha" \
  --arg packager_sha "$packager_sha" \
  --slurpfile agents_list_file "$AGENTS_LIST_EVIDENCE_PATH" \
  --slurpfile sessions_list_file "$SESSIONS_LIST_EVIDENCE_PATH" \
  --slurpfile sessions_send_file "$SESSIONS_SEND_EVIDENCE_PATH" \
  --slurpfile config_patch_file "$CONFIG_PATCH_EVIDENCE_PATH" \
  --slurpfile sessions_visibility_schema_file "$SESSIONS_VISIBILITY_SCHEMA_EVIDENCE_PATH" \
  --slurpfile agent_to_agent_schema_file "$AGENT_TO_AGENT_SCHEMA_EVIDENCE_PATH" \
  --slurpfile openclaw_config_file "$OPENCLAW_CONFIG_PATH" \
  --slurpfile local_tools_config_file "$LOCAL_TOOLS_CONFIG_PATH" \
  --slurpfile local_all_agent_sessions_file "$LOCAL_ALL_AGENT_SESSIONS_PATH" \
  --slurpfile signing_identity_file "$SIGNING_IDENTITY_PATH.json" \
  --slurpfile notarytool_file "$NOTARYTOOL_VERSION_PATH.json" \
  --slurpfile stapler_file "$STAPLER_VERSION_PATH.json" \
  --slurpfile packager_file "$PACKAGER_VERSION_PATH.json" \
  --slurpfile env_present_file "$env_present_json" \
  '
  ($agents_list_file[0]) as $agents
  | ($sessions_list_file[0]) as $visible_sessions
  | ($sessions_send_file[0]) as $send
  | ($config_patch_file[0]) as $patch
  | ($sessions_visibility_schema_file[0]) as $sessions_schema
  | ($agent_to_agent_schema_file[0]) as $agent_schema
  | ($openclaw_config_file[0]) as $config
  | ($local_tools_config_file[0]) as $tools
  | ($local_all_agent_sessions_file[0]) as $all_sessions
  | ($signing_identity_file[0]) as $identities
  | ($notarytool_file[0]) as $notarytool
  | ($stapler_file[0]) as $stapler
  | ($packager_file[0]) as $packager
  | ($env_present_file[0]) as $env
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def hepta_backend_session_count:
      (($all_sessions.sessions // []) | map(select(.agentId == "hepta-backend")) | length);
    def current_tool_sessions_visibility:
      ($tools.sessions.visibility // "tree_default");
    def current_agent_to_agent_enabled:
      ($tools.agentToAgent.enabled // false);
    def current_agent_to_agent_allow:
      ($tools.agentToAgent.allow // []);
    def valid_identity_count:
      ($identities.stdout | capture("(?<count>[0-9]+) valid identities found").count | tonumber? // 0);
    def notary_env_ready:
      (($env.APPLE_ID and $env.APPLE_APP_SPECIFIC_PASSWORD and $env.APPLE_TEAM_ID) or $env.NOTARYTOOL_PROFILE);
    def packager_signing_env_ready:
      ($env.CARGO_PACKAGER_SIGN_PRIVATE_KEY and $env.CARGO_PACKAGER_SIGN_CERTIFICATE);
    def cross_agent_visibility_blocked:
      $agents.allowAny == false
      and (($agents.agents | map(.id)) == ["hepta-ui"])
      and $visible_sessions.visibility.mode == "tree"
      and $visible_sessions.visibility.restricted == true
      and $send.status == "forbidden"
      and ($send.error | contains("tools.sessions.visibility=all"))
      and $patch.ok == false
      and ($patch.error | contains("protected config paths"))
      and current_tool_sessions_visibility == "tree_default"
      and current_agent_to_agent_enabled == false;
    def signing_capability_blocked:
      valid_identity_count == 0
      and notary_env_ready == false
      and packager_signing_env_ready == false
      and $notarytool.exit_code == 0
      and ($stapler.exit_code == 0 or ($stapler.exit_code == 64 and ($stapler.stderr | contains("Usage: stapler"))))
      and ($packager.exit_code == 0 or $packager.exit_code == 127);
    def source_chain_ready:
      cross_agent_visibility_blocked
      and hepta_backend_session_count >= 1
      and signing_capability_blocked
      and $sessions_schema.result.schema.enum == ["self","tree","agent","all"]
      and (($agent_schema.result.children | map(.path)) | index("tools.agentToAgent.enabled") != null)
      and (($agent_schema.result.children | map(.path)) | index("tools.agentToAgent.allow") != null)
      and sha_ready($agents_list_sha)
      and sha_ready($sessions_list_sha)
      and sha_ready($sessions_send_sha)
      and sha_ready($config_patch_sha)
      and sha_ready($sessions_visibility_schema_sha)
      and sha_ready($agent_to_agent_schema_sha)
      and sha_ready($openclaw_config_sha)
      and sha_ready($local_tools_config_sha)
      and sha_ready($local_all_agent_sessions_sha)
      and sha_ready($signing_identity_sha)
      and sha_ready($notarytool_sha)
      and sha_ready($stapler_sha)
      and sha_ready($packager_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      cross_agent_visibility_gate_ready:$ready,
      visibility_kind:"local_cross_agent_receipt_dispatch_visibility_preflight",
      visibility_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      visibility_dir:$visibility_dir,
      visibility_markdown_path:$visibility_markdown_path,
      source_reports:{
        agents_list:$agents_list_path,
        sessions_list:$sessions_list_path,
        sessions_send:$sessions_send_path,
        config_patch:$config_patch_path,
        sessions_visibility_schema:$sessions_visibility_schema_path,
        agent_to_agent_schema:$agent_to_agent_schema_path,
        openclaw_config:$openclaw_config_path,
        local_tools_config:$local_tools_config_path,
        local_all_agent_sessions:$local_all_agent_sessions_path,
        signing_identity:$signing_identity_path,
        notarytool_version:$notarytool_version_path,
        stapler_version:$stapler_version_path,
        packager_version:$packager_version_path
      },
      source_report_sha256:{
        agents_list:$agents_list_sha,
        sessions_list:$sessions_list_sha,
        sessions_send:$sessions_send_sha,
        config_patch:$config_patch_sha,
        sessions_visibility_schema:$sessions_visibility_schema_sha,
        agent_to_agent_schema:$agent_to_agent_schema_sha,
        openclaw_config:$openclaw_config_sha,
        local_tools_config:$local_tools_config_sha,
        local_all_agent_sessions:$local_all_agent_sessions_sha,
        signing_identity:$signing_identity_sha,
        notarytool_version:$notarytool_sha,
        stapler_version:$stapler_sha,
        packager_version:$packager_sha
      },
      cross_agent_visibility:{
        target_agent_id:"hepta-backend",
        agents_list_allow_any:$agents.allowAny,
        agents_list_ids:($agents.agents | map(.id)),
        visible_sessions_count:$visible_sessions.count,
        visible_sessions_mode:$visible_sessions.visibility.mode,
        visible_sessions_restricted:$visible_sessions.visibility.restricted,
        all_agent_session_store_hepta_backend_count:hepta_backend_session_count,
        sessions_send_status:$send.status,
        sessions_send_error:$send.error,
        sessions_send_session_key:$send.sessionKey,
        required_sessions_visibility:"all",
        required_agent_to_agent_enabled:true,
        required_agent_to_agent_allow:["hepta-backend"],
        current_config_sessions_visibility:current_tool_sessions_visibility,
        current_config_agent_to_agent_enabled:current_agent_to_agent_enabled,
        current_config_agent_to_agent_allow:current_agent_to_agent_allow,
        config_patch_attempted:$patch.attempted,
        config_patch_allowed:$patch.ok,
        config_patch_error:$patch.error,
        protected_config_paths:$patch.protected_paths,
        cross_agent_dispatch_ready:false,
        cross_agent_visibility_blocked:cross_agent_visibility_blocked
      },
      signing_notary_capability:{
        valid_codesigning_identity_count:valid_identity_count,
        notarytool_available:($notarytool.exit_code == 0),
        stapler_available:($stapler.exit_code == 0 or ($stapler.exit_code == 64 and ($stapler.stderr | contains("Usage: stapler")))),
        cargo_packager_available:($packager.exit_code == 0),
        notary_env_ready:notary_env_ready,
        packager_signing_env_ready:packager_signing_env_ready,
        credential_values_read:false,
        keychain_identity_lookup_performed:true,
        network_call_performed:false,
        notary_submission_performed:false,
        signing_capability_blocked:signing_capability_blocked,
        signed_notarized_stapled_artifact_ready:false
      },
      remaining_true_blockers:[
        {
          id:"openclaw_cross_agent_visibility_protected",
          owner_lane:"host_openclaw_config",
          state:"blocked",
          required_unblock:"Set tools.sessions.visibility=all and enable tools.agentToAgent.allow for hepta-backend from a host/admin context."
        },
        {
          id:"real_backend_receipt_missing",
          owner_lane:"hepta-backend",
          state:"blocked",
          required_unblock:"After cross-agent access is enabled, execute backend first-five readback and return a real backend receipt."
        },
        {
          id:"signed_notarized_stapled_artifact_missing",
          owner_lane:"release_operator",
          state:"blocked",
          required_unblock:"Install a valid Developer ID signing identity and notary credentials or a notarytool profile, then produce a signed/notarized/stapled artifact."
        },
        {
          id:"public_distribution_artifact_not_written",
          owner_lane:"release_operator",
          state:"blocked",
          required_unblock:"Only after a valid signed/notarized/stapled artifact exists, write the public distribution artifact."
        }
      ],
      next_unblock_sequence:[
        "Host/admin applies protected OpenClaw config: tools.sessions.visibility=all plus tools.agentToAgent.enabled=true and allow=[hepta-backend].",
        "hepta-ui sends the r72 backend first-five dispatch packet to hepta-backend through sessions_send.",
        "hepta-backend executes message_search, file_upload_send, media_download_playback, notifications, and room_settings readback and returns a real receipt.",
        "hepta-ui reruns no-window and full-hard readiness with the real backend receipt.",
        "Release operator installs signing identity and notary credentials/profile, then produces a real signed/notarized/stapled artifact.",
        "hepta-ui reruns release artifact intake and product readiness before any public/live release claim."
      ],
      claim_boundary:{
        local_cross_agent_visibility_preflight_ready:$ready,
        backend_agent_dispatch_claim_ready:false,
        real_backend_receipt_claim_ready:false,
        backend_receipt_claim_ready:false,
        release_artifact_claim_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        signing_notarization_performed:false,
        public_upload_performed:false,
        external_mutation:false
      },
      side_effects:{
        filesystem_read:true,
        local_report_written:true,
        local_markdown_written:true,
        openclaw_config_read:true,
        openclaw_config_patch_attempted:true,
        openclaw_config_patch_applied:false,
        sessions_send_attempted:true,
        sessions_send_delivered:false,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_config_probe:true,
        gateway_call:true,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        keychain_identity_lookup_performed:true,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Cross-Agent Visibility Gate\n\n"
  + "- Status: \(.status)\n"
  + "- Target agent: \(.cross_agent_visibility.target_agent_id)\n"
  + "- Visible agent IDs: \(.cross_agent_visibility.agents_list_ids | join(", "))\n"
  + "- Sessions visibility: \(.cross_agent_visibility.visible_sessions_mode), restricted=\(.cross_agent_visibility.visible_sessions_restricted)\n"
  + "- sessions_send status: \(.cross_agent_visibility.sessions_send_status)\n"
  + "- Config patch allowed: \(.cross_agent_visibility.config_patch_allowed)\n"
  + "- Protected paths: \(.cross_agent_visibility.protected_config_paths | join(", "))\n"
  + "- Valid code signing identities: \(.signing_notary_capability.valid_codesigning_identity_count)\n"
  + "- Notary env ready: \(.signing_notary_capability.notary_env_ready)\n"
  + "- Signed/notarized/stapled artifact ready: \(.signing_notary_capability.signed_notarized_stapled_artifact_ready)\n\n"
  + "## Remaining Blockers\n\n"
  + (.remaining_true_blockers[] | "- `\(.id)` (\(.owner_lane)): \(.required_unblock)\n")
  + "\n## Next Unblock Sequence\n\n"
  + (.next_unblock_sequence[] | "- \(. )\n")
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {visibility_markdown_sha256:$markdown_sha, visibility_markdown_bytes:$markdown_bytes}' \
  "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .cross_agent_visibility_gate_ready == true
  and .visibility_kind == "local_cross_agent_receipt_dispatch_visibility_preflight"
  and .cross_agent_visibility.target_agent_id == "hepta-backend"
  and .cross_agent_visibility.visible_sessions_mode == "tree"
  and .cross_agent_visibility.visible_sessions_restricted == true
  and .cross_agent_visibility.all_agent_session_store_hepta_backend_count >= 1
  and .cross_agent_visibility.sessions_send_status == "forbidden"
  and (.cross_agent_visibility.sessions_send_error | contains("tools.sessions.visibility=all"))
  and .cross_agent_visibility.required_sessions_visibility == "all"
  and .cross_agent_visibility.required_agent_to_agent_enabled == true
  and .cross_agent_visibility.required_agent_to_agent_allow == ["hepta-backend"]
  and .cross_agent_visibility.current_config_sessions_visibility == "tree_default"
  and .cross_agent_visibility.current_config_agent_to_agent_enabled == false
  and .cross_agent_visibility.config_patch_allowed == false
  and (.cross_agent_visibility.protected_config_paths | index("tools.sessions.visibility") != null)
  and (.cross_agent_visibility.protected_config_paths | index("tools.agentToAgent.enabled") != null)
  and (.cross_agent_visibility.protected_config_paths | index("tools.agentToAgent.allow") != null)
  and .cross_agent_visibility.cross_agent_dispatch_ready == false
  and .cross_agent_visibility.cross_agent_visibility_blocked == true
  and .signing_notary_capability.valid_codesigning_identity_count == 0
  and .signing_notary_capability.notarytool_available == true
  and .signing_notary_capability.stapler_available == true
  and .signing_notary_capability.notary_env_ready == false
  and .signing_notary_capability.packager_signing_env_ready == false
  and .signing_notary_capability.credential_values_read == false
  and .signing_notary_capability.network_call_performed == false
  and .signing_notary_capability.notary_submission_performed == false
  and .signing_notary_capability.signing_capability_blocked == true
  and .claim_boundary.backend_agent_dispatch_claim_ready == false
  and .claim_boundary.real_backend_receipt_claim_ready == false
  and .claim_boundary.backend_receipt_claim_ready == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .side_effects.openclaw_config_patch_attempted == true
  and .side_effects.openclaw_config_patch_applied == false
  and .side_effects.sessions_send_attempted == true
  and .side_effects.sessions_send_delivered == false
  and .side_effects.backend_repo_write == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
  and (.visibility_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .visibility_markdown_bytes > 0
' "$REPORT_TMP" >/dev/null

cp "$MARKDOWN_TMP" "$VISIBILITY_MARKDOWN_PATH"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
