#!/usr/bin/env python3
"""Repair deterministic V4 payload marker drift before materialization.

The package payloads remain allow-listed source generators. This normalizer only
corrects exact Python string markers whose indentation or multiplicity drifted after
Rust formatting; it neither executes payloads nor relaxes their authority boundary.
"""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PAYLOAD = ROOT / "tools" / "hepta-inference-v4-payloads"


def replace_assignment(text: str, name: str, replacement: str) -> str:
    start = text.index(f"          {name} =")
    end = text.index(f"          if source.count({name}) != 1:", start)
    return text[:start] + replacement + text[end:]


def normalize_s3() -> None:
    path = PAYLOAD / "inf-s3.yml"
    text = path.read_text(encoding="utf-8")
    text = replace_assignment(
        text,
        "open_marker",
        """          open_marker = (
              "    let receipt_store = Arc::new(\\n"
              "        ReceiptStore::open(\\n"
              "            config.receipt_dir.clone(),\\n"
              "            config.max_receipt_files,\\n"
              "            config.max_receipt_bytes,\\n"
              "        )\\n"
              "        .await?,\\n"
              "    );\\n"
          )
          open_insert = (
              "    let active_request_ids = HashSet::new();\\n"
              "    let retention = receipt_retention::compact_and_recover(\\n"
              "        &config.receipt_dir,\\n"
              "        ReceiptRetentionPolicy {\\n"
              "            minimum_retention: config.receipt_minimum_retention,\\n"
              "            compact_on_start: config.compact_receipts_on_start,\\n"
              "        },\\n"
              "        unix_time_ms()?,\\n"
              "        &active_request_ids,\\n"
              "        config.max_receipt_files,\\n"
              "        config.max_receipt_bytes,\\n"
              "    )\\n"
              "    .await?;\\n"
              "    let receipt_store = Arc::new(\\n"
              "        ReceiptStore::open(\\n"
              "            config.receipt_dir.clone(),\\n"
              "            config.max_receipt_files,\\n"
              "            config.max_receipt_bytes,\\n"
              "            retention,\\n"
              "        )\\n"
              "        .await?,\\n"
              "    );\\n"
          )
""",
    )
    text = replace_assignment(
        text,
        "name_marker",
        """          name_marker = (
              '            if !name.starts_with("receipt-") || !name.ends_with(".cbor") {\\n'
              '                return Err(invalid_data("INF_RECEIPT_STORE_UNKNOWN_FILE"));\\n'
              '            }\\n'
          )
          name_insert = (
              '            if name.starts_with("tombstone-") && name.ends_with(".txt") {\\n'
              '                continue;\\n'
              '            }\\n'
              '            if !name.starts_with("receipt-") || !name.ends_with(".cbor") {\\n'
              '                return Err(invalid_data("INF_RECEIPT_STORE_UNKNOWN_FILE"));\\n'
              '            }\\n'
          )
""",
    )
    text = replace_assignment(
        text,
        "receipt_marker",
        """          receipt_marker = (
              "            let key = ReceiptKey::from(&receipt);\\n"
              "            if state.receipts.insert(key, receipt).is_some() {\\n"
          )
          receipt_insert = (
              "            if retention.compacted_request_ids.contains(&receipt.request_id) {\\n"
              "                return Err(invalid_data(\\\"INF_RECEIPT_TOMBSTONE_RECOVERY_INCOMPLETE\\\"));\\n"
              "            }\\n"
              "            let key = ReceiptKey::from(&receipt);\\n"
              "            if state.receipts.insert(key, receipt).is_some() {\\n"
          )
""",
    )
    path.write_text(text, encoding="utf-8")


def normalize_r2() -> None:
    path = PAYLOAD / "inf-r2-chain.yml"
    text = path.read_text(encoding="utf-8")
    for old, new, label in (
        (
            '"            sequence,\\n            grant_digest,\\n            prompt_digest,\\n"',
            '"                sequence,\\n                grant_digest,\\n                prompt_digest,\\n"',
            "protocol destructure marker",
        ),
        (
            '"            sequence,\\n            initial_chain_digest,\\n            grant_digest,\\n            prompt_digest,\\n"',
            '"                sequence,\\n                initial_chain_digest,\\n                grant_digest,\\n                prompt_digest,\\n"',
            "protocol destructure insert",
        ),
    ):
        if text.count(old) != 1:
            raise SystemExit(f"unexpected R2 {label} count: {text.count(old)}")
        text = text.replace(old, new, 1)

    old_header = (
        "          call_marker = \"                3,\\n"
        "                digest('b'),\\n"
        "                digest('c'),\\n"
        "                8,\\n\"\n"
        "          if source.count(call_marker) != 3:\n"
        "              raise SystemExit(\"unexpected worker submit call count\")\n"
    )
    new_header = (
        "          call_marker = \"                3,\\n"
        "                digest('b'),\\n"
        "                digest('c'),\\n"
        "                8,\\n\"\n"
        "          compact_call_marker = \"            3,\\n"
        "            digest('b'),\\n"
        "            digest('c'),\\n"
        "            8,\\n\"\n"
        "          if source.count(call_marker) != 2 or source.count(compact_call_marker) != 1:\n"
        "              raise SystemExit(\"unexpected worker submit call shape\")\n"
    )
    if text.count(old_header) != 1:
        raise SystemExit(
            f"unexpected R2 process call header count: {text.count(old_header)}"
        )
    text = text.replace(old_header, new_header, 1)

    anchor = (
        "          source = source.replace(\n"
        "              call_marker,\n"
        "              \"                3,\\n"
        "                digest('e'),\\n"
        "                digest('b'),\\n"
        "                digest('c'),\\n"
        "                8,\\n\",\n"
        "          )\n"
        "          assertion_marker = \"    assert_eq!(receipt.output_tokens, 1);\\n\"\n"
    )
    replacement = (
        "          source = source.replace(\n"
        "              call_marker,\n"
        "              \"                3,\\n"
        "                digest('e'),\\n"
        "                digest('b'),\\n"
        "                digest('c'),\\n"
        "                8,\\n\",\n"
        "          )\n"
        "          source = source.replace(\n"
        "              compact_call_marker,\n"
        "              \"            3,\\n"
        "            digest('e'),\\n"
        "            digest('b'),\\n"
        "            digest('c'),\\n"
        "            8,\\n\",\n"
        "              1,\n"
        "          )\n"
        "          assertion_marker = \"    assert_eq!(receipt.output_tokens, 1);\\n\"\n"
    )
    if text.count(anchor) != 1:
        raise SystemExit(
            f"unexpected R2 process replacement anchor count: {text.count(anchor)}"
        )
    path.write_text(text.replace(anchor, replacement, 1), encoding="utf-8")


def normalize_post_hardening() -> None:
    path = PAYLOAD / "post_materialize_hardening.py"
    text = path.read_text(encoding="utf-8")
    old = "            '\"hepta-infer-worker-host\"',\n"
    new = "            '    \"hepta-infer-worker-host\",',\n"
    if text.count(old) != 1:
        raise SystemExit(
            f"unexpected worker-host truth marker count: {text.count(old)}"
        )
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    normalize_s3()
    normalize_r2()
    normalize_post_hardening()
    print("PASS_HEPTA_INFERENCE_V4_PAYLOAD_MARKER_NORMALIZATION")


if __name__ == "__main__":
    main()
