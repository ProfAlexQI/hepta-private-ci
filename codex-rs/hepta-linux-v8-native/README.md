# Hepta Linux qualification v8 native core

Status: `MODEL_AND_DISPOSABLE_TEST_CORE_ONLY / HARD_NO_GO_FOR_LIVE_USE`.

This crate contains fail-closed Linux syscall and durable-state primitives for
the v8 qualification successor. It is deliberately not an admission daemon,
recovery service, installer, signal executor, cgroup controller, qualification
driver, relay publisher, or target authorization boundary.

Current code may be compiled and exercised only in an isolated development
directory or explicitly disposable fault-injection environment. It grants no
authority to install root files, reload or start systemd units, signal runner
processes, execute the product candidate on the target, mutate refs or
production state, or publish a qualification PASS.

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
- a frozen root-owned state-root/install epoch plus an external monotonic
  anti-rollback watermark;
- a root-owned single-writer admission/recovery implementation;
- expected-peer/capability authorization layered over the kernel IPC identity;
- target collector, cgroup, runner lifecycle, relay, and final-receipt bindings;
- durable crash/reboot and adversarial test receipts from a disposable Linux
  environment;
- exact install and one-shot run authorities for the intended target.
