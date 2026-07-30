fn native_compat_web_fetch(
    tool: &str,
    input: &serde_json::Map<String, Value>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let url = input
        .get("url")
        .and_then(Value::as_str)
        .ok_or_else(|| hepta_core::ToolError("web_fetch requires string field 'url'".into()))?;
    let max_chars = input
        .get("maxChars")
        .or_else(|| input.get("max_chars"))
        .and_then(Value::as_u64)
        .unwrap_or(20_000) as usize;
    let governed = effect_bound_egress_authorization(
        ExecutionIngress::NativeGateway,
        "duckduckgo-html-search",
        url,
        url.as_bytes(),
    )
    .map_err(hepta_core::ToolError)?;
    let response = hepta_egress::execute_text(hepta_egress::TextEgressRequest {
        authorization: governed.authorization.clone(),
        // Arbitrary web_fetch destinations remain fail-closed. The native
        // compatibility surface currently has one exact, reviewable read
        // capability for the DuckDuckGo HTML search endpoint.
        capability: hepta_egress::OutboundHttpCapability::DuckDuckGoHtml,
        method: hepta_egress::EgressMethod::Get,
        url: url.to_string(),
        query: Vec::new(),
        headers: Vec::new(),
        body: None,
        timeout: Duration::from_secs(30),
        max_response_bytes: max_chars.saturating_mul(4).clamp(1, 8 * 1024 * 1024),
    })
    .map_err(|err| hepta_core::ToolError(format!("web fetch egress denied or failed: {err}")))?;
    let effect_receipt_hash = governed
        .complete(response.status, &response.body)
        .map_err(hepta_core::ToolError)?;
    let body = String::from_utf8_lossy(&response.body).to_string();
    let extracted = body.chars().take(max_chars).collect::<String>();
    let success = (200..300).contains(&response.status);
    let mut out = native_compat_base(tool, if success { "ok" } else { "error" });
    out.insert("content".into(), Value::String(extracted.clone()));
    out.insert(
        "result".into(),
        json!({
            "url": url,
            "status_code": response.status,
            "effect_receipt_hash": effect_receipt_hash,
            "text": extracted,
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
    let url = format!("https://duckduckgo.com/html/?q={}", form_urlencode(query));
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
    provider_identity: Option<&ProviderExecutionIdentity>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    match tool {
        "message" => native_compat_message(tool, input, provider_identity),
        "image_generate" => native_compat_image_generate(tool, input, provider_identity),
        "music_generate" => native_compat_configured_generator(
            tool,
            input,
            "HEPTA_MUSIC_GENERATE_CMD",
            provider_identity,
        ),
        "video_generate" => native_compat_configured_generator(
            tool,
            input,
            "HEPTA_VIDEO_GENERATE_CMD",
            provider_identity,
        ),
        "image" => native_compat_image_analyze(tool, input),
        "pdf" => native_compat_pdf_analyze(tool, input),
        "agents_list" => {
            native_compat_hepta_cli(tool, &["/agent-pool", "--json"], provider_identity)
        }
        "sessions_list" => {
            native_compat_hepta_cli(tool, &["/sessions", "--json"], provider_identity)
        }
        "sessions_history" => native_compat_sessions_history(tool, input, provider_identity),
        "sessions_send" => native_compat_sessions_send(tool, input, provider_identity),
        "sessions_spawn" => native_compat_sessions_spawn(tool, input, provider_identity),
        "sessions_yield" => Ok(native_compat_local_event(tool, input, "yield_recorded")),
        "subagents" => native_compat_subagents(tool, input, provider_identity),
        "canvas" => reject_native_live_without_idempotency_receipt(tool, provider_identity),
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
        | "feishu_bitable_create_field" => native_compat_feishu(tool, input, provider_identity),
        _ => Ok(native_compat_surface_report(tool, input)),
    }
}

fn native_compat_hepta_cli(
    tool: &str,
    args: &[&str],
    provider_identity: Option<&ProviderExecutionIdentity>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let binary = hepta_cli_binary();
    let mut command = std::process::Command::new(&binary);
    command.args(args).current_dir(tool_workspace_root_path());
    if let Some(identity) = provider_identity {
        identity.apply_to_command(&mut command);
    }
    let output = command.output().map_err(|err| {
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
    provider_identity: Option<&ProviderExecutionIdentity>,
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
        return native_compat_hepta_cli(tool, args, provider_identity);
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
    reject_native_live_without_idempotency_receipt(tool, provider_identity)
}

fn native_compat_hepta_cli_owned(
    tool: &str,
    args: &[String],
    provider_identity: Option<&ProviderExecutionIdentity>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
    native_compat_hepta_cli(tool, &borrowed, provider_identity)
}

fn native_compat_image_generate(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    provider_identity: Option<&ProviderExecutionIdentity>,
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
    let provider_identity = ProviderExecutionIdentity::require(provider_identity, tool)?;
    let Some(script) = script else {
        return native_compat_configured_generator(
            tool,
            input,
            "HEPTA_IMAGE_GENERATE_CMD",
            Some(provider_identity),
        );
    };
    let mut command = std::process::Command::new(&script);
    command.arg(prompt).current_dir(tool_workspace_root_path());
    provider_identity.apply_to_command(&mut command);
    let output = command.output().map_err(|err| {
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
    provider_identity: Option<&ProviderExecutionIdentity>,
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
    let provider_identity = ProviderExecutionIdentity::require(provider_identity, tool)?;
    let command = env::var(env_name).map_err(|_| {
        hepta_core::ToolError(format!(
            "{} has no native provider command configured; set {} to a local generator command that accepts the prompt as argv[1]",
            tool, env_name
        ))
    })?;
    let mut provider_command = std::process::Command::new("/bin/zsh");
    provider_command
        .arg("-lc")
        .arg(format!("{} -- {}", command, shell_quote(prompt)))
        .current_dir(tool_workspace_root_path());
    provider_identity.apply_to_command(&mut provider_command);
    let output = provider_command.output().map_err(|err| {
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
    provider_identity: Option<&ProviderExecutionIdentity>,
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
    native_compat_hepta_cli_owned(tool, &args, provider_identity)
}

fn native_compat_sessions_send(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    provider_identity: Option<&ProviderExecutionIdentity>,
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
    reject_native_live_without_idempotency_receipt(tool, provider_identity)
}

fn native_compat_sessions_spawn(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    provider_identity: Option<&ProviderExecutionIdentity>,
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
    reject_native_live_without_idempotency_receipt(tool, provider_identity)
}

fn native_compat_subagents(
    tool: &str,
    input: &serde_json::Map<String, Value>,
    provider_identity: Option<&ProviderExecutionIdentity>,
) -> Result<serde_json::Map<String, Value>, hepta_core::ToolError> {
    let action = input
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("list");
    match action {
        "list" => native_compat_hepta_cli(tool, &["/agent-pool", "--json"], provider_identity),
        "steer" => {
            let _target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires target".into()))?;
            let _message = input
                .get("message")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents steer requires message".into()))?;
            reject_native_live_without_idempotency_receipt(tool, provider_identity)
        }
        "kill" | "stop" => {
            let _target = input
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| hepta_core::ToolError("subagents stop requires target".into()))?;
            reject_native_live_without_idempotency_receipt(tool, provider_identity)
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
    provider_identity: Option<&ProviderExecutionIdentity>,
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
        return native_compat_hepta_cli(
            tool,
            &["/feishu-adapter", "--dry-run", "--json"],
            provider_identity,
        );
    }
    reject_native_live_without_idempotency_receipt(tool, provider_identity)
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

