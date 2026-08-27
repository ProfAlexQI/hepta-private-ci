# WEB-C1 minimal Servo worker source/API topology

Status: **contract implemented and locally fixture-qualified; exact source-bundle evidence pending**

## Purpose

This gate freezes the exact pinned Servo files and public embedding API that a
future Hepta worker may use. It closes a gap between “the complete source bundle
matches the pinned Git tree” and “the selected worker build root and API surface
are minimal and intentional.”

The canonical topology is:

```text
docs/hepta-vnext/browser/SERVO_WORKER_SOURCE_TOPOLOGY_V1.json
```

The canonical verifier is:

```text
scripts/hepta-servo-worker-source-topology.py
```

## Selected strategy

The worker is owned by Hepta and built outside the Servo and Codex workspaces.
It depends directly on `components/servo` with default features disabled.

Selected initial Servo features:

```text
background_hang_monitor
bundled
```

`js_jit` is conditionally permitted only after platform qualification. All
Bluetooth, testbinding, WebGPU, WebXR, WebGL, gamepad, media, clipboard and
default feature groups remain forbidden.

## Why servoshell is reference-only

At the pinned commit, `ports/servoshell/Cargo.toml` unconditionally enables
`servo/bluetooth`, `servo/testbinding`, and `webdriver_server`. Its runtime state
can start the WebDriver server, and the upstream server binds to `0.0.0.0`.

The topology therefore records the exact upstream blobs and conflict anchors
but sets:

```text
servoshell_build_root=false
servoshell_dependency=false
webdriver_server_dependency=false
patch_required_before_servoshell_build=true
```

No build recipe may use servoshell as its package or dependency unless a
successor topology and governed patch set are reviewed.

## Verification

Real verification performs these steps:

1. rerun the canonical source-bundle verifier v2;
2. require the exact pinned commit, tree, recomputed tree and closed authority;
3. hash the retained compressed archive and compare it with the source receipt;
4. inspect the deterministic tar without extracting it;
5. require all selected and reference-only files as regular files;
6. recompute each Git blob SHA-1;
7. compare it with the frozen upstream object ID;
8. require all public-API and conflict anchors;
9. emit a create-only, mode-0600, self-bound receipt.

The gate rejects duplicate JSON keys, noncanonical JSON, unsafe paths, hard
links, required symlinks, blob drift, anchor drift, feature-set widening,
servoshell selection, WebDriver selection, and positive authority.

## Receipt meaning

A valid
`hepta.servo.worker_source_topology_verification.v1` receipt proves only:

```text
canonical source bundle reverified
selected upstream blobs match
public embedding API anchors match
servoshell/WebDriver conflicts remain present and excluded
Hepta-owned embedder is required
```

It explicitly keeps false:

```text
build_recipe_created
servo_built
worker_artifact_created
servo_runtime_qualified
all authority fields
```

## Commands

```sh
python3 scripts/hepta-servo-worker-source-topology.py contract

python3 scripts/hepta-servo-worker-source-topology.py verify \
  --bundle-dir /absolute/private/accepted-source-bundle \
  --output /absolute/private/output/worker-source-topology-receipt.json
```

The second command may run only against an accepted exact source bundle. It
performs no network access, build, link, process launch, or Servo execution.
