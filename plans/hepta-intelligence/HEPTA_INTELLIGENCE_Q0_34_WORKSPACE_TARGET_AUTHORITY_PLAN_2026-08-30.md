# Hepta Intelligence Q0.34 — Canonical Workspace and Generated-Target Authority

Status: `IMPLEMENTATION_CANDIDATE / FAIL_CLOSED / NO_RUNTIME_AUTHORITY`

## 1. Exact dependency

Q0.34 is a strict single-parent successor to Q0.33:

```text
repository = ProfHepta/hepta-private-ci
parent PR = #163
parent branch = codex/hepta-intelligence-q0-33-setup-token-job-boundary-20260830
parent commit = 7403616a93ac57a8e7a557c5705c21ddb78e008a
parent tree = 80ae115eb88aae56356bef1a366366526ec14494
```

No Q0.33 source path is replaced by an older sibling. The setup-token job boundary remains intact.

## 2. Blockers closed

### Q0.34-A — stale source-verifier binding

Q0.33 changes `.github/actions/setup-bazel-ci/action.yml` to Git blob
`890567be46f3fd78c11b89a20950bef2f7af4bf6`, but the inherited Q0.29 source
verifier still requires the superseded blob
`ac4f5aa97c7556f6049bd1d0a33220759d9d13d1`.

Because the qualification fixture executes that verifier, Q0.33 cannot pass its
source gate on a real runner. Q0.34 updates the compatibility ratchet while
preserving every Q0.29 runtime-policy invariant.

### Q0.34-B — final workspace authority

Q0.32 verifies the repository workspace while resolving Bazelisk, but the final
Windows direct-Bazel subprocess did not set `cwd`. The checked workspace and
the workspace Bazel evaluates could therefore diverge.

Q0.34 returns the canonical resolved `GITHUB_WORKSPACE` from the final policy
and passes it explicitly as the direct Bazel subprocess working directory.
Non-qualifying and non-Windows paths retain their previous behavior.

### Q0.34-C — exact test and Clippy target vectors

Q0.29 checked release targets exactly, but test shards only required positive
workspace labels and Clippy only required a broad prefix/domain. A generator
omission or substitution could therefore retain a valid-looking command.

Q0.34 independently replays the reviewed target generators with the same
digest-bound direct Bazel executable, strict startup vector, canonical
workspace, and runner-controlled repository caches:

- Windows test query:
  `tests(//...) except tests(//third_party/v8:all) except attr(tags, "manual", tests(//...))`;
- test labels are ASCII-sorted and assigned with the exact POSIX `cksum`
  algorithm used by the workflow;
- Windows Clippy query:
  `kind("rust_test rule", attr(tags, "manual", //codex-rs/... except //codex-rs/v8-poc/...))`;
- native-only `*-test-bin` helpers are removed and the two canonical Clippy
  prefix entries are prepended;
- observed final targets must equal the recomputed vector in content and order;
- mismatch diagnostics expose only counts and SHA-256 digests.

Release targets remain governed by the existing exact Q0.29 tuple.

## 3. Changed-path envelope

The intended source delta is limited to:

```text
.github/scripts/run_bazel_q034_workspace_targets.py
.github/scripts/run_bazel_with_buildbuddy.py
.github/scripts/test_run_bazel_workspace_targets.py
.github/scripts/test_run_bazel_qualification_boundary.sh
.github/workflows/windows-gnullvm-qualification-boundary.yml
scripts/verify-windows-gnullvm-job-executable.py
scripts/verify-windows-gnullvm-direct-bazel.py
plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_34_WORKSPACE_TARGET_AUTHORITY_PLAN_2026-08-30.md
plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_34_STATUS.json
```

No Rust runtime, SQL migration, product caller, model/provider, network/effect,
`CALLERS`, promotion, or release path is changed.

## 4. Required executable evidence

A source commit is not qualification. The strict Q0.34 head and the refreshed
A0-first-parent integration candidate must obtain assigned runners, non-empty
steps, and terminal success for:

1. every existing `test_run_bazel*.py` regression;
2. all Q0.17–Q0.34 source verifiers;
3. a real pinned setup-bazel run followed by an empty/absent job-scoped token;
4. four Windows gnullvm test shards with exact regenerated target equality;
5. Windows gnullvm strict Clippy with exact regenerated target equality;
6. exact release target equality;
7. final direct Bazel launch with canonical workspace `cwd`;
8. full Rust, Bazel, SDK, policy, platform, blob, spelling, P0, and v8 admission;
9. clean worktree and exact commit/tree attribution.

Queued, pending, skipped, cancelled, runner-zero, empty-step, stale, source-only,
or superseded evidence is not PASS.

## 5. Authority boundary

```text
a0_candidate_qualified = false
independent_review = false
selected = false
full_repository_merge_green = false
live_ruleset_applied = false
runtime_wired = false
production_authority = false
operator_acceptance = false
promotion = false
release_authority = false
callers_ratchet = false
```

Q0.34 is repository-qualification repair only. It cannot issue independent
review, canonical selection, operator acceptance, production, promotion,
release, or `CALLERS` authority.
