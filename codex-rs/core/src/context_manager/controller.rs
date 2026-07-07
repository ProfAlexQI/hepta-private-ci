use crate::context_manager::history::estimate_response_item_model_visible_bytes;
use crate::context_manager::manifest;
use crate::session::turn_context::TurnContext;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TurnContextItem;
use codex_protocol::protocol::TurnContextManifestItem;
use codex_utils_output_truncation::approx_tokens_from_byte_count_i64;

/// Boundary for turning model-visible context candidates into the durable
/// `TurnContextItem` baseline that replay/resume/fork diff against.
pub(crate) struct ContextController;

struct ContextControllerAssembly<'a> {
    context_items: Vec<ResponseItem>,
    previous_manifest: Option<&'a TurnContextManifestItem>,
    manifest_options: &'a manifest::TurnContextManifestOptions,
    assembly_policy: &'a manifest::ContextAssemblyPolicy,
}

pub(crate) struct ContextControllerPlanInput<'a> {
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) reference_context_item: Option<&'a TurnContextItem>,
    pub(crate) manifest_options: &'a manifest::TurnContextManifestOptions,
    pub(crate) assembly_policy: &'a manifest::ContextAssemblyPolicy,
}

pub(crate) struct ContextControllerPendingContextInput<'a> {
    pub(crate) reference_context_item: Option<TurnContextItem>,
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) manifest_options: &'a manifest::TurnContextManifestOptions,
    pub(crate) existing_history_items: &'a [ResponseItem],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextControllerUpdateMode {
    FullInitialContext,
    SettingsDiff,
}

pub(crate) struct ContextControllerPendingContextItems {
    pub(crate) reference_context_item: Option<TurnContextItem>,
    pub(crate) context_items: Vec<ResponseItem>,
}

struct ContextControllerDecision {
    context_items: Vec<ResponseItem>,
    turn_context_item: TurnContextItem,
}

pub(crate) struct ContextControllerTurnPlan {
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) turn_context_item: TurnContextItem,
    pub(crate) estimated_context_update_tokens: i64,
}

impl ContextController {
    pub(crate) fn context_update_mode(
        reference_context_item: Option<&TurnContextItem>,
    ) -> ContextControllerUpdateMode {
        if reference_context_item.is_some() {
            ContextControllerUpdateMode::SettingsDiff
        } else {
            ContextControllerUpdateMode::FullInitialContext
        }
    }

    pub(crate) fn plan_pending_context_items(
        input: ContextControllerPendingContextInput<'_>,
    ) -> ContextControllerPendingContextItems {
        let mut context_items = input.context_items;
        if let Some(selected_snippets_context_item) =
            manifest::build_recall_selected_snippets_live_context_item(
                input.manifest_options.recall_selected_snippets.as_ref(),
            )
        {
            let selected_snippets_already_in_history = input
                .existing_history_items
                .iter()
                .any(|item| item == &selected_snippets_context_item);
            if !selected_snippets_already_in_history {
                context_items.push(selected_snippets_context_item);
            }
        }

        ContextControllerPendingContextItems {
            reference_context_item: input.reference_context_item,
            context_items,
        }
    }

    pub(crate) fn plan_turn_context(
        turn_context: &TurnContext,
        input: ContextControllerPlanInput<'_>,
    ) -> ContextControllerTurnPlan {
        let previous_manifest = input
            .reference_context_item
            .and_then(|item| item.context_manifest.as_ref());
        let decision = Self::assemble_turn_context(
            turn_context,
            ContextControllerAssembly {
                context_items: input.context_items,
                previous_manifest,
                manifest_options: input.manifest_options,
                assembly_policy: input.assembly_policy,
            },
        );
        let estimated_context_update_tokens = decision
            .context_items
            .iter()
            .map(|item| {
                approx_tokens_from_byte_count_i64(
                    estimate_response_item_model_visible_bytes(item).max(0),
                )
            })
            .sum();

        ContextControllerTurnPlan {
            context_items: decision.context_items,
            turn_context_item: decision.turn_context_item,
            estimated_context_update_tokens,
        }
    }

    fn assemble_turn_context(
        turn_context: &TurnContext,
        assembly: ContextControllerAssembly<'_>,
    ) -> ContextControllerDecision {
        let assembly_result = manifest::assemble_turn_context_with_policy(
            &assembly.context_items,
            assembly.previous_manifest,
            assembly.manifest_options,
            assembly.assembly_policy,
        );
        let mut turn_context_item = turn_context.to_turn_context_item();
        turn_context_item.context_manifest = assembly_result.context_manifest;

        ContextControllerDecision {
            context_items: assembly_result.context_items,
            turn_context_item,
        }
    }
}
