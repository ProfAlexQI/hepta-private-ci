# Hepta Watchdog Operator Security Attention Budget Tolerance

Date: 2026-05-27

The active watchdog now treats a bounded operator-security attention-budget
state as operationally healthy when every surrounding Telegram production guard
is still intact.

The tolerated state is narrow:

- `/api/operator-security` reports `attention`
- the attention reason is `security_gate_not_ready`
- the nested Telegram production readiness blocker is exactly
  `attention_budget_exceeded`
- recent Bot API evidence is OK
- observation, cursor, delivery ledger, and poll-loop readiness remain fresh
- owner mode remains `parallel_bots`
- double-poller risk remains false
- native POST activation remains scoped to the approved dry-run handler

This does not enable recovery behavior. The watchdog still performs no service
restart, launchd mutation, cursor write, Telegram read/send, provider/model
invocation, public claim, release artifact write, credential read, or secret-file
read.

The reason for this tolerance is preflight stability: long local gate chains can
query the read-only Telegram production readiness surface often enough to trip
its attention budget, while the actual health evidence remains bounded and
fresh. The dedicated diagnostic gate remains the deeper classifier:

- `scripts/hepta-operator-security-attention-budget-diagnostic-gate.sh`
