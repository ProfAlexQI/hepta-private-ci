# Hepta Intelligence P0.3 Source Tranche

**Date:** 2026-08-28  
**Status:** `SOURCE_ONLY / STACKED_DRAFT / ACTIVATION_BLOCKED`  
**Base branch:** `codex/hepta-intelligence-grounding-ledger-v2-20260828`  
**Base commit:** `ad0cf81cd820bbb4b8154633c61ea68fb35bf872`  
**Development branch:** `codex/hepta-intelligence-grounding-gate-v3-20260828`

## 1. Authority boundary

P0.1 and P0.2 remain `qualified=false` because the exact hosted jobs never received a runner. P0.3 therefore implements only compile-time/source qualification seams.

```text
tool_v3_registered=false
default_projection_pointer_changed=false
default_recall_query_changed=false
production_projection_gate=false
production_authority=false
external_effects=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

The current `remember`, `correct`, recall, explain, KG projection generation, and physical-send paths are unchanged.

## 2. Grounded tool v3 contract

`codex-rs/ext/hepta-memory/src/cognitive/grounding_v3.rs` defines the future input contract for grounded writes.

Every entity and relation must include 1–4 exact source spans:

```json
{
  "entities": [{
    "key": "aurora",
    "entity_type": "project",
    "label": "Project Aurora",
    "evidence": [{
      "start_byte": 0,
      "end_byte": 14,
      "sha256": "<64 lowercase hex>"
    }]
  }],
  "relations": [{
    "key": "aurora-uses-rust",
    "from_entity_key": "aurora",
    "to_entity_key": "rust",
    "relation": "uses",
    "evidence": [{
      "start_byte": 0,
      "end_byte": 24,
      "sha256": "<64 lowercase hex>"
    }]
  }]
}
```

The validator checks:

- entity/relation and total evidence limits;
- non-empty bounded strings;
- source-byte bounds;
- UTF-8 character boundaries;
- lowercase SHA-256 shape and exact source-byte digest;
- mandatory evidence for every fact.

The module is compiled and tested but intentionally not registered with `ToolContributor`.

## 3. Grounded projection shadow compare

`CognitiveStore::shadow_grounded_projection_compare`:

1. ensures and fully verifies the P0.2 durable grounding ledger;
2. reads the current scope heads and current projection receipt;
3. builds a candidate containing only `verified + active + grounded_v1` non-empty facts;
4. classifies exclusions as `legacy_unreviewed`, `zero_fact`, or `ineligible_head`;
5. emits candidate/current counts, deltas, included receipt digests, and a deterministic candidate digest;
6. performs no `INSERT`, `UPDATE`, `DELETE`, projection refresh, or pointer mutation.

The default projection pointer is unchanged.

## 4. Shadow grounding explain

`CognitiveStore::shadow_grounding_explain` emits:

- memory/revision and scope;
- memory and fact-set digests;
- grounding status;
- fact identity and durable receipt digests;
- bounded evidence metadata: fact kind/key, ordinal, byte range, evidence SHA.

It does not return source content and grants no instruction, execution, or production authority.

## 5. Qualification

Source gate:

```bash
python3 scripts/verify-hepta-intelligence-grounding-gate.py
```

Required Rust qualification:

```bash
cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory shadow_ -- --nocapture
cargo test -p codex-hepta-memory-extension grounding_v3 -- --nocapture
cargo test -p codex-hepta-memory
cargo test -p codex-hepta-memory-extension
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
cargo clippy -p codex-hepta-memory-extension --all-targets -- -D warnings
```

## 6. Exit gate

This tranche may move from `implemented` to `qualified` only after one exact head has:

- an executable source-gate PASS artifact;
- Rust formatting, focused tests, full crate tests, and clippy PASS;
- P0.1 and P0.2 exact-head executable qualification;
- shadow receipt review proving no projection or recall mutation;
- candidate freeze and operator review.

Even after source qualification, activation remains a separate decision. Production projection gating requires an explicit later CALLERS ratchet and rollback rehearsal.
