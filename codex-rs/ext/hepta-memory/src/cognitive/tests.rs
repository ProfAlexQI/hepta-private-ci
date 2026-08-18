use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_extension_api::EPHEMERAL_MODEL_INPUT_MAX_CONTENT_BYTES;
use codex_extension_api::EPHEMERAL_MODEL_INPUT_MAX_CONTENT_TOKENS;
use codex_extension_api::EPHEMERAL_MODEL_INPUT_SCHEMA_VERSION;
use codex_extension_api::EphemeralModelInputContext;
use codex_extension_api::EphemeralModelInputContributor;
use codex_extension_api::EphemeralModelInputProposal;
use codex_extension_api::ExtensionData;
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderSha256Digest;
use codex_extension_api::ModelProviderTransport;
use codex_extension_api::TurnInputContext;
use codex_extension_api::TurnInputContributor;
use codex_extension_api::TurnInputEnvironment;
use codex_hepta_contracts::AgentId;
use codex_hepta_memory::CognitiveScope;
use codex_hepta_memory::ForgetMemoryDraft;
use codex_hepta_memory::LedgerSourceKind;
use codex_hepta_memory::MemoryDraft;
use codex_hepta_memory::MemoryRevisionDraft;
use codex_hepta_memory::SourceDraft;
use codex_hepta_paths::HeptaFleetRoot;
use codex_protocol::user_input::UserInput;
use codex_utils_path_uri::PathUri;
use tempfile::TempDir;

use super::*;

const THREAD_ID: &str = "00000000-0000-4000-8000-000000000301";

struct CognitiveFixture {
    _temp: TempDir,
    store: Arc<CognitiveStore>,
    workspace: PathBuf,
    access: CognitiveAccess,
    scope: CognitiveScope,
    memory: MemoryRevisionRecord,
}

struct CountingRecallBackend {
    store: Arc<CognitiveStore>,
    retrieve_calls: AtomicUsize,
    revalidate_calls: AtomicUsize,
}

impl CountingRecallBackend {
    fn new(store: Arc<CognitiveStore>) -> Self {
        Self {
            store,
            retrieve_calls: AtomicUsize::new(0),
            revalidate_calls: AtomicUsize::new(0),
        }
    }
}

impl CognitiveRecallBackend for CountingRecallBackend {
    fn retrieve<'a>(
        &'a self,
        access: &'a CognitiveAccess,
        request: &'a RetrievalRequest,
    ) -> RetrievalFuture<'a> {
        self.retrieve_calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(self.store.retrieve_memory_candidates(access, request))
    }

    fn revalidate<'a>(
        &'a self,
        access: &'a CognitiveAccess,
        binding: &'a MemoryRevalidationBinding,
        now_unix_seconds: i64,
    ) -> RevalidationFuture<'a> {
        self.revalidate_calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(
            self.store
                .revalidate_memory_candidate(access, binding, now_unix_seconds),
        )
    }
}

fn text_input(text: &str) -> Vec<UserInput> {
    vec![UserInput::Text {
        text: text.to_string(),
        text_elements: Vec::new(),
    }]
}

async fn cognitive_fixture() -> CognitiveFixture {
    let temp = tempfile::tempdir().expect("tempdir");
    let fleet_root = temp.path().join("fleet");
    std::fs::create_dir_all(&fleet_root).expect("fleet root");
    let agent_id = AgentId::parse("00000000-0000-4000-8000-000000000311").expect("agent id");
    let layout = HeptaFleetRoot::parse(fleet_root)
        .expect("fleet")
        .layout()
        .agent(&agent_id);
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&workspace).expect("workspace");
    let store = Arc::new(
        CognitiveStore::open(&layout)
            .await
            .expect("cognitive store"),
    );
    let workspace_sha256 = workspace_digest(&workspace);
    let access = CognitiveAccess::workspace_private(agent_id, workspace_sha256.clone());
    let scope = CognitiveScope::WorkspacePrivate { workspace_sha256 };
    let now = now_unix_seconds().expect("time");
    let citation = store
        .append_source(
            &access,
            &SourceDraft {
                scope: scope.clone(),
                kind: LedgerSourceKind::ExplicitMemoryDirective,
                event_key: "seed-rust-durability".to_string(),
                content: b"rust durability is required".to_vec(),
                observed_at_unix_seconds: now,
            },
        )
        .await
        .expect("source");
    let memory = store
        .remember_memory(
            &access,
            &MemoryDraft {
                stable_key: "rust-durability".to_string(),
                revision: MemoryRevisionDraft {
                    scope: scope.clone(),
                    content: "rust durability is required".to_string(),
                    verification: MemoryVerification::Verified,
                    lifecycle: MemoryLifecycleState::Active,
                    valid_from_unix_seconds: now,
                    valid_to_unix_seconds: None,
                    citations: vec![citation],
                },
            },
        )
        .await
        .expect("memory");
    CognitiveFixture {
        _temp: temp,
        store,
        workspace,
        access,
        scope,
        memory,
    }
}

fn cognitive_thread_store() -> ExtensionData {
    let store = ExtensionData::new(THREAD_ID);
    store.insert(HeptaMemoryThreadState::for_cognitive_test(true));
    store
}

async fn prepare(
    extension: &CognitiveExtension,
    thread_store: &ExtensionData,
    turn_store: &ExtensionData,
    workspace: &Path,
    query: &str,
) {
    let session_store = ExtensionData::new("session-cognitive");
    let fragments = TurnInputContributor::contribute(
        extension,
        TurnInputContext {
            turn_id: turn_store.level_id().to_string(),
            user_input: text_input(query),
            environments: vec![TurnInputEnvironment {
                environment_id: "primary".to_string(),
                cwd: PathUri::from_host_native_path(workspace).expect("absolute workspace"),
                is_primary: true,
            }],
        },
        None,
        &session_store,
        thread_store,
        turn_store,
    )
    .await;
    assert!(fragments.is_empty());
    assert!(EphemeralModelInputContributor::is_active(
        extension,
        thread_store,
        turn_store,
    ));
}

async fn propose(
    extension: &CognitiveExtension,
    thread_store: &ExtensionData,
    turn_store: &ExtensionData,
    workspace: &Path,
    attempt_id: &str,
) -> Option<EphemeralModelInputProposal> {
    let session_store = ExtensionData::new("session-cognitive");
    let base = ModelProviderSha256Digest::parse("1".repeat(64)).expect("digest");
    EphemeralModelInputContributor::contribute(
        extension,
        EphemeralModelInputContext {
            schema_version: EPHEMERAL_MODEL_INPUT_SCHEMA_VERSION,
            session_store: &session_store,
            thread_store,
            turn_store,
            attempt_id,
            base_logical_request_sha256: &base,
            thread_id: THREAD_ID,
            turn_id: turn_store.level_id(),
            cwd: workspace,
            request_kind: ModelProviderRequestKind::Turn,
            provider_id: "provider-cognitive",
            model: "model-cognitive",
            transport: ModelProviderTransport::Http,
            generate: true,
            model_context_window: Some(128_000),
            max_content_bytes: EPHEMERAL_MODEL_INPUT_MAX_CONTENT_BYTES,
            max_content_tokens: EPHEMERAL_MODEL_INPUT_MAX_CONTENT_TOKENS,
        },
    )
    .await
    .expect("fail-open cognitive contributor")
}

#[test]
fn exact_directive_capture_is_digest_only_and_byte_exact() {
    let capture = capture_directive(&text_input("remember this exactly"));
    assert_eq!(capture.content_bytes, 21);
    assert_eq!(capture.query.as_deref(), Some("remember this exactly"));
    assert!(capture.byte_exact_verification_allowed);
    assert_eq!(
        capture.content_sha256,
        Sha256Digest::for_bytes(b"remember this exactly")
    );
}

#[test]
fn secret_detector_covers_common_key_shapes() {
    for secret in [
        "api_key=secret-value",
        "AKIAIOSFODNN7EXAMPLE",
        "sk-abcdefghijklmnopqrstuvwxyz0123456789",
        "ghp_abcdefghijklmnopqrstuvwxyz0123456789",
        "github_pat_abcdefghijklmnopqrstuvwxyz0123456789",
        "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature",
        "-----BEGIN PRIVATE KEY----- payload",
    ] {
        assert!(secret_like(secret.as_bytes()), "missed {secret}");
    }
    assert!(!secret_like(b"remember that API keys must never be stored"));
}

#[test]
fn query_larger_than_two_kib_is_not_prepared_but_witness_remains() {
    let text = "x".repeat(MAX_RETRIEVAL_QUERY_BYTES + 1);
    let capture = capture_directive(&text_input(&text));
    assert!(capture.query.is_none());
    assert_eq!(capture.content_bytes, text.len());
    assert!(capture.byte_exact_verification_allowed);
}

#[tokio::test]
async fn every_physical_attempt_revalidates_with_stable_source_content_and_binding() {
    let fixture = cognitive_fixture().await;
    let backend = Arc::new(CountingRecallBackend::new(fixture.store.clone()));
    let extension = CognitiveExtension::with_recall(fixture.store, backend.clone());
    let thread_store = cognitive_thread_store();
    let turn_store = ExtensionData::new("turn-retry-revalidation");
    prepare(
        &extension,
        &thread_store,
        &turn_store,
        &fixture.workspace,
        "rust durability",
    )
    .await;
    assert_eq!(backend.retrieve_calls.load(Ordering::SeqCst), 1);

    let first = propose(
        &extension,
        &thread_store,
        &turn_store,
        &fixture.workspace,
        "model-provider-attempt:v1:first",
    )
    .await
    .expect("first proposal");
    let first_source = first.source().as_str().to_string();
    let first_binding = first.source_binding_sha256().as_str().to_string();
    let first_content_sha = first.content_sha256().as_str().to_string();
    let first_content = first.into_content();

    let retry = propose(
        &extension,
        &thread_store,
        &turn_store,
        &fixture.workspace,
        "model-provider-attempt:v1:retry",
    )
    .await
    .expect("retry proposal");
    assert_eq!(retry.source().as_str(), COGNITIVE_SOURCE);
    assert_eq!(retry.source().as_str(), first_source);
    assert_eq!(
        retry.source_binding_sha256().as_str(),
        first_binding.as_str()
    );
    assert_eq!(retry.content_sha256().as_str(), first_content_sha.as_str());
    assert_eq!(retry.into_content(), first_content);
    assert_eq!(backend.retrieve_calls.load(Ordering::SeqCst), 1);
    assert_eq!(backend.revalidate_calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn prepared_revision_is_dropped_after_correction_and_after_tombstone() {
    let fixture = cognitive_fixture().await;
    let backend = Arc::new(CountingRecallBackend::new(fixture.store.clone()));
    let extension = CognitiveExtension::with_recall(fixture.store.clone(), backend.clone());
    let thread_store = cognitive_thread_store();
    let correction_turn = ExtensionData::new("turn-before-correction");
    prepare(
        &extension,
        &thread_store,
        &correction_turn,
        &fixture.workspace,
        "rust durability",
    )
    .await;

    let now = now_unix_seconds().expect("time");
    let correction_citation = fixture
        .store
        .append_source(
            &fixture.access,
            &SourceDraft {
                scope: fixture.scope.clone(),
                kind: LedgerSourceKind::ExplicitMemoryDirective,
                event_key: "correct-rust-durability".to_string(),
                content: b"rust durability is still required".to_vec(),
                observed_at_unix_seconds: now,
            },
        )
        .await
        .expect("correction source");
    fixture
        .store
        .correct_memory(
            &fixture.access,
            &fixture.memory.id.memory_id,
            1,
            &MemoryRevisionDraft {
                scope: fixture.scope.clone(),
                content: "rust durability is still required".to_string(),
                verification: MemoryVerification::Verified,
                lifecycle: MemoryLifecycleState::Active,
                valid_from_unix_seconds: now,
                valid_to_unix_seconds: None,
                citations: vec![correction_citation],
            },
        )
        .await
        .expect("correction");
    assert!(
        propose(
            &extension,
            &thread_store,
            &correction_turn,
            &fixture.workspace,
            "model-provider-attempt:v1:after-correction",
        )
        .await
        .is_none()
    );

    let tombstone_turn = ExtensionData::new("turn-before-tombstone");
    prepare(
        &extension,
        &thread_store,
        &tombstone_turn,
        &fixture.workspace,
        "rust durability",
    )
    .await;
    let tombstone_citation = fixture
        .store
        .append_source(
            &fixture.access,
            &SourceDraft {
                scope: fixture.scope.clone(),
                kind: LedgerSourceKind::ExplicitMemoryDirective,
                event_key: "forget-rust-durability".to_string(),
                content: b"forget rust durability".to_vec(),
                observed_at_unix_seconds: now,
            },
        )
        .await
        .expect("tombstone source");
    fixture
        .store
        .forget_memory(
            &fixture.access,
            &fixture.memory.id.memory_id,
            2,
            &ForgetMemoryDraft {
                scope: fixture.scope,
                reason: "explicitly withdrawn".to_string(),
                valid_from_unix_seconds: now,
                citations: vec![tombstone_citation],
            },
        )
        .await
        .expect("tombstone");
    assert!(
        propose(
            &extension,
            &thread_store,
            &tombstone_turn,
            &fixture.workspace,
            "model-provider-attempt:v1:after-tombstone",
        )
        .await
        .is_none()
    );
    assert_eq!(backend.retrieve_calls.load(Ordering::SeqCst), 2);
    assert_eq!(backend.revalidate_calls.load(Ordering::SeqCst), 2);
}
