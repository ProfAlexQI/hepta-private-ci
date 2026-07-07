#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-local-evidence-receipt-store-open-preconditions-readback-without-write-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-attempt-recording-denial-receipt-persistence-denial-terminal-no-persistence-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_RECEIPT_STORE_OPEN_PRECONDITIONS_READBACK_WITHOUT_WRITE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-local-evidence-receipt-store-open-preconditions-readback-without-write-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable local evidence receipt-store open-preconditions report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable terminal no-persistence source report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the local evidence receipt-store open-preconditions report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

source_json="$tmpdir/terminal-no-persistence.json"
target_json="$tmpdir/local-evidence-receipt-store-open-preconditions.json"

"$SOURCE_REPORT" >"$source_json" || fail "failed to render terminal no-persistence source report"
jq -e . "$source_json" >/dev/null || fail "terminal no-persistence source report did not render valid JSON"
HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_TERMINAL_NO_PERSISTENCE_JSON="$source_json" \
  "$REPORT" >"$target_json" || fail "failed to render local evidence receipt-store open-preconditions report from cached source"
jq -e . "$target_json" >/dev/null || fail "local evidence receipt-store open-preconditions report did not render valid JSON"

rg -q 'Controlled Live Evidence Receipt Store Local Evidence Receipt Store Open Preconditions Readback Without Write' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Local Evidence Receipt Store Open Preconditions Readback Without Write"
rg -q 'controlled live evidence receipt store local evidence receipt-store open-preconditions readback without write' "$DOC" \
  || fail "architecture note must document local evidence receipt-store open-preconditions readback without write"
rg -q 'no operator local-store approval request, dev evidence acceptance source recording, evidence acceptance recording, local receipt-store feature-gate opening, append-only store path grant, atomic append, post-append readback persistence, rollback anchor verification, retention policy commit, replay idempotency guard enablement, evidence record, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed local store open-preconditions boundary"

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_terminal_no_persistence_readback_ready == true
  and .source_terminal_entry_count == 7
  and .source_terminal_no_persistence_confirmed_count == 7
  and .source_terminal_closeout_recorded_count == 0
  and .source_terminal_closeout_persisted_count == 0
  and .source_terminal_closeout_accepted_count == 0
  and .source_terminal_closeout_authoritative_count == 0
  and .source_denial_receipt_persisted_count == 0
  and .source_write_attempt_recorded_count == 0
  and .source_receipt_store_written_count == 0
  and .source_receipt_persisted_count == 0
  and .source_live_execution_allowed == false
  and .source_cache_mode == "provided_source_json"
  and .source_cache_input_present == true
  and .source_report_render_count == 0
  and .target_source_reuse_count == 1
  and .lib_export_present == true
  and .local_open_preconditions_route == "readback://controlled-live/evidence-receipt-store/local-open-preconditions"
  and .open_precondition_entry_count == 7
  and .open_precondition_catalog_ready_count == 7
  and .source_terminal_closeout_attached_count == 7
  and .operator_local_store_approval_required_count == 7
  and .operator_local_store_approval_present_count == 0
  and .dev_evidence_acceptance_source_required_count == 7
  and .dev_evidence_acceptance_source_present_count == 0
  and .evidence_acceptance_required_count == 7
  and .evidence_acceptance_present_count == 0
  and .local_receipt_store_feature_gate_required_count == 7
  and .local_receipt_store_feature_gate_enabled_count == 0
  and .append_only_store_path_grant_required_count == 7
  and .append_only_store_path_grant_present_count == 0
  and .atomic_append_required_count == 7
  and .atomic_append_enabled_count == 0
  and .post_append_readback_required_count == 7
  and .post_append_readback_persisted_count == 0
  and .rollback_anchor_required_count == 7
  and .rollback_anchor_verified_count == 0
  and .retention_policy_required_count == 7
  and .retention_policy_committed_count == 0
  and .replay_idempotency_guard_required_count == 7
  and .replay_idempotency_guard_enabled_count == 0
  and .local_store_open_allowed_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .local_open_preconditions_readback_ready == true
  and .local_evidence_receipt_store_open_allowed == false
  and .operator_approval_request_allowed == false
  and .evidence_acceptance_recording_allowed == false
  and .receipt_store_feature_gate_open_allowed == false
  and .append_only_store_write_allowed == false
  and .receipt_store_write_attempt_recording_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_persistence_allowed == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("operator_local_store_approval_missing")) != null
  and (.blockers | index("dev_evidence_acceptance_source_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("local_receipt_store_feature_gate_closed")) != null
  and (.blockers | index("append_only_store_path_grant_missing")) != null
  and (.blockers | index("atomic_append_not_enabled")) != null
  and (.blockers | index("post_append_readback_missing")) != null
  and (.blockers | index("rollback_anchor_missing")) != null
  and (.blockers | index("retention_policy_not_committed")) != null
  and (.blockers | index("replay_idempotency_guard_disabled")) != null
  and (.blockers | index("receipt_store_write_attempt_recording_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_local_evidence_receipt_store_open_preconditions_without_write_"))
    and (.source_terminal_closeout_id | startswith("write-attempt-recording-denial-receipt-persistence-denial-terminal-no-persistence:controlled-live-evidence-receipt-store:"))
    and (.source_terminal_closeout_key | startswith("terminal-no-persistence:write-attempt-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:"))
    and (.source_terminal_closeout_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-attempt-recording-denial-receipts/persistence-denial/terminal-no-persistence/"))
    and .source_terminal_reason == "write_attempt_recording_denial_receipt_persistence_denied_retention_replay_projected_no_persistence_authority"
    and (.source_persistence_denial_id | startswith("write-attempt-recording-denial-receipt-persistence-denial:controlled-live-evidence-receipt-store:"))
    and (.local_open_precondition_set_id | startswith("local-evidence-receipt-store-open-preconditions:controlled-live-evidence-receipt-store:"))
    and (.local_open_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/local-open-preconditions/"))
    and (.operator_local_store_approval_id | startswith("operator-local-store-approval:controlled-live-evidence-receipt-store:"))
    and (.dev_evidence_acceptance_source_id | startswith("dev-evidence-acceptance-source:controlled-live-evidence-receipt-store:"))
    and (.evidence_acceptance_key | startswith("controlled_live.local_evidence_acceptance.required."))
    and (.local_receipt_store_feature_gate | startswith("feature-gate:controlled-live-local-evidence-receipt-store:"))
    and (.append_only_store_path_grant_key | startswith("append-only-store-path-grant:controlled-live-evidence-receipt-store:"))
    and (.atomic_append_plan_id | startswith("atomic-append-plan:controlled-live-local-evidence-receipt-store:"))
    and (.post_append_readback_route | startswith("readback://controlled-live/evidence-receipt-store/local-open-preconditions/post-append/"))
    and (.rollback_anchor_route | startswith("readback://controlled-live/evidence-receipt-store/local-open-preconditions/rollback-anchor/"))
    and (.retention_policy_id | startswith("retention-policy:controlled-live-local-evidence-receipt-store:"))
    and (.replay_idempotency_guard_key | startswith("replay-idempotency-guard:controlled-live-local-evidence-receipt-store:"))
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "local_evidence_receipt_store_open_preconditions_listed_without_write"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .source_terminal_closeout_attached == true
    and .terminal_no_persistence_confirmed == true
    and .operator_local_store_approval_required == true
    and .operator_local_store_approval_present == false
    and .dev_evidence_acceptance_source_required == true
    and .dev_evidence_acceptance_source_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .local_receipt_store_feature_gate_required == true
    and .local_receipt_store_feature_gate_enabled == false
    and .append_only_store_path_grant_required == true
    and .append_only_store_path_grant_present == false
    and .atomic_append_required == true
    and .atomic_append_enabled == false
    and .post_append_readback_required == true
    and .post_append_readback_persisted == false
    and .rollback_anchor_required == true
    and .rollback_anchor_verified == false
    and .retention_policy_required == true
    and .retention_policy_committed == false
    and .replay_idempotency_guard_required == true
    and .replay_idempotency_guard_enabled == false
    and .local_store_open_allowed == false
    and .evidence_recording_allowed == false
    and .evidence_recorded == false
    and .receipt_store_write_attempt_recording_allowed == false
    and .receipt_store_write_attempt_recorded == false
    and .receipt_store_write_allowed == false
    and .receipt_store_written == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .ledger_write_allowed == false
    and .ledger_written == false
    and .workflow_event_log_write_allowed == false
    and .workflow_event_log_written == false
    and .sqlite_write_allowed == false
    and .sqlite_written == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .local_open_precondition_route == "readback://controlled-live/evidence-receipt-store/local-open-preconditions/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .operator_local_store_approval_id == "operator-local-store-approval:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .dev_evidence_acceptance_source_id == "dev-evidence-acceptance-source:controlled-live-evidence-receipt-store:fresh_soak_readback_missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_local_evidence_acceptance_source_readback_without_recording"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' "$target_json" >/dev/null

jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_attempt_recording_denial_receipt_persistence_denial_terminal_no_persistence_readback"
  and .status == "ready_blocked"
  and .terminal_no_persistence_readback_ready == true
  and .terminal_entry_count == 7
  and .terminal_no_persistence_confirmed_count == 7
  and .terminal_closeout_recorded_count == 0
  and .terminal_closeout_persisted_count == 0
  and .terminal_closeout_accepted_count == 0
  and .terminal_closeout_authoritative_count == 0
  and .denial_receipt_persisted_count == 0
  and .write_attempt_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' "$source_json" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime \
    controlled_live_evidence_receipt_store_local_evidence_receipt_store_open_preconditions_readback_without_write --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-local-evidence-receipt-store-open-preconditions-readback-without-write-gate: PASS: local evidence receipt-store open preconditions are read back without evidence recording, receipt-store write, or live execution\n'
