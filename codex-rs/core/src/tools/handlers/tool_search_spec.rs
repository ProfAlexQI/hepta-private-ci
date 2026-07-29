use codex_tools::JsonSchema;
use codex_tools::TOOL_SEARCH_TOOL_NAME;
use codex_tools::ToolSearchSourceInfo;
use codex_tools::ToolSpec;
use codex_utils_string::take_bytes_at_char_boundary;
use std::collections::BTreeMap;

const MAX_TOOL_SEARCH_SOURCE_DESCRIPTION_BYTES: usize = 4 * 1024;

pub(crate) fn create_tool_search_tool(
    searchable_sources: &[ToolSearchSourceInfo],
    default_limit: usize,
) -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "query".to_string(),
            JsonSchema::string(Some("Search query for deferred tools.".to_string())),
        ),
        (
            "limit".to_string(),
            JsonSchema::number(Some(format!(
                "Maximum number of tools to return (defaults to {default_limit})."
            ))),
        ),
    ]);

    let mut source_descriptions = BTreeMap::new();
    for source in searchable_sources {
        source_descriptions
            .entry(source.name.clone())
            .and_modify(|existing: &mut Option<String>| {
                if existing.is_none() {
                    *existing = source.description.clone();
                }
            })
            .or_insert(source.description.clone());
    }

    let source_descriptions = if source_descriptions.is_empty() {
        "None currently enabled.".to_string()
    } else {
        let reserved_name_bytes = source_descriptions.keys().fold(
            source_descriptions.len().saturating_sub(1),
            |reserved, name| reserved.saturating_add(2).saturating_add(name.len()),
        );
        let mut description_budget =
            MAX_TOOL_SEARCH_SOURCE_DESCRIPTION_BYTES.saturating_sub(reserved_name_bytes);
        let mut rendered = String::new();
        for (name, description) in source_descriptions {
            let required = usize::from(!rendered.is_empty())
                .saturating_add(2)
                .saturating_add(name.len());
            if required > MAX_TOOL_SEARCH_SOURCE_DESCRIPTION_BYTES.saturating_sub(rendered.len()) {
                continue;
            }
            if !rendered.is_empty() {
                rendered.push('\n');
            }
            rendered.push_str("- ");
            rendered.push_str(&name);
            if let Some(description) = description
                && description_budget >= 2
            {
                rendered.push_str(": ");
                description_budget -= 2;
                let bounded = take_bytes_at_char_boundary(&description, description_budget);
                rendered.push_str(bounded);
                description_budget = description_budget.saturating_sub(bounded.len());
            }
        }
        rendered
    };

    let description = format!(
        "# Tool discovery\n\nSearches over deferred tool metadata with BM25 and exposes matching tools for the next model call.\n\nYou have access to tools from the following sources:\n{source_descriptions}\nSome of the tools may not have been provided to you upfront, and you should use this tool (`{TOOL_SEARCH_TOOL_NAME}`) to search for the required tools. For MCP tool discovery, always use `{TOOL_SEARCH_TOOL_NAME}` instead of `list_mcp_resources` or `list_mcp_resource_templates`."
    );

    ToolSpec::ToolSearch {
        execution: "client".to_string(),
        description,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["query".to_string()]),
            Some(false.into()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_tools::JsonSchema;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn create_tool_search_tool_deduplicates_and_renders_enabled_sources() {
        assert_eq!(
            create_tool_search_tool(
                &[
                    ToolSearchSourceInfo {
                        name: "Google Drive".to_string(),
                        description: Some(
                            "Use Google Drive as the single entrypoint for Drive, Docs, Sheets, and Slides work."
                                .to_string(),
                        ),
                    },
                    ToolSearchSourceInfo {
                        name: "Google Drive".to_string(),
                        description: None,
                    },
                    ToolSearchSourceInfo {
                        name: "docs".to_string(),
                        description: None,
                    },
                ],
                /*default_limit*/ 8,
            ),
            ToolSpec::ToolSearch {
                execution: "client".to_string(),
                description: "# Tool discovery\n\nSearches over deferred tool metadata with BM25 and exposes matching tools for the next model call.\n\nYou have access to tools from the following sources:\n- Google Drive: Use Google Drive as the single entrypoint for Drive, Docs, Sheets, and Slides work.\n- docs\nSome of the tools may not have been provided to you upfront, and you should use this tool (`tool_search`) to search for the required tools. For MCP tool discovery, always use `tool_search` instead of `list_mcp_resources` or `list_mcp_resource_templates`.".to_string(),
                parameters: JsonSchema::object(BTreeMap::from([
                        (
                            "limit".to_string(),
                            JsonSchema::number(Some(
                                    "Maximum number of tools to return (defaults to 8)."
                                        .to_string(),
                                ),),
                        ),
                        (
                            "query".to_string(),
                            JsonSchema::string(Some("Search query for deferred tools.".to_string()),),
                        ),
                    ]), Some(vec!["query".to_string()]), Some(false.into())),
            }
        );
    }

    #[test]
    fn create_tool_search_tool_bounds_aggregate_source_descriptions() {
        let sources = (0..8)
            .map(|index| ToolSearchSourceInfo {
                name: format!("source-{index:02}"),
                description: Some("🦀".repeat(300)),
            })
            .collect::<Vec<_>>();
        let ToolSpec::ToolSearch { description, .. } =
            create_tool_search_tool(&sources, /*default_limit*/ 8)
        else {
            panic!("expected tool search spec");
        };
        let (_, source_section) = description
            .split_once("You have access to tools from the following sources:\n")
            .expect("source introduction");
        let (source_descriptions, _) = source_section
            .split_once("\nSome of the tools may not have been provided to you upfront")
            .expect("discovery instructions");
        assert!(source_descriptions.len() <= MAX_TOOL_SEARCH_SOURCE_DESCRIPTION_BYTES);
        assert!(source_descriptions.starts_with("- source-00: 🦀"));
        assert_eq!(source_descriptions.lines().count(), sources.len());
    }
}
