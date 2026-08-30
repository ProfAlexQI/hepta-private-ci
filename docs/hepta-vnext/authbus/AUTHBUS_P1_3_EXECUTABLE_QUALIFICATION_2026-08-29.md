# AuthBus P1.3 Executable Qualification Receipt

Date: 2026-08-30  
Stage: P1.3 canonical quota registry  
Disposition: **EXECUTABLE-QUALIFIED / QUALIFICATION-ONLY / NO AUTHORITY**

## Exact source binding

```text
repository=ProfHepta/hepta-private-ci
repository_id=1320694176
base_branch=integration/vnext-main-full-ci-authbus-p1-3-20260829
base_commit=6b7aa91d7702a92a50297b1b1bd8170ffb7cb184
branch=codex/authbus-p1-3-clean-replay-v9-split-slim-20260830
source_head=8572f3d2182541b14e0719b229ccd8754494f134
source_tree=aac769f278dad18b97b3c63c97f9b43dd325aa24
registry_sha256=dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1
workflow=.github/workflows/authbus-p1-3-qualification.yml
run_id=33306644612
run_attempt=1
```

This receipt is evidence only for the exact source commit and tree above. The
receipt commit contains exactly this Markdown file and the adjacent JSON receipt
relative to the source head. It does not change source, workflow, lockfile,
workspace membership, runtime wiring, or authority.

## Hosted execution evidence

Run `33306644612` completed successfully. Every required job had a non-zero
hosted runner ID, non-empty steps, terminal success, and the exact source head.

| Required job | Job ID | Runner ID | Result |
|---|---:|---:|---|
| Source contracts, exact delta formatting, and receipt provenance | 99244273750 | 1000043126 | PASS |
| P0.2 through P1.3 executable matrices | 99244494589 | 1000043137 | PASS |
| All-target cargo check | 99244494605 | 1000043136 | PASS |
| Strict Clippy `-D warnings` and final source revalidation | 99246066718 | 1000043467 | PASS |

The exact-source run passed:

- P0.2, P0.3, P1.1, P1.2 and P1.3 source and negative-authority verifiers;
- the committed Rust 1.95 resolver-3 lock graph;
- exact-delta Rust formatting;
- all inherited default-off tests;
- P0.2 SQLite WAL recovery, failpoint, corruption and outbox coverage;
- P0.3 canonical replay, six-dimensional reservation and old-fence reconcile coverage;
- P1.1 signed identity, purpose, epoch, revocation, nonce and evidence anti-replay coverage;
- P1.2 durable identity/evidence replay, stale-writer fencing, GC and reopen coverage;
- P1.3 canonical quota registry, projection, migration and lossy-downgrade coverage;
- every affected all-target `cargo check`;
- every affected all-target Clippy gate with `-D warnings`;
- final exact-source and registry-digest revalidation.

## Canonical quota closure

The qualified source owns one six-dimensional descriptor registry in
`codex-hepta-contracts`:

```text
request_count
rpm
tpm
concurrency
day_budget
context
```

Wire, SQLite limit/reserved/used, receipt and metric projections originate from
that registry. The B4 and P0.2 five-dimensional types remain explicit legacy
adapters. Missing `request_count` rejects by default; `AssumeOnePerPermit` must
be selected explicitly. A downgrade is accepted only when
`request_count == 1`; every other value fails as lossy.

The canonical wire model preserves `known`, `explicit_unknown` and
`not_declared`. Unknown or absent quantities are never converted silently to
zero or unlimited capacity.

## Artifact provenance

| Artifact | Artifact ID | Archive SHA-256 |
|---|---:|---|
| Source-contract evidence | 9730699520 | `5c5dd065605a0ad3597cc14da3b1d1ef2b655774e60d64016c8a2ebacd5f0c43` |
| P0.2 through P1.3 test matrices | 9730874523 | `cf258148c26efe15dea5e7c94051d8ba2239c071f6d1b8d98153036a717156ab` |
| All-target check evidence | 9730879950 | `efb54d186356f215092c1a3e99b18abe9393fbea31eeac1631067a3892c29a66` |
| Aggregate strict-Clippy evidence | 9730998239 | `4870bbbc2b16f30823410ea23046d6128a7598e97225e6c8b0a9ba5f5195f75e` |

The adjacent JSON receipt carries the same run, job, runner, registry and
artifact identities for machine verification.

## Authority boundary

Executable qualification closes the P1.3 development tranche but grants no
product, provider, secret, effect, operator, promotion or release authority:

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
listener_enabled=false
provider_call_enabled=false
openbao_enabled=false
parent_workspace_wired=false
private_key_storage=false
raw_signature_storage=false
secret_storage=false
```

P1.4, product workspace membership, listener/provider/OpenBao wiring,
production activation, merge, promotion and release remain separate,
explicitly unauthorized boundaries. They are not implied by this receipt.
