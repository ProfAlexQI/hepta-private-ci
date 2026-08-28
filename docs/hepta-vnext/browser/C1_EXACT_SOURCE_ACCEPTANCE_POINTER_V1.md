# WEB-C1 exact Servo source acceptance pointer v1

Status: **implemented as qualification tooling and fixtures only; no accepted source pointer exists**

## Purpose

The exact-source review candidate v2 proves that retained workflow evidence and source bytes are internally consistent, but it deliberately leaves source acceptance false. This slice defines the separate review boundary that may convert one immutable candidate into a source-only accepted pointer.

The acceptance pointer does not authorize a build. Its only positive claim is:

```text
exact_servo_source_accepted=true
source_review_candidate_accepted=true
```

The following remain false:

```text
worker_source_topology_accepted
build_recipe_accepted
build_authorized
servo_built
worker_artifact_created
servo_runtime_qualified
operator_acceptance
promotion
release_qualified
```

All machine, runtime, product, effect, network, credential, execute, operator, promotion and release authority remains false.

## Inputs

The verifier binds four canonical JSON objects:

1. `hepta.servo.exact_source_review_candidate.v2`;
2. `hepta.servo.source_acceptance_review_policy.v1`;
3. `hepta.servo.source_acceptance_review_challenge.v1`;
4. a manually authored `hepta.servo.accepted_source_pointer.v1`.

The tool has no command that creates the accepted pointer. It can only create the review challenge and verify a proposed pointer.

## Review challenge

The challenge binds:

- candidate ID, SHA-256 and byte length;
- policy ID, SHA-256 and byte length;
- exact Hepta repository/ref/commit/tree;
- pinned Servo commit/tree and recomputed tree;
- exact source workflow run ID and attempt;
- the post-acceptance negative claim set;
- the requested decision `ACCEPT_EXACT_SERVO_SOURCE_ONLY`.

The challenge is create-only, mode `0600`, fsynced and self-bound by a domain-separated SHA-256 ID.

## Dedicated review PR

A real pointer proposal must be a non-draft pull request on a branch beginning:

```text
review/hepta-servo-source-acceptance-
```

It must target `integration/vnext-main-20260811` in the same repository and may change only:

- the canonical accepted-source pointer;
- the exact candidate snapshot named by candidate ID;
- the small set of current/status/readme pointer files listed by policy.

The review evidence must contain at least one distinct current-head approval from an OWNER, MEMBER or COLLABORATOR. The reviewer must differ from the PR author and must include an exact line:

```text
HEPTA_SOURCE_ACCEPT_V1 <challenge_id>
```

A current-head `CHANGES_REQUESTED`, stale approval, self-approval, untrusted association, draft PR, unknown changed path or missing candidate snapshot fails closed.

## Output boundary

`verify-pointer` proves only internal byte and claim binding. It reports that live review evidence is still required.

`verify-live-review` proves that the proposed pointer and the current PR review evidence satisfy the policy. It still does not build, link, launch or qualify Servo, and it never updates the repository pointer.

## Remaining hard gates

No real acceptance may occur until:

- canonical exact-source qualification v3 runs with a real runner and executable steps;
- raw run/job/artifact API JSON and retained source bytes are preserved;
- candidate v2 is compiled and independently reviewed;
- a dedicated pointer PR receives current-head approval;
- the resulting pointer is merged under repository review rules.

After source acceptance, a separate worker source/API topology receipt and pointer must still be reviewed before Worker crate or build work.
