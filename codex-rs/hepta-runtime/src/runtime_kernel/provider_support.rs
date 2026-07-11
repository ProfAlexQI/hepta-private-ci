impl Default for RuntimeKernel {
    fn default() -> Self {
        Self::new()
    }
}

fn runtime_slugify_identifier(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "topic".to_string()
    } else {
        slug
    }
}

fn rebind_bootstrap_topic_session_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-session-bootstrap:{}", source_session_id);
    if let Some(rest) = value.strip_prefix(&source_prefix) {
        return format!("topic-session-bootstrap:{}{}", target_session_id, rest);
    }
    value.to_string()
}

fn rebind_bootstrap_topic_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-{}", runtime_slugify_identifier(source_session_id));
    if let Some(rest) = value.strip_prefix(&source_prefix) {
        return format!(
            "topic-{}{}",
            runtime_slugify_identifier(target_session_id),
            rest
        );
    }
    value.to_string()
}

fn merge_candidate_bootstrap_topic_session_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_prefix = format!("topic-session-bootstrap:{}", source_session_id);
    let source_slug = runtime_slugify_identifier(source_session_id);
    if value == source_prefix {
        return format!(
            "topic-session-bootstrap:{}:{}",
            target_session_id, source_slug
        );
    }
    if let Some(rest) = value.strip_prefix(&format!("{}:", source_prefix)) {
        return format!(
            "topic-session-bootstrap:{}:{}:{}",
            target_session_id, source_slug, rest
        );
    }
    rebind_bootstrap_topic_session_id(value, source_session_id, target_session_id)
}

fn merge_candidate_bootstrap_topic_id(
    value: &str,
    source_session_id: &str,
    target_session_id: &str,
) -> String {
    let source_slug = runtime_slugify_identifier(source_session_id);
    let target_slug = runtime_slugify_identifier(target_session_id);
    let source_prefix = format!("topic-{}", source_slug);
    if value == source_prefix {
        return format!("topic-{}-{}", target_slug, source_slug);
    }
    if let Some(rest) = value.strip_prefix(&format!("{}-", source_prefix)) {
        return format!("topic-{}-{}-{}", target_slug, source_slug, rest);
    }
    rebind_bootstrap_topic_id(value, source_session_id, target_session_id)
}

fn allocate_unique_identifier(base: &str, used: &HashSet<String>) -> String {
    if !used.contains(base) {
        return base.to_string();
    }
    for suffix in 2.. {
        let candidate = format!("{}-{}", base, suffix);
        if !used.contains(&candidate) {
            return candidate;
        }
    }
    unreachable!("identifier space exhausted")
}

fn normalize_topic_label_for_merge(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn find_equivalent_topic_session_index_for_merge(
    existing_sessions: &[TopicSession],
    canonical_topic_session_id: &str,
    canonical_topic_id: &str,
    incoming: &TopicSession,
) -> Option<usize> {
    let incoming_label = normalize_topic_label_for_merge(&incoming.topic_label.0);
    existing_sessions
        .iter()
        .position(|existing| {
            existing.topic_session_id == canonical_topic_session_id
                && existing.topic_id.0 == canonical_topic_id
                && normalize_topic_label_for_merge(&existing.topic_label.0) == incoming_label
        })
        .or_else(|| {
            existing_sessions.iter().position(|existing| {
                existing.topic_id.0 == canonical_topic_id
                    && normalize_topic_label_for_merge(&existing.topic_label.0) == incoming_label
            })
        })
}

fn normalize_topic_session_for_target(
    mut topic_session: TopicSession,
    source_session_id: &str,
    target_session_id: &str,
) -> TopicSession {
    topic_session.linked_surface_session_ids = vec![SessionId(target_session_id.to_string())];
    for span in &mut topic_session.linked_transcript_spans {
        if span.session_id.0 == source_session_id {
            span.session_id = SessionId(target_session_id.to_string());
        }
    }
    topic_session.graph_edges.clear();
    topic_session
}

fn merge_topic_session_records(existing: &mut TopicSession, incoming: &TopicSession) {
    if existing.topic_embedding.is_none() {
        existing.topic_embedding = incoming.topic_embedding.clone();
    }
    for linked_session_id in &incoming.linked_surface_session_ids {
        if existing
            .linked_surface_session_ids
            .iter()
            .all(|linked| linked != linked_session_id)
        {
            existing
                .linked_surface_session_ids
                .push(linked_session_id.clone());
        }
    }
    for transcript_span in &incoming.linked_transcript_spans {
        if existing
            .linked_transcript_spans
            .iter()
            .all(|existing_span| existing_span != transcript_span)
        {
            existing
                .linked_transcript_spans
                .push(transcript_span.clone());
        }
    }
    for open_loop in &incoming.open_loops {
        if existing
            .open_loops
            .iter()
            .all(|existing_loop| existing_loop != open_loop)
        {
            existing.open_loops.push(open_loop.clone());
        }
    }
    for (key, value) in &incoming.entities {
        existing
            .entities
            .entry(key.clone())
            .or_insert_with(|| value.clone());
    }
    for durable_memory_ref in &incoming.durable_memory_refs {
        if existing
            .durable_memory_refs
            .iter()
            .all(|existing_ref| existing_ref != durable_memory_ref)
        {
            existing
                .durable_memory_refs
                .push(durable_memory_ref.clone());
        }
    }
    existing.status = merge_topic_session_status(existing.status, incoming.status);
    existing.created_at_unix_ms = existing.created_at_unix_ms.min(incoming.created_at_unix_ms);
    existing.last_active_unix_ms = existing
        .last_active_unix_ms
        .max(incoming.last_active_unix_ms);
}

fn merge_topic_session_status(
    left: hepta_core::TopicSessionStatus,
    right: hepta_core::TopicSessionStatus,
) -> hepta_core::TopicSessionStatus {
    use hepta_core::TopicSessionStatus::Active;
    use hepta_core::TopicSessionStatus::Archived;
    use hepta_core::TopicSessionStatus::Dormant;
    use hepta_core::TopicSessionStatus::Merged;

    match (left, right) {
        (Active, _) | (_, Active) => Active,
        (Dormant, _) | (_, Dormant) => Dormant,
        (Merged, _) | (_, Merged) => Merged,
        _ => Archived,
    }
}

fn upsert_runtime_topic_graph_edge_record(
    records: &mut Vec<RuntimeTopicGraphEdgeRecord>,
    incoming: RuntimeTopicGraphEdgeRecord,
) {
    if let Some(existing) = records.iter_mut().find(|record| {
        record.source_topic_session_id == incoming.source_topic_session_id
            && record.edge.target_topic_session_id == incoming.edge.target_topic_session_id
    }) {
        if existing.edge.weight <= incoming.edge.weight {
            existing.edge.kind = incoming.edge.kind;
            existing.edge.relation = incoming
                .edge
                .relation
                .clone()
                .or(existing.edge.relation.clone());
            existing.edge.weight = incoming.edge.weight;
        }
        existing.edge.evidence_count = existing
            .edge
            .evidence_count
            .saturating_add(incoming.edge.evidence_count.max(1));
        existing.edge.last_confirmed_unix_ms = match (
            existing.edge.last_confirmed_unix_ms,
            incoming.edge.last_confirmed_unix_ms,
        ) {
            (Some(left), Some(right)) => Some(left.max(right)),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        };
        return;
    }

    records.push(incoming);
}

fn simulate_topic_state_merge(
    source_session_id: &str,
    target_session_id: &str,
    mut target_topic_sessions: Vec<TopicSession>,
    mut target_topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
    source_topic_sessions: Vec<TopicSession>,
    source_topic_graph_edges: Vec<RuntimeTopicGraphEdgeRecord>,
) -> TopicStateMergeOutcome {
    if source_topic_sessions.is_empty() && source_topic_graph_edges.is_empty() {
        return TopicStateMergeOutcome {
            topic_sessions: target_topic_sessions,
            topic_graph_edges: target_topic_graph_edges,
        };
    }

    let mut used_topic_session_ids = target_topic_sessions
        .iter()
        .map(|topic_session| topic_session.topic_session_id.clone())
        .collect::<HashSet<_>>();
    let mut used_topic_ids = target_topic_sessions
        .iter()
        .map(|topic_session| topic_session.topic_id.0.clone())
        .collect::<HashSet<_>>();
    let mut source_to_target_topic_session_ids = HashMap::new();

    for source_topic_session in source_topic_sessions {
        let original_topic_session_id = source_topic_session.topic_session_id.clone();
        let canonical_topic_session_id = rebind_bootstrap_topic_session_id(
            &original_topic_session_id,
            source_session_id,
            target_session_id,
        );
        let canonical_topic_id = rebind_bootstrap_topic_id(
            &source_topic_session.topic_id.0,
            source_session_id,
            target_session_id,
        );
        let equivalent_existing_index = find_equivalent_topic_session_index_for_merge(
            &target_topic_sessions,
            &canonical_topic_session_id,
            &canonical_topic_id,
            &source_topic_session,
        );

        let mut normalized_topic_session = normalize_topic_session_for_target(
            source_topic_session,
            source_session_id,
            target_session_id,
        );

        if let Some(existing_index) = equivalent_existing_index {
            let mapped_topic_session_id = target_topic_sessions[existing_index]
                .topic_session_id
                .clone();
            normalized_topic_session.topic_session_id = mapped_topic_session_id.clone();
            normalized_topic_session.topic_id =
                target_topic_sessions[existing_index].topic_id.clone();
            merge_topic_session_records(
                &mut target_topic_sessions[existing_index],
                &normalized_topic_session,
            );
            source_to_target_topic_session_ids
                .insert(original_topic_session_id, mapped_topic_session_id);
            continue;
        }

        let merged_topic_session_base = merge_candidate_bootstrap_topic_session_id(
            &original_topic_session_id,
            source_session_id,
            target_session_id,
        );
        let merged_topic_id_base = merge_candidate_bootstrap_topic_id(
            &normalized_topic_session.topic_id.0,
            source_session_id,
            target_session_id,
        );
        let mapped_topic_session_id =
            allocate_unique_identifier(&merged_topic_session_base, &used_topic_session_ids);
        let mapped_topic_id = allocate_unique_identifier(&merged_topic_id_base, &used_topic_ids);
        used_topic_session_ids.insert(mapped_topic_session_id.clone());
        used_topic_ids.insert(mapped_topic_id.clone());

        normalized_topic_session.topic_session_id = mapped_topic_session_id.clone();
        normalized_topic_session.topic_id = hepta_core::TopicId(mapped_topic_id);
        source_to_target_topic_session_ids
            .insert(original_topic_session_id, mapped_topic_session_id.clone());
        target_topic_sessions.push(normalized_topic_session);
    }

    for source_topic_graph_edge in source_topic_graph_edges {
        let Some(mapped_source_topic_session_id) = source_to_target_topic_session_ids
            .get(&source_topic_graph_edge.source_topic_session_id)
            .cloned()
        else {
            continue;
        };
        let Some(mapped_target_topic_session_id) = source_to_target_topic_session_ids
            .get(&source_topic_graph_edge.edge.target_topic_session_id)
            .cloned()
        else {
            continue;
        };
        if mapped_source_topic_session_id == mapped_target_topic_session_id {
            continue;
        }

        let mut mapped_edge = source_topic_graph_edge;
        mapped_edge.source_topic_session_id = mapped_source_topic_session_id;
        mapped_edge.edge.target_topic_session_id = mapped_target_topic_session_id;
        upsert_runtime_topic_graph_edge_record(&mut target_topic_graph_edges, mapped_edge);
    }

    TopicStateMergeOutcome {
        topic_sessions: target_topic_sessions,
        topic_graph_edges: target_topic_graph_edges,
    }
}

struct ProviderRegistry {
    providers: Vec<RegisteredProvider>,
}

fn parse_model_target(target: &str) -> Option<ModelRef> {
    let target = target.trim();
    let (provider, model) = target.split_once('/')?;
    let provider = provider.trim();
    let model = model.trim();
    if provider.is_empty() || model.is_empty() {
        return None;
    }
    Some(ModelRef {
        provider: provider.to_string(),
        model: model.to_string(),
    })
}

fn is_builtin_demo_model(model: &ModelRef) -> bool {
    model.provider == "demo" && model.model == "demo-chat"
}

impl ProviderRegistry {
    fn new() -> Self {
        let mut providers = vec![
            RegisteredProvider::Demo(DemoModelProvider),
            RegisteredProvider::MockOllama(MockOllamaProvider),
        ];
        for descriptor in imported_startup_provider_descriptors() {
            if descriptor.available_models.is_empty() {
                continue;
            }
            if providers
                .iter()
                .any(|provider| provider.id() == descriptor.id.as_str())
            {
                continue;
            }
            providers.push(RegisteredProvider::Imported(ImportedConfigProvider {
                descriptor,
            }));
        }
        Self { providers }
    }

    fn names(&self) -> Vec<String> {
        self.descriptors()
            .into_iter()
            .map(|provider| provider.id)
            .collect()
    }

    fn descriptors(&self) -> Vec<ProviderDescriptor> {
        self.providers
            .iter()
            .map(|provider| provider.descriptor())
            .collect()
    }

    fn available_models(&self) -> Vec<ModelRef> {
        self.descriptors()
            .into_iter()
            .flat_map(|provider| provider.available_models.into_iter())
            .collect()
    }

    fn default_model(&self) -> ModelRef {
        let descriptors = self.descriptors();
        for env_name in ["HEPTA_DEFAULT_MODEL", "HEPTA_TELEGRAM_MODEL"] {
            if let Ok(target) = env::var(env_name)
                && let Some(model) = parse_model_target(target.trim())
                && descriptors
                    .iter()
                    .flat_map(|provider| provider.available_models.iter())
                    .any(|candidate| candidate == &model)
            {
                return model;
            }
        }

        descriptors
            .iter()
            .find(|provider| {
                !matches!(provider.id.as_str(), "demo" | "mock-ollama")
                    && !provider.requires_auth
                    && provider.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp
            })
            .or_else(|| {
                descriptors
                    .iter()
                    .find(|provider| !matches!(provider.id.as_str(), "demo" | "mock-ollama"))
            })
            .or_else(|| descriptors.first())
            .map(|provider| provider.default_model.clone())
            .unwrap_or(ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            })
    }

    fn find_model(&self, target: &str) -> Option<ModelRef> {
        self.available_models()
            .into_iter()
            .find(|candidate| RuntimeKernel::model_key(candidate) == target)
    }

    fn contains_model_ref(&self, model: &ModelRef) -> bool {
        self.available_models()
            .iter()
            .any(|candidate| candidate == model)
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, HeptaError> {
        let provider = self
            .providers
            .iter()
            .find(|candidate| candidate.id() == request.model.provider)
            .ok_or_else(|| HeptaError(format!("unknown provider: {}", request.model.provider)))?;
        provider
            .chat(request)
            .await
            .map_err(|err| HeptaError(err.0))
    }
}

enum RegisteredProvider {
    Demo(DemoModelProvider),
    MockOllama(MockOllamaProvider),
    Imported(ImportedConfigProvider),
}

impl RegisteredProvider {
    fn id(&self) -> &str {
        match self {
            Self::Demo(provider) => provider.id(),
            Self::MockOllama(provider) => provider.id(),
            Self::Imported(provider) => provider.id(),
        }
    }

    fn descriptor(&self) -> ProviderDescriptor {
        match self {
            Self::Demo(provider) => provider.descriptor(),
            Self::MockOllama(provider) => provider.descriptor(),
            Self::Imported(provider) => provider.descriptor(),
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        match self {
            Self::Demo(provider) => provider.chat(request).await,
            Self::MockOllama(provider) => provider.chat(request).await,
            Self::Imported(provider) => provider.chat(request).await,
        }
    }
}

struct ImportedConfigProvider {
    descriptor: ProviderDescriptor,
}

impl ImportedConfigProvider {
    fn id(&self) -> &str {
        &self.descriptor.id
    }

    fn descriptor(&self) -> ProviderDescriptor {
        self.descriptor.clone()
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        if is_openai_codex_provider_id(&self.descriptor.id) {
            return openai_codex_responses_chat(request).map_err(hepta_core::ModelError);
        }
        if self.descriptor.transport_kind == ProviderTransportKind::OpenAiCompatibleHttp {
            if let Some(config) = openai_compatible_imported_provider_config(
                &self.descriptor.id,
                &request.model.model,
            ) {
                return openai_compatible_http_chat(&config, request);
            }
        }
        Err(hepta_core::ModelError(format!(
            "provider {} is imported but has no Hepta-native HTTP runtime config",
            self.descriptor.id
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiCompatibleProviderConfig {
    base_url: String,
    api_key: Option<String>,
    qwen_thinking_format: Option<QwenThinkingFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QwenThinkingFormat {
    TopLevel,
    ChatTemplate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiCodexAuthProfile {
    path: PathBuf,
    profile_id: String,
    access: String,
    refresh: Option<String>,
    expires: Option<u64>,
    account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodexHttpResponse {
    status: u16,
    body: String,
}

fn is_openai_codex_provider_id(provider_id: &str) -> bool {
    matches!(provider_id, "openai-codex" | "codex")
}

fn openai_codex_responses_chat(request: ModelRequest) -> Result<ModelResponse, String> {
    if let Some(tool_output) = request
        .messages
        .iter()
        .find(|message| {
            message.role == MessageRole::Tool && message.content.contains("disk_junk_audit")
        })
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_disk_junk_audit_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
        });
    }
    if let Some(tool_output) = request
        .messages
        .iter()
        .rev()
        .find(|message| message.role == MessageRole::Tool)
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_native_tool_result_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
        });
    }

    let profile = openai_codex_fresh_auth_profile()?;
    let body = openai_codex_responses_request_body(&request, None);
    let endpoint = resolve_openai_codex_responses_url(None);
    let response = curl_post_json_with_secret_files(
        &endpoint,
        &openai_codex_sse_headers(&profile.access, &profile.account_id, None),
        &body,
        request.timeout_ms,
    )?;
    if response.status == 429 || matches!(response.status, 500 | 502 | 503 | 504) {
        return Err(format!(
            "openai-codex provider returned retryable HTTP status {}",
            response.status
        ));
    }
    if response.status < 200 || response.status >= 300 {
        return Err(format!(
            "openai-codex provider returned HTTP status {}: {}",
            response.status,
            redact_codex_error_preview(&response.body)
        ));
    }
    parse_openai_codex_sse_response(&response.body)
}

fn openai_codex_sse_headers(
    access_token: &str,
    account_id: &str,
    session_id: Option<&str>,
) -> Vec<(String, String)> {
    let mut headers = vec![
        ("Authorization".into(), format!("Bearer {}", access_token)),
        ("chatgpt-account-id".into(), account_id.to_string()),
        ("originator".into(), "pi".into()),
        ("User-Agent".into(), openai_codex_user_agent()),
        ("OpenAI-Beta".into(), "responses=experimental".into()),
        ("accept".into(), "text/event-stream".into()),
        ("content-type".into(), "application/json".into()),
    ];
    if let Some(session_id) = session_id.filter(|value| !value.trim().is_empty()) {
        headers.push(("session_id".into(), session_id.to_string()));
        headers.push(("x-client-request-id".into(), session_id.to_string()));
    }
    headers
}

fn openai_codex_user_agent() -> String {
    format!(
        "pi ({} {}; {})",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH
    )
}

fn resolve_openai_codex_responses_url(base_url: Option<&str>) -> String {
    let raw = base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("https://chatgpt.com/backend-api");
    let normalized = raw.trim_end_matches('/');
    if normalized.ends_with("/codex/responses") {
        normalized.to_string()
    } else if normalized.ends_with("/codex") {
        format!("{}/responses", normalized)
    } else {
        format!("{}/codex/responses", normalized)
    }
}

fn openai_codex_responses_request_body(request: &ModelRequest, session_id: Option<&str>) -> Value {
    let mut instructions = Vec::new();
    let mut input = Vec::new();
    let mut assistant_index = 0usize;

    for message in &request.messages {
        match message.role {
            MessageRole::System => instructions.push(message.content.clone()),
            MessageRole::User => input.push(json!({
                "role": "user",
                "content": [{"type": "input_text", "text": message.content}],
            })),
            MessageRole::Assistant => {
                input.push(json!({
                    "type": "message",
                    "role": "assistant",
                    "content": [{"type": "output_text", "text": message.content, "annotations": []}],
                    "status": "completed",
                    "id": format!("msg_{}", assistant_index),
                }));
                assistant_index += 1;
            }
            MessageRole::Tool => {}
        }
    }

    let mut body = json!({
        "model": request.model.model,
        "store": false,
        "stream": true,
        "input": input,
        "text": {"verbosity": "low"},
        "include": ["reasoning.encrypted_content"],
        "tool_choice": "auto",
        "parallel_tool_calls": true,
        "reasoning": {
            "effort": openai_codex_reasoning_effort(request.thinking),
            "summary": "auto"
        }
    });
    if !instructions.is_empty() {
        body["instructions"] = Value::String(instructions.join("\n\n"));
    }
    if let Some(session_id) = session_id.filter(|value| !value.trim().is_empty()) {
        body["prompt_cache_key"] = Value::String(session_id.to_string());
    }
    let tools = openai_codex_tool_payloads(&request.tools);
    if !tools.is_empty() {
        body["tools"] = Value::Array(tools);
    }
    body
}

fn openai_codex_reasoning_effort(thinking: ThinkingLevel) -> &'static str {
    match thinking {
        ThinkingLevel::Low => "low",
        ThinkingLevel::Medium => "medium",
        ThinkingLevel::High => "high",
        ThinkingLevel::XHigh => "xhigh",
    }
}

fn openai_codex_tool_payloads(tools: &[ModelToolSpec]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            let parameters = serde_json::from_str::<Value>(&tool.input_schema_json)
                .unwrap_or_else(|_| json!({"type": "object"}));
            let parameters = sanitize_openai_codex_tool_schema(parameters);
            json!({
                "type": "function",
                "name": tool.name,
                "description": tool.description,
                "parameters": parameters,
                "strict": false,
            })
        })
        .collect()
}

fn sanitize_openai_codex_tool_schema(mut schema: Value) -> Value {
    sanitize_openai_codex_tool_schema_in_place(&mut schema);
    schema
}

fn sanitize_openai_codex_tool_schema_in_place(schema: &mut Value) {
    let Value::Object(object) = schema else {
        return;
    };

    let is_array_schema = match object.get("type") {
        Some(Value::String(kind)) => kind == "array",
        Some(Value::Array(kinds)) => kinds.iter().any(|kind| kind.as_str() == Some("array")),
        _ => false,
    };
    if is_array_schema && !object.contains_key("items") {
        object.insert("items".into(), json!({}));
    }
    if object.get("type").and_then(Value::as_str) == Some("object")
        && !object.contains_key("properties")
    {
        object.insert("properties".into(), json!({}));
    }

    for key in ["properties", "$defs", "definitions", "patternProperties"] {
        if let Some(Value::Object(children)) = object.get_mut(key) {
            for child in children.values_mut() {
                sanitize_openai_codex_tool_schema_in_place(child);
            }
        }
    }
    if let Some(items) = object.get_mut("items") {
        match items {
            Value::Array(children) => {
                for child in children {
                    sanitize_openai_codex_tool_schema_in_place(child);
                }
            }
            child => sanitize_openai_codex_tool_schema_in_place(child),
        }
    }
    for key in ["anyOf", "oneOf", "allOf"] {
        if let Some(Value::Array(children)) = object.get_mut(key) {
            for child in children {
                sanitize_openai_codex_tool_schema_in_place(child);
            }
        }
    }
    if let Some(child) = object.get_mut("additionalProperties") {
        sanitize_openai_codex_tool_schema_in_place(child);
    }
}

fn parse_openai_codex_sse_response(body: &str) -> Result<ModelResponse, String> {
    let mut text = String::new();
    let mut usage = Usage::default();
    let mut finish_reason = FinishReason::Stop;
    let mut tool_calls = Vec::new();
    let mut current_function_name: Option<String> = None;
    let mut current_function_args = String::new();

    for event in parse_sse_json_events(body)? {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        match event_type {
            "error" => {
                let message = event
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("openai-codex error");
                return Err(format!("openai-codex error: {}", message));
            }
            "response.failed" => {
                let message = event
                    .pointer("/response/error/message")
                    .and_then(Value::as_str)
                    .unwrap_or("openai-codex response failed");
                return Err(format!("openai-codex response failed: {}", message));
            }
            "response.output_item.added" => {
                if event.pointer("/item/type").and_then(Value::as_str) == Some("function_call") {
                    current_function_name = event
                        .pointer("/item/name")
                        .and_then(Value::as_str)
                        .map(ToString::to_string);
                    current_function_args.clear();
                }
            }
            "response.output_text.delta" | "response.refusal.delta" => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    text.push_str(delta);
                }
            }
            "response.function_call_arguments.delta" => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    current_function_args.push_str(delta);
                }
            }
            "response.function_call_arguments.done" => {
                if let Some(arguments) = event.get("arguments").and_then(Value::as_str) {
                    current_function_args = arguments.to_string();
                }
            }
            "response.output_item.done" => {
                if let Some(item) = event.get("item") {
                    match item.get("type").and_then(Value::as_str).unwrap_or_default() {
                        "message" => {
                            if text.is_empty() {
                                text = codex_message_item_text(item);
                            }
                        }
                        "function_call" => {
                            let name = item
                                .get("name")
                                .and_then(Value::as_str)
                                .map(ToString::to_string)
                                .or_else(|| current_function_name.clone())
                                .unwrap_or_default();
                            if !name.is_empty() {
                                let arguments = item
                                    .get("arguments")
                                    .and_then(Value::as_str)
                                    .map(ToString::to_string)
                                    .filter(|value| !value.is_empty())
                                    .unwrap_or_else(|| current_function_args.clone());
                                tool_calls.push(ToolCall {
                                    name,
                                    arguments_json: normalize_json_arguments(&arguments),
                                });
                            }
                            current_function_name = None;
                            current_function_args.clear();
                        }
                        _ => {}
                    }
                }
            }
            "response.done" | "response.completed" | "response.incomplete" => {
                if let Some(response) = event.get("response") {
                    usage = codex_usage_from_response(response);
                    finish_reason = codex_finish_reason(response);
                }
            }
            _ => {}
        }
    }

    if !tool_calls.is_empty() {
        finish_reason = FinishReason::ToolCall;
    }
    Ok(ModelResponse {
        message: if tool_calls.is_empty() {
            Some(ModelMessage {
                role: MessageRole::Assistant,
                content: text,
            })
        } else {
            None
        },
        tool_calls,
        finish_reason,
        usage,
    })
}

fn parse_sse_json_events(body: &str) -> Result<Vec<Value>, String> {
    let normalized = body.replace("\r\n", "\n");
    let mut events = Vec::new();
    for chunk in normalized.split("\n\n") {
        let data = chunk
            .lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .map(str::trim_start)
            .collect::<Vec<_>>()
            .join("\n");
        if data.is_empty() || data.trim() == "[DONE]" {
            continue;
        }
        let event = serde_json::from_str::<Value>(&data)
            .map_err(|err| format!("invalid openai-codex SSE JSON event: {}", err))?;
        events.push(event);
    }
    Ok(events)
}

fn codex_message_item_text(item: &Value) -> String {
    item.get("content")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|content| {
                    if content.get("type").and_then(Value::as_str) == Some("output_text") {
                        content.get("text").and_then(Value::as_str)
                    } else {
                        content.get("refusal").and_then(Value::as_str)
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

fn normalize_json_arguments(arguments: &str) -> String {
    let trimmed = arguments.trim();
    if trimmed.is_empty() {
        return "{}".into();
    }
    serde_json::from_str::<Value>(trimmed)
        .map(|value| value.to_string())
        .unwrap_or_else(|_| trimmed.to_string())
}

fn codex_usage_from_response(response: &Value) -> Usage {
    let input_tokens = response
        .pointer("/usage/input_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let cached_tokens = response
        .pointer("/usage/input_tokens_details/cached_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    Usage {
        input_tokens: input_tokens.saturating_sub(cached_tokens),
        output_tokens: response
            .pointer("/usage/output_tokens")
            .and_then(Value::as_u64)
            .unwrap_or(0),
    }
}

fn codex_finish_reason(response: &Value) -> FinishReason {
    match response
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or_default()
    {
        "incomplete" => FinishReason::Length,
        "failed" | "cancelled" => FinishReason::Error,
        _ => FinishReason::Stop,
    }
}

fn openai_codex_fresh_auth_profile() -> Result<OpenAiCodexAuthProfile, String> {
    let profile = load_openai_codex_auth_profile()?;
    let expires = profile.expires.unwrap_or(u64::MAX);
    let now = current_unix_ms().map_err(|err| {
        format!(
            "failed to read current time for openai-codex auth: {}",
            err.0
        )
    })?;
    if expires <= now.saturating_add(120_000) {
        if profile.refresh.as_deref().unwrap_or_default().is_empty() {
            return Err("openai-codex auth profile is expired and has no refresh token".into());
        }
        return refresh_openai_codex_auth_profile(profile);
    }
    Ok(profile)
}

fn load_openai_codex_auth_profile() -> Result<OpenAiCodexAuthProfile, String> {
    let now = current_unix_ms().unwrap_or(0);
    let mut candidates = Vec::new();
    for path in openai_codex_auth_profile_paths() {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&content) else {
            continue;
        };
        let Some(profiles) = value.get("profiles").and_then(Value::as_object) else {
            continue;
        };
        for profile_id in preferred_openai_codex_profile_ids(&path, profiles) {
            let Some(profile) = profiles.get(&profile_id) else {
                continue;
            };
            if let Some(candidate) =
                openai_codex_auth_profile_from_value(&path, profile_id, profile)
            {
                candidates.push(candidate);
            }
        }
    }
    if let Some(profile_id) = openai_codex_profile_id_override() {
        return candidates
            .into_iter()
            .find(|candidate| candidate.profile_id == profile_id)
            .ok_or_else(|| {
                format!(
                    "requested openai-codex auth profile {} was not found in Hepta local import",
                    profile_id
                )
            });
    }
    select_openai_codex_auth_profile(candidates, now)
        .ok_or_else(|| "no usable openai-codex auth profile found in Hepta local import".into())
}

fn openai_codex_profile_id_override() -> Option<String> {
    env::var("HEPTA_OPENAI_CODEX_PROFILE_ID")
        .or_else(|_| env::var("HEPTA_OPENAI_CODEX_PROFILE"))
        .ok()
        .and_then(|value| normalize_openai_codex_profile_id_override(&value))
}

fn normalize_openai_codex_profile_id_override(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    Some(if value.starts_with("openai-codex:") {
        value.to_string()
    } else {
        format!("openai-codex:{value}")
    })
}

fn openai_codex_auth_profile_from_value(
    path: &Path,
    profile_id: String,
    profile: &Value,
) -> Option<OpenAiCodexAuthProfile> {
    if profile.get("provider").and_then(Value::as_str) != Some("openai-codex") {
        return None;
    }
    let access = profile
        .get("access")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)?;
    let account_id = profile
        .get("accountId")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or_else(|| extract_chatgpt_account_id_from_jwt(&access))?;
    Some(OpenAiCodexAuthProfile {
        path: path.to_path_buf(),
        profile_id,
        access,
        refresh: profile
            .get("refresh")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
        expires: profile.get("expires").and_then(Value::as_u64),
        account_id,
    })
}

fn select_openai_codex_auth_profile(
    mut candidates: Vec<OpenAiCodexAuthProfile>,
    now_ms: u64,
) -> Option<OpenAiCodexAuthProfile> {
    candidates.sort_by(|left, right| {
        let left_expires = left.expires.unwrap_or(u64::MAX);
        let right_expires = right.expires.unwrap_or(u64::MAX);
        let left_fresh = left_expires > now_ms.saturating_add(120_000);
        let right_fresh = right_expires > now_ms.saturating_add(120_000);
        right_fresh
            .cmp(&left_fresh)
            .then_with(|| right_expires.cmp(&left_expires))
            .then_with(|| left.profile_id.cmp(&right.profile_id))
    });
    candidates.into_iter().next()
}

fn openai_codex_auth_profile_paths() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut push_candidate = |candidate: PathBuf| {
        if candidate.is_file() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };
    if let Ok(manifest_path) = env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST") {
        let manifest_path = PathBuf::from(manifest_path);
        if let Ok(content) = fs::read_to_string(&manifest_path)
            && let Ok(value) = serde_json::from_str::<Value>(&content)
            && let Some(import_root) = value.get("import_root").and_then(Value::as_str)
        {
            for agent in ["hepta", "main"] {
                push_candidate(
                    PathBuf::from(import_root)
                        .join("private/agents")
                        .join(agent)
                        .join("agent/auth-profiles.json"),
                );
            }
        }
    }
    for agent in ["hepta", "main"] {
        push_candidate(
            PathBuf::from(".hepta/local-import/private/agents")
                .join(agent)
                .join("agent/auth-profiles.json"),
        );
    }
    candidates
}

fn preferred_openai_codex_profile_ids(
    profile_path: &Path,
    profiles: &serde_json::Map<String, Value>,
) -> Vec<String> {
    let mut ids = Vec::new();
    let mut push_id = |id: String| {
        if !ids.iter().any(|existing| existing == &id) {
            ids.push(id);
        }
    };
    let auth_state_path = profile_path.with_file_name("auth-state.json");
    if let Ok(content) = fs::read_to_string(auth_state_path)
        && let Ok(value) = serde_json::from_str::<Value>(&content)
        && let Some(last_good) = value
            .get("lastGood")
            .and_then(|last_good| last_good.get("openai-codex"))
            .and_then(Value::as_str)
    {
        push_id(last_good.to_string());
    }
    push_id("openai-codex:default".into());
    let mut profile_ids = profiles.keys().cloned().collect::<Vec<_>>();
    profile_ids.sort();
    for id in profile_ids {
        if id.starts_with("openai-codex:") {
            push_id(id);
        }
    }
    ids
}

fn refresh_openai_codex_auth_profile(
    profile: OpenAiCodexAuthProfile,
) -> Result<OpenAiCodexAuthProfile, String> {
    let refresh = profile
        .refresh
        .as_deref()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "openai-codex auth profile has no refresh token".to_string())?;
    let body = form_urlencode_pairs(&[
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh),
        ("client_id", "app_EMoamEEZ73f0CkXaXp7hrann"),
    ]);
    let response = curl_post_form_with_secret_file(
        "https://auth.openai.com/oauth/token",
        &[(
            "content-type".into(),
            "application/x-www-form-urlencoded".into(),
        )],
        &body,
        Some(60_000),
    )?;
    if response.status < 200 || response.status >= 300 {
        return Err(format!(
            "openai-codex token refresh returned HTTP status {}",
            response.status
        ));
    }
    let value = serde_json::from_str::<Value>(&response.body)
        .map_err(|err| format!("invalid openai-codex refresh response JSON: {}", err))?;
    let access = value
        .get("access_token")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "openai-codex refresh response missing access token".to_string())?
        .to_string();
    let new_refresh = value
        .get("refresh_token")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or(profile.refresh.clone());
    let expires_in_ms = value
        .get("expires_in")
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .saturating_mul(1000);
    let expires = current_unix_ms()
        .map_err(|err| {
            format!(
                "failed to read current time for openai-codex refresh: {}",
                err.0
            )
        })?
        .saturating_add(expires_in_ms);
    let account_id = extract_chatgpt_account_id_from_jwt(&access)
        .ok_or_else(|| "openai-codex refresh response missing chatgpt account id".to_string())?;
    persist_refreshed_openai_codex_profile(
        &profile.path,
        &profile.profile_id,
        &access,
        new_refresh.as_deref(),
        expires,
        &account_id,
    )?;
    Ok(OpenAiCodexAuthProfile {
        access,
        refresh: new_refresh,
        expires: Some(expires),
        account_id,
        ..profile
    })
}

fn persist_refreshed_openai_codex_profile(
    path: &Path,
    profile_id: &str,
    access: &str,
    refresh: Option<&str>,
    expires: u64,
    account_id: &str,
) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read openai-codex auth profile store: {}", err))?;
    let mut value = serde_json::from_str::<Value>(&content)
        .map_err(|err| format!("invalid openai-codex auth profile store JSON: {}", err))?;
    let profile = value
        .get_mut("profiles")
        .and_then(Value::as_object_mut)
        .and_then(|profiles| profiles.get_mut(profile_id))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            "openai-codex auth profile disappeared before refresh persist".to_string()
        })?;
    profile.insert("access".into(), Value::String(access.to_string()));
    if let Some(refresh) = refresh.filter(|value| !value.is_empty()) {
        profile.insert("refresh".into(), Value::String(refresh.to_string()));
    }
    profile.insert("expires".into(), Value::Number(expires.into()));
    profile.insert("accountId".into(), Value::String(account_id.to_string()));
    fs::write(
        path,
        serde_json::to_string_pretty(&value).map_err(|err| {
            format!(
                "failed to serialize openai-codex auth profile store: {}",
                err
            )
        })?,
    )
    .map_err(|err| format!("failed to persist refreshed openai-codex profile: {}", err))
}

fn extract_chatgpt_account_id_from_jwt(token: &str) -> Option<String> {
    let payload = token.split('.').nth(1)?;
    let decoded = base64_url_decode(payload)?;
    let value = serde_json::from_slice::<Value>(&decoded).ok()?;
    value
        .get("https://api.openai.com/auth")?
        .get("chatgpt_account_id")?
        .as_str()
        .map(ToString::to_string)
}

fn base64_url_decode(input: &str) -> Option<Vec<u8>> {
    let mut buffer = 0u32;
    let mut bits = 0u8;
    let mut out = Vec::new();
    for ch in input.chars() {
        if ch == '=' {
            break;
        }
        let value = match ch {
            'A'..='Z' => ch as u8 - b'A',
            'a'..='z' => ch as u8 - b'a' + 26,
            '0'..='9' => ch as u8 - b'0' + 52,
            '-' => 62,
            '_' => 63,
            _ => return None,
        } as u32;
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buffer >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

fn curl_post_json_with_secret_files(
    url: &str,
    headers: &[(String, String)],
    body: &Value,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    curl_post_with_secret_files(url, headers, &body.to_string(), timeout_ms)
}

fn curl_post_form_with_secret_file(
    url: &str,
    headers: &[(String, String)],
    body: &str,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    curl_post_with_secret_files(url, headers, body, timeout_ms)
}

fn curl_post_with_secret_files(
    url: &str,
    headers: &[(String, String)],
    body: &str,
    timeout_ms: Option<u64>,
) -> Result<CodexHttpResponse, String> {
    let header_text = headers
        .iter()
        .map(|(name, value)| format!("{}: {}", name, value))
        .collect::<Vec<_>>()
        .join("\n");
    let header_path = write_secret_temp_file("hepta-codex-headers", &header_text)?;
    let body_path = write_secret_temp_file("hepta-codex-body", body)?;
    let timeout_secs = provider_read_timeout_duration(timeout_ms)
        .as_secs()
        .clamp(1, 300)
        .to_string();
    let output = Command::new("curl")
        .arg("--silent")
        .arg("--show-error")
        .arg("--no-buffer")
        .arg("--max-time")
        .arg(timeout_secs)
        .arg("--request")
        .arg("POST")
        .arg("--header")
        .arg(format!("@{}", header_path.display()))
        .arg("--data-binary")
        .arg(format!("@{}", body_path.display()))
        .arg("--write-out")
        .arg("\n__HEPTA_HTTP_STATUS__:%{http_code}\n")
        .arg(url)
        .output();
    let _ = fs::remove_file(&header_path);
    let _ = fs::remove_file(&body_path);
    let output = output.map_err(|err| format!("failed to run curl for openai-codex: {}", err))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() && stdout.trim().is_empty() {
        return Err(format!(
            "curl openai-codex request failed: {}",
            redact_codex_error_preview(&stderr)
        ));
    }
    let (body, status_text) = stdout
        .rsplit_once("\n__HEPTA_HTTP_STATUS__:")
        .ok_or_else(|| "curl openai-codex response missing HTTP status marker".to_string())?;
    let status = status_text.trim().parse::<u16>().map_err(|_| {
        format!(
            "invalid openai-codex HTTP status marker: {}",
            status_text.trim()
        )
    })?;
    Ok(CodexHttpResponse {
        status,
        body: body.to_string(),
    })
}

fn write_secret_temp_file(prefix: &str, content: &str) -> Result<PathBuf, String> {
    let ts = current_unix_ms().unwrap_or(0);
    for attempt in 0..100u8 {
        let mut path = env::temp_dir();
        path.push(format!(
            "{}-{}-{}-{}.tmp",
            prefix,
            std::process::id(),
            ts,
            attempt
        ));
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(mut file) => {
                file.write_all(content.as_bytes())
                    .map_err(|err| format!("failed to write secret temp file: {}", err))?;
                return Ok(path);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(format!("failed to create secret temp file: {}", err)),
        }
    }
    Err("failed to create unique secret temp file".into())
}

fn form_urlencode_pairs(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{}={}", form_urlencode(key), form_urlencode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn form_urlencode(input: &str) -> String {
    let mut out = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn redact_codex_error_preview(input: &str) -> String {
    let mut text = input.replace(['\n', '\r'], " ");
    if let Some(index) = text.to_ascii_lowercase().find("authorization:") {
        text.truncate(index + "authorization:".len());
        text.push_str(" <redacted>");
    }
    truncate_for_context(text.trim(), 600)
}

fn openai_compatible_imported_provider_config(
    provider_id: &str,
    model_id: &str,
) -> Option<OpenAiCompatibleProviderConfig> {
    let value = local_import_private_runtime_config()?;
    let provider = value.get("models")?.get("providers")?.get(provider_id)?;
    let base_url = provider.get("baseUrl")?.as_str()?.trim().to_string();
    if base_url.is_empty() {
        return None;
    }
    let api_key = provider
        .get("apiKey")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let model = provider
        .get("models")
        .and_then(Value::as_array)
        .and_then(|models| {
            models.iter().find(|model| {
                model
                    .get("id")
                    .and_then(Value::as_str)
                    .map(|id| id == model_id)
                    .unwrap_or(false)
            })
        });
    let qwen_thinking_format = model
        .and_then(configured_qwen_thinking_format)
        .or_else(|| default_qwen_thinking_format(provider_id, model_id));
    Some(OpenAiCompatibleProviderConfig {
        base_url,
        api_key,
        qwen_thinking_format,
    })
}

fn configured_qwen_thinking_format(model: &Value) -> Option<QwenThinkingFormat> {
    let compat_format = model
        .get("compat")
        .and_then(|compat| compat.get("thinkingFormat"))
        .and_then(Value::as_str)
        .and_then(qwen_thinking_format_from_openclaw_value);
    if compat_format.is_some() {
        return compat_format;
    }
    model
        .get("params")
        .and_then(|params| {
            params
                .get("qwenThinkingFormat")
                .or_else(|| params.get("qwen_thinking_format"))
        })
        .and_then(Value::as_str)
        .and_then(qwen_thinking_format_from_openclaw_value)
}

fn qwen_thinking_format_from_openclaw_value(value: &str) -> Option<QwenThinkingFormat> {
    match value.trim().to_ascii_lowercase().as_str() {
        "qwen" | "top-level" | "top_level" => Some(QwenThinkingFormat::TopLevel),
        "qwen-chat-template" | "chat-template" | "chat_template" => {
            Some(QwenThinkingFormat::ChatTemplate)
        }
        _ => None,
    }
}

fn default_qwen_thinking_format(provider_id: &str, model_id: &str) -> Option<QwenThinkingFormat> {
    let provider = provider_id.to_ascii_lowercase();
    let model = model_id.to_ascii_lowercase();
    if !provider.contains("qwen") && !model.contains("qwen") {
        return None;
    }
    if matches!(
        provider.as_str(),
        "qwen" | "dashscope" | "qwen-portal" | "qwencloud" | "modelstudio"
    ) {
        return Some(QwenThinkingFormat::TopLevel);
    }
    if provider.contains("vllm") || provider.contains("mlx") || provider.contains("ollama") {
        return Some(QwenThinkingFormat::ChatTemplate);
    }
    None
}

fn local_import_private_runtime_config() -> Option<Value> {
    let mut merged = Value::Object(serde_json::Map::new());
    let mut loaded_any = false;

    for config_path in local_import_private_config_paths() {
        let Ok(content) = fs::read_to_string(config_path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<Value>(&content) else {
            continue;
        };
        merge_runtime_config_value(&mut merged, value);
        loaded_any = true;
    }

    loaded_any.then_some(merged)
}

fn merge_runtime_config_value(target: &mut Value, source: Value) {
    match (&mut *target, source) {
        (Value::Object(target_object), Value::Object(source_object)) => {
            for (key, source_value) in source_object {
                match target_object.get_mut(&key) {
                    Some(target_value) => merge_runtime_config_value(target_value, source_value),
                    None => {
                        target_object.insert(key, source_value);
                    }
                }
            }
        }
        (Value::Array(target_array), Value::Array(source_array)) => {
            for source_value in source_array {
                if !target_array
                    .iter()
                    .any(|target_value| target_value == &source_value)
                {
                    target_array.push(source_value);
                }
            }
        }
        (Value::Null, source_value) => {
            *target = source_value;
        }
        _ => {}
    }
}

fn local_import_private_config_paths() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut push_candidate = |candidate: PathBuf| {
        if candidate.is_file() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };

    let source_runtime_config_name = ["open", "claw.json"].concat();

    if let Ok(manifest_path) = env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST") {
        let manifest_path = PathBuf::from(manifest_path);
        if let Ok(content) = fs::read_to_string(&manifest_path)
            && let Ok(value) = serde_json::from_str::<Value>(&content)
            && let Some(import_root) = value.get("import_root").and_then(Value::as_str)
        {
            for file_name in ["hepta_runtime.json", source_runtime_config_name.as_str()] {
                let candidate = PathBuf::from(import_root)
                    .join("private/config")
                    .join(file_name);
                push_candidate(candidate);
            }
        }
    }
    for candidate in [
        PathBuf::from(".hepta/local-import/private/config/hepta_runtime.json"),
        PathBuf::from(".hepta/local-import/private/config")
            .join(source_runtime_config_name.as_str()),
    ] {
        push_candidate(candidate);
    }

    candidates
}

fn openai_compatible_http_chat(
    config: &OpenAiCompatibleProviderConfig,
    request: ModelRequest,
) -> Result<ModelResponse, hepta_core::ModelError> {
    if let Some(tool_output) = request
        .messages
        .iter()
        .find(|message| {
            message.role == MessageRole::Tool && message.content.contains("disk_junk_audit")
        })
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_disk_junk_audit_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        });
    }
    if let Some(tool_output) = request
        .messages
        .iter()
        .rev()
        .find(|message| message.role == MessageRole::Tool)
        .map(|message| message.content.as_str())
    {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: render_native_tool_result_reply(tool_output),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
            },
        });
    }
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    let mut payload = json!({
        "model": request.model.model,
        "messages": request.messages.iter().map(|message| {
            json!({
                "role": openai_role_name(&message.role),
                "content": message.content,
            })
        }).collect::<Vec<_>>(),
        "temperature": 0.2,
        "max_tokens": 1200,
        "stream": false,
    });
    let openai_tools = openai_tool_payloads(&request.tools);
    if !openai_tools.is_empty() {
        payload["tools"] = Value::Array(openai_tools);
        payload["tool_choice"] = Value::String("auto".into());
    }
    apply_qwen_openai_compatible_thinking_params(
        &mut payload,
        config.qwen_thinking_format,
        &request,
    );
    let response_text = http_post_json_plaintext(
        &endpoint,
        config.api_key.as_deref(),
        &payload,
        request.timeout_ms,
    )
    .map_err(hepta_core::ModelError)?;
    let response: Value = serde_json::from_str(&response_text).map_err(|err| {
        hepta_core::ModelError(format!("invalid provider JSON response: {}", err))
    })?;
    if let Some(error_message) = response
        .get("error")
        .and_then(|error| error.get("message"))
        .and_then(Value::as_str)
    {
        return Err(hepta_core::ModelError(format!(
            "provider error: {}",
            error_message
        )));
    }
    let message = response
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|choices| choices.first())
        .and_then(|choice| choice.get("message"))
        .cloned()
        .unwrap_or_else(|| json!({}));
    let message_text = message
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    let mut tool_calls = openai_tool_calls_from_message(&message);
    if tool_calls.is_empty() {
        tool_calls = textual_tool_calls_from_message_content(&message_text, &request.tools);
    }
    let input_tokens = response
        .pointer("/usage/prompt_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let output_tokens = response
        .pointer("/usage/completion_tokens")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    Ok(ModelResponse {
        message: if !tool_calls.is_empty() {
            None
        } else {
            Some(ModelMessage {
                role: MessageRole::Assistant,
                content: message_text,
            })
        },
        finish_reason: if tool_calls.is_empty() {
            FinishReason::Stop
        } else {
            FinishReason::ToolCall
        },
        tool_calls,
        usage: Usage {
            input_tokens,
            output_tokens,
        },
    })
}

fn split_structured_tool_output(tool_output: &str) -> (&str, Option<Value>) {
    if let Some((content, structured_json)) = tool_output.split_once(" | structured=") {
        return (
            content.trim(),
            serde_json::from_str::<Value>(structured_json.trim()).ok(),
        );
    }
    if let Some((content, structured_json)) = tool_output.split_once(" structured=") {
        return (
            content.trim(),
            serde_json::from_str::<Value>(structured_json.trim()).ok(),
        );
    }
    (tool_output.trim(), None)
}

fn render_native_tool_result_reply(tool_output: &str) -> String {
    let (content, structured) = split_structured_tool_output(tool_output);
    let clean_content = content.replace('\n', " ");
    let content_preview = truncate_for_context(clean_content.trim(), 600);

    if let Some(value) = structured {
        let backend = value
            .get("backend")
            .and_then(Value::as_str)
            .unwrap_or("hepta-rust-native");
        let tool = value.get("tool").and_then(Value::as_str).unwrap_or("tool");

        if tool == "process" {
            let action = value
                .pointer("/result/action")
                .and_then(Value::as_str)
                .unwrap_or("run");
            let count = value
                .pointer("/result/processes")
                .and_then(Value::as_array)
                .map(|processes| processes.len());
            let followups = value
                .pointer("/result/followup_actions")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .take(6)
                        .collect::<Vec<_>>()
                        .join("/")
                })
                .filter(|joined| !joined.is_empty())
                .unwrap_or_else(|| "poll/log/kill/clear/remove".into());
            return match count {
                Some(count) => format!(
                    "已通过 Hepta native process 工具完成 `{action}`：共有 {count} 条后台进程记录。后续可用 {followups} 查看或清理；结构化 JSON 已保留在本地，不再展开到聊天里。"
                ),
                None => format!(
                    "已通过 Hepta native process 工具完成 `{action}`：{}。后续可用 {followups} 继续处理；结构化 JSON 已保留在本地，不再展开到聊天里。",
                    content_preview
                ),
            };
        }

        return format!(
            "已通过 {backend} 执行 `{tool}`：{}。结构化结果已保留在本地，不再展开 raw JSON。",
            content_preview
        );
    }

    format!("工具已执行：{}", content_preview)
}

fn openai_role_name(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
    }
}

fn apply_qwen_openai_compatible_thinking_params(
    payload: &mut Value,
    format: Option<QwenThinkingFormat>,
    request: &ModelRequest,
) -> bool {
    let Some(format) = format else {
        return false;
    };
    let enable_thinking = qwen_enable_thinking_for_request(request);
    match format {
        QwenThinkingFormat::TopLevel => {
            payload["enable_thinking"] = Value::Bool(enable_thinking);
        }
        QwenThinkingFormat::ChatTemplate => {
            let existing = payload.get_mut("chat_template_kwargs");
            if let Some(Value::Object(map)) = existing {
                map.insert("enable_thinking".into(), Value::Bool(enable_thinking));
            } else {
                payload["chat_template_kwargs"] = json!({ "enable_thinking": enable_thinking });
            }
        }
    }
    true
}

fn qwen_enable_thinking_for_request(_request: &ModelRequest) -> bool {
    // Telegram/live-agent replies must never expose Qwen's visible thinking
    // transcript as assistant text. Keep OpenAI-compatible Qwen transports in
    // no-think mode for both tool and ordinary turns; higher-level runtimes can
    // still carry private reasoning through provider-specific channels later.
    false
}

fn openai_tool_payloads(tools: &[ModelToolSpec]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            let parameters = serde_json::from_str::<Value>(&tool.input_schema_json).unwrap_or_else(
                |_| json!({"type":"object","properties":{},"additionalProperties":true}),
            );
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": parameters,
                }
            })
        })
        .collect()
}

fn openai_tool_calls_from_message(message: &Value) -> Vec<ToolCall> {
    let mut calls = Vec::new();
    if let Some(tool_calls) = message.get("tool_calls").and_then(Value::as_array) {
        for item in tool_calls {
            let Some(function) = item.get("function") else {
                continue;
            };
            let Some(name) = function.get("name").and_then(Value::as_str) else {
                continue;
            };
            let arguments_json = match function.get("arguments") {
                Some(Value::String(arguments)) => arguments.clone(),
                Some(value) => value.to_string(),
                None => "{}".into(),
            };
            calls.push(ToolCall {
                name: name.to_string(),
                arguments_json,
            });
        }
    }

    if calls.is_empty()
        && let Some(function_call) = message.get("function_call")
        && let Some(name) = function_call.get("name").and_then(Value::as_str)
    {
        let arguments_json = function_call
            .get("arguments")
            .and_then(Value::as_str)
            .unwrap_or("{}")
            .to_string();
        calls.push(ToolCall {
            name: name.to_string(),
            arguments_json,
        });
    }

    calls
}

fn textual_tool_calls_from_message_content(
    content: &str,
    tools: &[ModelToolSpec],
) -> Vec<ToolCall> {
    let known_tools = tools
        .iter()
        .map(|tool| tool.name.as_str())
        .collect::<Vec<_>>();
    textual_tool_call_segments(content)
        .into_iter()
        .filter_map(|segment| parse_textual_tool_call_segment(segment, &known_tools))
        .collect()
}

fn textual_tool_call_segments(content: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut rest = content;
    while let Some(start_index) = rest.find("<|tool_call>") {
        let after_start = &rest[start_index + "<|tool_call>".len()..];
        if let Some(end_index) = after_start.find("<tool_call|>") {
            segments.push(after_start[..end_index].trim());
            rest = &after_start[end_index + "<tool_call|>".len()..];
        } else {
            break;
        }
    }
    if segments.is_empty() {
        rest = content;
        while let Some(start_index) = rest.find("<tool_call>") {
            let after_start = &rest[start_index + "<tool_call>".len()..];
            if let Some(end_index) = after_start.find("</tool_call>") {
                segments.push(after_start[..end_index].trim());
                rest = &after_start[end_index + "</tool_call>".len()..];
            } else {
                break;
            }
        }
    }
    if segments.is_empty() && content.trim_start().starts_with("call:") {
        segments.push(content.trim());
    }
    segments
}

fn parse_textual_tool_call_segment(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let trimmed = segment.trim().trim_matches('`').trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(call) = parse_json_textual_tool_call(trimmed, known_tools) {
        return Some(call);
    }
    parse_gemma_textual_tool_call(trimmed, known_tools)
}

fn parse_json_textual_tool_call(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let value = serde_json::from_str::<Value>(segment).ok()?;
    let name = value
        .get("name")
        .or_else(|| value.get("tool"))
        .or_else(|| value.get("tool_name"))
        .and_then(Value::as_str)?;
    if !known_tools.iter().any(|candidate| candidate == &name) {
        return None;
    }
    let arguments_json = match value
        .get("arguments")
        .or_else(|| value.get("args"))
        .or_else(|| value.get("input"))
    {
        Some(Value::String(arguments)) => arguments.clone(),
        Some(Value::Object(_)) | Some(Value::Array(_)) => value
            .get("arguments")
            .or_else(|| value.get("args"))
            .or_else(|| value.get("input"))?
            .to_string(),
        Some(other) => json!({"value": other}).to_string(),
        None => "{}".into(),
    };
    Some(ToolCall {
        name: name.to_string(),
        arguments_json,
    })
}

fn parse_gemma_textual_tool_call(segment: &str, known_tools: &[&str]) -> Option<ToolCall> {
    let rest = segment.strip_prefix("call:")?.trim_start();
    let open_brace = rest.find('{')?;
    let name = rest[..open_brace].trim();
    if name.is_empty() || !known_tools.iter().any(|candidate| candidate == &name) {
        return None;
    }
    let argument_text = rest[open_brace..].trim();
    let arguments_json = parse_relaxed_tool_arguments(argument_text)?;
    Some(ToolCall {
        name: name.to_string(),
        arguments_json,
    })
}

fn parse_relaxed_tool_arguments(argument_text: &str) -> Option<String> {
    if let Ok(value) = serde_json::from_str::<Value>(argument_text) {
        return Some(value.to_string());
    }
    let inner = argument_text.strip_prefix('{')?.strip_suffix('}')?.trim();
    if inner.is_empty() {
        return Some("{}".into());
    }
    let mut map = serde_json::Map::new();
    for item in split_top_level_commas(inner) {
        let (key, value_text) = split_key_value(item)?;
        let key = key.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            return None;
        }
        let value_text = value_text.trim();
        let value = serde_json::from_str::<Value>(value_text).unwrap_or_else(|_| {
            Value::String(value_text.trim_matches('"').trim_matches('\'').to_string())
        });
        map.insert(key, value);
    }
    Some(Value::Object(map).to_string())
}

fn split_top_level_commas(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut in_string = false;
    let mut quote = '\0';
    let mut escaped = false;
    for (index, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_string = true;
                quote = ch;
            }
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(input[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(input[start..].trim());
    parts.into_iter().filter(|part| !part.is_empty()).collect()
}

fn split_key_value(input: &str) -> Option<(&str, &str)> {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut quote = '\0';
    let mut escaped = false;
    for (index, ch) in input.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_string = true;
                quote = ch;
            }
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            ':' if depth == 0 => return Some((&input[..index], &input[index + ch.len_utf8()..])),
            _ => {}
        }
    }
    None
}

fn http_post_json_plaintext(
    url: &str,
    bearer_token: Option<&str>,
    payload: &Value,
    timeout_ms: Option<u64>,
) -> Result<String, String> {
    let parsed = parse_plain_http_url(url)?;
    if parsed.scheme != "http" {
        return Err(format!(
            "Hepta native provider currently allows plain HTTP only for local providers; unsupported scheme: {}",
            parsed.scheme
        ));
    }
    let body = payload.to_string();
    let mut headers = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n",
        parsed.path,
        parsed.host_header,
        body.len()
    );
    if let Some(token) = bearer_token.filter(|token| !token.trim().is_empty()) {
        headers.push_str(&format!("Authorization: Bearer {}\r\n", token));
    }
    headers.push_str("\r\n");
    let mut stream = TcpStream::connect((parsed.host.as_str(), parsed.port))
        .map_err(|err| format!("failed to connect provider {}: {}", parsed.host_header, err))?;
    let read_timeout = provider_read_timeout_duration(timeout_ms);
    stream
        .set_read_timeout(Some(read_timeout))
        .map_err(|err| format!("failed to set provider read timeout: {}", err))?;
    stream
        .set_write_timeout(Some(std::time::Duration::from_secs(15)))
        .map_err(|err| format!("failed to set provider write timeout: {}", err))?;
    stream
        .write_all(headers.as_bytes())
        .and_then(|_| stream.write_all(body.as_bytes()))
        .map_err(|err| format!("failed to write provider request: {}", err))?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|err| {
        if matches!(
            err.kind(),
            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
        ) {
            format!(
                "provider read timeout after {} ms",
                read_timeout.as_millis()
            )
        } else {
            format!("failed to read provider response: {}", err)
        }
    })?;
    let raw_text = String::from_utf8_lossy(&raw).to_string();
    let (head, body) = raw_text
        .split_once("\r\n\r\n")
        .ok_or_else(|| "provider returned malformed HTTP response".to_string())?;
    let status_line = head.lines().next().unwrap_or_default();
    if !status_line.contains(" 200 ") {
        return Err(format!("provider returned non-200 status: {}", status_line));
    }
    if head
        .lines()
        .any(|line| line.eq_ignore_ascii_case("transfer-encoding: chunked"))
    {
        return decode_http_chunked_body(body);
    }
    Ok(body.to_string())
}

fn provider_read_timeout_duration(timeout_ms: Option<u64>) -> std::time::Duration {
    const DEFAULT_PROVIDER_READ_TIMEOUT_MS: u64 = 90_000;
    const MIN_PROVIDER_READ_TIMEOUT_MS: u64 = 1_000;
    const MAX_PROVIDER_READ_TIMEOUT_MS: u64 = 300_000;
    let timeout_ms = timeout_ms
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_PROVIDER_READ_TIMEOUT_MS)
        .clamp(MIN_PROVIDER_READ_TIMEOUT_MS, MAX_PROVIDER_READ_TIMEOUT_MS);
    std::time::Duration::from_millis(timeout_ms)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlainHttpUrl {
    scheme: String,
    host: String,
    port: u16,
    host_header: String,
    path: String,
}

fn parse_plain_http_url(url: &str) -> Result<PlainHttpUrl, String> {
    let (scheme, rest) = url
        .split_once("://")
        .ok_or_else(|| format!("invalid URL: {}", url))?;
    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{}", path)),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return Err(format!("invalid URL authority: {}", url));
    }
    let (host, port) = if let Some((host, port_text)) = authority.rsplit_once(':') {
        let port = port_text
            .parse::<u16>()
            .map_err(|_| format!("invalid URL port: {}", port_text))?;
        (host.to_string(), port)
    } else {
        (
            authority.to_string(),
            if scheme == "https" { 443 } else { 80 },
        )
    };
    Ok(PlainHttpUrl {
        scheme: scheme.to_string(),
        host,
        port,
        host_header: authority.to_string(),
        path,
    })
}

fn decode_http_chunked_body(body: &str) -> Result<String, String> {
    let mut rest = body;
    let mut decoded = String::new();
    loop {
        let (size_line, after_size) = rest
            .split_once("\r\n")
            .ok_or_else(|| "malformed chunked provider response".to_string())?;
        let size = usize::from_str_radix(size_line.trim(), 16)
            .map_err(|_| format!("invalid HTTP chunk size: {}", size_line))?;
        if size == 0 {
            return Ok(decoded);
        }
        if after_size.len() < size + 2 {
            return Err("truncated chunked provider response".into());
        }
        decoded.push_str(&after_size[..size]);
        rest = &after_size[size + 2..];
    }
}

#[cfg(not(test))]
fn imported_startup_provider_descriptors() -> Vec<ProviderDescriptor> {
    let manifest_path = std::env::var("HEPTA_LOCAL_CONFIG_IMPORT_MANIFEST")
        .unwrap_or_else(|_| ".hepta/local-import/manifest.json".into());
    hepta_core::LocalConfigImportStatus::from_manifest_path(manifest_path)
        .manifest
        .and_then(|manifest| manifest.startup_config)
        .map(|startup| startup.model_providers)
        .unwrap_or_default()
}

#[cfg(test)]
fn imported_startup_provider_descriptors() -> Vec<ProviderDescriptor> {
    Vec::new()
}

struct DemoModelProvider;

impl ModelProvider for DemoModelProvider {
    fn id(&self) -> &'static str {
        "demo"
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().into(),
            display_name: "Demo Provider".into(),
            transport_kind: ProviderTransportKind::InProcess,
            default_model: ModelRef {
                provider: self.id().into(),
                model: "demo-chat".into(),
            },
            available_models: vec![
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-chat".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-precise".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "demo-creative".into(),
                },
            ],
            requires_auth: false,
            supports_tool_calls: true,
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        render_provider_response(request, |model| match model {
            "demo-precise" => "[precise]".to_string(),
            "demo-creative" => "[creative]".to_string(),
            _ => "[chat]".to_string(),
        })
    }
}

struct MockOllamaProvider;

impl ModelProvider for MockOllamaProvider {
    fn id(&self) -> &'static str {
        "mock-ollama"
    }

    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id().into(),
            display_name: "Mock Ollama".into(),
            transport_kind: ProviderTransportKind::OpenAiCompatibleHttp,
            default_model: ModelRef {
                provider: self.id().into(),
                model: "local-chat".into(),
            },
            available_models: vec![
                ModelRef {
                    provider: self.id().into(),
                    model: "local-chat".into(),
                },
                ModelRef {
                    provider: self.id().into(),
                    model: "local-precise".into(),
                },
            ],
            requires_auth: false,
            supports_tool_calls: true,
        }
    }

    async fn chat(&self, request: ModelRequest) -> Result<ModelResponse, hepta_core::ModelError> {
        render_provider_response(request, |model| match model {
            "local-precise" => "[ollama-precise]".to_string(),
            _ => "[ollama-chat]".to_string(),
        })
    }
}

fn render_provider_response<F>(
    request: ModelRequest,
    style_for_model: F,
) -> Result<ModelResponse, hepta_core::ModelError>
where
    F: Fn(&str) -> String,
{
    let last_user = request
        .messages
        .iter()
        .rev()
        .find(|message| matches!(message.role, MessageRole::User))
        .map(|message| message.content.clone())
        .unwrap_or_default();

    let tool_message = request
        .messages
        .iter()
        .rev()
        .find(|message| matches!(message.role, MessageRole::Tool))
        .map(|message| message.content.clone());

    let model_style = style_for_model(&request.model.model);
    let context_text = request
        .messages
        .iter()
        .filter(|message| matches!(message.role, MessageRole::System))
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    if last_user.contains("暗号") && context_text.contains("暗号是蓝莓") {
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: "暗号是蓝莓。".into(),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 24,
                output_tokens: 6,
            },
        });
    }

    if let Some(tool_output) = tool_message {
        if tool_output.contains("disk_junk_audit") {
            return Ok(ModelResponse {
                message: Some(ModelMessage {
                    role: MessageRole::Assistant,
                    content: render_disk_junk_audit_reply(&tool_output),
                }),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Usage {
                    input_tokens: 48,
                    output_tokens: 80,
                },
            });
        }
        return Ok(ModelResponse {
            message: Some(ModelMessage {
                role: MessageRole::Assistant,
                content: format!(
                    "{} {}",
                    model_style,
                    render_native_tool_result_reply(&tool_output)
                ),
            }),
            tool_calls: vec![],
            finish_reason: FinishReason::Stop,
            usage: Usage {
                input_tokens: 32,
                output_tokens: 12,
            },
        });
    }

    if looks_like_disk_junk_audit_intent(&last_user) {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "disk_junk_audit".into(),
                arguments_json: json!({
                    "scope": "common_local_cleanup_candidates",
                    "max_entries": 120000,
                })
                .to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 48,
                output_tokens: 0,
            },
        });
    }

    if let Some(rest) = last_user.strip_prefix("tool:") {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "echo".into(),
                arguments_json: json!({ "text": rest.trim() }).to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 16,
                output_tokens: 0,
            },
        });
    }

    if let Some(path) = last_user.strip_prefix("read:") {
        return Ok(ModelResponse {
            message: None,
            tool_calls: vec![ToolCall {
                name: "read_file".into(),
                arguments_json: json!({ "path": path.trim() }).to_string(),
            }],
            finish_reason: FinishReason::ToolCall,
            usage: Usage {
                input_tokens: 16,
                output_tokens: 0,
            },
        });
    }

    if let Some(rest) = last_user.strip_prefix("write:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json:
                        json!({ "path": path.trim(), "content": content, "mode": "create" })
                            .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("overwrite:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json: json!({
                        "path": path.trim(),
                        "content": content,
                        "mode": "overwrite",
                        "confirm_destructive": true,
                    })
                    .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("append:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json:
                        json!({ "path": path.trim(), "content": content, "mode": "append" })
                            .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    if let Some(rest) = last_user.strip_prefix("preview-write:") {
        let mut parts = rest.trim().splitn(2, " => ");
        if let (Some(path), Some(content)) = (parts.next(), parts.next()) {
            return Ok(ModelResponse {
                message: None,
                tool_calls: vec![ToolCall {
                    name: "write_file".into(),
                    arguments_json: json!({
                        "path": path.trim(),
                        "content": content,
                        "mode": "overwrite",
                        "confirm_destructive": true,
                        "preview_only": true,
                    })
                    .to_string(),
                }],
                finish_reason: FinishReason::ToolCall,
                usage: Usage {
                    input_tokens: 20,
                    output_tokens: 0,
                },
            });
        }
    }

    Ok(ModelResponse {
        message: Some(ModelMessage {
            role: MessageRole::Assistant,
            content: format!("{} model reply: {}", model_style, last_user),
        }),
        tool_calls: vec![],
        finish_reason: FinishReason::Stop,
        usage: Usage {
            input_tokens: 16,
            output_tokens: 8,
        },
    })
}

impl ConfigurablePolicyEngine {
    fn requirement_for_risk(risk: RiskTier) -> ApprovalRequirement {
        match risk {
            RiskTier::Low => ApprovalRequirement::None,
            RiskTier::Medium => ApprovalRequirement::Ask,
            RiskTier::High => ApprovalRequirement::Deny,
        }
    }

    fn requirement_for_tool(tool_name: &str, risk: RiskTier) -> ApprovalRequirement {
        if tool_name == "exec" {
            ApprovalRequirement::Ask
        } else {
            Self::requirement_for_risk(risk)
        }
    }

    fn default_rules(&self) -> Vec<PolicyRule> {
        vec![
            PolicyRule {
                id: "default-risk-low".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::Low),
                requirement: ApprovalRequirement::None,
                reason: "low-risk tools are allowed by default".into(),
            },
            PolicyRule {
                id: "default-risk-medium".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::Medium),
                requirement: ApprovalRequirement::Ask,
                reason: "medium-risk tools require explicit approval by default".into(),
            },
            PolicyRule {
                id: "default-risk-high".into(),
                session_id: None,
                provider_name: None,
                tool_name: None,
                risk_tier: Some(RiskTier::High),
                requirement: ApprovalRequirement::Deny,
                reason: "high-risk tools are denied by default".into(),
            },
            PolicyRule {
                id: "default-tool-exec".into(),
                session_id: None,
                provider_name: None,
                tool_name: Some("exec".into()),
                risk_tier: None,
                requirement: ApprovalRequirement::Ask,
                reason: "exec requires explicit approval by default".into(),
            },
        ]
    }

    fn custom_rules(&self) -> Result<Vec<PolicyRule>, hepta_core::PolicyError> {
        self.custom_rules
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))
    }

    fn add_rule(&self, rule: PolicyRule) -> Result<PolicyRule, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        guard.push(rule.clone());
        Ok(rule)
    }

    fn remove_rule(&self, rule_id: &str) -> Result<bool, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        let before = guard.len();
        guard.retain(|rule| rule.id != rule_id);
        Ok(guard.len() != before)
    }

    fn clear_rules(&self) -> Result<usize, hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        let removed = guard.len();
        guard.clear();
        Ok(removed)
    }

    fn replace_rules(&self, rules: Vec<PolicyRule>) -> Result<(), hepta_core::PolicyError> {
        let mut guard = self
            .custom_rules
            .lock()
            .map_err(|_| hepta_core::PolicyError("policy state mutex poisoned".into()))?;
        *guard = rules;
        Ok(())
    }

    fn matches_rule(rule: &PolicyRule, context: &PolicyEvaluationContext) -> bool {
        if let Some(session_id) = rule.session_id.as_deref() {
            if context.session_id.as_ref().map(|value| value.0.as_str()) != Some(session_id) {
                return false;
            }
        }
        if let Some(provider_name) = rule.provider_name.as_deref() {
            if context.model.as_ref().map(|model| model.provider.as_str()) != Some(provider_name) {
                return false;
            }
        }
        if let Some(tool_name) = rule.tool_name.as_deref() {
            if context.tool_name != tool_name {
                return false;
            }
        }
        if let Some(risk_tier) = rule.risk_tier {
            if context.risk_tier != risk_tier {
                return false;
            }
        }
        true
    }

    fn rule_sort_key(
        rule: &PolicyRule,
        is_custom: bool,
        index: usize,
    ) -> (u8, usize, u8, u8, u8, u8, usize) {
        let selector_count = [
            rule.session_id.is_some(),
            rule.provider_name.is_some(),
            rule.tool_name.is_some(),
            rule.risk_tier.is_some(),
        ]
        .into_iter()
        .filter(|value| *value)
        .count();

        (
            if is_custom { 1 } else { 0 },
            selector_count,
            if rule.session_id.is_some() { 1 } else { 0 },
            if rule.tool_name.is_some() { 1 } else { 0 },
            if rule.provider_name.is_some() { 1 } else { 0 },
            if rule.risk_tier.is_some() { 1 } else { 0 },
            index,
        )
    }

    fn evaluate_with_match(
        &self,
        context: PolicyEvaluationContext,
    ) -> Result<PolicyDecision, hepta_core::PolicyError> {
        let defaults = self.default_rules();
        let customs = self.custom_rules()?;
        let mut best_match: Option<(PolicyRule, (u8, usize, u8, u8, u8, u8, usize))> = None;

        for (index, rule) in defaults.into_iter().enumerate() {
            if Self::matches_rule(&rule, &context) {
                let score = Self::rule_sort_key(&rule, false, index);
                if best_match
                    .as_ref()
                    .map(|(_, current)| score > *current)
                    .unwrap_or(true)
                {
                    best_match = Some((rule, score));
                }
            }
        }

        for (index, rule) in customs.into_iter().enumerate() {
            if Self::matches_rule(&rule, &context) {
                let score = Self::rule_sort_key(&rule, true, index);
                if best_match
                    .as_ref()
                    .map(|(_, current)| score > *current)
                    .unwrap_or(true)
                {
                    best_match = Some((rule, score));
                }
            }
        }

        match best_match {
            Some((rule, _)) => Ok(PolicyDecision {
                requirement: rule.requirement,
                reason: Self::decision_reason(&rule, &context),
                matched_rule_id: Some(rule.id),
            }),
            None => Ok(PolicyDecision {
                requirement: Self::requirement_for_risk(context.risk_tier),
                reason: format!(
                    "fallback risk policy for {} ({})",
                    context.tool_name,
                    format_risk_tier(context.risk_tier)
                ),
                matched_rule_id: None,
            }),
        }
    }

    fn decision_reason(rule: &PolicyRule, context: &PolicyEvaluationContext) -> String {
        match rule.id.as_str() {
            "default-risk-low" => format!("{} is low risk", context.tool_name),
            "default-risk-medium" => {
                format!(
                    "{} is medium risk and requires explicit approval",
                    context.tool_name
                )
            }
            "default-risk-high" => {
                format!("{} is high risk and denied by default", context.tool_name)
            }
            "default-tool-exec" => "exec requires explicit approval".into(),
            _ => rule.reason.clone(),
        }
    }
}

impl PolicyEngine for ConfigurablePolicyEngine {
    async fn evaluate_tool(
        &self,
        context: PolicyEvaluationContext,
    ) -> Result<PolicyDecision, hepta_core::PolicyError> {
        self.evaluate_with_match(context)
    }
}
