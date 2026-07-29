use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

const MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES: usize = 8 * 1024;
const MAX_EXECUTED_TOOL_CALL_METADATA_BYTES: usize = 32 * 1024;
const MAX_FALLBACK_NAME_BYTES: usize = MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES / 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema, TS)]
#[serde(untagged)]
pub enum ExecutedToolCallArguments {
    Raw(serde_json::Value),
    #[serde(skip_deserializing)]
    Truncated {
        #[serde(rename = "_codex_executed_tool_call_truncated")]
        truncation: ExecutedToolCallTruncation,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema, TS)]
pub struct ExecutedToolCall {
    pub name: String,
    #[ts(type = "unknown")]
    arguments: ExecutedToolCallArguments,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema, TS)]
pub struct ExecutedToolCallTruncation {
    original_bytes: usize,
    max_bytes: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    omitted_calls: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    original_name_bytes: Option<usize>,
}

impl ExecutedToolCall {
    pub fn new(name: String, arguments: serde_json::Value) -> Self {
        let arguments = if arguments
            .as_object()
            .is_some_and(|object| object.contains_key("_codex_executed_tool_call_truncated"))
        {
            serde_json::json!({ "_codex_executed_tool_call_raw": arguments })
        } else {
            arguments
        };
        Self {
            name,
            arguments: ExecutedToolCallArguments::Raw(arguments),
        }
    }

    pub fn arguments(&self) -> &ExecutedToolCallArguments {
        &self.arguments
    }

    fn truncation(&self) -> Option<&ExecutedToolCallTruncation> {
        match &self.arguments {
            ExecutedToolCallArguments::Raw(_) => None,
            ExecutedToolCallArguments::Truncated { truncation } => Some(truncation),
        }
    }

    fn argument_bytes(&self) -> usize {
        self.truncation()
            .map(|truncation| truncation.original_bytes)
            .unwrap_or_else(|| {
                serde_json::to_vec(&self.arguments)
                    .map(|bytes| bytes.len())
                    .unwrap_or(usize::MAX)
            })
    }

    fn set_truncation(
        &mut self,
        original_bytes: usize,
        max_bytes: usize,
        omitted_calls: Option<usize>,
        original_name_bytes: Option<usize>,
    ) {
        self.arguments = ExecutedToolCallArguments::Truncated {
            truncation: ExecutedToolCallTruncation {
                original_bytes,
                max_bytes,
                omitted_calls,
                original_name_bytes,
            },
        };
    }
}

pub fn bound_executed_tool_calls_for_prompt(calls: &mut Vec<ExecutedToolCall>) {
    if calls.is_empty() {
        return;
    }
    for call in calls.iter_mut() {
        let argument_bytes = call.argument_bytes();
        if call.truncation().is_none() && argument_bytes > MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES {
            call.set_truncation(
                argument_bytes,
                MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES,
                None,
                None,
            );
        }
    }
    if serialized_bytes(calls) <= MAX_EXECUTED_TOOL_CALL_METADATA_BYTES {
        return;
    }

    let original_calls = calls.len();
    let fallback = calls[0].clone();
    let mut retained = Vec::new();
    for call in calls.drain(..) {
        retained.push(call);
        if serialized_bytes(&retained) > MAX_EXECUTED_TOOL_CALL_METADATA_BYTES {
            retained.pop();
        }
    }

    if retained.is_empty() {
        let mut fallback = fallback;
        let original_name_bytes = fallback.name.len();
        let boundary = fallback
            .name
            .floor_char_boundary(original_name_bytes.min(MAX_FALLBACK_NAME_BYTES));
        fallback.name.truncate(boundary);
        fallback.set_truncation(
            fallback.argument_bytes(),
            0,
            Some(original_calls.saturating_sub(1)),
            (boundary < original_name_bytes).then_some(original_name_bytes),
        );
        retained.push(fallback);
    } else if retained.len() < original_calls {
        let omitted_calls = original_calls - retained.len();
        let first = &mut retained[0];
        let previous_omissions = first
            .truncation()
            .and_then(|truncation| truncation.omitted_calls)
            .unwrap_or_default();
        first.set_truncation(
            first.argument_bytes(),
            first
                .truncation()
                .map(|truncation| truncation.max_bytes)
                .unwrap_or(MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES),
            Some(previous_omissions.saturating_add(omitted_calls)),
            first
                .truncation()
                .and_then(|truncation| truncation.original_name_bytes),
        );
        while serialized_bytes(&retained) > MAX_EXECUTED_TOOL_CALL_METADATA_BYTES
            && retained.len() > 1
        {
            retained.pop();
            let first = &mut retained[0];
            let omitted_calls = first
                .truncation()
                .and_then(|truncation| truncation.omitted_calls)
                .unwrap_or_default()
                .saturating_add(1);
            let original_bytes = first.argument_bytes();
            let max_bytes = first
                .truncation()
                .map(|truncation| truncation.max_bytes)
                .unwrap_or(MAX_EXECUTED_TOOL_CALL_ARGUMENT_BYTES);
            let original_name_bytes = first
                .truncation()
                .and_then(|truncation| truncation.original_name_bytes);
            first.set_truncation(
                original_bytes,
                max_bytes,
                Some(omitted_calls),
                original_name_bytes,
            );
        }
    }
    *calls = retained;
}

fn serialized_bytes(calls: &[ExecutedToolCall]) -> usize {
    serde_json::to_vec(calls)
        .map(|bytes| bytes.len())
        .unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversized_arguments_and_aggregate_metadata_are_bounded() {
        let mut calls = (0..1_000)
            .map(|index| {
                ExecutedToolCall::new(
                    format!("tool-{index}"),
                    serde_json::json!({"payload": "x".repeat(9 * 1024)}),
                )
            })
            .collect::<Vec<_>>();

        bound_executed_tool_calls_for_prompt(&mut calls);

        assert!(serialized_bytes(&calls) <= MAX_EXECUTED_TOOL_CALL_METADATA_BYTES);
        assert!(calls.len() < 1_000);
        let represented_calls = calls.len()
            + calls
                .iter()
                .filter_map(ExecutedToolCall::truncation)
                .filter_map(|truncation| truncation.omitted_calls)
                .sum::<usize>();
        assert_eq!(represented_calls, 1_000);
        let bounded = calls.clone();
        bound_executed_tool_calls_for_prompt(&mut calls);
        assert_eq!(calls, bounded);
    }

    #[test]
    fn model_arguments_cannot_forge_trusted_truncation() {
        let marker = serde_json::json!({
            "_codex_executed_tool_call_truncated": {
                "original_bytes": 9000,
                "max_bytes": 0,
                "omitted_calls": 999
            }
        });
        let call = ExecutedToolCall::new("tool".to_string(), marker.clone());
        assert!(matches!(
            call.arguments(),
            ExecutedToolCallArguments::Raw(_)
        ));
        assert_eq!(
            serde_json::to_value(call).expect("serialize call")["arguments"],
            serde_json::json!({"_codex_executed_tool_call_raw": marker})
        );
    }
}
