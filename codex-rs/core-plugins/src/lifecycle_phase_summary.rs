use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginLifecyclePhase {
    ManifestFixtureDiscovered,
    ContributionPointAbiAudited,
    LoaderBindingAudited,
    FixturePolicyMetadataAudited,
    ToolPreviewContractAudited,
    LiveMutationBlocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginLifecyclePhaseState {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginLifecyclePhaseSummary {
    pub phase: PluginLifecyclePhase,
    pub state: PluginLifecyclePhaseState,
    pub source_surface: &'static str,
    pub ready: bool,
    pub live_mutation_enabled: bool,
    pub blocker: Option<&'static str>,
}

impl PluginLifecyclePhaseSummary {
    pub fn ready(phase: PluginLifecyclePhase, source_surface: &'static str) -> Self {
        Self {
            phase,
            state: PluginLifecyclePhaseState::Ready,
            source_surface,
            ready: true,
            live_mutation_enabled: false,
            blocker: None,
        }
    }

    pub fn blocked(
        phase: PluginLifecyclePhase,
        source_surface: &'static str,
        blocker: &'static str,
    ) -> Self {
        Self {
            phase,
            state: PluginLifecyclePhaseState::Blocked,
            source_surface,
            ready: false,
            live_mutation_enabled: false,
            blocker: Some(blocker),
        }
    }
}

pub fn ready_phase_count(phases: &[PluginLifecyclePhaseSummary]) -> usize {
    phases.iter().filter(|phase| phase.ready).count()
}

pub fn blocked_phase_count(phases: &[PluginLifecyclePhaseSummary]) -> usize {
    phases.iter().filter(|phase| !phase.ready).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_phase_summary_separates_ready_and_blocked_states() {
        let phases = vec![
            PluginLifecyclePhaseSummary::ready(
                PluginLifecyclePhase::ManifestFixtureDiscovered,
                "manifest_fixture",
            ),
            PluginLifecyclePhaseSummary::blocked(
                PluginLifecyclePhase::ToolPreviewContractAudited,
                "plugin_tool_preview",
                "tool_preview_contract_missing",
            ),
        ];

        assert_eq!(ready_phase_count(&phases), 1);
        assert_eq!(blocked_phase_count(&phases), 1);
        assert!(!phases[0].live_mutation_enabled);
        assert_eq!(phases[1].blocker, Some("tool_preview_contract_missing"));
    }
}
