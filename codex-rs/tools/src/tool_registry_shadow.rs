use std::collections::BTreeMap;

use serde::Serialize;

use crate::PluginToolContributionInventoryPreviewPlan;
use crate::ToolRegistryInventoryApprovalKind;
use crate::ToolRegistryInventoryEntry;
use crate::ToolRegistryInventorySideEffectLevel;
use crate::ToolRegistryInventorySource;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ShadowToolRegistration {
    pub tool_id: String,
    pub display_name: String,
    pub source: ToolRegistryInventorySource,
    pub side_effect_level: ToolRegistryInventorySideEffectLevel,
    pub approval_kind: ToolRegistryInventoryApprovalKind,
    pub ledger_required: bool,
    pub idempotency_key: String,
    pub schema_digest: String,
}

impl ShadowToolRegistration {
    pub fn from_inventory(
        entry: &ToolRegistryInventoryEntry,
        idempotency_key: impl Into<String>,
    ) -> Self {
        let schema_digest = stable_digest(&format!(
            "tool:{}:input_schema={}:output_schema={}:defer={}",
            entry.id, entry.has_input_schema, entry.has_output_schema, entry.defer_loading
        ));

        Self {
            tool_id: entry.id.clone(),
            display_name: entry.name.clone(),
            source: entry.source,
            side_effect_level: entry.side_effect_level,
            approval_kind: entry.approval_kind,
            ledger_required: entry.ledger_required,
            idempotency_key: idempotency_key.into(),
            schema_digest,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShadowRegistrationRoute {
    Registered,
    IdempotentReplay,
    DuplicateToolId,
    RejectedMissingLedger,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ShadowRegistrationOutcome {
    pub tool_id: String,
    pub route: ShadowRegistrationRoute,
    pub entry_count_for_tool: usize,
    pub registry_mutated: bool,
    pub tool_registered_live: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShadowLookupRoute {
    ReadyReadOnly,
    RequiresApprovalLedger,
    BlockedMissingLedger,
    BlockedDuplicateToolId,
    MissingTool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ShadowLookupDecision {
    pub tool_id: String,
    pub route: ShadowLookupRoute,
    pub entry_found: bool,
    pub duplicate_id: bool,
    pub approval_required: bool,
    pub ledger_required: bool,
    pub registry_lookup_executed_live: bool,
    pub tool_invoked: bool,
    pub side_effect_free: bool,
    pub registration: Option<ShadowToolRegistration>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct InMemoryToolRegistryShadow {
    entries: BTreeMap<String, Vec<ShadowToolRegistration>>,
    ledger: TestOnlyToolInvocationLedgerRehearsal,
}

impl InMemoryToolRegistryShadow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, registration: ShadowToolRegistration) -> ShadowRegistrationOutcome {
        let tool_id = registration.tool_id.clone();

        if !registration.ledger_required {
            return ShadowRegistrationOutcome {
                tool_id,
                route: ShadowRegistrationRoute::RejectedMissingLedger,
                entry_count_for_tool: 0,
                registry_mutated: false,
                tool_registered_live: false,
                side_effect_free: true,
            };
        }

        let registrations = self.entries.entry(tool_id.clone()).or_default();
        if let Some(existing) = registrations.iter().find(|existing| {
            existing.idempotency_key == registration.idempotency_key
                && existing.schema_digest == registration.schema_digest
        }) {
            return ShadowRegistrationOutcome {
                tool_id: existing.tool_id.clone(),
                route: ShadowRegistrationRoute::IdempotentReplay,
                entry_count_for_tool: registrations.len(),
                registry_mutated: false,
                tool_registered_live: false,
                side_effect_free: true,
            };
        }

        registrations.push(registration);
        let route = if registrations.len() == 1 {
            ShadowRegistrationRoute::Registered
        } else {
            ShadowRegistrationRoute::DuplicateToolId
        };

        ShadowRegistrationOutcome {
            tool_id,
            route,
            entry_count_for_tool: registrations.len(),
            registry_mutated: true,
            tool_registered_live: false,
            side_effect_free: true,
        }
    }

    pub fn register_all<I>(&mut self, registrations: I) -> Vec<ShadowRegistrationOutcome>
    where
        I: IntoIterator<Item = ShadowToolRegistration>,
    {
        registrations
            .into_iter()
            .map(|registration| self.register(registration))
            .collect()
    }

    pub fn register_plugin_contribution_preview_plan(
        &mut self,
        plan: &PluginToolContributionInventoryPreviewPlan,
    ) -> Vec<ShadowRegistrationOutcome> {
        self.register_all(plugin_contribution_shadow_registrations(plan))
    }

    pub fn lookup(&self, tool_id: impl AsRef<str>) -> ShadowLookupDecision {
        let tool_id = tool_id.as_ref();
        let Some(registrations) = self.entries.get(tool_id) else {
            return ShadowLookupDecision {
                tool_id: tool_id.to_string(),
                route: ShadowLookupRoute::MissingTool,
                entry_found: false,
                duplicate_id: false,
                approval_required: false,
                ledger_required: true,
                registry_lookup_executed_live: false,
                tool_invoked: false,
                side_effect_free: true,
                registration: None,
            };
        };

        if registrations.len() > 1 {
            return ShadowLookupDecision {
                tool_id: tool_id.to_string(),
                route: ShadowLookupRoute::BlockedDuplicateToolId,
                entry_found: true,
                duplicate_id: true,
                approval_required: false,
                ledger_required: true,
                registry_lookup_executed_live: false,
                tool_invoked: false,
                side_effect_free: true,
                registration: None,
            };
        }

        let registration = registrations[0].clone();
        let approval_required = approval_required(&registration);
        let route = if !registration.ledger_required {
            ShadowLookupRoute::BlockedMissingLedger
        } else if approval_required {
            ShadowLookupRoute::RequiresApprovalLedger
        } else {
            ShadowLookupRoute::ReadyReadOnly
        };

        ShadowLookupDecision {
            tool_id: tool_id.to_string(),
            route,
            entry_found: true,
            duplicate_id: false,
            approval_required,
            ledger_required: registration.ledger_required,
            registry_lookup_executed_live: false,
            tool_invoked: false,
            side_effect_free: true,
            registration: Some(registration),
        }
    }

    pub fn rehearse_ledger_write(
        &mut self,
        tool_id: impl AsRef<str>,
        idempotency_key: impl Into<String>,
    ) -> ToolInvocationLedgerRehearsalOutcome {
        let decision = self.lookup(tool_id.as_ref());
        self.ledger.rehearse(decision, idempotency_key.into())
    }

    pub fn receipt_count(&self) -> usize {
        self.ledger.receipts.len()
    }

    pub fn receipts(&self) -> &[ToolInvocationLedgerRehearsalReceipt] {
        &self.ledger.receipts
    }
}

pub fn plugin_contribution_shadow_registrations(
    plan: &PluginToolContributionInventoryPreviewPlan,
) -> Vec<ShadowToolRegistration> {
    plan.candidate_inventory_entries
        .iter()
        .map(|entry| {
            ShadowToolRegistration::from_inventory(
                entry,
                format!("plugin-contribution:{}:{}", plan.plugin_id, entry.id),
            )
        })
        .collect()
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolInvocationLedgerRehearsalRoute {
    RehearsedReadOnlyReceipt,
    RehearsedApprovalRequiredReceipt,
    IdempotentReplay,
    BlockedMissingTool,
    BlockedDuplicateToolId,
    BlockedMissingLedger,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationLedgerRehearsalReceipt {
    pub receipt_id: String,
    pub tool_id: String,
    pub idempotency_key: String,
    pub ledger_entry_digest: String,
    pub approval_required: bool,
    pub in_memory_recorded: bool,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolInvocationLedgerRehearsalOutcome {
    pub tool_id: String,
    pub route: ToolInvocationLedgerRehearsalRoute,
    pub receipt: Option<ToolInvocationLedgerRehearsalReceipt>,
    pub replayed_existing_receipt: bool,
    pub in_memory_receipt_count: usize,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct TestOnlyToolInvocationLedgerRehearsal {
    receipts: Vec<ToolInvocationLedgerRehearsalReceipt>,
}

impl TestOnlyToolInvocationLedgerRehearsal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rehearse(
        &mut self,
        decision: ShadowLookupDecision,
        idempotency_key: String,
    ) -> ToolInvocationLedgerRehearsalOutcome {
        let blocked_route = match decision.route {
            ShadowLookupRoute::MissingTool => {
                Some(ToolInvocationLedgerRehearsalRoute::BlockedMissingTool)
            }
            ShadowLookupRoute::BlockedDuplicateToolId => {
                Some(ToolInvocationLedgerRehearsalRoute::BlockedDuplicateToolId)
            }
            ShadowLookupRoute::BlockedMissingLedger => {
                Some(ToolInvocationLedgerRehearsalRoute::BlockedMissingLedger)
            }
            ShadowLookupRoute::ReadyReadOnly | ShadowLookupRoute::RequiresApprovalLedger => None,
        };

        if let Some(route) = blocked_route {
            return ToolInvocationLedgerRehearsalOutcome {
                tool_id: decision.tool_id,
                route,
                receipt: None,
                replayed_existing_receipt: false,
                in_memory_receipt_count: self.receipts.len(),
                tool_invoked: false,
                ledger_persisted: false,
                receipt_persisted: false,
                side_effect_free: true,
            };
        }

        if let Some(existing) = self
            .receipts
            .iter()
            .find(|receipt| {
                receipt.tool_id == decision.tool_id && receipt.idempotency_key == idempotency_key
            })
            .cloned()
        {
            return ToolInvocationLedgerRehearsalOutcome {
                tool_id: decision.tool_id,
                route: ToolInvocationLedgerRehearsalRoute::IdempotentReplay,
                receipt: Some(existing),
                replayed_existing_receipt: true,
                in_memory_receipt_count: self.receipts.len(),
                tool_invoked: false,
                ledger_persisted: false,
                receipt_persisted: false,
                side_effect_free: true,
            };
        }

        let route = if decision.approval_required {
            ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt
        } else {
            ToolInvocationLedgerRehearsalRoute::RehearsedReadOnlyReceipt
        };
        let receipt = rehearse_receipt(&decision, idempotency_key);
        self.receipts.push(receipt.clone());

        ToolInvocationLedgerRehearsalOutcome {
            tool_id: decision.tool_id,
            route,
            receipt: Some(receipt),
            replayed_existing_receipt: false,
            in_memory_receipt_count: self.receipts.len(),
            tool_invoked: false,
            ledger_persisted: false,
            receipt_persisted: false,
            side_effect_free: true,
        }
    }
}

fn approval_required(registration: &ShadowToolRegistration) -> bool {
    registration.approval_kind != ToolRegistryInventoryApprovalKind::NotRequired
        || registration.side_effect_level != ToolRegistryInventorySideEffectLevel::ReadOnly
}

fn rehearse_receipt(
    decision: &ShadowLookupDecision,
    idempotency_key: String,
) -> ToolInvocationLedgerRehearsalReceipt {
    let ledger_entry_digest = stable_digest(&format!(
        "ledger-rehearsal:{}:{}:{}",
        decision.tool_id, idempotency_key, decision.approval_required
    ));
    ToolInvocationLedgerRehearsalReceipt {
        receipt_id: format!("ledger-rehearsal-receipt:{ledger_entry_digest}"),
        tool_id: decision.tool_id.clone(),
        idempotency_key,
        ledger_entry_digest,
        approval_required: decision.approval_required,
        in_memory_recorded: true,
        tool_invoked: false,
        ledger_persisted: false,
        receipt_persisted: false,
        side_effect_free: true,
    }
}

fn stable_digest(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;

    #[test]
    fn shadow_registry_registers_and_looks_up_read_only_tools_without_live_lookup() {
        let mut registry = InMemoryToolRegistryShadow::new();
        let outcomes = registry.register_all([
            read_only_status_registration("manifest:v1:status"),
            approval_required_connector_registration("manifest:v1:connector"),
        ]);

        assert_eq!(outcomes.len(), 2);
        assert!(outcomes.iter().all(|outcome| {
            outcome.route == ShadowRegistrationRoute::Registered
                && outcome.tool_registered_live == false
                && outcome.side_effect_free
        }));

        let status_lookup = registry.lookup("hepta.system.status");
        assert_eq!(status_lookup.route, ShadowLookupRoute::ReadyReadOnly);
        assert!(status_lookup.entry_found);
        assert!(!status_lookup.duplicate_id);
        assert!(!status_lookup.approval_required);
        assert!(!status_lookup.registry_lookup_executed_live);
        assert!(!status_lookup.tool_invoked);

        let connector_lookup = registry.lookup("hepta.system.connector.preflight");
        assert_eq!(
            connector_lookup.route,
            ShadowLookupRoute::RequiresApprovalLedger
        );
        assert!(connector_lookup.approval_required);
        assert!(!connector_lookup.registry_lookup_executed_live);
        assert!(!connector_lookup.tool_invoked);
    }

    #[test]
    fn shadow_registry_distinguishes_idempotent_replay_from_duplicate_id() {
        let mut registry = InMemoryToolRegistryShadow::new();

        let first = registry.register(read_only_status_registration("manifest:v1:status"));
        let replay = registry.register(read_only_status_registration("manifest:v1:status"));
        let duplicate = registry.register(read_only_status_registration("manifest:v2:status"));

        assert_eq!(first.route, ShadowRegistrationRoute::Registered);
        assert_eq!(replay.route, ShadowRegistrationRoute::IdempotentReplay);
        assert_eq!(replay.entry_count_for_tool, 1);
        assert_eq!(duplicate.route, ShadowRegistrationRoute::DuplicateToolId);
        assert_eq!(duplicate.entry_count_for_tool, 2);

        let lookup = registry.lookup("hepta.system.status");
        assert_eq!(lookup.route, ShadowLookupRoute::BlockedDuplicateToolId);
        assert!(lookup.duplicate_id);
        assert!(!lookup.registry_lookup_executed_live);
        assert!(!lookup.tool_invoked);
    }

    #[test]
    fn ledger_rehearsal_records_only_in_memory_receipts_and_replays_idempotently() {
        let mut registry = InMemoryToolRegistryShadow::new();
        registry.register(read_only_status_registration("manifest:v1:status"));

        let first = registry.rehearse_ledger_write("hepta.system.status", "invoke:v1:status");
        let replay = registry.rehearse_ledger_write("hepta.system.status", "invoke:v1:status");
        let second = registry.rehearse_ledger_write("hepta.system.status", "invoke:v2:status");

        assert_eq!(
            first.route,
            ToolInvocationLedgerRehearsalRoute::RehearsedReadOnlyReceipt
        );
        assert_eq!(first.in_memory_receipt_count, 1);
        assert!(!first.tool_invoked);
        assert!(!first.ledger_persisted);
        assert!(!first.receipt_persisted);
        assert_eq!(
            replay.route,
            ToolInvocationLedgerRehearsalRoute::IdempotentReplay
        );
        assert!(replay.replayed_existing_receipt);
        assert_eq!(replay.in_memory_receipt_count, 1);
        assert_eq!(
            replay.receipt.as_ref().map(|receipt| &receipt.receipt_id),
            first.receipt.as_ref().map(|receipt| &receipt.receipt_id)
        );
        assert_eq!(
            second.route,
            ToolInvocationLedgerRehearsalRoute::RehearsedReadOnlyReceipt
        );
        assert_eq!(second.in_memory_receipt_count, 2);
        assert_eq!(registry.receipt_count(), 2);
        assert!(registry.receipts().iter().all(|receipt| {
            receipt.in_memory_recorded
                && !receipt.tool_invoked
                && !receipt.ledger_persisted
                && !receipt.receipt_persisted
                && receipt.side_effect_free
        }));
    }

    #[test]
    fn ledger_rehearsal_blocks_missing_duplicate_and_missing_ledger_paths() {
        let mut registry = InMemoryToolRegistryShadow::new();

        let missing = registry.rehearse_ledger_write("missing.tool", "invoke:missing");
        assert_eq!(
            missing.route,
            ToolInvocationLedgerRehearsalRoute::BlockedMissingTool
        );
        assert_eq!(missing.in_memory_receipt_count, 0);

        registry.register(read_only_status_registration("manifest:v1:status"));
        registry.register(read_only_status_registration("manifest:v2:status"));
        let duplicate = registry.rehearse_ledger_write("hepta.system.status", "invoke:status");
        assert_eq!(
            duplicate.route,
            ToolInvocationLedgerRehearsalRoute::BlockedDuplicateToolId
        );
        assert_eq!(duplicate.in_memory_receipt_count, 0);

        let mut no_ledger = read_only_status_registration("manifest:v1:no-ledger");
        no_ledger.tool_id = "hepta.system.no-ledger".to_string();
        no_ledger.ledger_required = false;
        let rejected = registry.register(no_ledger);
        assert_eq!(
            rejected.route,
            ShadowRegistrationRoute::RejectedMissingLedger
        );
        let missing_after_reject =
            registry.rehearse_ledger_write("hepta.system.no-ledger", "invoke:no-ledger");
        assert_eq!(
            missing_after_reject.route,
            ToolInvocationLedgerRehearsalRoute::BlockedMissingTool
        );
    }

    #[test]
    fn hepta_system_plugin_contribution_preview_loads_into_shadow_registry() {
        let plan = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let mut registry = InMemoryToolRegistryShadow::new();

        let registrations = plugin_contribution_shadow_registrations(&plan);
        assert_eq!(registrations.len(), 2);
        assert!(registrations.iter().all(|registration| {
            registration.tool_id.starts_with("preview:")
                && registration.ledger_required
                && registration
                    .idempotency_key
                    .starts_with("plugin-contribution:hepta-system@hepta-local:")
        }));

        let outcomes = registry.register_plugin_contribution_preview_plan(&plan);
        assert_eq!(outcomes.len(), 2);
        assert!(outcomes.iter().all(|outcome| {
            outcome.route == ShadowRegistrationRoute::Registered
                && outcome.registry_mutated
                && !outcome.tool_registered_live
                && outcome.side_effect_free
        }));

        for entry in &plan.candidate_inventory_entries {
            let lookup = registry.lookup(&entry.id);
            assert_eq!(lookup.route, ShadowLookupRoute::RequiresApprovalLedger);
            assert!(lookup.entry_found);
            assert!(lookup.approval_required);
            assert!(lookup.ledger_required);
            assert!(!lookup.registry_lookup_executed_live);
            assert!(!lookup.tool_invoked);

            let rehearsal =
                registry.rehearse_ledger_write(&entry.id, format!("invoke:{}", entry.id));
            assert_eq!(
                rehearsal.route,
                ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt
            );
            assert!(rehearsal.receipt.as_ref().is_some_and(|receipt| {
                receipt.approval_required
                    && receipt.in_memory_recorded
                    && !receipt.tool_invoked
                    && !receipt.ledger_persisted
                    && !receipt.receipt_persisted
            }));
        }

        assert_eq!(registry.receipt_count(), 2);
    }

    fn read_only_status_registration(idempotency_key: &str) -> ShadowToolRegistration {
        ShadowToolRegistration {
            tool_id: "hepta.system.status".to_string(),
            display_name: "hepta-system status".to_string(),
            source: ToolRegistryInventorySource::Plugin,
            side_effect_level: ToolRegistryInventorySideEffectLevel::ReadOnly,
            approval_kind: ToolRegistryInventoryApprovalKind::NotRequired,
            ledger_required: true,
            idempotency_key: idempotency_key.to_string(),
            schema_digest: "schema:status:v1".to_string(),
        }
    }

    fn approval_required_connector_registration(idempotency_key: &str) -> ShadowToolRegistration {
        ShadowToolRegistration {
            tool_id: "hepta.system.connector.preflight".to_string(),
            display_name: "hepta-system connector preflight".to_string(),
            source: ToolRegistryInventorySource::Connector,
            side_effect_level: ToolRegistryInventorySideEffectLevel::LocalMutation,
            approval_kind: ToolRegistryInventoryApprovalKind::OnUse,
            ledger_required: true,
            idempotency_key: idempotency_key.to_string(),
            schema_digest: "schema:connector:v1".to_string(),
        }
    }
}
