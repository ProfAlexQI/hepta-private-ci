# Hepta Native TSP Wallet Destructive/Import Backend Contract - 2026-06-15

This contract captures the backend/platform requirements before Hepta Native can
promote TSP wallet delete/import and association removal from visible local
blocked controls to live behavior.

The current UI intentionally keeps these paths non-destructive:

- `WalletEntry` Delete Wallet shows loaded wallet metadata and emits only local
  blocked evidence plus a local preflight/result taxonomy packet.
- `TspRequest::DeleteWallet` is accepted by the worker but stays ignored and
  warning-only.
- `TspSettingsScreen` Import Existing Wallet shows local blocked metadata and
  opens a local preflight packet for picker/password/vault/persistence/duplicate
  boundaries while starting no file picker, vault open, password capture, or
  persistence write.
- Import Existing Wallet also renders a local result taxonomy packet for picker,
  authentication, vault-open, metadata, duplicate, persistence, retry, cancel,
  stale-operation, and audit-redaction states while `operation_id_slot` remains
  unassigned.
- `TspSettingsScreen` keeps a local worker receipt/result packet for already
  existing `TspWalletAction` and `TspIdentityAction` results, but it starts no
  new request and adds no retry/cancel/delete/import/remove behavior.
- create wallet / create DID pending Cancel remains disabled while worker
  cancellation semantics are undefined, but both modals now render local
  pending-cancel operation packets with operation-id and stale-result contract
  gaps.
- initiator-side association Cancel and Remove TSP Association remain local
  blocked evidence, now with an association cancel/remove operation packet.

## Delete Wallet Contract

Before `TspRequest::DeleteWallet` can delete anything, backend/platform must
provide a result contract with:

- wallet identity: wallet name, canonical sqlite URL/path, role
  default/secondary, opened/not-found state
- preflight: path exists, regular file, app-owned or explicitly user-selected,
  not shared with another loaded wallet, and deletion scope is a single wallet
  database file unless an explicit multi-file vault manifest exists
- closure: opened wallet is persisted and closed or proven safe to drop before
  filesystem deletion
- default fallback: deleting the default wallet either blocks, promotes a
  selected replacement, or leaves TSP disabled with a visible recovery state
- persistence result: saved TSP state removes exactly the deleted metadata and
  keeps unrelated wallets, identities, and associations intact
- filesystem result: deleted, already missing, permission denied, busy, not
  app-owned, and partial failure states are distinct
- retry/cancel: retry is confirmation-gated and idempotent; cancel sends no
  delete request
- audit: result labels contain no password, token, DID secret, private VID, or
  key material

Forbidden side effects:

- no Matrix request
- no gateway/runtime/provider/auth/channel delivery call
- no unrelated wallet database write
- no DID publication or association mutation
- no deletion outside the verified wallet path

Current UI delete preflight/result packet:

- wallet_identity: wallet name, path availability, opened/not-found state, and
  default/secondary role from the loaded row only
- path_validation_slot: backend_required_exists_regular_app_owned_single_scope
- ownership_scope: backend_required
- open_wallet_closure_slot: backend_required_close_or_prove_safe for opened
  wallets; not_open_or_not_found for missing wallets
- default_fallback_slot: backend_required_block_promote_or_disable for the
  default wallet; not_required_secondary_wallet for secondary wallets
- persistence_result_slot: not_started
- filesystem_result_taxonomy: deleted, already_missing, permission_denied, busy,
  not_app_owned, and partial_failure
- retry_cancel_policy: confirmation_gated_idempotent_retry_cancel_sends_no_request
- audit_redaction_policy: no password, token, DID secret, private VID, or key
  material
- side-effect boundary: no `TspRequest::DeleteWallet`, filesystem delete, wallet
  database write, TSP state mutation, Matrix request, gateway/runtime/auth, or
  live mutation

## Import Wallet Contract

Before Import Existing Wallet can open arbitrary wallet paths, backend/platform
must provide a result contract with:

- picker result: selected path, canceled path, inaccessible path, and unsupported
  URL scheme states
- authentication: password is handled only in modal-local memory and is never
  written to contract artifacts, logs, popup text, or readiness JSON
- vault open result: opened, invalid password, unsupported vault, corrupted
  database, already imported, duplicate path, and permission-denied states
- metadata result: wallet name, sanitized path label, default/secondary role,
  opened/not-found state, and duplicate handling
- persistence result: saved TSP state records the imported metadata only after a
  successful open or an explicit not-found import policy
- retry/cancel: retry reuses only the selected path and asks for fresh password
  input; cancel closes the modal and sends no TSP worker request
- audit: source/result labels redact password and secret/key material

Current UI preflight packet:

- picker_result: not_started
- selected_path: unavailable
- password_state: not_collected
- vault_open: not_started
- persistence_result: not_started
- duplicate_policy: loaded wallets require backend duplicate checks before
  import
- acknowledge/close: local UI dismissal only; no file picker, password capture,
  wallet database open, `TspRequest`, filesystem read/write, Matrix request,
  gateway/runtime/auth, or live mutation

Current UI import result taxonomy packet:

- operation_id_slot: not_assigned
- picker_result: canceled, selected_path_unavailable, inaccessible_path, and
  unsupported_url_scheme not_wired
- auth_result: password_not_collected, invalid_password, and
  redacted_retry_required not_wired
- vault_open_result: opened, invalid_password, unsupported_vault,
  corrupted_database, already_imported, duplicate_path, and permission_denied
  not_wired
- metadata_result: wallet_name_sanitized_path_default_role not_started
- duplicate_result: not_started_no_loaded_wallets or
  not_started_loaded_wallets_require_duplicate_check
- persistence_result: saved, duplicate_blocked, failed, and stale_operation
  not_started
- retry_policy: selected_path_reused_password_fresh_backend_required
- cancel_policy: local_dismiss_no_request
- stale_result_policy: backend_operation_id_required_before_import_live
- audit_redaction_policy: no_password_token_private_vid_key_material_raw_path
- side-effect boundary: no file picker, password capture, wallet database open,
  `TspRequest`, filesystem read/write, Matrix request, gateway/runtime/auth, or
  live mutation

Forbidden side effects:

- no wallet file copy/move/delete without explicit separate confirmation
- no Matrix request
- no gateway/runtime/provider/auth/channel delivery call
- no account/profile/room-state/membership mutation
- no implicit SetDefaultWallet unless selected and confirmed

## Worker Receipt/Result Contract

Before any TSP worker operation can expose retry, cancel, rollback, or
destructive result handling, backend/platform must provide a typed result
contract with:

- stable operation id for create wallet, create DID, open wallet, set default,
  remove wallet, delete wallet, republish DID, association send, association
  response, and receive-loop failures
- receipt source and target identity: existing `Cx::post_action` source, wallet
  row identity, DID/user target availability, and action owner
- result taxonomy: success, local validation error, backend error, canceled,
  stale-result-ignored, already-completed, retryable, and non-retryable states
- retry/cancel mapping: retries must reuse only redacted stable operation
  metadata; cancel must not race a later worker result into current UI state
- stale policy: worker results are applied only when operation id and local
  screen cache/target still match
- audit: no password, token, private VID, DID secret, key material, or raw
  wallet path in persistent artifacts or compact result labels

Current UI worker receipt/result packet:

- operation: one of the already requested wallet/identity worker paths
- request_slot: existing `TspRequest` or receive-loop action source
- operation_id_slot: not_assigned
- worker_receipt: `Cx_post_action`
- result_state: success/error/canceled/stale local taxonomy
- target: wallet/identity loaded-state summary only
- ui_effect: local redraw, popup, button restore, modal-owned result, or
  profile-widget state
- retry_slot: existing_guarded_paths_only
- stale_result_policy:
  local_screen_cache_match_only_backend_operation_id_required_for_cancel_or_retry
- audit_redaction_policy: no password, token, private VID, or key material
- side-effect boundary: no new `TspRequest`, no cancel/delete/import/remove
  behavior beyond already confirmed existing paths, no filesystem delete,
  Matrix request, gateway/runtime/auth, or live mutation

## Pending Creation Cancel Contract

Before Create Wallet or Create DID pending Cancel can become live:

- each worker request needs a stable operation id
- cancel result must distinguish not-started, canceled, already-completed,
  failed-to-cancel, and stale-result-ignored
- completed worker results arriving after cancel must be ignored or reconciled by
  operation id without mutating current UI state unexpectedly
- Create DID cancellation must not leave a partially published DID silently
  selected as default

Current UI pending packet:

- operation_id: missing_backend_contract
- local_operation_key: non-secret wallet/DID input metadata only
- cancel_state: disabled_no_request
- stale_result_policy: backend_operation_id_required
- password/secret redaction: true
- side-effect boundary: no `TspRequest` cancel, wallet rollback, DID rollback,
  filesystem delete/write, Matrix request, gateway/runtime/auth, or live
  mutation

## Association Cancel/Remove Contract

Before TSP association cancel/remove can become live:

- outbound association request ids or verification detail hashes must be stable
- cancel result must distinguish local-only cancel, remote cancel sent, already
  answered, failed cancel, and stale request
- remove association must specify whether it removes only the local Matrix user
  mapping, verified VID alias in wallet storage, pending receive-loop state, or
  all related local artifacts
- persistence and receive-loop side effects must be explicit and idempotent

Current UI association packet:

- request_id: missing_backend_contract
- local_association_key: non-secret target/DID availability only
- cancel_state: disabled_no_request
- persistence_scope: backend_required
- receive_loop_scope: backend_required
- stale_result_policy: backend_request_id_required
- result_taxonomy: local_only_cancel_not_sent, remote_cancel_not_sent,
  already_answered_local_state, failed_cancel_not_started,
  stale_request_blocked, remove_not_started
- persistence_result_slot: not_started
- receive_loop_result_slot: not_started
- responder_notification_slot: not_sent
- retry_policy: blocked_until_backend_request_id
- audit_redaction: target_did_presence_only
- side-effect boundary: no `CancelAssociateDidRequest`, `VerificationCancel`,
  `TspRequest` cancel, TSP state update, wallet database write, filesystem
  write, Matrix request, gateway/runtime/auth, or live mutation

## Acceptance Gate

`scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh` validates:

- this contract exists and names delete/import/worker-result/pending-cancel/
  association groups
- current code still exposes visible blocked metadata instead of unsafe
  destructive behavior
- Delete Wallet exposes the local preflight/result taxonomy packet while still
  submitting no delete request
- Import Existing Wallet exposes the local preflight packet while still
  submitting no import request
- Import Existing Wallet exposes the local result taxonomy packet while still
  submitting no file picker, password capture, wallet database open,
  `TspRequest`, or persistence request
- worker actions expose the local receipt/result packet while still submitting
  no new request from receipt handling
- `TspRequest::DeleteWallet` worker path remains warning-only until this
  contract is implemented
- readiness remains product-ready with backend contract gates ready when a
  readiness directory is supplied
