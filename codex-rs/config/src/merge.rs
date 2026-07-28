use crate::key_aliases::normalize_key_aliases;
use crate::key_aliases::normalized_with_key_aliases;
use toml::Value as TomlValue;

/// Merge config `overlay` into `base`, giving `overlay` precedence.
pub fn merge_toml_values(base: &mut TomlValue, overlay: &TomlValue) {
    merge_toml_values_at_path(base, overlay, &mut Vec::new());
}

pub(crate) fn is_multi_agent_v2_feature_path<S: AsRef<str>>(path: &[S]) -> bool {
    match path {
        [features, feature] => {
            features.as_ref() == "features" && feature.as_ref() == "multi_agent_v2"
        }
        [profiles, _, features, feature] => {
            profiles.as_ref() == "profiles"
                && features.as_ref() == "features"
                && feature.as_ref() == "multi_agent_v2"
        }
        _ => false,
    }
}

fn merge_toml_values_at_path(base: &mut TomlValue, overlay: &TomlValue, path: &mut Vec<String>) {
    if is_multi_agent_v2_feature_path(path) {
        if let TomlValue::Boolean(enabled) = base
            && overlay.is_table()
        {
            *base = TomlValue::Table(toml::map::Map::from_iter([(
                "enabled".to_string(),
                TomlValue::Boolean(*enabled),
            )]));
        } else if let TomlValue::Table(table) = base
            && let TomlValue::Boolean(enabled) = overlay
        {
            table.insert("enabled".to_string(), TomlValue::Boolean(*enabled));
            return;
        }
    }

    if let TomlValue::Table(overlay_table) = overlay
        && let TomlValue::Table(base_table) = base
    {
        normalize_key_aliases(path, base_table);
        let mut overlay_table = overlay_table.clone();
        normalize_key_aliases(path, &mut overlay_table);

        for (key, value) in overlay_table {
            path.push(key.clone());
            if let Some(existing) = base_table.get_mut(&key) {
                merge_toml_values_at_path(existing, &value, path);
            } else {
                base_table.insert(key, normalized_with_key_aliases(&value, path));
            }
            path.pop();
        }
    } else {
        *base = normalized_with_key_aliases(overlay, path);
    }
}

#[cfg(test)]
#[path = "merge_tests.rs"]
mod tests;
