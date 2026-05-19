//! Local context snapshot model for Hepta Native.
//!
//! The native client needs to show which agent/task/session/memory/artifact
//! context a staged action would carry. This model is local-only and does not
//! read memory files, session stores, task registries, or external services.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaContextKind {
    Agent,
    Task,
    Session,
    Memory,
    Artifact,
}

impl HeptaContextKind {
    pub fn chip_prefix(self) -> &'static str {
        match self {
            Self::Agent => "@",
            Self::Task => "#",
            Self::Session => "session:",
            Self::Memory => "memory:",
            Self::Artifact => "artifact:",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Task => "task",
            Self::Session => "session",
            Self::Memory => "memory",
            Self::Artifact => "artifact",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaContextChip {
    pub kind: HeptaContextKind,
    pub value: &'static str,
    pub description: &'static str,
    pub read_only: bool,
}

impl HeptaContextChip {
    pub fn token(&self) -> String {
        format!("{}{}", self.kind.chip_prefix(), self.value)
    }

    pub fn operator_line(&self) -> String {
        format!(
            "{} · {} · {} · read_only={}",
            self.token(),
            self.kind.label(),
            self.description,
            self.read_only,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaContextSnapshot {
    pub title: &'static str,
    pub chips: Vec<HeptaContextChip>,
}

impl HeptaContextSnapshot {
    pub fn count_kind(&self, kind: HeptaContextKind) -> usize {
        self.chips.iter().filter(|chip| chip.kind == kind).count()
    }

    pub fn summary_line(&self) -> String {
        format!(
            "{} agents · {} tasks · {} sessions · {} memories · {} artifacts",
            self.count_kind(HeptaContextKind::Agent),
            self.count_kind(HeptaContextKind::Task),
            self.count_kind(HeptaContextKind::Session),
            self.count_kind(HeptaContextKind::Memory),
            self.count_kind(HeptaContextKind::Artifact),
        )
    }

    pub fn tokens_line(&self) -> String {
        self.chips
            .iter()
            .map(HeptaContextChip::token)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn sample_context_snapshot() -> HeptaContextSnapshot {
    HeptaContextSnapshot {
        title: "Context snapshot",
        chips: vec![
            HeptaContextChip {
                kind: HeptaContextKind::Agent,
                value: "main",
                description: "current Hepta Native operator surface",
                read_only: true,
            },
            HeptaContextChip {
                kind: HeptaContextKind::Task,
                value: "hepta-native-full-client",
                description: "Matrix-heart full-client UI buildout",
                read_only: true,
            },
            HeptaContextChip {
                kind: HeptaContextKind::Session,
                value: "current",
                description: "active Telegram-requested development turn",
                read_only: true,
            },
            HeptaContextChip {
                kind: HeptaContextKind::Memory,
                value: "2026-05-14",
                description: "durable Hepta UI continuation notes",
                read_only: true,
            },
            HeptaContextChip {
                kind: HeptaContextKind::Artifact,
                value: "hepta-native-gates",
                description: "cargo check/test and control-ui smoke evidence",
                read_only: true,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_context_snapshot_contains_all_composer_chip_classes() {
        let snapshot = sample_context_snapshot();
        assert_eq!(snapshot.count_kind(HeptaContextKind::Agent), 1);
        assert_eq!(snapshot.count_kind(HeptaContextKind::Task), 1);
        assert_eq!(snapshot.count_kind(HeptaContextKind::Session), 1);
        assert_eq!(snapshot.count_kind(HeptaContextKind::Memory), 1);
        assert_eq!(snapshot.count_kind(HeptaContextKind::Artifact), 1);
        assert!(snapshot.chips.iter().all(|chip| chip.read_only));
    }

    #[test]
    fn tokens_line_matches_composer_context_syntax() {
        let snapshot = sample_context_snapshot();
        let tokens = snapshot.tokens_line();
        assert!(tokens.contains("@main"));
        assert!(tokens.contains("#hepta-native-full-client"));
        assert!(tokens.contains("session:current"));
        assert!(tokens.contains("memory:2026-05-14"));
        assert!(tokens.contains("artifact:hepta-native-gates"));
    }
}
