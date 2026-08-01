use crate::hepta_action_queue::{
    HeptaActionQueueStage, inspect_action_outbox, sample_action_queue_items,
    selected_action_detail, summarize_action_queue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HeptaFixtureOperationWorkbench {
    pub(super) item_count: usize,
    pub(super) local_preview_count: usize,
    pub(super) awaiting_confirmation_count: usize,
    pub(super) policy_blocked_count: usize,
    pub(super) all_external_mutation_disabled: bool,
    pub(super) composer_title: String,
    pub(super) composer_body: String,
    pub(super) approval_title: String,
    pub(super) approval_body: String,
    pub(super) outbox_title: String,
    pub(super) outbox_body: String,
}

impl HeptaFixtureOperationWorkbench {
    pub(super) fn composer_display_body(&self) -> String {
        self.composer_body
            .replace(" · mutation=false", " · review preview")
    }

    pub(super) fn approval_display_body(&self) -> String {
        self.approval_body
            .replace(" · mutation=false", " · approval required")
            .replace("mutation=false", "approval required")
    }

    pub(super) fn outbox_display_body(&self) -> String {
        format!(
            "{} local preview · {} need confirmation · {} blocked",
            self.local_preview_count, self.awaiting_confirmation_count, self.policy_blocked_count
        )
    }
}

pub(super) fn summarize_operation_workbench() -> HeptaFixtureOperationWorkbench {
    let items = sample_action_queue_items();
    let action_summary = summarize_action_queue(&items);
    let selected = selected_action_detail(&items);
    let inspections = inspect_action_outbox(&items);
    let approval = items
        .iter()
        .find(|item| item.stage == HeptaActionQueueStage::PolicyBlocked);
    let approval_inspection = approval.and_then(|approval| {
        inspections
            .iter()
            .find(|inspection| inspection.item_id == approval.id)
    });
    let all_external_mutation_disabled = items.iter().all(|item| !item.external_mutation_enabled);

    let (composer_title, composer_body) = selected
        .map(|detail| {
            (
                format!(
                    "{} · {}",
                    readable_token(detail.stage),
                    readable_token(&detail.mutation_class)
                ),
                format!(
                    "{} · {} · mutation=false",
                    detail.title, detail.target_display
                ),
            )
        })
        .unwrap_or_else(|| {
            (
                "No selected dry-run".to_string(),
                "The local action queue has no composer preview item.".to_string(),
            )
        });

    let (approval_title, approval_body) = match (approval, approval_inspection) {
        (Some(approval), Some(inspection)) => (
            approval.title.clone(),
            format!(
                "{} · request={} · preview required · mutation={}",
                readable_token(inspection.policy_decision_label),
                short_hash(&inspection.exact_payload_hash),
                inspection.external_mutation_enabled,
            ),
        ),
        _ => (
            "No approval request".to_string(),
            "No approval review item is queued in the local fixture.".to_string(),
        ),
    };

    HeptaFixtureOperationWorkbench {
        item_count: items.len(),
        local_preview_count: action_summary.local_preview,
        awaiting_confirmation_count: action_summary.awaiting_confirmation,
        policy_blocked_count: action_summary.policy_blocked,
        all_external_mutation_disabled,
        composer_title,
        composer_body,
        approval_title,
        approval_body,
        outbox_title: format!("{} staged local actions", items.len()),
        outbox_body: format!(
            "local={} · confirm={} · blocked={} · mutation={}",
            action_summary.local_preview,
            action_summary.awaiting_confirmation,
            action_summary.policy_blocked,
            !all_external_mutation_disabled
        ),
    }
}

fn short_hash(hash: &str) -> String {
    hash.chars().take(8).collect()
}

fn readable_token(value: &str) -> String {
    value.replace('_', " ")
}
