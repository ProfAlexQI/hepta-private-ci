# AuthBus P1.3 Semantic Closure V12

**Date:** 2026-08-31  
**Parent:** `8572f3d2182541b14e0719b229ccd8754494f134` / tree `aac769f278dad18b97b3c63c97f9b43dd325aa24`  
**Branch:** `codex/authbus-p1-3-semantic-closure-v12-gpt56-20260831`  
**State:** source implemented; exact-head execution, merge-candidate execution, and independent semantic acceptance remain pending.

## Problem corrected

The predecessor's executed test inventory passed, but review correctly rejected semantic qualification because its selected scheduler used one global `used`/`held` vector for RPM, TPM, and daily budget; accumulated per-request context into terminal global spend; and did not prove reservation conservation, canonical window identity, state-transition legality, or a recomputed digest chain.

V12 does not relabel that earlier evidence. It adds a separate qualification-only semantic kernel and adversarial test matrix.

## Source design

The qualification kernel provides per-request context enforcement while keeping terminal context outside aggregate spend.

`WindowedQuotaLedger` uses window-keyed accounting and separates the six canonical dimensions by lifecycle:

| Dimension | Lifecycle in V12 |
|---|---|
| `request_count` | cumulative used + held across all windows |
| `rpm` | exact UTC-minute key with independent used + held |
| `tpm` | exact UTC-minute key with independent used + held |
| `concurrency` | active hold only; released at terminal settlement |
| `day_budget` | exact UTC-day key with independent used + held |
| `context` | per-request upper bound; retained in terminal evidence but never accumulated globally |

Every window key binds quota domain, dimension, interval kind, canonical aligned start/end, and non-zero policy revision. RPM and TPM must bind the same minute. An observation outside its exact interval, a stale interval, a wrong kind, domain drift, or policy-revision drift fails closed.

## Reservation and recovery contract

A reservation binds:

- reservation ID and idempotency key;
- exact quota domain;
- final payload SHA-256;
- policy SHA-256 and policy revision;
- estimate and explicit safety margin;
- all three exact window keys;
- issued/expiry times;
- expected global ledger revision.

The state graph is deliberately bounded:

```text
insert → Held
Held → DispatchAttempted | Released | ExpiredPreDispatch
DispatchAttempted → Indeterminate | Completed | Released
Indeterminate → Completed | Released
```

Post-dispatch expiry is forbidden. `Indeterminate` remains lookup/reconcile-only. No blind retry path is introduced.

## Conservation and digest chain

Each projection carries immutable `held`, terminal `consumed`, and `remaining` vectors, with:

```text
held = consumed + remaining
```

Active and released states require zero consumed and complete remaining. Completed states require consumed to fit within the admitted hold and require terminal concurrency to be zero.

Every transition is appended to one global revision chain binding reservation identity, prior and next state, all three vectors, window binding digest, payload digest, policy digest, prior digest, and transition revision. The invariant verifier independently recomputes the chain and all aggregate counters from retained reservations.

## Adversarial qualification matrix

The focused suite covers:

1. same-minute RPM/TPM exhaustion and next-minute isolation;
2. same-day budget exhaustion and exact next-day rollover;
3. repeated maximum-context requests without aggregate context consumption;
4. context above the per-request limit rejected before counters change;
5. stale, wrong-kind, wrong-domain, and wrong-policy window rejection;
6. stale revision and changed idempotency binding rejection;
7. held/consumed/remaining conservation and tamper detection;
8. invalid transitions and forbidden post-dispatch expiry;
9. transition-chain digest tampering;
10. cumulative request-count enforcement across window rollover;
11. unknown-limit and authority-escape rejection.

## Qualification boundary

The source must still pass, on the exact source head and GitHub merge candidate in separately named, separately retained lanes:

```text
static source verifier
Rust 1.95 format
default-off posture
focused P1.3 semantic tests
complete P1.3 qualification tests
all-target cargo check
strict Clippy -D warnings
clean worktree
independent semantic review
```

The result cannot enable a listener, provider call, OpenBao path, parent-workspace wiring, production writer, external effect, G5, operator acceptance, promotion, or release. All such authority remains false.
