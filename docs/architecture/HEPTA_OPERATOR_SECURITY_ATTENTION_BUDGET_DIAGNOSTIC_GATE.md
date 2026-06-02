# Hepta Operator Security Attention Budget Diagnostic Gate

This gate turns the transient operator-security attention path into a
deterministic, read-only diagnostic contract.

## Scope

The gate reads only these local Hepta routes:

- `/api/operator-security`
- `/api/telegram-production-readiness`
- `/api/telegram-owner-handoff`
- `/api/telegram-poll-loop`

It does not restart launchd, reset observations, advance cursors, perform a live
Telegram read, send a message, mutate Gateway state, or persist evidence.

## Classification

The gate accepts these known states:

- `ready`: Telegram production readiness is ready and the attention budget is OK.
- `warming_observation_budget`: the poll-loop observation window is still
  warming.
- `attention_budget_exceeded`: Telegram production readiness has exceeded the
  bounded attention budget.
- `known_telegram_production_readiness_attention`: `/api/operator-security`
  maps its attention state to one of the known Telegram production readiness
  states.
- `legacy_owner_coexistence_attention`: operator security is intentionally
  attentioned because Telegram replacement has not been requested.
- `known_conflict_risk_double_poller_observation`: `/api/telegram-owner-handoff`
  reports `active_owner=conflict_risk` with `double_poller_risk=true`, while
  `/api/telegram-poll-loop` remains only observed as `armed` or `gated`.

Unexpected states fail the gate with a compact report, while known transient
states remain visible before later watchdog gates make their own pass/fail
decision.

## Boundaries

The report deliberately keeps these values false:

- `service_restarted`
- `launchd_mutated`
- `cursor_written`
- `live_read_performed`
- `telegram_send_performed`
- `owner_handoff_performed`
- `evidence_persisted`
- `credential_read`
- `secret_file_read`

This is a diagnostic gate only. It is not a recovery mechanism and does not
authorize public release claims, artifact writes, runtime activation, or
operator approval recording. A known conflict-risk/double-poller observation is
classified so preflight can report the blocker deterministically, but it still
does not authorize owner handoff, Telegram reads or sends, service restart,
launchd mutation, cursor writes, or evidence persistence.
