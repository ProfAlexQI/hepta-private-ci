# Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Retention/Expiry/GC Attachment

This attachment consumes the Public GA operator identity/session operator
intent/consent evidence artifact signing audit/evidence final index and
connects it to the artifact signing retention/expiry/garbage-collection denial
surface.

The attachment is ready-but-blocked. It only source-probes the
retention/expiry/GC denial gate and architecture note. It keeps retention
policy, TTL lease, expiry timestamp, expiry scheduler, expiry timer, expiry
acknowledgement, garbage-collection queue, GC scan, GC candidate, GC decision,
tombstone GC, delete-marker GC, archive, compaction, audit evidence retention,
immutable evidence retention, hash/attestation retention, witness/notary
expiry, ledger/index retention, delivery/status evidence retention, operator
approval derivation, release-publication authority, activation authority,
install/restart paths, active-binary mutation, provider/model invocation,
credential/secret reads, Public GA, public release, terminal live gates, and
rollback execution blocked.

It does not invoke the artifact signing retention/expiry/GC gate, the artifact
signing audit/evidence gate, any Public GA operator approval packet, any
terminal live gate, any long soak, any live URL, or any external send. The next
allowed local step is static readback from this attachment, without audit
evidence recording or retention lifecycle acceptance.
