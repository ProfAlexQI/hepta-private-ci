# Hepta Native Non-Base Edge Risk Plan - 2026-06-15

This plan covers Native UI surfaces that sit outside the 12 tracked
Telegram/Matrix base gaps. The 12 base gaps are already UI-complete and
backend-contract owned; this file tracks the remaining lower-layer edge
capabilities where UI/product evidence still matters.

## Current Closure

### Location Continuous Device Updates

Status: UI live local control complete.

`LocationPreview` now exposes explicit Start and Stop device-update controls.
Opening the preview still requests `LocationRequest::UpdateOnce`; Start submits
only `LocationRequest::StartUpdates`, and Stop or Cancel submits only
`LocationRequest::StopUpdates` when local continuous updates are active.

Boundary:

- no live-location Matrix event is created
- no `MatrixRequest::SendMessage` is submitted by Start/Stop
- no room-state, membership, account/profile, gateway/runtime/auth, provider,
  or channel delivery mutation is emitted
- the existing one-time location send remains behind the current confirmation
  guard

Evidence:

- `apps/hepta-native/src/home/location_preview.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-fixture-visual-smoke.sh`
- readiness artifact:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.location-live-controls-20260615`

### TSP Wallet Import Preflight Packet

Status: UI local preflight and result taxonomy packets complete.

`TspSettingsScreen` now opens an Import Wallet preflight modal from the
existing Import Existing Wallet control. The packet records picker, selected
path, password, vault-open, persistence, duplicate-policy, loaded wallet count,
active-wallet, other-wallet, and active-identity states before any backend
import behavior exists.

The same modal now also records a local import result taxonomy packet covering
picker, auth, vault-open, metadata, duplicate, persistence, retry, cancel,
stale-operation, and audit-redaction states with `operation_id_slot
not_assigned`.

Boundary:

- no file picker is opened
- no password is captured
- no wallet database is opened
- no `TspRequest` is submitted
- no filesystem read/write, Matrix request, gateway/runtime/auth, provider, or
  channel delivery mutation is emitted
- no import retry, cancel, persistence result, stale operation, or vault-open
  result is wired

Evidence:

- `apps/hepta-native/src/tsp/tsp_settings_screen.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### TSP Worker Receipt/Result Packet

Status: UI local result packet complete.

`TspSettingsScreen` now renders a local worker receipt/result packet whenever
the settings surface observes existing `TspWalletAction` or `TspIdentityAction`
results. The packet records the already-requested operation, request slot,
missing backend operation id, `Cx::post_action` receipt source, success/error/
canceled/stale taxonomy, local UI effect, retry slot, stale-result policy, and
audit redaction boundary.

Boundary:

- no new `TspRequest` is submitted from receipt handling
- no cancel/delete/import/remove behavior is added
- no filesystem delete, wallet database write beyond already requested worker
  paths, Matrix request, gateway/runtime/auth, provider, or channel delivery
  mutation is emitted
- raw password, token, private VID/key material, and raw wallet path are not
  copied into the compact packet

Evidence:

- `apps/hepta-native/src/tsp/tsp_settings_screen.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### TSP Pending Cancel Operation Packet

Status: UI local operation packet complete.

`CreateWalletModal` and `CreateDidModal` now replace the old plain pending
copy with a local pending-cancel packet while wallet or DID creation is in
flight. The packet records the missing backend operation id, a non-secret local
operation key, disabled cancel state, stale-result policy, password/secret
redaction, and relevant wallet/DID metadata.

Boundary:

- pending Cancel remains disabled
- no `TspRequest` cancel is submitted
- no wallet rollback, DID rollback, filesystem delete/write, Matrix request,
  gateway/runtime/auth, provider, or channel delivery mutation is emitted
- completion/error results still follow the existing worker result path

Evidence:

- `apps/hepta-native/src/tsp/create_wallet_modal.rs`
- `apps/hepta-native/src/tsp/create_did_modal.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh`

### TSP Association Cancel/Remove Packet

Status: UI local operation packet complete.

`TspVerifyUser` now renders a local association cancel/remove packet for
initiator cancel and Remove TSP Association states. The packet records the
missing backend request id, target/DID availability, disabled cancel/remove
state, persistence scope, receive-loop scope, stale-result policy, local result
taxonomy, persistence/receive-loop result slots, responder notification slot,
retry policy, and audit redaction before any association cancel/remove behavior
exists.

Boundary:

- no `CancelAssociateDidRequest` is submitted
- no `VerificationCancel` or `TspRequest` cancel is submitted
- no TSP state update, wallet database write, filesystem write, Matrix request,
  gateway/runtime/auth, provider, or channel delivery mutation is emitted
- the existing positive `AssociateDidWithUserId` request path is unchanged
- local taxonomy records local-only cancel, remote cancel, already answered,
  failed cancel, stale request, and remove-not-started states without sending a
  request

Evidence:

- `apps/hepta-native/src/tsp/verify_user.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh`

### Spaces Selected-Room Removed/Rejoin Packet

Status: UI local recovery packet complete.

`RoomsList` now keeps a visible selected-room removed/rejoin packet in the
status evidence row. Before any removed-room replacement or rejoin behavior is
promoted, the packet records the removed room id, membership state, active
selection match, FocusNone clear, selected-space scope, replacement UI slot,
rejoin request slot, and stale-event policy.

Boundary:

- no replacement room UI is opened
- no rejoin request body is built
- no `MatrixRequest::JoinRoomByIdOrAlias`, `LeaveRoom`, `Knock`, invite, or
  cancel-prior-knock request is submitted
- no message, room-state, membership, gateway/runtime/auth, provider, or
  channel delivery mutation is emitted

Evidence:

- `apps/hepta-native/src/home/rooms_list.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### Spaces Re-Knock/Cancel-Prior Packet

Status: UI local membership packet complete.

`AddRoomScreen` and `SpaceLobbyScreen` now render a local
re-knock/cancel-prior packet for already-knocked rooms or spaces. The packet
separates the existing confirmation-gated re-knock path from the still-missing
cancel-prior-knock request/result contract before any new membership behavior
is exposed.

Boundary:

- Add Room re-knock still uses only the existing `PositiveConfirmationModal`
  -> `MatrixRequest::Knock` path
- Space Lobby tree rows do not expose a new re-knock action
- no cancel-prior-knock request body or request id is built
- no `JoinRoom`, `LeaveRoom`, unconfirmed `Knock`, message, room-state,
  membership, gateway/runtime/auth, provider, or channel delivery mutation is
  emitted by the packet

Evidence:

- `apps/hepta-native/src/home/add_room.rs`
- `apps/hepta-native/src/home/space_lobby.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### Spaces Unread/Mention Aggregate Packet

Status: UI local aggregate packet complete.

`RoomsList` now renders a local People/Rooms unread/mention aggregate packet
from already loaded row state while keeping section header badges on the
existing local-zero placeholders. The packet records loaded direct/regular room
counts, loaded unread totals, loaded mention totals, manual-unread counts,
selected-space scope, filter state, header badge source, missing aggregate
refresh slot, partial parent-chain attribution, and undefined muted/low-priority
policy before any aggregate refresh or unread badge behavior is promoted.

Boundary:

- People/Rooms section header badge values remain local zero placeholders
- no aggregate refresh request body, cursor, or request id is built
- no read receipt, unread receipt, message, room-state, membership,
  gateway/runtime/auth, provider, or channel delivery mutation is emitted
- `JoinedSpaceInfo` `is:unread` / `is:mention` filtering still uses the
  room-display-filter zero source until an aggregate contract exists

Evidence:

- `apps/hepta-native/src/home/rooms_list.rs`
- `apps/hepta-native/src/room/room_display_filter.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### Spaces Room-List Load More Pagination Packet

Status: UI local pagination packet complete.

`RoomsList` now renders a local room-list Load More pagination packet in the
status evidence row. The packet records current loaded joined-room count,
displayed invite/people/rooms counts, server max hint when available,
selected-space child pagination state, direct child room/subspace counts,
missing explicit Load More button, cursor, in-flight, error, retry, and
exhaustion slots, plus the separate latest-message preview pagination source.

Boundary:

- no user-triggered Load More rooms button is rendered
- no explicit RoomListService page cursor or request body is built
- existing selected-space child pagination remains service-driven read-sync
- visible-row latest-message preview still uses the existing read-only
  `MatrixRequest::PaginateTimeline` path
- no message, room-state, membership, gateway/runtime/auth, provider, or
  channel delivery mutation is emitted

Evidence:

- `apps/hepta-native/src/home/rooms_list.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### Edit/Poll Answer Preview Result Packet

Status: UI local poll packet complete.

`RoomScreen` poll timeline rows now include a local poll answer preview/result
packet from already loaded `matrix_sdk_ui::timeline::PollState`. The packet
records answer count, total votes, max selections, open/closed state, edited
state, answer edit slot, vote-response slot, result mapping, stale poll policy,
and unsupported-capability boundary before any answer edit behavior exists.

Boundary:

- no poll response is sent
- no poll answer edit request body or result mapping is built
- no timeline reload, message, room-state, membership, gateway/runtime/auth,
  provider, or channel delivery mutation is emitted
- editing a poll still preserves existing answers and max selections while only
  editing the question through the existing confirmed edit path

Evidence:

- `apps/hepta-native/src/home/room_screen.rs`
- `apps/hepta-native/src/home/editing_pane.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-edit-poll-detail-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

### Edit Attachment Preflight Packet

Status: UI local attachment packet complete.

`EditingPane` now renders a local edit attachment preflight packet below the
main Edit/Poll detail packet. The packet records content kind, edited text
length, original attachment scope for image/audio/file/video caption edits or
non-media/poll edits, selected attachment availability, add/remove/replace
slots, upload request slot, media delete slot, caption-only EditMessage handoff,
MIME/size probe, retry policy, and cancel policy before any attachment edit
behavior exists.

Boundary:

- media message edits still only update body/formatted caption through the
  existing confirmed `MatrixRequest::EditMessage` path
- no `SendAttachment`, media delete, upload, timeline reload, room-state,
  membership, gateway/runtime/auth, provider, or channel delivery mutation is
  emitted
- cancel leaves the original media and any local selection untouched because no
  attachment selection/removal request is built

Evidence:

- `apps/hepta-native/src/home/editing_pane.rs`
- `apps/hepta-native/src/home/hepta_telegram_base_contract.rs`
- `scripts/hepta-native-edit-poll-detail-contract-gate.sh`
- `scripts/hepta-native-fixture-visual-smoke.sh`

## Remaining Edge Risks

### TSP Wallet Destructive/Import Actions

Status: backend contract spec/gate added; import has a local preflight packet;
delete wallet now has a local preflight/result taxonomy packet; worker actions
now have a local receipt/result packet; import result taxonomy is local; live
destructive/import/cancel implementation is still held.

Contract:

- `docs/architecture/HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md`
- `scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh`

Remaining work:

- delete wallet: preflight/result taxonomy is now visible locally; still define
  file deletion, open-vault closure, default-wallet fallback, persistence
  update, and failure recovery contracts before making
  `TspRequest::DeleteWallet` destructive
- import wallet: promote the local preflight packet to a backend/platform result
  only after file picker, wallet database open, password handling, duplicate
  metadata, typed import operation ids, and persistence result contracts are
  implemented; current result taxonomy remains local-only
- pending create-wallet/create-DID cancel: backend still needs stable operation
  ids and worker cancellation semantics before pending Cancel can become live
- association cancel/remove: backend still needs outbound verification cancel,
  local association removal, persistence, and remote notification semantics
  before live mutation; UI now has local result taxonomy and persistence/
  receive-loop result slots only

Next recommended slice:

Keep destructive/import actions held until backend/platform result contracts
exist. After the worker receipt and association result taxonomy packets, the
next UI-safe slice can be edit attachment preflight contract shape or another
local backend-result taxonomy packet, still not live deletion.

### Spaces And Room Membership Edges

Status: backend contract spec/gate added; selected-room removed/rejoin,
re-knock/cancel-prior, and unread/mention aggregate packets are local,
read/sync UI exists, and edge mutations stay local or implicit.

Contract:

- `docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md`
- `scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh`

Remaining work:

- live cancel-prior-knock for knocked rooms/spaces
- replacement/rejoin action when an already-open selected room disappears
- explicit room-list load-more controls if dynamic pagination becomes user
  visible beyond the current local packet
- live space unread/mention aggregate counters beyond the current local packet

Next recommended slice:

Revisit live Spaces aggregate or replacement/rejoin only after backend
aggregate source, parent-chain policy, and membership result taxonomy are
defined.

### Edit And Poll Detail Edges

Status: backend contract spec/gate added; edit save, history, source/diff UI,
the Edit/Poll detail packet, the poll answer preview/result packet, and the edit
attachment preflight packet are covered; save-result mapping and retry/error drilldown
now have local packets; the edit mention payload preflight and typed contract
shape are local-only; detail edits remain local.

Contract:

- `docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md`
- `scripts/hepta-native-edit-poll-detail-contract-gate.sh`

Remaining work:

- live edit attachment add/remove/replace after a typed upload/delete/result
  contract
- live mention extraction in edits
- live poll answer edits and persistent save spinner/result mapping
- live edit retry/error recovery after stable operation ids, late-result
  correlation, and retry idempotency exist

Next recommended slice:

Keep attachment edit, fresh mention payload extraction, poll answer edits, and
persistent edit operation ids held until typed payload/result contracts exist.
The next UI-safe local slice should move to another blocked edge or refine a
different edit result packet, without promoting edit retry, attachment mutation,
fresh mention payload extraction, or poll answer behavior live.

## Verification Gate

`scripts/hepta-native-non-base-edge-risk-plan-gate.sh` validates this plan
against the current code and, when a readiness directory is provided, the latest
readiness JSON.
