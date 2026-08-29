#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one occurrence, found {count}: {old[:80]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def fix_async_bind() -> None:
    path = ROOT / "codex-rs/hepta-inferd/src/lib.rs"
    replace_once(
        path,
        "    UnixListener::bind(socket_path)\n}",
        "    UnixListener::bind(socket_path).await\n}",
    )


def harden_adapter_tuple() -> None:
    path = ROOT / "codex-rs/hepta-infer-core/src/adapter.rs"
    text = path.read_text(encoding="utf-8")
    text = text.replace(
        "use crate::TerminalReceipt;\n",
        "use crate::TerminalReceipt;\nuse crate::hashing::sha256;\n",
        1,
    )
    text = text.replace(
        'const LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST: &str =\n'
        '    "sha256:834047c1a83968ab6d6b52dc1e00dc6cce748733cfad05bf27d05d85a0039900";\n',
        'const LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST: &str =\n'
        '    "sha256:834047c1a83968ab6d6b52dc1e00dc6cce748733cfad05bf27d05d85a0039900";\n'
        'const ADAPTER_TUPLE_DOMAIN: &[u8] = b"hepta.inference.adapter-tuple.v1\\0";\n',
        1,
    )
    old = '''#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactAdapterTuple {
    pub tuple_digest: Digest,
    pub model_id_digest: Digest,
    pub adapter: AdapterId,
    pub capabilities: AdapterCapabilities,
}

impl ExactAdapterTuple {
    pub fn fixed_ollama_granite4_1b(tuple_digest: Digest) -> Result<Self> {
        Ok(Self {
            tuple_digest,
            model_id_digest: Digest::parse(OLLAMA_GRANITE4_1B_MODEL_ID_DIGEST)?,
            adapter: AdapterId::Ollama,
            capabilities: AdapterCapabilities::fixed_ollama_granite4_1b(),
        })
    }

    pub fn fixed_lmstudio_granite4_micro(tuple_digest: Digest) -> Result<Self> {
        Ok(Self {
            tuple_digest,
            model_id_digest: Digest::parse(LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST)?,
            adapter: AdapterId::LmStudio,
            capabilities: AdapterCapabilities::fixed_lmstudio_granite4_micro(),
        })
    }
}
'''
    new = '''#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterTupleEvidence {
    pub provider_version_digest: Digest,
    pub model_artifact_digest: Digest,
    pub tokenizer_digest: Digest,
    pub chat_template_digest: Digest,
    pub runtime_artifact_digest: Digest,
    pub device_profile_digest: Digest,
    pub quantization: String,
}

impl AdapterTupleEvidence {
    pub fn validate(&self) -> Result<()> {
        if self.quantization.is_empty()
            || self.quantization.len() > 32
            || !self
                .quantization
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        {
            return Err(InferError::AdapterConfigInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactAdapterTuple {
    pub tuple_digest: Digest,
    pub model_id_digest: Digest,
    pub adapter: AdapterId,
    pub capabilities: AdapterCapabilities,
    pub evidence: AdapterTupleEvidence,
}

impl ExactAdapterTuple {
    pub fn fixed_ollama_granite4_1b(evidence: AdapterTupleEvidence) -> Result<Self> {
        Self::from_fixed(
            AdapterId::Ollama,
            Digest::parse(OLLAMA_GRANITE4_1B_MODEL_ID_DIGEST)?,
            AdapterCapabilities::fixed_ollama_granite4_1b(),
            evidence,
        )
    }

    pub fn fixed_lmstudio_granite4_micro(evidence: AdapterTupleEvidence) -> Result<Self> {
        Self::from_fixed(
            AdapterId::LmStudio,
            Digest::parse(LMSTUDIO_GRANITE4_MICRO_MODEL_ID_DIGEST)?,
            AdapterCapabilities::fixed_lmstudio_granite4_micro(),
            evidence,
        )
    }

    fn from_fixed(
        adapter: AdapterId,
        model_id_digest: Digest,
        capabilities: AdapterCapabilities,
        evidence: AdapterTupleEvidence,
    ) -> Result<Self> {
        evidence.validate()?;
        let tuple_digest =
            compute_tuple_digest(adapter, &model_id_digest, capabilities, &evidence)?;
        Ok(Self {
            tuple_digest,
            model_id_digest,
            adapter,
            capabilities,
            evidence,
        })
    }

    pub fn validate(&self) -> Result<()> {
        self.evidence.validate()?;
        let expected = compute_tuple_digest(
            self.adapter,
            &self.model_id_digest,
            self.capabilities,
            &self.evidence,
        )?;
        if self.tuple_digest == expected {
            Ok(())
        } else {
            Err(InferError::AdapterConfigInvalid)
        }
    }
}

fn compute_tuple_digest(
    adapter: AdapterId,
    model_id_digest: &Digest,
    capabilities: AdapterCapabilities,
    evidence: &AdapterTupleEvidence,
) -> Result<Digest> {
    let mut preimage = Vec::with_capacity(768);
    preimage.extend_from_slice(ADAPTER_TUPLE_DOMAIN);
    append_tuple_text(&mut preimage, adapter.as_str())?;
    append_tuple_text(&mut preimage, model_id_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.provider_version_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.model_artifact_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.tokenizer_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.chat_template_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.runtime_artifact_digest.as_str())?;
    append_tuple_text(&mut preimage, evidence.device_profile_digest.as_str())?;
    append_tuple_text(&mut preimage, &evidence.quantization)?;
    for qualified in [
        capabilities.semantic_text.is_qualified(),
        capabilities.native_tool_call.is_qualified(),
        capabilities.strict_sse.is_qualified(),
        capabilities.direct_provider_cancel.is_qualified(),
    ] {
        preimage.push(u8::from(qualified));
    }
    digest_from_tuple_bytes(sha256(&[preimage.as_slice()])?)
}

fn append_tuple_text(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
    let length = u64::try_from(value.len()).map_err(|_| InferError::AdapterConfigInvalid)?;
    buffer.extend_from_slice(&length.to_be_bytes());
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}

fn digest_from_tuple_bytes(bytes: [u8; 32]) -> Result<Digest> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Digest::parse(&encoded)
}
'''
    if old not in text:
        raise SystemExit("adapter tuple block not found")
    text = text.replace(old, new, 1)
    text = text.replace(
        "        for tuple in tuples {\n"
        "            if tuple_map\n",
        "        for tuple in tuples {\n"
        "            tuple.validate()?;\n"
        "            if tuple_map\n",
        1,
    )
    marker = '''    fn request(tuple: Digest, policy: Digest, name: &str) -> InferenceRequest {
'''
    helper = '''    fn tuple_evidence(seed: usize) -> AdapterTupleEvidence {
        const FILLS: [char; 15] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let next = |offset: usize| digest(FILLS[(seed + offset) % FILLS.len()]);
        AdapterTupleEvidence {
            provider_version_digest: next(0),
            model_artifact_digest: next(1),
            tokenizer_digest: next(2),
            chat_template_digest: next(3),
            runtime_artifact_digest: next(4),
            device_profile_digest: next(5),
            quantization: "Q4_K_M".to_owned(),
        }
    }

    fn ollama_tuple(seed: usize) -> ExactAdapterTuple {
        must(ExactAdapterTuple::fixed_ollama_granite4_1b(
            tuple_evidence(seed),
        ))
    }

    fn lmstudio_tuple(seed: usize) -> ExactAdapterTuple {
        must(ExactAdapterTuple::fixed_lmstudio_granite4_micro(
            tuple_evidence(seed),
        ))
    }

'''
    if marker not in text:
        raise SystemExit("adapter test request marker not found")
    text = text.replace(marker, helper + marker, 1)

    start = text.index("    #[test]\n    fn exact_policy_and_tuple_select_one_adapter_without_fallback()")
    end = text.rindex("\n}")
    tests = '''    #[test]
    fn exact_policy_and_tuple_select_one_adapter_without_fallback() {
        let tuple = ollama_tuple(0);
        let tuple_digest = tuple.tuple_digest.clone();
        let policy = digest('b');
        let registry = must(AdapterRegistry::new(
            [tuple],
            [PolicyProfile::new(
                policy.clone(),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        let admission = must(registry.admit(&request(
            tuple_digest,
            policy,
            "request-admit",
        )));
        assert_eq!(admission.adapter, AdapterId::Ollama);
        assert_eq!(admission.fallback_attempts, 0);
    }

    #[test]
    fn unsupported_capabilities_reject_before_controller_queueing() {
        let tuple = lmstudio_tuple(0);
        let tuple_digest = tuple.tuple_digest.clone();
        let tool_policy = digest('b');
        let cancel_policy = digest('d');
        let registry = must(AdapterRegistry::new(
            [tuple],
            [
                PolicyProfile::new(
                    tool_policy.clone(),
                    DispatchRequirements::native_tool_call(),
                ),
                PolicyProfile::new(
                    cancel_policy.clone(),
                    DispatchRequirements::cancel_required(),
                ),
            ],
            FallbackPolicy::closed(),
        ));
        let mut controller = must(QualifiedController::new(
            controller_config(tuple_digest.clone()),
            7,
            registry,
        ));
        assert_eq!(
            controller.admit(
                request(tuple_digest.clone(), tool_policy, "request-tool"),
                1
            ),
            Err(InferError::AdapterToolCallUnsupported)
        );
        assert_eq!(
            controller.admit(
                request(tuple_digest, cancel_policy, "request-cancel"),
                1
            ),
            Err(InferError::AdapterProviderCancelUnsupported)
        );
        assert_eq!(controller.snapshot().queued_requests, 0);
    }

    #[test]
    fn strict_sse_is_fail_closed_for_both_fixed_adapters() {
        for (index, tuple) in [ollama_tuple(0), lmstudio_tuple(7)]
            .into_iter()
            .enumerate()
        {
            let policy = digest(if index == 0 { 'b' } else { 'e' });
            let request = request(
                tuple.tuple_digest.clone(),
                policy.clone(),
                "request-sse",
            );
            let registry = must(AdapterRegistry::new(
                [tuple],
                [PolicyProfile::new(
                    policy,
                    DispatchRequirements::strict_sse(),
                )],
                FallbackPolicy::closed(),
            ));
            assert_eq!(
                registry.admit(&request),
                Err(InferError::AdapterStrictSseUnsupported)
            );
        }
    }

    #[test]
    fn unknown_policy_and_any_fallback_fail_closed() {
        let tuple = ollama_tuple(0);
        let tuple_digest = tuple.tuple_digest.clone();
        let registry = must(AdapterRegistry::new(
            [tuple],
            [PolicyProfile::new(
                digest('b'),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        assert_eq!(
            registry.admit(&request(
                tuple_digest,
                digest('c'),
                "request-policy"
            )),
            Err(InferError::AdapterPolicyUnknown)
        );
        assert!(matches!(
            AdapterRegistry::new(
                [ollama_tuple(0)],
                [PolicyProfile::new(
                    digest('b'),
                    DispatchRequirements::semantic_text(),
                )],
                FallbackPolicy {
                    text_fallback: true,
                    ..FallbackPolicy::closed()
                },
            ),
            Err(InferError::AdapterFallbackEnabled)
        ));
    }

    #[test]
    fn registry_and_controller_tuple_sets_must_match_exactly() {
        let controller_tuple = ollama_tuple(0).tuple_digest;
        let registry = must(AdapterRegistry::new(
            [lmstudio_tuple(7)],
            [PolicyProfile::new(
                digest('b'),
                DispatchRequirements::semantic_text(),
            )],
            FallbackPolicy::closed(),
        ));
        assert!(matches!(
            QualifiedController::new(controller_config(controller_tuple), 7, registry),
            Err(InferError::AdapterConfigInvalid)
        ));
    }

    #[test]
    fn tuple_digest_is_recomputed_and_every_evidence_field_is_bound() {
        let original = ollama_tuple(0);
        assert!(original.validate().is_ok());

        let mut mutations = Vec::new();

        let mut value = original.clone();
        value.evidence.provider_version_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.model_artifact_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.tokenizer_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.chat_template_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.runtime_artifact_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.device_profile_digest = digest('0');
        mutations.push(value);

        let mut value = original.clone();
        value.evidence.quantization = "Q8_0".to_owned();
        mutations.push(value);

        let mut value = original.clone();
        value.model_id_digest = digest('0');
        mutations.push(value);

        let mut value = original;
        value.capabilities.native_tool_call = CapabilityEvidence::UnsupportedFailClosed;
        mutations.push(value);

        for mutation in mutations {
            assert_eq!(mutation.validate(), Err(InferError::AdapterConfigInvalid));
            assert!(matches!(
                AdapterRegistry::new(
                    [mutation],
                    [PolicyProfile::new(
                        digest('b'),
                        DispatchRequirements::semantic_text(),
                    )],
                    FallbackPolicy::closed(),
                ),
                Err(InferError::AdapterConfigInvalid)
            ));
        }
    }
'''
    text = text[:start] + tests + text[end:]
    path.write_text(text, encoding="utf-8")

    crate_root = ROOT / "codex-rs/hepta-infer-core/src/lib.rs"
    root_text = crate_root.read_text(encoding="utf-8")
    export = "pub use adapter::AdapterTupleEvidence;\n"
    if export not in root_text:
        marker = "pub use adapter::AdapterRegistry;\n"
        if marker not in root_text:
            raise SystemExit("adapter export marker not found")
        root_text = root_text.replace(marker, marker + export, 1)
        crate_root.write_text(root_text, encoding="utf-8")


def harden_source_truth() -> None:
    path = ROOT / "scripts/hepta-inference-v4-source-truth.py"
    text = path.read_text(encoding="utf-8")
    text = text.replace(
        '    "codex-rs/hepta-infer-core/src/controller.rs",\n',
        '    "codex-rs/hepta-infer-core/src/adapter.rs",\n'
        '    "codex-rs/hepta-infer-core/src/capability.rs",\n'
        '    "codex-rs/hepta-infer-core/src/controller.rs",\n',
        1,
    )
    text = text.replace(
        '    "codex-rs/hepta-infer-core/src/tests.rs",\n',
        '    "codex-rs/hepta-infer-core/src/tests.rs",\n'
        '    "codex-rs/hepta-infer-core/src/worker.rs",\n',
        1,
    )
    old_load = '''def load_json(relative: str) -> dict[str, Any]:
    try:
        value = json.loads((ROOT / relative).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a JSON object")
    return value
'''
    new_load = '''def load_json(relative: str) -> dict[str, Any]:
    def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                fail(f"{relative} contains duplicate JSON key {key!r}")
            value[key] = item
        return value

    try:
        value = json.loads(
            (ROOT / relative).read_text(encoding="utf-8"),
            object_pairs_hook=reject_duplicate_keys,
        )
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a JSON object")
    return value
'''
    if old_load not in text:
        raise SystemExit("source-truth load_json block not found")
    text = text.replace(old_load, new_load, 1)
    old_sources = '''    sources = {
        "controller": (
            ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
        ).read_text(encoding="utf-8"),
'''
    new_sources = '''    sources = {
        "crate_root": (
            ROOT / "codex-rs/hepta-infer-core/src/lib.rs"
        ).read_text(encoding="utf-8"),
        "adapter": (
            ROOT / "codex-rs/hepta-infer-core/src/adapter.rs"
        ).read_text(encoding="utf-8"),
        "capability": (
            ROOT / "codex-rs/hepta-infer-core/src/capability.rs"
        ).read_text(encoding="utf-8"),
        "controller": (
            ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
        ).read_text(encoding="utf-8"),
'''
    if old_sources not in text:
        raise SystemExit("source-truth sources block marker not found")
    text = text.replace(old_sources, new_sources, 1)
    text = text.replace(
        '''    required = {
        "controller": [
''',
        '''    required = {
        "crate_root": [
            "mod adapter;",
            "mod capability;",
            "mod worker;",
            "pub use capability::CapabilityKey;",
            "pub use worker::WorkerSupervisor;",
        ],
        "adapter": [
            "AdapterTupleEvidence",
            "compute_tuple_digest",
            "tuple.validate()?",
        ],
        "capability": [
            "CapabilityKey",
            "RequestGrant",
            "constant_time_equal",
            "zeroize_private_slice",
        ],
        "controller": [
''',
        1,
    )
    old_return = '''    for component, markers in required.items():
        missing = [marker for marker in markers if marker not in sources[component]]
        if missing:
            fail(f"{component} hardening source missing markers: {missing}")
    return required
'''
    new_return = '''    for component, markers in required.items():
        missing = [marker for marker in markers if marker not in sources[component]]
        if missing:
            fail(f"{component} hardening source missing markers: {missing}")

    transient_writers = []
    for workflow in (ROOT / ".github/workflows").glob("hepta-inference*.yml"):
        content = workflow.read_text(encoding="utf-8")
        if "contents: write" in content:
            transient_writers.append(str(workflow.relative_to(ROOT)))
    if transient_writers:
        fail(f"inference workflows must remain read-only: {sorted(transient_writers)}")

    return required
'''
    if old_return not in text:
        raise SystemExit("source-truth hardening return not found")
    text = text.replace(old_return, new_return, 1)
    path.write_text(text, encoding="utf-8")


def main() -> None:
    fix_async_bind()
    harden_adapter_tuple()
    harden_source_truth()


if __name__ == "__main__":
    main()
