use serde_json::Value as JsonValue;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginManifestToolDeclarations {
    pub tool_schemas: Vec<PluginManifestToolSchemaDeclaration>,
    pub permissions: Vec<String>,
    pub permission_declarations: Vec<PluginManifestPermissionDeclaration>,
    pub activation_events: Vec<String>,
    pub activation_event_declarations: Vec<PluginManifestActivationEventDeclaration>,
    pub tool_policies: Vec<PluginManifestToolPolicyDeclaration>,
}

impl PluginManifestToolDeclarations {
    pub fn declared_candidate_ids(&self) -> Vec<String> {
        self.tool_schemas
            .iter()
            .map(|declaration| declaration.candidate_tool_id.as_str())
            .chain(self.permissions.iter().map(String::as_str))
            .chain(self.activation_events.iter().map(String::as_str))
            .chain(
                self.tool_policies
                    .iter()
                    .map(|declaration| declaration.candidate_tool_id.as_str()),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    pub fn schema_complete_candidate_ids(&self) -> Vec<String> {
        self.tool_schemas
            .iter()
            .filter(|declaration| {
                declaration.input_schema_declared && declaration.output_schema_declared
            })
            .map(|declaration| declaration.candidate_tool_id.clone())
            .collect()
    }

    pub fn policy_complete_candidate_ids(&self) -> Vec<String> {
        let permission_ids = self
            .permissions
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let activation_event_ids = self
            .activation_events
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        self.tool_policies
            .iter()
            .filter(|declaration| {
                permission_ids.contains(declaration.candidate_tool_id.as_str())
                    && activation_event_ids.contains(declaration.candidate_tool_id.as_str())
                    && declaration.approval_policy_declared
                    && declaration.ledger_policy_declared
                    && declaration.timeout_policy_declared
            })
            .map(|declaration| declaration.candidate_tool_id.clone())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestToolSchemaDeclaration {
    pub candidate_tool_id: String,
    pub input_schema_declared: bool,
    pub output_schema_declared: bool,
    pub input_schema_is_object: bool,
    pub output_schema_is_object: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestPermissionDeclaration {
    pub candidate_tool_id: String,
    pub network_declared: bool,
    pub network_none: bool,
    pub filesystem_read_only: bool,
    pub connector_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestActivationEventDeclaration {
    pub candidate_tool_id: String,
    pub activation_event_declared: bool,
    pub manual_activation_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestToolPolicyDeclaration {
    pub candidate_tool_id: String,
    pub approval_policy_declared: bool,
    pub approval_kind: Option<String>,
    pub ledger_policy_declared: bool,
    pub ledger_required: Option<bool>,
    pub timeout_policy_declared: bool,
    pub timeout_ms: Option<u64>,
}

pub(crate) fn resolve_tool_declarations(
    tool_schemas: Option<&JsonValue>,
    permissions: Option<&JsonValue>,
    activation_events: Option<&JsonValue>,
    tool_policies: Option<&JsonValue>,
) -> PluginManifestToolDeclarations {
    PluginManifestToolDeclarations {
        tool_schemas: tool_schema_declarations(tool_schemas),
        permissions: object_keys(permissions),
        permission_declarations: permission_declarations(permissions),
        activation_events: object_keys(activation_events),
        activation_event_declarations: activation_event_declarations(activation_events),
        tool_policies: tool_policy_declarations(tool_policies),
    }
}

fn tool_schema_declarations(
    tool_schemas: Option<&JsonValue>,
) -> Vec<PluginManifestToolSchemaDeclaration> {
    let Some(JsonValue::Object(tool_schemas)) = tool_schemas else {
        return Vec::new();
    };

    let mut declarations = tool_schemas
        .iter()
        .map(
            |(candidate_tool_id, declaration)| PluginManifestToolSchemaDeclaration {
                candidate_tool_id: candidate_tool_id.clone(),
                input_schema_declared: object_has_field(declaration, "inputSchema"),
                output_schema_declared: object_has_field(declaration, "outputSchema"),
                input_schema_is_object: object_field_is_object(declaration, "inputSchema"),
                output_schema_is_object: object_field_is_object(declaration, "outputSchema"),
            },
        )
        .collect::<Vec<_>>();
    declarations.sort_by(|left, right| left.candidate_tool_id.cmp(&right.candidate_tool_id));
    declarations
}

fn permission_declarations(
    permissions: Option<&JsonValue>,
) -> Vec<PluginManifestPermissionDeclaration> {
    let Some(JsonValue::Object(permissions)) = permissions else {
        return Vec::new();
    };

    let mut declarations = permissions
        .iter()
        .map(
            |(candidate_tool_id, declaration)| PluginManifestPermissionDeclaration {
                candidate_tool_id: candidate_tool_id.clone(),
                network_declared: object_has_field(declaration, "network"),
                network_none: object_string_field_eq(declaration, "network", "none"),
                filesystem_read_only: object_string_field_eq(
                    declaration,
                    "filesystem",
                    "read-only",
                ),
                connector_declared: object_has_field(declaration, "connector"),
            },
        )
        .collect::<Vec<_>>();
    declarations.sort_by(|left, right| left.candidate_tool_id.cmp(&right.candidate_tool_id));
    declarations
}

fn activation_event_declarations(
    activation_events: Option<&JsonValue>,
) -> Vec<PluginManifestActivationEventDeclaration> {
    let Some(JsonValue::Object(activation_events)) = activation_events else {
        return Vec::new();
    };

    let mut declarations = activation_events
        .iter()
        .map(|(candidate_tool_id, declaration)| {
            let events = declaration.as_array();
            PluginManifestActivationEventDeclaration {
                candidate_tool_id: candidate_tool_id.clone(),
                activation_event_declared: events.is_some_and(|events| !events.is_empty()),
                manual_activation_only: events.is_some_and(|events| {
                    !events.is_empty()
                        && events
                            .iter()
                            .all(|event| object_string_field_eq(event, "type", "manual"))
                }),
            }
        })
        .collect::<Vec<_>>();
    declarations.sort_by(|left, right| left.candidate_tool_id.cmp(&right.candidate_tool_id));
    declarations
}

fn tool_policy_declarations(
    tool_policies: Option<&JsonValue>,
) -> Vec<PluginManifestToolPolicyDeclaration> {
    let Some(JsonValue::Object(tool_policies)) = tool_policies else {
        return Vec::new();
    };

    let mut declarations = tool_policies
        .iter()
        .map(
            |(candidate_tool_id, declaration)| PluginManifestToolPolicyDeclaration {
                candidate_tool_id: candidate_tool_id.clone(),
                approval_policy_declared: object_has_field(declaration, "approval"),
                approval_kind: object_field(declaration, "approval")
                    .and_then(|approval| object_string_field(approval, "kind"))
                    .map(str::to_string),
                ledger_policy_declared: object_has_field(declaration, "ledger"),
                ledger_required: object_field(declaration, "ledger")
                    .and_then(|ledger| object_bool_field(ledger, "required")),
                timeout_policy_declared: object_has_field(declaration, "timeoutMs"),
                timeout_ms: object_u64_field(declaration, "timeoutMs"),
            },
        )
        .collect::<Vec<_>>();
    declarations.sort_by(|left, right| left.candidate_tool_id.cmp(&right.candidate_tool_id));
    declarations
}

fn object_keys(value: Option<&JsonValue>) -> Vec<String> {
    let Some(JsonValue::Object(value)) = value else {
        return Vec::new();
    };

    value
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn object_has_field(value: &JsonValue, field: &str) -> bool {
    let JsonValue::Object(value) = value else {
        return false;
    };
    value.get(field).is_some()
}

fn object_field<'a>(value: &'a JsonValue, field: &str) -> Option<&'a JsonValue> {
    let JsonValue::Object(value) = value else {
        return None;
    };
    value.get(field)
}

fn object_field_is_object(value: &JsonValue, field: &str) -> bool {
    matches!(object_field(value, field), Some(JsonValue::Object(_)))
}

fn object_string_field<'a>(value: &'a JsonValue, field: &str) -> Option<&'a str> {
    object_field(value, field).and_then(JsonValue::as_str)
}

fn object_string_field_eq(value: &JsonValue, field: &str, expected: &str) -> bool {
    object_string_field(value, field) == Some(expected)
}

fn object_bool_field(value: &JsonValue, field: &str) -> Option<bool> {
    object_field(value, field).and_then(JsonValue::as_bool)
}

fn object_u64_field(value: &JsonValue, field: &str) -> Option<u64> {
    object_field(value, field).and_then(JsonValue::as_u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plugin_manifest_tool_declaration_parser_reads_tool_metadata_fields() {
        let tool_schemas = json!({
            "preview:mcp:demo@local:server": {
                "inputSchema": { "type": "object" },
                "outputSchema": { "type": "object" }
            },
            "preview:connector:demo@local:app": {
                "inputSchema": { "type": "object" },
                "outputSchema": { "type": "object" }
            }
        });
        let permissions = json!({
            "preview:mcp:demo@local:server": { "network": "local" },
            "preview:connector:demo@local:app": { "connector": "calendar" }
        });
        let activation_events = json!({
            "preview:mcp:demo@local:server": [{ "type": "manual" }],
            "preview:connector:demo@local:app": [{ "type": "install" }]
        });
        let tool_policies = json!({
            "preview:mcp:demo@local:server": {
                "approval": { "kind": "onUse" },
                "ledger": { "required": true },
                "timeoutMs": 30000
            },
            "preview:connector:demo@local:app": {
                "approval": { "kind": "install" },
                "ledger": { "required": true },
                "timeoutMs": 30000
            }
        });

        let declarations = resolve_tool_declarations(
            Some(&tool_schemas),
            Some(&permissions),
            Some(&activation_events),
            Some(&tool_policies),
        );

        assert_eq!(
            declarations
                .tool_schemas
                .iter()
                .map(|declaration| (
                    declaration.candidate_tool_id.as_str(),
                    declaration.input_schema_declared,
                    declaration.output_schema_declared,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("preview:connector:demo@local:app", true, true),
                ("preview:mcp:demo@local:server", true, true),
            ]
        );
        assert_eq!(
            declarations
                .permission_declarations
                .iter()
                .map(|declaration| (
                    declaration.candidate_tool_id.as_str(),
                    declaration.network_declared,
                    declaration.network_none,
                    declaration.filesystem_read_only,
                    declaration.connector_declared,
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "preview:connector:demo@local:app",
                    false,
                    false,
                    false,
                    true
                ),
                ("preview:mcp:demo@local:server", true, false, false, false),
            ]
        );
        assert_eq!(
            declarations.permissions,
            vec![
                "preview:connector:demo@local:app".to_string(),
                "preview:mcp:demo@local:server".to_string(),
            ]
        );
        assert_eq!(
            declarations
                .activation_event_declarations
                .iter()
                .map(|declaration| (
                    declaration.candidate_tool_id.as_str(),
                    declaration.activation_event_declared,
                    declaration.manual_activation_only,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("preview:connector:demo@local:app", true, false),
                ("preview:mcp:demo@local:server", true, true),
            ]
        );
        assert_eq!(
            declarations.activation_events,
            vec![
                "preview:connector:demo@local:app".to_string(),
                "preview:mcp:demo@local:server".to_string(),
            ]
        );
        assert_eq!(
            declarations
                .tool_policies
                .iter()
                .map(|declaration| (
                    declaration.candidate_tool_id.as_str(),
                    declaration.approval_policy_declared,
                    declaration.approval_kind.as_deref(),
                    declaration.ledger_policy_declared,
                    declaration.ledger_required,
                    declaration.timeout_policy_declared,
                    declaration.timeout_ms,
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "preview:connector:demo@local:app",
                    true,
                    Some("install"),
                    true,
                    Some(true),
                    true,
                    Some(30000)
                ),
                (
                    "preview:mcp:demo@local:server",
                    true,
                    Some("onUse"),
                    true,
                    Some(true),
                    true,
                    Some(30000)
                ),
            ]
        );
        assert_eq!(
            declarations.declared_candidate_ids(),
            vec![
                "preview:connector:demo@local:app".to_string(),
                "preview:mcp:demo@local:server".to_string(),
            ]
        );
        assert_eq!(
            declarations.schema_complete_candidate_ids(),
            declarations.declared_candidate_ids()
        );
        assert_eq!(
            declarations.policy_complete_candidate_ids(),
            declarations.declared_candidate_ids()
        );
    }

    #[test]
    fn plugin_manifest_tool_declaration_parser_defaults_to_empty_fields() {
        let declarations = resolve_tool_declarations(
            /*tool_schemas*/ None, /*permissions*/ None, /*activation_events*/ None,
            /*tool_policies*/ None,
        );

        assert_eq!(declarations.declared_candidate_ids(), Vec::<String>::new());
        assert_eq!(
            declarations.schema_complete_candidate_ids(),
            Vec::<String>::new()
        );
        assert_eq!(
            declarations.policy_complete_candidate_ids(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn plugin_manifest_tool_declaration_parser_ignores_invalid_field_shapes() {
        let tool_schemas = json!([]);
        let permissions = json!("invalid");
        let activation_events = JsonValue::Null;
        let tool_policies = json!(123);

        let declarations = resolve_tool_declarations(
            Some(&tool_schemas),
            Some(&permissions),
            Some(&activation_events),
            Some(&tool_policies),
        );

        assert_eq!(declarations, Default::default());
    }
}
