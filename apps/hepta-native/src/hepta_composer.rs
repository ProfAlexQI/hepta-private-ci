//! Local Hepta composer command parsing and dry-run planning.
//!
//! The Robrix/Matrix composer still sends normal Matrix messages. These helpers
//! reserve explicit Hepta commands for the action bridge so native task/tool/
//! approval/status intents can be previewed without accidentally treating every
//! slash command as a runtime mutation.

use crate::{
    hepta_action_bridge::{
        HeptaActionBridgeDecision, MUTATION_DRAFT_AGENT_INSTRUCTION,
        MUTATION_DRAFT_APPROVAL_DECISION, MUTATION_DRAFT_TASK_PLAN, MUTATION_DRAFT_TOOL_CALL,
        MUTATION_READ_ONLY_RUNTIME_COMMAND,
    },
    hepta_bridge::HeptaBridgeEventInput,
    hepta_event::HeptaEventStatus,
};
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeptaComposerCommand {
    Task { summary: String },
    Agent { target: String, instruction: String },
    Tool { name: String, args: String },
    Approve { target: String },
    Reject { target: String },
    Status { target: Option<String> },
}

impl HeptaComposerCommand {
    pub fn operator_summary(&self) -> String {
        match self {
            Self::Task { summary } => format!("task draft · {summary}"),
            Self::Agent {
                target,
                instruction,
            } if instruction.is_empty() => {
                format!("agent draft · {target}")
            }
            Self::Agent {
                target,
                instruction,
            } => {
                format!("agent draft · {target} · {instruction}")
            }
            Self::Tool { name, args } if args.is_empty() => format!("tool draft · {name}"),
            Self::Tool { name, args } => format!("tool draft · {name} {args}"),
            Self::Approve { target } => format!("approval approve · {target}"),
            Self::Reject { target } => format!("approval reject · {target}"),
            Self::Status {
                target: Some(target),
            } => format!("status read-only · {target}"),
            Self::Status { target: None } => "status read-only · current workspace".to_string(),
        }
    }

    fn action_name(&self) -> &'static str {
        match self {
            Self::Task { .. } => "task",
            Self::Agent { .. } => "agent",
            Self::Tool { .. } => "tool",
            Self::Approve { .. } => "approve",
            Self::Reject { .. } => "reject",
            Self::Status { .. } => "status",
        }
    }

    fn event_kind(&self) -> &'static str {
        match self {
            Self::Task { .. } => "task",
            Self::Agent { .. } => "agent_run",
            Self::Tool { .. } => "tool_call",
            Self::Approve { .. } | Self::Reject { .. } => "approval_result",
            Self::Status { .. } => "runtime_event",
        }
    }

    fn mutation_class(&self) -> &'static str {
        match self {
            Self::Task { .. } => MUTATION_DRAFT_TASK_PLAN,
            Self::Agent { .. } => MUTATION_DRAFT_AGENT_INSTRUCTION,
            Self::Tool { .. } => MUTATION_DRAFT_TOOL_CALL,
            Self::Approve { .. } | Self::Reject { .. } => MUTATION_DRAFT_APPROVAL_DECISION,
            Self::Status { .. } => MUTATION_READ_ONLY_RUNTIME_COMMAND,
        }
    }

    fn requires_confirmation(&self) -> bool {
        !matches!(self, Self::Status { .. })
    }

    fn payload_value(&self) -> serde_json::Value {
        match self {
            Self::Task { summary } => json!({
                "summary": summary,
            }),
            Self::Agent {
                target,
                instruction,
            } => json!({
                "target": target,
                "instruction": instruction,
            }),
            Self::Tool { name, args } => json!({
                "tool_name": name,
                "args": args,
            }),
            Self::Approve { target } => json!({
                "decision": "approve",
                "target": target,
            }),
            Self::Reject { target } => json!({
                "decision": "reject",
                "target": target,
            }),
            Self::Status { target } => json!({
                "target": target,
            }),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeptaComposerContext {
    pub agents: Vec<String>,
    pub tasks: Vec<String>,
    pub sessions: Vec<String>,
    pub memory_citations: Vec<String>,
    pub artifacts: Vec<String>,
}

impl HeptaComposerContext {
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
            && self.tasks.is_empty()
            && self.sessions.is_empty()
            && self.memory_citations.is_empty()
            && self.artifacts.is_empty()
    }

    pub fn operator_summary(&self) -> String {
        if self.is_empty() {
            return "no explicit context chips".to_string();
        }
        let mut parts = Vec::new();
        if !self.agents.is_empty() {
            parts.push(format!("agents={}", self.agents.join(",")));
        }
        if !self.tasks.is_empty() {
            parts.push(format!("tasks={}", self.tasks.join(",")));
        }
        if !self.sessions.is_empty() {
            parts.push(format!("sessions={}", self.sessions.join(",")));
        }
        if !self.memory_citations.is_empty() {
            parts.push(format!("memory={}", self.memory_citations.join(",")));
        }
        if !self.artifacts.is_empty() {
            parts.push(format!("artifacts={}", self.artifacts.join(",")));
        }
        parts.join(" · ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaComposerPlan {
    pub command: HeptaComposerCommand,
    pub context: HeptaComposerContext,
    pub created_at_ms: u64,
    pub external_mutation_enabled: bool,
}

impl HeptaComposerPlan {
    pub fn event_kind(&self) -> &'static str {
        self.command.event_kind()
    }

    pub fn mutation_class(&self) -> &'static str {
        self.command.mutation_class()
    }

    pub fn requires_confirmation(&self) -> bool {
        self.command.requires_confirmation()
    }

    pub fn operator_summary(&self) -> String {
        format!(
            "{} · {} · {}",
            self.command.operator_summary(),
            self.mutation_class(),
            self.context.operator_summary(),
        )
    }

    pub fn to_bridge_input(&self) -> HeptaBridgeEventInput {
        let summary = self.command.operator_summary();
        let mut input = HeptaBridgeEventInput::new(
            self.event_kind(),
            format!(
                "composer-draft-{}-{}",
                self.created_at_ms,
                self.command.action_name()
            ),
            HeptaEventStatus::Waiting,
            format!("Dry-run composer preview: {summary}"),
        );
        input.created_at_ms = self.created_at_ms;
        let bridge_policy = HeptaActionBridgeDecision::preview_only(
            self.mutation_class(),
            self.requires_confirmation(),
        );
        input.payload = json!({
            "title": format!("Draft {} action", self.command.action_name()),
            "body": summary,
            "action": self.command.action_name(),
            "mutation_class": self.mutation_class(),
            "requires_confirmation": self.requires_confirmation(),
            "external_mutation_enabled": self.external_mutation_enabled,
            "bridge_policy": bridge_policy.as_payload_value(),
            "command_payload": self.command.payload_value(),
            "context": {
                "agents": &self.context.agents,
                "tasks": &self.context.tasks,
                "sessions": &self.context.sessions,
                "memory_citations": &self.context.memory_citations,
                "artifacts": &self.context.artifacts,
            },
        });
        input
    }
}

pub fn parse_hepta_composer_command(input: &str) -> Option<HeptaComposerCommand> {
    let trimmed = input.trim();
    if let Some(rest) = strip_hepta_prefix(trimmed) {
        return parse_hepta_command_rest(rest);
    }
    parse_direct_hepta_slash_command(trimmed)
}

pub fn plan_hepta_composer_command(input: &str, created_at_ms: u64) -> Option<HeptaComposerPlan> {
    let command = parse_hepta_composer_command(input)?;
    Some(HeptaComposerPlan {
        command,
        context: collect_context_chips(input),
        created_at_ms,
        // M4 is intentionally dry-run only. Phase 5 enables mutation classes one by one.
        external_mutation_enabled: false,
    })
}

pub fn looks_like_hepta_composer_command(input: &str) -> bool {
    let trimmed = input.trim();
    if strip_hepta_prefix(trimmed).is_some() {
        return true;
    }
    let Some(rest) = trimmed.strip_prefix('/') else {
        return false;
    };
    let verb = rest
        .split_whitespace()
        .next()
        .unwrap_or(rest)
        .trim()
        .to_ascii_lowercase();
    matches!(
        verb.as_str(),
        "task" | "agent" | "tool" | "approve" | "reject" | "status"
    )
}

fn strip_hepta_prefix(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix("/hepta")?;
    if rest.is_empty() || rest.chars().next().is_some_and(char::is_whitespace) {
        Some(rest.trim())
    } else {
        None
    }
}

fn parse_direct_hepta_slash_command(trimmed: &str) -> Option<HeptaComposerCommand> {
    let rest = trimmed.strip_prefix('/')?;
    let mut parts = rest.splitn(2, char::is_whitespace);
    let verb = parts.next()?.trim().to_ascii_lowercase();
    if !matches!(
        verb.as_str(),
        "task" | "agent" | "tool" | "approve" | "reject" | "status"
    ) {
        return None;
    }
    let body = parts.next().unwrap_or("").trim();
    parse_command_parts(&verb, body)
}

fn parse_hepta_command_rest(rest: &str) -> Option<HeptaComposerCommand> {
    if rest.is_empty() {
        return None;
    }
    let mut parts = rest.splitn(2, char::is_whitespace);
    let verb = parts.next()?.trim().to_ascii_lowercase();
    let body = parts.next().unwrap_or("").trim();
    parse_command_parts(&verb, body)
}

fn parse_command_parts(verb: &str, body: &str) -> Option<HeptaComposerCommand> {
    match verb {
        "task" if !body.is_empty() => Some(HeptaComposerCommand::Task {
            summary: body.to_string(),
        }),
        "agent" => parse_agent_command(body),
        "tool" => parse_tool_command(body),
        "approve" if !body.is_empty() => Some(HeptaComposerCommand::Approve {
            target: body.to_string(),
        }),
        "reject" if !body.is_empty() => Some(HeptaComposerCommand::Reject {
            target: body.to_string(),
        }),
        "status" => Some(HeptaComposerCommand::Status {
            target: (!body.is_empty()).then(|| body.to_string()),
        }),
        _ => None,
    }
}

fn parse_agent_command(body: &str) -> Option<HeptaComposerCommand> {
    let mut parts = body.splitn(2, char::is_whitespace);
    let target = parts.next()?.trim();
    if target.is_empty() {
        return None;
    }
    Some(HeptaComposerCommand::Agent {
        target: target.to_string(),
        instruction: parts.next().unwrap_or("").trim().to_string(),
    })
}

fn parse_tool_command(body: &str) -> Option<HeptaComposerCommand> {
    let mut parts = body.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim();
    if name.is_empty() {
        return None;
    }
    Some(HeptaComposerCommand::Tool {
        name: name.to_string(),
        args: parts.next().unwrap_or("").trim().to_string(),
    })
}

fn collect_context_chips(input: &str) -> HeptaComposerContext {
    let mut context = HeptaComposerContext::default();
    for raw in input.split_whitespace() {
        let token = normalize_context_token(raw);
        if token.len() < 2 {
            continue;
        }
        if token.starts_with('@') {
            push_unique(&mut context.agents, token);
        } else if token.starts_with('#') {
            push_unique(&mut context.tasks, token);
        } else if let Some(value) = token.strip_prefix("session:") {
            push_unique(&mut context.sessions, value.to_string());
        } else if let Some(value) = token.strip_prefix("memory:") {
            push_unique(&mut context.memory_citations, value.to_string());
        } else if let Some(value) = token.strip_prefix("artifact:") {
            push_unique(&mut context.artifacts, value.to_string());
        }
    }
    context
}

fn normalize_context_token(raw: &str) -> String {
    raw.trim_matches(|c: char| {
        matches!(
            c,
            ',' | ';' | ')' | '(' | '[' | ']' | '{' | '}' | '"' | '\''
        )
    })
    .to_string()
}

fn push_unique(target: &mut Vec<String>, value: String) {
    if !target.iter().any(|existing| existing == &value) {
        target.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        hepta_bridge::HeptaBridgeMatrixEvent,
        hepta_event::{HeptaEventEnvelope, EVENT_AGENT_RUN, EVENT_RUNTIME_EVENT, EVENT_TASK},
    };

    #[test]
    fn ignores_normal_matrix_messages_and_unknown_slash_commands() {
        assert_eq!(parse_hepta_composer_command("hello"), None);
        assert_eq!(
            parse_hepta_composer_command("/join #room:example.org"),
            None
        );
        assert_eq!(parse_hepta_composer_command("/hepta"), None);
        assert_eq!(parse_hepta_composer_command("/heptatask value"), None);
        assert_eq!(parse_hepta_composer_command("/hepta unknown value"), None);
    }

    #[test]
    fn parses_prefixed_task_tool_and_approval_commands() {
        assert_eq!(
            parse_hepta_composer_command("/hepta task summarize this runtime thread"),
            Some(HeptaComposerCommand::Task {
                summary: "summarize this runtime thread".to_string(),
            }),
        );
        assert_eq!(
            parse_hepta_composer_command("/hepta tool exec cargo check"),
            Some(HeptaComposerCommand::Tool {
                name: "exec".to_string(),
                args: "cargo check".to_string(),
            }),
        );
        assert_eq!(
            parse_hepta_composer_command("/hepta approve approval-123"),
            Some(HeptaComposerCommand::Approve {
                target: "approval-123".to_string(),
            }),
        );
        assert_eq!(
            parse_hepta_composer_command("/hepta reject approval-123"),
            Some(HeptaComposerCommand::Reject {
                target: "approval-123".to_string(),
            }),
        );
    }

    #[test]
    fn parses_direct_phase_four_slash_commands() {
        assert_eq!(
            parse_hepta_composer_command("/task close the mobile keyboard gate"),
            Some(HeptaComposerCommand::Task {
                summary: "close the mobile keyboard gate".to_string(),
            }),
        );
        assert_eq!(
            parse_hepta_composer_command("/agent @main inspect #task-7"),
            Some(HeptaComposerCommand::Agent {
                target: "@main".to_string(),
                instruction: "inspect #task-7".to_string(),
            }),
        );
        assert_eq!(
            parse_hepta_composer_command("/status"),
            Some(HeptaComposerCommand::Status { target: None }),
        );
        assert_eq!(
            parse_hepta_composer_command("/status session:current"),
            Some(HeptaComposerCommand::Status {
                target: Some("session:current".to_string()),
            }),
        );
    }

    #[test]
    fn command_prefix_detection_suppresses_matrix_typing_notice() {
        assert!(looks_like_hepta_composer_command("/task"));
        assert!(looks_like_hepta_composer_command("/hepta approve"));
        assert!(looks_like_hepta_composer_command(" /status current"));
        assert!(!looks_like_hepta_composer_command(
            "/topic normal matrix slash"
        ));
        assert!(!looks_like_hepta_composer_command("hello /task later"));
    }

    #[test]
    fn collects_context_chips_for_dry_run_plans() {
        let plan = plan_hepta_composer_command(
            "/agent @main inspect #task-7 session:current memory:2026-05-14 artifact:build-log @main",
            42,
        )
        .unwrap();
        assert_eq!(plan.context.agents, vec!["@main"]);
        assert_eq!(plan.context.tasks, vec!["#task-7"]);
        assert_eq!(plan.context.sessions, vec!["current"]);
        assert_eq!(plan.context.memory_citations, vec!["2026-05-14"]);
        assert_eq!(plan.context.artifacts, vec!["build-log"]);
        assert!(plan.requires_confirmation());
        assert!(!plan.external_mutation_enabled);
    }

    #[test]
    fn summaries_are_operator_readable() {
        let command = parse_hepta_composer_command("/hepta tool read MEMORY.md").unwrap();
        assert_eq!(command.operator_summary(), "tool draft · read MEMORY.md");
        let plan = plan_hepta_composer_command("/status", 0).unwrap();
        assert!(
            plan.operator_summary()
                .contains("read_only_runtime_command")
        );
    }

    #[test]
    fn dry_run_plan_becomes_matrix_shaped_preview_event() {
        let plan = plan_hepta_composer_command("/task close M4 #task-m4 @main", 99).unwrap();
        assert_eq!(plan.event_kind(), "task");
        let input = plan.to_bridge_input();
        assert_eq!(input.event_kind, "task");
        assert_eq!(input.id, "composer-draft-99-task");
        assert_eq!(input.status, HeptaEventStatus::Waiting);
        let event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@user-local:local",
            input,
        )
        .unwrap();
        assert_eq!(event.event_type, EVENT_TASK);
        let envelope = HeptaEventEnvelope::from_content_value(&event.content).unwrap();
        assert_eq!(
            envelope.payload_str("mutation_class"),
            Some("draft_task_plan")
        );
        assert_eq!(envelope.payload_str("action"), Some("task"));
        let bridge_policy = envelope.payload.get("bridge_policy").unwrap();
        assert_eq!(
            bridge_policy
                .get("disposition")
                .and_then(serde_json::Value::as_str),
            Some("draft_preview")
        );
        assert_eq!(
            bridge_policy
                .get("external_mutation_enabled")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
    }

    #[test]
    fn dry_run_status_and_agent_map_to_expected_event_types() {
        let status = plan_hepta_composer_command("/status", 1).unwrap();
        let status_event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@user-local:local",
            status.to_bridge_input(),
        )
        .unwrap();
        assert_eq!(status_event.event_type, EVENT_RUNTIME_EVENT);
        assert!(!status.requires_confirmation());

        let agent = plan_hepta_composer_command("/agent @subagent summarize", 2).unwrap();
        let agent_event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@user-local:local",
            agent.to_bridge_input(),
        )
        .unwrap();
        assert_eq!(agent_event.event_type, EVENT_AGENT_RUN);
    }
}
