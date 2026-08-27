# WEB-C1 graph-bound startup candidate descriptor

Status: **implemented as a fixture-only descriptor compiler; no real descriptor exists and launch remains unauthorized**

## Purpose

A verified receipt graph proves that source, build manifest, artifact receipt, reproducibility receipt, and worker bytes belong to one immutable candidate. The private startup bridge additionally needs a session-specific input binding:

```text
verified graph
+ selected worker bytes
+ BrowserSessionId
+ generation
+ owner epoch
+ private transport class
= startup candidate descriptor
```

The descriptor compiler creates that immutable input packet. It does not launch the worker, generate bootstrap secrets, open a transport, or qualify Servo. The descriptor is deliberately incapable of granting authority.

## Required inputs

The compiler accepts one canonical packet root and three unique relative paths:

- strict receipt-graph manifest;
- strict receipt-graph verification receipt;
- selected worker executable.

It also accepts:

- a 32-byte lowercase hexadecimal BrowserSessionId;
- positive nonzero generation;
- positive nonzero owner epoch;
- exactly one private transport class:
  - `unix_inherited_socketpair`, or
  - `windows_sid_named_pipe`.

TCP, loopback, WebSocket, filesystem socket, arbitrary endpoint, and external-network transport labels are rejected.

## Graph and worker checks

Before creating a descriptor, the compiler verifies:

1. manifest and verification receipt are compact canonical JSON with unique keys;
2. both bind the exact pinned Servo commit/tree;
3. manifest policy and all thirteen authority fields are fail-closed;
4. verification runtime posture fixes launch, execution, runtime qualification, and network use to false;
5. verification decision is `RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED`;
6. verification receipt ID recomputes from its complete payload;
7. verification `manifest_sha256` equals the actual manifest bytes;
8. both manifest and verification contain exactly the five required nodes;
9. selected worker path is the graph's binary worker node;
10. verifier-computed worker SHA-256 and byte length match both worker node records;
11. the graph reports all edges matched, five nodes, and at least seven edges.

The compiler does not replace strict graph verification. The output fixes `verification_required_again_at_launch=true`, so the future launcher must revalidate the graph immediately before opening or inheriting the executable handle.

## Secret exclusion

The descriptor contains no:

- startup capability;
- capability digest;
- host nonce;
- raw credential or SecretRef;
- authorization header;
- process environment secret.

These values are generated only after a future launch admission and are transported over the already-established private inherited channel. The descriptor compiler recursively rejects exact secret-bearing key names.

## Descriptor contents

`hepta.servo.worker_startup_descriptor.v1` binds:

- exact Servo commit/tree;
- BrowserSessionId, generation, owner epoch;
- private transport class with no network listener, filesystem endpoint, or external network;
- worker packet-relative path, SHA-256, and byte length;
- graph manifest path and SHA-256;
- graph verification path, SHA-256, and self-binding receipt ID;
- explicit re-verification-at-launch requirement;
- complete negative runtime and authority posture;
- decision `GRAPH_BOUND_STARTUP_CANDIDATE_LAUNCH_NOT_AUTHORIZED`;
- domain-separated self-binding descriptor ID.

Output is compact canonical JSON, create-only, mode `0600`, fsynced, and never overwritten.

## Separation from launch admission

A valid descriptor means only:

> This immutable worker candidate is bound to this browser lifecycle identity and this private transport class.

It does not mean:

- launch is authorized;
- the worker was executed;
- the graph verification is fresh enough for launch;
- the executable handle is TOCTOU-safe;
- the OS peer identity, sandbox, resource limits, or descendant cleanup are qualified;
- Servo rendered a WebView;
- operator acceptance, promotion, or release exists.

A future launcher must consume the descriptor by exact digest, reopen/reverify the graph, bind the verified bytes to a platform handle, perform a separate qualification admission, and only then generate one-use bootstrap secrets.

## Fixture coverage

The fixture suite covers:

1. descriptor creation and exact recomputation;
2. create-only output;
3. worker-byte drift rejection;
4. launch-authorized graph rejection;
5. invalid session identity rejection;
6. network transport rejection;
7. secret-bearing descriptor key rejection.

No real descriptor has been created. `real_descriptors_created=0` remains release-blocking.
