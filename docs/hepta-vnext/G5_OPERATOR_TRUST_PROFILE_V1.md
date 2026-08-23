# G5 head-scoped operator trust profile V1

This profile closes the local preparation gap between the evidence-only G5
assessor and an independently controlled operator signer. It is deliberately
read-only with respect to release authority:

- `prepare-g5` writes one canonical, head-scoped challenge;
- `assess-g5` verifies the challenge, external policy, revocation state, and an
  optional detached SSHSIG;
- neither command creates an acceptance receipt, modifies `CALLERS.toml`,
  enables `g5_allowed`, promotes an artifact, or deploys anything.

The implementation is `hepta-g5-trust-assessor` in the
`codex-hepta-operator-acceptance` crate. Formal invocation must use the
dedicated T5 lane:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run r2-g5-operator-trust-20260823 -- \
  /Volumes/T5/hepta-vnext/cache/cargo-targets/r2-g5-operator-trust-20260823/debug/hepta-g5-trust-assessor \
  prepare-g5 ...
```

The binary rejects a missing or mismatched lane environment. The `...` values
are intentionally positional and must be supplied by an independent manifest:

```text
challenge policy policy_sha256 allowed_signers revocations
base head parent_head parent_tree tree
aggregate_sha256 evidence_manifest_sha256 sha256sums_sha256
now_unix_seconds lifetime_seconds
```

`head`, `tree`, `parent_head`, and `parent_tree` are exact 40-character lower-
case Git IDs. The evidence fields are SHA-256 digests. The challenge binds all
five IDs and all three evidence digests; an assessor receives an independently
expected copy and fails closed on any mismatch.

## External trust files

The policy and revocation files are canonical compact sorted JSON. Their exact
bytes are pinned by the caller-supplied policy digest and the policy's
`allowed_signers_sha256`/`revocation_sha256` fields. The policy names one
Ed25519 principal, a trust-root revision, a maximum challenge lifetime (at most
900 seconds), and a revocation owner/revision. The revocation document is
monotonic within that root and may revoke a key fingerprint, challenge digest,
or nonce. A policy or revocation digest change invalidates an existing
challenge; it cannot silently widen trust.

The detached signature is an OpenSSH SSHSIG over the exact canonical challenge
bytes, using namespace `hepta-vnext-operator-acceptance-v1`. The process never
has a signing subcommand and never receives a private key. A test-only key may
be generated in a disposable directory to exercise verification, but that is
not independent operator acceptance.

## Status boundary

`assess-g5` emits a canonical assessment receipt with all authority flags false:

- `READY_FOR_CHALLENGE`: policy/revocation/challenge are valid and the window
  is open; an external signer still must sign the bytes.
- `SIGNATURE_VERIFIED_NO_AUTHORITY`: the supplied SSHSIG verifies, but no
  acceptance receipt or promotion authority is created.
- `EXPIRED`, `REVOKED`, or `SIGNATURE_INVALID`: fail-closed negative outcomes.

`READY_FOR_CHALLENGE` is therefore not `operator_acceptance`, `g5_allowed`, or
promotion. A later ceremony still needs an independent signer provenance,
fresh challenge consumption, a separate acceptance receipt, and the existing
provider/fleet gates.

