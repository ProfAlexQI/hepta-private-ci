use crate::context_manager::manifest;
use crate::session::turn_context::TurnContext;
use codex_protocol::models::ResponseItem;
use codex_protocol::protocol::TurnContextItem;
use codex_protocol::protocol::TurnContextManifestItem;

/// Boundary for turning model-visible context candidates into the durable
/// `TurnContextItem` baseline that replay/resume/fork diff against.
pub(crate) struct ContextController;

pub(crate) struct ContextControllerAssembly<'a> {
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) previous_manifest: Option<&'a TurnContextManifestItem>,
    pub(crate) manifest_options: &'a manifest::TurnContextManifestOptions,
    pub(crate) assembly_policy: &'a manifest::ContextAssemblyPolicy,
}

pub(crate) struct ContextControllerDecision {
    pub(crate) context_items: Vec<ResponseItem>,
    pub(crate) turn_context_item: TurnContextItem,
}

impl ContextController {
    pub(crate) fn assemble_turn_context(
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
