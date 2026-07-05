# Hepta Native Remote Result Rich UX Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Wave: 3 remote result, route context, full-history, and rich mention UX contracts
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_GAP_IDS:message_search,matrix_link_resolution,message_edit_history,mention_picker_send`
- `HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

P1 `message_search`:

- Live Matrix `/search` request with `next_batch` retry.
- Live server context preview parsing for `event_context` before/after windows.
- Live source-only current-room refetch for loaded result actions.
- Live loaded-scope filters, sender filter, media URL filter, and loaded Jump/Copy/Source/Thread/Sender handoffs.
- Local remote date/pins/scope/full-result result taxonomy packet.

P6 `matrix_link_resolution`:

- Live compact Matrix room/alias preview retry.
- Live cached Server context refresh for room/alias targets.
- Live current-room missing-event pagination.
- Live source-only fetch for current-room event JSON.
- Live confirmed browser handoff and room/alias Join, Knock, and current-room Invite requests.
- Local route/event-context result taxonomy packet.

P8 `message_edit_history`:

- Live paginated `m.replace` relations fetch and retry.
- Live local full snapshot modal.
- Live loaded original/replacement source side-by-side preview.
- Live full-body diff when both loaded JSON sources are present.
- Local remote full-history/source reconciliation result taxonomy packet.

P9 `mention_picker_send`:

- Live `SendMessage` and attachment-caption mentions.
- Live pill tray and local token insertion.
- Live Matrix user-directory search.
- Live bounded directory result promotion to user-id mention token.
- Live cached-directory or loaded-member hover-card snapshot.

## Promotion Boundary

Do not wire remaining remote result pages, route adapters, server-backed full-history views, rich disambiguation, remote hover profiles, edit/reply mention payload scopes, or cross-room event-context surfaces until the backend contract provides request identity, source hashes, stale-target guards, retry policy, error taxonomy, and side-effect boundaries.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Room-state or membership mutation outside explicit confirmed join/knock/invite results already covered by P6.
- Message send/edit/redact outside explicit P9 `SendMessage` or attachment-caption mention results.
- Account/profile mutation.
- Automatic join, invite, message send, or retry from preview/result rendering alone.
- Secret, token, or unredacted profile payload in logs, docs, clipboard, or gate reports.

## Remote Result Cursor Contract

Required fields:

- `surface_id`
- `query_hash`
- `cursor_id`
- `request_id`
- `source_hash`
- `page_direction`
- `page_size`
- `result_count`
- `next_cursor`
- `exhausted`
- `status`: `loading`, `ready`, `empty`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Cursor identity must remain stable across retry and pagination.
- A stale query, target, event id, or token must disable action buttons until refreshed.
- Result pages must say whether they are loaded from local timeline state, Matrix server search, room relations, user directory, or a backend adapter.
- Retry must require cached request identity and must not mutate room, message, account, or delivery state by itself.

## P1 Message Search Contract

live_wiring: `matrix_server_search_v3_search_next_batch_retry_context_pagination_sender_media_url_filter_loaded_scope_filters_loaded_result_actions_server_result_context_preview_source_refetch_live;remote_date_pins_scope_full_result_blocked`

Required contract groups:

- Matrix search request/result/error/retry and loaded result-action handoffs.
- Remote result page and cursor identity.
- Matrix `/search` event-context preview and source-only current-room source refetch.
- Remote date index.
- Remote pinned event fetch.
- Cross-room scope and sort contract.
- Remote date/pins/scope/full-result result taxonomy packet.
- Stale-query and idempotency guard.

Acceptance rules:

- Remote Date and Pins controls must not be enabled from loaded local filters alone.
- Cross-room search results must identify room id, event id, pagination cursor, and source availability.
- Source refetch success is not full remote result rendering.
- Date index, pinned fetch, scope expansion, and sort order must have distinct error kinds.

### P1 Remote Date Pins Scope Full-Result Taxonomy UI Packet

The UI now exposes a local `Taxonomy` packet in the message-search server preflight row.
The packet records the current live references as Matrix `/search` first-page reads,
`next_batch` Older pagination, failed Retry first-page resubmit, From sender filter,
Media URL filter, parsed `event_context` previews, current-room
`BackwardsPaginateUntilEvent` context pagination, source-only `FetchEventSource`,
loaded Jump/Copy/Thread/Sender handoffs, and loaded-scope Filter/Date/Pins over
existing timeline rows plus the pinned-events subscription.

The packet keeps the following remote adapters blocked:

- `remote_date_index_operation_id not_assigned`
- `remote_pinned_fetch_operation_id not_assigned`
- `cross_room_scope_request_id not_assigned`
- `full_result_cursor_id not_assigned`
- `full_result_page_id not_assigned`
- `sort_order_result not_wired`
- `room_preview_result not_wired`
- `non_current_room_context_result not_wired`
- `full_result_render_result not_wired`
- `stale_query_result not_wired`
- `retry_cancel_result not_wired`
- audit redaction limited to query hash, event ids, and room ids

The packet does not submit extra Matrix search beyond the explicit
Server/Older/Retry/From/Media controls, does not run a remote date index query,
does not fetch remote pinned events or send `PinEvent`, does not run cross-room
scope search, does not fetch room previews or non-current-room event context,
does not render a full remote result adapter, does not automate retry, and does
not perform message, room-state, membership, account/profile, gateway/runtime,
provider-auth, Telegram delivery, or live mutation side effects.

## P6 Matrix Link Route Contract

live_wiring: `matrix_link_compact_preview_retry_server_context_refresh_current_room_event_pagination_source_fetch_browser_opener_room_or_alias_join_knock_current_room_user_invite_live;event_context_route_adapter_blocked`

Required contract groups:

- Compact room preview request/result/error/retry.
- Current-room missing-event pagination result.
- Room, alias, event, and via route result.
- Server preview and non-current-room event context result.
- Confirmed browser handoff outcome.
- Confirmed room-or-alias Join, Knock, and current-room Invite results.
- Source-only event source fetch result.
- Route/event-context result taxonomy packet.
- Full event-context failure taxonomy.
- Source-hash and stale-target guard.

Acceptance rules:

- Server context refresh may reuse cached room/alias target metadata, but must not invent non-current-room event context.
- Browser handoff success must be distinct from Matrix join/knock/invite success.
- Invite controls must remain scoped to current-room user targets unless a backend route adapter returns an explicit invite target.
- Via servers, alias resolution, event context, and access-denied states must be separate result types.

### P6 Matrix Link Route/Event-Context Taxonomy UI Packet

Current UI evidence:

- The Matrix link preview strip exposes `Taxonomy` beside Room, Event, Via,
  Preview, Source, Packet, and Contract route-scope controls.
- The packet is derived only from cached preview-strip status, target, via
  servers, requested event id, preview metadata/error length, retry-cache state,
  and loaded current-room source availability.
- Existing live references are named explicitly: loaded alias navigation,
  loaded current-room event jump, current-room
  `BackwardsPaginateUntilEvent` / `PaginateTimeline`, compact
  `PreviewMatrixLinkTarget` preview, cached Server refresh, confirmed failed
  Retry, source-only `Room::load_or_fetch_event`, loaded or preview-fetched
  `EventSourceModal` Source, confirmed matrix.to Browser opener, confirmed
  JoinRoomByIdOrAlias, confirmed Knock, and confirmed current-room InviteUser.
- Remaining route adapter slots stay local: `route_adapter_request_id`,
  `alias_resolution_operation_id`,
  `non_current_room_event_context_operation_id`, `via_route_request_id`,
  `full_remote_source_request_id`, `event_context_window_result`,
  `alias_resolution_result`, `via_resolution_result`,
  `full_remote_source_result`, `access_denied_result`,
  `stale_target_result`, `retry_cancel_result`, and audit redaction.

The packet does not submit extra `PreviewMatrixLinkTarget` beyond compact
preview, Server refresh, or confirmed Retry controls, does not run
`BackwardsPaginateUntilEvent` outside current-room missing event links, does not
run server-side alias resolution, does not fetch event context, does not
paginate non-current-room timelines, does not fetch full remote source, does
not open Browser or Join/Knock/Invite without confirmation, and does not perform
message, room-state, membership outside confirmed join/knock/invite,
account/profile, gateway/runtime, provider-auth, Telegram delivery, or live
mutation side effects.

## P8 Edit History Full Result Contract

live_wiring: `matrix_edit_history_paginated_relations_retry_local_full_snapshot_modal_loaded_original_replacement_source_side_by_side_preview_full_body_diff_live;remote_full_history_result_adapter_context_source_reconciliation_server_backed_full_body_diff_blocked`

Required contract groups:

- Paginated `m.replace` relations request/result/error/retry.
- Full edit-history modal request/result.
- Loaded side-by-side preview and full-body diff snapshot.
- Server-backed full-history full-body diff payload.
- Replacement event context/source result.
- Remote full-history/source reconciliation result taxonomy packet.
- Stale-target, retry, and source-hash guard.

Acceptance rules:

- Loaded source JSON can power a local full-body diff, but not a server-backed full edit-history claim.
- Each replacement event must carry stable event id, relation id, timestamp, sender, source availability, and source hash.
- Full history pagination exhaustion must be explicit.
- Missing original or replacement source must downgrade to preview diff without claiming data loss.

### P8 Edit History Remote Full-History/Source Taxonomy UI Packet

The UI now exposes a local `Taxonomy` packet in the edit-history full-controls row.
The packet records the current live references as paginated
`MatrixRequest::FetchEditHistory` through `Room::relations` `next_batch`
exhaustion, confirmed failed-state Retry, local synthetic Full
`EventSourceModal`, loaded side-by-side preview/full-body diff snapshot, compact
diff clipboard handoff, cached latest replacement raw JSON `EventSourceModal`,
source-only `MatrixRequest::FetchEventSource` / `Room::load_or_fetch_event`
fallback, and loaded original `EventSourceModal` fallback.

The packet keeps the following remote adapters blocked:

- `remote_full_history_request_id not_assigned`
- `full_history_cursor_id not_assigned`
- `full_history_page_result not_wired`
- `server_backed_full_diff_operation_id not_assigned`
- `server_backed_full_diff_result not_wired`
- `replacement_source_reconciliation_operation_id not_assigned`
- `replacement_source_result not_wired`
- `event_context_operation_id not_assigned`
- `event_context_result not_wired`
- `stale_target_result not_wired`
- `retry_cancel_result confirmed_retry_only/cancel_local_only`
- source-hash policy pending original event, latest replacement source,
  relation page, body-normalization, and target hashes

The packet does not submit extra `MatrixRequest::FetchEditHistory`, does not
retry without `PositiveConfirmationModal`, does not request a remote
full-history modal, does not render server-backed side-by-side full diff, does
not fetch event context, does not reload/paginate the timeline, does not fetch
replacement source beyond the existing Source fallback, and does not perform
message, room-state, membership, account/profile, gateway/runtime,
provider-auth, Telegram delivery, or live mutation side effects.

## P9 Mention Rich UX Contract

live_wiring: `matrix_sendmessage_attachment_caption_mentions_pill_tray_user_directory_search_result_promotion_hover_card_snapshot_live;rich_disambiguation_remote_hover_edit_payload_scopes_blocked`

Required contract groups:

- Rich picker and richer directory-result UI beyond bounded user-id token promotion.
- Duplicate-name disambiguation result.
- Local hover-card snapshot plus remote hover-card/profile adapter.
- Source, tray, and pill draft contract.
- `SendMessage` `add_mentions` result.
- Attachment-caption `AttachmentConfig.mentions` result.
- Rich attachment editor and edit/reply mention payload scopes.
- Remote hover/profile/disambiguation/edit-reply result taxonomy packet.
- Stale-token, source-hash, and idempotency guard.

Acceptance rules:

- Directory result promotion must insert an explicit Matrix user id token, not a display-name guess.
- Duplicate display names must remain unresolved until a disambiguation result supplies stable user ids and display metadata.
- Local hover-card snapshots are not remote profile refreshes.
- Edit, reply, send, and attachment-caption mention payloads must be scoped separately.
- Retry or send must not occur from hover or picker rendering alone.

### P9 Mention Remote Result Taxonomy UI Packet

Current UI packet is local metadata only:

- Live references are limited to `SendMessage` `add_mentions`, attachment-caption
  `AttachmentConfig.mentions`, read-only `MatrixRequest::SearchUserDirectory`
  through `client.search_users`, `UserDirectorySearchAction::Searched`
  result/error metadata, bounded directory result promotion to literal Matrix
  user-id tokens, local hover-card snapshots from cached directory or loaded
  member metadata, local completed-token pill removal, local rich packet
  snapshots, and local Packet/Contract copy.
- `rich_picker_operation_id`, `duplicate_disambiguation_operation_id`,
  `remote_hover_profile_operation_id`, `rich_attachment_editor_operation_id`,
  `edit_payload_operation_id`, and `reply_payload_operation_id` are
  `not_assigned`.
- Richer directory result UI, duplicate-name resolution, hover profile result,
  avatar/profile fetch beyond directory response metadata, rich attachment
  editor result, edit/reply rewrite result, multi-select tray, pill editor
  mutation, retry/cancel automation, and source-hash reconciliation are
  `not_wired`.
- Promotion requires token hash, query hash, room id, member-cache generation,
  directory-result generation, composer revision, payload-scope hash, retry
  policy, stale-token handling, idempotency, and audit redaction.

The packet does not submit remote member lookup beyond explicit Directory, does
not fetch profile/avatar data, does not request a remote hover card, does not
run duplicate-name disambiguation, does not open rich popup search, does not
rewrite attachment/edit/reply mention payloads, does not submit extra
`SendMessage` or `SendAttachment`, and does not perform typing, room-state,
membership, account/profile, gateway/runtime, provider-auth, Telegram delivery,
or live mutation side effects.

## Verification Requirements

Before any backend adapter promotes these blocked controls, provide:

- Machine-readable request and result structs for each contract group.
- Error taxonomy that distinguishes unavailable, forbidden, stale, unsupported, failed, and exhausted states.
- Source-hash, target-hash, token-hash, and query-hash rules for stale UI.
- Retry eligibility and confirmation rules.
- Side-effect inventory for Matrix room/message/account writes.
- Screenshot or fixture marker coverage for every newly enabled control.
- Contract tests proving no gateway/runtime/provider-auth/Telegram delivery side effects are introduced.
