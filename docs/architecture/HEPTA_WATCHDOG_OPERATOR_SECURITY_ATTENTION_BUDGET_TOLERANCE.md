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

The watchdog and live-soak reports also surface the diagnostic classification so
the terminal summary does not collapse known legacy-owner warm-up into an
ambiguous unknown. In legacy-owner coexistence mode, a gated Telegram production
readiness state with `attention_budget_ok=true`, disabled Hepta poll loop, stale
observation, and no double-poller risk is reported as
`warming_observation_budget`. That classification remains read-only and does not
authorize Telegram ownership handoff, live reads, sends, evidence persistence,
service restart, public release claims, or artifact distribution.
