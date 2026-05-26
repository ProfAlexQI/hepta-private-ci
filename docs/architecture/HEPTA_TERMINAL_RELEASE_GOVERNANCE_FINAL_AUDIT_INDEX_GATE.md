# Hepta Terminal Release-Governance Final Audit Index Gate

`scripts/hepta-terminal-release-governance-final-audit-index-gate.sh` is a schema-only terminal audit over Hepta's current release-governance boundary.

It consumes three existing reports:

- `scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh`
- `scripts/hepta-active-service-dependency-isolation.sh`
- `scripts/hepta-memory-intelligence-closure.sh`

The gate exists to make the end-state easy to audit: publication evidence can be summarized, the active `hepta-cli` service stays isolated from tracked Codex engine crates, and memory/intelligence surfaces are absorbed or represented. None of that becomes an activation, public release claim, artifact write, evidence persistence record, operator approval, install, restart, provider invocation, channel delivery, or live mutation.

## Contract

The gate is ready only when all of these remain true:

- publication evidence non-persistence is ready and still denies public claims, public distribution, artifact writes, evidence persistence, and runtime mutation
- active service dependency isolation is ready with zero tracked Codex engine crates in the active `hepta-cli` cargo tree
- memory/intelligence closure reports `14/14` absorbed or represented surfaces, zero gap-only surfaces, and zero live mutation-enabled surfaces
- install, launchd restart, release build, signing, notarization, stapling, public distribution, release artifact writes, upstream fetch/merge/checkout, provider invocation, channel delivery, memory/skill/registry mutation, and live mutation remain false
- the final audit index itself is report-only and is not recorded, persisted, materialized, or written to the filesystem

## Current Denial Shape

The report currently emits:

- `final_audit_index_ready=true`
- `final_audit_index_mode=schema_only_release_governance_final_audit_no_activation`
- `final_audit_index_decision=release_governance_audited_without_publication_or_live_mutation`
- `required_source_count=3`
- `ready_source_count=3`
- `activation_blocking_source_count=3`
- `final_audit_denied_by_count=127`
- seven ready and activation-blocking final-audit families

This gate is intentionally not a public-release assertion. It is a terminal safety index proving that Hepta release-governance evidence is coherent while publication, artifact production, live mutation, and runtime activation remain blocked.
