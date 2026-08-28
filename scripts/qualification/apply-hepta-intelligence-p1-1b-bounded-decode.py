#!/usr/bin/env python3
"""Apply the exact P1.1b bounded ANN decoder hardening patch.

This helper exists only on an isolated qualification wrapper branch. The output
commit is created from the frozen source SHA and contains only the six reviewed
source/verifier paths.
"""

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


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True).strip()


def patch_index_module() -> None:
    replace_once(
        CRATE_ROOT / "src/index.rs",
        "use crate::embedding::LocalEmbeddingDescriptor;\n"
        "use crate::embedding::Q15_UNIT_NORM_SQUARED;\n",
        "use crate::embedding::LocalEmbeddingDescriptor;\n"
        "use crate::embedding::MAX_EMBEDDING_DIMENSIONS;\n"
        "use crate::embedding::Q15_UNIT_NORM_SQUARED;\n",
    )


def patch_index_types() -> None:
    path = CRATE_ROOT / "src/index/types.rs"
    replace_once(
        path,
        """        if self.item_count == 0
            || usize::try_from(self.item_count).unwrap_or(usize::MAX) > MAX_INDEX_ITEMS
            || self.bucket_count == 0
        {
            return Err(ContractError::Corrupt(
                "ANN manifest counts are outside bounded limits".to_string(),
            ));
        }
""",
        """        if self.item_count == 0
            || usize::try_from(self.item_count).unwrap_or(usize::MAX) > MAX_INDEX_ITEMS
            || self.bucket_count == 0
            || self.bucket_count > self.item_count
        {
            return Err(ContractError::Corrupt(
                "ANN manifest counts are outside bounded limits".to_string(),
            ));
        }
        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions) {
            return Err(ContractError::Corrupt(
                "ANN manifest dimensions exceed the bounded embedding contract".to_string(),
            ));
        }
""",
    )
    replace_once(
        path,
        """        if self.generation == 0 || self.dimensions == 0 {
            return Err(ContractError::Invalid(
                "expected ANN generation and dimensions must be non-zero".to_string(),
            ));
        }
""",
        """        if self.generation == 0
            || !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&self.dimensions)
        {
            return Err(ContractError::Invalid(format!(
                "expected ANN generation must be non-zero and dimensions must contain 8..={MAX_EMBEDDING_DIMENSIONS}"
            )));
        }
""",
    )


def patch_index_decoder() -> None:
    path = CRATE_ROOT / "src/index/impl.rs"
    replace_once(
        path,
        """        let dimensions = cursor.read_u32()?;
        let metric = metric_from_code(cursor.read_u32()?)?;
""",
        """        let dimensions = cursor.read_u32()?;
        if !(8..=MAX_EMBEDDING_DIMENSIONS).contains(&dimensions) {
            return Err(ContractError::Corrupt(
                "ANN file dimensions exceed the bounded embedding contract".to_string(),
            ));
        }
        let metric = metric_from_code(cursor.read_u32()?)?;
""",
    )
    replace_once(
        path,
        """        if item_count == 0
            || usize::try_from(item_count).unwrap_or(usize::MAX) > MAX_INDEX_ITEMS
            || bucket_count == 0
        {
""",
        """        if item_count == 0
            || usize::try_from(item_count).unwrap_or(usize::MAX) > MAX_INDEX_ITEMS
            || bucket_count == 0
            || bucket_count > item_count
        {
""",
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
    )
    replace_once(
        path,
        """    let file_bytes = file.metadata()?.len();
    if file_bytes == 0 || file_bytes > MAX_INDEX_FILE_BYTES {
        return Err(ContractError::Corrupt(
            "ANN index file size is outside bounded limits".to_string(),
        ));
    }
    let mut bytes = Vec::with_capacity(
        usize::try_from(file_bytes).map_err(|_| ContractError::Overflow)?,
    );
    file.read_to_end(&mut bytes)?;
""",
        """    let file_bytes = file.metadata()?.len();
    let bytes = read_bounded_index_bytes(&mut file, file_bytes)?;
""",
    )


def append_regression_tests() -> None:
    path = CRATE_ROOT / "src/index/tests_module.rs"
    text = path.read_text(encoding="utf-8")
    if not text.endswith("}\n"):
        raise SystemExit(f"{path}: expected final module brace")
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
    replace_once(
        VERIFIER,
        """                "OpenOptions::new().create_new(true)",
                "sync_all",
                "MAX_INDEX_FILE_BYTES",
""",
        """                "OpenOptions::new().create_new(true)",
                "sync_all",
                "MAX_INDEX_FILE_BYTES",
                "MAX_EMBEDDING_DIMENSIONS",
                "read_bounded_index_bytes",
""",
    )
    replace_once(
        VERIFIER,
        """                "tampered_file_and_binding_drift_fail_closed",
                "route_is_dependency_first_and_fails_to_lexical_only",
""",
        """                "tampered_file_and_binding_drift_fail_closed",
                "decode_rejects_oversized_dimensions_before_vector_allocation",
                "bounded_reader_rejects_growth_after_metadata_without_unbounded_read",
                "route_is_dependency_first_and_fails_to_lexical_only",
""",
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
