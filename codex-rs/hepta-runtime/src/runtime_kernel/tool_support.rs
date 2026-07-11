struct ToolRegistry {
    tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    fn new() -> Self {
        let mut tools = vec![
            RegisteredTool::Echo(EchoTool),
            RegisteredTool::ReadFile(ReadFileTool),
            RegisteredTool::WriteFile(WriteFileTool),
            RegisteredTool::ListDir(ListDirTool),
            RegisteredTool::SearchText(SearchTextTool),
            RegisteredTool::DiskJunkAudit(DiskJunkAuditTool),
            RegisteredTool::JsonGet(JsonGetTool),
            RegisteredTool::SkillPropose(SkillProposeTool),
            RegisteredTool::SkillScan(SkillScanTool),
            RegisteredTool::SkillApplyPlan(SkillApplyPlanTool),
            RegisteredTool::ToolManifestValidate(ToolManifestValidateTool),
            RegisteredTool::ToolGenerateStub(ToolGenerateStubTool),
        ];
        tools.extend(
            native_openclaw_compatible_tools()
                .into_iter()
                .map(RegisteredTool::NativeOpenClawCompatible),
        );
        Self { tools }
    }

    fn names(&self) -> Vec<String> {
        self.tools
            .iter()
            .map(|tool| tool.name().to_string())
            .collect()
    }

    fn descriptors(&self) -> Vec<ToolDescriptor> {
        self.tools
            .iter()
            .map(|tool| {
                let schema = tool.schema();
                ToolDescriptor {
                    name: schema.name,
                    description: schema.description,
                    risk_tier: tool.risk_tier(),
                    execution_metadata: tool.execution_metadata(),
                    default_approval_requirement: ConfigurablePolicyEngine::requirement_for_tool(
                        tool.name(),
                        tool.risk_tier(),
                    ),
                    input_schema_json: schema.input_schema_json,
                    output_schema_json: schema.output_schema_json,
                }
            })
            .collect()
    }

    fn model_tool_specs(&self) -> Vec<ModelToolSpec> {
        self.tools
            .iter()
            .map(|tool| {
                let schema = tool.schema();
                ModelToolSpec {
                    name: schema.name,
                    description: schema.description,
                    input_schema_json: schema.input_schema_json,
                }
            })
            .collect()
    }

    fn model_tool_specs_for_turn(&self, input: &str) -> Vec<ModelToolSpec> {
        if should_offer_model_tools_for_turn(input) {
            self.model_tool_specs()
        } else {
            Vec::new()
        }
    }

    fn contains(&self, name: &str) -> bool {
        self.tools.iter().any(|tool| tool.name() == name)
    }

    fn execution_metadata(
        &self,
        name: &str,
    ) -> Result<hepta_core::ToolExecutionMetadata, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.execution_metadata())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn schema(&self, name: &str) -> Result<hepta_core::ToolSchema, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.schema())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn risk_tier(&self, name: &str) -> Result<RiskTier, HeptaError> {
        self.tools
            .iter()
            .find(|tool| tool.name() == name)
            .map(|tool| tool.risk_tier())
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))
    }

    fn validate_input(&self, name: &str, input_json: &str) -> Result<(), HeptaError> {
        let schema = self.schema(name)?;
        validate_against_schema_json(&schema.name, "input", &schema.input_schema_json, input_json)
    }

    fn validate_output(&self, name: &str, output_json: &str) -> Result<(), HeptaError> {
        let schema = self.schema(name)?;
        validate_against_schema_json(
            &schema.name,
            "output",
            &schema.output_schema_json,
            output_json,
        )
    }

    async fn invoke(
        &self,
        name: &str,
        ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, HeptaError> {
        let tool = self
            .tools
            .iter()
            .find(|candidate| candidate.name() == name)
            .ok_or_else(|| HeptaError(format!("unknown tool: {}", name)))?;
        let result = if matches!(tool, RegisteredTool::NativeOpenClawCompatible(_)) {
            match tokio::time::timeout(
                Duration::from_millis(NATIVE_TOOL_INVOCATION_TIMEOUT_MS),
                tool.invoke(ctx, req),
            )
            .await
            {
                Ok(result) => result,
                Err(_) => Ok(native_tool_invocation_timeout_result(
                    name,
                    NATIVE_TOOL_INVOCATION_TIMEOUT_MS,
                )),
            }
        } else {
            tool.invoke(ctx, req).await
        };
        result.map_err(|err| HeptaError(err.0))
    }
}

enum RegisteredTool {
    Echo(EchoTool),
    ReadFile(ReadFileTool),
    WriteFile(WriteFileTool),
    ListDir(ListDirTool),
    SearchText(SearchTextTool),
    DiskJunkAudit(DiskJunkAuditTool),
    JsonGet(JsonGetTool),
    SkillPropose(SkillProposeTool),
    SkillScan(SkillScanTool),
    SkillApplyPlan(SkillApplyPlanTool),
    ToolManifestValidate(ToolManifestValidateTool),
    ToolGenerateStub(ToolGenerateStubTool),
    NativeOpenClawCompatible(NativeOpenClawCompatibleTool),
}

impl RegisteredTool {
    fn name(&self) -> &'static str {
        match self {
            Self::Echo(tool) => tool.name(),
            Self::ReadFile(tool) => tool.name(),
            Self::WriteFile(tool) => tool.name(),
            Self::ListDir(tool) => tool.name(),
            Self::SearchText(tool) => tool.name(),
            Self::DiskJunkAudit(tool) => tool.name(),
            Self::JsonGet(tool) => tool.name(),
            Self::SkillPropose(tool) => tool.name(),
            Self::SkillScan(tool) => tool.name(),
            Self::SkillApplyPlan(tool) => tool.name(),
            Self::ToolManifestValidate(tool) => tool.name(),
            Self::ToolGenerateStub(tool) => tool.name(),
            Self::NativeOpenClawCompatible(tool) => tool.name(),
        }
    }

    fn risk_tier(&self) -> RiskTier {
        match self {
            Self::Echo(tool) => tool.risk_tier(),
            Self::ReadFile(tool) => tool.risk_tier(),
            Self::WriteFile(tool) => tool.risk_tier(),
            Self::ListDir(tool) => tool.risk_tier(),
            Self::SearchText(tool) => tool.risk_tier(),
            Self::DiskJunkAudit(tool) => tool.risk_tier(),
            Self::JsonGet(tool) => tool.risk_tier(),
            Self::SkillPropose(tool) => tool.risk_tier(),
            Self::SkillScan(tool) => tool.risk_tier(),
            Self::SkillApplyPlan(tool) => tool.risk_tier(),
            Self::ToolManifestValidate(tool) => tool.risk_tier(),
            Self::ToolGenerateStub(tool) => tool.risk_tier(),
            Self::NativeOpenClawCompatible(tool) => tool.risk_tier(),
        }
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        match self {
            Self::Echo(tool) => tool.execution_metadata(),
            Self::ReadFile(tool) => tool.execution_metadata(),
            Self::WriteFile(tool) => tool.execution_metadata(),
            Self::ListDir(tool) => tool.execution_metadata(),
            Self::SearchText(tool) => tool.execution_metadata(),
            Self::DiskJunkAudit(tool) => tool.execution_metadata(),
            Self::JsonGet(tool) => tool.execution_metadata(),
            Self::SkillPropose(tool) => tool.execution_metadata(),
            Self::SkillScan(tool) => tool.execution_metadata(),
            Self::SkillApplyPlan(tool) => tool.execution_metadata(),
            Self::ToolManifestValidate(tool) => tool.execution_metadata(),
            Self::ToolGenerateStub(tool) => tool.execution_metadata(),
            Self::NativeOpenClawCompatible(tool) => tool.execution_metadata(),
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        match self {
            Self::Echo(tool) => tool.schema(),
            Self::ReadFile(tool) => tool.schema(),
            Self::WriteFile(tool) => tool.schema(),
            Self::ListDir(tool) => tool.schema(),
            Self::SearchText(tool) => tool.schema(),
            Self::DiskJunkAudit(tool) => tool.schema(),
            Self::JsonGet(tool) => tool.schema(),
            Self::SkillPropose(tool) => tool.schema(),
            Self::SkillScan(tool) => tool.schema(),
            Self::SkillApplyPlan(tool) => tool.schema(),
            Self::ToolManifestValidate(tool) => tool.schema(),
            Self::ToolGenerateStub(tool) => tool.schema(),
            Self::NativeOpenClawCompatible(tool) => tool.schema(),
        }
    }

    async fn invoke(
        &self,
        ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        match self {
            Self::Echo(tool) => tool.invoke(ctx, req).await,
            Self::ReadFile(tool) => tool.invoke(ctx, req).await,
            Self::WriteFile(tool) => tool.invoke(ctx, req).await,
            Self::ListDir(tool) => tool.invoke(ctx, req).await,
            Self::SearchText(tool) => tool.invoke(ctx, req).await,
            Self::DiskJunkAudit(tool) => tool.invoke(ctx, req).await,
            Self::JsonGet(tool) => tool.invoke(ctx, req).await,
            Self::SkillPropose(tool) => tool.invoke(ctx, req).await,
            Self::SkillScan(tool) => tool.invoke(ctx, req).await,
            Self::SkillApplyPlan(tool) => tool.invoke(ctx, req).await,
            Self::ToolManifestValidate(tool) => tool.invoke(ctx, req).await,
            Self::ToolGenerateStub(tool) => tool.invoke(ctx, req).await,
            Self::NativeOpenClawCompatible(tool) => tool.invoke(ctx, req).await,
        }
    }
}

struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Return the provided input as-is".into(),
            input_schema_json: r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let text = parse_required_string_field(&req.input_json, "text")?;
        Ok(ToolResult {
            content: format!("echo:{}", text),
            structured_json: Some(json!({ "text": text }).to_string()),
        })
    }
}

struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Read a UTF-8 text file from disk".into(),
            input_schema_json: r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string","minLength":1,"description":"relative or absolute file path"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","preview","line_count"],"properties":{"path":{"type":"string","minLength":1},"preview":{"type":"string"},"line_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let content = fs::read_to_string(&path).map_err(|err| {
            hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
        })?;
        let preview = content.lines().take(6).collect::<Vec<_>>().join(" | ");
        let line_count = content.lines().count();
        Ok(ToolResult {
            content: format!("read_file:{} => {}", path.display(), preview),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "preview": preview,
                    "line_count": line_count,
                })
                .to_string(),
            ),
        })
    }
}

struct WriteFileTool;

impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::High
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: false,
            destructive: true,
            idempotent: false,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Write a UTF-8 text file to disk with explicit create, overwrite, or append semantics".into(),
            input_schema_json: r#"{"type":"object","required":["path","content"],"properties":{"path":{"type":"string","minLength":1,"description":"relative or absolute file path"},"content":{"type":"string","minLength":0,"description":"UTF-8 file content to write"},"mode":{"type":"string","enum":["create","overwrite","append"],"description":"create=new file only, overwrite=replace existing, append=append to existing or create"},"confirm_destructive":{"type":"boolean","description":"required for overwriting an existing file"},"preview_only":{"type":"boolean","description":"when true, return diff/backup plan without mutating the filesystem"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","bytes_written","mode_requested","mode_applied","existed_before","preview_only","content_changed","bytes_before","bytes_after","backup_planned","backup_created","change_summary"],"properties":{"path":{"type":"string","minLength":1},"bytes_written":{"type":"integer","minimum":0},"mode_requested":{"type":"string","enum":["create","overwrite","append"]},"mode_applied":{"type":"string","enum":["create","overwrite","append"]},"existed_before":{"type":"boolean"},"preview_only":{"type":"boolean"},"content_changed":{"type":"boolean"},"bytes_before":{"type":"integer","minimum":0},"bytes_after":{"type":"integer","minimum":0},"backup_planned":{"type":"boolean"},"backup_created":{"type":"boolean"},"backup_path":{"type":"string","minLength":1},"change_summary":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let content = parse_required_string_field(&req.input_json, "content")?;
        let mode = parse_optional_string_field(&req.input_json, "mode")?
            .unwrap_or_else(|| "create".to_string());
        let preview_only =
            parse_optional_bool_field(&req.input_json, "preview_only")?.unwrap_or(false);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let existed_before = path.exists();
        let before_content = if existed_before {
            Some(fs::read_to_string(&path).map_err(|err| {
                hepta_core::ToolError(format!(
                    "failed to read existing content from {}: {}",
                    path.display(),
                    err
                ))
            })?)
        } else {
            None
        };
        let before_text = before_content.as_deref().unwrap_or("");
        let after_content = match mode.as_str() {
            "create" | "overwrite" => content.clone(),
            "append" => format!("{}{}", before_text, content),
            other => {
                return Err(hepta_core::ToolError(format!(
                    "unsupported write mode {} for {}",
                    other,
                    path.display()
                )));
            }
        };
        let bytes_before = before_text.len();
        let bytes_after = after_content.len();
        let content_changed = before_text != after_content;
        let backup_planned = existed_before && mode == "overwrite";
        let change_summary = summarize_write_change(
            mode.as_str(),
            existed_before,
            content_changed,
            bytes_before,
            bytes_after,
        );

        if preview_only {
            let mut output = serde_json::Map::new();
            output.insert("path".into(), json!(path.display().to_string()));
            output.insert("bytes_written".into(), json!(0));
            output.insert("mode_requested".into(), json!(mode.clone()));
            output.insert("mode_applied".into(), json!(mode.clone()));
            output.insert("existed_before".into(), json!(existed_before));
            output.insert("preview_only".into(), json!(true));
            output.insert("content_changed".into(), json!(content_changed));
            output.insert("bytes_before".into(), json!(bytes_before));
            output.insert("bytes_after".into(), json!(bytes_after));
            output.insert("backup_planned".into(), json!(backup_planned));
            output.insert("backup_created".into(), json!(false));
            if let Some(backup_path) = backup_planned
                .then(|| preview_backup_path(&workspace_root, &path))
                .transpose()?
            {
                output.insert(
                    "backup_path".into(),
                    json!(backup_path.display().to_string()),
                );
            }
            output.insert("change_summary".into(), json!(change_summary.clone()));
            return Ok(ToolResult {
                content: format!(
                    "write_file:{} => preview {}",
                    path.display(),
                    change_summary
                ),
                structured_json: Some(Value::Object(output).to_string()),
            });
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                hepta_core::ToolError(format!(
                    "failed to create parent directories for {}: {}",
                    path.display(),
                    err
                ))
            })?;
        }
        let mut backup_path = None;
        let mode_applied = match mode.as_str() {
            "create" => {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to create {}: {}",
                            path.display(),
                            err
                        ))
                    })?;
                use std::io::Write as _;
                file.write_all(content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
                })?;
                "create"
            }
            "overwrite" => {
                if let Some(previous_content) = before_content.as_deref() {
                    let planned_backup_path = preview_backup_path(&workspace_root, &path)?;
                    if let Some(parent) = planned_backup_path.parent() {
                        fs::create_dir_all(parent).map_err(|err| {
                            hepta_core::ToolError(format!(
                                "failed to create backup parent directories for {}: {}",
                                planned_backup_path.display(),
                                err
                            ))
                        })?;
                    }
                    fs::write(&planned_backup_path, previous_content.as_bytes()).map_err(
                        |err| {
                            hepta_core::ToolError(format!(
                                "failed to write backup {}: {}",
                                planned_backup_path.display(),
                                err
                            ))
                        },
                    )?;
                    backup_path = Some(planned_backup_path);
                }
                fs::write(&path, content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!(
                        "failed to overwrite {}: {}",
                        path.display(),
                        err
                    ))
                })?;
                "overwrite"
            }
            "append" => {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to append {}: {}",
                            path.display(),
                            err
                        ))
                    })?;
                use std::io::Write as _;
                file.write_all(content.as_bytes()).map_err(|err| {
                    hepta_core::ToolError(format!("failed to append {}: {}", path.display(), err))
                })?;
                "append"
            }
            other => {
                return Err(hepta_core::ToolError(format!(
                    "unsupported write mode {} for {}",
                    other,
                    path.display()
                )));
            }
        };
        Ok(ToolResult {
            content: format!(
                "write_file:{} => {} bytes ({})",
                path.display(),
                content.len(),
                mode_applied
            ),
            structured_json: Some({
                let mut output = serde_json::Map::new();
                output.insert("path".into(), json!(path.display().to_string()));
                output.insert("bytes_written".into(), json!(content.len()));
                output.insert("mode_requested".into(), json!(mode.clone()));
                output.insert("mode_applied".into(), json!(mode_applied));
                output.insert("existed_before".into(), json!(existed_before));
                output.insert("preview_only".into(), json!(false));
                output.insert("content_changed".into(), json!(content_changed));
                output.insert("bytes_before".into(), json!(bytes_before));
                output.insert("bytes_after".into(), json!(bytes_after));
                output.insert("backup_planned".into(), json!(backup_planned));
                output.insert("backup_created".into(), json!(backup_path.is_some()));
                if let Some(backup_path) = backup_path.as_ref() {
                    output.insert(
                        "backup_path".into(),
                        json!(backup_path.display().to_string()),
                    );
                }
                output.insert("change_summary".into(), json!(change_summary.clone()));
                Value::Object(output).to_string()
            }),
        })
    }
}

struct ListDirTool;

impl Tool for ListDirTool {
    fn name(&self) -> &'static str {
        "list_dir"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "List immediate files and directories under a workspace path".into(),
            input_schema_json: r#"{"type":"object","properties":{"path":{"type":"string","minLength":1},"max_entries":{"type":"integer","minimum":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","entry_count"],"properties":{"path":{"type":"string","minLength":1},"entry_count":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path =
            parse_optional_string_field(&req.input_json, "path")?.unwrap_or_else(|| ".".into());
        let max_entries = parse_optional_usize_field(&req.input_json, "max_entries")?.unwrap_or(50);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let mut entries = fs::read_dir(&path)
            .map_err(|err| {
                hepta_core::ToolError(format!("failed to list {}: {}", path.display(), err))
            })?
            .map(|entry| {
                entry
                    .map_err(|err| {
                        hepta_core::ToolError(format!("failed to read dir entry: {}", err))
                    })
                    .map(|entry| {
                        let path = entry.path();
                        json!({
                            "name": entry.file_name().to_string_lossy().to_string(),
                            "kind": if path.is_dir() { "dir" } else { "file" },
                        })
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
        let total = entries.len();
        let truncated = entries.len() > max_entries;
        entries.truncate(max_entries);
        Ok(ToolResult {
            content: format!("list_dir:{} => {} entries", path.display(), total),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "entry_count": total,
                    "truncated": truncated,
                    "entries": entries,
                })
                .to_string(),
            ),
        })
    }
}

struct SearchTextTool;

impl Tool for SearchTextTool {
    fn name(&self) -> &'static str {
        "search_text"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Search UTF-8 text files under a workspace path for a literal pattern".into(),
            input_schema_json: r#"{"type":"object","required":["path","pattern"],"properties":{"path":{"type":"string","minLength":1},"pattern":{"type":"string","minLength":1},"max_results":{"type":"integer","minimum":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["path","pattern","match_count"],"properties":{"path":{"type":"string","minLength":1},"pattern":{"type":"string","minLength":1},"match_count":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let requested_path = parse_required_string_field(&req.input_json, "path")?;
        let pattern = parse_required_string_field(&req.input_json, "pattern")?;
        let max_results = parse_optional_usize_field(&req.input_json, "max_results")?.unwrap_or(25);
        let workspace_root = tool_workspace_root_path();
        let path = resolve_path_within_root(&workspace_root, Path::new(&requested_path));
        let mut files = Vec::new();
        if path.is_file() {
            files.push(path.clone());
        } else {
            collect_files_recursive(&path, &mut files)
                .map_err(|err| hepta_core::ToolError(err.0))?;
        }
        files.sort();
        let mut matches = Vec::new();
        for file in files {
            if matches.len() >= max_results {
                break;
            }
            let Ok(content) = fs::read_to_string(&file) else {
                continue;
            };
            for (index, line) in content.lines().enumerate() {
                if line.contains(&pattern) {
                    matches.push(json!({
                        "path": file.display().to_string(),
                        "line": index + 1,
                        "preview": line.chars().take(180).collect::<String>(),
                    }));
                    if matches.len() >= max_results {
                        break;
                    }
                }
            }
        }
        let match_count = matches.len();
        Ok(ToolResult {
            content: format!("search_text:{} => {} matches", path.display(), match_count),
            structured_json: Some(
                json!({
                    "path": path.display().to_string(),
                    "pattern": pattern,
                    "match_count": match_count,
                    "truncated": match_count >= max_results,
                    "matches": matches,
                })
                .to_string(),
            ),
        })
    }
}

struct DiskJunkAuditTool;

#[derive(Debug, Clone)]
struct DiskJunkCandidate {
    path: PathBuf,
    kind: &'static str,
    bytes: u64,
    entries_scanned: usize,
    inaccessible_count: usize,
    truncated: bool,
    recommendation: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct BoundedDirSize {
    bytes: u64,
    entries_scanned: usize,
    inaccessible_count: usize,
    truncated: bool,
}

impl Tool for DiskJunkAuditTool {
    fn name(&self) -> &'static str {
        "disk_junk_audit"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Run a bounded, read-only local disk cleanup candidate audit over common cache/log/temp roots".into(),
            input_schema_json: r#"{"type":"object","properties":{"scope":{"type":"string"},"max_entries":{"type":"integer","minimum":1},"include_var_folders":{"type":"boolean"}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["status","read_only","candidate_count","estimated_reclaimable_bytes"],"properties":{"status":{"type":"string"},"read_only":{"type":"boolean"},"candidate_count":{"type":"integer","minimum":0},"estimated_reclaimable_bytes":{"type":"integer","minimum":0},"truncated":{"type":"boolean"}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let max_entries = parse_optional_usize_field(&req.input_json, "max_entries")?
            .unwrap_or(120_000)
            .clamp(100, 500_000);
        let include_var_folders =
            parse_optional_bool_field(&req.input_json, "include_var_folders")?.unwrap_or(true);
        let roots = disk_junk_candidate_roots(include_var_folders);
        let per_root_limit = (max_entries / roots.len().max(1)).max(500);
        let mut candidates = Vec::new();
        for (path, kind, recommendation) in roots {
            if !path.exists() {
                continue;
            }
            let size = bounded_dir_size(&path, per_root_limit, 12);
            if size.bytes == 0 && size.entries_scanned == 0 {
                continue;
            }
            candidates.push(DiskJunkCandidate {
                path,
                kind,
                bytes: size.bytes,
                entries_scanned: size.entries_scanned,
                inaccessible_count: size.inaccessible_count,
                truncated: size.truncated,
                recommendation,
            });
        }
        candidates.sort_by(|left, right| right.bytes.cmp(&left.bytes));
        let estimated_reclaimable_bytes = candidates.iter().map(|candidate| candidate.bytes).sum();
        let truncated = candidates.iter().any(|candidate| candidate.truncated);
        let top = candidates
            .iter()
            .take(12)
            .map(|candidate| {
                json!({
                    "path": candidate.path.display().to_string(),
                    "kind": candidate.kind,
                    "bytes": candidate.bytes,
                    "human_size": human_bytes(candidate.bytes),
                    "entries_scanned": candidate.entries_scanned,
                    "inaccessible_count": candidate.inaccessible_count,
                    "truncated": candidate.truncated,
                    "recommendation": candidate.recommendation,
                    "safe_action": "review_then_delete_contents_only",
                })
            })
            .collect::<Vec<_>>();
        let summary_lines = top
            .iter()
            .take(5)
            .filter_map(|value| {
                Some(format!(
                    "{} {}",
                    value.get("human_size")?.as_str()?,
                    value.get("path")?.as_str()?
                ))
            })
            .collect::<Vec<_>>();
        Ok(ToolResult {
            content: format!(
                "disk_junk_audit: read-only scan found {} cleanup candidate root(s), estimated reclaimable {}. {}",
                candidates.len(),
                human_bytes(estimated_reclaimable_bytes),
                summary_lines.join("; ")
            ),
            structured_json: Some(
                json!({
                    "status": "completed",
                    "read_only": true,
                    "scope": "common_local_cleanup_candidates",
                    "candidate_count": candidates.len(),
                    "estimated_reclaimable_bytes": estimated_reclaimable_bytes,
                    "estimated_reclaimable_human": human_bytes(estimated_reclaimable_bytes),
                    "truncated": truncated,
                    "note": "This audit only reads metadata/content sizes and does not delete anything.",
                    "top_candidates": top,
                })
                .to_string(),
            ),
        })
    }
}

struct JsonGetTool;

impl Tool for JsonGetTool {
    fn name(&self) -> &'static str {
        "json_get"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Extract a JSON value by RFC-6901 pointer from a JSON string".into(),
            input_schema_json: r#"{"type":"object","required":["json","pointer"],"properties":{"json":{"type":"string","minLength":1},"pointer":{"type":"string","minLength":0}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["pointer","found"],"properties":{"pointer":{"type":"string","minLength":0},"found":{"type":"boolean"},"value_json":{"type":"string","minLength":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let json_text = parse_required_string_field(&req.input_json, "json")?;
        let pointer = parse_required_string_field(&req.input_json, "pointer")?;
        let value: Value = serde_json::from_str(&json_text)
            .map_err(|err| hepta_core::ToolError(format!("invalid JSON payload: {}", err)))?;
        let selected = if pointer.is_empty() {
            Some(&value)
        } else {
            value.pointer(&pointer)
        };
        let value_json = selected.map(Value::to_string).unwrap_or_default();
        Ok(ToolResult {
            content: format!("json_get:{} => found={}", pointer, selected.is_some()),
            structured_json: Some(
                json!({
                    "pointer": pointer,
                    "found": selected.is_some(),
                    "value_json": value_json,
                })
                .to_string(),
            ),
        })
    }
}

struct SkillProposeTool;

impl Tool for SkillProposeTool {
    fn name(&self) -> &'static str {
        "skill_propose"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Generate a quarantined SKILL.md draft from transcript text".into(),
            input_schema_json: r#"{"type":"object","required":["transcript"],"properties":{"transcript":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["skill_name","safe_to_apply","audit_id"],"properties":{"skill_name":{"type":"string","minLength":1},"safe_to_apply":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let transcript = parse_required_string_field(&req.input_json, "transcript")?;
        let draft = hepta_core::propose_skill_from_transcript(&transcript);
        Ok(ToolResult {
            content: format!(
                "skill_propose:{} safe={}",
                draft.skill_name, draft.scan.safe_to_apply
            ),
            structured_json: Some(
                json!({
                    "skill_name": draft.skill_name,
                    "title": draft.title,
                    "description": draft.description,
                    "skill_md": draft.skill_md,
                    "safe_to_apply": draft.scan.safe_to_apply,
                    "finding_count": draft.scan.finding_count,
                    "quarantine_path": draft.quarantine_path,
                    "apply_path": draft.apply_path,
                    "audit_id": draft.audit_id,
                })
                .to_string(),
            ),
        })
    }
}

struct SkillScanTool;

impl Tool for SkillScanTool {
    fn name(&self) -> &'static str {
        "skill_scan"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Scan a SKILL.md draft for local safety and structure violations".into(),
            input_schema_json: r#"{"type":"object","required":["skill_md"],"properties":{"skill_md":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["safe_to_apply","finding_count"],"properties":{"safe_to_apply":{"type":"boolean"},"finding_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let skill_md = parse_required_string_field(&req.input_json, "skill_md")?;
        let scan = hepta_core::scan_skill_markdown(&skill_md);
        Ok(ToolResult {
            content: format!(
                "skill_scan safe={} findings={}",
                scan.safe_to_apply, scan.finding_count
            ),
            structured_json: Some(
                serde_json::to_string(&scan)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct SkillApplyPlanTool;

impl Tool for SkillApplyPlanTool {
    fn name(&self) -> &'static str {
        "skill_apply_plan"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Medium
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Create a review-gated atomic apply plan for a generated skill draft".into(),
            input_schema_json: r#"{"type":"object","required":["transcript"],"properties":{"transcript":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["skill_name","safe_to_apply","review_required","snapshot_refresh_required"],"properties":{"skill_name":{"type":"string","minLength":1},"safe_to_apply":{"type":"boolean"},"review_required":{"type":"boolean"},"snapshot_refresh_required":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let transcript = parse_required_string_field(&req.input_json, "transcript")?;
        let draft = hepta_core::propose_skill_from_transcript(&transcript);
        let plan = hepta_core::skill_apply_plan_from_draft(&draft);
        Ok(ToolResult {
            content: format!(
                "skill_apply_plan:{} safe={} review={}",
                plan.skill_name, plan.safe_to_apply, plan.review_required
            ),
            structured_json: Some(
                serde_json::to_string(&plan)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct ToolManifestValidateTool;

impl Tool for ToolManifestValidateTool {
    fn name(&self) -> &'static str {
        "tool_manifest_validate"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Validate a generated tool manifest before promotion".into(),
            input_schema_json: r#"{"type":"object","required":["manifest_json"],"properties":{"manifest_json":{"type":"string","minLength":1}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["valid","issue_count"],"properties":{"valid":{"type":"boolean"},"issue_count":{"type":"integer","minimum":0}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let manifest_json = parse_required_string_field(&req.input_json, "manifest_json")?;
        let manifest: hepta_core::GeneratedToolManifest = serde_json::from_str(&manifest_json)
            .map_err(|err| {
                hepta_core::ToolError(format!("invalid generated tool manifest: {}", err))
            })?;
        let validation = hepta_core::validate_tool_manifest(&manifest);
        Ok(ToolResult {
            content: format!(
                "tool_manifest_validate:{} valid={} issues={}",
                manifest.name, validation.valid, validation.issue_count
            ),
            structured_json: Some(
                serde_json::to_string(&validation)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

struct ToolGenerateStubTool;

impl Tool for ToolGenerateStubTool {
    fn name(&self) -> &'static str {
        "tool_generate_stub"
    }

    fn risk_tier(&self) -> RiskTier {
        RiskTier::Low
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: true,
            destructive: false,
            idempotent: true,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name().into(),
            description: "Generate a canonical local tool manifest/stub from operator intent".into(),
            input_schema_json: r#"{"type":"object","required":["name"],"properties":{"name":{"type":"string","minLength":1},"description":{"type":"string","minLength":0}}}"#.into(),
            output_schema_json: r#"{"type":"object","required":["name","risk_tier","read_only","audit_id"],"properties":{"name":{"type":"string","minLength":1},"risk_tier":{"type":"string","minLength":1},"read_only":{"type":"boolean"},"audit_id":{"type":"string","minLength":1}}}"#.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let name = parse_required_string_field(&req.input_json, "name")?;
        let description =
            parse_optional_string_field(&req.input_json, "description")?.unwrap_or_default();
        let manifest = hepta_core::generate_tool_manifest(&name, &description);
        Ok(ToolResult {
            content: format!(
                "tool_generate_stub:{} risk={}",
                manifest.name, manifest.risk_tier
            ),
            structured_json: Some(
                serde_json::to_string(&manifest)
                    .map_err(|err| hepta_core::ToolError(err.to_string()))?,
            ),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct NativeOpenClawCompatibleTool {
    name: &'static str,
    description: &'static str,
    risk_tier: RiskTier,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    behavior: NativeOpenClawCompatibleBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeOpenClawCompatibleBehavior {
    Read,
    Write,
    Edit,
    ApplyPatch,
    Exec,
    Process,
    WebFetch,
    WebSearch,
    MemorySearch,
    MemoryGet,
    SessionStatus,
    PlanEcho,
    NativeSurface,
}

const NATIVE_OPENCLAW_COMPAT_INPUT_SCHEMA: &str = r#"{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"},"command":{"type":"string"},"query":{"type":"string"},"url":{"type":"string"},"action":{"type":"string"},"message":{"type":"string"},"text":{"type":"string"},"input":{"type":"string"},"edits":{"type":"array"},"offset":{"type":"integer"},"limit":{"type":"integer"},"page_size":{"type":"integer"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"background":{"type":"boolean"},"dryRun":{"type":"boolean"},"preview_only":{"type":"boolean"}},"additionalProperties":true}"#;
const NATIVE_OPENCLAW_COMPAT_OUTPUT_SCHEMA: &str = r#"{"type":"object","properties":{"tool":{"type":"string"},"status":{"type":"string"},"native_runtime":{"type":"boolean"},"backend":{"type":"string"},"proxy_used":{"type":"boolean"},"content":{"type":"string"},"result":{"type":"object"},"error":{"type":"string"}},"additionalProperties":true}"#;

fn native_openclaw_compat_input_schema(
    tool: &str,
    behavior: NativeOpenClawCompatibleBehavior,
) -> &'static str {
    match (tool, behavior) {
        ("read", _) | (_, NativeOpenClawCompatibleBehavior::Read) => {
            r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string","description":"File path relative to the Hepta workspace unless absolute paths are allowed by runtime policy"},"offset":{"type":"integer","minimum":1,"default":1},"limit":{"type":"integer","minimum":1,"default":2000}},"additionalProperties":true}"#
        }
        ("write", _) | (_, NativeOpenClawCompatibleBehavior::Write) => {
            r#"{"type":"object","required":["path","content"],"properties":{"path":{"type":"string"},"content":{"type":"string"},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("edit", _) | (_, NativeOpenClawCompatibleBehavior::Edit) => {
            r#"{"type":"object","required":["path","edits"],"properties":{"path":{"type":"string"},"edits":{"type":"array","items":{"type":"object","required":["oldText","newText"],"properties":{"oldText":{"type":"string"},"newText":{"type":"string"}}}},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("apply_patch", _) | (_, NativeOpenClawCompatibleBehavior::ApplyPatch) => {
            r#"{"type":"object","required":["input"],"properties":{"input":{"type":"string","description":"apply_patch format, including *** Begin Patch and *** End Patch"},"patch":{"type":"string"},"dryRun":{"type":"boolean","default":false},"preview_only":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("exec", _) | (_, NativeOpenClawCompatibleBehavior::Exec) => {
            r#"{"type":"object","required":["command"],"properties":{"command":{"type":"string"},"workdir":{"type":"string"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"background":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("process", _) | (_, NativeOpenClawCompatibleBehavior::Process) => {
            r#"{"type":"object","properties":{"action":{"type":"string","enum":["list","status","poll","log","read","write","submit","kill","terminate","clear","remove"]},"sessionId":{"type":"string"},"session_id":{"type":"string"},"id":{"type":"string"},"data":{"type":"string"},"text":{"type":"string"},"offset":{"type":"integer"},"limit":{"type":"integer"},"timeout":{"type":"integer"},"timeoutMs":{"type":"integer"},"eof":{"type":"boolean"}},"additionalProperties":true}"#
        }
        ("web_fetch", _) | (_, NativeOpenClawCompatibleBehavior::WebFetch) => {
            r#"{"type":"object","required":["url"],"properties":{"url":{"type":"string"},"extractMode":{"type":"string"},"maxChars":{"type":"integer","default":20000}},"additionalProperties":true}"#
        }
        ("web_search", _) | (_, NativeOpenClawCompatibleBehavior::WebSearch) => {
            r#"{"type":"object","required":["query"],"properties":{"query":{"type":"string"},"count":{"type":"integer","default":5},"maxChars":{"type":"integer","default":20000}},"additionalProperties":true}"#
        }
        ("memory_search", _) | (_, NativeOpenClawCompatibleBehavior::MemorySearch) => {
            r#"{"type":"object","required":["query"],"properties":{"query":{"type":"string"},"maxResults":{"type":"integer","default":10},"max_results":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("memory_get", _) | (_, NativeOpenClawCompatibleBehavior::MemoryGet) => {
            r#"{"type":"object","required":["path"],"properties":{"path":{"type":"string"},"from":{"type":"integer","minimum":1},"lines":{"type":"integer","minimum":1}},"additionalProperties":true}"#
        }
        ("message", _) => {
            r#"{"type":"object","required":["action"],"properties":{"action":{"type":"string","enum":["send","read","channel-list","channel-info","member-info"]},"channel":{"type":"string","default":"telegram"},"target":{"type":"string"},"message":{"type":"string"},"text":{"type":"string"},"dryRun":{"type":"boolean","default":true},"confirmSend":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("tts", _) => {
            r#"{"type":"object","required":["text"],"properties":{"text":{"type":"string"},"path":{"type":"string"},"filename":{"type":"string"},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("image_generate", _) | ("music_generate", _) | ("video_generate", _) => {
            r#"{"type":"object","required":["prompt"],"properties":{"prompt":{"type":"string"},"filename":{"type":"string"},"model":{"type":"string"},"durationSeconds":{"type":"integer"},"dryRun":{"type":"boolean","default":false},"timeoutMs":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("image", _) | ("pdf", _) => {
            r#"{"type":"object","properties":{"image":{"type":"string"},"images":{"type":"array","items":{"type":"string"}},"pdf":{"type":"string"},"pdfs":{"type":"array","items":{"type":"string"}},"prompt":{"type":"string"},"pages":{"type":"string"},"maxBytesMb":{"type":"integer"}},"additionalProperties":true}"#
        }
        ("sessions_history", _) => {
            r#"{"type":"object","properties":{"sessionKey":{"type":"string"},"session_id":{"type":"string"},"limit":{"type":"integer","default":20},"includeTools":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("sessions_send", _) => {
            r#"{"type":"object","required":["message"],"properties":{"sessionKey":{"type":"string"},"label":{"type":"string"},"message":{"type":"string"},"execute":{"type":"boolean","default":true},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        ("sessions_spawn", _) => {
            r#"{"type":"object","required":["task"],"properties":{"task":{"type":"string"},"label":{"type":"string"},"agentId":{"type":"string"},"execute":{"type":"boolean","default":true},"dryRun":{"type":"boolean","default":false}},"additionalProperties":true}"#
        }
        _ => NATIVE_OPENCLAW_COMPAT_INPUT_SCHEMA,
    }
}

impl Tool for NativeOpenClawCompatibleTool {
    fn name(&self) -> &'static str {
        self.name
    }

    fn risk_tier(&self) -> RiskTier {
        self.risk_tier
    }

    fn execution_metadata(&self) -> hepta_core::ToolExecutionMetadata {
        hepta_core::ToolExecutionMetadata {
            read_only: self.read_only,
            destructive: self.destructive,
            idempotent: self.idempotent,
            produces_structured_output: true,
        }
    }

    fn schema(&self) -> hepta_core::ToolSchema {
        hepta_core::ToolSchema {
            name: self.name.into(),
            description: self.description.into(),
            input_schema_json: native_openclaw_compat_input_schema(self.name, self.behavior).into(),
            output_schema_json: NATIVE_OPENCLAW_COMPAT_OUTPUT_SCHEMA.into(),
        }
    }

    async fn invoke(
        &self,
        _ctx: ToolContext,
        req: ToolCallRequest,
    ) -> Result<ToolResult, hepta_core::ToolError> {
        let input = parse_tool_input_object(&req.input_json)?;
        let result = match self.behavior {
            NativeOpenClawCompatibleBehavior::Read => native_compat_read(self.name, &input),
            NativeOpenClawCompatibleBehavior::Write => native_compat_write(self.name, &input),
            NativeOpenClawCompatibleBehavior::Edit => native_compat_edit(self.name, &input),
            NativeOpenClawCompatibleBehavior::ApplyPatch => {
                native_compat_apply_patch(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::Exec => native_compat_exec(self.name, &input),
            NativeOpenClawCompatibleBehavior::Process => native_compat_process(self.name, &input),
            NativeOpenClawCompatibleBehavior::WebFetch => {
                native_compat_web_fetch(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::WebSearch => {
                native_compat_web_search(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::MemorySearch => {
                native_compat_memory_search(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::MemoryGet => {
                native_compat_memory_get(self.name, &input)
            }
            NativeOpenClawCompatibleBehavior::SessionStatus => {
                Ok(native_compat_status_report(self.name, &input))
            }
            NativeOpenClawCompatibleBehavior::PlanEcho => {
                Ok(native_compat_plan_echo(self.name, &input))
            }
            NativeOpenClawCompatibleBehavior::NativeSurface => {
                native_compat_live_surface(self.name, &input)
            }
        }?;
        let content = result
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or("native tool completed")
            .to_string();
        Ok(ToolResult {
            content,
            structured_json: Some(Value::Object(result).to_string()),
        })
    }
}

fn native_openclaw_compatible_tools() -> Vec<NativeOpenClawCompatibleTool> {
    use NativeOpenClawCompatibleBehavior as B;
    vec![
        native_tool(
            "read",
            "Read a text file using Hepta's Rust-native workspace reader",
            RiskTier::Medium,
            true,
            false,
            true,
            B::Read,
        ),
        native_tool(
            "write",
            "Write a file using Hepta's Rust-native workspace writer",
            RiskTier::High,
            false,
            true,
            false,
            B::Write,
        ),
        native_tool(
            "edit",
            "Apply exact text replacements using Hepta's Rust-native editor",
            RiskTier::High,
            false,
            true,
            false,
            B::Edit,
        ),
        native_tool(
            "apply_patch",
            "Apply a bounded apply_patch-format patch using Hepta's Rust-native patch parser",
            RiskTier::High,
            false,
            true,
            false,
            B::ApplyPatch,
        ),
        native_tool(
            "exec",
            "Run a local shell command through Hepta's Rust-native process runner; use this for filesystem maintenance or cache cleanup only with the normal high-risk approval gate",
            RiskTier::High,
            false,
            true,
            false,
            B::Exec,
        ),
        native_tool(
            "process",
            "Inspect or control Hepta background process sessions created by exec background=true; not for deleting files, caches, or workspace storage",
            RiskTier::Medium,
            true,
            false,
            true,
            B::Process,
        ),
        native_tool(
            "canvas",
            "Run Hepta-native canvas-plane adapter/audit actions without OpenClaw proxying",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "message",
            "Send or preview Telegram messages through Hepta's native gated channel adapter",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "tts",
            "Synthesize local speech through Hepta's native macOS TTS adapter",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "image_generate",
            "Generate images through Hepta's native local Ollama/helper adapter",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "music_generate",
            "Generate music through a configured Hepta-native local generator command",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "video_generate",
            "Generate video through a configured Hepta-native local generator command",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "agents_list",
            "List Hepta-native agent surface metadata without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "update_plan",
            "Echo a model-supplied plan through Hepta's native structured-output surface",
            RiskTier::Low,
            false,
            false,
            false,
            B::PlanEcho,
        ),
        native_tool(
            "sessions_list",
            "List Hepta-native session surface metadata without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_history",
            "Read Hepta-native session history through the local runtime CLI",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_send",
            "Run a prompt in a Hepta-native session through the local runtime CLI",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_spawn",
            "Spawn a durable Hepta-native worker task through the local runtime CLI",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "sessions_yield",
            "Record a Hepta-native session yield event without OpenClaw proxying",
            RiskTier::Low,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "subagents",
            "List, steer, or stop Hepta-native top-level agents through the local runtime CLI",
            RiskTier::Medium,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "session_status",
            "Return Hepta native runtime/tool status without OpenClaw proxying",
            RiskTier::Low,
            true,
            false,
            true,
            B::SessionStatus,
        ),
        native_tool(
            "web_search",
            "Run a best-effort Rust-native web search via local curl, not OpenClaw",
            RiskTier::Medium,
            true,
            false,
            true,
            B::WebSearch,
        ),
        native_tool(
            "web_fetch",
            "Fetch a URL via local curl from Hepta native code, not OpenClaw",
            RiskTier::Medium,
            true,
            false,
            true,
            B::WebFetch,
        ),
        native_tool(
            "image",
            "Analyze local image metadata through Hepta-native filesystem tools",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "pdf",
            "Extract local PDF text/metadata through Hepta-native filesystem tools",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "memory_search",
            "Search Hepta/OpenClaw workspace memory files using local Rust filesystem reads",
            RiskTier::Low,
            true,
            false,
            true,
            B::MemorySearch,
        ),
        native_tool(
            "memory_get",
            "Read a bounded excerpt from a workspace memory file using local Rust filesystem reads",
            RiskTier::Low,
            true,
            false,
            true,
            B::MemoryGet,
        ),
        native_tool(
            "feishu_doc",
            "Run Hepta-native Feishu document adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_app_scopes",
            "Run Hepta-native Feishu app-scope adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_chat",
            "Run Hepta-native Feishu chat adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_wiki",
            "Run Hepta-native Feishu wiki adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_drive",
            "Run Hepta-native Feishu drive adapter readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_get_meta",
            "Run Hepta-native Feishu bitable metadata adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_list_fields",
            "Run Hepta-native Feishu bitable field-list adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_list_records",
            "Run Hepta-native Feishu bitable record-list adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_get_record",
            "Run Hepta-native Feishu bitable record-get adapter readiness or gated live probe",
            RiskTier::Medium,
            true,
            false,
            true,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_record",
            "Run Hepta-native Feishu bitable record-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_update_record",
            "Run Hepta-native Feishu bitable record-update readiness or gated live probe",
            RiskTier::High,
            false,
            true,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_app",
            "Run Hepta-native Feishu bitable app-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
        native_tool(
            "feishu_bitable_create_field",
            "Run Hepta-native Feishu bitable field-create readiness or gated live probe",
            RiskTier::High,
            false,
            false,
            false,
            B::NativeSurface,
        ),
    ]
}

fn native_tool(
    name: &'static str,
    description: &'static str,
    risk_tier: RiskTier,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    behavior: NativeOpenClawCompatibleBehavior,
) -> NativeOpenClawCompatibleTool {
    NativeOpenClawCompatibleTool {
        name,
        description,
        risk_tier,
        read_only,
        destructive,
        idempotent,
        behavior,
    }
}

fn parse_tool_input_object(
    input_json: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let value: Value = serde_json::from_str(input_json)
        .map_err(|err| hepta_core::ToolError(format!("invalid JSON tool input: {}", err)))?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| hepta_core::ToolError("tool input must be a JSON object".into()))
}

fn native_compat_base(tool: &str, status: &str) -> serde_json::Map<String, Value> {
    let mut out = serde_json::Map::new();
    out.insert("tool".into(), Value::String(tool.into()));
    out.insert("status".into(), Value::String(status.into()));
    out.insert("native_runtime".into(), Value::Bool(true));
    out.insert("backend".into(), Value::String("hepta-rust-native".into()));
    out.insert("proxy_used".into(), Value::Bool(false));
    out.insert("openclaw_gateway_invoked".into(), Value::Bool(false));
    out
}

fn native_tool_invocation_timeout_result(tool: &str, timeout_ms: u64) -> ToolResult {
    let error = format!("ToolTimeout/{} timed out after {} ms", tool, timeout_ms);
    let mut out = native_compat_base(tool, "timeout");
    out.insert("content".into(), Value::String(error.clone()));
    out.insert("error".into(), Value::String(error.clone()));
    out.insert("error_kind".into(), Value::String("ToolTimeout".into()));
    out.insert("timeout".into(), Value::Bool(true));
    out.insert(
        "result".into(),
        json!({
            "timeout": true,
            "timeout_ms": timeout_ms,
            "fallback_reason": "tool-timeout",
            "duplicate_tool_replay_prevented": true,
        }),
    );
    ToolResult {
        content: error,
        structured_json: Some(Value::Object(out).to_string()),
    }
}

fn tool_result_is_timeout(tool_result: &ToolResult) -> bool {
    if tool_result.content.contains("ToolTimeout/") || tool_result.content.contains(" timed out") {
        return true;
    }
    let Some(structured_json) = tool_result.structured_json.as_deref() else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<Value>(structured_json) else {
        return false;
    };
    value.get("status").and_then(Value::as_str) == Some("timeout")
        || value.get("timeout").and_then(Value::as_bool) == Some(true)
        || value.get("error_kind").and_then(Value::as_str) == Some("ToolTimeout")
        || value
            .get("result")
            .and_then(|result| result.get("timeout"))
            .and_then(Value::as_bool)
            == Some(true)
}

fn native_compat_read(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("read requires string field 'path'".into()))?;
    let offset = input
        .get("offset")
        .and_then(Value::as_u64)
        .unwrap_or(1)
        .max(1) as usize;
    let limit = input
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(2000)
        .max(1) as usize;
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let content = fs::read_to_string(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
    })?;
    let lines: Vec<&str> = content.lines().collect();
    let start = offset.saturating_sub(1).min(lines.len());
    let end = start.saturating_add(limit).min(lines.len());
    let excerpt = lines[start..end].join("\n");
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(excerpt.clone()));
    out.insert(
        "result".into(),
        json!({
            "path": path.display().to_string(),
            "offset": offset,
            "limit": limit,
            "line_count": lines.len(),
            "returned_lines": end.saturating_sub(start),
            "truncated": end < lines.len(),
            "text": excerpt
        }),
    );
    Ok(out)
}

fn native_compat_write(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("write requires string field 'path'".into()))?;
    let content = input
        .get("content")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("write requires string field 'content'".into()))?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let existed_before = path.exists();
    if !preview_only {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                hepta_core::ToolError(format!("failed to create {}: {}", parent.display(), err))
            })?;
        }
        fs::write(&path, content).map_err(|err| {
            hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
        })?;
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} bytes to {}{}",
            if preview_only { "would write" } else { "wrote" },
            content.len(),
            path.display(),
            if existed_before {
                " (overwrote existing file)"
            } else {
                ""
            }
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "path": path.display().to_string(),
            "bytes": content.len(),
            "existed_before": existed_before,
            "preview_only": preview_only
        }),
    );
    Ok(out)
}

fn native_compat_edit(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path_text = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("edit requires string field 'path'".into()))?;
    let edits = input
        .get("edits")
        .and_then(Value::as_array)
        .ok_or_else(|| hepta_core::ToolError("edit requires array field 'edits'".into()))?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let workspace_root = tool_workspace_root_path();
    let path = resolve_path_within_root(&workspace_root, Path::new(path_text));
    let mut content = fs::read_to_string(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to read {}: {}", path.display(), err))
    })?;
    let mut applied = 0usize;
    for edit in edits {
        let old_text = edit
            .get("oldText")
            .or_else(|| edit.get("old_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires oldText".into()))?;
        let new_text = edit
            .get("newText")
            .or_else(|| edit.get("new_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| hepta_core::ToolError("each edit requires newText".into()))?;
        let count = content.matches(old_text).count();
        if count != 1 {
            return Err(hepta_core::ToolError(format!(
                "oldText must match exactly once; matched {} times",
                count
            )));
        }
        content = content.replacen(old_text, new_text, 1);
        applied += 1;
    }
    if !preview_only {
        fs::write(&path, content).map_err(|err| {
            hepta_core::ToolError(format!("failed to write {}: {}", path.display(), err))
        })?;
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} edit(s) in {}",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            applied,
            path.display()
        )),
    );
    out.insert(
        "result".into(),
        json!({ "path": path.display().to_string(), "edits_applied": applied, "preview_only": preview_only }),
    );
    Ok(out)
}

fn native_compat_apply_patch(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let patch = input
        .get("input")
        .or_else(|| input.get("patch"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("apply_patch requires string field 'input' or 'patch'".into())
        })?;
    let preview_only = input
        .get("preview_only")
        .or_else(|| input.get("dryRun"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let ops = parse_native_apply_patch(patch)?;
    let workspace_root = tool_workspace_root_path();
    let mut summaries = Vec::new();
    for op in ops {
        match op {
            NativePatchOp::Add { path, content } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                if target.exists() {
                    return Err(hepta_core::ToolError(format!(
                        "cannot add existing file {}",
                        target.display()
                    )));
                }
                if !preview_only {
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent).map_err(|err| {
                            hepta_core::ToolError(format!(
                                "failed to create {}: {}",
                                parent.display(),
                                err
                            ))
                        })?;
                    }
                    fs::write(&target, &content).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to add {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(
                    json!({"op":"add","path":target.display().to_string(),"bytes":content.len()}),
                );
            }
            NativePatchOp::Delete { path } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                if !target.exists() {
                    return Err(hepta_core::ToolError(format!(
                        "cannot delete missing file {}",
                        target.display()
                    )));
                }
                if !preview_only {
                    fs::remove_file(&target).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to delete {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(json!({"op":"delete","path":target.display().to_string()}));
            }
            NativePatchOp::Update { path, old, new } => {
                let target = resolve_path_within_root(&workspace_root, Path::new(&path));
                let current = fs::read_to_string(&target).map_err(|err| {
                    hepta_core::ToolError(format!("failed to read {}: {}", target.display(), err))
                })?;
                let count = current.matches(&old).count();
                if count != 1 {
                    return Err(hepta_core::ToolError(format!(
                        "patch update for {} matched old hunk {} times; expected exactly once",
                        target.display(),
                        count
                    )));
                }
                let updated = current.replacen(&old, &new, 1);
                if !preview_only {
                    fs::write(&target, updated).map_err(|err| {
                        hepta_core::ToolError(format!(
                            "failed to update {}: {}",
                            target.display(),
                            err
                        ))
                    })?;
                }
                summaries.push(json!({"op":"update","path":target.display().to_string(),"old_bytes":old.len(),"new_bytes":new.len()}));
            }
        }
    }
    let mut out = native_compat_base(tool, if preview_only { "preview" } else { "ok" });
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} {} patch operation(s)",
            if preview_only {
                "would apply"
            } else {
                "applied"
            },
            summaries.len()
        )),
    );
    out.insert(
        "result".into(),
        json!({"operation_count": summaries.len(), "operations": summaries, "preview_only": preview_only}),
    );
    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NativePatchOp {
    Add {
        path: String,
        content: String,
    },
    Delete {
        path: String,
    },
    Update {
        path: String,
        old: String,
        new: String,
    },
}

fn parse_native_apply_patch(patch: &str) -> Result<Vec<NativePatchOp>, hepta_core::ToolError> {
    let lines: Vec<&str> = patch.lines().collect();
    if lines.first().copied() != Some("*** Begin Patch")
        || lines.last().copied() != Some("*** End Patch")
    {
        return Err(hepta_core::ToolError(
            "apply_patch input must start with *** Begin Patch and end with *** End Patch".into(),
        ));
    }
    let mut ops = Vec::new();
    let mut i = 1usize;
    while i + 1 < lines.len() {
        let line = lines[i];
        if let Some(path) = line.strip_prefix("*** Add File: ") {
            i += 1;
            let mut content = String::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                let raw = lines[i];
                if let Some(added) = raw.strip_prefix('+') {
                    content.push_str(added);
                } else {
                    content.push_str(raw);
                }
                content.push('\n');
                i += 1;
            }
            ops.push(NativePatchOp::Add {
                path: path.trim().into(),
                content,
            });
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Delete File: ") {
            ops.push(NativePatchOp::Delete {
                path: path.trim().into(),
            });
            i += 1;
            continue;
        }
        if let Some(path) = line.strip_prefix("*** Update File: ") {
            i += 1;
            let mut old = String::new();
            let mut new = String::new();
            while i < lines.len() && !lines[i].starts_with("*** ") {
                let raw = lines[i];
                if raw.starts_with("@@") {
                    i += 1;
                    continue;
                }
                if let Some(removed) = raw.strip_prefix('-') {
                    old.push_str(removed);
                    old.push('\n');
                } else if let Some(added) = raw.strip_prefix('+') {
                    new.push_str(added);
                    new.push('\n');
                } else if let Some(context) = raw.strip_prefix(' ') {
                    old.push_str(context);
                    old.push('\n');
                    new.push_str(context);
                    new.push('\n');
                } else {
                    old.push_str(raw);
                    old.push('\n');
                    new.push_str(raw);
                    new.push('\n');
                }
                i += 1;
            }
            if old.is_empty() && new.is_empty() {
                return Err(hepta_core::ToolError(format!(
                    "update patch for {} has no hunk content",
                    path.trim()
                )));
            }
            ops.push(NativePatchOp::Update {
                path: path.trim().into(),
                old,
                new,
            });
            continue;
        }
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        return Err(hepta_core::ToolError(format!(
            "unsupported apply_patch line: {}",
            line
        )));
    }
    if ops.is_empty() {
        return Err(hepta_core::ToolError(
            "apply_patch input contained no operations".into(),
        ));
    }
    Ok(ops)
}

const NATIVE_EXEC_DEFAULT_TIMEOUT_MS: u64 = 45_000;
const NATIVE_EXEC_MIN_TIMEOUT_MS: u64 = 100;
const NATIVE_EXEC_MAX_TIMEOUT_MS: u64 = 300_000;
const NATIVE_EXEC_POLL_INTERVAL_MS: u64 = 50;
const NATIVE_EXEC_KILL_GRACE_MS: u64 = 750;
const NATIVE_TOOL_INVOCATION_TIMEOUT_MS: u64 = 60_000;

#[derive(Debug, Clone)]
struct NativeCommandRunOutput {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    success: bool,
    timed_out: bool,
    killed_process_tree: bool,
    timeout_ms: u64,
    elapsed_ms: u64,
}

fn native_timeout_ms_from_input(input: &serde_json::Map<String, Value>) -> u64 {
    let requested_ms = input
        .get("timeoutMs")
        .or_else(|| input.get("timeout_ms"))
        .and_then(Value::as_u64)
        .or_else(|| {
            input
                .get("timeout")
                .and_then(Value::as_u64)
                .map(|seconds| seconds.saturating_mul(1_000))
        })
        .unwrap_or(NATIVE_EXEC_DEFAULT_TIMEOUT_MS);
    requested_ms.clamp(NATIVE_EXEC_MIN_TIMEOUT_MS, NATIVE_EXEC_MAX_TIMEOUT_MS)
}

fn native_command_temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    env::temp_dir().join(format!(
        "hepta-native-{}-{}-{}.tmp",
        label,
        std::process::id(),
        nanos
    ))
}

fn prepare_native_command(command: &str, workdir: &Path) -> Command {
    let mut cmd = Command::new("/bin/zsh");
    cmd.arg("-lc").arg(command).current_dir(workdir);
    #[cfg(unix)]
    {
        cmd.process_group(0);
    }
    cmd
}

fn native_send_signal_to_pid_tree(pid: u32, signal: &str) -> bool {
    let mut ok = false;
    #[cfg(unix)]
    {
        ok |= Command::new("/bin/kill")
            .arg(signal)
            .arg(format!("-{}", pid))
            .status()
            .map(|status| status.success())
            .unwrap_or(false);
    }
    ok |= Command::new("/bin/kill")
        .arg(signal)
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    ok
}

fn native_wait_for_child_exit(child: &mut Child, wait_ms: u64) -> Option<Option<i32>> {
    let started = SystemTime::now();
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return Some(status.code());
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= wait_ms)
            .unwrap_or(true)
        {
            return None;
        }
        thread::sleep(Duration::from_millis(NATIVE_EXEC_POLL_INTERVAL_MS));
    }
}

fn native_kill_child_process_tree(child: &mut Child) -> bool {
    let pid = child.id();
    let mut signalled = native_send_signal_to_pid_tree(pid, "-TERM");
    if native_wait_for_child_exit(child, NATIVE_EXEC_KILL_GRACE_MS).is_some() {
        return signalled;
    }
    signalled |= native_send_signal_to_pid_tree(pid, "-KILL");
    signalled |= child.kill().is_ok();
    let _ = child.wait();
    signalled
}

fn native_run_command_with_deadline(
    command: &str,
    workdir: &Path,
    timeout_ms: u64,
) -> Result<NativeCommandRunOutput, hepta_core::ToolError> {
    let stdout_path = native_command_temp_path("stdout");
    let stderr_path = native_command_temp_path("stderr");
    let stdout_file = fs::File::create(&stdout_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to create {}: {}",
            stdout_path.display(),
            err
        ))
    })?;
    let stderr_file = fs::File::create(&stderr_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to create {}: {}",
            stderr_path.display(),
            err
        ))
    })?;
    let mut child = prepare_native_command(command, workdir)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|err| hepta_core::ToolError(format!("failed to spawn command: {}", err)))?;
    let started = SystemTime::now();
    let mut exit_code = None::<i32>;
    let mut success = false;
    let mut timed_out = false;
    let mut killed_process_tree = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                exit_code = status.code();
                success = status.success();
                break;
            }
            Ok(None) => {}
            Err(err) => {
                return Err(hepta_core::ToolError(format!(
                    "failed to poll native exec command: {}",
                    err
                )));
            }
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= timeout_ms)
            .unwrap_or(true)
        {
            timed_out = true;
            killed_process_tree = native_kill_child_process_tree(&mut child);
            break;
        }
        thread::sleep(Duration::from_millis(NATIVE_EXEC_POLL_INTERVAL_MS));
    }
    let elapsed_ms = started
        .elapsed()
        .map(|elapsed| elapsed.as_millis() as u64)
        .unwrap_or(timeout_ms);
    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&stderr_path).unwrap_or_default();
    let _ = fs::remove_file(&stdout_path);
    let _ = fs::remove_file(&stderr_path);
    Ok(NativeCommandRunOutput {
        stdout,
        stderr,
        exit_code,
        success,
        timed_out,
        killed_process_tree,
        timeout_ms,
        elapsed_ms,
    })
}

fn native_compat_exec(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let command = input
        .get("command")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("exec requires string field 'command'".into()))?;
    let workdir = input
        .get("workdir")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(tool_workspace_root_path);
    let background = input
        .get("background")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if background {
        return native_compat_exec_background(tool, command, &workdir);
    }
    let timeout_ms = native_timeout_ms_from_input(input);
    let output = native_run_command_with_deadline(command, &workdir, timeout_ms)?;
    if output.timed_out {
        let error = format!(
            "ToolTimeout/native_compat_exec timed out after {} ms",
            output.timeout_ms
        );
        let mut out = native_compat_base(tool, "timeout");
        out.insert("content".into(), Value::String(error.clone()));
        out.insert("error".into(), Value::String(error.clone()));
        out.insert("error_kind".into(), Value::String("ToolTimeout".into()));
        out.insert("timeout".into(), Value::Bool(true));
        out.insert(
            "result".into(),
            json!({
                "command": command,
                "workdir": workdir.display().to_string(),
                "exit_code": output.exit_code,
                "stdout": output.stdout,
                "stderr": output.stderr,
                "timeout": true,
                "timeout_ms": output.timeout_ms,
                "elapsed_ms": output.elapsed_ms,
                "killed_process_tree": output.killed_process_tree,
                "fallback_reason": "tool-timeout",
                "duplicate_tool_replay_prevented": true,
            }),
        );
        return Ok(out);
    }
    let mut out = native_compat_base(tool, if output.success { "ok" } else { "error" });
    out.insert("content".into(), Value::String(output.stdout.clone()));
    out.insert(
        "result".into(),
        json!({
            "command": command,
            "workdir": workdir.display().to_string(),
            "exit_code": output.exit_code.unwrap_or(-1),
            "stdout": output.stdout,
            "stderr": output.stderr,
            "timeout": false,
            "timeout_ms": output.timeout_ms,
            "elapsed_ms": output.elapsed_ms
        }),
    );
    Ok(out)
}

struct NativeBackgroundProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    command: String,
    workdir: PathBuf,
    log_path: PathBuf,
    started_at_unix_ms: u64,
}

static NATIVE_BACKGROUND_PROCESSES: OnceLock<Mutex<HashMap<String, NativeBackgroundProcess>>> =
    OnceLock::new();

fn native_process_registry() -> &'static Mutex<HashMap<String, NativeBackgroundProcess>> {
    NATIVE_BACKGROUND_PROCESSES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn native_compat_exec_background(
    tool: &str,
    command: &str,
    workdir: &Path,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let started_at_unix_ms = current_unix_ms().map_err(|err| hepta_core::ToolError(err.0))?;
    let log_dir = native_process_log_dir();
    fs::create_dir_all(&log_dir).map_err(|err| {
        hepta_core::ToolError(format!("failed to create {}: {}", log_dir.display(), err))
    })?;
    let temp_session_id = format!("hepta-proc-{}-pending", started_at_unix_ms);
    let temp_log_path = log_dir.join(format!("{}.log", temp_session_id));
    fs::write(
        &temp_log_path,
        format!("$ {}\n", command.replace('\n', "\\n")),
    )
    .map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to initialize {}: {}",
            temp_log_path.display(),
            err
        ))
    })?;
    let stdout_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&temp_log_path)
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to open {} for stdout capture: {}",
                temp_log_path.display(),
                err
            ))
        })?;
    let stderr_file = stdout_file.try_clone().map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to clone {} for stderr capture: {}",
            temp_log_path.display(),
            err
        ))
    })?;
    let mut child = prepare_native_command(command, workdir)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .map_err(|err| hepta_core::ToolError(format!("failed to spawn command: {}", err)))?;
    let pid = child.id();
    let session_id = format!("hepta-proc-{}-{}", started_at_unix_ms, pid);
    let log_path = log_dir.join(format!("{}.log", session_id));
    fs::rename(&temp_log_path, &log_path).map_err(|err| {
        hepta_core::ToolError(format!(
            "failed to finalize log path {} -> {}: {}",
            temp_log_path.display(),
            log_path.display(),
            err
        ))
    })?;
    let stdin = child.stdin.take();
    native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?
        .insert(
            session_id.clone(),
            NativeBackgroundProcess {
                child,
                stdin,
                command: command.to_string(),
                workdir: workdir.to_path_buf(),
                log_path: log_path.clone(),
                started_at_unix_ms,
            },
        );
    let mut out = native_compat_base(tool, "backgrounded");
    out.insert(
        "content".into(),
        Value::String(format!(
            "command started in background as {}; use process poll/log/write/kill/clear/remove",
            session_id
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "sessionId": session_id,
            "id": session_id,
            "pid": pid,
            "command": command,
            "workdir": workdir.display().to_string(),
            "log_path": log_path.display().to_string(),
            "running": true,
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"]
        }),
    );
    Ok(out)
}

fn native_process_log_dir() -> PathBuf {
    tool_workspace_root_path().join("target/hepta-processes")
}

fn native_process_log_path(id: &str) -> PathBuf {
    native_process_log_dir().join(format!("{}.log", id))
}

fn native_process_pid_from_id(id: &str) -> Option<u32> {
    id.rsplit_once('-')
        .and_then(|(_, pid)| pid.parse::<u32>().ok())
}

fn native_process_started_at_from_id(id: &str) -> Option<u64> {
    let rest = id.strip_prefix("hepta-proc-")?;
    rest.split_once('-')
        .and_then(|(started, _)| started.parse::<u64>().ok())
}

fn native_process_pid_alive(pid: u32) -> bool {
    std::process::Command::new("/bin/kill")
        .arg("-0")
        .arg(pid.to_string())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn native_process_command_from_log(log_path: &Path) -> String {
    fs::read_to_string(log_path)
        .ok()
        .and_then(|text| {
            text.lines()
                .next()
                .map(|line| line.trim_start_matches("$ ").to_string())
        })
        .unwrap_or_else(|| "<unknown>".into())
}

fn native_process_snapshot_from_log(id: &str) -> Option<Value> {
    let log_path = native_process_log_path(id);
    if !log_path.exists() {
        return None;
    }
    let pid = native_process_pid_from_id(id);
    let running = pid.map(native_process_pid_alive).unwrap_or(false);
    Some(json!({
        "sessionId": id,
        "id": id,
        "pid": pid,
        "command": native_process_command_from_log(&log_path),
        "workdir": tool_workspace_root_path().display().to_string(),
        "log_path": log_path.display().to_string(),
        "started_at_unix_ms": native_process_started_at_from_id(id),
        "running": running,
        "exit_code": null,
        "stdin_open": false,
        "registry_backed": false,
        "log_backed": true,
    }))
}

fn native_process_log_snapshots() -> Vec<Value> {
    let Ok(entries) = fs::read_dir(native_process_log_dir()) else {
        return Vec::new();
    };
    let mut snapshots = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("log") {
                return None;
            }
            let id = path.file_stem().and_then(|stem| stem.to_str())?;
            if !id.starts_with("hepta-proc-") {
                return None;
            }
            native_process_snapshot_from_log(id)
        })
        .collect::<Vec<_>>();
    snapshots.sort_by_key(|snapshot| {
        snapshot
            .get("started_at_unix_ms")
            .and_then(Value::as_u64)
            .unwrap_or(0)
    });
    snapshots
}

fn native_compat_process(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("list");
    match action {
        "list" | "status" => native_process_list(tool, action),
        "poll" => native_process_poll(tool, input),
        "log" | "read" => native_process_log(tool, input),
        "write" | "submit" => native_process_write(tool, input),
        "kill" | "terminate" => native_process_kill(tool, input),
        "clear" | "remove" => native_process_remove(tool, input, action),
        other => Err(hepta_core::ToolError(format!(
            "unsupported process action '{}'; supported actions: list, poll, log, write, kill, clear, remove",
            other
        ))),
    }
}

fn native_process_id(
    input: &serde_json::Map<String, Value>,
) -> Result<String, hepta_core::ToolError> {
    input
        .get("sessionId")
        .or_else(|| input.get("session_id"))
        .or_else(|| input.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| hepta_core::ToolError("process action requires sessionId".into()))
}

fn native_process_snapshot(
    id: &str,
    process: &mut NativeBackgroundProcess,
) -> Result<Value, hepta_core::ToolError> {
    let status = process
        .child
        .try_wait()
        .map_err(|err| hepta_core::ToolError(format!("failed to poll {}: {}", id, err)))?;
    let running = status.is_none();
    let exit_code = status.and_then(|status| status.code());
    Ok(json!({
        "sessionId": id,
        "id": id,
        "pid": process.child.id(),
        "command": process.command,
        "workdir": process.workdir.display().to_string(),
        "log_path": process.log_path.display().to_string(),
        "started_at_unix_ms": process.started_at_unix_ms,
        "running": running,
        "exit_code": exit_code,
        "stdin_open": process.stdin.is_some()
    }))
}

fn native_process_list(
    tool: &str,
    action: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let mut processes = Vec::new();
    for (id, process) in registry.iter_mut() {
        processes.push(native_process_snapshot(id, process)?);
    }
    let registry_ids = registry
        .keys()
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    drop(registry);
    for snapshot in native_process_log_snapshots() {
        let Some(id) = snapshot.get("id").and_then(Value::as_str) else {
            continue;
        };
        if !registry_ids.contains(id) {
            processes.push(snapshot);
        }
    }
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("{} native background process(es)", processes.len())),
    );
    out.insert(
        "result".into(),
        json!({
            "action": action,
            "processes": processes,
            "native_registry_present": true,
            "background_exec_capture_supported": true,
            "log_backed_followup_supported": true,
            "followup_actions": ["poll", "log", "write", "kill", "clear", "remove"]
        }),
    );
    Ok(out)
}

fn native_process_poll(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let timeout_ms = input
        .get("timeout")
        .or_else(|| input.get("timeoutMs"))
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .min(30_000);
    let started = SystemTime::now();
    loop {
        {
            let mut registry = native_process_registry().lock().map_err(|_| {
                hepta_core::ToolError("native process registry lock poisoned".into())
            })?;
            let snapshot = if let Some(process) = registry.get_mut(&id) {
                native_process_snapshot(&id, process)?
            } else {
                native_process_snapshot_from_log(&id).ok_or_else(|| {
                    hepta_core::ToolError(format!("no native background process found for {}", id))
                })?
            };
            if snapshot.get("running").and_then(Value::as_bool) != Some(true) || timeout_ms == 0 {
                let mut out = native_compat_base(tool, "ok");
                out.insert(
                    "content".into(),
                    Value::String(format!(
                        "process {} poll: running={}",
                        id, snapshot["running"]
                    )),
                );
                out.insert(
                    "result".into(),
                    json!({"action":"poll", "process": snapshot}),
                );
                return Ok(out);
            }
        }
        if started
            .elapsed()
            .map(|elapsed| elapsed.as_millis() as u64 >= timeout_ms)
            .unwrap_or(true)
        {
            return native_process_poll(tool, &{
                let mut next = input.clone();
                next.insert("timeout".into(), Value::Number(serde_json::Number::from(0)));
                next
            });
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn native_process_log(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let (log_path, snapshot) = {
        let mut registry = native_process_registry()
            .lock()
            .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
        if let Some(process) = registry.get_mut(&id) {
            (
                process.log_path.clone(),
                native_process_snapshot(&id, process)?,
            )
        } else {
            let snapshot = native_process_snapshot_from_log(&id).ok_or_else(|| {
                hepta_core::ToolError(format!("no native background process found for {}", id))
            })?;
            (native_process_log_path(&id), snapshot)
        }
    };
    let offset = input.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = input.get("limit").and_then(Value::as_u64).unwrap_or(50_000) as usize;
    let bytes = fs::read(&log_path).unwrap_or_default();
    let start = offset.min(bytes.len());
    let end = start.saturating_add(limit).min(bytes.len());
    let text = String::from_utf8_lossy(&bytes[start..end]).to_string();
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(text.clone()));
    out.insert(
        "result".into(),
        json!({
            "action": "log",
            "process": snapshot,
            "offset": offset,
            "next_offset": end,
            "truncated": end < bytes.len(),
            "text": text
        }),
    );
    Ok(out)
}

fn native_process_write(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let data = input
        .get("data")
        .or_else(|| input.get("text"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let eof = input.get("eof").and_then(Value::as_bool).unwrap_or(false);
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let process = registry.get_mut(&id).ok_or_else(|| {
        hepta_core::ToolError(format!("no native background process found for {}", id))
    })?;
    if let Some(stdin) = process.stdin.as_mut() {
        stdin
            .write_all(data.as_bytes())
            .and_then(|_| stdin.flush())
            .map_err(|err| hepta_core::ToolError(format!("failed writing to {}: {}", id, err)))?;
    } else if !data.is_empty() {
        return Err(hepta_core::ToolError(format!("stdin is closed for {}", id)));
    }
    if eof {
        process.stdin.take();
    }
    let snapshot = native_process_snapshot(&id, process)?;
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("wrote {} bytes to {}", data.len(), id)),
    );
    out.insert(
        "result".into(),
        json!({"action":"write", "bytes": data.len(), "eof": eof, "process": snapshot}),
    );
    Ok(out)
}

fn native_process_kill(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let id = native_process_id(input)?;
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let snapshot = if let Some(process) = registry.get_mut(&id) {
        let killed_tree = native_kill_child_process_tree(&mut process.child);
        let snapshot = native_process_snapshot(&id, process)?;
        if !killed_tree && snapshot.get("exit_code").and_then(Value::as_i64).is_none() {
            return Err(hepta_core::ToolError(format!(
                "failed to signal native process tree for {}",
                id
            )));
        }
        snapshot
    } else if let Some(pid) = native_process_pid_from_id(&id) {
        let _ = native_send_signal_to_pid_tree(pid, "-TERM");
        native_process_snapshot_from_log(&id).ok_or_else(|| {
            hepta_core::ToolError(format!("no native background process found for {}", id))
        })?
    } else {
        return Err(hepta_core::ToolError(format!(
            "no native background process found for {}",
            id
        )));
    };
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(format!("killed {}", id)));
    out.insert(
        "result".into(),
        json!({"action":"kill", "process": snapshot}),
    );
    Ok(out)
}

fn native_process_remove(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    action: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let mut registry = native_process_registry()
        .lock()
        .map_err(|_| hepta_core::ToolError("native process registry lock poisoned".into()))?;
    let removed = if action == "clear" && native_process_id(input).is_err() {
        let count = registry.len();
        registry.clear();
        let log_removed = fs::read_dir(native_process_log_dir())
            .ok()
            .into_iter()
            .flat_map(|entries| entries.filter_map(Result::ok))
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("log"))
            .filter(|entry| fs::remove_file(entry.path()).is_ok())
            .count();
        count + log_removed
    } else {
        let id = native_process_id(input)?;
        if let Some(mut process) = registry.remove(&id) {
            if process.child.try_wait().ok().flatten().is_none() {
                let _ = process.child.kill();
                let _ = process.child.wait();
            }
            let _ = fs::remove_file(native_process_log_path(&id));
            1
        } else if native_process_log_path(&id).exists() {
            let _ = fs::remove_file(native_process_log_path(&id));
            1
        } else {
            0
        }
    };
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!(
            "removed {} native process registry entrie(s)",
            removed
        )),
    );
    out.insert(
        "result".into(),
        json!({"action": action, "removed": removed}),
    );
    Ok(out)
}

fn native_compat_web_fetch(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let url = input
        .get("url")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("web_fetch requires string field 'url'".into()))?;
    let output = std::process::Command::new("curl")
        .arg("-L")
        .arg("--max-time")
        .arg("30")
        .arg("--silent")
        .arg("--show-error")
        .arg(url)
        .output()
        .map_err(|err| hepta_core::ToolError(format!("failed to run curl: {}", err)))?;
    let body = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let max_chars = input
        .get("maxChars")
        .or_else(|| input.get("max_chars"))
        .and_then(Value::as_u64)
        .unwrap_or(20_000) as usize;
    let extracted = body.chars().take(max_chars).collect::<String>();
    let mut out = native_compat_base(
        tool,
        if output.status.success() {
            "ok"
        } else {
            "error"
        },
    );
    out.insert("content".into(), Value::String(extracted.clone()));
    out.insert(
        "result".into(),
        json!({
            "url": url,
            "status_code_available": false,
            "text": extracted,
            "stderr": stderr,
            "truncated": body.chars().count() > max_chars
        }),
    );
    Ok(out)
}

fn native_compat_web_search(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let query = input
        .get("query")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("web_search requires string field 'query'".into()))?;
    let encoded = query.replace(' ', "+");
    let url = format!("https://duckduckgo.com/html/?q={}", encoded);
    let mut fetch_input = serde_json::Map::new();
    fetch_input.insert("url".into(), Value::String(url.clone()));
    fetch_input.insert(
        "maxChars".into(),
        Value::Number(serde_json::Number::from(20_000)),
    );
    let fetched = native_compat_web_fetch(tool, &fetch_input)?;
    let text = fetched
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let mut out = native_compat_base(tool, "ok");
    out.insert("content".into(), Value::String(text.clone()));
    out.insert(
        "result".into(),
        json!({ "query": query, "search_url": url, "raw_html_excerpt": text }),
    );
    Ok(out)
}

fn native_compat_memory_search(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let query = input.get("query").and_then(Value::as_str).ok_or_else(|| {
        hepta_core::ToolError("memory_search requires string field 'query'".into())
    })?;
    let max_results = input
        .get("maxResults")
        .or_else(|| input.get("max_results"))
        .and_then(Value::as_u64)
        .unwrap_or(10) as usize;
    let workspace = tool_workspace_root_path();
    let mut hits = Vec::new();
    for path in memory_candidate_paths(&workspace) {
        if let Ok(content) = fs::read_to_string(&path) {
            for (idx, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&query.to_lowercase()) {
                    hits.push(json!({
                        "path": path.display().to_string(),
                        "line": idx + 1,
                        "snippet": line
                    }));
                    if hits.len() >= max_results {
                        break;
                    }
                }
            }
        }
        if hits.len() >= max_results {
            break;
        }
    }
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!("{} memory hit(s)", hits.len())),
    );
    out.insert("result".into(), json!({ "query": query, "hits": hits }));
    Ok(out)
}

fn native_compat_memory_get(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let path = input
        .get("path")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("memory_get requires string field 'path'".into()))?;
    native_compat_read(tool, &{
        let mut mapped = serde_json::Map::new();
        mapped.insert("path".into(), Value::String(path.into()));
        if let Some(from) = input.get("from") {
            mapped.insert("offset".into(), from.clone());
        }
        if let Some(lines) = input.get("lines") {
            mapped.insert("limit".into(), lines.clone());
        }
        mapped
    })
}

fn memory_candidate_paths(workspace: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let root_memory = workspace.join("MEMORY.md");
    if root_memory.is_file() {
        paths.push(root_memory);
    }
    let memory_dir = workspace.join("memory");
    if let Ok(entries) = fs::read_dir(memory_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                paths.push(path);
            }
        }
    }
    paths
}

fn native_compat_status_report(
    tool: &str,
    _input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String("Hepta native runtime status: OpenClaw proxy disabled".into()),
    );
    out.insert(
        "result".into(),
        json!({
            "runtime": "hepta-rust-native",
            "openclaw_proxy_tools_enabled": false,
            "native_openclaw_compatible_tool_count": native_openclaw_compatible_tools().len()
        }),
    );
    out
}

fn native_compat_plan_echo(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String("plan accepted by native Hepta surface".into()),
    );
    out.insert("result".into(), Value::Object(input.clone()));
    out
}

fn native_compat_live_surface(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    match tool {
        "message" => native_compat_message(tool, input),
        "tts" => native_compat_tts(tool, input),
        "image_generate" => native_compat_image_generate(tool, input),
        "music_generate" => {
            native_compat_configured_generator(tool, input, "HEPTA_MUSIC_GENERATE_CMD")
        }
        "video_generate" => {
            native_compat_configured_generator(tool, input, "HEPTA_VIDEO_GENERATE_CMD")
        }
        "image" => native_compat_image_analyze(tool, input),
        "pdf" => native_compat_pdf_analyze(tool, input),
        "agents_list" => native_compat_hepta_cli(tool, &["/agent-pool", "--json"]),
        "sessions_list" => native_compat_hepta_cli(tool, &["/sessions", "--json"]),
        "sessions_history" => native_compat_sessions_history(tool, input),
        "sessions_send" => native_compat_sessions_send(tool, input),
        "sessions_spawn" => native_compat_sessions_spawn(tool, input),
        "sessions_yield" => Ok(native_compat_local_event(tool, input, "yield_recorded")),
        "subagents" => native_compat_subagents(tool, input),
        "canvas" => {
            native_compat_hepta_cli(tool, &["/canvas-plane", "--all", "--sample-run", "--json"])
        }
        "feishu_app_scopes"
        | "feishu_chat"
        | "feishu_doc"
        | "feishu_drive"
        | "feishu_wiki"
        | "feishu_bitable_get_meta"
        | "feishu_bitable_list_fields"
        | "feishu_bitable_list_records"
        | "feishu_bitable_get_record"
        | "feishu_bitable_create_record"
        | "feishu_bitable_update_record"
        | "feishu_bitable_create_app"
        | "feishu_bitable_create_field" => native_compat_feishu(tool, input),
        _ => Ok(native_compat_surface_report(tool, input)),
    }
}

fn native_compat_hepta_cli(
    tool: &str,
    args: &[&str],
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let binary = hepta_cli_binary();
    let output = std::process::Command::new(&binary)
        .args(args)
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to run Hepta native CLI {}: {}",
                binary.display(),
                err
            ))
        })?;
    command_output_to_native_result(tool, &binary.display().to_string(), args, output)
}

fn hepta_cli_binary() -> PathBuf {
    if let Ok(path) = env::var("HEPTA_NATIVE_TOOL_CLI_BIN") {
        return PathBuf::from(path);
    }
    env::current_exe().unwrap_or_else(|_| PathBuf::from("/Users/qianqi/.local/bin/hepta"))
}

fn command_output_to_native_result(
    tool: &str,
    command: &str,
    args: &[&str],
    output: std::process::Output,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let mut out = native_compat_base(
        tool,
        if output.status.success() {
            "ok"
        } else {
            "error"
        },
    );
    let parsed_json = serde_json::from_str::<Value>(&stdout).ok();
    out.insert("content".into(), Value::String(stdout.clone()));
    out.insert(
        "result".into(),
        json!({
            "command": command,
            "args": args,
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": stdout,
            "stderr": stderr,
            "parsed_json": parsed_json,
            "live_adapter_invoked": true,
            "openclaw_proxy_used": false
        }),
    );
    Ok(out)
}

fn native_compat_message(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("send");
    let channel = input
        .get("channel")
        .and_then(Value::as_str)
        .unwrap_or("telegram");
    if action != "send" {
        let args = match action {
            "channel-list" | "channel-info" | "member-info" => {
                ["/telegram-adapter", "--dry-run", "--json"].as_slice()
            }
            _ => ["/telegram-adapter", "--dry-run", "--json"].as_slice(),
        };
        return native_compat_hepta_cli(tool, args);
    }
    let target = input
        .get("target")
        .or_else(|| input.get("to"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("message send requires string field 'target'".into())
        })?;
    let text = input
        .get("message")
        .or_else(|| input.get("text"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("message send requires string field 'message'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let confirm_send = input
        .get("confirmSend")
        .or_else(|| input.get("confirm_send"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if channel != "telegram" {
        return Err(hepta_core::ToolError(format!(
            "message native live send currently supports telegram; requested channel '{}'",
            channel
        )));
    }
    if dry_run || !confirm_send {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(
                "telegram send preview ready; set dryRun=false and confirmSend=true to send".into(),
            ),
        );
        out.insert(
            "result".into(),
            json!({
                "channel": channel,
                "target_shape": redact_identifier_shape(target),
                "message_chars": text.chars().count(),
                "would_send": true,
                "sent": false,
                "requires_confirmSend": true
            }),
        );
        return Ok(out);
    }
    let args = vec![
        "/telegram-adapter".to_string(),
        "--live-send".to_string(),
        "--confirm-send".to_string(),
        "--to".to_string(),
        target.to_string(),
        "--text".to_string(),
        text.to_string(),
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_hepta_cli_owned(
    tool: &str,
    args: &[String],
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
    native_compat_hepta_cli(tool, &borrowed)
}

fn native_compat_tts(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let text = input
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("tts requires string field 'text'".into()))?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let target = input
        .get("path")
        .or_else(|| input.get("filename"))
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            tool_workspace_root_path().join(format!(
                "target/hepta-tts-{}-{}.aiff",
                std::process::id(),
                current_unix_ms().unwrap_or(0)
            ))
        });
    let target = resolve_path_within_root(&tool_workspace_root_path(), &target);
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!(
                "would synthesize {} chars to {}",
                text.chars().count(),
                target.display()
            )),
        );
        out.insert("result".into(), json!({"path": target.display().to_string(), "chars": text.chars().count(), "dryRun": true}));
        return Ok(out);
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            hepta_core::ToolError(format!("failed to create {}: {}", parent.display(), err))
        })?;
    }
    let output = std::process::Command::new("say")
        .arg("-o")
        .arg(&target)
        .arg(text)
        .output()
        .map_err(|err| hepta_core::ToolError(format!("failed to run macOS say: {}", err)))?;
    let mut out = command_output_to_native_result(
        tool,
        "say",
        &["-o", "<redacted-path>", "<redacted-text>"],
        output,
    )?;
    out.insert(
        "content".into(),
        Value::String(format!("synthesized speech to {}", target.display())),
    );
    out.insert("result".into(), json!({"path": target.display().to_string(), "chars": text.chars().count(), "format": "aiff", "live_adapter_invoked": true}));
    Ok(out)
}

fn native_compat_image_generate(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let prompt = input
        .get("prompt")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("image_generate requires string field 'prompt'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let script = image_generation_helper_script();
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String("would invoke local Ollama image generation helper".into()),
        );
        out.insert("result".into(), json!({"script": script.as_ref().map(|path| path.display().to_string()), "env_fallback": "HEPTA_IMAGE_GENERATE_CMD", "prompt_chars": prompt.chars().count(), "dryRun": true}));
        return Ok(out);
    }
    let Some(script) = script else {
        return native_compat_configured_generator(tool, input, "HEPTA_IMAGE_GENERATE_CMD");
    };
    let output = std::process::Command::new(&script)
        .arg(prompt)
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!(
                "failed to run image helper {}: {}",
                script.display(),
                err
            ))
        })?;
    command_output_to_native_result(
        tool,
        &script.display().to_string(),
        &["<redacted-prompt>"],
        output,
    )
}

fn image_generation_helper_script() -> Option<PathBuf> {
    let root = tool_workspace_root_path();
    let candidates = [
        root.join("ollama-image-generation/generate.sh"),
        root.parent()
            .unwrap_or(root.as_path())
            .join("ollama-image-generation/generate.sh"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn native_compat_configured_generator(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    env_name: &str,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let prompt = input
        .get("prompt")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError(format!("{} requires string field 'prompt'", tool)))?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if dry_run {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would invoke configured generator {}", env_name)),
        );
        out.insert(
            "result".into(),
            json!({"env": env_name, "prompt_chars": prompt.chars().count(), "dryRun": true}),
        );
        return Ok(out);
    }
    let command = env::var(env_name).map_err(|_| {
        hepta_core::ToolError(format!(
            "{} has no native provider command configured; set {} to a local generator command that accepts the prompt as argv[1]",
            tool, env_name
        ))
    })?;
    let output = std::process::Command::new("/bin/zsh")
        .arg("-lc")
        .arg(format!("{} -- {}", command, shell_quote(prompt)))
        .current_dir(tool_workspace_root_path())
        .output()
        .map_err(|err| {
            hepta_core::ToolError(format!("failed to run configured generator: {}", err))
        })?;
    command_output_to_native_result(
        tool,
        env_name,
        &["<configured-command>", "<redacted-prompt>"],
        output,
    )
}

fn native_compat_image_analyze(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let image_path = input
        .get("image")
        .and_then(Value::as_str)
        .or_else(|| {
            input
                .get("images")
                .and_then(Value::as_array)
                .and_then(|arr| arr.first())
                .and_then(Value::as_str)
        })
        .ok_or_else(|| {
            hepta_core::ToolError("image requires 'image' or first item in 'images'".into())
        })?;
    let path = resolve_path_within_root(&tool_workspace_root_path(), Path::new(image_path));
    let metadata = fs::metadata(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to stat {}: {}", path.display(), err))
    })?;
    let file_output = std::process::Command::new("file").arg(&path).output().ok();
    let sips_output = std::process::Command::new("sips")
        .args(["-g", "pixelWidth", "-g", "pixelHeight"])
        .arg(&path)
        .output()
        .ok();
    let file_text = file_output
        .as_ref()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let sips_text = sips_output
        .as_ref()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(format!(
            "image metadata for {}: {}",
            path.display(),
            file_text.trim()
        )),
    );
    out.insert("result".into(), json!({"path": path.display().to_string(), "bytes": metadata.len(), "file": file_text, "sips": sips_text, "vision_model_invoked": false, "local_metadata_analyzed": true}));
    Ok(out)
}

fn native_compat_pdf_analyze(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let pdf_path = input
        .get("pdf")
        .and_then(Value::as_str)
        .or_else(|| {
            input
                .get("pdfs")
                .and_then(Value::as_array)
                .and_then(|arr| arr.first())
                .and_then(Value::as_str)
        })
        .ok_or_else(|| {
            hepta_core::ToolError("pdf requires 'pdf' or first item in 'pdfs'".into())
        })?;
    let path = resolve_path_within_root(&tool_workspace_root_path(), Path::new(pdf_path));
    let metadata = fs::metadata(&path).map_err(|err| {
        hepta_core::ToolError(format!("failed to stat {}: {}", path.display(), err))
    })?;
    let max_chars = input
        .get("maxChars")
        .or_else(|| input.get("max_chars"))
        .and_then(Value::as_u64)
        .unwrap_or(20_000) as usize;
    let text_output = std::process::Command::new("pdftotext")
        .arg(&path)
        .arg("-")
        .output()
        .ok();
    let extracted = text_output
        .as_ref()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .chars()
                .take(max_chars)
                .collect::<String>()
        })
        .unwrap_or_default();
    let mut out = native_compat_base(tool, "ok");
    out.insert(
        "content".into(),
        Value::String(if extracted.is_empty() {
            format!(
                "pdf metadata for {}; pdftotext unavailable or returned no text",
                path.display()
            )
        } else {
            extracted.clone()
        }),
    );
    out.insert("result".into(), json!({"path": path.display().to_string(), "bytes": metadata.len(), "text": extracted, "text_extracted": !extracted.is_empty(), "external_model_invoked": false}));
    Ok(out)
}

fn native_compat_sessions_history(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let session = input
        .get("sessionKey")
        .or_else(|| input.get("session_id"))
        .and_then(Value::as_str)
        .unwrap_or("session-main");
    let limit = input
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(20)
        .to_string();
    let args = vec![
        "/history".to_string(),
        session.to_string(),
        "--limit".to_string(),
        limit,
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_sessions_send(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let session = input
        .get("sessionKey")
        .or_else(|| input.get("session_id"))
        .or_else(|| input.get("label"))
        .and_then(Value::as_str)
        .unwrap_or("session-main");
    let message = input
        .get("message")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("sessions_send requires string field 'message'".into())
        })?;
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let execute = input
        .get("execute")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if dry_run || !execute {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would run prompt in session {}", session)),
        );
        out.insert("result".into(), json!({"session": session, "message_chars": message.chars().count(), "would_execute": true}));
        return Ok(out);
    }
    let args = vec![
        "/run-in".to_string(),
        session.to_string(),
        message.to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_sessions_spawn(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let task = input
        .get("task")
        .or_else(|| input.get("message"))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            hepta_core::ToolError("sessions_spawn requires string field 'task'".into())
        })?;
    let worker = input
        .get("agentId")
        .or_else(|| input.get("worker_id"))
        .and_then(Value::as_str)
        .unwrap_or("native-tool-worker");
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let execute = input
        .get("execute")
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if dry_run || !execute {
        let mut out = native_compat_base(tool, "preview");
        out.insert(
            "content".into(),
            Value::String(format!("would spawn task for worker {}", worker)),
        );
        out.insert(
            "result".into(),
            json!({"worker_id": worker, "task_chars": task.chars().count(), "would_execute": true}),
        );
        return Ok(out);
    }
    let args = vec![
        "/spawn-task".to_string(),
        worker.to_string(),
        task.to_string(),
        "--json".to_string(),
    ];
    native_compat_hepta_cli_owned(tool, &args)
}

fn native_compat_subagents(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("list");
    match action {
        "list" => native_compat_hepta_cli(tool, &["/agent-pool", "--json"]),
        "steer" => {
            let target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires target".into()))?;
            let message = input
                .get("message")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires message".into()))?;
            let args = vec![
                "/agent-steer".to_string(),
                target.to_string(),
                message.to_string(),
                "--json".to_string(),
            ];
            native_compat_hepta_cli_owned(tool, &args)
        }
        "kill" | "stop" => {
            let target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents stop requires target".into()))?;
            let args = vec![
                "/agent-stop".to_string(),
                target.to_string(),
                "--json".to_string(),
            ];
            native_compat_hepta_cli_owned(tool, &args)
        }
        other => Err(hepta_core::ToolError(format!(
            "unsupported subagents action '{}'",
            other
        ))),
    }
}

fn native_compat_feishu(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let dry_run = input
        .get("dryRun")
        .or_else(|| input.get("dry_run"))
        .and_then(Value::as_bool)
        .unwrap_or(true);
    let live_probe = input
        .get("liveProbe")
        .or_else(|| input.get("live_probe"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if dry_run || !live_probe {
        return native_compat_hepta_cli(tool, &["/feishu-adapter", "--dry-run", "--json"]);
    }
    native_compat_hepta_cli(tool, &["/feishu-adapter", "--live-probe", "--json"])
}

fn native_compat_local_event(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    status: &str,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, status);
    out.insert(
        "content".into(),
        Value::String(format!("{} accepted by native Hepta runtime", tool)),
    );
    out.insert("result".into(), Value::Object(input.clone()));
    out
}

fn redact_identifier_shape(value: &str) -> String {
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        format!("numeric:{}", value.len())
    } else if value.contains('@') {
        "handle:<redacted>".into()
    } else {
        format!("text:{}", value.chars().count())
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn native_compat_surface_report(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> serde_json::Map<String, Value> {
    let mut out = native_compat_base(tool, "native_surface_registered");
    out.insert(
        "content".into(),
        Value::String(format!(
            "{} is registered as a Hepta Rust-native tool surface; OpenClaw proxy is disabled for this tool",
            tool
        )),
    );
    out.insert(
        "result".into(),
        json!({
            "input_keys": input.keys().cloned().collect::<Vec<_>>(),
            "native_surface_registered": true,
            "provider_adapter_required_for_live_side_effects": true
        }),
    );
    out
}

fn validate_against_schema_json(
    schema_name: &str,
    schema_kind: &str,
    schema_json: &str,
    payload_json: &str,
) -> Result<(), HeptaError> {
    let schema_value: Value = serde_json::from_str(schema_json).map_err(|err| {
        HeptaError(format!(
            "invalid {} schema for {}: {}",
            schema_kind, schema_name, err
        ))
    })?;
    let input_value: Value = serde_json::from_str(payload_json).map_err(|err| {
        HeptaError(format!(
            "invalid JSON {} for {}: {}",
            schema_kind, schema_name, err
        ))
    })?;

    match schema_value.get("type").and_then(Value::as_str) {
        Some("object") => {
            validate_object_schema(schema_name, schema_kind, &schema_value, &input_value)
        }
        Some(other) => Err(HeptaError(format!(
            "unsupported root {} schema type for {}: {}",
            schema_kind, schema_name, other
        ))),
        None => Err(HeptaError(format!(
            "{} schema missing root type for {}",
            schema_kind, schema_name
        ))),
    }
}

fn ensure_tool_schema_has_field(
    schema_json: &str,
    tool_name: &str,
    field: &str,
) -> Result<(), HeptaError> {
    let schema_value: Value = serde_json::from_str(schema_json)
        .map_err(|err| HeptaError(format!("invalid input schema for {}: {}", tool_name, err)))?;
    let properties = schema_value
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| HeptaError(format!("tool {} schema is missing properties", tool_name)))?;
    if properties.contains_key(field) {
        Ok(())
    } else {
        Err(HeptaError(format!(
            "tool {} input schema has no field '{}'",
            tool_name, field
        )))
    }
}

fn path_argument_name_for_tool(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "read_file" | "list_dir" | "search_text" => Some("path"),
        _ => None,
    }
}

fn write_path_argument_name_for_tool(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "write_file" => Some("path"),
        _ => None,
    }
}

fn preview_backup_path(
    workspace_root: &Path,
    target_path: &Path,
) -> Result<PathBuf, hepta_core::ToolError> {
    let backup_root = workspace_root.join("artifacts/backups/write_file");
    let relative = if target_path.starts_with(workspace_root) {
        PathBuf::from("workspace").join(
            target_path
                .strip_prefix(workspace_root)
                .unwrap_or(target_path),
        )
    } else {
        let external = target_path
            .strip_prefix(Path::new("/"))
            .unwrap_or(target_path);
        PathBuf::from("external").join(external)
    };

    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            hepta_core::ToolError(format!(
                "cannot derive backup file name for {}",
                target_path.display()
            ))
        })?;
    let start_ts = current_unix_ms().map_err(|err| hepta_core::ToolError(err.0))?;
    preview_backup_path_from_ts(&backup_root, &relative, file_name, start_ts)
}

fn preview_backup_path_from_ts(
    backup_root: &Path,
    relative: &Path,
    file_name: &str,
    start_ts: u64,
) -> Result<PathBuf, hepta_core::ToolError> {
    let backup_dir = backup_root.join(relative.parent().unwrap_or_else(|| Path::new("")));
    let mut ts = start_ts;

    loop {
        let candidate = backup_dir.join(format!("{}.hepta-bak-{}", file_name, ts));
        if !candidate.exists() {
            return Ok(candidate);
        }
        ts = ts.checked_add(1).ok_or_else(|| {
            hepta_core::ToolError(format!(
                "backup timestamp overflow while planning path for {}",
                relative.display()
            ))
        })?;
    }
}

fn preview_transaction_checkpoint_path(
    workspace_root: &Path,
    target_path: &Path,
    transaction_id: &str,
) -> Result<PathBuf, HeptaError> {
    let checkpoint_root = workspace_root.join("artifacts/checkpoints/write_txn");
    let relative = if target_path.starts_with(workspace_root) {
        PathBuf::from("workspace").join(
            target_path
                .strip_prefix(workspace_root)
                .unwrap_or(target_path),
        )
    } else {
        let external = target_path
            .strip_prefix(Path::new("/"))
            .unwrap_or(target_path);
        PathBuf::from("external").join(external)
    };

    let file_name = relative
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            HeptaError(format!(
                "cannot derive transaction checkpoint name for {}",
                target_path.display()
            ))
        })?;
    Ok(checkpoint_root
        .join(relative.parent().unwrap_or_else(|| Path::new("")))
        .join(format!(
            "{}.hepta-txn-{}.checkpoint",
            file_name, transaction_id
        )))
}

fn summarize_write_change(
    mode: &str,
    existed_before: bool,
    content_changed: bool,
    bytes_before: usize,
    bytes_after: usize,
) -> String {
    match (mode, existed_before) {
        ("create", false) => format!("create new file (0 -> {} bytes)", bytes_after),
        ("create", true) => format!(
            "create would fail because target already exists ({} bytes)",
            bytes_before
        ),
        ("overwrite", false) => format!(
            "overwrite will create new file (0 -> {} bytes)",
            bytes_after
        ),
        ("overwrite", true) if content_changed => {
            format!(
                "overwrite existing file ({} -> {} bytes)",
                bytes_before, bytes_after
            )
        }
        ("overwrite", true) => format!(
            "overwrite existing file with identical content ({} bytes)",
            bytes_before
        ),
        ("append", false) => format!("append will create new file (0 -> {} bytes)", bytes_after),
        ("append", true) => format!(
            "append to existing file ({} -> {} bytes)",
            bytes_before, bytes_after
        ),
        _ => format!(
            "write operation {} ({} -> {} bytes)",
            mode, bytes_before, bytes_after
        ),
    }
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), HeptaError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|err| {
        HeptaError(format!(
            "failed to read backup directory {}: {}",
            dir.display(),
            err
        ))
    })? {
        let entry =
            entry.map_err(|err| HeptaError(format!("failed to read backup dir entry: {}", err)))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn looks_like_disk_junk_audit_intent(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    let cleanup_words = input.contains("垃圾")
        || input.contains("清理")
        || input.contains("空间")
        || input.contains("磁盘")
        || input.contains("硬盘")
        || lower.contains("junk")
        || lower.contains("cleanup")
        || lower.contains("clean up")
        || lower.contains("disk")
        || lower.contains("cache")
        || lower.contains("storage");
    let scan_words = input.contains("扫")
        || input.contains("看看")
        || input.contains("审计")
        || input.contains("检查")
        || lower.contains("scan")
        || lower.contains("audit")
        || lower.contains("check");
    cleanup_words && scan_words
}

fn native_pre_model_tool_call(input: &str) -> Option<ToolCall> {
    if let Some(tool_call) = extract_explicit_echo_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_exec_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_process_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(tool_call) = extract_explicit_write_file_tool_call(input) {
        return Some(tool_call);
    }
    if let Some(path) = extract_read_intent_path(input) {
        return Some(ToolCall {
            name: "read".into(),
            arguments_json: json!({
                "path": path,
                "offset": 1,
                "limit": 80,
            })
            .to_string(),
        });
    }
    if looks_like_disk_junk_audit_intent(input) {
        return Some(ToolCall {
            name: "disk_junk_audit".into(),
            arguments_json: json!({
                "scope": "common_local_cleanup_candidates",
                "max_entries": 120000,
            })
            .to_string(),
        });
    }
    None
}

fn should_offer_model_tools_for_turn(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_lower = lower.split_whitespace().collect::<String>();

    if native_pre_model_tool_call(user_text).is_some()
        || extract_read_intent_path(user_text).is_some()
        || looks_like_disk_junk_audit_intent(user_text)
    {
        return true;
    }

    if [
        "tool:",
        "read:",
        "write:",
        "overwrite:",
        "append:",
        "preview-write:",
    ]
    .iter()
    .any(|prefix| lower.starts_with(prefix))
    {
        return true;
    }

    let explicit_tool_action = [
        "use",
        "call",
        "invoke",
        "run",
        "execute",
        "调用",
        "使用",
        "请用",
        "帮我用",
        "直接用",
        "必须用",
        "通过",
        "执行",
        "运行",
    ]
    .iter()
    .any(|verb| lower.contains(verb) || user_text.contains(verb));
    let explicit_tool_surface = [
        " tool",
        "tool ",
        "工具",
        "openclaw_",
        "hepta_",
        "write_file",
        "read_file",
        "web_search",
        "web_fetch",
        "process",
        "exec",
        "sessions_",
        "message",
    ]
    .iter()
    .any(|needle| lower.contains(needle) || user_text.contains(needle));
    if explicit_tool_action && explicit_tool_surface {
        return true;
    }

    [
        "calltool",
        "usetool",
        "invoketool",
        "runtool",
        "executetool",
    ]
    .iter()
    .any(|needle| compact_lower.contains(needle))
}

fn model_identity_response(input: &str, active_model: &ModelRef) -> Option<String> {
    if !looks_like_model_identity_intent(input) {
        return None;
    }
    Some(format!(
        "当前会话使用的模型是 `{}/{}`。这次是 Hepta Rust-native runtime 直接读取会话模型绑定，没有调用工具。",
        active_model.provider, active_model.model
    ))
}

fn deterministic_runtime_response(
    input: &str,
    active_model: &ModelRef,
    messages: &[ModelMessage],
) -> Option<String> {
    if let Some(response) = model_identity_response(input, active_model) {
        return Some(response);
    }
    if let Some(response) = assistant_identity_response(input) {
        return Some(response);
    }
    deterministic_memory_marker_response(input, messages)
}

fn assistant_identity_response(input: &str) -> Option<String> {
    if !looks_like_assistant_identity_intent(input) {
        return None;
    }
    Some(
        "我是发发_1，Hepta Telegram 里的 Rust-native 助手实例。当前这条消息由 Hepta runtime 直接按身份问答处理，没有调用工具。"
            .to_string(),
    )
}

fn deterministic_memory_marker_response(input: &str, messages: &[ModelMessage]) -> Option<String> {
    let user_text = hepta_agent_body_or_input(input);
    if !looks_like_live_agent_marker_recall_intent(user_text) {
        return None;
    }
    let context = messages
        .iter()
        .filter(|message| matches!(message.role, MessageRole::System))
        .map(|message| message.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let marker = extract_recent_transcript_live_agent_e2e_marker(&context)
        .or_else(|| extract_live_agent_e2e_marker(&context))?;
    Some(format!("The live-agent-e2e marker is {marker}."))
}

fn looks_like_live_agent_marker_recall_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let has_marker_surface = lower.contains("live-agent-e2e marker")
        || lower.contains("live_agent_e2e marker")
        || lower.contains("live-agent-e2e-marker")
        || lower.contains("live_agent_e2e_marker");
    if !has_marker_surface || looks_like_live_agent_marker_remember_intent(user_text) {
        return false;
    }
    lower.contains("what")
        || lower.contains("which")
        || lower.contains("recall")
        || lower.contains("remembered")
        || user_text.contains("是什么")
        || user_text.contains("是多少")
        || user_text.contains("告诉我")
        || user_text.contains("读回")
}

fn looks_like_live_agent_marker_remember_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    (lower.contains("remember")
        || user_text.contains("记住")
        || user_text.contains("保存")
        || user_text.contains("写入"))
        && (lower.contains("live-agent-e2e marker")
            || lower.contains("live_agent_e2e marker")
            || lower.contains("live-agent-e2e-marker")
            || lower.contains("live_agent_e2e_marker"))
}

fn extract_recent_transcript_live_agent_e2e_marker(context: &str) -> Option<String> {
    let transcript = context.split_once("Recent session transcript:\n")?.1;
    let transcript = transcript
        .split_once("\n\nRelevant memory records:")
        .map(|(before, _)| before)
        .unwrap_or(transcript);
    extract_live_agent_e2e_marker(transcript)
}

fn extract_live_agent_e2e_marker(input: &str) -> Option<String> {
    let prefix = "hepta-live-agent-e2e-";
    let mut rest = input;
    let mut latest = None::<String>;
    let mut latest_numeric_suffix = 0_u64;
    while let Some(index) = rest.find(prefix) {
        let candidate = rest[index..]
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
            .collect::<String>();
        let suffix = candidate.strip_prefix(prefix).unwrap_or_default();
        let numeric_suffix = suffix
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .ok();
        if let Some(numeric_suffix) = numeric_suffix
            && numeric_suffix >= latest_numeric_suffix
        {
            latest_numeric_suffix = numeric_suffix;
            latest = Some(candidate);
        }
        rest = &rest[index + prefix.len()..];
    }
    latest
}

fn looks_like_model_identity_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_lower = lower.split_whitespace().collect::<String>();
    let compact_text = user_text.split_whitespace().collect::<String>();
    let model_surface = user_text.contains("模型") || lower.contains("model");
    if !model_surface {
        return false;
    }
    let menu_or_mutation_intent = [
        "模型列表",
        "可用模型",
        "切换模型",
        "选择模型",
        "换模型",
        "/model",
        "/model-in",
        "model list",
        "available model",
        "switch model",
        "select model",
        "change model",
    ]
    .iter()
    .any(|needle| lower.contains(needle) || compact_text.contains(needle));
    if menu_or_mutation_intent {
        return false;
    }
    [
        "你是什么模型",
        "你是哪个模型",
        "你用什么模型",
        "你用的什么模型",
        "你用的是哪个模型",
        "你现在是什么模型",
        "你现在用什么模型",
        "你接的什么模型",
        "现在是什么模型",
        "当前是什么模型",
        "当前模型是什么",
        "什么模型",
    ]
    .iter()
    .any(|needle| compact_text.contains(needle))
        || [
            "whatmodelareyou",
            "whichmodelareyou",
            "whatmodeldoyouuse",
            "whichmodeldoyouuse",
            "currentmodel",
            "activemodel",
        ]
        .iter()
        .any(|needle| compact_lower.contains(needle))
}

fn looks_like_assistant_identity_intent(input: &str) -> bool {
    let user_text = hepta_agent_body_or_input(input).trim();
    if user_text.is_empty() {
        return false;
    }
    let lower = user_text.to_ascii_lowercase();
    let compact_text = user_text.split_whitespace().collect::<String>();
    let compact_lower = lower.split_whitespace().collect::<String>();

    if [
        "你是谁",
        "你是誰",
        "你叫什么",
        "你叫什麼",
        "你叫什么名字",
        "你叫什麼名字",
        "你是哪位",
        "你是什么",
        "你是什麼",
    ]
    .iter()
    .any(|needle| compact_text.contains(needle))
    {
        return true;
    }

    [
        "whoareyou",
        "whatareyou",
        "whatisyourname",
        "what'syourname",
        "tellmewhoyouare",
    ]
    .iter()
    .any(|needle| compact_lower.contains(needle))
}

fn extract_explicit_write_file_tool_call(input: &str) -> Option<ToolCall> {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let explicit_write_file = lower.contains("write_file tool")
        || lower.contains("use write_file")
        || lower.contains("call write_file")
        || lower.contains("write_file 工具")
        || user_text.contains("调用 write_file")
        || user_text.contains("用 write_file");
    if !explicit_write_file {
        return None;
    }
    let start = user_text.find('{')?;
    let end = user_text.rfind('}')?;
    if end <= start {
        return None;
    }
    let args: Value = serde_json::from_str(&user_text[start..=end]).ok()?;
    let object = args.as_object()?;
    if !object.contains_key("path") || !object.contains_key("content") {
        return None;
    }
    Some(ToolCall {
        name: "write_file".into(),
        arguments_json: args.to_string(),
    })
}

fn extract_explicit_echo_tool_call(input: &str) -> Option<ToolCall> {
    let user_text = hepta_agent_body_or_input(input);
    let lower = user_text.to_ascii_lowercase();
    let explicit_echo = lower.contains("echo tool")
        || lower.contains("use echo")
        || lower.contains("call echo")
        || user_text.contains("echo 工具")
        || user_text.contains("调用 echo")
        || user_text.contains("用 echo")
        || user_text.contains("通过 echo");
    if !explicit_echo {
        return None;
    }
    let text = extract_echo_text_argument(user_text)?;
    Some(ToolCall {
        name: "echo".into(),
        arguments_json: json!({ "text": text }).to_string(),
    })
}

fn extract_echo_text_argument(input: &str) -> Option<String> {
    if let Some(value) = extract_json_string_field(input, "text") {
        return Some(value);
    }
    for marker in [
        "内容是",
        "内容为",
        "返回",
        "输出",
        "text exactly",
        "message exactly",
        "message:",
        "text:",
        "text=",
        "say:",
        "return",
    ] {
        if let Some((_, after)) = input.split_once(marker) {
            let text = trim_echo_clause(after);
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

fn extract_json_string_field(input: &str, field: &str) -> Option<String> {
    let field_marker = format!("\"{}\"", field);
    let field_index = input.find(&field_marker)?;
    let after_field = &input[field_index + field_marker.len()..];
    let colon_index = after_field.find(':')?;
    let after_colon = after_field[colon_index + 1..].trim_start();
    let rest = after_colon.strip_prefix('"')?;
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest.chars() {
        if escaped {
            value.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return (!value.trim().is_empty()).then(|| value.trim().to_string());
        } else {
            value.push(ch);
        }
    }
    None
}

fn trim_echo_clause(input: &str) -> String {
    let mut clause = input.trim();
    for separator in [
        "，不要",
        "。不要",
        "；不要",
        ", do not",
        ". do not",
        "; do not",
        "不要",
        "do not",
        "without",
        "并且",
        "然后",
    ] {
        if let Some((before, _)) = clause.split_once(separator) {
            clause = before.trim();
        }
    }
    clause
        .trim_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    '`' | '"' | '\'' | '“' | '”' | '。' | '，' | ',' | ';' | '；' | ':' | '：'
                )
        })
        .to_string()
}

fn hepta_agent_body_or_input(input: &str) -> &str {
    input
        .rsplit_once("BodyForHeptaAgent:")
        .map(|(_, body)| body.trim())
        .unwrap_or(input)
}

fn extract_explicit_exec_tool_call(input: &str) -> Option<ToolCall> {
    let lower = input.to_ascii_lowercase();
    let explicit_exec = lower.contains("exec 工具")
        || lower.contains("exec tool")
        || lower.contains("调用 exec")
        || lower.contains("use exec")
        || lower.contains("用 exec")
        || lower.contains("通过 exec")
        || lower.contains("run with exec");
    if !explicit_exec {
        return None;
    }
    let command = extract_exec_command_text(input)?;
    let background = lower.contains("background=true")
        || lower.contains("background: true")
        || lower.contains("后台")
        || lower.contains("background")
        || lower.contains("异步");
    let mut arguments = json!({
        "command": command,
        "background": background,
    });
    if let Some(timeout_ms) = extract_exec_timeout_ms(input) {
        arguments["timeoutMs"] = Value::Number(serde_json::Number::from(timeout_ms));
    }
    Some(ToolCall {
        name: "exec".into(),
        arguments_json: arguments.to_string(),
    })
}

fn extract_exec_timeout_ms(input: &str) -> Option<u64> {
    for marker in ["timeoutMs=", "timeoutMs:", "timeout_ms=", "timeout_ms:"] {
        if let Some((_, after)) = input.split_once(marker) {
            let digits = after
                .trim_start()
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>();
            if let Ok(value) = digits.parse::<u64>() {
                return Some(value);
            }
        }
    }
    None
}

fn extract_exec_command_text(input: &str) -> Option<String> {
    let trimmed = input.trim();
    for marker in [
        "命令：",
        "命令:",
        "运行：",
        "运行:",
        "执行：",
        "执行:",
        "command:",
        "run:",
        "exec:",
    ] {
        if let Some((_, after)) = trimmed.split_once(marker) {
            let candidate = trim_command_clause(after);
            if !candidate.is_empty() {
                return Some(candidate);
            }
        }
    }

    let lower = trimmed.to_ascii_lowercase();
    if let Some(index) = lower.find("exec") {
        let after = trimmed[index + "exec".len()..].trim_start_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(ch, '工' | '具' | ':' | '：' | '-' | '—' | '，' | ',' | '。')
        });
        let candidate = trim_command_clause(after);
        if !candidate.is_empty() {
            return Some(candidate);
        }
    }
    None
}

fn trim_command_clause(input: &str) -> String {
    let mut clause = input.trim();
    for separator in [
        "；timeoutMs",
        "; timeoutMs",
        " timeoutMs=",
        " timeout_ms=",
        "；timeout",
        "; timeout",
        "；然后",
        "; then",
        "然后",
        "再调用",
        "再用",
        " and then ",
    ] {
        if let Some((before, _)) = clause.split_once(separator) {
            clause = before.trim();
        }
    }
    clause
        .trim_matches(|ch: char| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    '`' | '"' | '\'' | '“' | '”' | '。' | '，' | ',' | ';' | '；'
                )
        })
        .to_string()
}

fn extract_explicit_process_tool_call(input: &str) -> Option<ToolCall> {
    let lower = input.to_ascii_lowercase();
    let explicit_process = lower.contains("process 工具")
        || lower.contains("process tool")
        || lower.contains("调用 process")
        || lower.contains("use process")
        || lower.contains("用 process")
        || lower.contains("通过 process");
    if !explicit_process {
        return None;
    }
    let action = if lower.contains("poll") || input.contains("轮询") || input.contains("状态") {
        "poll"
    } else if lower.contains("log") || input.contains("日志") || input.contains("输出") {
        "log"
    } else if lower.contains("write") || input.contains("写入") || input.contains("输入") {
        "write"
    } else if lower.contains("kill") || input.contains("终止") || input.contains("杀掉") {
        "kill"
    } else if lower.contains("clear") || lower.contains("remove") || input.contains("清除") {
        "clear"
    } else {
        "list"
    };
    let mut args = json!({"action": action});
    if let Some(session_id) = extract_hepta_process_id(input) {
        args["sessionId"] = Value::String(session_id);
    }
    Some(ToolCall {
        name: "process".into(),
        arguments_json: args.to_string(),
    })
}

fn extract_hepta_process_id(input: &str) -> Option<String> {
    input
        .split(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '"' | '\'' | ',' | ';'))
        .find_map(|token| {
            let cleaned = token.trim_matches(|ch: char| {
                matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | '。' | '，')
            });
            if cleaned.starts_with("hepta-proc-") {
                Some(cleaned.to_string())
            } else {
                None
            }
        })
}

fn extract_read_intent_path(input: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let looks_like_read = input.contains("读取")
        || input.contains("读一下")
        || input.contains("打开")
        || lower.contains("read ")
        || lower.contains("cat ")
        || lower.contains("show ");
    if !looks_like_read {
        return None;
    }
    for raw in input.split(|ch: char| {
        ch.is_whitespace()
            || matches!(
                ch,
                '，' | '。' | '；' | ';' | ',' | '：' | ':' | '"' | '\'' | '`' | '“' | '”'
            )
    }) {
        let token = raw.trim_matches(|ch: char| {
            matches!(
                ch,
                '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '。' | '，' | ',' | ';' | '；'
            )
        });
        if token.is_empty() {
            continue;
        }
        let token_lower = token.to_ascii_lowercase();
        let path_like = token.contains('/')
            || [
                ".md", ".txt", ".json", ".rs", ".toml", ".yaml", ".yml", ".log", ".csv",
            ]
            .iter()
            .any(|suffix| token_lower.ends_with(suffix));
        if path_like {
            return Some(token.to_string());
        }
    }
    None
}

fn disk_junk_candidate_roots(
    include_var_folders: bool,
) -> Vec<(PathBuf, &'static str, &'static str)> {
    let mut roots = Vec::new();
    if let Some(home) = env::var_os("HOME").map(PathBuf::from) {
        roots.push((
            home.join("Library/Caches"),
            "user_cache",
            "通常可清理，但应先关闭相关应用；优先清理内容而不是删除目录本身。",
        ));
        roots.push((
            home.join("Library/Logs"),
            "user_logs",
            "旧日志通常可删；近期日志建议保留以便排障。",
        ));
        roots.push((
            home.join(".cache"),
            "unix_user_cache",
            "常见 CLI/开发工具缓存；建议按子目录确认后清理。",
        ));
        roots.push((
            home.join("Library/Developer/Xcode/DerivedData"),
            "xcode_derived_data",
            "Xcode 派生数据可重建；确认没有正在构建后再清理。",
        ));
        roots.push((
            home.join("Library/Developer/CoreSimulator/Caches"),
            "simulator_cache",
            "模拟器缓存可重建；先停止模拟器。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/Cache"),
            "vscode_cache",
            "编辑器缓存；关闭 VS Code 后再清理更稳。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/CachedData"),
            "vscode_cached_data",
            "VS Code 可重建缓存；关闭应用后再清理。",
        ));
        roots.push((
            home.join("Library/Application Support/Code/User/workspaceStorage"),
            "vscode_workspace_storage",
            "可能含工作区状态；只建议清理确认不用的旧工作区条目。",
        ));
        roots.push((
            home.join(".npm/_cacache"),
            "npm_cache",
            "npm 缓存可重建；可用 npm cache verify/clean 管理。",
        ));
        roots.push((
            home.join(".cargo/registry/cache"),
            "cargo_registry_cache",
            "Rust registry 包缓存可重拉；清理会导致后续构建重新下载。",
        ));
        roots.push((
            home.join(".cargo/git/checkouts"),
            "cargo_git_checkouts",
            "Cargo git checkout 缓存；清理会导致后续构建重新拉取。",
        ));
    }
    roots.push((
        env::temp_dir(),
        "temp_dir",
        "临时目录可能有正在使用的文件；只清理明显过期条目。",
    ));
    roots.push((
        PathBuf::from("/Library/Caches"),
        "system_cache",
        "系统级缓存需要更谨慎，通常不建议自动删除。",
    ));
    if include_var_folders {
        roots.push((
            PathBuf::from("/private/var/folders"),
            "darwin_user_temp_cache",
            "macOS 用户临时/缓存根目录；只做只读估算，不建议整根删除。",
        ));
    }
    roots
}

fn bounded_dir_size(path: &Path, max_entries: usize, max_depth: usize) -> BoundedDirSize {
    let mut stack = vec![(path.to_path_buf(), 0usize)];
    let mut bytes = 0u64;
    let mut entries_scanned = 0usize;
    let mut inaccessible_count = 0usize;
    let mut truncated = false;

    while let Some((current, depth)) = stack.pop() {
        if entries_scanned >= max_entries {
            truncated = true;
            break;
        }
        entries_scanned = entries_scanned.saturating_add(1);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(_) => {
                inaccessible_count = inaccessible_count.saturating_add(1);
                continue;
            }
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_file() {
            bytes = bytes.saturating_add(metadata.len());
            continue;
        }
        if !metadata.is_dir() {
            bytes = bytes.saturating_add(metadata.len());
            continue;
        }
        bytes = bytes.saturating_add(metadata.len());
        if depth >= max_depth {
            truncated = true;
            continue;
        }
        let entries = match fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(_) => {
                inaccessible_count = inaccessible_count.saturating_add(1);
                continue;
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => stack.push((entry.path(), depth.saturating_add(1))),
                Err(_) => inaccessible_count = inaccessible_count.saturating_add(1),
            }
        }
    }

    BoundedDirSize {
        bytes,
        entries_scanned,
        inaccessible_count,
        truncated,
    }
}

fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index + 1 < UNITS.len() {
        value /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", value, UNITS[unit_index])
    }
}

fn render_disk_junk_audit_reply(tool_output: &str) -> String {
    let structured = tool_output
        .split("structured=")
        .nth(1)
        .or_else(|| tool_output.split("structured:").nth(1))
        .and_then(|json_text| serde_json::from_str::<Value>(json_text.trim()).ok());
    let Some(value) = structured else {
        return format!(
            "我已走 Hepta native runtime 做了只读磁盘垃圾审计。原始结果：{}\n\n我没有删除任何东西；如果要清理，需要你明确确认。",
            tool_output.chars().take(600).collect::<String>()
        );
    };
    let total = value
        .get("estimated_reclaimable_human")
        .and_then(Value::as_str)
        .unwrap_or("未知");
    let count = value
        .get("candidate_count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let mut lines = vec![format!(
        "扫完了：这是 Hepta native runtime 的只读审计，没删任何文件。共发现 {} 个候选位置，粗略可回收约 {}。",
        count, total
    )];
    if let Some(candidates) = value.get("top_candidates").and_then(Value::as_array) {
        lines.push("\n优先看这几个：".into());
        for candidate in candidates.iter().take(6) {
            let size = candidate
                .get("human_size")
                .and_then(Value::as_str)
                .unwrap_or("未知大小");
            let path = candidate
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or("未知路径");
            let kind = candidate
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or("candidate");
            let truncated = candidate
                .get("truncated")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let suffix = if truncated {
                "（估算被截断，实际可能更大）"
            } else {
                ""
            };
            lines.push(format!("- {} · {} · {}{}", size, kind, path, suffix));
        }
    }
    lines.push("\n建议：先从用户缓存/开发缓存下手；系统级缓存和 /private/var/folders 不要整根删。你确认后我再执行具体清理。".into());
    lines.join("\n")
}

fn resolve_backup_reference(backup_root: &Path, backup_ref: &str) -> Result<PathBuf, HeptaError> {
    let direct = PathBuf::from(backup_ref);
    if direct.exists() {
        return Ok(fs::canonicalize(&direct).unwrap_or(direct));
    }
    let nested = backup_root.join(backup_ref);
    if nested.exists() {
        Ok(fs::canonicalize(&nested).unwrap_or(nested))
    } else {
        Err(HeptaError(format!(
            "backup reference not found: {}",
            backup_ref
        )))
    }
}

fn parse_backup_entry(
    workspace_root: &Path,
    backup_root: &Path,
    backup_path: &Path,
) -> Result<Option<BackupEntryReport>, HeptaError> {
    let Ok(relative) = backup_path.strip_prefix(backup_root) else {
        return Ok(None);
    };
    let mut components = relative.components();
    let Some(scope_component) = components
        .next()
        .and_then(|component| component.as_os_str().to_str())
    else {
        return Ok(None);
    };
    let remainder = components.as_path();
    let original_relative = parse_backup_relative_target(remainder)?;
    let (scope, target_path) = match scope_component {
        "workspace" => (
            "workspace".to_string(),
            workspace_root.join(&original_relative),
        ),
        "external" => (
            "external".to_string(),
            PathBuf::from("/").join(&original_relative),
        ),
        _ => return Ok(None),
    };
    let file_name = backup_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            HeptaError(format!(
                "invalid backup file name: {}",
                backup_path.display()
            ))
        })?;
    let created_at_unix_ms = file_name
        .rsplit_once(".hepta-bak-")
        .and_then(|(_, ts)| ts.parse::<u64>().ok())
        .ok_or_else(|| {
            HeptaError(format!(
                "backup file missing timestamp suffix: {}",
                backup_path.display()
            ))
        })?;
    let metadata = match fs::metadata(backup_path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => {
            return Err(HeptaError(format!(
                "failed to stat backup {}: {}",
                backup_path.display(),
                err
            )));
        }
    };
    Ok(Some(BackupEntryReport {
        id: relative.display().to_string(),
        backup_path: backup_path.display().to_string(),
        target_path: target_path.display().to_string(),
        scope,
        created_at_unix_ms,
        bytes: metadata.len(),
    }))
}

fn parse_backup_relative_target(path: &Path) -> Result<PathBuf, HeptaError> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| HeptaError(format!("invalid backup target path: {}", path.display())))?;
    let Some((original_name, _)) = file_name.rsplit_once(".hepta-bak-") else {
        return Err(HeptaError(format!(
            "backup file missing original suffix: {}",
            path.display()
        )));
    };
    let mut target = PathBuf::from(path.parent().unwrap_or_else(|| Path::new("")));
    target.push(original_name);
    Ok(target)
}

fn validate_object_schema(
    schema_name: &str,
    schema_kind: &str,
    schema_value: &Value,
    input_value: &Value,
) -> Result<(), HeptaError> {
    let input_object = input_value.as_object().ok_or_else(|| {
        HeptaError(format!(
            "tool {} expects a JSON object {}",
            schema_name, schema_kind
        ))
    })?;

    if let Some(required) = schema_value.get("required").and_then(Value::as_array) {
        for field in required.iter().filter_map(Value::as_str) {
            if !input_object.contains_key(field) {
                return Err(HeptaError(format!(
                    "tool {} missing required field '{}'",
                    schema_name, field
                )));
            }
        }
    }

    if let Some(properties) = schema_value.get("properties").and_then(Value::as_object) {
        for (field, field_schema) in properties {
            if let Some(value) = input_object.get(field) {
                validate_property(schema_name, field, field_schema, value)?;
            }
        }
    }

    Ok(())
}

fn validate_property(
    schema_name: &str,
    field: &str,
    field_schema: &Value,
    value: &Value,
) -> Result<(), HeptaError> {
    match field_schema.get("type").and_then(Value::as_str) {
        Some("string") => {
            let string_value = value.as_str().ok_or_else(|| {
                HeptaError(format!(
                    "tool {} field '{}' must be a string",
                    schema_name, field
                ))
            })?;

            if let Some(min_length) = field_schema.get("minLength").and_then(Value::as_u64) {
                if string_value.chars().count() < min_length as usize {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be at least {} characters",
                        schema_name, field, min_length
                    )));
                }
            }

            if let Some(allowed) = field_schema.get("enum").and_then(Value::as_array) {
                let allowed_values = allowed.iter().filter_map(Value::as_str).collect::<Vec<_>>();
                if !allowed_values.is_empty()
                    && !allowed_values.iter().any(|item| *item == string_value)
                {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be one of: {}",
                        schema_name,
                        field,
                        allowed_values.join(", ")
                    )));
                }
            }

            Ok(())
        }
        Some("boolean") => {
            if value.is_boolean() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be a boolean",
                    schema_name, field
                )))
            }
        }
        Some("integer") => {
            let integer_value = value.as_i64().ok_or_else(|| {
                HeptaError(format!(
                    "tool {} field '{}' must be an integer",
                    schema_name, field
                ))
            })?;

            if let Some(minimum) = field_schema.get("minimum").and_then(Value::as_i64) {
                if integer_value < minimum {
                    return Err(HeptaError(format!(
                        "tool {} field '{}' must be at least {}",
                        schema_name, field, minimum
                    )));
                }
            }

            Ok(())
        }
        Some("number") => {
            if value.is_number() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be a number",
                    schema_name, field
                )))
            }
        }
        Some("array") => {
            if value.is_array() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be an array",
                    schema_name, field
                )))
            }
        }
        Some("object") => {
            if value.is_object() {
                Ok(())
            } else {
                Err(HeptaError(format!(
                    "tool {} field '{}' must be an object",
                    schema_name, field
                )))
            }
        }
        Some(other) => Err(HeptaError(format!(
            "tool {} field '{}' uses unsupported schema type {}",
            schema_name, field, other
        ))),
        None => Err(HeptaError(format!(
            "tool {} field '{}' is missing a type",
            schema_name, field
        ))),
    }
}

fn format_tool_message(tool_result: &ToolResult) -> String {
    match &tool_result.structured_json {
        Some(structured_json) => {
            format!("{} | structured={}", tool_result.content, structured_json)
        }
        None => tool_result.content.clone(),
    }
}

fn format_tool_memory_content(tool_result: &ToolResult) -> String {
    match &tool_result.structured_json {
        Some(structured_json) => format!(
            "tool:{} | structured:{}",
            tool_result.content, structured_json
        ),
        None => format!("tool:{}", tool_result.content),
    }
}

fn current_unix_ms() -> Result<u64, HeptaError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| HeptaError(format!("system clock before unix epoch: {}", err)))?;
    Ok(duration.as_millis() as u64)
}

fn summarize_user_intent(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(120)
        .collect()
}

fn format_selected_context_recall_for_native_turn(
    envelope: &CoreTurnContextRecallSelectedSnippetEnvelope,
) -> Option<String> {
    if !envelope.has_shadow_integrity() || envelope.snippets.is_empty() {
        return None;
    }
    if envelope
        .snippets
        .iter()
        .any(|snippet| !native_selected_context_recall_text_is_prompt_safe(&snippet.text))
    {
        return None;
    }

    let snippets = envelope
        .snippets
        .iter()
        .map(|snippet| {
            format!(
                "- hash={} estimated_tokens={} redacted={} truncated={}: {}",
                snippet.snippet_hash,
                snippet.estimated_tokens,
                snippet.redacted,
                snippet.truncated,
                snippet.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Some(format!(
        "<selected_context_recall>\n{}\n</selected_context_recall>",
        snippets
    ))
}

fn native_selected_snippet_prompt_count(
    envelope: Option<&CoreTurnContextRecallSelectedSnippetEnvelope>,
) -> u32 {
    envelope
        .filter(|envelope| format_selected_context_recall_for_native_turn(envelope).is_some())
        .map(|envelope| envelope.selected_snippet_count)
        .unwrap_or(0)
}

fn native_selected_context_recall_text_is_prompt_safe(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    let forbidden = [
        "source_id",
        "source lane",
        "source_lane",
        "raw ranked",
        "raw_ranked",
        "ranked_payload",
        "score reason",
        "score_reason",
        "[hepta-memory:",
        "query payload",
        "query_payload",
        "per-source",
        "per_source",
        "memory_id",
        "topic_id",
        "neuron_id",
        "<selected_context_recall",
        "</selected_context_recall",
    ];

    forbidden.iter().all(|marker| !lowered.contains(marker))
}

fn truncate_for_context(input: &str, max_chars: usize) -> String {
    let mut output = input.chars().take(max_chars).collect::<String>();
    if input.chars().count() > max_chars {
        output.push('…');
    }
    output
}

fn memory_context_keyword(input: &str) -> String {
    let trimmed = input.trim();
    for keyword in ["暗号", "密码", "口令", "remember", "memory", "记住", "之前"] {
        if trimmed.contains(keyword) {
            return keyword.to_string();
        }
    }
    trimmed
        .split_whitespace()
        .next()
        .unwrap_or("")
        .chars()
        .take(32)
        .collect()
}

fn merge_approval_snapshots(
    current: ApprovalSnapshot,
    incoming: ApprovalSnapshot,
) -> ApprovalSnapshot {
    let mut granted_tools = current.granted_tools;
    for tool in incoming.granted_tools {
        if !granted_tools.iter().any(|existing| existing == &tool) {
            granted_tools.push(tool);
        }
    }

    let mut pending = current.pending;
    for item in incoming.pending {
        if granted_tools.iter().any(|tool| tool == &item.tool_name) {
            continue;
        }
        if !pending
            .iter()
            .any(|existing| existing.tool_name == item.tool_name)
        {
            pending.push(item);
        }
    }

    ApprovalSnapshot {
        granted_tools,
        pending,
    }
}
