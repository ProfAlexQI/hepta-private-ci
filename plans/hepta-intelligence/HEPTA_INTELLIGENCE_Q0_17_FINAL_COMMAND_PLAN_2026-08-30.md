# Hepta Intelligence Q0.17 — Bazel final-command and rc-input ratchet

## Status

Q0.17 is a bounded source commit plus a metadata-only receipt commit,
and is a qualification-only successor to Q0.16. It closes the
remaining gap between the shell wrapper's canonical argument construction and
the command that the Bazel client actually receives.

Q0.16 validates explicit wrapper arguments, but Bazel also consumes startup rc
sources and accepts alternate/split option forms. Repeated options, workspace
imports, runner-local rc files, endpoint flags, strategy overrides, and explicit
action/test environment injection therefore require a final gate immediately
before Bazel execution.

## Exact base

```text
repository = ProfHepta/hepta-private-ci
base PR = #119
base branch = codex/hepta-intelligence-q0-16-keyless-gnullvm-exact-set-20260830
base commit = cb685205d396b2b73e69b8f0d7049516749dcd4a
base tree = 5c5d6a01d357ef1be0dc9aae73ba4e3189a24fcc
```

The implementation commit and tree are bound by a second metadata-only receipt
commit after Git object creation. The PR body binds both objects. Source validation does not
   constitute executable Windows or
repository qualification.

## Required behavior

For credential-free Windows gnullvm GitHub Actions invocations:

1. master, system and home rc files are disabled;
2. implicit workspace rc discovery is disabled;
3. exactly one reviewed workspace `.bazelrc` is loaded by absolute path;
4. the `.bazelrc` Git blob must equal
   `0736ecbb6e8183b31f0e2739abef901c47235e9d`;
5. the optional workspace `user.bazelrc` import must be absent;
6. the final command must use one canonical `ci-windows` config followed by the
   exact host, target, C/C++ discovery, execution-platform, toolchain, local
   strategy, concurrency, test-environment and evidence-tag options;
7. split-form authority options, remote endpoints, additional strategies,
   non-canonical action/host/test environments, duplicate exact options and
   option smuggling after the target separator fail closed;
8. the authenticated BuildBuddy/RBE path remains unchanged;
9. the manual MSVC diagnostic remains non-qualifying and outside this gate.

## Validation

Source-level validation for this tranche is:

```text
python3 -m py_compile .github/scripts/run_bazel_with_buildbuddy.py
python3 .github/scripts/test_run_bazel_final_command.py
python3 -m unittest discover -s .github/scripts -p 'test_run_bazel*.py'
bash -n .github/scripts/test_run_bazel_qualification_boundary.sh
```

A PASS requires fresh assigned-runner execution on both the exact source head
and its synthetic merge candidate. Queued, cancelled, runner-zero, empty-step,
source-only or superseded results are not qualification.

## Authority boundary

```text
a0_candidate_qualified = false
selected = false
full_repository_merge_green = false
runtime_wired = false
production_authority = false
operator_acceptance = false
promotion = false
release_authority = false
callers_ratchet = false
```
