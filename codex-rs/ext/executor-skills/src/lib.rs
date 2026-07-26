//! Step-scoped executor skill discovery and resource-read authority.
//!
//! The host attaches one immutable catalog derived from the effective skills
//! selected for a sampling step. Tool calls must echo opaque authority,
//! package, and resource handles from that catalog; no ambient filesystem path
//! is accepted.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;

use codex_core_skills::SkillLoadOutcome;
use codex_exec_server::ExecutorFileSystem;
use codex_extension_api::ExtensionData;
use codex_extension_api::ExtensionRegistryBuilder;
use codex_extension_api::FunctionCallError;
use codex_extension_api::JsonToolOutput;
use codex_extension_api::ResponsesApiTool;
use codex_extension_api::ToolCall;
use codex_extension_api::ToolContributor;
use codex_extension_api::ToolExecutor;
use codex_extension_api::ToolName;
use codex_extension_api::ToolOutput;
use codex_extension_api::ToolSpec;
use codex_extension_api::parse_tool_input_schema;
use codex_tools::ResponsesApiNamespace;
use codex_tools::ResponsesApiNamespaceTool;
use codex_tools::default_namespace_description;
use codex_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use uuid::Uuid;

mod schema;

const SKILLS_NAMESPACE: &str = "skills/";
const MAX_HANDLE_BYTES: usize = 2_048;
const MAX_AUTHORIZED_SKILLS: usize = 4_096;
const MAX_SKILLS_PER_PAGE: usize = 20;
const MAX_LIST_RESPONSE_BYTES: usize = 512 * 1024;
const MAX_READ_RESPONSE_BYTES: usize = 512 * 1024;
const MAX_RESOURCE_CONTENT_BYTES: usize = 1024 * 1024;
const MAX_NAME_BYTES: usize = 256;
const MAX_DESCRIPTION_BYTES: usize = 4 * 1024;

/// Installs executor-owned skill tools. With no step authority the extension
/// contributes no tools.
pub fn install<C>(registry: &mut ExtensionRegistryBuilder<C>) {
    registry.tool_contributor(Arc::new(ExecutorSkillsExtension));
}

/// Replaces the exact executor-skill authority attached to one sampling step.
///
/// Calling this with an empty outcome rotates the generation and revokes any
/// previously captured tool executor.
pub async fn attach_step_authority(
    step_store: &ExtensionData,
    outcome: &SkillLoadOutcome,
    step_id: &str,
) {
    let next = Arc::new(StepGrant::from_outcome(outcome, step_id).await);
    if let Some(state) = step_store.get::<StepAuthorityState>() {
        state.replace(next);
    } else {
        step_store.insert(StepAuthorityState::new(next));
    }
}

#[derive(Clone, Copy, Debug)]
struct ExecutorSkillsExtension;

impl ToolContributor for ExecutorSkillsExtension {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        _thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn ToolExecutor<ToolCall>>> {
        Vec::new()
    }

    fn tools_for_step(
        &self,
        _session_store: &ExtensionData,
        _thread_store: &ExtensionData,
        step_store: &ExtensionData,
    ) -> Vec<Arc<dyn ToolExecutor<ToolCall>>> {
        let Some(state) = step_store.get::<StepAuthorityState>() else {
            return Vec::new();
        };
        let current = state.current();
        let generation = current.generation.clone();
        if current.entries.is_empty() && current.warnings.is_empty() {
            return Vec::new();
        }
        let context = ToolContext { state, generation };
        let mut tools: Vec<Arc<dyn ToolExecutor<ToolCall>>> = vec![Arc::new(ListTool {
            context: context.clone(),
        })];
        if !current.entries.is_empty() {
            tools.push(Arc::new(ReadTool { context }));
        }
        tools
    }
}

struct StepAuthorityState {
    current: RwLock<Arc<StepGrant>>,
}

impl StepAuthorityState {
    fn new(current: Arc<StepGrant>) -> Self {
        Self {
            current: RwLock::new(current),
        }
    }

    fn current(&self) -> Arc<StepGrant> {
        Arc::clone(&self.current.read().unwrap_or_else(PoisonError::into_inner))
    }

    fn replace(&self, next: Arc<StepGrant>) {
        *self.current.write().unwrap_or_else(PoisonError::into_inner) = next;
    }
}

struct StepGrant {
    generation: String,
    entries: Vec<AuthorizedSkill>,
    warnings: Vec<String>,
}

impl StepGrant {
    async fn from_outcome(outcome: &SkillLoadOutcome, step_id: &str) -> Self {
        struct Candidate {
            root: AbsolutePathBuf,
            path: AbsolutePathBuf,
            package_relative_path: PathBuf,
            name: String,
            description: String,
            file_system: Arc<dyn ExecutorFileSystem>,
        }

        struct FrozenCandidate {
            candidate: Candidate,
            main_resource_snapshot: Arc<ResourceSnapshot>,
        }

        let mut candidates = Vec::new();
        let mut warnings = Vec::new();
        for skill in &outcome.skills {
            if !outcome.is_skill_enabled(skill) || !outcome.is_executor_skill(skill) {
                continue;
            }
            let Some(root) = outcome.root_for_skill(skill).cloned() else {
                warnings.push(format!(
                    "skill `{}` omitted: selected root authority is unavailable",
                    truncate_utf8(&skill.name, MAX_NAME_BYTES)
                ));
                continue;
            };
            let Some(file_system) = outcome.file_system_for_skill(skill) else {
                warnings.push(format!(
                    "skill `{}` omitted: executor filesystem authority is unavailable",
                    truncate_utf8(&skill.name, MAX_NAME_BYTES)
                ));
                continue;
            };
            let Some(package_root) = skill.path_to_skills_md.parent() else {
                warnings.push(format!(
                    "skill `{}` omitted: package root is unavailable",
                    truncate_utf8(&skill.name, MAX_NAME_BYTES)
                ));
                continue;
            };
            let Some(package_relative_path) = relative_path_beneath_root(&root, &package_root)
            else {
                warnings.push(format!(
                    "skill `{}` omitted: package is outside its selected root authority",
                    truncate_utf8(&skill.name, MAX_NAME_BYTES)
                ));
                continue;
            };
            if skill.path_to_skills_md != package_root.join("SKILL.md") {
                warnings.push(format!(
                    "skill `{}` omitted: main resource is not canonical SKILL.md",
                    truncate_utf8(&skill.name, MAX_NAME_BYTES)
                ));
                continue;
            }
            candidates.push(Candidate {
                root,
                path: skill.path_to_skills_md.clone(),
                package_relative_path,
                name: truncate_utf8(&skill.name, MAX_NAME_BYTES),
                description: truncate_utf8(&skill.description, MAX_DESCRIPTION_BYTES),
                file_system,
            });
        }
        candidates.sort_by(|a, b| {
            a.root
                .cmp(&b.root)
                .then_with(|| a.path.cmp(&b.path))
                .then_with(|| a.name.cmp(&b.name))
        });
        if candidates.len() > MAX_AUTHORIZED_SKILLS {
            warnings.push(format!(
                "executor skill authority omitted {} entries above the {MAX_AUTHORIZED_SKILLS}-skill limit",
                candidates.len() - MAX_AUTHORIZED_SKILLS
            ));
            candidates.truncate(MAX_AUTHORIZED_SKILLS);
        }

        let mut frozen_candidates = Vec::new();
        for candidate in candidates {
            let authority_relative_path = candidate.package_relative_path.join("SKILL.md");
            let bytes = match candidate
                .file_system
                .read_file_beneath(
                    &candidate.root,
                    &authority_relative_path,
                    MAX_RESOURCE_CONTENT_BYTES as u64,
                    /*sandbox*/ None,
                )
                .await
            {
                Ok(bytes) => bytes,
                Err(_) => {
                    warnings.push(format!(
                        "skill `{}` omitted: executor lacks atomic bounded-read authority or the main resource changed",
                        candidate.name
                    ));
                    continue;
                }
            };
            let main_resource_snapshot = match ResourceSnapshot::from_bytes(bytes) {
                Ok(snapshot) => Arc::new(snapshot),
                Err(_) => {
                    warnings.push(format!(
                        "skill `{}` omitted: main resource is not valid bounded UTF-8",
                        candidate.name
                    ));
                    continue;
                }
            };
            frozen_candidates.push(FrozenCandidate {
                candidate,
                main_resource_snapshot,
            });
        }

        let mut generation_hasher = Sha256::new();
        hash_field(&mut generation_hasher, b"hepta-executor-skill-step-v1");
        hash_field(
            &mut generation_hasher,
            Uuid::new_v4().as_hyphenated().to_string().as_bytes(),
        );
        hash_field(&mut generation_hasher, step_id.as_bytes());
        for frozen in &frozen_candidates {
            let candidate = &frozen.candidate;
            hash_field(
                &mut generation_hasher,
                candidate.root.to_string_lossy().as_bytes(),
            );
            hash_field(
                &mut generation_hasher,
                candidate.path.to_string_lossy().as_bytes(),
            );
            hash_field(&mut generation_hasher, candidate.name.as_bytes());
            hash_field(&mut generation_hasher, candidate.description.as_bytes());
            hash_field(
                &mut generation_hasher,
                frozen.main_resource_snapshot.content_digest.as_bytes(),
            );
        }
        let generation = digest_hex(generation_hasher.finalize().as_slice());
        let mut root_authorities = HashMap::new();
        let mut entries = Vec::new();
        for frozen in frozen_candidates {
            let candidate = frozen.candidate;
            let root_key = candidate.root.to_string_lossy().into_owned();
            let authority = root_authorities
                .entry(root_key.clone())
                .or_insert_with(|| opaque_id("root", &[&generation, &root_key]))
                .clone();
            let package_token = opaque_id(
                "package",
                &[&authority, candidate.path.to_string_lossy().as_ref()],
            );
            let package = format!("skill://{authority}/{package_token}");
            let main_resource = format!("{package}/SKILL.md");
            entries.push(AuthorizedSkill {
                authority,
                package,
                main_resource,
                authority_root: candidate.root,
                package_relative_path: candidate.package_relative_path,
                name: candidate.name,
                description: candidate.description,
                file_system: candidate.file_system,
                main_resource_snapshot: frozen.main_resource_snapshot,
                resource_snapshots: RwLock::new(HashMap::new()),
            });
        }
        Self {
            generation,
            entries,
            warnings: warnings
                .into_iter()
                .take(8)
                .map(|warning| truncate_utf8(&warning, MAX_DESCRIPTION_BYTES))
                .collect(),
        }
    }
}

struct AuthorizedSkill {
    authority: String,
    package: String,
    main_resource: String,
    authority_root: AbsolutePathBuf,
    package_relative_path: PathBuf,
    name: String,
    description: String,
    file_system: Arc<dyn ExecutorFileSystem>,
    main_resource_snapshot: Arc<ResourceSnapshot>,
    resource_snapshots: RwLock<HashMap<PathBuf, Arc<ResourceSnapshot>>>,
}

struct ResourceSnapshot {
    contents: String,
    content_digest: String,
}

impl ResourceSnapshot {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, FunctionCallError> {
        let content_digest = format!("sha256:{}", digest_hex(Sha256::digest(&bytes).as_slice()));
        let contents = String::from_utf8(bytes)
            .map_err(|_| model_error("skill resource is not valid UTF-8"))?;
        Ok(Self {
            contents,
            content_digest,
        })
    }
}

#[derive(Clone)]
struct ToolContext {
    state: Arc<StepAuthorityState>,
    generation: String,
}

impl ToolContext {
    fn current(&self) -> Result<Arc<StepGrant>, FunctionCallError> {
        let current = self.state.current();
        if current.generation != self.generation {
            return Err(FunctionCallError::RespondToModel(
                "executor skill capability is stale; rebuild tools for the current step".into(),
            ));
        }
        Ok(current)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum AuthoritySelector {
    Executor,
}

#[derive(Clone, Debug, Deserialize, Hash, JsonSchema, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SkillAuthority {
    Executor { id: String },
}

impl SkillAuthority {
    fn id(&self) -> &str {
        match self {
            Self::Executor { id } => id,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ListArgs {
    authority: AuthoritySelector,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Hash, JsonSchema, Serialize)]
#[schemars(deny_unknown_fields)]
struct ListedSkill {
    authority: SkillAuthority,
    package: String,
    name: String,
    description: String,
    main_resource: String,
    main_resource_digest: String,
}

#[derive(JsonSchema, Serialize)]
#[schemars(deny_unknown_fields)]
struct ListResponse {
    skills: Vec<ListedSkill>,
    warnings: Vec<String>,
    next_cursor: Option<String>,
}

#[derive(Clone)]
struct ListTool {
    context: ToolContext,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ListTool {
    fn tool_name(&self) -> ToolName {
        tool_name("list")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(function_tool::<ListArgs, ListResponse>(
            "list",
            "List skills owned by executor roots selected for this exact step. Pass returned authority, package, and main_resource values unchanged to skills.read.",
        ))
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: ListArgs = parse_args(&call)?;
        match args.authority {
            AuthoritySelector::Executor => {}
        }
        let grant = self.context.current()?;
        let skills = grant
            .entries
            .iter()
            .map(|entry| ListedSkill {
                authority: SkillAuthority::Executor {
                    id: entry.authority.clone(),
                },
                package: entry.package.clone(),
                name: entry.name.clone(),
                description: entry.description.clone(),
                main_resource: entry.main_resource.clone(),
                main_resource_digest: entry.main_resource_snapshot.content_digest.clone(),
            })
            .collect::<Vec<_>>();
        let fingerprint = value_fingerprint(&(&grant.generation, &skills))?;
        let start = parse_cursor(args.cursor.as_deref(), &fingerprint, "skills.list")?;
        if start > skills.len() {
            return Err(model_error("skills.list cursor is invalid"));
        }
        let mut end = (start + MAX_SKILLS_PER_PAGE).min(skills.len());
        let mut warnings = if start == 0 {
            grant.warnings.clone()
        } else {
            Vec::new()
        };
        loop {
            let response = ListResponse {
                skills: skills[start..end].to_vec(),
                warnings: warnings.clone(),
                next_cursor: (end < skills.len()).then(|| cursor(&fingerprint, end)),
            };
            if serialized_len(&response)? <= MAX_LIST_RESPONSE_BYTES {
                return json_output(&response);
            }
            if end > start + 1 {
                end -= 1;
            } else if !warnings.is_empty() {
                warnings.clear();
            } else {
                return Err(model_error("skill metadata is too large to list"));
            }
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ReadArgs {
    authority: SkillAuthority,
    package: String,
    resource: String,
    cursor: Option<String>,
}

#[derive(JsonSchema, Serialize)]
#[schemars(deny_unknown_fields)]
struct ReadResponse {
    resource: String,
    content_digest: String,
    contents: String,
    next_cursor: Option<String>,
}

#[derive(Clone)]
struct ReadTool {
    context: ToolContext,
}

#[async_trait::async_trait]
impl ToolExecutor<ToolCall> for ReadTool {
    fn tool_name(&self) -> ToolName {
        tool_name("read")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(function_tool::<ReadArgs, ReadResponse>(
            "read",
            "Read one bounded page of a resource from an executor skill package selected for this exact step. Opaque handles must come unchanged from skills.list.",
        ))
    }

    async fn handle(&self, call: ToolCall) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: ReadArgs = parse_args(&call)?;
        validate_handle("authority.id", args.authority.id())?;
        validate_handle("package", &args.package)?;
        validate_handle("resource", &args.resource)?;
        let grant = self.context.current()?;
        let Some(entry) = grant
            .entries
            .iter()
            .find(|entry| entry.authority == args.authority.id() && entry.package == args.package)
        else {
            return Err(model_error(
                "skill package is not available from the requested executor authority",
            ));
        };
        let resource = entry.resource_path(&args.resource)?;
        let snapshot = entry.snapshot_resource(&resource).await?;
        let fingerprint =
            value_fingerprint(&(&grant.generation, &args.resource, &snapshot.content_digest))?;
        let start = parse_cursor(args.cursor.as_deref(), &fingerprint, "skills.read")?;
        if start > snapshot.contents.len() || !snapshot.contents.is_char_boundary(start) {
            return Err(model_error("skills.read cursor is invalid"));
        }
        json_output(&read_page(&args.resource, &snapshot, &fingerprint, start)?)
    }
}

impl AuthorizedSkill {
    async fn snapshot_resource(
        &self,
        resource: &AuthorizedResource,
    ) -> Result<Arc<ResourceSnapshot>, FunctionCallError> {
        if resource.package_relative_path == PathBuf::from("SKILL.md") {
            return Ok(Arc::clone(&self.main_resource_snapshot));
        }
        if let Some(snapshot) = self
            .resource_snapshots
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&resource.package_relative_path)
            .cloned()
        {
            return Ok(snapshot);
        }
        let authority_relative_path = self
            .package_relative_path
            .join(&resource.package_relative_path);
        let bytes = self
            .file_system
            .read_file_beneath(
                &self.authority_root,
                &authority_relative_path,
                MAX_RESOURCE_CONTENT_BYTES as u64,
                /*sandbox*/ None,
            )
            .await
            .map_err(|_| model_error("failed to read skill resource"))?;
        let snapshot = Arc::new(ResourceSnapshot::from_bytes(bytes)?);
        let mut snapshots = self
            .resource_snapshots
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        Ok(Arc::clone(
            snapshots
                .entry(resource.package_relative_path.clone())
                .or_insert(snapshot),
        ))
    }

    fn resource_path(&self, resource: &str) -> Result<AuthorizedResource, FunctionCallError> {
        if resource == self.main_resource {
            return Ok(AuthorizedResource {
                package_relative_path: PathBuf::from("SKILL.md"),
            });
        }
        let Some(relative) = resource
            .strip_prefix(self.package.trim_end_matches('/'))
            .and_then(|value| value.strip_prefix('/'))
        else {
            return Err(model_error("skill resource does not match its package"));
        };
        if relative.is_empty()
            || relative.starts_with('/')
            || relative.contains('\\')
            || relative
                .split('/')
                .any(|segment| matches!(segment, "" | "." | ".."))
        {
            return Err(model_error("skill resource path is invalid"));
        }
        let mut package_relative_path = PathBuf::new();
        for segment in relative.split('/') {
            let component = PathBuf::from(segment);
            if !matches!(
                component.components().collect::<Vec<_>>().as_slice(),
                [std::path::Component::Normal(_)]
            ) {
                return Err(model_error("skill resource path is invalid"));
            }
            package_relative_path.push(component);
        }
        Ok(AuthorizedResource {
            package_relative_path,
        })
    }
}

fn relative_path_beneath_root(
    root: &AbsolutePathBuf,
    package_root: &AbsolutePathBuf,
) -> Option<PathBuf> {
    let relative = package_root.as_path().strip_prefix(root.as_path()).ok()?;
    let mut safe_relative = PathBuf::new();
    for component in relative.components() {
        let std::path::Component::Normal(segment) = component else {
            return None;
        };
        safe_relative.push(segment);
    }
    Some(safe_relative)
}

struct AuthorizedResource {
    package_relative_path: PathBuf,
}

fn read_page(
    resource: &str,
    snapshot: &ResourceSnapshot,
    fingerprint: &str,
    start: usize,
) -> Result<ReadResponse, FunctionCallError> {
    let contents = &snapshot.contents;
    let response = |end, next_cursor| ReadResponse {
        resource: resource.to_string(),
        content_digest: snapshot.content_digest.clone(),
        contents: contents[start..end].to_string(),
        next_cursor,
    };
    let complete = response(contents.len(), None);
    if serialized_len(&complete)? <= MAX_READ_RESPONSE_BYTES {
        return Ok(complete);
    }
    let mut end = contents.len();
    while end > start {
        end = start + (end - start) / 2;
        while !contents.is_char_boundary(end) {
            end -= 1;
        }
        let candidate = response(end, Some(cursor(fingerprint, end)));
        if serialized_len(&candidate)? <= MAX_READ_RESPONSE_BYTES {
            return Ok(candidate);
        }
    }
    Err(FunctionCallError::Fatal(
        "skill resource handle leaves no room for contents".into(),
    ))
}

fn tool_name(name: &str) -> ToolName {
    ToolName::namespaced(SKILLS_NAMESPACE, name)
}

fn function_tool<I: JsonSchema, O: JsonSchema>(name: &str, description: &str) -> ToolSpec {
    ToolSpec::Namespace(ResponsesApiNamespace {
        name: SKILLS_NAMESPACE.to_string(),
        description: default_namespace_description(SKILLS_NAMESPACE),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: name.to_string(),
            description: description.to_string(),
            strict: false,
            defer_loading: None,
            parameters: parse_tool_input_schema(&schema::input_schema_for::<I>()).unwrap_or_else(
                |_| codex_tools::JsonSchema::object(BTreeMap::new(), None, Some(false.into())),
            ),
            output_schema: Some(schema::output_schema_for::<O>()),
        })],
    })
}

fn parse_args<T: for<'de> Deserialize<'de>>(call: &ToolCall) -> Result<T, FunctionCallError> {
    let arguments = call.function_arguments()?;
    let value = if arguments.trim().is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_str(arguments)
            .map_err(|error| FunctionCallError::RespondToModel(error.to_string()))?
    };
    serde_json::from_value(value)
        .map_err(|error| FunctionCallError::RespondToModel(error.to_string()))
}

fn validate_handle(name: &str, value: &str) -> Result<(), FunctionCallError> {
    if !value.is_empty() && value.len() <= MAX_HANDLE_BYTES && !value.chars().any(char::is_control)
    {
        return Ok(());
    }
    Err(model_error(format!(
        "{name} must be non-empty, contain no control characters, and be at most {MAX_HANDLE_BYTES} bytes"
    )))
}

fn parse_cursor(
    cursor_value: Option<&str>,
    fingerprint: &str,
    tool: &str,
) -> Result<usize, FunctionCallError> {
    let Some(cursor_value) = cursor_value else {
        return Ok(0);
    };
    validate_handle("cursor", cursor_value)?;
    let Some((actual, offset)) = cursor_value.split_once(':') else {
        return Err(model_error(format!("{tool} cursor is invalid")));
    };
    if actual != fingerprint {
        return Err(model_error(format!(
            "{tool} cursor is stale; restart from the first page"
        )));
    }
    offset
        .parse::<usize>()
        .map_err(|_| model_error(format!("{tool} cursor is invalid")))
}

fn cursor(fingerprint: &str, offset: usize) -> String {
    format!("{fingerprint}:{offset}")
}

fn value_fingerprint(value: &impl Serialize) -> Result<String, FunctionCallError> {
    let encoded =
        serde_json::to_vec(value).map_err(|error| FunctionCallError::Fatal(error.to_string()))?;
    Ok(opaque_id(
        "cursor",
        &[std::str::from_utf8(&encoded).unwrap_or("")],
    ))
}

fn opaque_id(domain: &str, values: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, b"hepta-executor-skill-handle-v1");
    hash_field(&mut hasher, domain.as_bytes());
    for value in values {
        hash_field(&mut hasher, value.as_bytes());
    }
    digest_hex(hasher.finalize().as_slice())
}

fn hash_field(hasher: &mut Sha256, field: &[u8]) {
    hasher.update((field.len() as u64).to_be_bytes());
    hasher.update(field);
}

fn digest_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(result, "{byte:02x}");
    }
    result
}

fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_string();
    }
    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}

fn serialized_len(value: &impl Serialize) -> Result<usize, FunctionCallError> {
    serde_json::to_vec(value)
        .map(|value| value.len())
        .map_err(|error| FunctionCallError::Fatal(error.to_string()))
}

fn json_output(value: &impl Serialize) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let value =
        serde_json::to_value(value).map_err(|error| FunctionCallError::Fatal(error.to_string()))?;
    Ok(Box::new(JsonToolOutput::new(value)))
}

fn model_error(message: impl Into<String>) -> FunctionCallError {
    FunctionCallError::RespondToModel(message.into())
}

#[cfg(test)]
mod tests;
