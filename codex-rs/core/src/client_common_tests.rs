use codex_api::OpenAiVerbosity;
use codex_api::ResponsesApiRequest;
use codex_api::TextControls;
use codex_api::create_text_param_for_request;
use codex_protocol::config_types::ServiceTier;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ImageDetail;
use pretty_assertions::assert_eq;
use serde_json::value::RawValue;
use std::sync::Arc;

use super::*;

fn empty_tools() -> Arc<RawValue> {
    Arc::from(RawValue::from_string("[]".to_string()).expect("valid tool JSON"))
}

fn user_text_item(text: impl Into<String>) -> ResponseItem {
    ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText { text: text.into() }],
        phase: None,
        internal_chat_message_metadata_passthrough: None,
    }
}

fn prompt_only_authority(thread_id: &str, turn_id: &str) -> EphemeralInputAuthorityBinding {
    EphemeralInputAuthorityBinding::new(
        thread_id,
        turn_id,
        ModelProviderSha256Digest::parse("a".repeat(64)).expect("authority digest"),
    )
    .expect("authority binding")
}

fn provider_digest(byte: char) -> ModelProviderSha256Digest {
    ModelProviderSha256Digest::parse(byte.to_string().repeat(64)).expect("provider digest")
}

fn prompt_with_image_outputs() -> Prompt {
    Prompt {
        input: vec![
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputImage {
                    image_url: "https://example.com/image.png".to_string(),
                    detail: Some(ImageDetail::Original),
                }],
                phase: None,
                internal_chat_message_metadata_passthrough: None,
            },
            ResponseItem::FunctionCallOutput {
                id: None,
                call_id: "function-call".to_string(),
                output: FunctionCallOutputPayload::from_content_items(vec![
                    FunctionCallOutputContentItem::InputImage {
                        image_url: "data:image/png;base64,function".to_string(),
                        detail: Some(ImageDetail::High),
                    },
                ]),
                internal_chat_message_metadata_passthrough: None,
            },
            ResponseItem::CustomToolCallOutput {
                id: None,
                call_id: "custom-call".to_string(),
                name: None,
                output: FunctionCallOutputPayload::from_content_items(vec![
                    FunctionCallOutputContentItem::InputImage {
                        image_url: "data:image/png;base64,custom".to_string(),
                        detail: Some(ImageDetail::Auto),
                    },
                ]),
                internal_chat_message_metadata_passthrough: None,
            },
        ],
        ..Default::default()
    }
}

#[test]
fn responses_lite_request_copies_strip_image_details() {
    let prompt = prompt_with_image_outputs();
    let original = prompt.input.clone();

    let stripped = prompt.get_formatted_input_for_request(/*use_responses_lite*/ true);

    assert_eq!(
        stripped,
        vec![
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputImage {
                    image_url: "https://example.com/image.png".to_string(),
                    detail: None,
                }],
                phase: None,
                internal_chat_message_metadata_passthrough: None,
            },
            ResponseItem::FunctionCallOutput {
                id: None,
                call_id: "function-call".to_string(),
                output: FunctionCallOutputPayload::from_content_items(vec![
                    FunctionCallOutputContentItem::InputImage {
                        image_url: "data:image/png;base64,function".to_string(),
                        detail: None,
                    },
                ]),
                internal_chat_message_metadata_passthrough: None,
            },
            ResponseItem::CustomToolCallOutput {
                id: None,
                call_id: "custom-call".to_string(),
                name: None,
                output: FunctionCallOutputPayload::from_content_items(vec![
                    FunctionCallOutputContentItem::InputImage {
                        image_url: "data:image/png;base64,custom".to_string(),
                        detail: None,
                    },
                ]),
                internal_chat_message_metadata_passthrough: None,
            },
        ]
    );
    assert_eq!(prompt.input, original);
    assert_eq!(
        prompt.get_formatted_input_for_request(/*use_responses_lite*/ false),
        original
    );
}

#[test]
fn prompt_only_input_is_request_visible_without_mutating_history() {
    let history = user_text_item("persisted history");
    let ephemeral = user_text_item("bounded prompt-only reference");
    let mut prompt = Prompt {
        input: vec![history.clone()],
        ..Default::default()
    };

    prompt
        .set_ephemeral_input(vec![ephemeral.clone()])
        .expect("bounded prompt-only input");

    assert_eq!(prompt.input, vec![history.clone()]);
    assert_eq!(
        prompt.get_formatted_input_for_request(/*use_responses_lite*/ false),
        vec![history, ephemeral]
    );
    assert!(prompt.ephemeral_input_sha256().is_some());
}

#[test]
fn prompt_only_input_rejects_items_at_or_above_one_thousand_bytes() {
    let mut prompt = Prompt::default();

    let error = prompt
        .set_ephemeral_input(vec![user_text_item("x".repeat(1_000))])
        .expect_err("oversized prompt-only input must fail closed");

    assert_eq!(error.reason_code(), "ephemeral_model_input_invalid");
    assert!(prompt.ephemeral_input_sha256().is_none());
    assert!(
        prompt
            .get_formatted_input_for_request(/*use_responses_lite*/ false)
            .is_empty()
    );
}

#[test]
fn prompt_only_witness_is_exact_scope_and_single_use_across_clones() {
    let mut prompt = Prompt::default();
    prompt
        .set_ephemeral_input_with_witness(
            vec![user_text_item("bounded prompt-only reference")],
            prompt_only_authority("thread-1", "turn-1"),
        )
        .expect("host-minted witness");
    let cloned = prompt.clone();

    let mismatch = prompt
        .consume_ephemeral_input_witness("thread-2", "turn-1", "attempt-1", &provider_digest('b'))
        .expect_err("scope substitution must fail closed");
    assert_eq!(
        mismatch.reason_code(),
        "ephemeral_model_input_witness_scope_mismatch"
    );

    let witness = cloned
        .consume_ephemeral_input_witness("thread-1", "turn-1", "attempt-1", &provider_digest('b'))
        .expect("exact first consume")
        .expect("witness digest");
    assert_eq!(witness.as_str().len(), 64);

    let replay = prompt
        .consume_ephemeral_input_witness("thread-1", "turn-1", "attempt-1", &provider_digest('b'))
        .expect_err("cloned prompt must share single-use state");
    assert_eq!(
        replay.reason_code(),
        "ephemeral_model_input_witness_replayed"
    );
    let retry = prompt
        .consume_ephemeral_input_witness("thread-1", "turn-1", "attempt-2", &provider_digest('b'))
        .expect("a distinct host attempt mints a fresh one-shot witness")
        .expect("retry witness digest");
    assert_ne!(witness, retry);
}

#[test]
fn prompt_only_witness_binds_the_final_logical_request() {
    let mut left = Prompt::default();
    left.set_ephemeral_input_with_witness(
        vec![user_text_item("same attachment")],
        prompt_only_authority("thread-1", "turn-1"),
    )
    .expect("left witness");
    let mut right = Prompt::default();
    right
        .set_ephemeral_input_with_witness(
            vec![user_text_item("same attachment")],
            prompt_only_authority("thread-1", "turn-1"),
        )
        .expect("right witness");

    let left = left
        .consume_ephemeral_input_witness(
            "thread-1",
            "turn-1",
            "attempt-left",
            &provider_digest('b'),
        )
        .expect("left consume");
    let right = right
        .consume_ephemeral_input_witness(
            "thread-1",
            "turn-1",
            "attempt-right",
            &provider_digest('c'),
        )
        .expect("right consume");

    assert_ne!(left, right);
}

#[test]
fn prompt_only_witness_attempt_budget_is_bounded() {
    let mut prompt = Prompt::default();
    prompt
        .set_ephemeral_input_with_witness(
            vec![user_text_item("bounded prompt-only reference")],
            prompt_only_authority("thread-1", "turn-1"),
        )
        .expect("host-minted witness");

    for index in 0..Prompt::MAX_EPHEMERAL_INPUT_ATTEMPTS {
        prompt
            .consume_ephemeral_input_witness(
                "thread-1",
                "turn-1",
                format!("attempt-{index}").as_str(),
                &provider_digest('b'),
            )
            .expect("bounded attempt should be accepted");
    }
    let error = prompt
        .consume_ephemeral_input_witness(
            "thread-1",
            "turn-1",
            "attempt-overflow",
            &provider_digest('b'),
        )
        .expect_err("attempt budget overflow must fail closed");
    assert_eq!(
        error.reason_code(),
        "ephemeral_model_input_witness_attempt_limit"
    );
}

#[test]
fn serializes_text_verbosity_when_set() {
    let input: Vec<ResponseItem> = vec![];
    let req = ResponsesApiRequest {
        model: "gpt-5.4".to_string(),
        instructions: "i".to_string(),
        input,
        tools: Some(empty_tools().into()),
        tool_choice: "auto".to_string(),
        parallel_tool_calls: true,
        reasoning: None,
        store: false,
        stream: true,
        stream_options: None,
        include: vec![],
        prompt_cache_key: None,
        service_tier: None,
        text: Some(TextControls {
            verbosity: Some(OpenAiVerbosity::Low),
            format: None,
        }),
        client_metadata: None,
    };

    let v = serde_json::to_value(&req).expect("json");
    assert_eq!(
        v.get("text")
            .and_then(|t| t.get("verbosity"))
            .and_then(|s| s.as_str()),
        Some("low")
    );
}

#[test]
fn serializes_text_schema_with_strict_format() {
    let input: Vec<ResponseItem> = vec![];
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "answer": {"type": "string"}
        },
        "required": ["answer"],
    });
    let text_controls = create_text_param_for_request(
        /*verbosity*/ None,
        &Some(schema.clone()),
        /*output_schema_strict*/ true,
    )
    .expect("text controls");

    let req = ResponsesApiRequest {
        model: "gpt-5.4".to_string(),
        instructions: "i".to_string(),
        input,
        tools: Some(empty_tools().into()),
        tool_choice: "auto".to_string(),
        parallel_tool_calls: true,
        reasoning: None,
        store: false,
        stream: true,
        stream_options: None,
        include: vec![],
        prompt_cache_key: None,
        service_tier: None,
        text: Some(text_controls),
        client_metadata: None,
    };

    let v = serde_json::to_value(&req).expect("json");
    let text = v.get("text").expect("text field");
    assert!(text.get("verbosity").is_none());
    let format = text.get("format").expect("format field");

    assert_eq!(
        format.get("name"),
        Some(&serde_json::Value::String("codex_output_schema".into()))
    );
    assert_eq!(
        format.get("type"),
        Some(&serde_json::Value::String("json_schema".into()))
    );
    assert_eq!(format.get("strict"), Some(&serde_json::Value::Bool(true)));
    assert_eq!(format.get("schema"), Some(&schema));
}

#[test]
fn serializes_text_schema_with_non_strict_format() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "answer": {"type": "string"},
            "rationale": {"type": "string"}
        },
        "required": ["answer"],
        "additionalProperties": false
    });
    let text_controls = create_text_param_for_request(
        /*verbosity*/ None,
        &Some(schema.clone()),
        /*output_schema_strict*/ false,
    )
    .expect("text controls");

    let format = text_controls.format.expect("format field");
    assert!(!format.strict);
    assert_eq!(format.schema, schema);
}

#[test]
fn omits_text_when_not_set() {
    let input: Vec<ResponseItem> = vec![];
    let req = ResponsesApiRequest {
        model: "gpt-5.4".to_string(),
        instructions: "i".to_string(),
        input,
        tools: Some(empty_tools().into()),
        tool_choice: "auto".to_string(),
        parallel_tool_calls: true,
        reasoning: None,
        store: false,
        stream: true,
        stream_options: None,
        include: vec![],
        prompt_cache_key: None,
        service_tier: None,
        text: None,
        client_metadata: None,
    };

    let v = serde_json::to_value(&req).expect("json");
    assert!(v.get("text").is_none());
}

#[test]
fn serializes_flex_service_tier_when_set() {
    let req = ResponsesApiRequest {
        model: "gpt-5.4".to_string(),
        instructions: "i".to_string(),
        input: vec![],
        tools: Some(empty_tools().into()),
        tool_choice: "auto".to_string(),
        parallel_tool_calls: true,
        reasoning: None,
        store: false,
        stream: true,
        stream_options: None,
        include: vec![],
        prompt_cache_key: None,
        service_tier: Some(ServiceTier::Flex.to_string()),
        text: None,
        client_metadata: None,
    };

    let v = serde_json::to_value(&req).expect("json");
    assert_eq!(
        v.get("service_tier").and_then(|tier| tier.as_str()),
        Some("flex")
    );
}
