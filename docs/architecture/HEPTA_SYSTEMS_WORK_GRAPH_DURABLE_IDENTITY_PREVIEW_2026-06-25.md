# Hepta Systems WorkGraph Durable Identity Preview

Date: 2026-06-25

This note pins the smallest durable WorkGraph identity contract before any
runtime persistence, replay, rollback, or promotion is enabled.

## Canonical Fields

- `workflow_id`: stable workflow scope, derived from trace/source fields.
- `run_id`: scheduler, worker, or job attempt scope.
- `step_id`: plan, task, node, or edge step identity.
- `checkpoint`: WAL-derived checkpoint evidence pointer.
- `replay_key`: deterministic replay key from validated WAL inputs.
- `rollback_anchor`: named checkpoint/recovery anchor required before rollback.
- `receipt_hash`: redacted evidence hash required before promotion or audit.

## Bound Existing Previews

- State-store persistence preview supplies WAL records, checkpoint contracts,
  idempotency guards, and readback probes.
- Replay/readback preview supplies replay stages, drift detectors, recovery
  previews, and redacted readback evidence.
- Promotion precondition remains the next preview gate; durable runtime is not
  enabled by this note.

## Promotion Precondition Binding

`hepta_work_graph_promotion_precondition_preview_gate` now requires the durable
identity preview as a prior gate. Every promotion target has a
`durable_identity_evidence_ready` check, and every audit receipt requires the
seven canonical fields alongside the legacy readback/audit fields.

## Activation Enforcement Binding

`hepta_work_graph_activation_enforcement_blocker_preview_gate` also requires the
durable identity preview as a prior gate. Every activation surface is blocked by
`durable_identity_evidence_missing`, and the `durable_identity_evidence_packet`
enablement remains unsatisfied in preview mode.

## Shadow Adapter Readback Binding

`hepta_work_graph_shadow_adapter_readback_preview_gate` also requires the
durable identity preview as a prior gate. Collection readbacks and shadow
evidence packets carry the seven canonical fields before projected/readback hash
comparison, while shadow execution and adapter enforcement remain disabled.

## Persistence Feature Flag Binding

`hepta_work_graph_persistence_feature_flag_preview_gate` also requires the
durable identity preview as a prior gate. Every persistence/replay feature flag
requires `durable_identity_evidence_packet`, which remains unsatisfied while all
feature flags stay default-off and non-mutable in preview.

## Persistence Canary Dry-Run Binding

`hepta_work_graph_persistence_canary_dry_run_preview_gate` also requires the
durable identity preview as a prior gate. Every canary dry-run scenario requires
`durable_identity_evidence_packet`, rollback receipt previews include the seven
canonical fields, and canary traffic, writes, feature-flag mutation, replay,
rollback, promotion, scheduler cutover, and live persistence all remain disabled.

## Persistence Canary Readback Receipt Binding

`hepta_work_graph_persistence_canary_readback_receipt_preview_gate` also requires
the durable identity preview as a prior gate. Every canary receipt contract
requires the seven canonical fields, durable identity has its own digest check
and denial reason, and local operator/auditor views include durable identifiers
without enabling receipt persistence or external delivery.

## Persistence Promotion Blocker Binding

`hepta_work_graph_persistence_promotion_blocker_preview_gate` also requires the
durable identity preview as a prior gate. Promotion blockers, release denials,
operator acknowledgements, and quarantine previews all carry durable identity
evidence before any persistence promotion, release publication, or live
execution can be considered.

## Persistence Shadow/Live Readback Comparison Binding

`hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate` also
requires the durable identity preview as a prior gate. Shadow/future-live
readback pairs carry durable identity fields, durable identity absence is a
critical mismatch classifier with its own promotion denial, and local comparison
views keep durable identifiers while live readback and comparison execution stay
disabled.

## Persistence Enforcement Rollout Blocker Binding

`hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate` also
requires the durable identity preview as a prior gate. Rollout stages, operator
enablement packets, rollback owner receipts, release denials, and traffic ramp
blockers all require the seven canonical fields before any future enforcement
rollout can be considered. Traffic remains at 0 ppm, operator approval remains
unrecorded, and enforcement, release publication, scheduler cutover, live
persistence, and external delivery stay disabled.

## Persistence Operator Readiness Packet Binding

`hepta_work_graph_persistence_operator_readiness_packet_preview_gate` also
requires the durable identity preview as a prior gate. Every operator readiness
packet template includes a `durable_identity_section` with the seven canonical
fields, validation denies missing durable identity evidence, and the acceptance
guard set includes `guard_durable_identity_evidence_declared`. Packet sections
remain incomplete and redacted/hash-only, operator acceptance is false, approval
recording is false, and readiness packet persistence, enforcement rollout,
release publication, live persistence, and external delivery stay disabled.

## Persistence Operator Readiness Receipt Binding

`hepta_work_graph_persistence_operator_readiness_receipt_preview_gate` also
requires the durable identity preview as a prior gate. Every readiness receipt
contract carries the seven canonical fields before hash-only receipt fields,
durable identity has its own digest check, missing durable identity blocks
signature/acceptance paths, and all local readback views include durable
identifiers. Receipt persistence, operator acceptance, approval recording, live
readback, enforcement rollout, release publication, and external delivery stay
disabled.

## Persistence Operator Readiness Receipt Acknowledgement Binding

`hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every readiness
receipt acknowledgement contract and local operator/auditor/release/authority
view carries the seven canonical fields, missing durable identity evidence is a
non-acceptance reason, acknowledgement recording is denied for durable identity
gaps, and the acknowledgement invariant set requires durable identity evidence.
Acknowledgement recording, operator acceptance, approval recording, authority
grant, enforcement rollout, release publication, live persistence, and external
delivery stay disabled.

## Persistence Acceptance Authority Blocker Binding

`hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate` also
requires the durable identity preview as a prior gate. Every authority surface,
escalation guard, required authority record, and local authority view carries the
seven canonical fields. Missing durable identity evidence is an authority
denial, and the invariant set requires durable identity evidence before any
authority blocker could clear. Operator acceptance, approval recording, authority
grant, live persistence, WAL/checkpoint writes, enforcement rollout, release
publication, and external delivery stay disabled.

## Persistence Acceptance Record Intake Binding

`hepta_work_graph_persistence_acceptance_record_intake_preview_gate` also
requires the durable identity preview as a prior gate. Every acceptance record
template, intake guard, redaction digest, and local intake view carries the
seven canonical fields. Missing durable identity evidence is a validation
denial, and the invariant set requires durable identity evidence before any
acceptance record could be considered for intake. Acceptance record persistence,
operator acceptance, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, release publication, and external delivery stay
disabled.

## Persistence Acceptance Record Receipt Binding

`hepta_work_graph_persistence_acceptance_record_receipt_preview_gate` also
requires the durable identity preview as a prior gate. Every acceptance record
receipt contract, digest check, and local receipt readback view carries the
seven canonical fields. Missing durable identity evidence is a receipt denial,
and the invariant set requires durable identity evidence before any acceptance
record receipt can advance toward acknowledgement. Acceptance record receipt
persistence, operator acceptance, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, release publication, and external
delivery stay disabled.

## Persistence Acceptance Record Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every acceptance
record receipt acknowledgement contract and local acknowledgement view carries
the seven canonical fields. Missing durable identity evidence is a
non-acceptance reason, acknowledgement recording is denied for durable identity
gaps, and the invariant set requires durable identity evidence before any
acknowledgement can advance toward effect application blocking. Acknowledgement
recording, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, release publication, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Blocker Binding

`hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate`
also requires the durable identity preview as a prior gate. Every acceptance
effect surface, apply guard, and local effect blocker view carries the seven
canonical fields. Missing durable identity evidence is an effect application
blocker across all acceptance effect surfaces, and the invariant set requires
durable identity evidence before any effect can advance toward denial receipt.
Effect application, acknowledgement recording, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
release publication, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate`
also requires the durable identity preview as a prior gate. Every effect denial
receipt contract and local operator/auditor/release/runtime denial receipt view
carries the seven canonical fields. Missing durable identity evidence is a
receipt mismatch denial, `check_durable_identity_digest` blocks receipt
acceptance, and the invariant set requires durable identity evidence before any
denial receipt can advance toward acknowledgement. Denial receipt persistence,
effect application, acknowledgement recording, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
release publication, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every denial receipt
acknowledgement contract and local operator/auditor/release/runtime
acknowledgement view carries the seven canonical fields. Missing durable
identity evidence is a non-acceptance reason, acknowledgement recording is
denied for durable identity gaps, and the invariant set requires durable
identity evidence before any acknowledgement can advance toward replay and
idempotency preview. Acknowledgement recording, denial receipt persistence,
effect application, acceptance recording, approval recording, authority grant,
live persistence, WAL/checkpoint writes, rollout, release publication, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate`
also requires the durable identity preview as a prior gate. Every replay
scenario, idempotency guard, monotonicity check, and local
operator/auditor/release/runtime replay view carries the seven canonical fields.
Missing durable identity evidence is a replay denial, and the invariant set
requires durable identity evidence before replay/idempotency evidence can
advance toward retention expiry preview. Replay execution, idempotency index
mutation, acknowledgement recording, denial receipt persistence, effect
application, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, release publication, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate`
also requires the durable identity preview as a prior gate. Every retention
policy, supersession guard, and local operator/auditor/release/runtime retention
view carries the seven canonical fields. Missing durable identity evidence is a
garbage collection and retention denial, and the invariant set requires durable
identity evidence before retention expiry evidence can advance toward readback
receipt preview. Retention state persistence, expiry recording, garbage
collection, tombstone persistence, replay execution, idempotency index mutation,
acknowledgement recording, denial receipt persistence, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, release publication, and external delivery stay
disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate`
also requires the durable identity preview as a prior gate. Every retention
readback receipt, digest check, receipt guard, and local
operator/auditor/release/runtime readback view carries the seven canonical
fields. Missing durable identity evidence is a readback mismatch denial,
`check_durable_identity_digest` blocks receipt acceptance, and the invariant set
requires durable identity evidence before retention expiry readback evidence can
advance toward acknowledgement preview. Readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
replay execution, idempotency index mutation, acknowledgement recording, denial
receipt persistence, effect application, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
release publication, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every retention
readback acknowledgement contract and local operator/auditor/release/runtime
acknowledgement view carries the seven canonical fields. Missing durable
identity evidence is a non-acceptance reason, acknowledgement recording is
denied for durable identity gaps, and the invariant set requires durable
identity evidence before the acknowledgement can advance toward replay and
idempotency preview. Readback acknowledgement recording, readback receipt
persistence, retention state persistence, expiry recording, garbage collection,
tombstone persistence, replay execution, idempotency index mutation, denial
receipt persistence, effect application, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
release publication, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview as a prior gate. Every retention
readback acknowledgement replay scenario, idempotency guard, monotonicity check,
and local operator/auditor/release/runtime replay view carries the seven
canonical fields. Missing durable identity evidence is a replay denial, and the
invariant set requires durable identity evidence before replay/idempotency
evidence can advance toward terminal decision non-promotion preview. Replay
execution, idempotency index mutation, readback acknowledgement recording,
readback receipt persistence, retention state persistence, expiry recording,
garbage collection, tombstone persistence, denial receipt persistence, effect
application, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, release publication, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision surface, authority guard, release/delivery guard, and local
operator/auditor/release/runtime terminal decision view carries the seven
canonical fields. Missing durable identity evidence is a non-promotion denial,
and the invariant set requires durable identity evidence before terminal
decision evidence can advance toward receipt preview. Terminal decision
recording, status promotion, authority grant, release publication, rollout,
traffic routing, readback acknowledgement recording, replay execution,
idempotency index mutation, persistence, effect application, acceptance
recording, approval recording, live persistence, WAL/checkpoint writes, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt, digest check, receipt guard, and local
operator/auditor/release/runtime receipt view carries the seven canonical
fields. Missing durable identity evidence is a receipt mismatch denial, and the
invariant set requires durable identity evidence before terminal decision
receipt evidence can advance toward acknowledgement preview. Terminal decision
receipt recording/persistence, terminal decision recording/persistence, status
promotion, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt acknowledgement contract, expiry/replay guard,
and local operator/auditor/release/runtime acknowledgement view carries the
seven canonical fields. Missing durable identity evidence is both a
non-acceptance reason and a recording denial, and the invariant set requires
durable identity evidence before receipt acknowledgement evidence can advance
toward replay/idempotency preview. Terminal decision receipt acknowledgement
recording, terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
traffic routing, release publication, public claim recording, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt acknowledgement replay scenario, idempotency
guard, monotonicity check, and local operator/auditor/release/runtime replay
view carries the seven canonical fields. Missing durable identity evidence is a
replay denial, and the invariant set requires durable identity evidence before
receipt acknowledgement replay/idempotency evidence can advance toward terminal
decision receipt retention/expiry preview. Replay execution, idempotency index
mutation, terminal decision receipt acknowledgement recording, terminal decision
receipt recording/persistence, terminal decision recording/persistence, status
promotion, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt retention policy, supersession guard, and local
operator/auditor/release/runtime retention-expiry view carries the seven
canonical fields. Missing durable identity evidence is a garbage collection
denial, and the invariant set requires durable identity evidence before terminal
receipt retention/expiry evidence can advance toward readback receipt preview.
Retention state persistence, expiry recording, garbage collection, tombstone
persistence, terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt retention/expiry readback receipt, digest check,
receipt guard, and local operator/auditor/release/runtime readback view carries
the seven canonical fields. Missing durable identity evidence is a mismatch
denial, and the invariant set requires durable identity evidence before terminal
receipt retention/expiry readback receipt evidence can advance toward readback
acknowledgement preview. Readback receipt persistence, retention state
persistence, expiry recording, garbage collection, tombstone persistence,
terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt retention/expiry readback acknowledgement
contract and local operator/auditor/release/runtime acknowledgement view carries
the seven canonical fields. Missing durable identity evidence is a
non-acceptance reason and a recording denial, and the invariant set requires
durable identity evidence before terminal receipt retention/expiry readback
acknowledgement evidence can advance toward replay/idempotency preview.
Readback acknowledgement recording, readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision non-promotion receipt retention/expiry readback acknowledgement replay
scenario, idempotency guard, monotonicity check, and local
operator/auditor/release/runtime replay view carries the seven canonical
fields. Missing durable identity evidence is a replay denial, and the invariant
set requires durable identity evidence before replay/idempotency evidence can
advance toward terminal decision non-promotion preview. Replay execution,
idempotency index mutation, readback acknowledgement recording, readback receipt
persistence, retention state persistence, terminal decision receipt
recording/persistence, terminal decision recording/persistence, status
promotion, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision surface, authority guard, release/delivery guard, and local
operator/auditor/release/runtime terminal decision view carries the seven
canonical fields. Missing durable identity evidence is a non-promotion denial,
and the invariant set requires durable identity evidence before terminal
receipt retention/expiry readback acknowledgement terminal decision evidence
can advance toward terminal decision receipt preview. Terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, readback acknowledgement recording, readback receipt persistence,
retention state persistence, terminal decision receipt recording/persistence,
effect application, acceptance recording, approval recording, authority grant,
live persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision receipt, digest check, receipt guard, and local
operator/auditor/release/runtime receipt view carries the seven canonical
fields. Missing durable identity evidence is a receipt mismatch denial, and the
invariant set requires durable identity evidence before terminal receipt
retention/expiry readback acknowledgement terminal decision receipt evidence
can advance toward receipt acknowledgement preview. Terminal decision receipt
recording/persistence, terminal decision recording/persistence, status
promotion, replay execution, idempotency index mutation, readback
acknowledgement recording, readback receipt persistence, retention state
persistence, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision receipt acknowledgement contract, expiry/replay guard, and local
operator/auditor/release/runtime acknowledgement view carries the seven
canonical fields. Missing durable identity evidence is both a non-acceptance
reason and a recording denial, and the invariant set requires durable identity
evidence before terminal receipt retention/expiry readback acknowledgement
terminal decision receipt acknowledgement evidence can advance toward
acknowledgement replay/idempotency preview. Terminal decision receipt
acknowledgement recording, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, replay execution,
idempotency index mutation, readback acknowledgement recording, readback receipt
persistence, retention state persistence, effect application, acceptance
recording, approval recording, authority grant, live persistence, WAL/checkpoint
writes, rollout, traffic routing, release publication, public claim recording,
and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview as a prior gate. Every terminal
decision receipt acknowledgement replay scenario, idempotency guard,
monotonicity check, and local operator/auditor/release/runtime replay view
carries the seven canonical fields. Missing durable identity evidence is a
replay denial, and the invariant set requires durable identity evidence before
terminal receipt retention/expiry readback acknowledgement terminal decision
receipt acknowledgement replay/idempotency evidence can advance toward
retention/expiry preview. Replay execution, idempotency index mutation, receipt
acknowledgement recording, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, readback
acknowledgement recording, readback receipt persistence, retention state
persistence, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement replay/idempotency gate. Every terminal decision receipt
retention policy, supersession guard, and local
operator/auditor/release/runtime retention/expiry view carries the seven
canonical fields. Missing durable identity evidence is a garbage-collection
denial, and the invariant set requires durable identity evidence before
terminal receipt retention/expiry readback acknowledgement terminal decision
receipt retention/expiry evidence can advance toward readback receipt preview.
Retention state persistence, expiry recording, garbage collection, tombstone
persistence, replay execution, idempotency index mutation, receipt
acknowledgement recording, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, readback
acknowledgement recording, readback receipt persistence, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

The terminal decision report/gate now probes the current deep terminal decision
Rust module, the immediate readback acknowledgement replay/idempotency
report/gate, and the durable identity root. Its prior tail is fixed to that
immediate replay/idempotency gate plus the durable identity root instead of
expanding the older downstream report chain, avoiding a stale six-segment
terminal decision projection and preventing cyclic report recursion.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry gate. Every terminal decision receipt retention/expiry
readback receipt, digest check, receipt guard, and local
operator/auditor/release/runtime readback receipt view carries the seven
canonical fields. Missing durable identity evidence is a readback receipt
mismatch denial, and the invariant set requires durable identity evidence
before terminal receipt retention/expiry readback acknowledgement terminal
decision receipt retention/expiry readback receipt evidence can advance toward
readback acknowledgement preview. Readback receipt persistence, retention state
persistence, expiry recording, garbage collection, tombstone persistence,
replay execution, idempotency index mutation, receipt acknowledgement
recording, terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, effect application, acceptance
recording, approval recording, authority grant, live persistence, WAL/checkpoint
writes, rollout, traffic routing, release publication, public claim recording,
and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback receipt gate. Every terminal decision receipt
retention/expiry readback acknowledgement contract and local
operator/auditor/release/runtime acknowledgement view carries the seven
canonical fields. Missing durable identity evidence is a non-acceptance reason
and recording denial, and the invariant set requires durable identity evidence
before terminal receipt retention/expiry readback acknowledgement terminal
decision receipt retention/expiry readback acknowledgement evidence can advance
toward replay/idempotency preview. Readback acknowledgement recording, readback
receipt persistence, retention state persistence, expiry recording, garbage
collection, tombstone persistence, replay execution, idempotency index
mutation, receipt acknowledgement recording, terminal decision receipt
recording/persistence, terminal decision recording/persistence, status
promotion, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement gate. Every terminal decision receipt
retention/expiry readback acknowledgement replay scenario, idempotency guard,
monotonicity check, and local operator/auditor/release/runtime replay view
carries the seven canonical fields. Missing durable identity evidence is a
replay denial, and the invariant set requires durable identity evidence before
terminal receipt retention/expiry readback acknowledgement terminal decision
receipt retention/expiry readback acknowledgement replay/idempotency evidence
can advance toward terminal decision preview. Replay execution, idempotency
index mutation, replay recording, readback acknowledgement recording, readback
receipt persistence, retention state persistence, expiry recording, garbage
collection, tombstone persistence, receipt acknowledgement recording, terminal
decision receipt recording/persistence, terminal decision recording/persistence,
status promotion, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement replay/idempotency gate. Every
terminal decision surface, authority guard, release/delivery guard, and local
operator/auditor/release/runtime terminal decision view carries the seven
canonical fields. Missing durable identity evidence is a non-promotion denial,
and the invariant set requires durable identity evidence before terminal receipt
retention/expiry readback acknowledgement terminal decision receipt
retention/expiry readback acknowledgement terminal decision evidence can advance
toward terminal decision receipt preview. Terminal decision recording, status
promotion, replay execution, idempotency index mutation, replay recording,
readback acknowledgement recording, readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
receipt acknowledgement recording, terminal decision receipt
recording/persistence, effect application, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
traffic routing, release publication, public claim recording, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion gate. Every terminal decision receipt, digest check, receipt
guard, and local operator/auditor/release/runtime receipt view carries the
seven canonical fields. Missing durable identity evidence is a mismatch denial,
and the invariant set requires durable identity evidence before terminal
receipt retention/expiry readback acknowledgement terminal decision receipt
evidence can advance toward receipt acknowledgement preview. Terminal decision
receipt recording/persistence, terminal decision recording/persistence, status
promotion, replay execution, idempotency index mutation, replay recording,
readback acknowledgement recording, readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
receipt acknowledgement recording, effect application, acceptance recording,
approval recording, authority grant, live persistence, WAL/checkpoint writes,
rollout, traffic routing, release publication, public claim recording, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision receipt
gate. Every terminal decision receipt acknowledgement contract, expiry/replay
guard, and local operator/auditor/release/runtime acknowledgement view carries
the seven canonical fields. Missing durable identity evidence is a
non-acceptance reason and recording denial, and the invariant set requires
durable identity evidence before terminal receipt retention/expiry readback
acknowledgement terminal decision receipt acknowledgement evidence can advance
toward acknowledgement replay/idempotency preview. Receipt acknowledgement
recording, terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, replay recording, readback acknowledgement recording, readback
receipt persistence, retention state persistence, expiry recording, garbage
collection, tombstone persistence, effect application, acceptance recording,
approval recording, authority grant, live persistence, WAL/checkpoint writes,
rollout, traffic routing, release publication, public claim recording, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement gate. Every replay scenario, idempotency guard, monotonicity
check, and local operator/auditor/release/runtime replay view carries the seven
canonical fields. Missing durable identity evidence is a replay denial, and the
invariant set requires durable identity evidence before terminal receipt
retention/expiry readback acknowledgement terminal decision receipt
acknowledgement replay/idempotency evidence can advance toward retention/expiry
preview. Replay execution, idempotency index mutation, replay recording, receipt
acknowledgement recording, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, readback
acknowledgement recording, readback receipt persistence, retention state
persistence, expiry recording, garbage collection, tombstone persistence, effect
application, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement replay/idempotency gate. Every terminal decision receipt
retention policy, supersession guard, and local
operator/auditor/release/runtime retention/expiry view carries the seven
canonical fields. Missing durable identity evidence is a garbage-collection
denial, and the invariant set requires durable identity evidence before
terminal receipt retention/expiry readback acknowledgement terminal decision
receipt retention/expiry evidence can advance toward readback receipt preview.
Retention state persistence, expiry recording, garbage collection, tombstone
persistence, replay execution, idempotency index mutation, replay recording,
receipt acknowledgement recording, terminal decision receipt
recording/persistence, terminal decision recording/persistence, status
promotion, readback acknowledgement recording, readback receipt persistence,
effect application, acceptance recording, approval recording, authority grant,
live persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry gate. Every terminal decision receipt retention/expiry
readback receipt, digest check, receipt guard, and local
operator/auditor/release/runtime readback receipt view carries the seven
canonical fields. Missing durable identity evidence is a readback receipt
mismatch denial, and the invariant set requires durable identity evidence
before terminal receipt retention/expiry readback acknowledgement terminal
decision receipt retention/expiry readback receipt evidence can advance toward
readback acknowledgement preview. Readback receipt persistence, retention state
persistence, expiry recording, garbage collection, tombstone persistence,
replay execution, idempotency index mutation, replay recording, receipt
acknowledgement recording, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, readback
acknowledgement recording, effect application, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
traffic routing, release publication, public claim recording, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback receipt gate. Every terminal decision receipt
retention/expiry readback acknowledgement contract, expiry/replay guard, and
local operator/auditor/release/runtime acknowledgement view carries the seven
canonical fields. Missing durable identity evidence is a non-acceptance reason
and recording denial, and the invariant set requires durable identity evidence
before terminal receipt retention/expiry readback acknowledgement terminal
decision receipt retention/expiry readback acknowledgement evidence can advance
toward acknowledgement replay/idempotency preview. Readback acknowledgement
recording, readback receipt persistence, retention state persistence, expiry
recording, garbage collection, tombstone persistence, replay execution,
idempotency index mutation, replay recording, receipt acknowledgement recording,
terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, effect application, acceptance
recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement gate. Every replay scenario,
idempotency guard, monotonicity check, and local
operator/auditor/release/runtime replay view carries the seven canonical
fields. Missing durable identity evidence is a replay denial, and the invariant
set requires durable identity evidence before terminal receipt
retention/expiry readback acknowledgement terminal decision receipt
retention/expiry readback acknowledgement replay/idempotency evidence can
advance toward terminal decision non-promotion preview. Replay execution,
idempotency index mutation, replay recording, readback acknowledgement
recording, readback receipt persistence, retention state persistence, expiry
recording, garbage collection, tombstone persistence, receipt acknowledgement
recording, terminal decision receipt recording/persistence, terminal decision
recording/persistence, status promotion, effect application, acceptance
recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

The report and gate source probes for the same acknowledgement
replay/idempotency preview were also corrected to read from the current
deep-chain module instead of the stale shallower projection. The exported report
now includes durable identity evidence, eight replay denials, seven invariants,
the durable root as the final prior gate, and durable fields on every replay
scenario, idempotency guard, monotonicity check, and local replay view while
keeping replay execution, idempotency mutation, recording, persistence,
promotion, release, and live execution disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement replay/idempotency gate. Every
terminal decision surface, authority guard, release/delivery guard, and local
operator/auditor/release/runtime terminal decision view carries the seven
canonical fields. Missing durable identity evidence is a non-promotion denial,
and the invariant set requires durable identity evidence before terminal
decision recording, status promotion, receipt preview, or release/delivery can
be considered. Terminal decision recording/persistence, status promotion,
receipt recording/persistence, replay execution, idempotency index mutation,
replay recording, readback acknowledgement recording, readback receipt
persistence, retention state persistence, expiry recording, garbage collection,
tombstone persistence, effect application, acceptance recording, approval
recording, authority grant, live persistence, WAL/checkpoint writes, rollout,
traffic routing, release publication, public claim recording, and external
delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion gate. Every terminal decision receipt, digest check, receipt
guard, and local operator/release-owner/auditor/runtime receipt view carries
the seven canonical fields. Missing durable identity evidence is a receipt
mismatch denial, and the invariant set requires durable identity evidence
before receipt recording, acceptance, authority, release publication, public
claim, or external delivery can be considered. Receipt recording/persistence,
terminal decision recording/persistence, status promotion, replay execution,
idempotency index mutation, replay recording, readback acknowledgement
recording, readback receipt persistence, retention state persistence, expiry
recording, garbage collection, tombstone persistence, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion receipt gate. Every receipt acknowledgement contract,
expiry/replay guard, and local operator/release-owner/auditor/runtime
acknowledgement view carries the seven canonical fields. Missing durable
identity evidence is both a non-acceptance reason and a recording denial, and
the invariant set requires durable identity evidence before acknowledgement
recording, acceptance, authority, release publication, public claim, or external
delivery can be considered. Receipt acknowledgement recording, receipt
recording/persistence, terminal decision recording/persistence, status
promotion, replay execution, idempotency index mutation, replay recording,
readback acknowledgement recording, readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
terminal decision receipt recording/persistence, effect application, acceptance
recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion receipt acknowledgement gate. Every replay scenario, idempotency
guard, monotonicity check, and local operator/release-owner/auditor/runtime
replay view carries the seven canonical fields. Missing durable identity
evidence is a replay denial, and the invariant set requires durable identity
evidence before replay execution, idempotency mutation, replay recording,
receipt acknowledgement recording, receipt recording/persistence, terminal
decision recording/status promotion, retention/readback persistence, promotion,
release, or live execution can be considered. Replay execution, idempotency
index mutation, replay recording, receipt acknowledgement recording, receipt
recording/persistence, terminal decision recording/persistence, status
promotion, readback acknowledgement recording, readback receipt persistence,
retention state persistence, expiry recording, garbage collection, tombstone
persistence, terminal decision receipt recording/persistence, effect
application, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement replay/idempotency gate. Every retention policy, supersession
guard, and local operator/auditor/release/runtime retention view carries the
seven canonical fields. Missing durable identity evidence is a garbage
collection denial, and the invariant set requires durable identity evidence
before retention state persistence, expiry recording, garbage collection,
tombstone persistence, readback receipt preview, or live retention can be
considered. Retention state persistence, expiry recording, garbage collection,
tombstone persistence, replay execution, idempotency index mutation, replay
recording, receipt acknowledgement recording, receipt recording/persistence,
terminal decision recording/persistence, status promotion, readback
acknowledgement recording, readback receipt persistence, terminal decision
receipt recording/persistence, effect application, acceptance recording,
approval recording, authority grant, live persistence, WAL/checkpoint writes,
rollout, traffic routing, release publication, public claim recording, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry gate. Every readback receipt, digest check, receipt guard, and
local operator/auditor/release/runtime readback receipt view carries the seven
canonical fields. Missing durable identity evidence is a receipt mismatch
denial, and the invariant set requires durable identity evidence before
readback receipt persistence, acknowledgement recording, retention state
persistence, expiry recording, garbage collection, tombstone persistence, or
live retention can be considered. Readback receipt persistence,
acknowledgement recording, retention state persistence, expiry recording,
garbage collection, tombstone persistence, replay execution, idempotency index
mutation, replay recording, receipt acknowledgement recording, receipt
recording/persistence, terminal decision recording/persistence, status
promotion, terminal decision receipt recording/persistence, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback receipt gate. Every acknowledgement contract,
expiry/replay guard, and local operator/auditor/release/runtime acknowledgement
view carries the seven canonical fields. Missing durable identity evidence is a
non-acceptance and recording denial, and the invariant set requires durable
identity evidence before readback acknowledgement recording, readback receipt
persistence, retention state persistence, expiry recording, garbage collection,
tombstone persistence, replay execution, idempotency index mutation, or live
retention can be considered. Readback acknowledgement recording, readback
receipt persistence, retention state persistence, expiry recording, garbage
collection, tombstone persistence, replay execution, idempotency index mutation,
replay recording, receipt acknowledgement recording, receipt
recording/persistence, terminal decision recording/persistence, status
promotion, terminal decision receipt recording/persistence, persistence state
writes, effect application, acceptance recording, approval recording, authority
grant, live persistence, WAL/checkpoint writes, rollout, traffic routing,
release publication, public claim recording, and external delivery stay
disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement gate. Every replay scenario,
idempotency guard, monotonicity check, and local operator/auditor/release/runtime
replay view carries the seven canonical fields. Missing durable identity
evidence is a replay denial, and the invariant set requires durable identity
evidence before replay execution, idempotency mutation, replay recording,
readback acknowledgement recording, readback receipt persistence, retention
state persistence, expiry recording, garbage collection, tombstone persistence,
or live retention can be considered. Replay execution, idempotency index
mutation, replay recording, readback acknowledgement recording, readback receipt
persistence, retention state persistence, expiry recording, garbage collection,
tombstone persistence, receipt acknowledgement recording, receipt
recording/persistence, terminal decision recording/persistence, status
promotion, terminal decision receipt recording/persistence, persistence state
writes, effect application, acceptance recording, approval recording, authority
grant, live persistence, WAL/checkpoint writes, rollout, traffic routing,
release publication, public claim recording, and external delivery stay
disabled.

The replay/idempotency report/gate now probes the current deep replay Rust
module, the immediate readback acknowledgement report/gate, and the durable
identity root, keeping the replay layer bound to the current acknowledgement
projection instead of a stale shorter-chain report.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate`
also requires the durable identity preview after the terminal decision receipt
retention/expiry readback acknowledgement replay/idempotency gate. Every
terminal decision surface, authority guard, release/delivery guard, and local
operator/auditor/release/runtime decision view carries the seven canonical
fields. Missing durable identity evidence is a non-promotion denial, and the
invariant set requires durable identity evidence before terminal decision
recording, status promotion, receipt recording, replay execution, idempotency
mutation, persistence promotion, release publication, public claims, or external
delivery can be considered. Terminal decision recording/persistence, status
promotion, receipt recording/persistence, replay execution, idempotency index
mutation, replay recording, readback acknowledgement recording, readback receipt
persistence, retention state persistence, expiry recording, garbage collection,
tombstone persistence, receipt acknowledgement recording, terminal decision
receipt recording/persistence, persistence state writes, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion gate. Every receipt, digest check, receipt guard, and local
operator/auditor/release/runtime receipt view carries the seven canonical
fields. Missing durable identity evidence is a receipt mismatch denial, and the
invariant set requires durable identity evidence before terminal decision
receipt recording, receipt persistence, acceptance, authority grant, rollout,
release publication, public claims, or external delivery can be considered.
Terminal decision receipt recording/persistence, receipt recording/persistence,
terminal decision recording/persistence, status promotion, replay execution,
idempotency index mutation, replay recording, readback acknowledgement
recording, readback receipt persistence, retention state persistence, expiry
recording, garbage collection, tombstone persistence, receipt acknowledgement
recording, persistence state writes, effect application, acceptance recording,
approval recording, authority grant, live persistence, WAL/checkpoint writes,
rollout, traffic routing, release publication, public claim recording, and
external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate`
also requires the durable identity preview after the terminal decision
non-promotion receipt gate. Every receipt acknowledgement contract,
expiry/replay guard, and local operator/auditor/release/runtime acknowledgement
view carries the seven canonical fields. Missing durable identity evidence is a
non-acceptance and recording denial, and the invariant set requires durable
identity evidence before receipt acknowledgement recording, terminal decision
receipt recording/persistence, receipt persistence, acceptance, authority grant,
rollout, release publication, public claims, or external delivery can be
considered. Receipt acknowledgement recording, terminal decision receipt
recording/persistence, receipt recording/persistence, terminal decision
recording/persistence, status promotion, replay execution, idempotency index
mutation, replay recording, readback acknowledgement recording, readback
receipt persistence, retention state persistence, expiry recording, garbage
collection, tombstone persistence, persistence state writes, effect application,
acceptance recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Acknowledgement Replay Idempotency Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement gate. Every replay scenario, idempotency guard, monotonicity
check, and local operator/auditor/release/runtime replay view carries the seven
canonical fields. Missing durable identity evidence is a replay denial, and the
invariant set requires durable identity evidence before replay execution,
idempotency mutation, replay recording, receipt acknowledgement recording,
terminal decision receipt recording/persistence, receipt persistence, acceptance,
authority grant, rollout, release publication, public claims, or external
delivery can be considered. Replay execution, idempotency index mutation,
replay recording, receipt acknowledgement recording, terminal decision receipt
recording/persistence, receipt recording/persistence, terminal decision
recording/persistence, status promotion, readback acknowledgement recording,
readback receipt persistence, retention state persistence, expiry recording,
garbage collection, tombstone persistence, persistence state writes, effect
application, acceptance recording, approval recording, authority grant, live
persistence, WAL/checkpoint writes, rollout, traffic routing, release
publication, public claim recording, and external delivery stay disabled.

## Terminal Decision Receipt Acknowledgement Source Probe Correction

The terminal decision receipt acknowledgement report/gate now probes the current
deep Rust module
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_ack_preview`
instead of the older shallower acknowledgement projection. This keeps the
acknowledgement replay/idempotency gate bound to the current 6 contract, 8
non-acceptance, 8 recording-denial, 5 expiry/replay guard, 4 local-view, and 7
invariant durable identity shape. The correction only tightens report/gate
provenance and leaves acknowledgement recording, receipt persistence, terminal
decision recording, status promotion, replay execution, idempotency mutation,
release, live execution, provider invocation, and external delivery disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate`
also requires the durable identity preview after the terminal decision receipt
acknowledgement replay/idempotency gate. Every retention policy, supersession
guard, and local operator/auditor/release/runtime retention/expiry view carries
the seven canonical fields. Missing durable identity evidence is the first
garbage-collection denial, and the invariant set requires durable identity
evidence before retention state persistence, expiry recording, garbage
collection, tombstone persistence, replay execution, idempotency mutation,
recording/persistence, acceptance, authority grant, rollout, release
publication, public claims, or external delivery can be considered. The
report/gate now probes the current deep retention/expiry Rust module plus the
upstream acknowledgement replay/idempotency gate and durable identity root, so
stale shorter-chain projections cannot satisfy this gate. Retention state
persistence, expiry recording, garbage collection execution, tombstone
persistence, replay execution, idempotency index mutation, replay recording,
receipt acknowledgement recording, receipt recording/persistence, terminal
decision recording/persistence, status promotion, readback acknowledgement
recording, readback receipt persistence, terminal decision receipt
recording/persistence, persistence state writes, effect application, acceptance
recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, and external delivery stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Receipt Source Probe Correction

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate`
now probes the current deep readback receipt Rust module
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_receipt_retention_expiry_readback_rcpt_preview`
instead of the older shallower readback receipt projection. The gate requires
the durable identity preview after the terminal decision receipt
retention/expiry gate, carries the seven canonical fields on all readback
receipts, digest checks, receipt guards, and local readback views, and fixes the
report projection to 6 readback receipts, 7 digest checks, 8 mismatch denials,
5 receipt guards, 4 local views, and 7 invariants. Missing durable identity
evidence is the first mismatch denial, and the prior tail is the
retention/expiry preview gate followed by `hepta_work_graph_durable_identity_preview_gate`.
This correction prevents stale shorter-chain readback receipt projections from
satisfying the current gate. Readback receipt persistence, retention state
persistence, expiry recording, garbage collection execution, tombstone
persistence, replay execution, idempotency index mutation, recording/persistence,
status promotion, effect application, acceptance recording, approval recording,
authority grant, live persistence, WAL/checkpoint writes, rollout, traffic
routing, release publication, public claim recording, and external delivery
stay disabled.

## Persistence Acceptance Effect Application Denial Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Terminal Decision Receipt Retention Expiry Readback Acknowledgement Binding

`hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate`
now requires the durable identity preview after the terminal decision receipt
retention/expiry readback receipt gate. Every acknowledgement contract,
expiry/replay guard, and local operator/auditor/release/runtime acknowledgement
view carries the seven canonical fields. Missing durable identity evidence is
the first non-acceptance reason and first recording denial, and the invariant
set requires durable identity evidence before readback acknowledgement
recording, readback receipt persistence, retention/expiry persistence,
garbage collection, tombstone persistence, replay/idempotency mutation,
recording/persistence, promotion, release publication, public claims, external
delivery, provider invocation, or live execution can be considered.

The acknowledgement report/gate now probes the current deep acknowledgement Rust
module, the immediate readback receipt report/gate, and the durable identity
root. While hardening this gate, the abbreviated readback receipt report/gate
that it consumes was also corrected from the older 6 digest / 7 mismatch / 6
invariant projection to the current durable 6 readback receipt, 7 digest check,
8 mismatch denial, 5 receipt guard, 4 local view, and 7 invariant shape. This
keeps the acknowledgement layer from accepting a stale shorter-chain readback
receipt projection. Readback acknowledgement recording, readback receipt
persistence, retention/expiry persistence, garbage collection execution,
tombstone persistence, replay execution, idempotency index mutation,
recording/persistence, status promotion, effect application, acceptance
recording, approval recording, authority grant, live persistence,
WAL/checkpoint writes, rollout, traffic routing, release publication, public
claim recording, external delivery, provider/model invocation, gateway/Native
or Telegram routing mutation, git add/commit/push/deploy, and live execution
stay disabled.

The current terminal decision receipt preview report/gate probes the
seven-segment
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_preview`
Rust module, its immediate terminal decision non-promotion report/gate, and the
durable identity root. Its prior tail is explicit terminal decision
non-promotion -> durable identity root, preventing the receipt preview from
following stale six-segment projections or expanding a cyclic downstream report
chain. Receipt recording/persistence, terminal decision receipt persistence,
terminal decision recording/status promotion, replay/idempotency mutation,
retention/readback persistence, effect application, release/live, provider/model
invocation, gateway/Native or Telegram routing mutation, and external delivery
stay disabled.

The current terminal decision receipt acknowledgement preview report/gate probes
the seven-segment
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_ack_preview`
Rust module, its immediate terminal decision receipt report/gate, and the
durable identity root. Its prior tail is explicit terminal decision receipt ->
durable identity root, preventing the acknowledgement preview from following a
stale six-segment projection or recursively expanding older downstream report
chains. Receipt acknowledgement recording, receipt recording/persistence,
terminal decision receipt persistence, terminal decision recording/status
promotion, replay/idempotency mutation, retention/readback persistence, effect
application, release/live, provider/model invocation, gateway/Native or Telegram
routing mutation, and external delivery stay disabled.

The current terminal decision receipt acknowledgement replay/idempotency preview
report/gate probes the seven-segment
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_ack_replay_preview`
Rust module, its immediate terminal decision receipt acknowledgement report/gate,
and the durable identity root. Its prior tail is explicit receipt
acknowledgement -> durable identity root, preventing replay/idempotency preview
from following stale six-segment projections or recursively expanding older
downstream report chains. Replay scenarios, idempotency guards, monotonicity
checks, and local replay views carry the seven durable fields. Missing durable
identity evidence is a replay denial, and durable identity evidence is required
before replay execution, idempotency mutation, replay recording, acknowledgement
recording, receipt persistence, terminal decision recording/status promotion,
retention/readback persistence, effect application, release/live,
provider/model invocation, gateway/Native or Telegram routing mutation, or
external delivery can be considered.

The current terminal decision receipt retention/expiry preview report/gate
probes the seven-segment
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry_preview`
Rust module, its immediate terminal decision receipt acknowledgement
replay/idempotency report/gate, and the durable identity root. Its prior tail
is explicit receipt acknowledgement replay/idempotency -> durable identity
root, preventing retention/expiry preview from following stale shorter
projections or recursively expanding older downstream report chains. Retention
policies, supersession guards, and local retention/expiry views carry the seven
durable fields. Missing durable identity evidence is the first garbage
collection denial, and durable identity evidence is required before retention
state persistence, expiry recording, garbage collection, tombstone persistence,
replay/idempotency mutation, recording/persistence, promotion, release/live,
provider/model invocation, gateway/Native or Telegram routing mutation, or
external delivery can be considered.

The current terminal decision receipt retention/expiry readback receipt preview
report/gate probes the seven-segment
`wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_preview`
Rust module, its immediate terminal decision receipt retention/expiry
report/gate, and the durable identity root. Its prior tail is explicit
retention/expiry -> durable identity root, preventing readback receipt preview
from following stale shorter projections or recursively expanding older
downstream report chains. Readback receipts, digest checks, receipt guards, and
local readback views carry the seven durable fields. Missing durable identity
evidence is the first mismatch denial, `check_durable_identity_digest` blocks
receipt acceptance, and durable identity evidence is required before readback
acknowledgement recording, readback receipt persistence, retention/expiry
persistence, expiry recording, garbage collection, tombstone persistence,
replay/idempotency mutation, recording/persistence, promotion, release/live,
provider/model invocation, gateway/Native or Telegram routing mutation, or
external delivery can be considered.

## Boundary

The durable identity preview is read-only. It does not persist graph state,
append WAL records, write checkpoints, execute replay, execute rollback, record
approval, promote state, invoke models, or send externally.

The next acceptable step is to thread the durable identity evidence through the
persistence acceptance effect application denial receipt retention expiry
readback acknowledgement terminal decision non-promotion receipt retention/expiry
readback acknowledgement terminal decision non-promotion receipt retention/expiry
readback acknowledgement terminal decision non-promotion receipt retention/expiry
readback acknowledgement terminal decision non-promotion receipt retention/expiry
readback acknowledgement preview while keeping readback acknowledgement
recording, readback receipt persistence, retention/expiry persistence, expiry
recording, garbage collection, tombstone persistence, replay execution,
idempotency index mutation, replay recording, receipt acknowledgement recording,
receipt recording/persistence, terminal decision receipt recording/persistence,
terminal decision recording/persistence, status promotion, effect application,
authority grant, scheduler cutover, release, and live execution disabled.
