use serde_json::Value as JsonValue;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginManifestToolDeclarations {
    pub tool_schemas: Vec<PluginManifestToolSchemaDeclaration>,
    pub permissions: Vec<String>,
    pub activation_events: Vec<String>,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifestToolPolicyDeclaration {
    pub candidate_tool_id: String,
    pub approval_policy_declared: bool,
    pub ledger_policy_declared: bool,
    pub timeout_policy_declared: bool,
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
        activation_events: object_keys(activation_events),
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
            },
        )
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
                ledger_policy_declared: object_has_field(declaration, "ledger"),
                timeout_policy_declared: object_has_field(declaration, "timeoutMs"),
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
            declarations.permissions,
            vec![
                "preview:connector:demo@local:app".to_string(),
                "preview:mcp:demo@local:server".to_string(),
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
                    declaration.ledger_policy_declared,
                    declaration.timeout_policy_declared,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("preview:connector:demo@local:app", true, true, true),
                ("preview:mcp:demo@local:server", true, true, true),
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
