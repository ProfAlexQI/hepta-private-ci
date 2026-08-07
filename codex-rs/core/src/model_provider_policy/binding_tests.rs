use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::ModelProviderRequestKind;
use codex_extension_api::ModelProviderTransport;
use serde_json::Value;

use super::ModelProviderPolicyContext;
use super::canonical_endpoint_sha256;
use super::canonical_sha256;
use super::prepare_model_provider_policy;

fn stores() -> (ExtensionData, ExtensionData, ExtensionData) {
    (
        ExtensionData::new("session"),
        ExtensionData::new("thread"),
        ExtensionData::new("turn"),
    )
}

#[test]
fn canonical_digest_is_independent_of_object_key_order() {
    let left: Value =
        serde_json::from_str(r#"{"b":{"y":2,"x":1},"a":0}"#).expect("left JSON should parse");
    let right: Value =
        serde_json::from_str(r#"{"a":0,"b":{"x":1,"y":2}}"#).expect("right JSON should parse");

    assert_eq!(
        canonical_sha256(&left).expect("left digest should succeed"),
        canonical_sha256(&right).expect("right digest should succeed")
    );
}

#[test]
fn retry_stable_binding_has_fresh_physical_attempt_identity() {
    let registry = ExtensionRegistryBuilder::<crate::config::Config>::new().build();
    let (session_store, thread_store, turn_store) = stores();
    let context = ModelProviderPolicyContext {
        registry: &registry,
        session_store: &session_store,
        thread_store: &thread_store,
        turn_store: &turn_store,
        thread_id: "thread-1".to_string(),
        turn_id: "turn-1".to_string(),
        request_kind: ModelProviderRequestKind::Turn,
    };
    let logical = serde_json::json!({"input": [1, 2, 3], "model": "model-1"});
    let websocket_wire = serde_json::json!({
        "request": {"input": [3], "previous_response_id": "prior"},
        "routing_hint": "model=model-1;tier=fast",
    });
    let http_wire = serde_json::json!({
        "request": logical,
        "routing_hint": "model=model-1;tier=fast",
    });

    let first = prepare_model_provider_policy(
        &context,
        "provider-1",
        "model-1",
        ModelProviderTransport::WebSocket,
        "https://user:secret@example.test/responses?region=one&api_key=secret-one",
        &logical,
        &websocket_wire,
        Some("prior"),
        true,
    )
    .expect("first binding should succeed");
    let second = prepare_model_provider_policy(
        &context,
        "provider-1",
        "model-1",
        ModelProviderTransport::Http,
        "https://other:rotated@example.test/responses?api_key=rotated&region=two",
        &logical,
        &http_wire,
        None,
        true,
    )
    .expect("second binding should succeed");

    assert_eq!(first.request_binding_id, second.request_binding_id);
    assert_ne!(first.attempt_id, second.attempt_id);
    assert_eq!(first.provider_config_sha256, second.provider_config_sha256);
    assert_eq!(first.endpoint_sha256, second.endpoint_sha256);
    assert_ne!(first.wire_semantic_sha256, second.wire_semantic_sha256);
}

#[test]
fn routing_hint_is_physical_wire_semantics_not_logical_identity() {
    let registry = ExtensionRegistryBuilder::<crate::config::Config>::new().build();
    let (session_store, thread_store, turn_store) = stores();
    let context = ModelProviderPolicyContext {
        registry: &registry,
        session_store: &session_store,
        thread_store: &thread_store,
        turn_store: &turn_store,
        thread_id: "thread-1".to_string(),
        turn_id: "turn-1".to_string(),
        request_kind: ModelProviderRequestKind::Turn,
    };
    let logical = serde_json::json!({
        "input": [1],
        "model": "model-1",
        "service_tier": "fast",
    });
    let with_hint =
        serde_json::json!({"request": logical, "routing_hint": "model=model-1;tier=fast"});
    let without_hint = serde_json::json!({"request": logical, "routing_hint": null});

    let first = prepare_model_provider_policy(
        &context,
        "provider-1",
        "model-1",
        ModelProviderTransport::Http,
        "https://example.test/responses",
        &logical,
        &with_hint,
        None,
        true,
    )
    .expect("hinted binding should succeed");
    let second = prepare_model_provider_policy(
        &context,
        "provider-1",
        "model-1",
        ModelProviderTransport::Http,
        "https://example.test/responses",
        &logical,
        &without_hint,
        None,
        true,
    )
    .expect("unhinted binding should succeed");

    assert_eq!(first.request_binding_id, second.request_binding_id);
    assert_ne!(first.wire_semantic_sha256, second.wire_semantic_sha256);
}

#[test]
fn endpoint_digest_sorts_query_names_and_excludes_secrets() {
    let left = canonical_endpoint_sha256(
        "https://alice:secret@example.test/v1/responses?z=secret-z&a=secret-a&z=again",
    )
    .expect("left endpoint should parse");
    let right = canonical_endpoint_sha256(
        "https://bob:rotated@example.test/v1/responses?z=changed&z=other&a=changed",
    )
    .expect("right endpoint should parse");
    let different_path =
        canonical_endpoint_sha256("https://example.test/v2/responses?a=changed&z=changed&z=other")
            .expect("different endpoint should parse");

    assert_eq!(left, right);
    assert_ne!(left, different_path);
}
