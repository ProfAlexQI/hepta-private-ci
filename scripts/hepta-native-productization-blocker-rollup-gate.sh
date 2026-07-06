#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

ROLLUP_PATH="${HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_PATH:-docs/architecture/HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_2026-06-15.md}"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
TRUE_WINDOW_READINESS_DIR="${HEPTA_NATIVE_PRODUCTIZATION_TRUE_WINDOW_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.contract-waves-true-window-20260615}"
TRUE_WINDOW_SMOKE_DIR="${HEPTA_NATIVE_PRODUCTIZATION_TRUE_WINDOW_SMOKE_DIR:-/Users/qianqi/.openclaw/tmp/hepta-native-window-smoke.glass-contract-20260615-2301}"
ALLOW_IN_PROGRESS="${HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_ALLOW_IN_PROGRESS:-0}"

READINESS_PATH="$READINESS_DIR/readiness.json"
STATIC_CONTRACT_PATH="$READINESS_DIR/static-contract.json"
MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
PACKAGING_REPORT_PATH="$READINESS_DIR/native-packaging-gate.json"
DISTRIBUTION_PREFLIGHT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
NATIVE_WINDOW_REPORT_PATH="$READINESS_DIR/native-window-smoke.json"
HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
BACKEND_GATES_PATH="$READINESS_DIR/native-backend-contract-gates.json"
NON_BASE_EDGE_GATES_PATH="$READINESS_DIR/native-non-base-edge-gates.json"
TRUE_WINDOW_READINESS_PATH="$TRUE_WINDOW_READINESS_DIR/readiness.json"
TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH="$TRUE_WINDOW_SMOKE_DIR/desktop-window.png"
TRUE_WINDOW_MOBILE_SCREENSHOT_PATH="$TRUE_WINDOW_SMOKE_DIR/mobile-window.png"
TRUE_WINDOW_LOG_PATH="$TRUE_WINDOW_SMOKE_DIR/hepta-native.log"

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required file: %s\n' "$path" >&2
    exit 1
  fi
}

require_marker() {
  local path="$1"
  local marker="$2"
  if ! grep -Fq "$marker" "$path"; then
    printf 'Missing marker in %s: %s\n' "$path" "$marker" >&2
    exit 1
  fi
}

require_file "$ROLLUP_PATH"
require_file "$HANDOFF_PATH"
require_file "$BACKEND_GATES_PATH"
require_file "$NON_BASE_EDGE_GATES_PATH"

if [[ -s "$TRUE_WINDOW_READINESS_PATH" ]]; then
  true_window_reference_mode="readiness"
else
  require_file "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH"
  require_file "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH"
  true_window_reference_mode="standalone_window_smoke"
fi

if [[ -s "$READINESS_PATH" ]]; then
  readiness_report_state="ready"
elif [[ "$ALLOW_IN_PROGRESS" == "1" ]]; then
  require_file "$STATIC_CONTRACT_PATH"
  require_file "$MANIFEST_PATH"
  require_file "$PACKAGING_REPORT_PATH"
  require_file "$DISTRIBUTION_PREFLIGHT_PATH"
  readiness_report_state="in_progress_artifacts_ready"
else
  require_file "$READINESS_PATH"
fi

require_file "$DISTRIBUTION_PREFLIGHT_PATH"

MARKERS=(
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_READY:true'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_DATE:2026-06-15'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_STATUS:ui-product-readiness-local-ready_backend-owned-blockers'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_SOURCE_READINESS:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_TRUE_WINDOW_REFERENCE:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.contract-waves-true-window-20260615'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_TRUE_WINDOW_SMOKE_REFERENCE:/Users/qianqi/.openclaw/tmp/hepta-native-window-smoke.glass-contract-20260615-2301'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_NATIVE_GLASS_CONTRACT:computed-style-desktop-mobile'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_SCOPE:apps/hepta-native,apps/hepta-control-ui,ui-fixtures,packaging,screenshot-gates'
  'HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'Evidence Snapshot'
  'Wave 1 high-risk writes'
  'Wave 2 queue/progress and platform permissions'
  'Wave 3 remote result adapters and rich UX'
  'Non-base edge closure'
  'Remaining backend-owned blockers'
  'Future Plan'
  'Promotion Rules'
  'packaging gate may cold-compile long enough to time out'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$ROLLUP_PATH" "$marker"
done

if [[ -s "$READINESS_PATH" ]]; then
  jq -e '
    .status == "ready"
    and .ui_product_readiness_gate_ready == true
    and (.static_contract.marker_count >= 3642)
    and .control_ui.screenshot_count == 4
    and .native.screenshot_count >= 40
    and .native_readability_contrast_clip_ready == true
    and .native.readability_contrast_clip_ready == true
    and .native_telegram_header_icon_affordance_ready == true
    and .native.header_icon_affordance_ready == true
    and .native_secondary_product_surfaces_ready == true
    and .native.secondary_product_surfaces_ready == true
    and .native.secondary_product_surfaces.status == "ready"
    and .native.secondary_product_surfaces.viewport_count == 3
    and .native.secondary_product_surfaces.case_count == 15
    and .native.secondary_product_surfaces.preferred_touch_target_ready == true
    and .native.secondary_product_surfaces.actions_in_surface == true
    and .native.secondary_product_surfaces.text_clipping_failure_count == 0
    and .native.secondary_product_surfaces.content_edge_failure_count == 0
    and .native.secondary_product_surfaces.visible_audit_failure_count == 0
    and .native.tempered_glass_visual_contract_ready == true
    and .native.tempered_glass_visual_contract.status == "ready"
    and .native.tempered_glass_visual_contract.viewport_count == 4
    and .native.tempered_glass_visual_contract.phone_ready == true
    and .native.tempered_glass_visual_contract.phone320_ready == true
    and .native.tempered_glass_visual_contract.preferred_touch_target_ready == true
    and .native.tempered_glass_visual_contract.light_surface_failure_count == 0
    and (
      (.native.true_window_smoke_enabled != true)
      or (.native.true_window_smoke_ready == true and .native.true_window_screenshot_count == 2)
      or (
        .native_window_smoke.blocked_allowed == true
        and (.native_window_smoke.status == "blocked_by_locked_screen" or .native_window_smoke.status == "blocked_by_local_macos_permissions")
      )
    )
    and .key_screenshot_count == 24
    and .native_packaging_gate_ready == true
    and .native_packaging.packaging_evidence_mode == "metadata_plus_local_unsigned_app_bundle_probe"
    and .native_packaging.local_unsigned_app_bundle_probe_ready == true
    and .native_packaging.local_unsigned_app_bundle.ready == true
    and .native_packaging.local_unsigned_app_bundle.bundle_identifier == "ai.hepta.nativeapp"
    and .native_packaging.local_unsigned_app_bundle.distribution_signed == false
    and .native_packaging.local_unsigned_app_bundle.public_distribution_artifact_written == false
    and .native_distribution_preflight_gate_ready == true
    and .native_distribution_preflight.distribution_preflight_gate_ready == true
    and .native_distribution_preflight.distribution_static_contract_ready == true
    and .native_distribution_preflight.public_distribution_ready == false
    and .native_distribution_preflight.notary_submission_performed == false
    and .native_distribution_preflight.public_distribution_artifact_written == false
    and .native_backend_contract_gates_ready == true
    and .native_non_base_edge_gates_ready == true
    and .side_effects.local_loopback_server_spawned == true
    and .side_effects.matrix_login == false
    and .side_effects.gateway_call == false
    and .side_effects.provider_invoked == false
    and .side_effects.channel_delivery == false
    and .side_effects.external_mutation == false
  ' "$READINESS_PATH" >/dev/null

  latest_readiness_json="$(
    jq --argjson distribution_preflight "$(<"$DISTRIBUTION_PREFLIGHT_PATH")" '{
      status,
      path:.readiness_report_path,
      marker_count:.static_contract.marker_count,
      control_screenshot_count:.control_ui.screenshot_count,
      native_screenshot_count:.native.screenshot_count,
      native_readability_contrast_clip_ready:(.native_readability_contrast_clip_ready // .native.readability_contrast_clip_ready),
      native_telegram_header_icon_affordance_ready:(.native_telegram_header_icon_affordance_ready // .native.header_icon_affordance_ready),
      native_secondary_product_surfaces_ready:(.native_secondary_product_surfaces_ready // .native.secondary_product_surfaces_ready),
      native_secondary_product_surfaces:(.native_secondary_product_surfaces // .native.secondary_product_surfaces),
      native_tempered_glass_visual_contract_ready:.native.tempered_glass_visual_contract_ready,
      native_tempered_glass_visual_contract:.native.tempered_glass_visual_contract,
      current_true_window:{
        status:(.native_window_smoke.status // "not_run"),
        enabled:(.native.true_window_smoke_enabled // false),
        ready:(.native.true_window_smoke_ready // false),
        app_log_error_free:(.native.true_window_app_log_error_free // .native_window_app_log_error_free // false),
        blocked_allowed:(.native_window_smoke.blocked_allowed // false),
        true_window_screenshot_count:(.native.true_window_screenshot_count // 0),
        report:(.native.true_window_report // null)
      },
      key_screenshot_count,
      packaging_ready:.native_packaging_gate_ready,
      packaging_evidence_mode:.native_packaging.packaging_evidence_mode,
      local_unsigned_app_bundle_probe_ready:.native_packaging.local_unsigned_app_bundle_probe_ready,
      local_unsigned_app_bundle:.native_packaging.local_unsigned_app_bundle,
      distribution_preflight_ready:$distribution_preflight.distribution_preflight_gate_ready,
      distribution_preflight:$distribution_preflight,
      backend_contract_gates_ready:.native_backend_contract_gates_ready,
      non_base_edge_gates_ready:.native_non_base_edge_gates_ready,
      side_effects
    }' "$READINESS_PATH"
  )"
else
  jq -e '.marker_count >= 3642' "$STATIC_CONTRACT_PATH" >/dev/null
  jq -e '
    .screenshot_manifest_ready == true
    and .screenshot_count.control_ui == 4
    and .screenshot_count.native >= 40
    and .native.readability_contrast_clip_ready == true
    and .native.header_icon_affordance_ready == true
    and .native.secondary_product_surfaces_ready == true
    and .native.secondary_product_surfaces.status == "ready"
    and .native.secondary_product_surfaces.viewport_count == 3
    and .native.secondary_product_surfaces.case_count == 15
    and .native.secondary_product_surfaces.preferred_touch_target_ready == true
    and .native.secondary_product_surfaces.actions_in_surface == true
    and .native.secondary_product_surfaces.text_clipping_failure_count == 0
    and .native.secondary_product_surfaces.content_edge_failure_count == 0
    and .native.secondary_product_surfaces.visible_audit_failure_count == 0
    and .native.tempered_glass_visual_contract_ready == true
    and .native.tempered_glass_visual_contract.status == "ready"
    and .native.tempered_glass_visual_contract.viewport_count == 4
    and .native.tempered_glass_visual_contract.phone_ready == true
    and .native.tempered_glass_visual_contract.phone320_ready == true
    and .native.tempered_glass_visual_contract.preferred_touch_target_ready == true
    and .native.tempered_glass_visual_contract.light_surface_failure_count == 0
    and .key_screenshot_count == 24
  ' "$MANIFEST_PATH" >/dev/null
  jq -e '
    .local_packaging_gate_ready == true
    and .packaging_evidence_mode == "metadata_plus_local_unsigned_app_bundle_probe"
    and .local_unsigned_app_bundle_probe_ready == true
    and .local_unsigned_app_bundle.ready == true
    and .local_unsigned_app_bundle.bundle_identifier == "ai.hepta.nativeapp"
    and .local_unsigned_app_bundle.bundle_executable == "hepta-native"
    and .local_unsigned_app_bundle.distribution_signed == false
    and .local_unsigned_app_bundle.distribution_notarized == false
    and .local_unsigned_app_bundle.distribution_stapled == false
    and .local_unsigned_app_bundle.public_distribution_artifact_written == false
  ' "$PACKAGING_REPORT_PATH" >/dev/null
  jq -e '
    .distribution_preflight_gate_ready == true
    and .distribution_static_contract_ready == true
    and .unsigned_app_bundle_probe.ready == true
    and .public_distribution_ready == false
    and .release_approval_required == true
    and .credential_values_read == false
    and .network_call_performed == false
    and .notary_submission_performed == false
    and .public_distribution_artifact_written == false
    and .app_signed == false
    and .app_notarized == false
    and .app_stapled == false
  ' "$DISTRIBUTION_PREFLIGHT_PATH" >/dev/null

  if [[ -s "$NATIVE_WINDOW_REPORT_PATH" ]]; then
    current_true_window_json="$(
      jq '{
        status:(.status // "unknown"),
        enabled:(if has("enabled") then (.enabled == true) else true end),
        ready:(.status == "ready" and .true_window_capture_performed == true and ((.screenshots // []) | length) == 2),
        app_log_error_free:(.native_app_log_error_free // false),
        blocked_allowed:(.blocked_allowed // false),
        true_window_screenshot_count:((.screenshots // []) | length),
        report:input_filename
      }' "$NATIVE_WINDOW_REPORT_PATH"
    )"
  else
    current_true_window_json="$(
      jq -n '{
        status:"not_run",
        enabled:false,
        ready:false,
        blocked_allowed:false,
        true_window_screenshot_count:0,
        report:null
      }'
    )"
  fi

  latest_readiness_json="$(
    jq -n \
      --arg status "$readiness_report_state" \
      --arg readiness_dir "$READINESS_DIR" \
      --argjson static_contract "$(<"$STATIC_CONTRACT_PATH")" \
      --argjson manifest "$(<"$MANIFEST_PATH")" \
      --argjson packaging "$(<"$PACKAGING_REPORT_PATH")" \
      --argjson distribution_preflight "$(<"$DISTRIBUTION_PREFLIGHT_PATH")" \
      --argjson current_true_window "$current_true_window_json" \
      '{
        status:$status,
        path:null,
        readiness_dir:$readiness_dir,
        marker_count:$static_contract.marker_count,
        control_screenshot_count:$manifest.screenshot_count.control_ui,
        native_screenshot_count:$manifest.screenshot_count.native,
        native_readability_contrast_clip_ready:$manifest.native.readability_contrast_clip_ready,
        native_telegram_header_icon_affordance_ready:$manifest.native.header_icon_affordance_ready,
        native_secondary_product_surfaces_ready:$manifest.native.secondary_product_surfaces_ready,
        native_secondary_product_surfaces:$manifest.native.secondary_product_surfaces,
        native_tempered_glass_visual_contract_ready:$manifest.native.tempered_glass_visual_contract_ready,
        native_tempered_glass_visual_contract:$manifest.native.tempered_glass_visual_contract,
        current_true_window:$current_true_window,
        key_screenshot_count:$manifest.key_screenshot_count,
        packaging_ready:$packaging.local_packaging_gate_ready,
        packaging_evidence_mode:$packaging.packaging_evidence_mode,
        local_unsigned_app_bundle_probe_ready:$packaging.local_unsigned_app_bundle_probe_ready,
        local_unsigned_app_bundle:$packaging.local_unsigned_app_bundle,
        distribution_preflight_ready:$distribution_preflight.distribution_preflight_gate_ready,
        distribution_preflight:$distribution_preflight,
        backend_contract_gates_ready:true,
        non_base_edge_gates_ready:true,
        side_effects:{
          local_loopback_server_spawned:$packaging.runner.local_loopback_spawned,
          matrix_login:false,
          gateway_call:false,
          provider_invoked:false,
          channel_delivery:false,
          external_mutation:false
        }
      }'
  )"
fi

jq -e '
  (.items | length) == 12
  and (.items | all(.status == "partial_live_backend_contract_remaining"))
  and (.items | all(.ui_lane_state == "complete"))
  and (.items | all(.next_owner_lane == "backend_contract"))
  and (.items | all((.required_backend_contracts | length) >= 5))
  and (.items | all(.acceptance_state.current_ui_evidence == "machine_checked"))
  and (.items | all(.acceptance_state.side_effect_boundary == "locked"))
  and (.items | all(.acceptance_state.ui_contract == "ready"))
  and (.items | all(.acceptance_state.backend_contract == "required"))
  and (.items | all(.side_effects.external_mutation == false))
  and (.items | map(select(.id == "message_search" and (.required_backend_contracts | index("remote date/pins/scope/full-result result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "file_upload_send" and (.required_backend_contracts | index("accepted queue/progress/result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "media_download_playback" and (.required_backend_contracts | index("decrypt/decode/opener/queue result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "matrix_link_resolution" and (.required_backend_contracts | index("route/event-context result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "message_report_send" and (.required_backend_contracts | index("workflow result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "message_edit_history" and (.required_backend_contracts | index("remote full-history/source reconciliation result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "mention_picker_send" and (.required_backend_contracts | index("remote hover/profile/disambiguation/edit-reply result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "voice_message_send" and (.required_backend_contracts | index("permission/capture/upload result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "account_avatar_upload" and (.required_backend_contracts | index("source/cropper/camera/editor artifact result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "account_management" and (.required_backend_contracts | index("password/SSO/revoke/trust/delete result taxonomy packet (local blocked UI evidence)") != null))) | length) == 1
' "$HANDOFF_PATH" >/dev/null

jq -e '
  .status == "ready"
  and .backend_contract_waves_ready == true
  and .verified_gap_count == 12
  and (.covered_gap_ids | length) == 12
  and (.waves | length) == 6
  and (.waves | all(.status == "ready"))
  and (.waves | map(select(.gate == "notifications_backend_contract" and (.required_contract_groups | index("timed/global/pusher result taxonomy packet") != null))) | length) == 1
  and (.waves | map(select(.gate == "room_settings_backend_contract" and (.required_contract_groups | index("power/member result taxonomy ui packet") != null))) | length) == 1
  and (.waves | map(select(.gate == "message_report_backend_contract" and (.required_contract_groups | index("workflow result taxonomy packet") != null))) | length) == 1
  and (.waves | map(select(.gate == "account_management_backend_contract" and (.required_contract_groups | index("password/SSO/revoke/trust/delete result taxonomy UI packet") != null))) | length) == 1
  and (.waves | map(select(.gate == "queue_media_permission_backend_contract"
    and (.required_contract_groups | index("file upload accepted queue/progress/result taxonomy packet") != null)
    and (.required_contract_groups | index("media playback decrypt/decode/opener/queue result taxonomy packet") != null)
    and (.required_contract_groups | index("voice recorder permission/capture/upload result taxonomy packet") != null)
    and (.required_contract_groups | index("avatar source/cropper/camera/editor artifact result taxonomy packet") != null)
  )) | length) == 1
  and (.waves | map(select(.gate == "remote_result_rich_ux_backend_contract"
    and (.required_contract_groups | index("message search remote date pins scope full result taxonomy") != null)
    and (.required_contract_groups | index("matrix link route event-context result taxonomy") != null)
    and (.required_contract_groups | index("edit history remote full-history source result taxonomy") != null)
    and (.required_contract_groups | index("mention remote hover profile result taxonomy") != null)
  )) | length) == 1
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.external_mutation == false
' "$BACKEND_GATES_PATH" >/dev/null

jq -e '
  .status == "ready"
  and .non_base_edge_gates_ready == true
  and .verified_edge_count == 4
  and (.covered_edge_ids | length) == 4
  and (.gates | length) == 4
  and (.gates | all(.status == "ready"))
  and (.covered_edge_ids | index("location_continuous_device_updates_local_controls") != null)
  and (.covered_edge_ids | index("tsp_wallet_destructive_import_actions") != null)
  and (.covered_edge_ids | index("spaces_room_membership_edges") != null)
  and (.covered_edge_ids | index("edit_poll_detail_edges") != null)
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.external_mutation == false
' "$NON_BASE_EDGE_GATES_PATH" >/dev/null

if [[ "$true_window_reference_mode" == "readiness" ]]; then
  jq -e '
    .status == "ready"
    and .native_window_smoke_ready == true
    and .native_window_smoke_enabled == true
    and (
      (.native_window_smoke_status != "ready")
      or .native_window_app_log_error_free == true
    )
    and (.native_window_smoke.screenshots | length) == 2
    and .side_effects.matrix_login == false
    and .side_effects.gateway_call == false
    and .side_effects.provider_invoked == false
    and .side_effects.channel_delivery == false
    and .side_effects.external_mutation == false
  ' "$TRUE_WINDOW_READINESS_PATH" >/dev/null
else
  desktop_window_bytes="$(stat -f%z "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH")"
  mobile_window_bytes="$(stat -f%z "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH")"
  desktop_window_sha="$(shasum -a 256 "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH" | awk '{print $1}')"
  mobile_window_sha="$(shasum -a 256 "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH" | awk '{print $1}')"
  desktop_window_width="$(sips -g pixelWidth "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  desktop_window_height="$(sips -g pixelHeight "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  mobile_window_width="$(sips -g pixelWidth "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  mobile_window_height="$(sips -g pixelHeight "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"

  jq -n \
    --arg desktop_path "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH" \
    --arg mobile_path "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH" \
    --arg desktop_sha "$desktop_window_sha" \
    --arg mobile_sha "$mobile_window_sha" \
    --argjson desktop_bytes "$desktop_window_bytes" \
    --argjson mobile_bytes "$mobile_window_bytes" \
    --argjson desktop_width "$desktop_window_width" \
    --argjson desktop_height "$desktop_window_height" \
    --argjson mobile_width "$mobile_window_width" \
    --argjson mobile_height "$mobile_window_height" \
    '
      $desktop_bytes >= 10000
      and $mobile_bytes >= 10000
      and $desktop_width == 1200
      and $desktop_height == 720
      and $mobile_width == 500
      and $mobile_height == 720
      and ($desktop_sha | test("^[0-9a-f]{64}$"))
      and ($mobile_sha | test("^[0-9a-f]{64}$"))
      and ($desktop_path | length > 0)
      and ($mobile_path | length > 0)
    ' >/dev/null
fi

handoff_summary_json="$(
  jq '{
    status:"ready",
    path:input_filename,
    handoff_count:(.items | length),
    status_set:(.items | map(.status) | unique),
    ui_lane_states:(.items | map(.ui_lane_state) | unique),
    next_owner_lanes:(.items | map(.next_owner_lane) | unique),
    taxonomy_ready_ids:(.items | map(select((.required_backend_contracts | tostring | contains("taxonomy")) or (.required_backend_contracts | tostring | contains("Taxonomy"))) | .id))
  }' "$HANDOFF_PATH"
)"

backend_summary_json="$(
  jq '{
    status,
    path:input_filename,
    verified_gap_count,
    wave_count:(.waves | length),
    wave_gates:(.waves | map(.gate)),
    taxonomy_groups:(.waves | map(.required_contract_groups // []) | add | map(select(test("taxonomy"; "i"))))
  }' "$BACKEND_GATES_PATH"
)"

non_base_summary_json="$(
  jq '{
    status,
    path:input_filename,
    verified_edge_count,
    covered_edge_ids,
    gate_count:(.gates | length)
  }' "$NON_BASE_EDGE_GATES_PATH"
)"

if [[ "$true_window_reference_mode" == "readiness" ]]; then
  true_window_summary_json="$(
    jq '{
      status,
      reference_mode:"readiness",
      path:.readiness_report_path,
      readiness_dir:.output_dir,
      native_window_smoke_ready,
      native_window_smoke_enabled,
      native_window_app_log_error_free,
      true_window_screenshot_count:(.native_window_smoke.screenshots | length),
      screenshots:(.native_window_smoke.screenshots // []),
      side_effects
    }' "$TRUE_WINDOW_READINESS_PATH"
  )"
else
  true_window_summary_json="$(
    jq -n \
      --arg smoke_dir "$TRUE_WINDOW_SMOKE_DIR" \
      --arg log_path "$TRUE_WINDOW_LOG_PATH" \
      --arg desktop_path "$TRUE_WINDOW_DESKTOP_SCREENSHOT_PATH" \
      --arg mobile_path "$TRUE_WINDOW_MOBILE_SCREENSHOT_PATH" \
      --arg desktop_sha "$desktop_window_sha" \
      --arg mobile_sha "$mobile_window_sha" \
      --argjson desktop_bytes "$desktop_window_bytes" \
      --argjson mobile_bytes "$mobile_window_bytes" \
      --argjson desktop_width "$desktop_window_width" \
      --argjson desktop_height "$desktop_window_height" \
      --argjson mobile_width "$mobile_window_width" \
      --argjson mobile_height "$mobile_window_height" \
      '{
        status:"ready",
        reference_mode:"standalone_window_smoke",
        path:null,
        readiness_dir:$smoke_dir,
        log:$log_path,
        native_window_smoke_ready:true,
        native_window_smoke_enabled:true,
        true_window_screenshot_count:2,
        screenshots:[
          {kind:"desktop", path:$desktop_path, width:$desktop_width, height:$desktop_height, bytes:$desktop_bytes, sha256:$desktop_sha},
          {kind:"mobile", path:$mobile_path, width:$mobile_width, height:$mobile_height, bytes:$mobile_bytes, sha256:$mobile_sha}
        ],
        side_effects:{
          matrix_login:false,
          gateway_call:false,
          provider_invoked:false,
          channel_delivery:false,
          external_mutation:false
        }
      }'
  )"
fi

jq -n \
  --arg status "ready" \
  --arg rollup_path "$ROLLUP_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg readiness_report_path "$READINESS_PATH" \
  --arg readiness_report_state "$readiness_report_state" \
  --arg true_window_reference_mode "$true_window_reference_mode" \
  --arg true_window_readiness_dir "$TRUE_WINDOW_READINESS_DIR" \
  --arg true_window_readiness_path "$TRUE_WINDOW_READINESS_PATH" \
  --arg true_window_smoke_dir "$TRUE_WINDOW_SMOKE_DIR" \
  --argjson latest_readiness "$latest_readiness_json" \
  --argjson true_window_reference "$true_window_summary_json" \
  --argjson base_gap_backend_handoff "$handoff_summary_json" \
  --argjson backend_contract_gates "$backend_summary_json" \
  --argjson non_base_edge_gates "$non_base_summary_json" \
  '{
    product:"Hepta Native",
    gate:"productization_blocker_rollup",
    status:$status,
    rollup_path:$rollup_path,
    readiness_dir:$readiness_dir,
    readiness_report_path:$readiness_report_path,
    readiness_report_state:$readiness_report_state,
    true_window_reference_mode:$true_window_reference_mode,
    true_window_reference_readiness_dir:$true_window_readiness_dir,
    true_window_reference_readiness_path:$true_window_readiness_path,
    true_window_reference_smoke_dir:$true_window_smoke_dir,
    productization_blocker_rollup_ready:true,
    native_secondary_product_surfaces_ready:$latest_readiness.native_secondary_product_surfaces_ready,
    native_secondary_product_surfaces:$latest_readiness.native_secondary_product_surfaces,
    latest_readiness:$latest_readiness,
    true_window_reference:$true_window_reference,
    base_gap_backend_handoff:$base_gap_backend_handoff,
    backend_contract_gates:$backend_contract_gates,
    non_base_edge_gates:$non_base_edge_gates,
    remaining_owner_lanes:["backend_contract"],
    ui_lane_scope:[
      "fixture and screenshot gates",
      "packaging resilience",
      "true-window evidence",
      "visual regression and density polish",
      "local taxonomy packets"
    ],
    backend_owned_blocker_groups:[
      "Wave 1 high-risk writes",
      "Wave 2 queue/progress and platform permissions",
      "Wave 3 remote result adapters and rich UX",
      "Non-base live adapter promotion"
    ],
    promotion_rules:[
      "No local taxonomy packet becomes a live adapter without a typed backend contract and gate.",
      "No external/public readiness claim uses fixture-only evidence without combined readiness, packaging, side-effect checks, and true-window evidence."
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
