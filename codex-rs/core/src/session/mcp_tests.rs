use super::*;
use rmcp::model::BooleanSchema;
use rmcp::model::ElicitationSchema;
use rmcp::model::JsonObject;
use rmcp::model::PrimitiveSchema;
use rmcp::model::Tool;
use rmcp::model::ToolAnnotations;
use serde_json::json;
use std::sync::Arc;

fn meta(value: Value) -> Option<Meta> {
    let Value::Object(map) = value else {
        panic!("metadata must be an object");
    };
    Some(Meta(map))
}

fn guardian_meta(tool_params: Option<Value>) -> Option<Meta> {
    let mut value = json!({
        "codex_approval_kind": "mcp_tool_call",
        "codex_request_type": "approval_request",
        "connector_id": "browser-use",
        "connector_name": "Browser Use",
        "tool_name": "access_browser_origin",
        "tool_title": "Access browser origin",
    });
    if let Some(tool_params) = tool_params {
        value["tool_params"] = tool_params;
    }
    meta(value)
}

fn form_request(meta: Option<Meta>) -> ElicitationReviewRequest {
    ElicitationReviewRequest {
        server_name: "browser-use".to_string(),
        request_id: rmcp::model::NumberOrString::Number(7),
        elicitation: CreateElicitationRequestParams::FormElicitationParams {
            meta,
            message: "Allow origin?".to_string(),
            requested_schema: ElicitationSchema::builder()
                .build()
                .expect("schema should build"),
        },
    }
}

#[test]
fn guardian_elicitation_review_request_builds_mcp_tool_call() {
    let request = form_request(guardian_meta(Some(json!({
        "origin": "https://example.com",
    }))));

    let GuardianElicitationReview::ApprovalRequest(guardian_request) =
        guardian_elicitation_review_request(&request)
    else {
        panic!("expected Guardian MCP tool call request");
    };
    let crate::guardian::GuardianApprovalRequest::McpToolCall {
        id,
        server,
        tool_name,
        arguments,
        connector_id,
        connector_name,
        connector_description,
        tool_title,
        tool_description,
        annotations,
    } = *guardian_request
    else {
        panic!("expected Guardian MCP tool call request");
    };

    assert_eq!(id, "mcp_elicitation:browser-use:7");
    assert_eq!(server, "browser-use");
    assert_eq!(tool_name, "access_browser_origin");
    assert_eq!(arguments, Some(json!({ "origin": "https://example.com" })));
    assert_eq!(connector_id.as_deref(), Some("browser-use"));
    assert_eq!(connector_name.as_deref(), Some("Browser Use"));
    assert_eq!(connector_description, None);
    assert_eq!(tool_title.as_deref(), Some("Access browser origin"));
    assert_eq!(tool_description, None);
    assert_eq!(annotations, None);
}

#[test]
fn guardian_elicitation_review_request_defaults_missing_tool_params() {
    let request = form_request(guardian_meta(/*tool_params*/ None));

    let GuardianElicitationReview::ApprovalRequest(guardian_request) =
        guardian_elicitation_review_request(&request)
    else {
        panic!("expected Guardian MCP tool call request");
    };
    let crate::guardian::GuardianApprovalRequest::McpToolCall { arguments, .. } = *guardian_request
    else {
        panic!("expected Guardian MCP tool call request");
    };

    assert_eq!(arguments, Some(json!({})));
}

#[test]
fn guardian_elicitation_review_request_requires_opt_in() {
    let request = form_request(meta(json!({
        "codex_approval_kind": "mcp_tool_call",
        "tool_name": "access_browser_origin",
    })));

    assert_eq!(
        guardian_elicitation_review_request(&request),
        GuardianElicitationReview::NotRequested
    );
}

#[test]
fn codex_apps_elicitation_uses_current_connector_reviewer_authority() {
    let authority = codex_protocol::protocol::McpElicitationAuthority {
        approval_policy: AskForApproval::OnRequest,
        permission_profile: PermissionProfile::default(),
        approvals_reviewer: ApprovalsReviewer::User,
        apps_approvals_reviewers: codex_protocol::protocol::McpAppsApprovalsReviewerAuthority {
            default: None,
            apps: std::collections::HashMap::from([(
                "browser-use".to_string(),
                ApprovalsReviewer::AutoReview,
            )]),
        },
    };
    assert_eq!(
        mcp_elicitation_approvals_reviewer(
            &authority,
            CODEX_APPS_MCP_SERVER_NAME,
            Some("browser-use"),
        ),
        ApprovalsReviewer::AutoReview,
    );
    assert_eq!(
        mcp_elicitation_approvals_reviewer(&authority, "custom-server", Some("browser-use")),
        ApprovalsReviewer::User,
    );
}

fn codex_apps_catalog_tool(
    tool_name: &str,
    connector_id: &str,
    connector_name: &str,
) -> codex_mcp::ToolInfo {
    let mut tool = Tool::new_with_raw(
        tool_name.to_string(),
        Some("Trusted tool description".into()),
        Arc::new(JsonObject::default()),
    );
    tool.title = Some("Trusted tool title".to_string());
    tool.annotations = Some(ToolAnnotations::new().read_only(true));
    codex_mcp::ToolInfo {
        server_name: CODEX_APPS_MCP_SERVER_NAME.to_string(),
        supports_parallel_tool_calls: false,
        server_origin: None,
        callable_name: tool_name.to_string(),
        callable_namespace: format!("mcp__codex_apps__{connector_id}"),
        namespace_description: Some("Trusted connector description".to_string()),
        tool,
        connector_id: Some(connector_id.to_string()),
        connector_name: Some(connector_name.to_string()),
        plugin_display_names: Vec::new(),
    }
}

#[test]
fn codex_apps_elicitation_binds_identity_to_frozen_tool_catalog() {
    let mut request = form_request(guardian_meta(Some(json!({}))));
    request.server_name = CODEX_APPS_MCP_SERVER_NAME.to_string();
    let GuardianElicitationReview::ApprovalRequest(guardian_request) =
        guardian_elicitation_review_request(&request)
    else {
        panic!("expected Guardian MCP tool call request");
    };
    let tools = vec![codex_apps_catalog_tool(
        "access_browser_origin",
        "browser-use",
        "Trusted Browser",
    )];

    let guardian_request = bind_codex_apps_guardian_request_to_catalog(*guardian_request, &tools)
        .expect("frozen catalog identity should match");

    assert_eq!(
        guardian_request,
        crate::guardian::GuardianApprovalRequest::McpToolCall {
            id: "mcp_elicitation:codex_apps:7".to_string(),
            server: CODEX_APPS_MCP_SERVER_NAME.to_string(),
            tool_name: "access_browser_origin".to_string(),
            arguments: Some(json!({})),
            connector_id: Some("browser-use".to_string()),
            connector_name: Some("Trusted Browser".to_string()),
            connector_description: Some("Trusted connector description".to_string()),
            tool_title: Some("Trusted tool title".to_string()),
            tool_description: Some("Trusted tool description".to_string()),
            annotations: Some(crate::guardian::GuardianMcpAnnotations {
                destructive_hint: None,
                open_world_hint: None,
                read_only_hint: Some(true),
            }),
        }
    );
}

#[test]
fn codex_apps_elicitation_rejects_spoofed_connector_or_tool_identity() {
    let tools = vec![codex_apps_catalog_tool(
        "access_browser_origin",
        "browser-use",
        "Trusted Browser",
    )];
    for metadata in [
        json!({
            "codex_approval_kind": "mcp_tool_call",
            "codex_request_type": "approval_request",
            "connector_id": "attacker",
            "tool_name": "access_browser_origin",
        }),
        json!({
            "codex_approval_kind": "mcp_tool_call",
            "codex_request_type": "approval_request",
            "connector_id": "browser-use",
            "tool_name": "attacker_tool",
        }),
    ] {
        let mut request = form_request(meta(metadata));
        request.server_name = CODEX_APPS_MCP_SERVER_NAME.to_string();
        let GuardianElicitationReview::ApprovalRequest(guardian_request) =
            guardian_elicitation_review_request(&request)
        else {
            panic!("expected Guardian MCP tool call request");
        };

        assert_eq!(
            bind_codex_apps_guardian_request_to_catalog(*guardian_request, &tools),
            Err(
                "codex_apps guardian elicitation identity does not match the frozen MCP tool catalog"
            )
        );
    }
}

#[test]
fn guardian_elicitation_review_request_declines_unsupported_opt_in_shapes() {
    let url_request = ElicitationReviewRequest {
        server_name: "browser-use".to_string(),
        request_id: rmcp::model::NumberOrString::Number(8),
        elicitation: CreateElicitationRequestParams::UrlElicitationParams {
            meta: guardian_meta(Some(json!({}))),
            message: "Open URL".to_string(),
            url: "https://example.com".to_string(),
            elicitation_id: "elicit-1".to_string(),
        },
    };
    assert!(matches!(
        guardian_elicitation_review_request(&url_request),
        GuardianElicitationReview::Decline(_)
    ));

    let non_empty_schema_request = ElicitationReviewRequest {
        server_name: "browser-use".to_string(),
        request_id: rmcp::model::NumberOrString::Number(9),
        elicitation: CreateElicitationRequestParams::FormElicitationParams {
            meta: guardian_meta(Some(json!({}))),
            message: "Allow origin?".to_string(),
            requested_schema: ElicitationSchema::builder()
                .required_property("confirmed", PrimitiveSchema::Boolean(BooleanSchema::new()))
                .build()
                .expect("schema should build"),
        },
    };
    assert!(matches!(
        guardian_elicitation_review_request(&non_empty_schema_request),
        GuardianElicitationReview::Decline(_)
    ));

    let missing_tool_name_request = form_request(meta(json!({
        "codex_approval_kind": "mcp_tool_call",
        "codex_request_type": "approval_request",
    })));
    assert!(matches!(
        guardian_elicitation_review_request(&missing_tool_name_request),
        GuardianElicitationReview::Decline(_)
    ));
}

#[test]
fn guardian_decisions_map_to_elicitation_responses_without_session_state() {
    assert_eq!(
        mcp_elicitation_response_from_guardian_decision_parts(
            ReviewDecision::Approved,
            /*denial_message*/ None,
        ),
        ElicitationResponse {
            action: ElicitationAction::Accept,
            content: Some(json!({})),
            meta: Some(json!({
                "approvals_reviewer": ApprovalsReviewer::AutoReview,
            })),
        }
    );
    assert_eq!(
        mcp_elicitation_response_from_guardian_decision_parts(
            ReviewDecision::Denied,
            Some("Denied by Guardian".to_string()),
        ),
        ElicitationResponse {
            action: ElicitationAction::Decline,
            content: None,
            meta: Some(json!({
                "approvals_reviewer": ApprovalsReviewer::AutoReview,
                "message": "Denied by Guardian",
            })),
        }
    );
    assert_eq!(
        mcp_elicitation_response_from_guardian_decision_parts(
            ReviewDecision::TimedOut,
            /*denial_message*/ None,
        ),
        ElicitationResponse {
            action: ElicitationAction::Decline,
            content: None,
            meta: Some(json!({
                "approvals_reviewer": ApprovalsReviewer::AutoReview,
                "message": crate::guardian::guardian_timeout_message(),
            })),
        }
    );
    assert_eq!(
        mcp_elicitation_response_from_guardian_decision_parts(
            ReviewDecision::Abort,
            /*denial_message*/ None,
        ),
        ElicitationResponse {
            action: ElicitationAction::Cancel,
            content: None,
            meta: Some(json!({
                "approvals_reviewer": ApprovalsReviewer::AutoReview,
            })),
        }
    );
}
