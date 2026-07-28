use super::*;
use crate::config_toml::ConfigToml;
use crate::types::MemoriesToml;
use pretty_assertions::assert_eq;

fn parse_toml(value: &str) -> TomlValue {
    toml::from_str(value).expect("TOML should parse")
}

#[test]
fn merge_toml_values_normalizes_legacy_key_from_base_layer() {
    let mut base = parse_toml(
        r#"
[memories]
no_memories_if_mcp_or_web_search = false
"#,
    );
    let overlay = parse_toml(
        r#"
[memories]
disable_on_external_context = true
"#,
    );

    merge_toml_values(&mut base, &overlay);

    let expected = parse_toml(
        r#"
[memories]
disable_on_external_context = true
"#,
    );
    assert_eq!(base, expected);

    let config: ConfigToml = base.try_into().expect("merged config should deserialize");
    assert_eq!(
        config.memories,
        Some(MemoriesToml {
            disable_on_external_context: Some(true),
            ..Default::default()
        })
    );
}

#[test]
fn merge_toml_values_normalizes_legacy_key_from_overlay_layer() {
    let mut base = parse_toml(
        r#"
[memories]
disable_on_external_context = false
"#,
    );
    let overlay = parse_toml(
        r#"
[memories]
no_memories_if_mcp_or_web_search = true
"#,
    );

    merge_toml_values(&mut base, &overlay);

    let expected = parse_toml(
        r#"
[memories]
disable_on_external_context = true
"#,
    );
    assert_eq!(base, expected);

    let config: ConfigToml = base.try_into().expect("merged config should deserialize");
    assert_eq!(
        config.memories,
        Some(MemoriesToml {
            disable_on_external_context: Some(true),
            ..Default::default()
        })
    );
}

#[test]
fn merge_toml_values_prefers_canonical_key_when_one_layer_has_both_names() {
    let mut base = TomlValue::Table(toml::map::Map::new());
    let overlay = parse_toml(
        r#"
[memories]
disable_on_external_context = true
no_memories_if_mcp_or_web_search = false
"#,
    );

    merge_toml_values(&mut base, &overlay);

    let expected = parse_toml(
        r#"
[memories]
disable_on_external_context = true
"#,
    );
    assert_eq!(base, expected);
}

#[test]
fn merge_multi_agent_v2_preserves_toggle_and_nested_configuration() {
    for feature_path in ["features", "profiles.work.features"] {
        let mut boolean_base = parse_toml(&format!("[{feature_path}]\nmulti_agent_v2 = true\n"));
        let table_overlay = parse_toml(&format!(
            "[{feature_path}.multi_agent_v2]\nsubagent_usage_hint_text = \"Delegate carefully.\"\n"
        ));
        merge_toml_values(&mut boolean_base, &table_overlay);
        let enabled_table = parse_toml(&format!(
            "[{feature_path}.multi_agent_v2]\nenabled = true\nsubagent_usage_hint_text = \"Delegate carefully.\"\n"
        ));
        assert_eq!(boolean_base, enabled_table);

        let mut table_base = enabled_table;
        let boolean_overlay = parse_toml(&format!("[{feature_path}]\nmulti_agent_v2 = false\n"));
        merge_toml_values(&mut table_base, &boolean_overlay);
        assert_eq!(
            table_base,
            parse_toml(&format!(
                "[{feature_path}.multi_agent_v2]\nenabled = false\nsubagent_usage_hint_text = \"Delegate carefully.\"\n"
            ))
        );
    }
}

#[test]
fn multi_agent_v2_cli_overrides_preserve_toggle_and_nested_configuration() {
    for feature_path in ["features", "profiles.work.features"] {
        let enabled = (
            format!("{feature_path}.multi_agent_v2"),
            TomlValue::Boolean(true),
        );
        let hint = (
            format!("{feature_path}.multi_agent_v2.subagent_usage_hint_text"),
            TomlValue::String("Delegate carefully.".to_string()),
        );
        let expected = parse_toml(&format!(
            "[{feature_path}.multi_agent_v2]\nenabled = true\nsubagent_usage_hint_text = \"Delegate carefully.\"\n"
        ));

        assert_eq!(
            crate::build_cli_overrides_layer(&[enabled.clone(), hint.clone()]),
            expected
        );
        assert_eq!(crate::build_cli_overrides_layer(&[hint, enabled]), expected);
    }
}
