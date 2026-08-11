# Hepta vNext Signed Operator Acceptance V1

## Status and boundary

This document describes the implemented independent gate after the frozen
qualification receipt for candidate
`3110c5aba5daa0af1498b3eec85272011589ce8e`. The gate accepts only the exact
qualification evidence encoded in an implementation-generated challenge. It
does not grant or trigger execution authority.

The words **MUST**, **MUST NOT**, **SHOULD**, and **MAY** are normative.

Creating this document, asking an agent to continue, approving a plan in chat,
or sending any other natural-language instruction is not operator acceptance.
In particular, the instruction that requested work on this gate is not a
signature. The gate can pass only after an independently controlled Ed25519
private key creates an OpenSSH SSHSIG over the exact canonical challenge bytes
and the implementation verifies and seals that signature.

No part of this V1 gate enters Enforce, promotion, outbound, or retirement.

## Implemented command-line interface and T5 entrypoint

Formal use MUST enter through the T5 fail-closed wrapper. The executable has
three positional interfaces and no signing subcommand:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance -- \
  /Volumes/T5/hepta-vnext/artifacts/tools/hepta-operator-acceptance-v1/hepta-operator-acceptance prepare \
  /Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z \
  /Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build \
  /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name> \
  /absolute/external/allowed_signers \
  /absolute/external/operator-acceptance-trust-policy.json \
  <externally-pinned-trust-policy-sha256>

/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance -- \
  /Volumes/T5/hepta-vnext/artifacts/tools/hepta-operator-acceptance-v1/hepta-operator-acceptance verify \
  /Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z \
  /Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build \
  /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name> \
  /absolute/external/allowed_signers \
  /absolute/external/operator-acceptance-trust-policy.json \
  <externally-pinned-trust-policy-sha256> \
  /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name>/operator-acceptance-challenge.json.sig

/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance -- \
  /Volumes/T5/hepta-vnext/artifacts/tools/hepta-operator-acceptance-v1/hepta-operator-acceptance verify-receipt \
  /Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z \
  /Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build \
  /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name> \
  /absolute/external/allowed_signers \
  /absolute/external/operator-acceptance-trust-policy.json \
  <externally-pinned-trust-policy-sha256>
```

The wrapper checks `/Volumes/T5` with `diskutil` before execution. It requires
Volume UUID `FB804D1B-24CB-4D6E-AEA7-A9E180807758`, `Owners: Enabled`, a writable
canonical SSD repository, and the `operator-acceptance` worktree. A missing T5,
wrong UUID, disabled ownership semantics, read-only repository, or unknown lane
fails before the command runs.

The absolute executable shown above is the frozen, privately permissioned V1
tool artifact. Formal use MUST verify that artifact directory's `SHA256SUMS`
before `prepare` and MUST use the same exact executable for `prepare`, `verify`,
and `verify-receipt`. A mutable Cargo/Bazel cache binary, `cargo run`, a bare
`PATH` lookup, or a different executable is not a formal ceremony entrypoint.

The CLI independently rejects a missing or mismatched exact lane identity:

```text
HEPTA_SSD_ROOT=/Volumes/T5/hepta-vnext
HEPTA_SSD_VOLUME_UUID=FB804D1B-24CB-4D6E-AEA7-A9E180807758
HEPTA_LANE=operator-acceptance
HEPTA_WORKTREE=/Volumes/T5/hepta-vnext/worktrees/operator-acceptance
HEPTA_ARTIFACTS_DIR=/Volumes/T5/hepta-vnext/artifacts
```

The wrapper exports those values. Invoking `hepta-operator-acceptance` without
that wrapper-provided identity or under another lane's environment is not a
supported formal entrypoint and MUST fail the CLI guard.

All CLI-provided roots and input files MUST be absolute, already canonical paths
without symlink components, owned by the effective user, and inaccessible to
group and other users. Input files MUST be regular files with one hard link. The
qualification and product-audit roots MUST equal the two exact paths shown
above. The sidecar root MUST already exist as a strict canonical child of
`/Volumes/T5/hepta-vnext/artifacts/acceptances`; the prefix itself is not a valid
sidecar. It MUST also be disjoint from both evidence roots. The `allowed-signers`
and trust-policy files MUST be distinct and outside the qualification,
product-audit, and sidecar roots.

`prepare` fully verifies the two frozen evidence inventories and the
external trust inputs, advances the trusted-time watermark, and creates:

```text
<sidecar-root>/operator-acceptance-challenge.json
```

It fails if a challenge, nonce claim, or acceptance receipt already exists in
that sidecar. On success it prints compact JSON containing `challenge_path`,
`challenge_sha256`, and `expires_at_unix_seconds`.

The operator signs the exact challenge file outside this program. A compatible
external signing command is:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance -- \
  ssh-keygen -Y sign \
    -f /independently-controlled/operator-ed25519-key \
    -n hepta-vnext-operator-acceptance-v1 \
    /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name>/operator-acceptance-challenge.json

/Volumes/T5/hepta-vnext/bin/hepta-ssd-run operator-acceptance -- \
  chmod 600 \
  /Volumes/T5/hepta-vnext/artifacts/acceptances/<acceptance-store-name>/operator-acceptance-challenge.json.sig
```

OpenSSH commonly creates the `.sig` with mode `0644`; the explicit wrapper-gated
`chmod 600` is therefore part of this local ceremony. The resulting file MUST
be effective-user-owned and private before it is passed to `verify`.

`verify` accepts the resulting detached SSHSIG envelope. On first successful
consumption it creates a nonce claim and an acceptance receipt, then prints
compact JSON containing `acceptance_receipt_path`,
`acceptance_receipt_sha256`, and `challenge_sha256`.

`verify-receipt` verifies an already sealed receipt using the SSHSIG envelope
embedded in that receipt. It takes no detached-signature path, creates no new
acceptance, and prints the same three-field result only after the current
evidence, trust binding, challenge, stored signature, claim, and receipt agree.
It requires and acquires the existing `.operator-acceptance.lock`; a missing
lock fails closed. It does not create or modify the lock, challenge, nonce claim,
acceptance receipt, evidence, or external trust files.

## Canonical JSON bytes

The signed statement is the exact byte sequence in
`operator-acceptance-challenge.json`. It is not a line-oriented `key=value`
document.

The implementation's canonical JSON algorithm is:

1. Serialize the typed value to a JSON value.
2. Sort every object recursively by key in ascending lexical order.
3. Preserve array order while recursively sorting objects inside arrays.
4. Serialize with compact `serde_json` encoding and no trailing LF or other
   whitespace.

This implementation-specific encoding, not a pretty-printed rendering, is what
is hashed and signed. On read, the implementation deserializes with unknown
fields denied, reconstructs canonical bytes, and requires an exact byte-for-byte
match. Missing, duplicate, unknown, reordered, or differently encoded fields
fail closed. The challenge, trust policy, nonce claim, receipt, and time
watermark all use this compact sorted JSON encoding.

The SHA-256 values in this document are lowercase hexadecimal. The challenge
digest is SHA-256 over the exact canonical challenge bytes. The detached
signature digest is SHA-256 over the exact SSHSIG envelope bytes.

## External trust contract

The trust-policy file MUST itself be canonical compact sorted JSON with exactly
these fields:

```text
acceptance_store_root
allowed_signers_sha256
key_fingerprint
maximum_lifetime_seconds
principal
schema
schema_version
trust_policy_scope
trust_root_id
trust_root_revision
```

The following values and constraints are implemented:

- `acceptance_store_root` is the exact canonical absolute sidecar-root path.
- `schema` is `hepta_operator_acceptance_trust_policy_v1`.
- `schema_version` is `1`.
- `trust_policy_scope` is
  `externally_pinned_single_ed25519_external_revocation_responsibility_no_local_krl_v1`.
- `trust_root_revision` is nonzero.
- `maximum_lifetime_seconds` is in `1..=3600`.
- `principal` and `trust_root_id` are 1--128 ASCII bytes from
  `[A-Za-z0-9._:/@-]`.
- `allowed_signers_sha256` is the SHA-256 of the exact external
  `allowed_signers` bytes.
- `key_fingerprint` is the OpenSSH `SHA256:` fingerprint of the exact key blob.

The caller MUST independently obtain and positionally supply the 64-character
`externally-pinned-trust-policy-sha256`. The verifier hashes the exact canonical
trust-policy file and rejects a mismatch. That digest is also copied into the
challenge as `operator.trust_policy_sha256` and scopes the durable time
watermark. It is not read from or self-authorized by the acceptance packet.
The verifier also requires the actual sidecar path to equal the policy's
`acceptance_store_root`; the external policy therefore pins both the signer and
the acceptance store used for nonce and receipt durability.

V1 accepts exactly one LF-terminated `allowed_signers` line, with no carriage
return and exactly three whitespace-separated fields:

```text
<principal> ssh-ed25519 <base64-encoded-raw-Ed25519-public-key>
```

The principal MUST equal the policy principal. Certificates, additional keys,
options, and other key algorithms are rejected. The implementation parses the
SSH key blob, requires a valid non-weak 32-byte Ed25519 point, recomputes its
fingerprint, and compares both the fingerprint and the exact-file digest with
the external policy.

The detached signature MUST be an LF-terminated OpenSSH SSHSIG envelope. The
verifier invokes `/usr/bin/ssh-keygen -Y verify` with the exact principal and
namespace `hepta-vnext-operator-acceptance-v1`. It supplies the pinned
`allowed_signers` and signature bytes through inherited pipes rather than
trusting paths embedded in the packet or leaving verification-staging files.

V1 has no KRL argument, performs no local KRL check, and does not query a
revocation service. Key validity and revocation are explicitly the external
policy owner's responsibility. Revocation MUST therefore be enforced outside
this verifier through control of the independently pinned policy and signer
material. This document does not claim an unimplemented local revocation check.

## Challenge schema and exact boundary

The challenge schema is `hepta_operator_acceptance_v1`, version `1`. Its
top-level fields, in canonical order, are:

```text
automatic_transition
authority
candidate
decision
declaration
expires_at_unix_seconds
excluded_gates
frozen_product
issued_at_unix_seconds
namespace
nonce
not_before_unix_seconds
operator
oracle
qualification_receipt
schema
schema_version
scope
signature_algorithm
```

The fixed control values are:

| Field | Exact value |
| --- | --- |
| `schema` | `hepta_operator_acceptance_v1` |
| `schema_version` | `1` |
| `namespace` | `hepta-vnext-operator-acceptance-v1` |
| `signature_algorithm` | `openssh-sshsig-ed25519` |
| `scope` | `qualification_evidence_only` |
| `decision` | `accept` |
| `automatic_transition` | `false` |

`declaration` is exactly:

```text
Accept only the exact qualification evidence and signed exclusions. This grants no authority for Enforce, promotion, outbound, or retirement. V1 applies no local KRL; the externally pinned policy owner remains responsible for key validity and revocation.
```

`authority` is exactly the following canonical JSON object in both the
challenge and the receipt:

```json
{"authority":false,"enforce":false,"operator_acceptance":true,"outbound":false,"promotion":false,"qualification_authority":false,"retirement":false}
```

`excluded_gates` is exactly:

```json
{"github_gate_run":false,"memory_gate_run":false,"proof_gate_run":false,"s2_gate_run":false,"s5_gate_run":false,"windows_gate_run":false}
```

The `operator` object contains exactly `acceptance_store_root`,
`allowed_signers_sha256`, `key_fingerprint`, `maximum_lifetime_seconds`,
`principal`, `trust_policy_scope`, `trust_policy_sha256`, `trust_root_id`, and
`trust_root_revision`, all reconstructed from the externally verified trust
inputs. `acceptance_store_root` repeats the externally pinned canonical sidecar
path in the signed challenge.

### Frozen evidence fields

The `candidate` object binds:

| Field | Exact value |
| --- | --- |
| `base` | `89a335ed50258dc9dc5b3d7f410db61b431244f9` |
| `bundle_sha256` | `eb57cf87d7b85b85722d0ad3802ee414717e460d933c8acd4665decaf795592b` |
| `head` | `3110c5aba5daa0af1498b3eec85272011589ce8e` |
| `tree` | `90164e397240e3e5e85027876394df7045991ff6` |

The `qualification_receipt` object contains exactly these fields:

```text
candidate_bundle_sha256
git_tree_manifest_sha256
manifest_entry_count
manifest_root_kind
manifest_sha256
receipt_id
receipt_root
runs
soak_summary_sha256
status_sha256
tracked_content_manifest_sha256
```

Its fixed values include:

| Field | Exact value |
| --- | --- |
| `candidate_bundle_sha256` | `eb57cf87d7b85b85722d0ad3802ee414717e460d933c8acd4665decaf795592b` |
| `git_tree_manifest_sha256` | `6d5f8c9e6d61a326cb1f6c585f0a2ca15edd151f8e6944be24502301626c54ca` |
| `manifest_entry_count` | `1786` |
| `manifest_root_kind` | `sha256_of_sha256sums_bytes` |
| `manifest_sha256` | `9ed5fcc120af363f89c83969ac29956722f66c780d4b9bb7e86a27d7965d663f` |
| `receipt_id` | `qualification-3110c5aba5-final-20260810T192902Z` |
| `receipt_root` | `/Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z` |
| `soak_summary_sha256` | `093d1a1ddf554e90551e27b2fde11c7ba4f9ce9b6603e263c163bf023d60e0ec` |
| `status_sha256` | `8c913bf997fbed194c165694993b329eaff5cd4ba1436571781f0c32d3da43dc` |
| `tracked_content_manifest_sha256` | `cf2cd0b8c473a3c98fbccf572d2d23489e080a068429e298eb3fe0b9eb85c914` |

`manifest_sha256` is SHA-256 over the exact top-level `SHA256SUMS` bytes;
`manifest_root_kind` makes that meaning explicit. `receipt_root` is the exact
hard-pinned canonical absolute qualification-root path shown in the table and
is included in the signed bytes. It is not another digest.

`runs` is an ordered array of exactly three validated soak-run bindings with
indices `1`, `2`, and `3`. Each object contains exactly
`evidence_set_sha256`, `index`, `manifest_sha256`,
`qualification_report_sha256`, `run_id`, `run_root_relative_path`,
`terminal_seal_file_sha256`, `terminal_seal_sha256`,
`transport_evidence_sha256`, and `transport_manifest_sha256`. Their values are
read from, cross-checked against, and transitively frozen by the exact 1786-entry
qualification manifest.

The `frozen_product` object contains exactly these fields and bindings:

| Field | Exact or implemented value |
| --- | --- |
| `audit_manifest_entry_count` | `6` |
| `audit_manifest_sha256` | `21e9bef2e8ea60dce76c9d6c78871afd64db13bc050921ca42f9b95bff295be2` |
| `audit_root` | `/Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build` |
| `binary_relative_path` | `hepta-2f704dc7c1-aarch64-apple-darwin` |
| `binary_sha256` | `8843df374eac70246a9398feaf25045558ac0aa7a25e6af92d186df7d7b3434c` |
| `binary_size_bytes` | `556410456` |
| `platform` | `aarch64-apple-darwin` |
| `source_commit` | `2f704dc7c1172cefca908852456beccf4d02a5d1` |
| `source_tree` | `7be9a382b2610790838eef874cb4d381b5025490` |

`audit_root`, like `receipt_root`, is hard-pinned and included in the signed
challenge. The frozen product is the qualified oracle product, not a release
binary built from the candidate; this remains qualification-evidence
acceptance, not release acceptance.

The `oracle` object binds:

| Field | Exact value |
| --- | --- |
| `commit` | `2f704dc7c1172cefca908852456beccf4d02a5d1` |
| `corpus_sha256` | `dfe4f04d26895a6fabfb8435b77d7e807f57379fbb8d2a96c85af747e996cda7` |
| `expected_normalized_receipt_sha256` | `8904f0cc74e8a1b465eb75c7cd0c3f6ebef916c414dc9f5b6610d5822e9f68c0` |
| `sample_id_sha256` | `426468e3c420e5557f2edbbb0adfc845b611c00416112c1ed95d99219fa9c5ef` |
| `tree` | `7be9a382b2610790838eef874cb4d381b5025490` |

Before challenge construction and again during first-consumption verification,
the implementation verifies the exact inventory and SHA-256 of every artifact
under both evidence roots, the qualification status, candidate identity,
product audit, three soak runs, reports, terminal seals, and oracle bindings.

## Threat model and proof boundary

The cryptographic acceptance fact in V1 is deliberately narrow: a private key
corresponding to the externally pinned single Ed25519 public key produced a valid
SSHSIG, under the exact namespace, over the exact canonical challenge bytes.
That signature binds the evidence, exclusions, operator/trust/store binding,
nonce, and challenge validity-window fields contained in those bytes.

The signature is not a trusted timestamp. In particular, it does not by itself
prove when the operator signed, when a verifier first consumed the signature, or
that consumption happened before expiration. The receipt's
`accepted_at_unix_seconds` is added by the verifier and is not covered by a
second operator signature.

Live first-consumption checks assume a trusted local verifier and a cooperative,
effective-user-owned acceptance-store owner. `SystemTime::now()`, the watermark,
the exclusive lock, create-new claim, and claim-before-receipt ordering are local
ceremony and replay controls under that assumption. They are not a TSA-backed
timestamp, TPM monotonic counter, secure-clock proof, host attestation, or
Byzantine exactly-once protocol.

The permission, no-symlink, one-hard-link, hashing, `fsync`, and canonical-path
checks reduce accidental corruption and unprivileged interference. They do not
defend against a malicious or compromised process with the same effective UID.
Such a process can modify or replace the sidecar, trust inputs, evidence,
verifier binary, or clock-facing environment; resistance to that store owner is
outside the V1 threat model.

`verify-receipt` cryptographically reverifies the embedded SSHSIG and checks
that the current evidence, external trust binding, challenge, receipt, and claim
are internally consistent. It does not consult the live watermark or attest an
unforgeable historical consumption time. Consequently, portable stored-receipt
verification proves the signature and binding consistency, not that
`accepted_at_unix_seconds` was created before expiry or that the nonce was
consumed exactly once against a malicious store owner. The policy-pinned
absolute evidence and acceptance-store paths must still match, so
`verify-receipt` is self-contained with respect to the original `.sig` file but
is not location-independent.

## Time, watermark, nonce, and sealing order

`prepare` samples `SystemTime::now()` in whole Unix seconds, advances the
durable watermark, and sets:

```text
not_before_unix_seconds = issued_at_unix_seconds
expires_at_unix_seconds = issued_at_unix_seconds + min(900, maximum_lifetime_seconds)
```

The nonce is 32 bytes from the operating-system CSPRNG encoded as 64 lowercase
hexadecimal characters; an all-zero nonce is rejected. The validity window is
half-open:

```text
not_before_unix_seconds <= trusted_now < expires_at_unix_seconds
```

The implemented first-consumption order is intentionally fail closed:

1. Acquire the exclusive sidecar lock, sample trusted host time, and durably
   advance the policy-scoped time watermark. Clock rollback fails.
2. Reverify the evidence, external trust inputs, and canonical challenge.
3. If no receipt or claim exists, require the first time sample to be inside the
   signed window.
4. Verify the SSHSIG, reload and compare the trust binding, reverify the evidence,
   and require that the canonical challenge bytes did not change.
5. Sample `accepted_at_unix_seconds`, durably advance the watermark again, and
   require that final sample to be inside the same half-open window.
6. Immediately recheck that neither claim nor receipt appeared, then durably
   create the nonce claim with create-new semantics.
7. Construct, validate, and durably create the acceptance receipt.

Thus the final trusted-time sample, rollback-protected watermark update, and
window validation are adjacent to nonce consumption. A first consumption at or
after expiration fails closed. Because the generated lifetime is
`min(900, policy maximum)`, the first-consumption window is never more than 900
seconds and can be shorter. Crossing the expiration boundary during signature
verification also fails before the nonce claim is written.

The watermark file is canonical JSON named
`operator-acceptance-time-watermark.json` with schema
`hepta_operator_acceptance_time_watermark_v1`, version `1`,
`last_observed_unix_seconds`, and `trust_policy_sha256`. The watermark MAY
advance even when the overall verification later fails.

The nonce claim is canonical JSON named
`operator-acceptance-nonce-claim.json` with schema
`hepta_operator_acceptance_nonce_claim_v1`, version `1`, and exactly
`accepted_at_unix_seconds`, `challenge_sha256`,
`detached_signature_sha256`, and `nonce`. Under the trusted local store
assumption, claim-first persistence prevents a crash after consumption from
being retried as a fresh acceptance: a preserved claim without a PASS receipt
always fails closed. V1's nonce-consumption record is sidecar-local; it is not a
global nonce registry or a cryptographic exactly-once proof.

An exact stored replay is idempotent only when a receipt already exists and the
current challenge, external trust binding, SSHSIG envelope embedded in the
receipt, recomputed detached-signature digest, canonical receipt, and canonical
claim all match. It returns the stored receipt digest and does not reapply a
transition. The original validity window is not rechecked for that already
sealed exact replay. A different or missing claim, receipt, challenge, trust
binding, or stored signature fails closed.

`verify-receipt` performs that stored verification directly and does not sample
or advance the current time watermark. `verify` also takes the stored-verification
path when a receipt already exists. That command has already sampled time and
advanced the watermark, but it does not apply the old challenge window to the
stored receipt; verification uses the receipt's embedded SSHSIG rather than the
positional detached-signature file. Neither path turns an expired challenge into
a new first consumption.

## Acceptance receipt and PASS condition

The acceptance receipt is canonical compact sorted JSON named
`operator-acceptance-receipt.json`, with exactly these top-level fields:

```text
accepted_at_unix_seconds
authority
challenge
challenge_sha256
schema
schema_version
signature
```

Its schema is `hepta_operator_acceptance_receipt_v1`, version `1`.
`challenge` is the complete signed challenge, and `challenge_sha256` binds its
exact canonical bytes. `accepted_at_unix_seconds` MUST be within the challenge's
half-open validity window.

The `signature` object contains exactly `algorithm`,
`allowed_signers_sha256`, `detached_signature_sha256`,
`detached_signature_sshsig_base64`, `key_fingerprint`, `namespace`, and
`principal`. `detached_signature_sshsig_base64` is canonical standard Base64 as
emitted by `base64::STANDARD`, including required padding, over the complete
SSHSIG envelope, making later `verify-receipt`
cryptographic verification independent of the original signature path. Its
decoded SHA-256 and all other values MUST match the verified signature and the
operator binding embedded in the challenge.

PASS exists only after `verify` successfully persists this receipt. In both the
receipt's top-level `authority` and its nested challenge, the only true field is:

```text
operator_acceptance=true
```

The other six authority fields MUST all remain false:

```text
authority=false
enforce=false
outbound=false
promotion=false
qualification_authority=false
retirement=false
```

There is no separate `acceptance=true` field in the implemented schema.
`automatic_transition` also remains false, and all six `excluded_gates` fields
remain false. Any discrepancy in either authority copy invalidates the receipt.

The PASS receipt acknowledges only the exact qualification evidence and signed
exclusions. It neither contains nor triggers Enforce, promotion, outbound, or
retirement; it does not edit configuration, mint a promotion receipt, send an
artifact or message, retire a route, or invoke a later-gate entrypoint. Every
later gate requires a separately scoped implementation and explicit invocation.

Any evidence mismatch, noncanonical JSON, unknown field, trust-policy digest
mismatch, wrong signer or namespace, invalid SSHSIG envelope, clock rollback,
expired first consumption, sidecar race, or persistence inconsistency fails
closed. A failure before claim creation produces no PASS receipt; a durable
claim followed by a receipt-write failure leaves subsequent conforming runs
fail-closed while that store is preserved, until handled outside this V1 flow.
The reference implementation only reads frozen qualification and product
evidence; the same-UID threat-model limitation above still applies.
