//! Exact local-provider effect plans and post-commit acknowledgements.

use std::fs;
use std::fs::OpenOptions;
use std::io::Read;

use hepta_contracts::ContentHash;
use hepta_core::HeptaError;
use hepta_core::ToolError;
use hepta_core::ToolResult;
use hepta_memory::ExecutionEffectAck;
use hepta_memory::ExecutionEffectAckParts;
use hepta_memory::ExecutionIntent;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

use super::context_freezer::framed_hash;
use super::outcome_sink::SharedOutcomeReceiptSink;
use crate::ExecutionEffectInspectionState;
use crate::NativePatchOp;
use crate::PendingExecutionEffectInspection;
use crate::PreparedWriteTransaction;

const EFFECT_PLAN_SCHEMA_VERSION: u32 = 1;
const EFFECT_ACK_SCHEMA_VERSION: u32 = 1;
const EFFECT_ACK_OUTPUT_FIELD: &str = "provider_effect_ack";
const WRITE_FILE_BACKUP_POLICY: &str = "write_file_overwrite_backup_v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderEffectPlan {
    schema_version: u32,
    tool: String,
    operation: String,
    target_path: String,
    mode: String,
    before_content_hash: Option<String>,
    after_content_hash: String,
    secondary_effect_policy: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderEffectExpectation {
    attempt_id: String,
    idempotency_key: String,
    effect_plan_hash: ContentHash,
    plan: ProviderEffectPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderEffectAckWire {
    schema_version: u32,
    attempt_id: String,
    idempotency_key_fingerprint: String,
    tool: String,
    operation: String,
    effect_plan_hash: String,
    target_path: String,
    observed_after_content_hash: String,
    status: String,
    secondary_effects: Vec<ProviderSecondaryEffectAck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderSecondaryEffectAck {
    kind: String,
    path: String,
    content_hash: String,
}

pub(crate) fn canonical_effect_plan_for(
    tool: &str,
    prepared: &[PreparedWriteTransaction],
    input_json: &str,
) -> Result<Option<String>, HeptaError> {
    if prepared.is_empty() {
        return Ok(None);
    }
    if prepared.iter().all(|transaction| transaction.preview_only) {
        return Ok(None);
    }
    if prepared.iter().any(|transaction| transaction.preview_only) {
        return Err(HeptaError(
            "provider effect plan refuses mixed preview and live targets".into(),
        ));
    }
    let (prepared, after_bytes, secondary_effect_policy) = match tool {
        "write_file" => {
            let [prepared] = prepared else {
                return Err(HeptaError(
                    "write_file effect plan requires one sealed target".into(),
                ));
            };
            let requested_path =
                crate::parse_required_string_field(input_json, "path").map_err(tool_error)?;
            let content =
                crate::parse_required_string_field(input_json, "content").map_err(tool_error)?;
            let mode = crate::parse_optional_string_field(input_json, "mode")
                .map_err(tool_error)?
                .unwrap_or_else(|| "create".to_string());
            let preview_only = crate::parse_optional_bool_field(input_json, "preview_only")
                .map_err(tool_error)?
                .unwrap_or(false);
            if requested_path != prepared.requested_path
                || mode != prepared.mode_requested
                || preview_only != prepared.preview_only
            {
                return Err(HeptaError(
                    "write_file effect plan no longer matches its sealed reservation".into(),
                ));
            }
            crate::verify_atomic_replace_source_unchanged(
                &prepared.sealed_target,
                prepared.before_bytes.as_deref(),
            )?;
            let after_bytes = final_write_bytes(prepared, &mode, content.as_bytes())?;
            let backup_policy = (prepared.target_existed_before && mode == "overwrite")
                .then(|| WRITE_FILE_BACKUP_POLICY.to_string());
            (prepared, after_bytes, backup_policy)
        }
        "write" => {
            crate::preflight_prepared_native_mutation(tool, prepared, input_json)
                .map_err(tool_error)?;
            let [prepared] = prepared else {
                return Err(HeptaError(
                    "native write effect plan requires one sealed target".into(),
                ));
            };
            let content =
                crate::parse_required_string_field(input_json, "content").map_err(tool_error)?;
            (
                prepared,
                final_write_bytes(prepared, &prepared.mode_requested, content.as_bytes())?,
                None,
            )
        }
        "edit" => {
            crate::preflight_prepared_native_mutation(tool, prepared, input_json)
                .map_err(tool_error)?;
            let [prepared] = prepared else {
                return Err(HeptaError(
                    "native edit effect plan requires one sealed target".into(),
                ));
            };
            (prepared, edited_bytes(prepared, input_json)?, None)
        }
        "apply_patch" => {
            crate::preflight_prepared_native_mutation(tool, prepared, input_json)
                .map_err(tool_error)?;
            let [prepared] = prepared else {
                return Err(HeptaError(
                    "live apply_patch effect plan requires one sealed target".into(),
                ));
            };
            (prepared, patched_bytes(prepared, input_json)?, None)
        }
        "tts" => {
            crate::preflight_prepared_native_mutation(tool, prepared, input_json)
                .map_err(tool_error)?;
            let [prepared] = prepared else {
                return Err(HeptaError(
                    "live tts effect plan requires one sealed target".into(),
                ));
            };
            let after_bytes = prepared.staged_after_bytes.clone().ok_or_else(|| {
                HeptaError("live tts effect plan lacks exact privately staged audio bytes".into())
            })?;
            (prepared, after_bytes, None)
        }
        _ => {
            return Err(HeptaError(format!(
                "tool {tool} has sealed write resources but no provider effect planner"
            )));
        }
    };
    let plan = ProviderEffectPlan {
        schema_version: EFFECT_PLAN_SCHEMA_VERSION,
        tool: tool.to_owned(),
        operation: prepared.operation.clone(),
        target_path: prepared.sealed_target.canonical_path.display().to_string(),
        mode: prepared.mode_requested.clone(),
        before_content_hash: prepared
            .before_bytes
            .as_deref()
            .map(crate::mutation_content_hash),
        after_content_hash: crate::mutation_content_hash(&after_bytes),
        secondary_effect_policy,
    };
    serde_json::to_string(&plan).map(Some).map_err(|error| {
        HeptaError(format!(
            "failed to canonicalize provider effect plan: {error}"
        ))
    })
}

impl ProviderEffectExpectation {
    pub(crate) fn from_intent(intent: &ExecutionIntent) -> Result<Option<Self>, HeptaError> {
        let (Some(canonical_plan), Some(effect_plan_hash)) =
            (intent.canonical_effect_plan(), intent.effect_plan_hash())
        else {
            if intent.canonical_effect_plan().is_some() || intent.effect_plan_hash().is_some() {
                return Err(HeptaError(
                    "execution intent has an incomplete provider effect binding".into(),
                ));
            }
            return Ok(None);
        };
        let plan: ProviderEffectPlan = serde_json::from_str(canonical_plan).map_err(|error| {
            HeptaError(format!(
                "execution intent contains an invalid provider effect plan: {error}"
            ))
        })?;
        if plan.schema_version != EFFECT_PLAN_SCHEMA_VERSION
            || serde_json::to_string(&plan).ok().as_deref() != Some(canonical_plan)
        {
            return Err(HeptaError(
                "execution intent provider effect plan is not canonical".into(),
            ));
        }
        Ok(Some(Self {
            attempt_id: intent.attempt_id().to_owned(),
            idempotency_key: intent.idempotency_key().to_owned(),
            effect_plan_hash: effect_plan_hash.clone(),
            plan,
        }))
    }
}

pub(crate) fn acknowledge_provider_invocation(
    result: Result<ToolResult, ToolError>,
    prepared: &[PreparedWriteTransaction],
    expectation: Option<&ProviderEffectExpectation>,
    _sink: &SharedOutcomeReceiptSink,
) -> Result<ToolResult, ToolError> {
    match result {
        Ok(result) => acknowledge_committed_provider_result(result, prepared, expectation),
        Err(provider_error) => {
            let Some(expectation) = expectation else {
                return Err(provider_error);
            };
            if expectation.plan.secondary_effect_policy.is_some() {
                return Err(provider_error);
            }
            let [prepared_target] = prepared else {
                return Err(provider_error);
            };
            let observed_after =
                crate::read_committed_sealed_target(&prepared_target.sealed_target)
                    .ok()
                    .map(|(bytes, _)| crate::mutation_content_hash(&bytes));
            if observed_after.as_deref() != Some(expectation.plan.after_content_hash.as_str()) {
                return Err(provider_error);
            }
            let synthetic = ToolResult {
                content: "provider effect committed before an error was returned".into(),
                structured_json: Some(
                    json!({
                        "status": "error",
                        "error": provider_error.0,
                        "provider_error_after_commit": true,
                    })
                    .to_string(),
                ),
            };
            acknowledge_committed_provider_result(synthetic, prepared, Some(expectation))
        }
    }
}

fn acknowledge_committed_provider_result(
    mut result: ToolResult,
    prepared: &[PreparedWriteTransaction],
    expectation: Option<&ProviderEffectExpectation>,
) -> Result<ToolResult, ToolError> {
    if prepared.iter().all(|transaction| transaction.preview_only) {
        if expectation.is_some() {
            return Err(ToolError(
                "preview provider unexpectedly received a live effect plan".into(),
            ));
        }
        return Ok(result);
    }
    let [prepared] = prepared else {
        return Err(ToolError(
            "provider effect ACK requires one atomic sealed target".into(),
        ));
    };
    let expectation = expectation.ok_or_else(|| {
        ToolError("live local mutation lacks its staged provider effect plan".into())
    })?;
    validate_prepared_binding(prepared, expectation).map_err(|error| ToolError(error.0))?;
    let (after_bytes, _) = crate::read_committed_sealed_target(&prepared.sealed_target)
        .map_err(|error| ToolError(error.0))?;
    let observed_after_content_hash = crate::mutation_content_hash(&after_bytes);
    if observed_after_content_hash != expectation.plan.after_content_hash {
        return Err(ToolError(
            "committed provider target differs from the staged effect plan".into(),
        ));
    }
    let mut output: Value = result
        .structured_json
        .as_deref()
        .ok_or_else(|| ToolError("provider effect ACK requires structured output".into()))
        .and_then(|output| {
            serde_json::from_str(output)
                .map_err(|error| ToolError(format!("provider output is invalid JSON: {error}")))
        })?;
    let secondary_effects =
        secondary_effect_acks(&expectation.plan, &output).map_err(|error| ToolError(error.0))?;
    let wire = ProviderEffectAckWire {
        schema_version: EFFECT_ACK_SCHEMA_VERSION,
        attempt_id: expectation.attempt_id.clone(),
        idempotency_key_fingerprint: idempotency_fingerprint(&expectation.idempotency_key),
        tool: expectation.plan.tool.clone(),
        operation: expectation.plan.operation.clone(),
        effect_plan_hash: expectation.effect_plan_hash.as_str().to_owned(),
        target_path: expectation.plan.target_path.clone(),
        observed_after_content_hash,
        status: "committed".into(),
        secondary_effects,
    };
    let canonical_provider_ack = serde_json::to_string(&wire).map_err(|error| {
        ToolError(format!(
            "failed to canonicalize provider effect ACK: {error}"
        ))
    })?;
    let ack = ExecutionEffectAck::try_new(ExecutionEffectAckParts {
        attempt_id: expectation.attempt_id.clone(),
        idempotency_key: expectation.idempotency_key.clone(),
        effect_plan_hash: expectation.effect_plan_hash.clone(),
        canonical_provider_ack: canonical_provider_ack.clone(),
    })
    .map_err(|error| ToolError(format!("failed to bind provider effect ACK: {error}")))?;
    let object = output
        .as_object_mut()
        .ok_or_else(|| ToolError("provider effect ACK requires an object output".into()))?;
    object.insert(
        EFFECT_ACK_OUTPUT_FIELD.into(),
        serde_json::to_value(&wire)
            .map_err(|error| ToolError(format!("failed to expose provider effect ACK: {error}")))?,
    );
    object.insert(
        "provider_effect_ack_hash".into(),
        json!(ack.ack_hash().as_str()),
    );
    result.structured_json = Some(output.to_string());
    Ok(result)
}

pub(crate) fn confirm_provider_effect_ack(
    intent: &ExecutionIntent,
    prepared: &[PreparedWriteTransaction],
    result: Option<&ToolResult>,
) -> Result<Option<ExecutionEffectAck>, HeptaError> {
    let Some(expectation) = ProviderEffectExpectation::from_intent(intent)? else {
        return Ok(None);
    };
    let [prepared] = prepared else {
        return Err(HeptaError(
            "planned provider effect no longer owns one sealed target".into(),
        ));
    };
    validate_prepared_binding(prepared, &expectation)?;
    let output = result
        .and_then(|result| result.structured_json.as_deref())
        .ok_or_else(|| {
            HeptaError(format!(
                "provider effect ACK output is missing for execution attempt {}",
                intent.attempt_id()
            ))
        })?;
    let output: Value = serde_json::from_str(output)
        .map_err(|error| HeptaError(format!("provider effect ACK output is invalid: {error}")))?;
    let wire: ProviderEffectAckWire = serde_json::from_value(
        output
            .get(EFFECT_ACK_OUTPUT_FIELD)
            .cloned()
            .ok_or_else(|| {
                HeptaError(format!(
                    "provider effect ACK is missing for execution attempt {}",
                    intent.attempt_id()
                ))
            })?,
    )
    .map_err(|error| HeptaError(format!("provider effect ACK wire is invalid: {error}")))?;
    let canonical_provider_ack = serde_json::to_string(&wire)
        .map_err(|error| HeptaError(format!("provider effect ACK is not canonical: {error}")))?;
    let ack = ExecutionEffectAck::try_new(ExecutionEffectAckParts {
        attempt_id: expectation.attempt_id.clone(),
        idempotency_key: expectation.idempotency_key.clone(),
        effect_plan_hash: expectation.effect_plan_hash.clone(),
        canonical_provider_ack,
    })
    .map_err(|error| HeptaError(format!("failed to bind provider effect ACK: {error}")))?;
    if output
        .get("provider_effect_ack_hash")
        .and_then(Value::as_str)
        != Some(ack.ack_hash().as_str())
    {
        return Err(HeptaError(
            "provider effect ACK hash output disagrees with exact material".into(),
        ));
    }
    let wire = validate_ack_payload(&expectation, &ack)?;
    let (after_bytes, _) = crate::read_committed_sealed_target(&prepared.sealed_target)?;
    if crate::mutation_content_hash(&after_bytes) != expectation.plan.after_content_hash {
        return Err(HeptaError(
            "sealed target drifted before provider effect ACK confirmation".into(),
        ));
    }
    validate_secondary_effect_acks(&expectation.plan, &wire.secondary_effects)?;
    Ok(Some(ack))
}

pub(crate) fn inspect_pending_effect(
    intent: &ExecutionIntent,
    ack: Option<&ExecutionEffectAck>,
) -> Result<PendingExecutionEffectInspection, HeptaError> {
    let Some(expectation) = ProviderEffectExpectation::from_intent(intent)? else {
        return Ok(PendingExecutionEffectInspection {
            attempt_id: intent.attempt_id().to_owned(),
            tool_name: intent.tool_name().to_owned(),
            state: ExecutionEffectInspectionState::Unplanned,
            target_path: None,
            expected_before_content_hash: None,
            expected_after_content_hash: None,
            observed_content_hash: None,
            effect_plan_hash: None,
            effect_ack_hash: None,
            detail: "execution intent has no provider-owned filesystem effect plan".into(),
        });
    };
    let ack_valid = match ack {
        Some(ack) => validate_ack_payload(&expectation, ack).is_ok(),
        None => false,
    };
    let observed = inspect_target_hash(&expectation.plan.target_path)?;
    let before_matches = observed.as_deref() == expectation.plan.before_content_hash.as_deref();
    let after_matches = observed.as_deref() == Some(expectation.plan.after_content_hash.as_str());
    let state = if ack.is_some() && !ack_valid {
        ExecutionEffectInspectionState::Drifted
    } else if after_matches && ack_valid {
        ExecutionEffectInspectionState::AppliedAcknowledged
    } else if after_matches {
        ExecutionEffectInspectionState::AppliedUnacknowledged
    } else if before_matches && ack.is_none() {
        if expectation.plan.secondary_effect_policy.is_some() {
            ExecutionEffectInspectionState::InDoubt
        } else {
            ExecutionEffectInspectionState::NotApplied
        }
    } else {
        ExecutionEffectInspectionState::Drifted
    };
    let detail = match state {
        ExecutionEffectInspectionState::Unplanned => {
            "execution intent has no provider-owned effect plan"
        }
        ExecutionEffectInspectionState::NotApplied => {
            "sealed target still matches the staged before-image and no ACK exists"
        }
        ExecutionEffectInspectionState::AppliedAcknowledged => {
            "sealed target matches the staged after-image and the durable ACK is exact"
        }
        ExecutionEffectInspectionState::AppliedUnacknowledged => {
            "sealed target matches the staged after-image but no durable ACK exists"
        }
        ExecutionEffectInspectionState::InDoubt => {
            "primary target matches the before-image, but a planned secondary effect cannot be excluded"
        }
        ExecutionEffectInspectionState::Drifted => {
            "current target or ACK differs from the staged exact effect bindings"
        }
    };
    Ok(PendingExecutionEffectInspection {
        attempt_id: intent.attempt_id().to_owned(),
        tool_name: intent.tool_name().to_owned(),
        state,
        target_path: Some(expectation.plan.target_path.clone()),
        expected_before_content_hash: expectation.plan.before_content_hash.clone(),
        expected_after_content_hash: Some(expectation.plan.after_content_hash.clone()),
        observed_content_hash: observed,
        effect_plan_hash: Some(expectation.effect_plan_hash.as_str().to_owned()),
        effect_ack_hash: ack.map(|ack| ack.ack_hash().as_str().to_owned()),
        detail: detail.into(),
    })
}

fn final_write_bytes(
    prepared: &PreparedWriteTransaction,
    mode: &str,
    content: &[u8],
) -> Result<Vec<u8>, HeptaError> {
    match mode {
        "append" => {
            let mut after = prepared.before_bytes.clone().unwrap_or_default();
            after.extend_from_slice(content);
            Ok(after)
        }
        "create" | "overwrite" => Ok(content.to_vec()),
        _ => Err(HeptaError(format!(
            "unsupported provider effect plan write mode {mode}"
        ))),
    }
}

fn edited_bytes(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
) -> Result<Vec<u8>, HeptaError> {
    let input = crate::parse_tool_input_object(input_json).map_err(tool_error)?;
    let edits = input
        .get("edits")
        .and_then(Value::as_array)
        .ok_or_else(|| HeptaError("edit effect plan requires array field 'edits'".into()))?;
    let mut content = String::from_utf8(prepared.before_bytes.clone().unwrap_or_default())
        .map_err(|error| {
            HeptaError(format!(
                "failed to plan UTF-8 edit for {}: {error}",
                prepared.target_path
            ))
        })?;
    for edit in edits {
        let old_text = edit
            .get("oldText")
            .or_else(|| edit.get("old_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| HeptaError("each planned edit requires oldText".into()))?;
        let new_text = edit
            .get("newText")
            .or_else(|| edit.get("new_text"))
            .and_then(Value::as_str)
            .ok_or_else(|| HeptaError("each planned edit requires newText".into()))?;
        let count = content.matches(old_text).count();
        if count != 1 {
            return Err(HeptaError(format!(
                "planned edit oldText matched {count} times; expected exactly once"
            )));
        }
        content = content.replacen(old_text, new_text, 1);
    }
    Ok(content.into_bytes())
}

fn patched_bytes(
    prepared: &PreparedWriteTransaction,
    input_json: &str,
) -> Result<Vec<u8>, HeptaError> {
    let (_, operations) = crate::parsed_native_patch(input_json).map_err(tool_error)?;
    let [operation] = operations.as_slice() else {
        return Err(HeptaError(
            "live apply_patch effect plan requires one operation".into(),
        ));
    };
    match operation {
        NativePatchOp::Add { content, .. } => Ok(content.as_bytes().to_vec()),
        NativePatchOp::Update { old, new, .. } => {
            let current = std::str::from_utf8(prepared.before_bytes.as_deref().unwrap_or_default())
                .map_err(|error| {
                    HeptaError(format!(
                        "failed to plan UTF-8 patch for {}: {error}",
                        prepared.target_path
                    ))
                })?;
            let count = current.matches(old).count();
            if count != 1 {
                return Err(HeptaError(format!(
                    "planned patch hunk matched {count} times; expected exactly once"
                )));
            }
            Ok(current.replacen(old, new, 1).into_bytes())
        }
        NativePatchOp::Delete { .. } => Err(HeptaError(
            "provider effect plan refuses delete operations".into(),
        )),
    }
}

fn validate_prepared_binding(
    prepared: &PreparedWriteTransaction,
    expectation: &ProviderEffectExpectation,
) -> Result<(), HeptaError> {
    if prepared.operation != expectation.plan.operation
        || prepared.mode_requested != expectation.plan.mode
        || prepared.sealed_target.canonical_path.display().to_string()
            != expectation.plan.target_path
        || prepared
            .before_bytes
            .as_deref()
            .map(crate::mutation_content_hash)
            != expectation.plan.before_content_hash
    {
        return Err(HeptaError(
            "sealed provider target disagrees with its staged effect plan".into(),
        ));
    }
    Ok(())
}

fn validate_ack_payload(
    expectation: &ProviderEffectExpectation,
    ack: &ExecutionEffectAck,
) -> Result<ProviderEffectAckWire, HeptaError> {
    if ack.attempt_id() != expectation.attempt_id
        || ack.idempotency_key() != expectation.idempotency_key
        || ack.effect_plan_hash() != &expectation.effect_plan_hash
    {
        return Err(HeptaError(
            "provider effect ACK disagrees with the staged execution identity".into(),
        ));
    }
    let wire: ProviderEffectAckWire = serde_json::from_str(ack.canonical_provider_ack())
        .map_err(|error| HeptaError(format!("provider effect ACK payload is invalid: {error}")))?;
    if wire.schema_version != EFFECT_ACK_SCHEMA_VERSION
        || serde_json::to_string(&wire).ok().as_deref() != Some(ack.canonical_provider_ack())
        || wire.attempt_id != expectation.attempt_id
        || wire.idempotency_key_fingerprint != idempotency_fingerprint(&expectation.idempotency_key)
        || wire.tool != expectation.plan.tool
        || wire.operation != expectation.plan.operation
        || wire.effect_plan_hash != expectation.effect_plan_hash.as_str()
        || wire.target_path != expectation.plan.target_path
        || wire.observed_after_content_hash != expectation.plan.after_content_hash
        || wire.status != "committed"
    {
        return Err(HeptaError(
            "provider effect ACK payload disagrees with the staged plan".into(),
        ));
    }
    Ok(wire)
}

fn inspect_target_hash(target_path: &str) -> Result<Option<String>, HeptaError> {
    let metadata = match fs::symlink_metadata(target_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(HeptaError(format!(
                "failed to inspect pending effect target {target_path}: {error}"
            )));
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(HeptaError(format!(
            "pending effect target is not a regular non-symlink file: {target_path}"
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;

        if metadata.nlink() != 1 {
            return Err(HeptaError(format!(
                "pending effect target has {} hard links: {target_path}",
                metadata.nlink()
            )));
        }
    }
    let bytes = fs::read(target_path).map_err(|error| {
        HeptaError(format!(
            "failed to read pending effect target {target_path}: {error}"
        ))
    })?;
    Ok(Some(crate::mutation_content_hash(&bytes)))
}

fn secondary_effect_acks(
    plan: &ProviderEffectPlan,
    output: &Value,
) -> Result<Vec<ProviderSecondaryEffectAck>, HeptaError> {
    match plan.secondary_effect_policy.as_deref() {
        None => Ok(Vec::new()),
        Some(WRITE_FILE_BACKUP_POLICY) => {
            if output.get("backup_created").and_then(Value::as_bool) != Some(true) {
                return Err(HeptaError(
                    "write_file provider did not confirm its planned backup effect".into(),
                ));
            }
            let backup_path = output
                .get("backup_path")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    HeptaError("write_file provider omitted its backup path ACK".into())
                })?;
            let bytes = read_regular_single_link_file(backup_path, "provider backup effect")?;
            let content_hash = crate::mutation_content_hash(&bytes);
            if plan.before_content_hash.as_deref() != Some(content_hash.as_str()) {
                return Err(HeptaError(
                    "write_file provider backup differs from the staged before-image".into(),
                ));
            }
            Ok(vec![ProviderSecondaryEffectAck {
                kind: "write_file_backup".into(),
                path: backup_path.to_owned(),
                content_hash,
            }])
        }
        Some(policy) => Err(HeptaError(format!(
            "unsupported provider secondary-effect policy {policy}"
        ))),
    }
}

fn validate_secondary_effect_acks(
    plan: &ProviderEffectPlan,
    effects: &[ProviderSecondaryEffectAck],
) -> Result<(), HeptaError> {
    match plan.secondary_effect_policy.as_deref() {
        None if effects.is_empty() => Ok(()),
        Some(WRITE_FILE_BACKUP_POLICY) => {
            let [effect] = effects else {
                return Err(HeptaError(
                    "provider backup ACK cardinality disagrees with its effect plan".into(),
                ));
            };
            if effect.kind != "write_file_backup"
                || plan.before_content_hash.as_deref() != Some(effect.content_hash.as_str())
            {
                return Err(HeptaError(
                    "provider backup ACK disagrees with its staged before-image".into(),
                ));
            }
            let bytes =
                read_regular_single_link_file(&effect.path, "acknowledged provider backup")?;
            if crate::mutation_content_hash(&bytes) != effect.content_hash {
                return Err(HeptaError(
                    "acknowledged provider backup drifted before terminal capture".into(),
                ));
            }
            Ok(())
        }
        _ => Err(HeptaError(
            "provider secondary-effect ACKs disagree with the staged policy".into(),
        )),
    }
}

fn read_regular_single_link_file(path: &str, description: &str) -> Result<Vec<u8>, HeptaError> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;

        options.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
    }
    let mut file = options
        .open(path)
        .map_err(|error| HeptaError(format!("failed to open {description} {path}: {error}")))?;
    let metadata = file
        .metadata()
        .map_err(|error| HeptaError(format!("failed to inspect {description} {path}: {error}")))?;
    if !metadata.is_file() {
        return Err(HeptaError(format!(
            "{description} is not a regular file: {path}"
        )));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;

        if metadata.nlink() != 1 {
            return Err(HeptaError(format!(
                "{description} has {} hard links: {path}",
                metadata.nlink()
            )));
        }
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| HeptaError(format!("failed to read {description} {path}: {error}")))?;
    Ok(bytes)
}

fn idempotency_fingerprint(idempotency_key: &str) -> String {
    framed_hash(
        "hepta.runtime.provider-idempotency-fingerprint.v1",
        &[("idempotency_key", idempotency_key.as_bytes())],
    )
    .into_inner()
}

fn tool_error(error: ToolError) -> HeptaError {
    HeptaError(error.0)
}
