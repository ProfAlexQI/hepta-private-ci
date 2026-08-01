#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GateScriptKind {
    Gate,
    Report,
}

impl GateScriptKind {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Gate => "gate",
            Self::Report => "report",
        }
    }

    pub(super) fn candidate_names(self, id: &str) -> Vec<String> {
        match self {
            Self::Gate => vec![
                format!("{id}-gate.sh"),
                format!("{id}-route-gate.sh"),
                format!("{id}-lane-gate.sh"),
            ],
            Self::Report => vec![format!("{id}-report.sh")],
        }
    }
}
