# AuthBus P0.2 SQLite WAL implementation status

**Plan:** `AUTHBUS-P0-PLAN-2026-08-28`  
**Tranche:** P0.2  
**Stack base:** `integration/vnext-main-full-ci-authbus-p0-1-20260828`  
**Development branch:** `integration/vnext-main-full-ci-authbus-p0-2-20260828`  
**Current decision:** `SOURCE_PRESENT / EXECUTABLE_QUALIFICATION_RUNNING / NO_AUTHORITY`

## Repository truth

P0.2 is an isolated, default-off SQLite WAL qualification coordinator. It is not part of the parent product workspace and does not expose a listener, provider-call path, OpenBao integration, production caller, production writer, or effect authority.

The branch now contains:

- the qualification crate and committed Rust 1.95 dependency lock;
- SQLite migrations and the durable coordinator implementation;
- the SQLite WAL crash/recovery regression matrix;
- the fail-closed source/authority verifier;
- the exact-head hosted qualification workflow;
- no temporary `_probe` files and no one-shot bootstrap workflow.

## Deterministic dependency and formatting closure

The nested workspace is bound to:

```text
rust-version = 1.95
resolver = 3
committed Cargo.lock
qualification commands use --locked
package-scoped rustfmt
```

Hosted bootstrap run `33155983901` executed on a real assigned runner and passed every step:

```text
exact checkout and ancestry                     PASS
source and negative-authority gate              PASS
Rust 1.95 toolchain                             PASS
deterministic Cargo.lock generation             PASS
package-scoped rustfmt                           PASS
changed-path allowlist                          PASS
atomic force-with-lease commit                   PASS
```

It produced commit `35627354397a281e432061edfc681e9ec286b0e7`. The temporary bootstrap workflow was then removed by user-authored commit `2ca865693cba67c8fa28e334078fe9eb69e029d7` so the final candidate can obtain normal exact-head Actions evidence rather than GitHub's bot-recursion `action_required` result.

Bootstrap success is not itself the P0.2 executable qualification receipt. The final user-authored exact head still has to pass the complete source/fmt/test/check/Clippy matrix with non-empty job steps.

## Implemented durable control plane

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

The coordinator uses the repository SQLite durable-evidence pool with WAL and full synchronous durability. Row digests, `quick_check`, writer boot/generation, operation revision, status revision, observation time, binding digest, and fences all fail closed.

## Exact executable qualification commands

```bash
python3 scripts/verify-authbus-p0-2.py

cargo fmt \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --package codex-hepta-authbus-qualification \
  -- --check

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --no-default-features --lib -- --nocapture

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --tests -- --nocapture

cargo check --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --all-targets

cargo clippy --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --all-targets -- -D warnings
```

## Definition of done

P0.2 becomes executable-qualified only when one exact commit/tree has evidence for all of the following:

- default feature set keeps the coordinator inactive;
- source and negative-authority gate passes;
- package-scoped formatting passes;
- committed lock graph is unchanged under `--locked`;
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
parent_workspace_wired=false
```

P0.3 may remain stacked for review, but it must not be promoted, merged, or interpreted as production evidence until the P0.2 exact base receives executable qualification.
