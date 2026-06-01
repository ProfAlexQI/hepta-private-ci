# Hepta Memory, Intelligence, and KG Full Enablement KG External Adapter Staging Receipt Gate

This gate binds the full-enablement memory staging fixture to the KG prompt-preview rollback and kill-switch checklist, then declares the external adapter staging receipt shape for Graphiti, Neo4j, and CocoIndex.

It does not read credentials, capture credential values, construct external adapter clients, attempt network calls, write KG state, execute rollback, persist receipts, restart services, or publish release claims.

## Contract

- Script: `scripts/hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate.sh`
- Gate: `hepta_memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_gate`
- Schema: `memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_v1`
- Mode: `kg_external_adapter_credential_and_rollback_receipt_shape_no_credential_read_no_adapter_invocation_no_kg_write`
- Status: `ready`

## Source Evidence

The gate composes two report-only surfaces:

- `hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate.sh` proves the full-enablement lane family is ready for operator-approved slicing while memory writes, KG reads, credential reads, and live execution stay disabled.
- `hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate.sh` proves rollback and kill-switch evidence are still required and missing before prompt preview, context injection, external adapter reads, or KG writes can happen.

The new gate only stages adapter receipt fields that can later be bound to an explicit operator packet. It uses key names and schema slots, not secret values.

## Adapter Receipt Shape

The receipt inventory covers:

- `graphiti`
- `neo4j`
- `cocoindex`

For each adapter, the gate declares the feature-gate key, endpoint key, credential-reference key, rollback-plan key, post-write validation key, and projection family. All live fields remain closed:

- credential references recorded: `0`
- credential values captured: `0`
- credential reads: `0`
- endpoint values captured: `0`
- accepted rollback receipts: `0`
- accepted kill-switch receipts: `0`
- staging-ready adapters: `0`
- network calls attempted: `0`
- adapter clients constructed: `0`
- KG writes performed: `0`
- persisted records: `0`

## Non-Activation Guarantees

The report keeps these actions false:

- memory-store write or mutation
- Hepta Intelligence live context attachment
- prompt preview, prompt payload materialization, or context injection
- provider/model invocation
- credential or secret read
- Graphiti, Neo4j, or CocoIndex client construction
- external KG adapter read or network call
- external database write or live KG write
- rollback execution
- external/channel send
- service restart or active binary mutation
- public release or GA claim

## Next Slice

The next safe slice is a bounded prompt-preview and context-handoff activation-packet gate. It should remain report-only: no prompt rendering, no context injection, no model invocation, no credential read, and no KG write.
