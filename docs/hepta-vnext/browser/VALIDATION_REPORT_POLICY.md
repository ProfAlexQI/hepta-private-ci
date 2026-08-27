# Hepta Browser validation-report policy

Local and hosted validation answer different questions and neither can silently substitute for the other.

## Local isolated validation

Local validation may prove that a checked-out candidate passes deterministic Python fixtures, static contract verifiers and focused Rust tests in one container. The report must bind the exact Git commit/tree and omit machine-local paths from any repository-persistent form.

A local pass cannot establish:

- GitHub merge-gate success;
- independent exact Servo source acquisition;
- retained workflow artifacts;
- cross-platform qualification;
- production or release authority.

## GitHub exact-head validation

Hosted checks must run on the exact PR head or PR merge candidate and must retain their own evidence. A check that fails before recording steps is an infrastructure/policy blocker, not a code-test result. An older green run cannot qualify a newer head.

## Source, build and runtime evidence

Source acceptance requires the manual exact-source v3 workflow plus a separately reviewed pointer. A build requires accepted source/toolchain/recipe, a sealed real build-input manifest and a real preflight receipt. Runtime qualification additionally requires artifact/SBOM binding, sandbox/listener/egress evidence and one real local-fixture Servo WebView.

## State transition

No script or workflow in this development slice may directly set operator acceptance, promotion, release qualification or production authority. Those fields remain false until separate reviewed ceremonies bind one exact source, artifact and evidence set.
