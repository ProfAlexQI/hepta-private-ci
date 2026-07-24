#!/usr/bin/env bash

hepta_expected_public_ga_blockers_json() {
  jq -cn '[
    "gateway_replacement_not_ready",
    "telegram_owner_handoff_not_operator_approved",
    "telegram_live_poll_model_send_soak_not_complete",
    "native_post_real_activation_not_operator_approved",
    "credentialed_provider_live_smoke_not_operator_approved",
    "channel_live_delivery_not_operator_approved",
    "release_artifact_pack_not_operator_approved",
    "external_public_release_not_operator_approved",
    "control_ui_live_truth_not_available_on_active_legacy_schema",
    "control_ui_product_behavior_evidence_not_bound"
  ]'
}
