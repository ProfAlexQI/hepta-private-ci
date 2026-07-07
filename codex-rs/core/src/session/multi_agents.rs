use crate::session::turn_context::TurnContext;
use codex_features::Feature;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;

pub(crate) const MULTI_AGENT_USAGE_HINT_SOURCE_ID: &str = "multi_agent_usage_hint";
pub(crate) const MULTI_AGENT_USAGE_HINT_OPEN_TAG: &str = "<multi_agent_usage_hint>";
pub(crate) const MULTI_AGENT_USAGE_HINT_CLOSE_TAG: &str = "</multi_agent_usage_hint>";

pub(super) fn usage_hint_text<'a>(
    turn_context: &'a TurnContext,
    session_source: &SessionSource,
) -> Option<&'a str> {
    if !turn_context.features.enabled(Feature::MultiAgentV2) {
        return None;
    }

    let multi_agent_v2 = &turn_context.config.multi_agent_v2;
    match session_source {
        SessionSource::SubAgent(SubAgentSource::ThreadSpawn { .. }) => {
            multi_agent_v2.subagent_usage_hint_text.as_deref()
        }
        SessionSource::Cli
        | SessionSource::VSCode
        | SessionSource::Exec
        | SessionSource::Mcp
        | SessionSource::Custom(_)
        | SessionSource::Unknown => multi_agent_v2.root_agent_usage_hint_text.as_deref(),
        SessionSource::Internal(_) | SessionSource::SubAgent(_) => None,
    }
}

pub(crate) fn render_usage_hint(text: &str) -> String {
    format!("{MULTI_AGENT_USAGE_HINT_OPEN_TAG}\n{text}\n{MULTI_AGENT_USAGE_HINT_CLOSE_TAG}")
}

pub(crate) fn render_usage_hint_clear() -> String {
    render_usage_hint(
        "Multi-agent usage hint was cleared. Do not continue applying previously injected multi-agent usage guidance.",
    )
}
