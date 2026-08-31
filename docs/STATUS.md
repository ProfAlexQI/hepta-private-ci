# Hepta Current Development Status

**Observed:** 2026-08-31 08:01:10 UTC  
**Scope:** discovery only; this document grants no runtime, production, operator, promotion, or release authority.  
**Machine source:** [`CURRENT.json`](CURRENT.json)

## Default baseline

The default branch is `integration/vnext-main-20260811` at commit `b621768b70a09d56626bb8a2c331e3dc424e6a4d`. It is a verification-mirror baseline, not the selected production release.

## Latest architecture-plan candidate

The latest reviewed plan candidate is `HEPTA-ARCHITECTURE-CONVERGENCE-V5` version `5.0.1` on:

```text
branch = codex/hepta-architecture-v5-b0-exact-restack-20260831
commit = ad7845a8d67390299f86e931bab11d8b0ec13115
tree = 81a07047a20d107aafad1670f670a904effcf514
```

It has not been selected into the default branch and its exact executable qualification is not proven.

## Active architecture stack

| Package | Exact source | Current truthful state |
|---|---|---|
| P0.7a signed runtime bootstrap | `92d22e241972fd02f2a3a0bf69849b0b4c7a8b7f` | Source implemented; observed qualification runs cancelled; no activation |
| P0.7b/B0 verified-use kernel | `ad7845a8d67390299f86e931bab11d8b0ec13115` | Source implemented; observed qualification cancelled; no activation |
| P0.7b/B1a provider boundary | `537394a0067d204b215db8bee3de533494535481` | Source implemented; exact qualification not proven; no product caller |
| P0.7b/B1b model boundary | `6c876b2c259ccabef928c7921c2037b3c10f051b` | Materializer exists, but final source is absent after a compressed-payload checksum failure |
| B1b read-only recovery | `cffd615f559fdf6f4f143fb7d5b50b2512b9e36c` | Portable recovery lanes queued; queued work is not evidence |
| AuthBus P1.3 quota semantics | `8572f3d2182541b14e0719b229ccd8754494f134` | Executed test inventory passed; semantic qualification remains blocked |

## AuthBus semantic blockers

The current P1.3 candidate still requires:

1. window-keyed and revision-bound accounting for RPM, TPM, and daily budget;
2. per-request context enforcement without aggregate terminal accumulation;
3. reservation conservation and state-specific vector rules;
4. canonical window-kind/interval binding and recomputed digest-chain verification;
5. adversarial multi-window, repeated-context, stale-window, conservation, transition, and tamper tests.

## Evidence policy

A PASS requires an exact candidate, an assigned runner, non-empty attributable steps, successful required gates, and any separately required independent decision. The following are not PASS:

- queued, cancelled, timed-out, skipped, stale, or superseded runs;
- `runner_id=0` or empty steps;
- source-only verification;
- generated source or an uploaded artifact by itself;
- a Draft PR description or label;
- a self-issued independent-review, operator, promotion, or release claim.

## Authority posture

Every authority flag in `CURRENT.json` is false. The repository-controlled work may close source and executable gaps, but independent review, physical-device or real-provider evidence, operator acceptance, production trust-root ceremony, promotion, and release must be issued by their designated external actors.
