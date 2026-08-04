#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."
REPO_ROOT="$(pwd -P)"
. "$REPO_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_REPORT_PATH:-$READINESS_DIR/ui-backend-delivery-receipt-roundtrip-gate.json}"
ROUNDTRIP_DIR="${HEPTA_UI_BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_DIR:-$READINESS_DIR/backend-delivery-receipt-roundtrip}"
READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
ROUNDTRIP_DIR="$(hepta_safe_normalize_path roundtrip "$ROUNDTRIP_DIR")"
SIMULATED_RECEIPT_PATH="$ROUNDTRIP_DIR/simulated-delivery-receipt.json"
WAITING_AUDIT_DIR="$ROUNDTRIP_DIR/waiting-delivery-audit"
WAITING_AUDIT_REPORT_PATH="$ROUNDTRIP_DIR/ui-backend-delivery-audit-waiting-gate.json"
SIMULATED_AUDIT_DIR="$ROUNDTRIP_DIR/simulated-delivery-audit"
SIMULATED_AUDIT_REPORT_PATH="$ROUNDTRIP_DIR/ui-backend-delivery-audit-present-gate.json"
ROUNDTRIP_MARKDOWN_PATH="$ROUNDTRIP_DIR/backend-delivery-receipt-roundtrip.md"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target roundtrip "$ROUNDTRIP_DIR"
hepta_safe_require_directory_target waiting_audit "$WAITING_AUDIT_DIR"
hepta_safe_require_directory_target simulated_audit "$SIMULATED_AUDIT_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
hepta_safe_require_regular_target simulated_receipt "$SIMULATED_RECEIPT_PATH"
hepta_safe_require_regular_target waiting_audit_report "$WAITING_AUDIT_REPORT_PATH"
hepta_safe_require_regular_target simulated_audit_report "$SIMULATED_AUDIT_REPORT_PATH"
hepta_safe_require_regular_target roundtrip_markdown "$ROUNDTRIP_MARKDOWN_PATH"
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'backend-delivery roundtrip readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$ROUNDTRIP_DIR" "$READINESS_DIR"; then
  printf 'backend-delivery roundtrip directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'backend-delivery roundtrip report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$ROUNDTRIP_DIR"; then
  printf 'backend-delivery roundtrip report and managed directory must be disjoint\n' >&2
  exit 64
fi

BACKEND_DELIVERY_AUDIT_REPORT_PATH="$READINESS_DIR/ui-backend-delivery-audit-gate.json"
BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH="$READINESS_DIR/backend-delivery-audit/backend-delivery-receipt-template.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
for protected_input in "$BACKEND_DELIVERY_AUDIT_REPORT_PATH" \
  "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH" "$BACKEND_DISPATCH_PACKET_REPORT_PATH"; do
  if hepta_safe_paths_overlap "$protected_input" "$ROUNDTRIP_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'backend-delivery roundtrip output overlaps protected input: %s\n' "$protected_input" >&2
    exit 64
  fi
done

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI backend delivery receipt roundtrip gate"
HEPTA_UI_REPORT_INPUT_LABEL="backend delivery receipt roundtrip"
source scripts/lib/hepta-ui-gate-common-v1.sh

require_command jq
require_command shasum

require_report "$BACKEND_DELIVERY_AUDIT_REPORT_PATH"
require_report "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"

mkdir -p "$ROUNDTRIP_DIR" "$WAITING_AUDIT_DIR" "$SIMULATED_AUDIT_DIR"
mkdir -p "$REPORT_PARENT"
hepta_safe_revalidate_directory roundtrip "$ROUNDTRIP_DIR"
hepta_safe_revalidate_directory waiting_audit "$WAITING_AUDIT_DIR"
hepta_safe_revalidate_directory simulated_audit "$SIMULATED_AUDIT_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-delivery-receipt-roundtrip.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/backend-delivery-receipt-roundtrip-draft.json"
REPORT_TMP="$TMP_DIR/backend-delivery-receipt-roundtrip-report.json"
MARKDOWN_TMP="$TMP_DIR/backend-delivery-receipt-roundtrip.md"
SIMULATED_RECEIPT_TMP="$TMP_DIR/simulated-delivery-receipt.json"
trap 'rm -rf "$TMP_DIR"' EXIT

template_sha="$(file_sha256 "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH")"
dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
template_bytes="$(file_bytes "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH")"

jq -n \
  --slurpfile template_file "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  '
  ($template_file[0]) as $template
  | ($dispatch_file[0]) as $dispatch
  | {
      delivery_kind:"backend_dispatch_packet_delivery_receipt",
      delivery_version:1,
      delivery_mode:"local_simulated_delivery_receipt_roundtrip_only",
      delivered:true,
      delivery_channel:"local_simulated_backend_lane_manual",
      delivery_receipt_id:"simulated-backend-delivery-receipt-r67",
      owner_lane:"backend_contract",
      target_agent_id:"backend_lane_manual",
      target_repo:$template.target_repo,
      dispatch_archive_sha256:$template.dispatch_archive_sha256,
      payload_manifest_sha256:$dispatch.manifest_sha256,
      selected_ids:$template.selected_ids,
      backend_execution_started:false,
      backend_receipt_returned:false,
      live_runtime_mutation:false,
      external_public_action:false,
      simulated_provenance:{
        source:"hepta-ui-backend-delivery-receipt-roundtrip-gate",
        backend_agent_spawned:false,
        backend_repo_write:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        network_delivery_performed:false,
        external_mutation:false
      },
      claim_boundary:{
        simulated_delivery_receipt_branch_ready:true,
        backend_delivery_claim_ready:false,
        real_backend_receipt_claim_ready:false,
        backend_receipt_claim_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      }
    }' >"$SIMULATED_RECEIPT_TMP"
hepta_safe_atomic_replace "$SIMULATED_RECEIPT_TMP" "$SIMULATED_RECEIPT_PATH" simulated_delivery_receipt

env -u HEPTA_UI_BACKEND_DELIVERY_RECEIPT_INPUT_PATH \
  HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
  HEPTA_UI_BACKEND_DELIVERY_AUDIT_REPORT_PATH="$WAITING_AUDIT_REPORT_PATH" \
  HEPTA_UI_BACKEND_DELIVERY_AUDIT_DIR="$WAITING_AUDIT_DIR" \
  ./scripts/hepta-ui-backend-delivery-audit-gate.sh >/dev/null

require_report "$WAITING_AUDIT_REPORT_PATH"

waiting_audit_sha="$(file_sha256 "$WAITING_AUDIT_REPORT_PATH")"

env \
  HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
  HEPTA_UI_BACKEND_DELIVERY_AUDIT_REPORT_PATH="$SIMULATED_AUDIT_REPORT_PATH" \
  HEPTA_UI_BACKEND_DELIVERY_AUDIT_DIR="$SIMULATED_AUDIT_DIR" \
  HEPTA_UI_BACKEND_DELIVERY_RECEIPT_INPUT_PATH="$SIMULATED_RECEIPT_PATH" \
  ./scripts/hepta-ui-backend-delivery-audit-gate.sh >/dev/null

require_report "$SIMULATED_AUDIT_REPORT_PATH"

simulated_receipt_sha="$(file_sha256 "$SIMULATED_RECEIPT_PATH")"
simulated_audit_sha="$(file_sha256 "$SIMULATED_AUDIT_REPORT_PATH")"
simulated_receipt_bytes="$(file_bytes "$SIMULATED_RECEIPT_PATH")"
simulated_audit_bytes="$(file_bytes "$SIMULATED_AUDIT_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_delivery_receipt_roundtrip_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg roundtrip_dir "$ROUNDTRIP_DIR" \
  --arg markdown_path "$ROUNDTRIP_MARKDOWN_PATH" \
  --arg waiting_audit_report_path "$WAITING_AUDIT_REPORT_PATH" \
  --arg delivery_receipt_template_path "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH" \
  --arg dispatch_packet_report_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg simulated_receipt_path "$SIMULATED_RECEIPT_PATH" \
  --arg simulated_audit_report_path "$SIMULATED_AUDIT_REPORT_PATH" \
  --arg waiting_audit_sha "$waiting_audit_sha" \
  --arg template_sha "$template_sha" \
  --arg dispatch_sha "$dispatch_sha" \
  --arg simulated_receipt_sha "$simulated_receipt_sha" \
  --arg simulated_audit_sha "$simulated_audit_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson simulated_receipt_bytes "$simulated_receipt_bytes" \
  --argjson simulated_audit_bytes "$simulated_audit_bytes" \
  --slurpfile waiting_audit_file "$WAITING_AUDIT_REPORT_PATH" \
  --slurpfile template_file "$BACKEND_DELIVERY_RECEIPT_TEMPLATE_PATH" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile simulated_receipt_file "$SIMULATED_RECEIPT_PATH" \
  --slurpfile simulated_audit_file "$SIMULATED_AUDIT_REPORT_PATH" \
  '
  ($waiting_audit_file[0]) as $waiting
  | ($template_file[0]) as $template
  | ($dispatch_file[0]) as $dispatch
  | ($simulated_receipt_file[0]) as $receipt
  | ($simulated_audit_file[0]) as $present
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def waiting_branch_ready:
      $waiting.backend_delivery_audit_gate_ready == true
      and $waiting.status == "ready"
      and $waiting.audit_kind == "local_backend_dispatch_delivery_boundary"
      and $waiting.delivery_state.delivery_receipt_present == false
      and $waiting.delivery_state.delivery_receipt_valid == false
      and $waiting.delivery_state.waiting_for_delivery_receipt == true
      and $waiting.delivery_state.backend_delivery_claim_ready == false
      and ($waiting.delivery_state.real_backend_receipt_present | type) == "boolean"
      and ($waiting.delivery_state.backend_receipt_valid | type) == "boolean"
      and $waiting.critical_blocker_count == ($waiting.source_alignment.blocker_closure_critical_blocker_count + 1)
      and $waiting.claim_boundary.local_backend_delivery_audit_ready == true
      and $waiting.claim_boundary.backend_delivery_claim_ready == false
      and ($waiting.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
      and ($waiting.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
      and $waiting.claim_boundary.live_product_claim_ready == false
      and $waiting.claim_boundary.release_claim_ready == false;
    def simulated_receipt_ready:
      $receipt.delivery_kind == "backend_dispatch_packet_delivery_receipt"
      and $receipt.delivery_version == 1
      and $receipt.delivery_mode == "local_simulated_delivery_receipt_roundtrip_only"
      and $receipt.delivered == true
      and $receipt.target_agent_id == "backend_lane_manual"
      and $receipt.target_repo == $template.target_repo
      and $receipt.dispatch_archive_sha256 == $template.dispatch_archive_sha256
      and $receipt.dispatch_archive_sha256 == $dispatch.archive_sha256
      and $receipt.payload_manifest_sha256 == $dispatch.manifest_sha256
      and $receipt.selected_ids == selected_ids
      and $receipt.backend_execution_started == false
      and $receipt.backend_receipt_returned == false
      and $receipt.live_runtime_mutation == false
      and $receipt.external_public_action == false
      and $receipt.simulated_provenance.backend_agent_spawned == false
      and $receipt.simulated_provenance.backend_repo_write == false
      and $receipt.simulated_provenance.gateway_call == false
      and $receipt.simulated_provenance.provider_invoked == false
      and $receipt.simulated_provenance.channel_delivery == false
      and $receipt.simulated_provenance.external_mutation == false
      and $receipt.claim_boundary.simulated_delivery_receipt_branch_ready == true
      and $receipt.claim_boundary.backend_delivery_claim_ready == false
      and $receipt.claim_boundary.real_backend_receipt_claim_ready == false
      and $receipt.claim_boundary.backend_receipt_claim_ready == false
      and $receipt.claim_boundary.release_claim_ready == false;
    def present_branch_ready:
      $present.backend_delivery_audit_gate_ready == true
      and $present.status == "ready"
      and $present.audit_kind == "local_backend_dispatch_delivery_boundary"
      and $present.delivery_state.delivery_receipt_present == true
      and $present.delivery_state.delivery_receipt_valid == true
      and $present.delivery_state.delivery_receipt_sha256 == $simulated_receipt_sha
      and $present.delivery_state.delivery_receipt_bytes == $simulated_receipt_bytes
      and $present.delivery_state.waiting_for_delivery_receipt == false
      and $present.delivery_state.backend_delivery_claim_ready == true
      and ($present.delivery_state.real_backend_receipt_present | type) == "boolean"
      and ($present.delivery_state.backend_receipt_valid | type) == "boolean"
      and $present.critical_blocker_count == $present.source_alignment.delivery_base_critical_blocker_count
      and (
        $present.source_alignment.stale_backend_agent_dispatch_blocker_suppressed == false
        or ($present.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) == null
      )
      and (
        ($present.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) == null
        or ($present.critical_blockers | map(select(.id == "backend_dispatch_delivery_receipt_missing" and .state == "ready")) | length) == 1
      )
      and $present.claim_boundary.local_backend_delivery_audit_ready == true
      and $present.claim_boundary.backend_delivery_claim_ready == true
      and ($present.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
      and ($present.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
      and $present.claim_boundary.live_product_claim_ready == false
      and $present.claim_boundary.public_distribution_claim_ready == false
      and $present.claim_boundary.release_claim_ready == false
      and $present.side_effects.external_mutation == false;
    (
      waiting_branch_ready
      and simulated_receipt_ready
      and present_branch_ready
      and sha_ready($waiting_audit_sha)
      and sha_ready($template_sha)
      and sha_ready($dispatch_sha)
      and sha_ready($simulated_receipt_sha)
      and sha_ready($simulated_audit_sha)
      and $template_bytes > 0
      and $simulated_receipt_bytes > 0
      and $simulated_audit_bytes > 0
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_delivery_receipt_roundtrip_gate_ready:$ready,
      roundtrip_kind:"local_backend_delivery_receipt_valid_branch_replay",
      roundtrip_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      roundtrip_dir:$roundtrip_dir,
      markdown_path:$markdown_path,
      source_reports:{
        waiting_delivery_audit:$waiting_audit_report_path,
        delivery_receipt_template:$delivery_receipt_template_path,
        backend_dispatch_packet:$dispatch_packet_report_path,
        simulated_delivery_receipt:$simulated_receipt_path,
        simulated_delivery_audit:$simulated_audit_report_path
      },
      source_report_sha256:{
        waiting_delivery_audit:$waiting_audit_sha,
        delivery_receipt_template:$template_sha,
        backend_dispatch_packet:$dispatch_sha,
        simulated_delivery_receipt:$simulated_receipt_sha,
        simulated_delivery_audit:$simulated_audit_sha
      },
      source_report_bytes:{
        delivery_receipt_template:$template_bytes,
        simulated_delivery_receipt:$simulated_receipt_bytes,
        simulated_delivery_audit:$simulated_audit_bytes
      },
      selected_ids:selected_ids,
      roundtrip_ready_count:(
        [waiting_branch_ready, simulated_receipt_ready, present_branch_ready]
        | map(select(. == true))
        | length
      ),
      source_alignment:{
        waiting_branch_ready:waiting_branch_ready,
        simulated_receipt_ready:simulated_receipt_ready,
        present_branch_ready:present_branch_ready,
        waiting_branch_delivery_receipt_present:$waiting.delivery_state.delivery_receipt_present,
        waiting_branch_delivery_receipt_valid:$waiting.delivery_state.delivery_receipt_valid,
        waiting_branch_backend_delivery_claim_ready:$waiting.delivery_state.backend_delivery_claim_ready,
        present_branch_delivery_receipt_present:$present.delivery_state.delivery_receipt_present,
        present_branch_delivery_receipt_valid:$present.delivery_state.delivery_receipt_valid,
        present_branch_backend_delivery_claim_ready:$present.delivery_state.backend_delivery_claim_ready,
        present_branch_critical_blocker_count:$present.critical_blocker_count,
        present_branch_delivery_base_critical_blocker_count:$present.source_alignment.delivery_base_critical_blocker_count,
        present_branch_stale_backend_agent_dispatch_blocker_suppressed:$present.source_alignment.stale_backend_agent_dispatch_blocker_suppressed,
        present_branch_real_backend_receipt_present:$present.delivery_state.real_backend_receipt_present,
        present_branch_backend_receipt_valid:$present.delivery_state.backend_receipt_valid,
        present_branch_real_backend_receipt_claim_ready:$present.claim_boundary.real_backend_receipt_claim_ready,
        present_branch_backend_receipt_claim_ready:$present.claim_boundary.backend_receipt_claim_ready,
        dispatch_archive_match:(
          $receipt.dispatch_archive_sha256 == $dispatch.archive_sha256
          and $present.delivery_state.dispatch_archive_sha256 == $dispatch.archive_sha256
        ),
        payload_manifest_match:($receipt.payload_manifest_sha256 == $dispatch.manifest_sha256),
        root_report_replay_required_count_after_roundtrip:45
      },
      claim_boundary:{
        local_backend_delivery_receipt_roundtrip_ready:$ready,
        local_backend_delivery_audit_ready:$waiting.claim_boundary.local_backend_delivery_audit_ready,
        simulated_delivery_receipt_branch_ready:simulated_receipt_ready,
        simulated_delivery_audit_present_branch_ready:present_branch_ready,
        backend_delivery_claim_ready:false,
        real_backend_receipt_claim_ready:false,
        backend_receipt_claim_ready:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false
      },
      side_effects:{
        filesystem_read:true,
        local_simulated_receipt_written:true,
        local_present_branch_report_written:true,
        local_markdown_written:true,
        local_report_written:true,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Backend Delivery Receipt Roundtrip\n\n"
  + "- Kind: \(.roundtrip_kind)\n"
  + "- Status: \(.status)\n"
  + "- Root replay after this gate: \(.source_alignment.root_report_replay_required_count_after_roundtrip)\n"
  + "- Waiting branch ready: \(.source_alignment.waiting_branch_ready)\n"
  + "- Present branch ready: \(.source_alignment.present_branch_ready)\n"
  + "- Simulated receipt ready: \(.source_alignment.simulated_receipt_ready)\n"
  + "- Present branch delivery receipt valid: \(.source_alignment.present_branch_delivery_receipt_valid)\n"
  + "- Present branch backend delivery claim ready: \(.source_alignment.present_branch_backend_delivery_claim_ready)\n"
  + "- Present branch real backend receipt present: \(.source_alignment.present_branch_real_backend_receipt_present)\n\n"
  + "## Boundary\n\n"
  + "- This is a local simulated delivery receipt replay only.\n"
  + "- It proves the valid delivery receipt branch without claiming backend execution, backend receipt, live product readiness, public distribution, or release readiness.\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

hepta_safe_atomic_replace "$MARKDOWN_TMP" "$ROUNDTRIP_MARKDOWN_PATH" backend_delivery_roundtrip_markdown

markdown_sha="$(file_sha256 "$ROUNDTRIP_MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$ROUNDTRIP_MARKDOWN_PATH")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {
    roundtrip_markdown_sha256:$markdown_sha,
    roundtrip_markdown_bytes:$markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .backend_delivery_receipt_roundtrip_gate_ready == true
  and .roundtrip_kind == "local_backend_delivery_receipt_valid_branch_replay"
  and .roundtrip_version == 1
  and .selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .roundtrip_ready_count == 3
  and .source_alignment.waiting_branch_ready == true
  and .source_alignment.simulated_receipt_ready == true
  and .source_alignment.present_branch_ready == true
  and .source_alignment.waiting_branch_delivery_receipt_present == false
  and .source_alignment.waiting_branch_delivery_receipt_valid == false
  and .source_alignment.waiting_branch_backend_delivery_claim_ready == false
  and .source_alignment.present_branch_delivery_receipt_present == true
  and .source_alignment.present_branch_delivery_receipt_valid == true
  and .source_alignment.present_branch_backend_delivery_claim_ready == true
  and (.source_alignment.present_branch_critical_blocker_count >= 0 and .source_alignment.present_branch_critical_blocker_count <= 10)
  and (.source_alignment.present_branch_real_backend_receipt_present | type) == "boolean"
  and (.source_alignment.present_branch_backend_receipt_valid | type) == "boolean"
  and (.source_alignment.present_branch_real_backend_receipt_claim_ready | type) == "boolean"
  and (.source_alignment.present_branch_backend_receipt_claim_ready | type) == "boolean"
  and .source_alignment.dispatch_archive_match == true
  and .source_alignment.payload_manifest_match == true
  and .source_alignment.root_report_replay_required_count_after_roundtrip == 45
  and .claim_boundary.local_backend_delivery_receipt_roundtrip_ready == true
  and .claim_boundary.local_backend_delivery_audit_ready == true
  and .claim_boundary.simulated_delivery_receipt_branch_ready == true
  and .claim_boundary.simulated_delivery_audit_present_branch_ready == true
  and .claim_boundary.backend_delivery_claim_ready == false
  and .claim_boundary.real_backend_receipt_claim_ready == false
  and .claim_boundary.backend_receipt_claim_ready == false
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and (.source_report_sha256.waiting_delivery_audit | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.delivery_receipt_template | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.backend_dispatch_packet | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.simulated_delivery_receipt | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.simulated_delivery_audit | test("^[0-9a-f]{64}$"))
  and (.roundtrip_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .source_report_bytes.delivery_receipt_template > 0
  and .source_report_bytes.simulated_delivery_receipt > 0
  and .source_report_bytes.simulated_delivery_audit > 0
  and .roundtrip_markdown_bytes > 0
  and .side_effects.local_simulated_receipt_written == true
  and .side_effects.local_present_branch_report_written == true
  and .side_effects.local_markdown_written == true
  and .side_effects.local_report_written == true
  and .side_effects.backend_agent_spawned == false
  and .side_effects.backend_repo_write == false
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" backend_delivery_roundtrip_report
cat "$REPORT_PATH"
