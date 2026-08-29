#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "codex-rs/hepta-infer-adapter/src/lib.rs"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL_HEPTA_INFERENCE_INF2A_REPAIR: {message}")


def main() -> None:
    text = SOURCE.read_text(encoding="utf-8")
    original = """        let key = adapter.profile().model_tuple_digest.clone();
        if self.adapters.insert(key, adapter).is_some() {
            return Err(AdapterError::DuplicateTuple);
        }
        Ok(())
"""
    repaired = """        let key = adapter.profile().model_tuple_digest.clone();
        if self.adapters.contains_key(&key) {
            return Err(AdapterError::DuplicateTuple);
        }
        self.adapters.insert(key, adapter);
        Ok(())
"""
    if original in text:
        text = text.replace(original, repaired, 1)
    else:
        require(repaired in text, "duplicate registration anchor drift")

    anchor = """    #[test]
    fn unsafe_profiles_are_rejected() {
"""
    regression = """    #[tokio::test]
    async fn duplicate_registration_preserves_original_adapter() {
        let original = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let replacement = Arc::new(DeterministicAdapter::new(matrix(
            QUALIFIED,
            UNSUPPORTED,
            UNSUPPORTED,
        )));
        let mut registry = AdapterRegistry::default();
        if let Err(error) = registry.register(original.clone()) {
            panic!(\"unexpected registration error: {error}\");
        }
        assert_eq!(
            registry.register(replacement.clone()),
            Err(AdapterError::DuplicateTuple)
        );
        if let Err(error) = registry
            .execute(&request(QualificationFixture::SemanticHeptaOk))
            .await
        {
            panic!(\"unexpected execution error: {error}\");
        }
        assert_eq!(original.executions.load(Ordering::SeqCst), 1);
        assert_eq!(replacement.executions.load(Ordering::SeqCst), 0);
    }

"""
    if regression not in text:
        require(anchor in text, "regression test anchor drift")
        text = text.replace(anchor, regression + anchor, 1)
    require(text.count(regression) == 1, "duplicate regression test")
    SOURCE.write_text(text, encoding="utf-8")
    print("PASS_HEPTA_INFERENCE_INF2A_ADAPTER_CONTRACT_REPAIR")


if __name__ == "__main__":
    main()
