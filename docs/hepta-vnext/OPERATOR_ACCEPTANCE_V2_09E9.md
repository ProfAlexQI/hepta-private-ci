# Hepta vNext Head-Specific Operator Acceptance V2 (09e9)

## Status

This document specifies the prepared, fail-closed V2 ceremony for candidate:

```text
head  09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8
tree  bc14150f75cee49515e9bf244e15c526eb74e79e
bundle SHA-256 c6dca268010e98f759e15cde4009d8ebf49b413181ba122c3013ea6b3158d9a0
bundle size 177075328 bytes
parent 1 8a84ec2d76cd576f8f07eebd39764692c8bdd134
parent 2 / upstream cutoff c4b287cf5791d7f4336b925f7dfdb55ee4c3b668
```

V2 is prepared but **not accepted**. No 09e9 challenge has been generated, no
09e9 challenge has been signed, and no 09e9 nonce or acceptance receipt has
been consumed. The existing 3110 V1 acceptance, signer material, trust policy,
challenge, signature, nonce claim, and receipt are separate and unchanged.

At the time this V2 line was prepared, GitHub Actions for the exact candidate
was `BLOCKED_EXTERNAL`: every candidate job had zero executed workflow steps.
That receipt is evidence of an external blocker, not a platform PASS. The
implemented all-platform policy therefore keeps `prepare` closed even if native
Windows independently passes.

## Boundary

V2 accepts qualification evidence only. A valid V2 receipt sets only the
operator-acceptance fact to true. These values remain false:

```text
authority
enforce
outbound
promotion
qualification_authority
retirement
automatic_transition
```

It does not change a local or remote `main`, a repository default branch,
production services, state, releases, network policy, or any authority bit.

Natural-language approval is not a V2 signature. The program has no signing
subcommand. A formal acceptance can exist only after an external Ed25519 key
signs the exact canonical challenge bytes in namespace
`hepta-vnext-operator-acceptance-v2`, and the verifier durably consumes that
signature.

## Exact evidence graph

The exact aggregate root is:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-aggregate-qualification-v2
```

Its `SHA256SUMS` digest must be supplied positionally from an independent pin.
The manifest must contain canonical `qualification-packet.json`. The challenge
binds the aggregate root, manifest digest and entry count, packet digest, and
the complete typed packet.

The packet contains five required platform gates in this order:

1. `macos-aarch64`
2. `linux-x86_64`
3. `nix-x86_64-linux`
4. `windows-x86_64-native`
5. `github-actions`

Each gate binds a canonical receipt root, the SHA-256 of its `SHA256SUMS`, the
manifest entry count, a status artifact path and digest, execution facts, and
its PASS/exclusion state. The verifier inventories and hashes every file in
each receipt before interpreting its status artifact.

The policy is exact and cannot be relaxed by packet data:

```text
require_all_required_gates_pass=true
blocked_external_satisfies_required_gate=false
zero_step_execution_satisfies_pass=false
native_windows_substitutes_for_github=false
```

A PASS gate must have real candidate execution, at least one executed step, no
candidate failure, and `excluded_from_pass=false`. The only accepted non-PASS
shape is the exact GitHub `BLOCKED_EXTERNAL` classification, which must have
zero candidate execution, zero executed steps, no candidate failure,
`pass=false`, and `excluded_from_pass=true`. Because GitHub remains a required
gate, that exclusion is a blocker rather than a waiver.

The aggregate also binds and re-verifies the canonical path/trust prerequisite:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-canonical-path-trust-20260812T143920Z
SHA256(SHA256SUMS)=319c08c585a3cff07be504e78446240422bb476846fc20ed6b46d104f4acb20b
```

This prerequisite proves the 09e9 canonical integration ref, SSD UUID and
ownership, agent path instructions, Codex trust entries, independent UI tree,
and the fact that default `main` and production were not changed.

The second non-platform prerequisite freezes the upstream cutoff observation:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-upstream-cutoff-observation-20260812T1456Z
SHA256(SHA256SUMS)=67653bf2ef6bd035401d26e6f80d8af23e2deb14f91e763364c7876057fd11de
frozen cutoff=c4b287cf5791d7f4336b925f7dfdb55ee4c3b668
observed later upstream=9dd22890f5ff47e4af128c20e32b9758a61d78d2
post-cutoff commits=4
post-cutoff changed files=23
```

This receipt establishes that post-cutoff commits enter the next development
cycle backlog and do not move the exact qualification target. It is not a
platform gate and contributes nothing to the platform PASS count.

The V2 preparation artifact also inventories this read-only production plan:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-production-permission-remediation-plan-20260812T1501Z
SHA256(SHA256SUMS)=468906b1f9a5d51eb93b0d38b57fe030e56cb4f8f266544792cca0d3b387c55b
status=prepared_not_applied
offender_count=22
planned mode change=0444 -> 0400
```

This is `non_gate_readonly_plan` evidence only. It is intentionally outside the
platform and prerequisite PASS calculations. Its mode changes were not applied,
snapshot materialization remains false, execution authorization remains false,
and production remains unchanged.

The legacy product oracle is not inferred from the new binary. V2 reuses the
V1 verifier to fully re-inventory the exact 3110 qualification and frozen
2f704 product audit, then embeds the resulting frozen-product and oracle
bindings in the aggregate packet:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z
/Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build
```

This re-verification reads old V1 evidence. It does not alter or extend the old
3110 acceptance.

## Platform receipt interpretation

V2 checks the following status artifacts in addition to full manifest hashes:

- Mac: `qualification-status.txt`, exact 09e9 head/tree, exact-head fresh
  binary, isolated full-state canary, closed authorities, and no production or
  default-branch change. `steps.tsv` supplies the execution count. The current
  exact receipt manifest digest is
  `5caed37f7439f696c69b3b5c4f4979c7582fa0626d180a0f34129a85d6e6162b`.
- Linux: `result.txt`, exact head/tree, PASS, completed execution, verified
  postflight/source identity, clean worktree, and no production change.
  `steps.tsv` supplies the execution count.
- Nix: `result.txt`, exact head/tree, candidate and harness PASS, zero metadata,
  flake-check, build and output-verification return codes, fresh source/store,
  strict remote/T5 manifests, clean postflight, and no default/service change.
- Native Windows: `result.txt` with schema
  `hepta_vnext_windows_native_qualification_v2`, exact head/tree, completed
  native execution, candidate PASS, clean source/postflight, and no production
  change. `steps.tsv` supplies the execution count. This is separate from the
  GitHub gate.
- GitHub: `github-external-gate.json`. Every run must point to 09e9, contain a
  positive job count, have `zero_step_jobs == jobs`, and publish zero artifacts.
  PASS fields must all be false. The exact blocker receipt is:

```text
/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-github-external-20260812T143239Z
SHA256(SHA256SUMS)=821be15fa70583f4f56b9c41a0cd78af24767b21ce9007ee9c36dd2872dea7b3
```

## Formal CLI

Every formal command must run through the exact SSD lane:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance-09e9 -- \
  /absolute/frozen/tool/hepta-operator-acceptance-v2 assess \
  /Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-aggregate-qualification-v2 \
  <externally-pinned-aggregate-SHA256SUMS-sha256> \
  /Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build
```

`assess` is read-only. It returns a typed assessment including
`ready_for_challenge`, `blockers`, and `github_excluded_from_pass`.

The mutating ceremony entrypoint is:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance-09e9 -- \
  /absolute/frozen/tool/hepta-operator-acceptance-v2 prepare \
  <aggregate-root> <aggregate-SHA256SUMS-sha256> <legacy-product-audit-root> \
  <v2-sidecar-root> <allowed-signers> <v2-trust-policy> <v2-trust-policy-sha256>
```

`prepare` verifies the full evidence graph before it creates the sidecar lock,
time watermark, or challenge. If any required gate is not PASS, it returns an
error naming the blockers and leaves the sidecar without new ceremony files.

`verify` adds a final detached-signature argument. `verify-receipt` omits that
argument and performs read-only replay verification against the embedded SSHSIG
envelope. The file names are:

```text
operator-acceptance-v2-challenge.json
operator-acceptance-v2-nonce-claim.json
operator-acceptance-v2-receipt.json
operator-acceptance-v2-time-watermark.json
```

The V2 sidecar must be an effective-user-private directory directly under
`/Volumes/T5/hepta-vnext/artifacts/acceptances` and have a name beginning with
`vnext-main-09e9e9ff7f-operator-acceptance-v2`. Evidence, sidecar, signer, and
trust-policy paths must be absolute, canonical, symlink-free, privately owned,
and disjoint. Manifest inventories reject extra files, symlinks, hardlinks,
special files, metadata changes during hashing, digest changes, and
`SUPERSEDED.txt` platform receipts.

## Trust and canonical bytes

V2 requires an independently pinned canonical trust policy with:

```text
schema=hepta_operator_acceptance_trust_policy_v2
schema_version=2
trust_policy_scope=externally_pinned_single_ed25519_external_revocation_responsibility_no_local_krl_v2
namespace=hepta-vnext-operator-acceptance-v2 (implied by the verifier)
```

The remaining signer constraints match V1: exactly one LF-terminated raw
Ed25519 allowed-signer line, exact principal, independently pinned policy
digest, exact allowed-signers digest and key fingerprint, a nonzero trust-root
revision, and a challenge lifetime of at most 900 seconds and no more than the
external policy maximum. V2 does not create or copy a trust policy or private
key.

Challenge, receipt, nonce claim, watermark, trust policy, and aggregate packet
use recursively key-sorted compact JSON with no trailing newline. Typed reads
deny unknown fields and require byte-for-byte canonical reserialization.

V2 retains the V1 crash boundary: the nonce claim is durably written before the
acceptance receipt. A claim without a receipt fails closed. Exact replay returns
the same receipt only after signature, challenge, trust, evidence, nonce, time,
and authority bindings are all re-verified.
