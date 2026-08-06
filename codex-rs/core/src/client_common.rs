pub use codex_api::ResponseEvent;
use codex_extension_api::ModelProviderPolicyError;
use codex_extension_api::ModelProviderSha256Digest;
use codex_protocol::error::Result;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::ContentItem;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::ResponseItem;
use codex_tools::ToolSpec;
use futures::Stream;
use serde_json::Value;
use sha2::Digest as _;
use sha2::Sha256;
use std::collections::BTreeSet;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// API request payload for a single model turn
#[derive(Debug, Clone)]
pub struct Prompt {
    /// Conversation context input items.
    pub input: Vec<ResponseItem>,

    /// Prompt-only input that must never be recorded in conversation history.
    ephemeral_input: Vec<ResponseItem>,

    /// Exact digest of `ephemeral_input`, set atomically with the items.
    ephemeral_input_sha256: Option<ModelProviderSha256Digest>,

    /// Host-owned authority that mints one single-use witness per provider attempt.
    ephemeral_input_witness: Option<EphemeralInputWitness>,

    /// Tools available to the model, including additional tools sourced from
    /// external MCP servers.
    pub(crate) tools: Vec<ToolSpec>,

    /// Whether parallel tool calls are permitted for this prompt.
    pub(crate) parallel_tool_calls: bool,

    pub base_instructions: BaseInstructions,

    /// Optional the output schema for the model's response.
    pub output_schema: Option<Value>,

    /// Whether the Responses API should strictly validate `output_schema`.
    pub output_schema_strict: bool,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            input: Vec::new(),
            ephemeral_input: Vec::new(),
            ephemeral_input_sha256: None,
            ephemeral_input_witness: None,
            tools: Vec::new(),
            parallel_tool_calls: false,
            base_instructions: BaseInstructions::default(),
            output_schema: None,
            output_schema_strict: true,
        }
    }
}

impl Prompt {
    const MAX_EPHEMERAL_INPUT_BYTES: usize = 999;
    const MAX_EPHEMERAL_INPUT_ATTEMPTS: usize = 16;

    /// Replaces the prompt-only input with one bounded untrusted user reference.
    ///
    /// The items stay separate from `input`, so history, rollout, resume, fork,
    /// and compaction cannot acquire them through normal conversation recording.
    pub(crate) fn set_ephemeral_input(
        &mut self,
        items: Vec<ResponseItem>,
    ) -> std::result::Result<(), ModelProviderPolicyError> {
        if items.is_empty() {
            self.ephemeral_input.clear();
            self.ephemeral_input_sha256 = None;
            self.ephemeral_input_witness = None;
            return Ok(());
        }
        let text = match items.as_slice() {
            [
                ResponseItem::Message {
                    id: None,
                    role,
                    content,
                    phase: None,
                    internal_chat_message_metadata_passthrough: None,
                },
            ] if role == "user" => match content.as_slice() {
                [ContentItem::InputText { text }] => text,
                _ => return Err(ephemeral_input_error()),
            },
            _ => return Err(ephemeral_input_error()),
        };
        if text.is_empty() || text.len() > Self::MAX_EPHEMERAL_INPUT_BYTES {
            return Err(ephemeral_input_error());
        }
        let encoded = serde_json::to_vec(&items).map_err(|error| {
            ModelProviderPolicyError::new(
                "ephemeral_model_input_serialization_failed",
                format!("failed to serialize ephemeral model input: {error}"),
            )
        })?;
        let digest = format!("{:x}", Sha256::digest(encoded));
        self.ephemeral_input_sha256 = Some(ModelProviderSha256Digest::parse(digest)?);
        self.ephemeral_input = items;
        self.ephemeral_input_witness = None;
        Ok(())
    }

    /// Replaces prompt-only input and binds it to one host-owned authority.
    pub(crate) fn set_ephemeral_input_with_witness(
        &mut self,
        items: Vec<ResponseItem>,
        authority: EphemeralInputAuthorityBinding,
    ) -> std::result::Result<(), ModelProviderPolicyError> {
        if items.is_empty() {
            return Err(ephemeral_input_error());
        }
        self.set_ephemeral_input(items)?;
        self.ephemeral_input_witness = Some(EphemeralInputWitness {
            authority,
            consumed_attempt_ids: Arc::new(Mutex::new(BTreeSet::new())),
        });
        Ok(())
    }

    pub(crate) fn ephemeral_input_sha256(&self) -> Option<&ModelProviderSha256Digest> {
        self.ephemeral_input_sha256.as_ref()
    }

    /// Consumes one exact-attempt witness and binds it to the final logical request.
    pub(crate) fn consume_ephemeral_input_witness(
        &self,
        thread_id: &str,
        turn_id: &str,
        attempt_id: &str,
        logical_request_sha256: &ModelProviderSha256Digest,
    ) -> std::result::Result<Option<ModelProviderSha256Digest>, ModelProviderPolicyError> {
        let Some(ephemeral_input_sha256) = self.ephemeral_input_sha256.as_ref() else {
            if self.ephemeral_input_witness.is_some() {
                return Err(ModelProviderPolicyError::new(
                    "ephemeral_model_input_witness_orphaned",
                    "prompt-only authority exists without prompt-only input",
                ));
            }
            return Ok(None);
        };
        let Some(witness) = self.ephemeral_input_witness.as_ref() else {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_missing",
                "prompt-only model input requires a host-minted witness",
            ));
        };
        if witness.authority.thread_id != thread_id || witness.authority.turn_id != turn_id {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_scope_mismatch",
                "prompt-only witness does not match the provider thread and turn",
            ));
        }
        if attempt_id.trim().is_empty() {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_attempt_invalid",
                "prompt-only witness requires a non-empty host attempt identity",
            ));
        }
        let mut consumed_attempt_ids = witness.consumed_attempt_ids.lock().map_err(|_| {
            ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_state_unavailable",
                "prompt-only witness replay state is unavailable",
            )
        })?;
        if consumed_attempt_ids.contains(attempt_id) {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_replayed",
                "prompt-only witness was already consumed for this provider attempt",
            ));
        }
        if consumed_attempt_ids.len() >= Self::MAX_EPHEMERAL_INPUT_ATTEMPTS {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_attempt_limit",
                "prompt-only witness exceeded its bounded provider-attempt budget",
            ));
        }
        consumed_attempt_ids.insert(attempt_id.to_string());
        drop(consumed_attempt_ids);

        let mut hasher = Sha256::new();
        for part in [
            b"codex:ephemeral-model-input-witness:v1".as_slice(),
            thread_id.as_bytes(),
            turn_id.as_bytes(),
            attempt_id.as_bytes(),
            witness.authority.policy_binding_sha256.as_str().as_bytes(),
            ephemeral_input_sha256.as_str().as_bytes(),
            logical_request_sha256.as_str().as_bytes(),
        ] {
            hasher.update((part.len() as u64).to_be_bytes());
            hasher.update(part);
        }
        ModelProviderSha256Digest::parse(format!("{:x}", hasher.finalize())).map(Some)
    }

    pub(crate) fn get_formatted_input_for_request(
        &self,
        use_responses_lite: bool,
    ) -> Vec<ResponseItem> {
        let mut input = self.input.clone();
        input.extend(self.ephemeral_input.clone());
        if use_responses_lite {
            strip_image_details(&mut input);
        }
        input
    }
}

/// Host-frozen authority inputs used to mint a prompt-only witness.
#[derive(Debug, Clone)]
pub(crate) struct EphemeralInputAuthorityBinding {
    thread_id: String,
    turn_id: String,
    policy_binding_sha256: ModelProviderSha256Digest,
}

impl EphemeralInputAuthorityBinding {
    pub(crate) fn new(
        thread_id: impl Into<String>,
        turn_id: impl Into<String>,
        policy_binding_sha256: ModelProviderSha256Digest,
    ) -> std::result::Result<Self, ModelProviderPolicyError> {
        let thread_id = thread_id.into();
        let turn_id = turn_id.into();
        if thread_id.trim().is_empty() || turn_id.trim().is_empty() {
            return Err(ModelProviderPolicyError::new(
                "ephemeral_model_input_witness_scope_invalid",
                "prompt-only witness requires non-empty thread and turn identities",
            ));
        }
        Ok(Self {
            thread_id,
            turn_id,
            policy_binding_sha256,
        })
    }
}

#[derive(Debug, Clone)]
struct EphemeralInputWitness {
    authority: EphemeralInputAuthorityBinding,
    consumed_attempt_ids: Arc<Mutex<BTreeSet<String>>>,
}

fn ephemeral_input_error() -> ModelProviderPolicyError {
    ModelProviderPolicyError::new(
        "ephemeral_model_input_invalid",
        "ephemeral model input must be one non-empty user text item below 1,000 bytes",
    )
}

fn strip_image_details(items: &mut [ResponseItem]) {
    for item in items {
        match item {
            ResponseItem::Message { content, .. } => {
                for content_item in content {
                    if let ContentItem::InputImage { detail, .. } = content_item {
                        *detail = None;
                    }
                }
            }
            ResponseItem::FunctionCallOutput { output, .. }
            | ResponseItem::CustomToolCallOutput { output, .. } => {
                if let Some(content) = output.content_items_mut() {
                    for content_item in content {
                        if let FunctionCallOutputContentItem::InputImage { detail, .. } =
                            content_item
                        {
                            *detail = None;
                        }
                    }
                }
            }
            ResponseItem::AdditionalTools { .. }
            | ResponseItem::Reasoning { .. }
            | ResponseItem::AgentMessage { .. }
            | ResponseItem::LocalShellCall { .. }
            | ResponseItem::FunctionCall { .. }
            | ResponseItem::ToolSearchCall { .. }
            | ResponseItem::CustomToolCall { .. }
            | ResponseItem::ToolSearchOutput { .. }
            | ResponseItem::WebSearchCall { .. }
            | ResponseItem::ImageGenerationCall { .. }
            | ResponseItem::Compaction { .. }
            | ResponseItem::CompactionTrigger { .. }
            | ResponseItem::ContextCompaction { .. }
            | ResponseItem::Other => {}
        }
    }
}

pub struct ResponseStream {
    pub(crate) rx_event: mpsc::Receiver<Result<ResponseEvent>>,
    /// Signals the mapper task that the consumer stopped polling before the
    /// provider stream reached its own terminal event.
    pub(crate) consumer_dropped: CancellationToken,
}

impl Stream for ResponseStream {
    type Item = Result<ResponseEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx_event.poll_recv(cx)
    }
}

impl Drop for ResponseStream {
    fn drop(&mut self) {
        self.consumer_dropped.cancel();
    }
}

#[cfg(test)]
#[path = "client_common_tests.rs"]
mod tests;
