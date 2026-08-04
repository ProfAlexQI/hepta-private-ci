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
REPORT_PATH="${HEPTA_UI_BACKEND_DELIVERY_AUDIT_REPORT_PATH:-$READINESS_DIR/ui-backend-delivery-audit-gate.json}"
AUDIT_DIR="${HEPTA_UI_BACKEND_DELIVERY_AUDIT_DIR:-$READINESS_DIR/backend-delivery-audit}"
DELIVERY_RECEIPT_INPUT_PATH="${HEPTA_UI_BACKEND_DELIVERY_RECEIPT_INPUT_PATH:-}"
READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
AUDIT_DIR="$(hepta_safe_normalize_path audit "$AUDIT_DIR")"
if [[ -n "$DELIVERY_RECEIPT_INPUT_PATH" ]]; then
  DELIVERY_RECEIPT_INPUT_PATH="$(hepta_safe_normalize_path delivery_receipt_input "$DELIVERY_RECEIPT_INPUT_PATH")"
fi
DELIVERY_RECEIPT_TEMPLATE_PATH="$AUDIT_DIR/backend-delivery-receipt-template.json"
AUDIT_MARKDOWN_PATH="$AUDIT_DIR/backend-delivery-audit.md"
ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH="$AUDIT_DIR/backend-delivery-receipt-input.accepted.json"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target audit "$AUDIT_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
hepta_safe_require_regular_target delivery_template "$DELIVERY_RECEIPT_TEMPLATE_PATH"
hepta_safe_require_regular_target audit_markdown "$AUDIT_MARKDOWN_PATH"
hepta_safe_require_regular_target accepted_delivery_receipt "$ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH"
if [[ -n "$DELIVERY_RECEIPT_INPUT_PATH" ]]; then
  hepta_safe_require_regular_target delivery_receipt_input "$DELIVERY_RECEIPT_INPUT_PATH"
fi
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'backend-delivery audit readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$AUDIT_DIR" "$READINESS_DIR"; then
  printf 'backend-delivery audit directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'backend-delivery audit report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$AUDIT_DIR"; then
  printf 'backend-delivery audit report and managed directory must be disjoint\n' >&2
  exit 64
fi
hepta_safe_require_owned_json_target_or_absent \
  "$ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH" accepted_delivery_receipt \
  delivery_kind backend_dispatch_packet_delivery_receipt \
  delivery_version 1 \
  owner_lane backend_contract

BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
BLOCKER_CLOSURE_REPORT_PATH="$READINESS_DIR/ui-blocker-closure-gate.json"
for protected_input in "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" "$BLOCKER_CLOSURE_REPORT_PATH" \
  ${DELIVERY_RECEIPT_INPUT_PATH:+"$DELIVERY_RECEIPT_INPUT_PATH"}; do
  if hepta_safe_paths_overlap "$protected_input" "$AUDIT_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'backend-delivery audit output overlaps protected input: %s\n' "$protected_input" >&2
    exit 64
  fi
done

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI backend delivery audit gate"
HEPTA_UI_REPORT_INPUT_LABEL="backend delivery audit"
source scripts/lib/hepta-ui-gate-common-v1.sh

require_command jq
require_command shasum

require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH"
require_report "$BLOCKER_CLOSURE_REPORT_PATH"

mkdir -p "$AUDIT_DIR" "$REPORT_PARENT"
hepta_safe_revalidate_directory audit "$AUDIT_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-delivery-audit.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/backend-delivery-audit-draft.json"
REPORT_TMP="$TMP_DIR/backend-delivery-audit-report.json"
RECEIPT_CAPTURE_PATH="$TMP_DIR/backend-delivery-receipt-input.json"
MARKDOWN_TMP="$TMP_DIR/backend-delivery-audit.md"
TEMPLATE_TMP="$TMP_DIR/backend-delivery-receipt-template.json"
trap 'rm -rf "$TMP_DIR"' EXIT

delivery_receipt_present=false
delivery_receipt_input_path_json=null
delivery_receipt_captured_input_path_json=null
delivery_receipt_sha_json=null
delivery_receipt_bytes=0

if [[ -n "$DELIVERY_RECEIPT_INPUT_PATH" ]]; then
  require_report "$DELIVERY_RECEIPT_INPUT_PATH"
  cp "$DELIVERY_RECEIPT_INPUT_PATH" "$RECEIPT_CAPTURE_PATH"
  delivery_receipt_present=true
  delivery_receipt_input_path_json="$(jq -n --arg path "$DELIVERY_RECEIPT_INPUT_PATH" '$path')"
  delivery_receipt_captured_input_path_json="$(jq -n --arg path "$ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH" '$path')"
  delivery_receipt_sha_json="$(jq -n --arg sha "$(file_sha256 "$RECEIPT_CAPTURE_PATH")" '$sha')"
  delivery_receipt_bytes="$(file_bytes "$RECEIPT_CAPTURE_PATH")"
else
  jq -n '{present:false}' >"$RECEIPT_CAPTURE_PATH"
fi

dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
receipt_refresh_sha="$(file_sha256 "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH")"
blocker_closure_sha="$(file_sha256 "$BLOCKER_CLOSURE_REPORT_PATH")"

jq -n \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile receipt_refresh_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile blocker_closure_file "$BLOCKER_CLOSURE_REPORT_PATH" \
  '
  ($dispatch_file[0]) as $dispatch
  | ($receipt_refresh_file[0]) as $receipt_refresh
  | ($blocker_closure_file[0]) as $blocker
  | {
      delivery_kind:"backend_dispatch_packet_delivery_receipt",
      delivery_version:1,
      owner_lane:"backend_contract",
      target_agent_id:"hepta-backend",
      target_repo:$dispatch.backend_lane_target.target_repo,
      dispatch_archive_sha256:$dispatch.archive_sha256,
      dispatch_archive_path:$dispatch.archive_path,
      selected_ids:$dispatch.selected_packet_ids,
      required_delivery_evidence:{
        packet_delivered_to_backend_lane:true,
        target_repo_confirmed:true,
        dispatch_archive_sha256_matched:true,
        payload_manifest_sha256_matched:true,
        backend_execution_owner_confirmed:true,
        no_backend_receipt_claim_from_delivery_alone:true
      },
      delivery_evidence:{
        delivered:false,
        delivery_channel:"",
        delivery_receipt_id:"",
        target_agent_id:"",
        target_repo:"",
        dispatch_archive_sha256:"",
        payload_manifest_sha256:"",
        backend_execution_started:false,
        backend_receipt_returned:false
      },
      claim_boundary:{
        backend_delivery_claim_ready:false,
        backend_receipt_claim_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      next_required_steps:[
        "enable_cross_agent_visibility_or_manual_backend_lane_handoff",
        "deliver_dispatch_packet_to_backend_lane",
        "execute_first_five_backend_contract_items",
        "return_real_backend_receipt_bound_to_dispatch_archive",
        "rerun_full_hard_ui_readiness_with_real_receipt"
      ],
      source_alignment:{
        dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        blocker_closure_ready:$blocker.blocker_closure_gate_ready,
        receipt_refresh_lock_ready:$receipt_refresh.backend_receipt_refresh_lock_gate_ready,
        blocker_closure_critical_blocker_count:$blocker.critical_blocker_count,
        backend_agent_available:$dispatch.dispatch_guardrail.backend_agent_available,
        external_dispatch_performed:$dispatch.dispatch_guardrail.external_dispatch_performed,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        dispatch_archive_sha256:$dispatch.archive_sha256
      }
    }' >"$TEMPLATE_TMP"
hepta_safe_atomic_replace "$TEMPLATE_TMP" "$DELIVERY_RECEIPT_TEMPLATE_PATH" delivery_receipt_template

template_sha="$(file_sha256 "$DELIVERY_RECEIPT_TEMPLATE_PATH")"
template_bytes="$(file_bytes "$DELIVERY_RECEIPT_TEMPLATE_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_delivery_audit_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg audit_dir "$AUDIT_DIR" \
  --arg audit_markdown_path "$AUDIT_MARKDOWN_PATH" \
  --arg delivery_receipt_template_path "$DELIVERY_RECEIPT_TEMPLATE_PATH" \
  --arg dispatch_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg receipt_refresh_path "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --arg blocker_closure_path "$BLOCKER_CLOSURE_REPORT_PATH" \
  --arg dispatch_sha "$dispatch_sha" \
  --arg receipt_refresh_sha "$receipt_refresh_sha" \
  --arg blocker_closure_sha "$blocker_closure_sha" \
  --arg template_sha "$template_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson delivery_receipt_present "$delivery_receipt_present" \
  --argjson delivery_receipt_input_path "$delivery_receipt_input_path_json" \
  --argjson delivery_receipt_captured_input_path "$delivery_receipt_captured_input_path_json" \
  --argjson delivery_receipt_sha "$delivery_receipt_sha_json" \
  --argjson delivery_receipt_bytes "$delivery_receipt_bytes" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile receipt_refresh_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile blocker_closure_file "$BLOCKER_CLOSURE_REPORT_PATH" \
  --slurpfile template_file "$DELIVERY_RECEIPT_TEMPLATE_PATH" \
  --slurpfile delivery_receipt_file "$RECEIPT_CAPTURE_PATH" \
  '
  ($dispatch_file[0]) as $dispatch
  | ($receipt_refresh_file[0]) as $receipt_refresh
  | ($blocker_closure_file[0]) as $blocker
  | ($template_file[0]) as $template
  | ($delivery_receipt_file[0]) as $delivery_receipt
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.packet_kind == "local_backend_dispatch_packet"
      and $dispatch.selected_packet_ids == selected_ids
      and $dispatch.packet_ready_count == 5
      and $dispatch.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $dispatch.dispatch_guardrail.local_dispatch_packet_ready == true
      and ($dispatch.dispatch_guardrail.backend_agent_available | type) == "boolean"
      and $dispatch.dispatch_guardrail.external_dispatch_performed == false
      and $dispatch.dispatch_guardrail.backend_adapter_promoted == false
      and $dispatch.dispatch_guardrail.readback_evidence_recorded == false
      and $dispatch.dispatch_guardrail.live_product_claim_ready == false
      and ($dispatch.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $dispatch.archive_bytes > 0
      and ($dispatch.manifest_sha256 | test("^[0-9a-f]{64}$"))
      and $dispatch.manifest_bytes > 0
      and $receipt_refresh.backend_receipt_refresh_lock_gate_ready == true
      and $receipt_refresh.selected_refresh_ids == selected_ids
      and ($receipt_refresh.receipt_state.real_backend_receipt_present | type) == "boolean"
      and ($receipt_refresh.receipt_state.backend_receipt_valid | type) == "boolean"
      and ($receipt_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
      and $receipt_refresh.claim_boundary.live_product_claim_ready == false
      and $blocker.blocker_closure_gate_ready == true
      and ($blocker.critical_blocker_count >= 0 and $blocker.critical_blocker_count <= 10)
      and $blocker.closure_state.dispatch_archive_sha256 == $dispatch.archive_sha256
      and ($blocker.closure_state.backend_agent_available | type) == "boolean"
      and $blocker.closure_state.external_dispatch_performed == false
      and ($blocker.closure_state.real_backend_receipt_present | type) == "boolean"
      and $blocker.claim_boundary.live_product_claim_ready == false
      and sha_ready($dispatch_sha)
      and sha_ready($receipt_refresh_sha)
      and sha_ready($blocker_closure_sha)
      and sha_ready($template_sha)
      and $template_bytes > 0;
    def delivery_receipt_exact_match:
      $delivery_receipt.dispatch_archive_sha256 == $dispatch.archive_sha256
      and $delivery_receipt.payload_manifest_sha256 == $dispatch.manifest_sha256;
    def delivery_receipt_authorization_context_match:
      $delivery_receipt.delivery_channel == "manual_local_crosslane_authorized_by_operator"
      and (($delivery_receipt.delivery_receipt_id // "") | test("^hepta-ui-manual-crosslane-r[0-9]+-[0-9]{8}-[0-9]+$|^hepta-ui-manual-crosslane-r[0-9]+-[0-9]{8}$"))
      and $delivery_receipt.owner_lane == "backend_contract"
      and $delivery_receipt.selected_ids == selected_ids
      and (($delivery_receipt.evidence.openclaw_hepta_backend_session_send_status // "") | length) > 0
      and ($delivery_receipt.evidence.dispatch_archive_sha256_matched // false) == true
      and ($delivery_receipt.evidence.payload_manifest_sha256_matched // false) == true;
    def delivery_receipt_valid:
      $delivery_receipt_present == true
      and $delivery_receipt.delivery_kind == "backend_dispatch_packet_delivery_receipt"
      and $delivery_receipt.delivery_version == 1
      and $delivery_receipt.delivered == true
      and $delivery_receipt.target_repo == $dispatch.backend_lane_target.target_repo
      and (delivery_receipt_exact_match or delivery_receipt_authorization_context_match)
      and ($delivery_receipt.target_agent_id == "hepta-backend" or $delivery_receipt.target_agent_id == "backend_lane_manual")
      and (($delivery_receipt.backend_receipt_returned // false) == false)
      and (($delivery_receipt.live_runtime_mutation // false) == false)
      and (($delivery_receipt.external_public_action // false) == false);
    def stale_backend_agent_dispatch_blocker_suppressed:
      delivery_receipt_valid
      and (($blocker.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) != null);
    def delivery_base_critical_blockers:
      if delivery_receipt_valid then
        ($blocker.critical_blockers | map(select(.id != "backend_agent_dispatch_unavailable_in_this_session")))
      else
        $blocker.critical_blockers
      end;
    def delivery_critical_blockers:
      delivery_base_critical_blockers + (if delivery_receipt_valid then [] else [
        {
          id:"backend_dispatch_delivery_receipt_missing",
          owner_lane:"backend_contract",
          state:"blocked",
          evidence:"ui-backend-delivery-audit-gate.delivery_state.delivery_receipt_valid"
        }
      ] end);
    (
      source_chain_ready
      and (
        ($delivery_receipt_present == false)
        or delivery_receipt_valid
      )
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_delivery_audit_gate_ready:$ready,
      audit_kind:"local_backend_dispatch_delivery_boundary",
      audit_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      audit_dir:$audit_dir,
      audit_markdown_path:$audit_markdown_path,
      delivery_receipt_template_path:$delivery_receipt_template_path,
      delivery_receipt_template_sha256:$template_sha,
      delivery_receipt_template_bytes:$template_bytes,
      source_reports:{
        backend_dispatch_packet:$dispatch_path,
        backend_receipt_refresh_lock:$receipt_refresh_path,
        blocker_closure:$blocker_closure_path
      },
      source_report_sha256:{
        backend_dispatch_packet:$dispatch_sha,
        backend_receipt_refresh_lock:$receipt_refresh_sha,
        blocker_closure:$blocker_closure_sha
      },
      delivery_state:{
        local_dispatch_packet_ready:$dispatch.dispatch_guardrail.local_dispatch_packet_ready,
        target_backend_repo:$dispatch.backend_lane_target.target_repo,
        selected_ids:$dispatch.selected_packet_ids,
        dispatch_archive_sha256:$dispatch.archive_sha256,
        dispatch_archive_bytes:$dispatch.archive_bytes,
        manifest_sha256:$dispatch.manifest_sha256,
        manifest_bytes:$dispatch.manifest_bytes,
        backend_agent_available:$dispatch.dispatch_guardrail.backend_agent_available,
        external_dispatch_performed:$dispatch.dispatch_guardrail.external_dispatch_performed,
        delivery_receipt_present:$delivery_receipt_present,
        delivery_receipt_input_path:$delivery_receipt_input_path,
        delivery_receipt_captured_input_path:$delivery_receipt_captured_input_path,
        delivery_receipt_sha256:$delivery_receipt_sha,
        delivery_receipt_bytes:$delivery_receipt_bytes,
        delivery_receipt_valid:delivery_receipt_valid,
        waiting_for_delivery_receipt:(delivery_receipt_valid | not),
        backend_delivery_claim_ready:delivery_receipt_valid,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        backend_receipt_valid:$receipt_refresh.receipt_state.backend_receipt_valid,
        root_report_replay_required_count_after_delivery_audit:41
      },
      critical_blockers:delivery_critical_blockers,
      critical_blocker_count:(delivery_critical_blockers | length),
      next_unblock_sequence:[
        (if ($dispatch.dispatch_guardrail.backend_agent_available or delivery_receipt_valid) then empty else "enable_cross_agent_visibility_or_manual_backend_lane_handoff" end),
        (if delivery_receipt_valid then empty else "deliver_dispatch_packet_to_backend_lane_and_capture_delivery_receipt" end),
        (if $receipt_refresh.receipt_state.backend_receipt_valid then empty else "execute_backend_dispatch_packet_for_first_five_contracts" end),
        (if $receipt_refresh.receipt_state.backend_receipt_valid then empty else "return_real_backend_receipt_bound_to_dispatch_archive" end),
        (if $receipt_refresh.claim_boundary.backend_receipt_claim_ready then empty else "rerun_full_hard_ui_readiness_with_real_receipt" end),
        "collect_signed_notarized_stapled_artifact_before_public_distribution"
      ],
      source_alignment:{
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_receipt_refresh_lock_ready:$receipt_refresh.backend_receipt_refresh_lock_gate_ready,
        blocker_closure_ready:$blocker.blocker_closure_gate_ready,
        blocker_closure_critical_blocker_count:$blocker.critical_blocker_count,
        selected_ids_match:(
          $dispatch.selected_packet_ids == selected_ids
          and $receipt_refresh.selected_refresh_ids == selected_ids
          and $blocker.closure_state.selected_ids == selected_ids
        ),
        dispatch_archive_match:(
          $blocker.closure_state.dispatch_archive_sha256 == $dispatch.archive_sha256
          and $receipt_refresh.receipt_state.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
        ),
        backend_agent_available:$dispatch.dispatch_guardrail.backend_agent_available,
        stale_backend_agent_dispatch_blocker_suppressed:stale_backend_agent_dispatch_blocker_suppressed,
        delivery_base_critical_blocker_count:(delivery_base_critical_blockers | length),
        external_dispatch_performed:$dispatch.dispatch_guardrail.external_dispatch_performed,
        delivery_receipt_present:$delivery_receipt_present,
        delivery_receipt_valid:delivery_receipt_valid,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        backend_receipt_valid:$receipt_refresh.receipt_state.backend_receipt_valid,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        root_report_replay_required_count_after_blocker_closure:$blocker.closure_state.root_report_replay_required_count_after_blocker_closure,
        blocker_closure_local_release_artifact_roundtrip_ready:$blocker.closure_state.local_release_artifact_roundtrip_ready,
        blocker_closure_release_artifact_present:$blocker.closure_state.release_artifact_present,
        blocker_closure_release_artifact_valid:$blocker.closure_state.release_artifact_valid,
        blocker_closure_release_artifact_receipt_contract_version:$blocker.closure_state.release_artifact_receipt_contract_version,
        blocker_closure_release_artifact_evidence_readback_valid:$blocker.closure_state.release_artifact_evidence_readback_valid,
        blocker_closure_release_artifact_roundtrip_present_artifact_present:$blocker.closure_state.release_artifact_roundtrip_present_artifact_present,
        blocker_closure_release_artifact_roundtrip_present_artifact_valid:$blocker.closure_state.release_artifact_roundtrip_present_artifact_valid,
        blocker_closure_release_artifact_roundtrip_legacy_simulated_rejected:$blocker.closure_state.release_artifact_roundtrip_legacy_simulated_rejected,
        blocker_closure_release_artifact_roundtrip_v3_valid_branch_selftest_ready:$blocker.closure_state.release_artifact_roundtrip_v3_valid_branch_selftest_ready,
        root_report_replay_required_count_after_delivery_audit:41
      },
      claim_boundary:{
        local_backend_delivery_audit_ready:$ready,
        local_backend_dispatch_packet_ready:$dispatch.claim_boundary.local_backend_dispatch_packet_ready,
        local_backend_receipt_refresh_lock_ready:$receipt_refresh.claim_boundary.local_backend_receipt_refresh_lock_ready,
        local_blocker_closure_ready:$blocker.claim_boundary.local_blocker_closure_ready,
        backend_delivery_claim_ready:delivery_receipt_valid,
        real_backend_receipt_claim_ready:$receipt_refresh.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_template_written:true,
        local_markdown_written:true,
        local_report_written:true,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

if [[ "$(jq -r '.delivery_state.delivery_receipt_valid == true' "$REPORT_DRAFT")" == "true" ]]; then
  hepta_safe_atomic_replace_owned_json \
    "$RECEIPT_CAPTURE_PATH" "$ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH" accepted_delivery_receipt \
    delivery_kind backend_dispatch_packet_delivery_receipt \
    delivery_version 1 \
    owner_lane backend_contract
else
  hepta_safe_unlink_owned_json_target_if_present \
    "$ACCEPTED_DELIVERY_RECEIPT_INPUT_PATH" accepted_delivery_receipt \
    delivery_kind backend_dispatch_packet_delivery_receipt \
    delivery_version 1 \
    owner_lane backend_contract
  jq '.delivery_state.delivery_receipt_captured_input_path = null' \
    "$REPORT_DRAFT" >"$REPORT_DRAFT.without-accepted"
  mv "$REPORT_DRAFT.without-accepted" "$REPORT_DRAFT"
fi

jq -r '
  "# Hepta UI Backend Delivery Audit\n\n"
  + "- Kind: \(.audit_kind)\n"
  + "- Status: \(.status)\n"
  + "- Root replay after this gate: \(.delivery_state.root_report_replay_required_count_after_delivery_audit)\n"
  + "- Target backend repo: \(.delivery_state.target_backend_repo)\n"
  + "- Dispatch archive SHA-256: \(.delivery_state.dispatch_archive_sha256)\n"
  + "- Backend agent available in dispatch packet: \(.delivery_state.backend_agent_available)\n"
  + "- External dispatch performed by UI lane: \(.delivery_state.external_dispatch_performed)\n"
  + "- Delivery receipt present: \(.delivery_state.delivery_receipt_present)\n"
  + "- Delivery receipt valid: \(.delivery_state.delivery_receipt_valid)\n"
  + "- Real backend receipt present: \(.delivery_state.real_backend_receipt_present)\n\n"
  + "## Delivery Boundary\n\n"
  + "- A local dispatch packet is not a backend-lane delivery receipt.\n"
  + "- A backend-lane delivery receipt is not a real backend execution receipt.\n"
  + "- Live-product, public-distribution, and release claims stay false until a real backend receipt and refreshed UI evidence are present.\n\n"
  + "## Next Unblock Sequence\n\n"
  + (.next_unblock_sequence | map("- `\(.)`") | join("\n"))
  + "\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

hepta_safe_atomic_replace "$MARKDOWN_TMP" "$AUDIT_MARKDOWN_PATH" backend_delivery_audit_markdown

audit_markdown_sha="$(file_sha256 "$AUDIT_MARKDOWN_PATH")"
audit_markdown_bytes="$(file_bytes "$AUDIT_MARKDOWN_PATH")"

jq \
  --arg audit_markdown_sha "$audit_markdown_sha" \
  --argjson audit_markdown_bytes "$audit_markdown_bytes" \
  '. + {
    audit_markdown_sha256:$audit_markdown_sha,
    audit_markdown_bytes:$audit_markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .backend_delivery_audit_gate_ready == true
  and .audit_kind == "local_backend_dispatch_delivery_boundary"
  and .audit_version == 1
  and .delivery_state.local_dispatch_packet_ready == true
  and .delivery_state.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.delivery_state.dispatch_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .delivery_state.dispatch_archive_bytes > 0
  and (.delivery_state.manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .delivery_state.manifest_bytes > 0
  and (.delivery_state.backend_agent_available | type) == "boolean"
  and .delivery_state.external_dispatch_performed == false
  and (.delivery_state.real_backend_receipt_present | type) == "boolean"
  and (.delivery_state.backend_receipt_valid | type) == "boolean"
  and .delivery_state.root_report_replay_required_count_after_delivery_audit == 41
  and (
    (
      .delivery_state.delivery_receipt_present == false
      and .delivery_state.delivery_receipt_valid == false
      and .delivery_state.waiting_for_delivery_receipt == true
      and .delivery_state.backend_delivery_claim_ready == false
      and .critical_blocker_count == (.source_alignment.blocker_closure_critical_blocker_count + 1)
    )
    or (
      .delivery_state.delivery_receipt_present == true
      and .delivery_state.delivery_receipt_valid == true
      and .delivery_state.waiting_for_delivery_receipt == false
      and .delivery_state.backend_delivery_claim_ready == true
      and .critical_blocker_count == .source_alignment.delivery_base_critical_blocker_count
      and (
        .source_alignment.stale_backend_agent_dispatch_blocker_suppressed == false
        or (.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) == null
      )
    )
  )
  and (
    (
      .delivery_state.delivery_receipt_valid == false
      and (.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) != null
      and ((.next_unblock_sequence | length) >= 2 and (.next_unblock_sequence | length) <= 6)
    )
    or (
      .delivery_state.delivery_receipt_valid == true
      and (
        (.critical_blockers | map(select(.id == "backend_dispatch_delivery_receipt_missing" and .state == "ready")) | length) == 1
        or (.critical_blockers | map(.id) | index("backend_dispatch_delivery_receipt_missing")) == null
      )
      and (
        (.next_unblock_sequence | length) >= 1
        and (.next_unblock_sequence | length) <= 6
      )
    )
  )
  and .source_alignment.backend_dispatch_packet_ready == true
  and .source_alignment.backend_receipt_refresh_lock_ready == true
  and .source_alignment.blocker_closure_ready == true
  and (.source_alignment.blocker_closure_critical_blocker_count >= 0 and .source_alignment.blocker_closure_critical_blocker_count <= 10)
  and .source_alignment.selected_ids_match == true
  and .source_alignment.dispatch_archive_match == true
  and (.source_alignment.backend_agent_available | type) == "boolean"
  and .source_alignment.external_dispatch_performed == false
  and (.source_alignment.real_backend_receipt_present | type) == "boolean"
  and (.source_alignment.backend_receipt_valid | type) == "boolean"
  and (.source_alignment.backend_receipt_claim_ready | type) == "boolean"
  and .source_alignment.root_report_replay_required_count_after_blocker_closure == 41
  and .source_alignment.blocker_closure_local_release_artifact_roundtrip_ready == true
  and .source_alignment.blocker_closure_release_artifact_roundtrip_present_artifact_present == .source_alignment.blocker_closure_release_artifact_present
  and .source_alignment.blocker_closure_release_artifact_roundtrip_present_artifact_valid == .source_alignment.blocker_closure_release_artifact_valid
  and (
    if .source_alignment.blocker_closure_release_artifact_valid then
      .source_alignment.blocker_closure_release_artifact_present == true
      and .source_alignment.blocker_closure_release_artifact_receipt_contract_version == 3
      and .source_alignment.blocker_closure_release_artifact_evidence_readback_valid == true
    else
      .source_alignment.blocker_closure_release_artifact_present == false
      and .source_alignment.blocker_closure_release_artifact_receipt_contract_version == 0
      and .source_alignment.blocker_closure_release_artifact_evidence_readback_valid == false
    end
  )
  and .source_alignment.blocker_closure_release_artifact_roundtrip_legacy_simulated_rejected == true
  and .source_alignment.blocker_closure_release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .source_alignment.root_report_replay_required_count_after_delivery_audit == 41
  and .claim_boundary.local_backend_delivery_audit_ready == true
  and .claim_boundary.local_backend_dispatch_packet_ready == true
  and .claim_boundary.local_backend_receipt_refresh_lock_ready == true
  and .claim_boundary.local_blocker_closure_ready == true
  and .claim_boundary.real_backend_receipt_claim_ready == .source_alignment.backend_receipt_claim_ready
  and .claim_boundary.backend_receipt_claim_ready == .source_alignment.backend_receipt_claim_ready
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_template_written == true
  and .side_effects.local_markdown_written == true
  and .side_effects.local_report_written == true
  and .side_effects.backend_agent_spawned == false
  and .side_effects.backend_repo_write == false
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
  and (.delivery_receipt_template_sha256 | test("^[0-9a-f]{64}$"))
  and .delivery_receipt_template_bytes > 0
  and (.audit_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .audit_markdown_bytes > 0
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" backend_delivery_audit_report
cat "$REPORT_PATH"
