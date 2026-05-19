//! Local quick-command templates for Hepta Native.
//!
//! These are UI suggestions only. They are validated through the same composer
//! dry-run planner so adding a template cannot silently bypass local-only policy.

use crate::hepta_composer::{plan_hepta_composer_command, HeptaComposerPlan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaCommandTemplate {
    pub label: &'static str,
    pub command: &'static str,
    pub description: &'static str,
}

impl HeptaCommandTemplate {
    pub fn plan(&self) -> HeptaComposerPlan {
        plan_hepta_composer_command(self.command, 0)
            .expect("Hepta command templates must always parse")
    }

    pub fn operator_line(&self) -> String {
        let plan = self.plan();
        format!(
            "{} · {} · confirmation={} · external_mutation_enabled={}",
            self.command,
            plan.mutation_class(),
            plan.requires_confirmation(),
            plan.external_mutation_enabled,
        )
    }
}

pub fn sample_command_templates() -> Vec<HeptaCommandTemplate> {
    vec![
        HeptaCommandTemplate {
            label: "Read current runtime status",
            command: "/status session:current",
            description: "read-only status preview for the active workspace",
        },
        HeptaCommandTemplate {
            label: "Draft a task",
            command: "/task verify mobile confirmation UX #hepta-native-full-client @main",
            description: "stages a task plan without task-registry mutation",
        },
        HeptaCommandTemplate {
            label: "Draft an agent instruction",
            command: "/agent @main inspect artifact:hepta-native-gates",
            description: "stages an agent instruction without subagent control",
        },
        HeptaCommandTemplate {
            label: "Draft a tool call",
            command: "/tool exec cargo check --manifest-path apps/hepta-native/Cargo.toml",
            description: "stages an exact tool payload preview only",
        },
        HeptaCommandTemplate {
            label: "Draft an approval decision",
            command: "/approve approval-install-cargo-makepad",
            description: "stages approval intent without approving any tool execution",
        },
    ]
}

pub fn template_summary_line() -> String {
    let templates = sample_command_templates();
    let confirmation_required = templates
        .iter()
        .filter(|template| template.plan().requires_confirmation())
        .count();
    let read_only = templates.len() - confirmation_required;
    format!(
        "{} templates · {} read-only · {} confirmation-required",
        templates.len(),
        read_only,
        confirmation_required,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_templates_parse_through_safe_composer_planner() {
        let templates = sample_command_templates();
        assert_eq!(templates.len(), 5);
        for template in templates {
            let plan = template.plan();
            assert!(!plan.external_mutation_enabled);
            assert!(!plan.operator_summary().is_empty());
        }
    }

    #[test]
    fn template_summary_separates_read_only_from_confirmation_required() {
        let summary = template_summary_line();
        assert!(summary.contains("5 templates"));
        assert!(summary.contains("1 read-only"));
        assert!(summary.contains("4 confirmation-required"));
    }
}
