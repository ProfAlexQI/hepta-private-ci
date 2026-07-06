use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryInventorySource {
    BuiltIn,
    Mcp,
    Dynamic,
    Plugin,
    Connector,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryInventorySideEffectLevel {
    Unknown,
    ReadOnly,
    LocalMutation,
    ExternalMutation,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryInventoryApprovalKind {
    Unknown,
    NotRequired,
    OnUse,
    Install,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryInvocationGuardRoute {
    AllowReadOnlyLedger,
    RequireApprovalLedger,
    BlockUnknownMetadata,
    BlockMissingLedger,
    BlockInvalidPolicy,
    BlockDuplicateId,
    BlockUnknownTool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInventoryEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub source: ToolRegistryInventorySource,
    pub owner: Option<String>,
    pub has_input_schema: bool,
    pub has_output_schema: bool,
    pub defer_loading: bool,
    pub has_skills: bool,
    pub mcp_server_names: Vec<String>,
    pub app_connector_ids: Vec<String>,
    pub side_effect_level: ToolRegistryInventorySideEffectLevel,
    pub approval_kind: ToolRegistryInventoryApprovalKind,
    pub auth_required: bool,
    pub timeout_ms: Option<u64>,
    pub ledger_required: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInvocationGuardEntry {
    pub id: String,
    pub name: String,
    pub source: ToolRegistryInventorySource,
    pub side_effect_level: ToolRegistryInventorySideEffectLevel,
    pub approval_kind: ToolRegistryInventoryApprovalKind,
    pub auth_required: bool,
    pub ledger_required: bool,
    pub route: ToolRegistryInvocationGuardRoute,
    pub blocked_reason: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInvocationGuardDecision {
    pub tool_id: String,
    pub entry_found: bool,
    pub duplicate_id: bool,
    pub route: ToolRegistryInvocationGuardRoute,
    pub approval_required: bool,
    pub ledger_required: bool,
    pub blocked: bool,
    pub blocked_reason: Option<&'static str>,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub side_effect_free: bool,
    pub entry: Option<ToolRegistryInvocationGuardEntry>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInvocationGuardReport {
    pub entry_count: usize,
    pub allow_read_only_ledger_count: usize,
    pub require_approval_ledger_count: usize,
    pub block_unknown_metadata_count: usize,
    pub block_missing_ledger_count: usize,
    pub block_invalid_policy_count: usize,
    pub blocked_count: usize,
    pub duplicate_id_count: usize,
    pub duplicate_ids: Vec<String>,
    pub all_entries_require_ledger: bool,
    pub unknown_metadata_blocked: bool,
    pub mutating_tools_require_approval: bool,
    pub invocation_guard_ready: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub side_effect_free: bool,
    pub entries: Vec<ToolRegistryInvocationGuardEntry>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct ToolRegistryInventory {
    pub entries: Vec<ToolRegistryInventoryEntry>,
}

impl ToolRegistryInventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn duplicate_ids(&self) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut duplicates = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.id.clone()) {
                duplicates.insert(entry.id.clone());
            }
        }
        duplicates.into_iter().collect()
    }

    pub fn plan_invocation_guard(&self) -> ToolRegistryInvocationGuardReport {
        let entries = self
            .entries
            .iter()
            .map(invocation_guard_entry)
            .collect::<Vec<_>>();
        let duplicate_ids = self.duplicate_ids();
        let allow_read_only_ledger_count = count_guard_route(
            &entries,
            ToolRegistryInvocationGuardRoute::AllowReadOnlyLedger,
        );
        let require_approval_ledger_count = count_guard_route(
            &entries,
            ToolRegistryInvocationGuardRoute::RequireApprovalLedger,
        );
        let block_unknown_metadata_count = count_guard_route(
            &entries,
            ToolRegistryInvocationGuardRoute::BlockUnknownMetadata,
        );
        let block_missing_ledger_count = count_guard_route(
            &entries,
            ToolRegistryInvocationGuardRoute::BlockMissingLedger,
        );
        let block_invalid_policy_count = count_guard_route(
            &entries,
            ToolRegistryInvocationGuardRoute::BlockInvalidPolicy,
        );
        let blocked_count =
            block_unknown_metadata_count + block_missing_ledger_count + block_invalid_policy_count;
        let all_entries_require_ledger = entries.iter().all(|entry| entry.ledger_required);
        let unknown_metadata_blocked = entries.iter().all(|entry| {
            if entry.side_effect_level == ToolRegistryInventorySideEffectLevel::Unknown
                || entry.approval_kind == ToolRegistryInventoryApprovalKind::Unknown
            {
                entry.route == ToolRegistryInvocationGuardRoute::BlockUnknownMetadata
            } else {
                true
            }
        });
        let mutating_tools_require_approval = entries.iter().all(|entry| {
            if matches!(
                entry.side_effect_level,
                ToolRegistryInventorySideEffectLevel::LocalMutation
                    | ToolRegistryInventorySideEffectLevel::ExternalMutation
            ) {
                matches!(
                    entry.approval_kind,
                    ToolRegistryInventoryApprovalKind::OnUse
                        | ToolRegistryInventoryApprovalKind::Install
                )
            } else {
                true
            }
        });
        let invocation_guard_ready = duplicate_ids.is_empty()
            && all_entries_require_ledger
            && unknown_metadata_blocked
            && mutating_tools_require_approval
            && block_invalid_policy_count == 0;

        ToolRegistryInvocationGuardReport {
            entry_count: entries.len(),
            allow_read_only_ledger_count,
            require_approval_ledger_count,
            block_unknown_metadata_count,
            block_missing_ledger_count,
            block_invalid_policy_count,
            blocked_count,
            duplicate_id_count: duplicate_ids.len(),
            duplicate_ids,
            all_entries_require_ledger,
            unknown_metadata_blocked,
            mutating_tools_require_approval,
            invocation_guard_ready,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            side_effect_free: true,
            entries,
        }
    }

    pub fn plan_invocation_guard_for_tool(
        &self,
        tool_id: impl AsRef<str>,
    ) -> ToolRegistryInvocationGuardDecision {
        let tool_id = tool_id.as_ref();
        let matches = self
            .entries
            .iter()
            .filter(|entry| entry.id == tool_id)
            .collect::<Vec<_>>();

        if matches.is_empty() {
            return ToolRegistryInvocationGuardDecision {
                tool_id: tool_id.to_string(),
                entry_found: false,
                duplicate_id: false,
                route: ToolRegistryInvocationGuardRoute::BlockUnknownTool,
                approval_required: false,
                ledger_required: true,
                blocked: true,
                blocked_reason: Some("tool_id_not_registered"),
                tool_invoked: false,
                ledger_written: false,
                approval_requested: false,
                side_effect_free: true,
                entry: None,
            };
        }

        if matches.len() > 1 {
            return ToolRegistryInvocationGuardDecision {
                tool_id: tool_id.to_string(),
                entry_found: true,
                duplicate_id: true,
                route: ToolRegistryInvocationGuardRoute::BlockDuplicateId,
                approval_required: false,
                ledger_required: true,
                blocked: true,
                blocked_reason: Some("duplicate_tool_id"),
                tool_invoked: false,
                ledger_written: false,
                approval_requested: false,
                side_effect_free: true,
                entry: None,
            };
        }

        let entry = invocation_guard_entry(matches[0]);
        let route = entry.route;

        ToolRegistryInvocationGuardDecision {
            tool_id: tool_id.to_string(),
            entry_found: true,
            duplicate_id: false,
            approval_required: route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger,
            ledger_required: entry.ledger_required,
            blocked: invocation_guard_route_is_blocked(route),
            blocked_reason: entry.blocked_reason,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            side_effect_free: true,
            route,
            entry: Some(entry),
        }
    }
}

fn count_guard_route(
    entries: &[ToolRegistryInvocationGuardEntry],
    route: ToolRegistryInvocationGuardRoute,
) -> usize {
    entries.iter().filter(|entry| entry.route == route).count()
}

fn invocation_guard_entry(entry: &ToolRegistryInventoryEntry) -> ToolRegistryInvocationGuardEntry {
    let (route, blocked_reason) = invocation_guard_route(entry);
    ToolRegistryInvocationGuardEntry {
        id: entry.id.clone(),
        name: entry.name.clone(),
        source: entry.source,
        side_effect_level: entry.side_effect_level,
        approval_kind: entry.approval_kind,
        auth_required: entry.auth_required,
        ledger_required: entry.ledger_required,
        route,
        blocked_reason,
    }
}

fn invocation_guard_route(
    entry: &ToolRegistryInventoryEntry,
) -> (ToolRegistryInvocationGuardRoute, Option<&'static str>) {
    if !entry.ledger_required {
        return (
            ToolRegistryInvocationGuardRoute::BlockMissingLedger,
            Some("ledger_required_false"),
        );
    }

    if entry.side_effect_level == ToolRegistryInventorySideEffectLevel::Unknown
        || entry.approval_kind == ToolRegistryInventoryApprovalKind::Unknown
    {
        return (
            ToolRegistryInvocationGuardRoute::BlockUnknownMetadata,
            Some("unknown_side_effect_or_approval_metadata"),
        );
    }

    match (entry.side_effect_level, entry.approval_kind) {
        (
            ToolRegistryInventorySideEffectLevel::ReadOnly,
            ToolRegistryInventoryApprovalKind::NotRequired,
        ) => (ToolRegistryInvocationGuardRoute::AllowReadOnlyLedger, None),
        (
            ToolRegistryInventorySideEffectLevel::LocalMutation
            | ToolRegistryInventorySideEffectLevel::ExternalMutation,
            ToolRegistryInventoryApprovalKind::OnUse | ToolRegistryInventoryApprovalKind::Install,
        ) => (
            ToolRegistryInvocationGuardRoute::RequireApprovalLedger,
            None,
        ),
        _ => (
            ToolRegistryInvocationGuardRoute::BlockInvalidPolicy,
            Some("side_effect_and_approval_policy_mismatch"),
        ),
    }
}

fn invocation_guard_route_is_blocked(route: ToolRegistryInvocationGuardRoute) -> bool {
    matches!(
        route,
        ToolRegistryInvocationGuardRoute::BlockUnknownMetadata
            | ToolRegistryInvocationGuardRoute::BlockMissingLedger
            | ToolRegistryInvocationGuardRoute::BlockInvalidPolicy
            | ToolRegistryInvocationGuardRoute::BlockDuplicateId
            | ToolRegistryInvocationGuardRoute::BlockUnknownTool
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(
        id: &str,
        side_effect_level: ToolRegistryInventorySideEffectLevel,
        approval_kind: ToolRegistryInventoryApprovalKind,
    ) -> ToolRegistryInventoryEntry {
        ToolRegistryInventoryEntry {
            id: id.to_string(),
            name: id.to_string(),
            description: None,
            source: ToolRegistryInventorySource::Plugin,
            owner: Some("hepta-system@hepta-local".to_string()),
            has_input_schema: true,
            has_output_schema: true,
            defer_loading: true,
            has_skills: false,
            mcp_server_names: Vec::new(),
            app_connector_ids: Vec::new(),
            side_effect_level,
            approval_kind,
            auth_required: false,
            timeout_ms: Some(30_000),
            ledger_required: true,
        }
    }

    #[test]
    fn inventory_guard_allows_read_only_and_requires_approval_for_mutation() {
        let inventory = ToolRegistryInventory {
            entries: vec![
                entry(
                    "read",
                    ToolRegistryInventorySideEffectLevel::ReadOnly,
                    ToolRegistryInventoryApprovalKind::NotRequired,
                ),
                entry(
                    "write",
                    ToolRegistryInventorySideEffectLevel::LocalMutation,
                    ToolRegistryInventoryApprovalKind::OnUse,
                ),
            ],
        };

        let guard = inventory.plan_invocation_guard();

        assert_eq!(guard.entry_count, 2);
        assert_eq!(guard.allow_read_only_ledger_count, 1);
        assert_eq!(guard.require_approval_ledger_count, 1);
        assert_eq!(guard.blocked_count, 0);
        assert!(guard.invocation_guard_ready);
        assert!(!guard.tool_invoked);
        assert!(!guard.ledger_written);
        assert!(!guard.approval_requested);
        assert!(guard.side_effect_free);
    }

    #[test]
    fn inventory_guard_blocks_unknown_or_missing_policy() {
        let mut unknown = entry(
            "unknown",
            ToolRegistryInventorySideEffectLevel::Unknown,
            ToolRegistryInventoryApprovalKind::Unknown,
        );
        unknown.ledger_required = false;
        let inventory = ToolRegistryInventory {
            entries: vec![unknown],
        };

        let guard = inventory.plan_invocation_guard();

        assert_eq!(guard.block_missing_ledger_count, 1);
        assert_eq!(guard.blocked_count, 1);
        assert!(!guard.invocation_guard_ready);
    }

    #[test]
    fn inventory_guard_decision_is_side_effect_free() {
        let inventory = ToolRegistryInventory {
            entries: vec![entry(
                "write",
                ToolRegistryInventorySideEffectLevel::ExternalMutation,
                ToolRegistryInventoryApprovalKind::Install,
            )],
        };

        let decision = inventory.plan_invocation_guard_for_tool("write");

        assert!(decision.entry_found);
        assert!(!decision.duplicate_id);
        assert_eq!(
            decision.route,
            ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        );
        assert!(decision.approval_required);
        assert!(decision.ledger_required);
        assert!(!decision.blocked);
        assert!(!decision.tool_invoked);
        assert!(!decision.ledger_written);
        assert!(!decision.approval_requested);
        assert!(decision.side_effect_free);
    }
}
