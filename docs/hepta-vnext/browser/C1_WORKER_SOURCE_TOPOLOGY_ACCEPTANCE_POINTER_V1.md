# WEB-C1 Worker source/API topology acceptance pointer v1

Status: **implemented as qualification tooling and synthetic fixtures only; no accepted topology pointer exists**

## Purpose

An accepted exact-source pointer proves that one immutable Servo source packet passed the separate source-only review. The Worker source/API topology verifier then proves that those accepted bytes still expose the frozen minimal public embedding API and preserve the servoshell/WebDriver exclusion facts.

This slice adds the second independent review boundary:

```text
accepted exact source pointer
  -> source/API topology verification receipt
  -> create-only topology review challenge
  -> dedicated current-head review PR
  -> verify-only accepted topology pointer
```

The tool never creates or updates the final accepted topology pointer. The pointer must be manually authored in a dedicated, non-draft review PR and validated by trusted-base code.

## Required bindings

The review challenge and pointer bind exact bytes for:

- the accepted source pointer;
- the topology verification receipt;
- `SERVO_WORKER_SOURCE_TOPOLOGY_V1.json`;
- the topology acceptance policy;
- the review challenge itself;
- the source archive and source-bundle receipt digests;
- the frozen topology ID, topology digest and selected-file projection digest;
- the dedicated PR number and exact head ref.

The topology receipt must match the accepted source pointer for:

```text
servo repository
servo commit
servo tree
recomputed Git tree
independent-source-bundle.receipt.json SHA-256
servo-source-a.tar.gz SHA-256
```

It must preserve:

```text
worker_owner = hepta
servo_crate_path = components/servo
servo_default_features = false
servoshell_dependency = false
webdriver_server_dependency = false
required features = background_hang_monitor, bundled
conditional feature = js_jit
one process / one mutation owner / Option<WebView>
```

## Dedicated review lane

A real topology pointer proposal must use:

```text
head ref prefix:
review/hepta-servo-worker-topology-acceptance-

required approval line:
HEPTA_WORKER_TOPOLOGY_ACCEPT_V1 <challenge_id>

required status check:
Worker source/API topology accepted pointer live review
```

The reviewer must be different from the PR author, have `OWNER`, `MEMBER`, or `COLLABORATOR` association, approve the exact current head, and submit the approval after that head commit. A current-head `CHANGES_REQUESTED`, stale approval, draft PR, unknown changed path, missing receipt/challenge snapshot, wrong PR number or wrong head ref fails closed.

`author_association` is a trusted-collaborator check, not a CODEOWNER identity proof. Repository rules may independently require CODEOWNER review.

## Trusted-base execution

The live workflow checks out the exact base SHA. It fetches only the governed pointer, receipt and challenge bytes from the PR head through the read-only GitHub API. It never executes Python, shell, actions or configuration from the untrusted PR head.

The source acceptance live workflow is upgraded to the same trusted-base pattern in this slice.

## Acceptance boundary

A valid future pointer may set:

```text
exact_servo_source_accepted = true
source_review_candidate_accepted = true
worker_source_topology_accepted = true
```

It must keep all of the following false:

```text
build_recipe_accepted
build_authorized
servo_built
worker_artifact_created
servo_runtime_qualified
machine/runtime/product/effect/network/credential authority
operator acceptance
G5/execute
promotion
release qualification
```

Therefore, topology acceptance permits only the next reviewed step: creating the Hepta-owned out-of-tree Worker crate and preparing a separate build-recipe review. It does not permit a build.

## Entrypoints

```sh
python3 scripts/hepta-servo-worker-source-topology-acceptance-pointer-v1.py contract \
  --policy "$(pwd)/docs/hepta-vnext/browser/WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json"

python3 scripts/tests/test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py -v
python3 scripts/verify-hepta-servo-worker-source-topology-acceptance-pointer-v1.py
```
