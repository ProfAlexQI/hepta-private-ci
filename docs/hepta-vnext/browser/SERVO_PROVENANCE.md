# Servo source provenance and SBOM preflight

Status: **DEVELOPMENT / qualification-only / no Servo source imported**

This procedure turns an independently prepared Servo checkout into a canonical
source receipt. It performs no network access and grants no runtime, caller,
effect, operator or promotion authority.

## Inputs

The checkout must be an absolute canonical path to a non-symlink Git worktree
at the exact commit and tree in `SERVO_UPSTREAM_PIN.json`. It must be clean,
have no untracked files, and contain the exact reviewed blobs listed in
`SERVO_SOURCE_IMPORT_TOPOLOGY.yaml`.

The generator also binds the Hepta patch queue. Unregistered `.patch` files,
missing patch files, changed patch bytes or a patch inventory bound to another
Servo source fail closed.

## Command

```sh
python3 scripts/generate-hepta-servo-provenance.py \
  --servo-source /absolute/canonical/servo-checkout \
  --output /absolute/private/output/servo-source-receipt.json
```

The output path is intentionally external to the source checkout. The generated
receipt contains no machine-local source path. Re-running against unchanged
inputs produces byte-identical compact sorted JSON.

## Verified facts

The generator verifies:

- exact Git `HEAD` and `HEAD^{tree}`;
- empty tracked, staged and untracked status;
- a non-empty unique tracked-file inventory;
- the frozen Git blob ID of every reviewed source file;
- SHA-256 and byte length of reviewed and required source files;
- the expected MPL-2.0 license text;
- the exact patch inventory and patch SHA-256 values;
- negative authority and external-network flags.

A source receipt is not an SBOM by itself. C1-002 remains open until the same
exact checkout also produces a Cargo/native dependency SBOM, license inventory,
toolchain compatibility receipt and source bundle checksum set.

## Deliberate exclusions

This procedure does not:

- fetch or update Servo;
- trust a branch name or mutable tag;
- build Servo;
- start a browser or listener;
- enable external network access;
- create a production caller;
- satisfy WEB-C1 runtime qualification;
- replace operator acceptance or promotion.

The receipt schema is
`hepta.servo.source_receipt.v1.schema.json`. A schema-valid receipt still needs
independent verification and inclusion in a later exact artifact manifest.
