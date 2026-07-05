# Hepta Native Spaces/Room Membership Edge Contract - 2026-06-15

This contract covers Native UI edges around spaces, room-list membership, and
selected-room recovery. These surfaces already have read/sync evidence; live
promotion needs precise membership and pagination contracts so UI does not
invent unsafe room actions.

## Current UI State

- RoomsList removes banned rooms and skips knocked/left rooms locally.
- AddRoom and SpaceLobby show banned/knocked edge evidence and now render a
  local re-knock/cancel-prior packet; AddRoom re-knock may reuse the existing
  confirmed Knock path only where the target is explicit, SpaceLobby tree
  re-knock remains unexposed, and cancel-prior-knock remains unwired.
- Room-list adapter consumes the current `RoomListService` result locally; there
  is no visible Load more rooms control. The status row now renders a local
  Load More pagination packet with loaded counts, server max hint,
  selected-space child pagination state, and explicit cursor/result/retry slots
  held as not exposed/not built.
- Space hierarchy uses existing `GetChildren`, `GetDetailedChildren`,
  `GetTopLevelSpaceDetails`, subscribe, and paginate read/sync paths.
- `UnsubscribeFromSpaceRoomList` is service lifecycle cleanup, not a user-facing
  stop-sync control.
- selected-space parent filtering uses cached SpaceService child/subspace maps.
- space unread/mention counters and People/Rooms section mention aggregates
  now expose local aggregate packets while section header badges remain local
  zero placeholders.
- removed-room handling clears stale selected-room focus locally and now keeps
  a selected-room removed/rejoin packet with room id, membership state, active
  selection match, FocusNone clear, selected-space scope, replacement UI slot,
  rejoin request slot, and stale-event policy; replacement/rejoin action remains
  unwired.

## Re-Knock / Cancel Prior Knock Contract

Current UI packet:

- AddRoom knocked previews render a local packet with target, join rule,
  existing confirmed re-knock path, missing previous-knock request id, missing
  cancel-prior request/result slots, and stale membership policy
- SpaceLobby knocked tree rows render a local packet with target kind, target
  id, join rule, suggestion state, tree re-knock action slot, missing
  cancel-prior request/result slots, and stale membership policy
- AddRoom re-knock stays behind the existing `PositiveConfirmationModal` and
  `MatrixRequest::Knock` path
- SpaceLobby tree rows expose no new re-knock action
- neither packet submits cancel-prior-knock, unconfirmed knock, join, leave,
  message, room-state, or membership mutation

Before any new knocked-room action is promoted:

- target must include room id or alias, via servers when available, current
  membership state, and source surface (AddRoom, SpaceLobby, matrix link, row)
- re-knock must be confirmation-gated and must reuse the existing Matrix Knock
  result/error/retry taxonomy
- cancel-prior-knock must define the Matrix operation, server support, result
  states, stale membership handling, retry, and error taxonomy before UI exposure
- banned/restricted/unknown join-rule cases must stay disabled unless a newer
  typed membership contract allows them

Forbidden side effects:

- no unconfirmed `JoinRoom`, `LeaveRoom`, `Knock`, invite, or membership write
- no room-state mutation
- no message send/edit/redact
- no gateway/runtime/provider/auth/channel delivery call

## Selected Room Removed / Rejoin Contract

Current UI packet:

- the selected-room removed/rejoin packet is visible in the RoomsList status
  evidence row before any replacement UI exists
- packet state is derived only from `RoomsListUpdate::RemoveRoom`, current
  selected room, current selected space, and local RoomsList cache
- active-room removals clear focus with `AppStateAction::FocusNone`
- non-active removals record a no-op focus state
- replacement UI and rejoin request slots stay `not_wired`/`not_built`

Before a removed active room can show replacement/rejoin UI:

- removal reason must distinguish left, kicked, banned, tombstoned/successor,
  forgotten, inaccessible, and unknown
- current active-room state must be cleared or replaced exactly once
- replacement/rejoin action must be confirmation-gated and must name the exact
  Matrix request it will submit
- stale selected-room events must not resurrect removed room state after the
  replacement UI appears

## Room-List Pagination Contract

Current UI packet:

- `RoomsList` renders a Room-list Load More pagination packet from local
  `RoomListService` and `SpaceService` state
- packet state includes loaded joined-room count, displayed invite/people/rooms
  counts, server max hint when available, selected-space child pagination state,
  direct child room/subspace counts, missing Load More button slot, missing
  explicit cursor slot, local in-flight/error/retry/exhaustion slots, and the
  visible-row latest-message preview pagination source
- no user-triggered Load More button is rendered
- existing selected-space child pagination remains the current service-driven
  read-sync path
- visible-row latest-message prefetch remains the existing
  `MatrixRequest::PaginateTimeline` read-only path
- the packet sends no user-triggered room-list pagination, message, room-state,
  or membership mutation

Before a Load More rooms UI is added:

- adapter must expose current loaded count, total/unknown total, cursor or
  pagination token, loading state, exhausted state, and error state
- repeated Load More must be idempotent while a page is in flight
- failures must expose retry/cancel metadata without mutating membership or
  sending messages
- visible row prefetch for latest messages must remain separate from room-list
  pagination

## Space Aggregate Contract

Current UI packet:

- RoomsList status evidence renders a People/Rooms unread/mention aggregate
  packet from already loaded row state
- packet state includes direct/regular loaded room counts, loaded unread
  totals, loaded mention totals, manual-unread counts, selected-space scope,
  active filter state, header badge source, missing aggregate refresh slot,
  partial parent-chain attribution, and undefined muted/low-priority policy
- People/Rooms section header badge values remain local zero placeholders
- `JoinedSpaceInfo` keeps unread and mention fields at local zero, so
  `is:unread` and `is:mention` over spaces use the room-display-filter zero
  source
- neither packet sends aggregate refresh, read receipt, message, room-state, or
  membership mutation

Before space unread/mention filters or section aggregates become live:

- aggregate source must define per-room unread, per-room mention, muted rooms,
  archived/low-priority inclusion, spaces parent-chain attribution, and direct
  room handling
- counters must update without sending read receipts
- filter matches must be deterministic when parent-chain cache is partial
- aggregate refresh must be read-only and must not trigger membership mutation

## Acceptance Gate

`scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh` validates
this contract against current code, existing local evidence, and optional
readiness artifacts.
