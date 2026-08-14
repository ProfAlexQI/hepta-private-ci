# Hepta Linux qualification v8 native core

Status: `SIGNED_INSTALL_V2_FAIL_CLOSED / PREFLIGHT_ONLY / NO_LIVE_AUTHORITY`.

This crate contains fail-closed Linux syscall and durable-state primitives for
the v8 qualification successor. The `hepta-linux-v8ctl` binary now provides a
read-only exact install plan and a root-only, no-replace installer for three
current-build ELF files, two fixed systemd units, the complete trusted-state
root layout, and its single empty 0600 `state.lock`. The installed
`admissiond` and `recover` binaries are preflight-only: they open the compiled
production state root and its already-installed, never-replaced singleton lock
(runtime code never creates it), but grant no admission or recovery run
authority.

The admission service is the sole future cgroup owner. Its fixed delegated
root is `/sys/fs/cgroup/system.slice/hepta-linux-v8-admissiond.service`;
install plans and preflight reports bind that exact path. Recovery has no
delegation and sets `ProtectControlGroups=yes`, so it may not create child
cgroups. The unified `/sys/fs/cgroup` mount is only a host prerequisite and is
never treated as an owned writable qualification subtree.

`install-plan` may run unprivileged and is read-only. `install --execute`
requires uid 0, exact canonical plan bytes, a fresh plan digest, unchanged
Linux/cgroup-v2/systemd/syscall probes, unchanged pinned ELF bytes, and a
purpose-specific signed `InstallV2` authority that exactly binds the target
machine/boot and complete mutation inventory. Plan bytes or their SHA-256 are
not authorization. Plan, authority, and ELF inputs are bounded descriptor
reads beneath `openat2` anchors with symlink/magic-link rejection. File
publication uses deterministic same-directory staging, file fsync,
`renameat2(RENAME_NOREPLACE)`, parent fsync, exact retry/recovery, and a final
closed-world inventory replay. The installer never enables or starts the
units. It grants no authority to reload or start systemd units, signal runner
processes, execute the product candidate, mutate refs or production state, or
publish a qualification PASS.

The Linux test harness launches a separate test process and exits it after
successful syscall-return boundaries around create, write, file fsync,
no-replace rename, directory fsync, and final reopen. It verifies fail-closed
restart behavior for active-attempt, nonce, and journal publication. This is a
process-crash model only: it does not claim mid-syscall interruption, power
loss, page-cache loss, filesystem replay, torn-sector, ENOSPC, EIO, or reboot
coverage.

Live use remains blocked until all of the following exist and are independently
frozen and audited:

- published purpose-specific trust profiles;
- a frozen root-owned install epoch plus an independently keyed, separately
  hosted durable CAS watermark provider that publishes exact genesis/current
  tip and commit acknowledgement contracts;
- admission/recovery state machines beyond the current root-owned preflight
  opener and singleton-lock boundary;
- expected-peer/capability authorization layered over the kernel IPC identity;
- target collector, cgroup, runner lifecycle, relay, and final-receipt bindings;
- durable crash/reboot and adversarial test receipts from a disposable Linux
  environment;
- exact one-shot run authority for the intended target.

The production trust profile and external watermark provider are deliberately
not compiled or injectable from CLI input. Until independently published and
pinned, production signature verification fails closed before any privileged
mutation.

## Two-phase target bootstrap (no circular self-trust)

The target-bound desktop profile must be produced in two phases and converge
on one final source HEAD for every sealed qualification artifact:

1. Build an initial installer/preflight collector with the
   production state-root profile deliberately absent. It is permanently
   `NO_AUTHORITY`: it may derive descriptor-bound machine, root, filesystem,
   mount, namespace, layout, unit, and exact six-process runner22/23 profile
   bytes, but it cannot admit, signal, activate, or publish PASS.
2. Export those derived bytes for independent external audit. After the audit
   freezes the exact desktop machine/root/layout/runner pins, commit those
   constants into the single final source HEAD and rebuild every sealed binary
   from that HEAD.
   Only an exclusive, versioned install/replay of those exact final binary
   bytes may be considered for live authority. The final helper and units use
   new versioned paths/names; they never replace a previously trusted helper
   or unit in place.

The remaining bridge is intentionally absent today: no audited desktop profile
constants are compiled, no exclusive versioned activation/replay state machine
exists, and preflight therefore cannot mint live authority. A bootstrap
collector receipt is evidence for phase two, never an authorization input to
phase one.
