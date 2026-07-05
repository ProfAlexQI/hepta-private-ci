# Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Retention/Expiry/GC Final Index

This final index consumes the artifact signing retention/expiry/GC readback
and keeps the Public GA operator identity/session intent/consent evidence chain
in a ready-but-blocked state.

The final index exposes a stable local handoff for the next report-only
export/query/observability denial slice. It confirms that no retention policy,
TTL lease, expiry timestamp, expiry scheduler, expiry timer, expiry
acknowledgement, garbage-collection queue, GC scan, GC decision, archive,
compaction, evidence retention, operator approval, release-publication
authority, activation authority, install path, restart path, active-binary
mutation, provider/model invocation, credential/secret read, public release, or
Public GA claim has been accepted, recorded, persisted, materialized,
executed, delivered, or derived.

It does not invoke the artifact signing retention/expiry/GC gate, the artifact
signing audit/evidence gate, any Public GA operator approval packet, any
terminal live gate, any long soak, any live URL, or any external send. The next
migration remains local and report-only:
`attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_retention_expiry_gc_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_without_retention`.
