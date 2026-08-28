# WEB-C1 exact Servo source acceptance pointer v1

Status: **live-review workflow implemented; fixtures pass; no real candidate, approval, or accepted pointer exists**

## Purpose

The exact-source review candidate v2 proves that retained workflow evidence and source bytes are internally consistent, but deliberately leaves source acceptance false. This slice defines the separate review boundary that may convert one immutable candidate into a source-only accepted pointer.

The acceptance pointer does not authorize a build. Its only positive claims are:

```text
exact_servo_source_accepted=true
source_review_candidate_accepted=true
```

Worker topology acceptance, recipe acceptance, build authorization, Servo build/runtime qualification, operator acceptance, promotion and release remain false. All machine, runtime, product, effect, network, credential and execute authority remains false.

## Inputs

The verifier binds four canonical JSON objects:

1. `hepta.servo.exact_source_review_candidate.v2`;
2. `hepta.servo.source_acceptance_review_policy.v1`;
3. `hepta.servo.source_acceptance_review_challenge.v1`;
4. a manually authored `hepta.servo.accepted_source_pointer.v1`.

The tool has no command that creates the accepted pointer. It can only create the review challenge and verify a proposed pointer and live review evidence.

## Immutable snapshots

A proposal must retain both immutable inputs under deterministic paths:

```text
source-acceptance/candidates/<candidate-id-digest>.json
source-acceptance/challenges/<challenge-id-digest>.json
```

The pointer binds both snapshot paths, byte lengths, SHA-256 digests and self-bound IDs. Missing or differently named snapshots fail closed.

## Review challenge

The create-only challenge binds candidate bytes, policy bytes, exact Hepta repository/ref/commit/tree, pinned Servo commit/tree, exact source workflow run and attempt, the post-acceptance negative claim set, and the requested decision `ACCEPT_EXACT_SERVO_SOURCE_ONLY`.

The challenge is written mode `0600`, fsynced and self-bound by a domain-separated SHA-256 ID.

## Dedicated review PR

A real proposal must use a non-draft pull request on a branch beginning:

```text
review/hepta-servo-source-acceptance-
```

It must target `integration/vnext-main-20260811` in the same repository. The PR may change only the pointer, deterministic candidate and challenge snapshots, and the small auxiliary pointer/status files listed by policy.

Because the PR number is assigned only after opening the review lane, the author may first open the PR with the immutable candidate and challenge snapshots, then add the pointer in its final pre-review commit. The pointer must bind the assigned pull-request number and exact head ref.

The verifier intentionally does **not** claim that GitHub `author_association` proves CODEOWNER identity. Instead it requires a distinct current-head reviewer with association `OWNER`, `MEMBER`, or `COLLABORATOR`. The reviewer must differ from the PR author. Repository rules may add a separate CODEOWNER requirement, but this v1 receipt never fabricates one.

Every approval must include one exact line:

```text
HEPTA_SOURCE_ACCEPT_V1 <challenge_id>
```

A current-head `CHANGES_REQUESTED`, stale approval, self-approval, untrusted association, draft PR, PR-number/head-ref mismatch, unknown changed path, or missing candidate/challenge snapshot fails closed.

## Live status check

The required status check is:

```text
Source-only accepted pointer live review
```

`.github/workflows/hepta-servo-exact-source-acceptance-live-review.yml`:

- passes explicitly as not applicable outside the dedicated branch prefix;
- checks out the exact proposed head;
- resolves the pointer, candidate and challenge inside the repository root;
- fetches paginated PR reviews and changed files plus the exact PR and head-commit API objects using read-only permissions;
- stores bounded raw API evidence mode `0600`;
- runs `verify-live-review` against the current head;
- uploads evidence without modifying the pointer.

## Output boundary

`verify-pointer` proves internal byte and claim binding and reports that live review evidence is still required.

`verify-live-review` proves that the proposed pointer and current GitHub review evidence satisfy policy. It still does not build, link, launch or qualify Servo, and never creates or updates the repository pointer.

## Remaining hard gates

No real acceptance may occur until canonical exact-source qualification v3 records executable steps; raw run/job/artifact JSON and retained source bytes are preserved; candidate v2 and challenge v1 are compiled; a dedicated pointer PR receives the exact current-head approval and required status check; and the source-only pointer is merged under repository rules.

After source acceptance, a separate Worker source/API topology receipt and pointer must still be reviewed before Worker crate or build work.
