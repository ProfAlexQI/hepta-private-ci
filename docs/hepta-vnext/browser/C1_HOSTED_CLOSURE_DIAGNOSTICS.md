# C1 hosted closure diagnostics

Status: `DEVELOPMENT / QUALIFICATION_ONLY`

The exact-head hosted closure workflow writes a bounded text log for every attempted lane and uploads it even when the closure producer fails. The log is diagnostic evidence only. It cannot accept Servo source or topology, authorize a build, grant runtime authority, approve a merge, promote, or release.

A closure lane remains failed unless the producer exits successfully. The workflow retains `set -o pipefail`, exact-head compare-and-swap checks, a normal non-force push, and all negative-authority boundaries.
