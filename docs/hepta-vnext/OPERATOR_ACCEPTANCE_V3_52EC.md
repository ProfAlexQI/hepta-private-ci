# Hepta vNext 52ec aggregate qualification V3

This qualification-only tool is developed on the local branch
`agent/hepta-vnext-operator-acceptance-v3-52ec-20260813` in the SSD-owned
worktree `operator-acceptance-52ec-v3`. The branch is based directly on the
immutable candidate, but is not part of it and must not be merged into it:

- candidate HEAD: `52ec4b3868fc5272e19ed516d00e11e44c549ea4`
- tree: `247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d`
- parents: [`32fb822ccc4eda7949b0fc4101f594604e31f282`]
- integration merge: `8b60a902b537a1b01f7580327bcf08317f9a145a`
- upstream cutoff: `74004b5397b24662a87a5264a6ae80664168c7f3`

The upstream cutoff is independent of Git parentage. V3 does not invent a
second parent for this single-parent candidate.

The replayed 09e9 V2 module, binary, tests, and document exist only as an
auditable implementation provenance layer. Their formal environment, receipt
roots, receipt digests, candidate topology, namespace, and ceremony commands
are explicitly inapplicable to 52ec and must not be invoked for this candidate.
V3 carries no fixed platform receipt root or digest; those bindings enter only
through a later externally pinned build spec after each final receipt exists.

## Scope and authority

V3 implements only:

1. fail-closed aggregate build planning;
2. explicit one-shot aggregate construction and sealing;
3. full aggregate and source-evidence verification; and
4. read-only qualification assessment.

It intentionally has no command that creates or verifies an operator
challenge, detached signature, nonce claim, acceptance receipt, trust policy,
Git ref, production transition, or GitHub run. It never reads a private key.

Every aggregate packet carries `automatic_transition=false` and an entirely
closed authority boundary. Even a five-gate PASS packet grants no authority for
operator acceptance, promotion, Enforce, outbound activity, default-branch
switching, production cutover, rollback, recutover, or retirement.

## Required evidence graph

All three prerequisites are required, unique, non-superseded, and canonically
ordered:

1. `portable-inputs`
2. `canonical-path-trust`
3. `upstream-cutoff-observation`

Portable inputs own the candidate materialization binding. The exact bundle is
`candidate-52ec4b3868.bundle`, SHA-256
`cd27e0b0a7bbbb14fd78183b1ffe5aa5ea9fb7d187a08ce381305f29f8d7feb3`,
size 176,335,964 bytes. Linux is not treated as the bundle owner.

All five platform gates are required and canonically ordered:

1. `macos-aarch64`
2. `linux-x86_64`
3. `nix-x86_64-linux`
4. `windows-x86_64-native`
5. `github-actions`

`BLOCKED_EXTERNAL` is an assessable GitHub state, but never satisfies the
required GitHub gate. Native Windows cannot substitute for GitHub. A PASS gate
must bind nonzero candidate execution, qualification PASS, no candidate or
harness failure, and no refs or production mutation.

## Explicit manifest and semantic bindings

The canonical build spec is externally digest-pinned and uses
`deny_unknown_fields` throughout. Each receipt explicitly binds:

- canonical receipt root;
- ordered manifest layers, each with root-relative path, manifest filename,
  manifest kind, manifest SHA-256, and exact entry count;
- status or supporting artifacts with format, relative path, digest, and exact
  field assertions; and
- semantic claims whose selectors must also appear among those exact
  assertions.

A direct receipt normally has one `SHA256SUMS` layer. A nested attempt whose
outer layer is `ATTEMPT.sha256` is required to bind the inner
`receipt/SHA256SUMS` layer as well. Both layers verify complete inventories,
not selected files. Extra files, missing files, unsafe paths, symlinks,
hardlinks, special files, changed metadata, digest drift, duplicate roots, and
`SUPERSEDED.txt` fail closed.

This generic mechanism permits platform-specific status schemas while keeping
the candidate, verdict, execution, authority, and mutation invariants typed and
mandatory. Final platform roots and digests must be supplied only after those
receipts are atomically sealed; prepared driver roots are not gate evidence.

## CLI and one-shot boundary

Every formal command must run through the exact lane:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance-52ec-v3 -- \
  /absolute/frozen/tool/hepta-operator-acceptance-v3 build-plan \
  /absolute/canonical/build-spec.json <externally-pinned-spec-sha256> \
  /Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-aggregate-qualification-v3-<nonce>
```

`build-plan` verifies the whole evidence graph and prints the exact files that
would be created. It does not create the output root. Mutation requires the
separate explicit form:

```sh
hepta-operator-acceptance-v3 build --execute \
  <canonical-build-spec.json> <externally-pinned-spec-sha256> <new-output-root>
```

The output root must be a nonexistent immediate child of the canonical receipt
store and begin with
`vnext-main-52ec4b3868-aggregate-qualification-v3`. Reuse is rejected. The
builder writes canonical `build-spec.json`, `qualification-packet.json`, and
`aggregate-build-record.json`, records exact modes, seals `SHA256SUMS`, makes
the artifacts read-only, and re-verifies the complete source evidence graph
before returning its manifest digest.

`verify-aggregate` independently re-verifies the aggregate manifest, modes,
canonical JSON, build record, every source receipt, and the recomputed packet.
`assess` performs that same read-only verification and only reports
`ready_for_challenge=true` when all five gates and all three prerequisites are
PASS.

## Deferred operator ceremony

This implementation does not prepare an operator ceremony. After a future
five-gate PASS aggregate has been independently pinned and assessed, a separate
head-scoped ceremony implementation and a fresh explicit operator
authorization are still required. No V1 or 09e9 V2 challenge, nonce,
signature, trust-policy digest, or acceptance receipt is reusable. Acceptance
itself remains evidence-only and does not authorize later ref or production
steps.
