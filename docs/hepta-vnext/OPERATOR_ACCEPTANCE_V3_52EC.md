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
V3 revision 8 preserves every revision-7 admissible final PASS receipt
identity: canonical
root, outer and (where applicable) inner manifest digest and entry count, and
mode/type inventory digest. A build spec may repeat those identities but can
never choose or replace them. An uncompiled identity fails before the tool
reads a spec-selected receipt root. The exact profile-set identifier is
`hepta_vnext_52ec_evidence_profiles_v3_revision_8`.

At this revision, canonical path trust attempt 4, the upstream-cutoff
observation, and independently audited provenance-preserving Mac, portable,
and Nix wrappers have compiled final identities. Their untouched legacy roots
also retain separately compiled original identities. Linux, Windows, and
GitHub final identities remain unpinned. Therefore no formal aggregate can
currently be built or become challenge-ready.

`build --execute` is stricter than planning: any nonempty blocker set aborts
before an incoming directory or formal aggregate output is created. Diagnostic
NO-GO material is not published under the formal aggregate namespace.

## Scope and authority

V3 implements only:

1. fail-closed aggregate build planning;
2. explicit one-shot aggregate construction and sealing;
3. full aggregate and source-evidence verification; and
4. read-only qualification assessment.

It intentionally has no command that creates an aggregate operator challenge,
detached signature, nonce claim, acceptance receipt, trust policy, Git ref,
production transition, or GitHub run, and it cannot execute any of them. It
never reads a private key. Historical Linux V5 challenge, signature,
authorization, and trust-policy parsing remains available for negative tests,
but the profile is permanently unpinned: none of those inputs can authorize an
execution or reach receipt validation, and they grant no aggregate ceremony
authority.

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

The GitHub-hosted v2 semantic profile and prepared-driver identity are
compiled, including exact wrapper/workflow identity, the three hosted job
roles, per-job artifact and log closures, API evidence, and pre/post ref
observations. The prepared profile is not a qualification receipt. Its final
receipt identity is still unpinned, billing remains an external execution
blocker, and the gate therefore emits `PROFILE_UNPINNED`. A
GitHub-orchestrated self-hosted run is supplemental diagnostic evidence only:
it cannot satisfy this hosted gate and must never be reported as
GitHub-hosted. Native Windows cannot substitute for GitHub. A formal aggregate
admits only exact terminal PASS identities. A PASS gate must bind nonzero
candidate execution, qualification PASS, no candidate or harness failure, and
no refs or production mutation. `BLOCKED_HARNESS` and `FAIL_CANDIDATE`
receipts remain useful diagnostics but are explicitly not aggregate inputs.

## Compiled evidence profiles and exact inventories

The canonical build spec is externally digest-pinned and uses
`deny_unknown_fields` throughout. It cannot define selectors, semantic claims,
schema names, or artifact paths. Each role instead selects one compiled
profile, and each receipt must repeat the compiled identity for that profile's
fixed layers. Every layer binds:

- canonical receipt root;
- ordered manifest layers, each with root-relative path, manifest filename,
  manifest kind, manifest SHA-256, and exact entry count;
- exactly one fixed mode/type manifest with an exact digest.

A nested platform receipt always binds both its outer verification layer and
its `receipt` inner authority layer. Candidate verdict, status, execution, and
step evidence come only from the inner layer; the outer layer may only attest
relay and fixed preflight facts. Reserved status, candidate, execution,
mutation, and authority field names cannot appear as uncompiled aliases in
either artifact. Linux and Nix receipts stop at their first candidate failure;
Windows candidate verdicts bind all five ordered step rows and every JSON
return code, while a Windows harness blocker may only expose a fixed prefix
and must not claim completed JSON step results. Every layer verifies a complete
file and directory inventory. POSIX layers bind root, type, exact mode, size,
special bits, ACLs, and extended attributes. The Windows guest's NTFS-native
`FILES.tsv` binds the complete root/type/size closure without inventing POSIX
mode claims; the copied outer layer remains a full POSIX `MODES.tsv` inventory.
Extra or empty directories, files, unsafe paths,
symlinks, hardlinks, special files, changed metadata, digest drift, duplicate
roots, and `SUPERSEDED.txt` fail closed. Strict JSON parsing rejects duplicate
keys; booleans, integers, and status literals are never coerced.

The Linux V5 profile below is retained as a permanent historical NO-GO and its
frozen identity remains `None`; revision 8 does not relax or repurpose it. The
active Linux gate selects a distinct `LinuxExactV6` name only as an explicit
closed placeholder. Independent review also classified the corresponding V6
driver as permanent `HARD_NO_GO`, so V6 has no frozen driver or receipt
identity and cannot accept a receipt. Its typed inner/outer schemas preserve
the proposed vocabulary for a root-owned guardian, capability ledger, cgroup
closure, exact runner set, terminal-workload observation, copy acknowledgement,
restore ordering, and an event chain. Parsing those claims is deliberately not
treated as proof that the unsafe historical driver implemented them. Both V5
and V6 therefore fail with `PROFILE_IDENTITY_UNPINNED` before any proposed
receipt is read.

The historical Linux V5 profile independently replays the sealed
resource-watchdog log;
it does not trust the driver's PASS booleans or digest claim alone. The inner
and outer results must exactly cross-bind watchdog start/stop, candidate-window
start/completion, row count, first/last observations, interval, event absence,
and log SHA-256. The log must be nonempty canonical UTF-8 with LF only and
exactly seven tab-separated columns: timestamp, sample kind, request sequence,
Listener count, Worker count, other-Hepta-build count, and host-lock-held
state. Every sample must show zero competing work and the exact live lock;
timestamps are monotonic whole UTC seconds with no adjacent gap over 20
seconds. Exactly one request/ack-bound candidate-start row and one
candidate-completion row must bind the full candidate window and their row
digests. The filter and replay verifier must be byte-identical between the
sealed driver and receipt.

V5 also preserves the historical lifecycle vocabulary that was proposed for a
future final receipt:
a Nix-first binding; a frozen acceptance contract; fresh operator authority
and execution-authorization SSHSIGs; exact runner and independent-workload
freeze/restore inventories; a root-owned never-unlinked host-lock profile;
inner lock evidence; and byte-stable legacy production observations before
remote contact and after completion. The driver and final receipt identities
remain deliberately unpinned because the prepared driver is a permanent
`HARD_NO_GO`, lacks a separately sealed revision-7 trust-policy root, and must
not execute.

### Historical Linux V5 revision-7 trust boundary

The retained revision-7 contract described use of the existing public Ed25519
signer without reusing the old acceptance-V1 policy as Linux execution
authority. The proposed, never-admitted trust-policy root would have required
the exact canonical JSON below and the exact one-line `allowed_signers` payload
compiled by the verifier. Unknown, missing, differently typed, or differently
serialized fields fail closed, but satisfying this historical parser does not
make V5 executable or admissible.

```json
{"acceptance_profile_revision":7,"allowed_signers_sha256":"8c87ea612c4c37c8a0c13a1e4bd04d38bcbde49aeaa313e3d181cbecf9eb588d","authorization_scope":"single_linux_exact_v5_direct_launch_runner_and_independent_workload_lifecycle","authorized_action":"linux_exact_v5_execute_runner_and_workload_freeze_restore","candidate_head":"52ec4b3868fc5272e19ed516d00e11e44c549ea4","candidate_nix_process_pause_authority":false,"candidate_tree":"247e9e7cfcb41dbfcc8c5b3b531b1e1407c0bd5d","challenge_maximum_lifetime_seconds":900,"challenge_schema":"hepta_vnext_linux_operator_challenge_v2","delete_authority":false,"driver_revision":5,"execution_authorization_schema":"hepta_vnext_linux_execution_authorization_v1","fresh_authorization_nonce_required":true,"fresh_challenge_required":true,"independent_workload_pause_restore_authority":true,"key_fingerprint":"SHA256:+eNqmF4lJYlL0besra7M4BSftivEiFsQaTzFkKZKE2E","nix_container_volume_source_mutation_authority":false,"parent_trust_policy_sha256":"7aa71fe6a56a3c5e2bb091bc64e18f2a48f360451b42ad59fb3d7882305f5a49","principal":"qianqi@hepta-operator","production_authority":false,"promotion_authority":false,"qualification_host":"desktop-ts","runner_pause_restore_authority":true,"schema":"hepta_vnext_linux_operator_trust_policy_v7","schema_version":1,"signature_algorithm":"sshsig-ed25519","signature_namespace":"hepta-linux-exact-v5-execution","single_use":true,"trust_policy_scope":"candidate_52ec_linux_v5_runner_and_independent_workload_lifecycle_only","trust_root_id":"qianqi-existing-github-ed25519-2026","trust_root_revision":2,"unregister_authority":false}
```

```text
qianqi@hepta-operator ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIBzqTTB5U+BgfaVDDRmvdMrdRy/Qu9HTiNTsfG8MMX1b
```

The historical detached-signature format was scoped to one fresh, nonce-bound
direct Linux V5 qualification on `desktop-ts`, including pause-and-restore of
the self-hosted runner and independently owned competing workloads. Because V5
is permanently `HARD_NO_GO`, no such signature is accepted as execution
authority. Its format explicitly excludes pausing candidate Nix work, mutating
Nix containers, volumes, or sources, unregistering a runner, deleting data,
changing production, or promoting refs. The historical parser checks the
principal, public-key fingerprint,
allowed-signers digest, SSHSIG namespace, action, scope, 900-second maximum
lifetime, single-use challenge, and all closed authority bits
cryptographically; a policy file or claimed PASS boolean cannot substitute.

Earlier 52ec receipts lacking the profile's full mode/type/directory closure
must be re-emitted. V3 does not lower this bar or retrofit incomplete evidence.
Final roots and digests are compiled only after receipts are atomically sealed
and independently audited; prepared driver roots are not gate evidence.

### Provenance-preserving re-emission

An otherwise complete PASS attempt does not need to rerun candidate work only
because its legacy receipt lacks the V3 inventory format. Such a receipt may be
projected into a new, one-shot sibling wrapper, subject to all of these rules:

- the original root is verified under its original seal before and after the
  operation and is never modified;
- the wrapper retains the complete original outer and inner trees, original
  manifests, their externally pinned digests, and any hardlink-topology record
  as sealed provenance (an exact lossless archive is acceptable when carrying
  original hardlinks directly would violate the V3 no-hardlink rule);
- the compiled original identity separately pins the byte length, LF-row count,
  and SHA-256 of the complete POSIX metadata inventory, hardlink topology, and
  ACL/xattr inventory. A wrapper cannot self-attest a chmod, de-aliasing, or
  extended-metadata change that the legacy content manifest did not seal;
- every authoritative result, outer verification artifact, step roster, and
  log in the canonical projection is byte-identical to its original. A wrapper
  must not add a missing semantic field, change a verdict, or manufacture
  freshness;
- a sealed projection map binds every copied original path and digest to its
  canonical path. Only the new hash and metadata inventories may differ;
- canonical POSIX inventories use the compiled four-column format and reach a
  self-size fixed point. Canonical manifests bind the complete wrapper tree,
  including the provenance material, and the outer manifest recursively binds
  the canonical inner manifest;
- the resulting wrapper contains no symlink, hardlink, special file, ACL,
  extended attribute, unsafe path, or empty directory and is atomically
  published without replacement.
- the `provenance/` namespace is an exact closure: six fixed provenance
  artifacts plus exactly one de-aliased original-tree copy per frozen original
  entry and only their necessary parent directories. Unmapped payloads are
  rejected even when a wrapper is freshly self-sealed.

Revision 8 retains revision 7's treatment of additional sealed provenance
manifests (for example a
historical `DRIVER-MODES.tsv`) as ordinary evidence, not as an alternative
authority inventory. It still requires every compiled canonical inventory path
to be present and digest-bound. The verifier parses the projection map, checks
complete archive and canonical projections byte-for-byte, independently
rebuilds the original inventory, metadata, and hardlink topology before and
after, and verifies the re-emission attestation. Both the original and final
wrapper identities must be independently audited and compiled before use.

This exception is only a packaging upgrade. It applies to final Mac and Nix
PASS evidence and portable inputs whose semantics already satisfy the compiled
profile. Nix does not need a rerun solely for a missing typed inventory if its
untouched original is a complete terminal PASS, has a complete outer seal, and
passes live metadata stability and unsafe-type checks; otherwise a new Nix
attempt is required. The wrapper archives the entire original, projects
semantic evidence byte-identically, and adds only generated typed inventories,
hashes, and provenance. It
cannot turn a Windows harness blocker into PASS, fill the missing mutation
fields of an old Windows result, make self-hosted GitHub evidence hosted, or
refresh a time-sensitive path-trust/upstream observation. Linux uses a native
V5 receipt and is not wrapper-eligible. The Windows V6 driver emits canonical
guest and outer inventories natively and must not use a wrapper. Revisions
through r4 are incompatible; r4's fractional timestamps cannot satisfy the
exact-second UTC contract. The sealed r5 driver identity remains documented
for static audit only and is programmatically unpinned: the compiler returns
no admissible Windows driver identity, so an accidental r5 PASS cannot enter
an aggregate. Formal execution is NO-GO because the x230 host currently has a
self-hosted runner Listener and r5 checks only the guest boundary. A
later immutable driver revision must acquire a real shared host lock and seal
host-side Listener, Worker, and heavy-build preflight, watchdog, synchronous
candidate-boundary, and postflight evidence before it can be independently
audited and pinned. The existing r5 semantic profile still records its exact
default disk and memory floors (25,769,803,776 and 1,610,612,736 bytes),
observations at or above those floors, nonce-derived
`C:\q\52ec-<nonce>` run/source/vendor/target paths, and
`yyyy-MM-ddTHH:mm:ssZ` timestamps, but none of those facts overrides the host
lock blocker.

## CLI and one-shot boundary

Every formal command must run through the exact lane:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance-52ec-v3 -- \
  /absolute/frozen/tool/hepta-operator-acceptance-v3 build-plan \
  /absolute/canonical/build-spec.json <externally-pinned-spec-sha256> \
  /Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-aggregate-qualification-v3-<nonce>
```

Before any real aggregate, an independent freeze receipt must pin this tool's
final source commit and tree, the complete source diff (if any), the built
binary SHA-256, and the absolute frozen invocation path. Runtime environment
checks do not make a running binary self-authenticating; that external tool
identity is a prerequisite for trusting aggregate output.

Revision 8's frozen-tool publisher uses long-lived parent/root directory
descriptors; descriptor-relative `openat`/`fstatat` with no-follow checks;
atomic `renameatx_np(RENAME_EXCL)` or `renameat2(RENAME_NOREPLACE)`; parent and
tree fsync; and a post-publication same-inode replay. Its joint fixed point
seals `SHA256SUMS`, four-column `MODES.tsv`, nine-column `METADATA.tsv`
(`type,dev,inode,uid,gid,nlink,mode,size,path`), and per-node `ACL.tsv` and
`XATTRS.tsv`. Root, directories, and files are covered; symlinks, hardlinks,
special nodes, ACLs, xattrs, unsafe paths, inode swaps, and replacement fail
closed. A no-replace sibling `PUBLICATION-RESULT` records all five inventory
digests/row counts plus publisher, source, acceptance binary, parent/root
inode, rename, and pre/post bindings.

These artifacts are only tamper-evident when the sibling publication-result
digest is pinned outside the artifact tree. A self-contained manifest cannot
authenticate its own replacement. Likewise, any recorded Git relationship is
strictly `local_remote_tracking_only`; it is not proof of a remote commit,
remote ref, push, hosted run, or repository authority.

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
builder first requires `ready_for_challenge=true` and an empty blocker list;
otherwise it returns without creating either the output or `.incoming-*`.
builder stages into a same-parent hidden `.incoming-*` directory, writes
canonical `build-spec.json`, `qualification-packet.json`, and
`aggregate-build-record.json`, records exact modes, seals `SHA256SUMS`, makes
the artifacts read-only, then re-verifies the complete source evidence graph.
Only after terminal verification and directory fsync does it publish with the
operating system's atomic no-replace primitive (`renameatx_np(RENAME_EXCL)` on
macOS, `renameat2(RENAME_NOREPLACE)` on Linux), reverify the published root,
and fsync the parent again. A crash leaves an unaccepted hidden incoming tree,
never a partially valid final root.

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
