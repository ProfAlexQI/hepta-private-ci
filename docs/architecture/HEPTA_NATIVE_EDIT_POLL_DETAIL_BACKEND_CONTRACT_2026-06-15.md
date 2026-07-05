# Hepta Native Edit/Poll Detail Backend Contract - 2026-06-15

This contract covers the remaining edit and poll details outside the 12 tracked
base gaps. The main message edit path and edit-history/source/diff surfaces are
already productized; these detail edges remain local until payload and result
semantics are typed.

## Current UI State

- `EditingPane` guards Save Edit behind confirmation before the existing
  `MatrixRequest::EditMessage` path is requested.
- HTML/plain prefixes, attachment add/remove, mention extraction, poll answer
  edits, and Save spinner now have a visible local Edit/Poll detail packet.
- Edit attachment add/remove/replace also has a local attachment preflight
  packet that records original media scope, selected/add/remove/replace/upload/
  delete slots, caption-only handoff, MIME/size probe, retry, and cancel policy
  before any attachment edit behavior exists.
- Poll timeline rows render read-only previews from loaded
  `matrix_sdk_ui::timeline::PollState` through `populate_poll_message_content`.
- Poll timeline rows now include a local poll answer preview/result packet that
  records answer count, total votes, max selections, open/closed state, edited
  state, answer edit slot, vote-response slot, result mapping, stale poll
  policy, and unsupported-capability boundary from the loaded `PollState` only.
- Editing a poll currently preserves existing answers and only edits the
  question through the existing edit path.

Current UI packet:

- `EditingPane` renders a detail packet with content kind, edited text length,
  attachment edit slot, mention payload scope, poll answer edit slot, save
  spinner operation id, result mapping, and stale-result policy
- media message edits still only update body/formatted caption through the
  existing confirmed `MatrixRequest::EditMessage` path
- mention handling preserves existing mentions only when present; it does not
  extract a fresh Matrix Mentions payload from edited text
- poll editing preserves loaded answers and max-selection data, edits only the
  question, and sends no poll answer edit packet
- poll answer preview/result packet remains read-only and sends no poll
  response, poll answer edit, timeline reload, message, room-state, or
  membership request
- save-result mapping packet remains local: `operation_id_slot not_assigned`,
  `request_slot existing_confirmed_MatrixRequest_EditMessage`,
  `spinner_slot not_rendered`, `result_mapping` saved/failed/canceled/stale and
  ignored-late states, `stale_result_guard timeline_event_item_id_match_only`,
  repeated-save policy, and retry slot
- retry/error drilldown packet remains local: `failure_source
  existing_MatrixRequest_EditMessage_result_only`, `retry_request_slot
  not_built`, `retry_confirmation_slot not_built`, `late_result_guard
  timeline_event_item_id_match_only_without_operation_id`,
  `pending_operation_id missing_backend_contract`, `spinner_state
  not_rendered`, `cancel_state confirmation_cancel_no_request`, and
  `error_redaction popup_text_not_persisted_or_reused`
- attachment preflight packet remains local: `original_attachment_scope`
  image/audio/file/video caption-only or no-media/poll, `selected_attachment_slot
  unavailable`, `add_attachment_slot not_built`, `remove_attachment_slot
  not_built`, `replace_attachment_slot not_built`, `upload_request_slot
  not_built`, `media_delete_slot not_built`, `caption_edit_handoff
  existing_confirmed_MatrixRequest_EditMessage_body_only`, `mime_size_probe
  not_started`, `retry_policy no_duplicate_upload_without_operation_id`, and
  `cancel_policy leaves_original_media_and_local_selection_untouched`

## Edit Attachment Contract

### Attachment Preflight Packet

Current UI packet:

- `EditingPane` renders a local attachment preflight packet under the main
  Edit/Poll detail packet
- packet state includes content kind, edited text length, original attachment
  scope for image/audio/file/video caption edits or non-media/poll edits,
  selected attachment availability, add/remove/replace slots, upload request
  slot, media delete slot, caption-only EditMessage handoff, MIME/size probe,
  retry policy, and cancel policy
- media message edits still only update body/formatted caption through the
  existing confirmed `MatrixRequest::EditMessage` path
- no `SendAttachment`, media delete, upload, timeline reload, room-state,
  membership, gateway/runtime/auth, or live mutation is sent by the packet

Before edit attachment add/remove can be live:

- payload must define whether the edit replaces, removes, or supplements media
  from the original event
- upload/delete semantics must be separate from text edit confirmation
- result must distinguish upload failed, edit failed, media removed, media
  preserved, and stale original event
- retry must not duplicate media upload or send a plain text fallback
- cancel must leave the original event and selected local media untouched

Forbidden side effects:

- no unconfirmed upload, media delete, or `SendAttachment`
- no room-state or membership mutation
- no gateway/runtime/provider/auth/channel delivery call

## Edit Mention Payload Contract

Current local packets:

- `EditingPane` renders a mention payload preflight packet from edited text only
- packet state includes edited `@` token count, literal Matrix user-id token
  count, `@room` recheck scope, unavailable directory result scope,
  completed-pill reconciliation not connected to the editing pane, existing
  mentions handoff, retry source-hash gap, stale token policy, and cancel policy
- `EditingPane` also renders a typed contract packet for the future backend
  shape: local contract version, edited-text scan source, unavailable directory
  and completed-pill snapshot ids, not-assigned source hash, not-built fresh
  mentions payload result, missing retry idempotency key, body-source-hash stale
  guard, not-wired result mapping, and token-count-only redaction
- no fresh Matrix Mentions payload, directory/profile lookup, `SendMessage`,
  `SendAttachment`, room-state, or membership request is sent

Before edit-time mention extraction is live:

- edited body and formatted body must produce a deterministic Matrix Mentions
  payload
- completed mention pills, literal Matrix user ids, `@room`, and remote
  directory selections must be reconciled before the edit request
- mention payload must not leak stale reply/send-time mention state
- retry must reuse the confirmed edited body and mention payload together

## Poll Answer Edit Contract

Current UI packet:

- `RoomScreen` renders a Poll answer preview/result packet in the read-only poll
  timeline row
- packet state includes answer count, total votes, max selections, open/closed
  status, edited state, `answer_edit_slot not_built`, `vote_response_slot
  not_sent`, `result_mapping read_only_loaded_pollstate`, stale poll policy,
  and unsupported server capability boundary
- packet data is derived only from already loaded `PollState`
- no poll response, poll answer edit, timeline reload, message, room-state, or
  membership request is sent

Before poll answer edits are live:

- answer ids, edited labels, max selections, open/closed state, and previously
  cast votes must have explicit semantics
- unsupported server capabilities must keep answer editing disabled
- result must distinguish accepted, rejected, stale poll, closed poll,
  permission denied, and malformed answer set
- cancel must preserve the loaded poll state without sending any edit

## Save Spinner / Result Contract

Before a persistent edit save spinner is live:

- pending operation id must survive redraws and room switches
- result must map to saved, failed, stale, canceled, and ignored-late-result
  states
- spinner must clear on confirmation cancel and on stale result
- repeated Save while pending must be held locally

Current UI save-result packet:

- lifecycle_state: idle_preflight, confirmation_opened, saved_hide_pane,
  failed_popup, stale_event_id_ignored
- operation_id_slot: not_assigned
- request_slot: existing_confirmed_MatrixRequest_EditMessage
- spinner_slot: not_rendered
- result_mapping: saved_hide_pane, failed_popup, canceled_no_request,
  stale_event_id_ignored, ignored_late_result_without_matching_operation_id
- stale_result_guard: timeline_event_item_id_match_only
- repeated_save_policy: not_held_until_pending_operation_id
- retry_slot: not_built
- side-effect boundary: no attachment upload/remove, Matrix mention payload,
  poll answer edit, timeline reload, message send, room-state, or membership
  request beyond the existing confirmed edit request

## Retry/Error Drilldown Packet

Current UI packet:

- lifecycle_state: idle_preflight, confirmation_opened, saved_hide_pane,
  failed_popup, stale_event_id_ignored
- failure_source: existing_MatrixRequest_EditMessage_result_only
- error_redaction: popup_text_not_persisted_or_reused
- retry_request_slot: not_built
- retry_confirmation_slot: not_built
- late_result_guard:
  timeline_event_item_id_match_only_without_operation_id
- pending_operation_id: missing_backend_contract
- spinner_state: not_rendered
- cancel_state: confirmation_cancel_no_request
- repeated_save_policy: not_held_until_pending_operation_id
- stale_result_policy: ignore_late_result_without_matching_operation_id
- side-effect boundary: no attachment upload/remove, Matrix mention payload,
  poll answer edit, timeline reload, extra message send beyond the existing
  confirmed edit request, room-state, membership, gateway/runtime/auth, or live
  mutation

## Acceptance Gate

`scripts/hepta-native-edit-poll-detail-contract-gate.sh` validates this contract
against current edit/poll local evidence and optional readiness artifacts.
