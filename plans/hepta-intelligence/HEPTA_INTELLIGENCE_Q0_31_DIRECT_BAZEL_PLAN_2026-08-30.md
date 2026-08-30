# Hepta Intelligence Q0.31 — Direct Bazel CAS and pre-launch authority closure

## Mission

Close the final repository-controlled executable-identity gaps left after canonical Q0.29 while preserving the Q0.30 canonical-branch CI governance repair.

Q0.31 is a repository-qualification tranche only. It does not change Hepta runtime or product behavior and grants no A0 selection, operator, production, promotion, release, or `CALLERS` authority.

## Exact stack

```text
repository = ProfHepta/hepta-private-ci
selected Q0.28 PR = #146
selected Q0.28 commit = f6031d119092ca9a71e109cd94cdc56e81a3e884
selected Q0.29 PR = #150
selected Q0.29 commit = 925f2871754f89c31051080e20a9bd0448302245
Q0.30 governance PR = #154
Q0.30 parent commit = 043a9a7120693ddb9296c6cfdff03475511d70cb
Q0.30 parent tree = d0f750d9c1976d06967176e99154e751311499cb
superseded sibling = PR #155 / 58f7df731a8c0febd8118be9a34cd69663089253
```

## Root causes

### Canonical Clippy exclusion

The actual reviewed Windows Clippy lane excludes exactly `-//codex-rs/v8-poc:all`, but the inherited Q0.26 policy rejected every negative Clippy target. Q0.31 permits only this one reviewed exclusion and rejects every other negative target.

### Exact output base

The CI setup exports both `BAZEL_OUTPUT_USER_ROOT` and an explicit setup-bazel `BAZEL_OUTPUT_BASE`. Q0.31 requires both values and includes both in the exact startup vector. Missing, conflicting, reordered, duplicate, or additional startup controls fail closed.

### Direct cached Bazel identity

Q0.29 verified Bazelisk immediately before launch and pinned the desired Bazel asset digest, but execution still entered Bazelisk rather than a freshly rehashed cached Bazel object. Q0.31:

1. verifies the official Bazelisk executable;
2. calls its bounded `--print_env` resolution path;
3. requires exactly one child `PATH` binding;
4. requires the child executable at the Bazelisk content-addressed suffix `downloads/sha256/<reviewed digest>/bin/bazel.exe`;
5. verifies the cached Bazel SHA-256;
6. preserves the verified child `PATH` for nested Bazel invocations;
7. executes the direct cached Bazel path;
8. rehashes that exact executable and reruns the Q0.28/Q0.29 command, job, runner, path, metadata, shard and log contracts immediately before process launch.

Bazelisk 1.28.1 and Bazel 9.0.0 remain bound to their official Windows x86-64 asset digests.

## Compatibility and migration rules

- `run_bazel_q029_execution_context.py` remains byte-identical and retains its 23 focused tests.
- The public wrapper keeps Q0.29 compatibility function names so its launch-order tests remain valid.
- Q0.31 replaces only the final Bazelisk-mediated execution step with the verified direct CAS Bazel execution step.
- Authenticated BuildBuddy/RBE and non-keyless/non-Windows paths remain passthrough behavior.
- The existing Q0.30 canonical-branch workflow coverage remains part of the parent tree.

## Required source gates

```text
Python bytecode compilation for all changed Python files
all 23 Q0.29 execution-context tests
all Q0.28 startup tests, including output_base
all Q0.26/Q0.27 lane tests
all Q0.31 direct Bazel tests
final-command, lane, startup, execution-context and direct-Bazel source verifiers
boundary Bash syntax
workflow YAML parse
exact changed-path allowlist
single-parent topology
clean worktree
```

## Required executable gates

The final exact head and its GitHub synthetic merge candidate must both obtain assigned runners, non-empty steps, and terminal success for the complete repository matrix. In particular, all four Windows gnullvm test shards, Windows strict Clippy and Windows release build must execute the exact direct Bazel CAS object. Execution logs must show no recurrence of the mixed-ABI rustls-provider/AWS-LC/SQLite failures.

A queued, pending, skipped, runner-zero, empty-step, cancelled, stale-head, source-only, sibling, or synthetic-only receipt is not PASS.

## External boundary

This tranche cannot issue independent A0 review, canonical selection, live ruleset enforcement, reviewed corpus/model-license evidence, physical hardware/device evidence, long-soak evidence, operator acceptance, promotion, release, or `CALLERS` authority.
