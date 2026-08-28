# AuthBus P0.2 SQLite WAL implementation status

**Plan:** `AUTHBUS-P0-PLAN-2026-08-28`  
**Tranche:** P0.2  
**Stack base:** `integration/vnext-main-full-ci-authbus-p0-1-20260828`  
**Development branch:** `integration/vnext-main-full-ci-authbus-p0-2-20260828`  
**Status at source publication:** `SOURCE_PRESENT / QUALIFICATION_PENDING / NO_AUTHORITY`

## Corrected repository truth

The P0.2 branch previously contained the SQLite source files but no dedicated tests, workflow, status document, or pull request, and retained two temporary `_probe` files. Those facts are not equivalent to a completed or qualified tranche.

This completion commit:

- removes both probe files;
- adds a dedicated SQLite WAL regression matrix;
- adds a fail-closed source/authority gate;
- adds an exact-head hosted qualification workflow;
- documents the precise authority and evidence boundary.

Until a real runner records non-empty steps for the exact commit, all Rust results remain unknown. A job with `runner_id=0`, an empty runner name, or `steps=[]` is an infrastructure blocker, neither PASS nor a code failure.

## Implemented scope

The qualification crate models one durable control plane containing:

```text
admission + quota HELD + token-family claim
  -> operation intent durable
  -> dispatch-attempt durable
  -> provider-boundary ticket may be returned
  -> accepted / completed / verified-no-effect / unknown marker
  -> status-by-effect-key reconciliation
  -> quota complete or release
  -> claim release on terminal evidence
  -> outbox delivery + cursor CAS
```

SQLite tables:

```text
authbus_p0_2_meta
operations
token_family_claims
quota_reservations
dispatch_attempts
status_observations
outbox
outbox_cursor
fsync_receipts
```

The coordinator uses the repository SQLite shim with WAL and full synchronous durability. Row digests, `quick_check`, writer boot/generation, operation revision, status revision, observation time, and fences fail closed.

## Required executable qualification

```bash
python3 scripts/verify-authbus-p0-2.py
cargo fmt --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --no-default-features --lib -- --nocapture
cargo test --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --tests -- --nocapture
cargo check --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --all-targets
cargo clippy --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml --features sqlite-qualification --all-targets -- -D warnings
```

## P0.2 definition of done

P0.2 becomes executable-qualified only when one exact commit/tree has evidence for all of the following:

- default feature set keeps the coordinator inactive;
- source gate passes;
- formatting passes;
- all SQLite WAL tests pass;
- all-target check passes;
- Clippy passes with warnings denied;
- exact replay and changed-binding conflict pass;
- attempt-durable reopen returns lookup-only;
- writer/fence/status anti-replay tests pass;
- transaction failpoints leave no partial state;
- corruption is detected;
- no raw-secret schema names exist;
- every authority field remains false.

## Authority boundary

```text
qualification_only=true
authority=false
effect_authority=false
production_caller=false
production_writer=false
operator_acceptance=false
promotion=false
g5_allowed=false
execute_allowed=false
listener=false
provider_call=false
OpenBao_integration=false
```

P0.3 may be source-stacked for review, but it must not be promoted, merged, or interpreted as production evidence until the P0.2 exact base receives executable qualification.
