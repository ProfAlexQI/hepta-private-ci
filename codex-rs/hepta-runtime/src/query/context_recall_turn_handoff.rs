use super::RuntimeContextRecallSelectedSnippetEnvelope;
use super::RuntimeContextRecallSlice;
use super::RuntimeContextRecallTurnHandoff;
use super::context_recall_provider_rollup;
use super::context_recall_selected_snippet_envelope;

pub(super) fn build(
    slice: &RuntimeContextRecallSlice,
    query_text: Option<&str>,
    experimental_api_enabled: bool,
) -> RuntimeContextRecallTurnHandoff {
    let provider_rollup = context_recall_provider_rollup::build(slice);
    let runtime_selected_snippets = experimental_api_enabled
        .then(|| context_recall_selected_snippet_envelope::build(slice, query_text));
    let selected_snippets =
        RuntimeContextRecallSelectedSnippetEnvelope::into_core_envelope_for_experimental_client(
            runtime_selected_snippets,
            experimental_api_enabled,
        );

    RuntimeContextRecallTurnHandoff {
        provider_rollup,
        selected_snippets,
    }
}
