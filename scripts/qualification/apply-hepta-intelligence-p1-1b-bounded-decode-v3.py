#!/usr/bin/env python3
"""Apply the reviewed P1.1b bounded ANN decoder hardening patch."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

CRATE_ROOT = Path("codex-rs/hepta-memory-p1-1b-qualification")
VERIFIER = Path("scripts/verify-hepta-intelligence-local-embedding-index.py")
EXPECTED_CHANGED_PATHS = [
    "codex-rs/hepta-memory-p1-1b-qualification/src/index.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/impl.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/types.rs",
    "scripts/verify-hepta-intelligence-local-embedding-index.py",
]


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True).strip()


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one anchor in {path}, found {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def insert_after_once(path: Path, anchor: str, addition: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(anchor)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one anchor in {path}, found {count}")
    if addition in text:
        raise SystemExit(f"{label}: addition already present in {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1), encoding="utf-8")


def replace_section(
    path: Path,
    start_marker: str,
    end_marker: str,
    replacement: str,
    required_fragment: str,
    label: str,
) -> None:
    text = path.read_text(encoding="utf-8")
    if text.count(start_marker) != 1 or text.count(end_marker) != 1:
        raise SystemExit(f"{label}: section anchors are not unique in {path}")
    start = text.index(start_marker)
    end = text.index(end_marker, start)
    old = text[start:end]
    if required_fragment not in old:
        raise SystemExit(f"{label}: required fragment missing from section in {path}")
    path.write_text(text[:start] + replacement + text[end:], encoding="utf-8")


def patch_index_module() -> None:
    path = CRATE_ROOT / "src/index.rs"
    insert_after_once(
        path,
        "use crate::embedding::LocalEmbeddingDescriptor;\n",
        "use crate::embedding::MAX_EMBEDDING_DIMENSIONS;\n",
        "index-dimension-import",
    )


def patch_index_types() -> None:
    path = CRATE_ROOT / "src/index/types.rs"
    replace_once(
        path,
        "            || self.bucket_count == 0\n",
        "            || self.bucket_count == 0\n"
        "            || self.bucket_count > self.item_count\n",
        "manifest-bucket-bound",
    )
    insert_after_once(
        path,
        """        }
        validate_id(&self.algorithm, "ANN algorithm")?;
""",
        """        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions) {
            return Err(ContractError::Corrupt(
                "ANN manifest dimensions exceed the bounded embedding contract".to_string(),
            ));
        }
""",
        "manifest-dimension-bound",
    )
    replace_once(
        path,
        "        if self.generation == 0 || self.dimensions == 0 {\n",
        """        if self.generation == 0
            || !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions)
        {
""",
        "expected-binding-dimension-condition",
    )
    replace_once(
        path,
        '                "expected ANN generation and dimensions must be non-zero".to_string(),\n',
        '                "expected ANN generation or dimensions are outside bounded limits".to_string(),\n',
        "expected-binding-dimension-message",
    )


def patch_index_decoder() -> None:
    path = CRATE_ROOT / "src/index/impl.rs"
    insert_after_once(
        path,
        "        let dimensions = cursor.read_u32()?;\n",
        """        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&dimensions) {
            return Err(ContractError::Corrupt(
                "ANN file dimensions exceed the bounded embedding contract".to_string(),
            ));
        }
""",
        "decode-dimension-bound",
    )
    replace_once(
        path,
        "            || bucket_count == 0\n",
        "            || bucket_count == 0\n"
        "            || bucket_count > item_count\n",
        "decode-bucket-bound",
    )


def patch_bounded_reader() -> None:
    path = CRATE_ROOT / "src/index/build.rs"
    replace_once(
        path,
        "pub fn reopen_local_ann_index(\n",
        """fn read_bounded_index_bytes<R: Read>(
    reader: &mut R,
    expected_file_bytes: u64,
) -> Result<Vec<u8>, ContractError> {
    if expected_file_bytes == 0 || expected_file_bytes > MAX_INDEX_FILE_BYTES {
        return Err(ContractError::Corrupt(
            "ANN index file size is outside bounded limits".to_string(),
        ));
    }
    let expected_len =
        usize::try_from(expected_file_bytes).map_err(|_| ContractError::Overflow)?;
    let mut bytes = Vec::with_capacity(expected_len);
    reader
        .take(MAX_INDEX_FILE_BYTES.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() != expected_len {
        return Err(ContractError::Corrupt(
            "ANN index file changed while being read or exceeded the bounded limit".to_string(),
        ));
    }
    Ok(bytes)
}

pub fn reopen_local_ann_index(
""",
        "bounded-read-helper",
    )
    replace_section(
        path,
        "    let file_bytes = file.metadata()?.len();\n",
        "    let index = LocalAnnIndex::decode(&bytes)?;\n",
        """    let file_bytes = file.metadata()?.len();
    let bytes = read_bounded_index_bytes(&mut file, file_bytes)?;
""",
        "file.read_to_end(&mut bytes)?;",
        "bounded-read-call",
    )


def append_regression_tests() -> None:
    path = CRATE_ROOT / "src/index/tests_module.rs"
    text = path.read_text(encoding="utf-8")
    if not text.endswith("}\n"):
        raise SystemExit(f"tests-tail: expected final module brace in {path}")
    if "decode_rejects_oversized_dimensions_before_vector_allocation" in text:
        raise SystemExit(f"tests-tail: hardening tests already present in {path}")
    addition = r'''

    fn sample_index() -> LocalAnnIndex {
        let descriptor = descriptor();
        let mut registry = EmbeddingRegistry::new();
        registry
            .register(Box::new(
                QualificationHashOneHotProvider::new(descriptor.clone()).expect("provider"),
            ))
            .expect("register");
        let embedding = registry
            .embed_batch("qualification-hash-one-hot", &["bounded sample"])
            .expect("embed")
            .remove(0);
        build_local_ann_index(AnnIndexBuildDraft {
            index_id: "bounded-index".to_string(),
            generation: 1,
            seed_sha256: Digest32::for_bytes(b"bounded-seed"),
            provider: descriptor,
            items: vec![AnnIndexItemDraft {
                candidate_id: "memory-bounded".to_string(),
                memory_revision: 1,
                content_sha256: embedding.input_sha256,
                embedding,
            }],
        })
        .expect("index")
    }

    #[test]
    fn decode_rejects_oversized_dimensions_before_vector_allocation() {
        let mut bytes = sample_index().encode().expect("encode");
        let dimensions_offset = {
            let mut cursor = ByteCursor::new(&bytes);
            cursor.take(INDEX_MAGIC.len()).expect("magic");
            cursor.read_u32().expect("schema");
            cursor.read_string().expect("namespace");
            cursor.read_string().expect("index id");
            cursor.read_u64().expect("generation");
            cursor.read_u32().expect("algorithm");
            cursor.read_string().expect("provider id");
            cursor.read_digest().expect("provider digest");
            cursor.read_digest().expect("model digest");
            cursor.read_digest().expect("tokenizer digest");
            cursor.position
        };
        bytes[dimensions_offset..dimensions_offset + 4]
            .copy_from_slice(&(MAX_EMBEDDING_DIMENSIONS + 1).to_be_bytes());

        let error = LocalAnnIndex::decode(&bytes).expect_err("oversized dimensions");
        assert!(matches!(
            error,
            ContractError::Corrupt(message) if message.contains("dimensions")
        ));
    }

    #[test]
    fn bounded_reader_rejects_growth_after_metadata_without_unbounded_read() {
        let mut reader = std::io::Cursor::new(vec![0_u8; 16]);
        let error = read_bounded_index_bytes(&mut reader, 8).expect_err("growth");
        assert!(matches!(
            error,
            ContractError::Corrupt(message) if message.contains("changed while being read")
        ));
    }
'''
    path.write_text(text[:-2] + addition + "}\n", encoding="utf-8")


def patch_source_verifier() -> None:
    insert_after_once(
        VERIFIER,
        '                "MAX_INDEX_FILE_BYTES",\n',
        '                "MAX_EMBEDDING_DIMENSIONS",\n'
        '                "read_bounded_index_bytes",\n',
        "verifier-bounded-read-markers",
    )
    insert_after_once(
        VERIFIER,
        '                "tampered_file_and_binding_drift_fail_closed",\n',
        '                "decode_rejects_oversized_dimensions_before_vector_allocation",\n'
        '                "bounded_reader_rejects_growth_after_metadata_without_unbounded_read",\n',
        "verifier-regression-tests",
    )


def verify_scope() -> None:
    actual = git("diff", "--name-only").splitlines()
    if actual != EXPECTED_CHANGED_PATHS:
        raise SystemExit(
            json.dumps(
                {"expected": EXPECTED_CHANGED_PATHS, "actual": actual},
                indent=2,
                sort_keys=True,
            )
        )


def main() -> None:
    patch_index_module()
    patch_index_types()
    patch_index_decoder()
    patch_bounded_reader()
    append_regression_tests()
    patch_source_verifier()
    verify_scope()
    print(
        json.dumps(
            {
                "status": "PASS_P1_1B_BOUNDED_DECODE_PATCH_APPLIED",
                "changed_paths": EXPECTED_CHANGED_PATHS,
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
