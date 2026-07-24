use super::PluginManifest;
use super::RawPluginManifest;
use super::RawPluginManifestInterface;
use super::apply_codex_agent_plugin_extension;
use super::compatibility_json_error;
use super::resolve_raw_plugin_manifest;
use codex_utils_plugins::AGENT_PLUGIN_SCHEMA_PREFIX;
use codex_utils_plugins::AGENT_PLUGIN_SCHEMA_URI;
use codex_utils_plugins::SUPPORTED_AGENT_PLUGIN_SCHEMA_URIS;
use codex_utils_plugins::SkillDiscoveryMode;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::path::Path;

const CODEX_AGENT_PLUGIN_EXTENSION_NAMESPACE: &str = "com.openai";
const AGENT_PLUGIN_FIELDS: &[&str] = &[
    "$schema",
    "name",
    "version",
    "description",
    "author",
    "homepage",
    "repository",
    "license",
    "keywords",
    "extensions",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawAgentPluginManifest {
    #[serde(rename = "$schema")]
    schema: String,
    name: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    author: Option<RawAgentPluginAuthor>,
    #[serde(default)]
    homepage: Option<String>,
    #[serde(default, rename = "repository")]
    _repository: Option<String>,
    #[serde(default, rename = "license")]
    _license: Option<String>,
    #[serde(default)]
    keywords: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAgentPluginAuthor {
    #[serde(default)]
    name: Option<String>,
    #[serde(default, rename = "email")]
    _email: Option<String>,
    #[serde(default, rename = "url")]
    _url: Option<String>,
}

pub(super) fn parse_agent_plugin_manifest(
    plugin_root: &Path,
    manifest_path: &Path,
    contents: &str,
    overlay: Option<(&Path, &str)>,
) -> Result<PluginManifest, serde_json::Error> {
    let value = serde_json::from_str::<JsonValue>(contents)?;
    let JsonValue::Object(mut object) = value else {
        return Err(compatibility_json_error(
            "Agent Plugins root `plugin.json` must contain a JSON object",
        ));
    };
    for field in object.keys() {
        if !AGENT_PLUGIN_FIELDS.contains(&field.as_str()) {
            tracing::warn!(
                path = %manifest_path.display(),
                field,
                "ignoring unknown Agent Plugins manifest field"
            );
        }
    }
    object.retain(|field, _| AGENT_PLUGIN_FIELDS.contains(&field.as_str()));
    if object
        .get("extensions")
        .is_some_and(|extensions| !extensions.is_object())
    {
        tracing::warn!(
            path = %manifest_path.display(),
            "ignoring non-object Agent Plugins `extensions` field"
        );
        object.remove("extensions");
    }
    let codex_extension = object
        .get("extensions")
        .and_then(JsonValue::as_object)
        .and_then(|extensions| extensions.get(CODEX_AGENT_PLUGIN_EXTENSION_NAMESPACE))
        .and_then(|extension| {
            if extension.is_object() {
                Some(serde_json::to_string(extension))
            } else {
                tracing::warn!(
                    path = %manifest_path.display(),
                    namespace = CODEX_AGENT_PLUGIN_EXTENSION_NAMESPACE,
                    "ignoring non-object Agent Plugins extension"
                );
                None
            }
        })
        .transpose()?;
    for field in [
        "version",
        "description",
        "author",
        "homepage",
        "repository",
        "license",
    ] {
        if object.get(field).is_some_and(JsonValue::is_null) {
            return Err(compatibility_json_error(format!(
                "Agent Plugins `{field}` must use its declared type when present"
            )));
        }
    }
    if let Some(author) = object.get("author").and_then(JsonValue::as_object) {
        for field in ["name", "email", "url"] {
            if author.get(field).is_some_and(JsonValue::is_null) {
                return Err(compatibility_json_error(format!(
                    "Agent Plugins `author.{field}` must be a string when present"
                )));
            }
        }
    }

    let raw = serde_json::from_value::<RawAgentPluginManifest>(JsonValue::Object(object))?;
    if !SUPPORTED_AGENT_PLUGIN_SCHEMA_URIS.contains(&raw.schema.as_str()) {
        let message = if raw.schema.starts_with(AGENT_PLUGIN_SCHEMA_PREFIX) {
            format!(
                "unsupported Agent Plugins schema `{}`; supported schemas: `{AGENT_PLUGIN_SCHEMA_URI}`",
                raw.schema
            )
        } else {
            format!(
                "root `plugin.json` is not an Agent Plugins manifest; expected `$schema` `{AGENT_PLUGIN_SCHEMA_URI}`"
            )
        };
        return Err(compatibility_json_error(message));
    }
    if !is_valid_agent_plugin_name(&raw.name) {
        return Err(compatibility_json_error(format!(
            "invalid Agent Plugins name `{}`; use lowercase letters, numbers, dots, or hyphens",
            raw.name
        )));
    }

    let version = raw.version.and_then(non_empty_trimmed);
    let description = raw.description.and_then(non_empty_trimmed);
    let developer_name = raw
        .author
        .and_then(|author| author.name)
        .and_then(non_empty_trimmed);
    let homepage = raw.homepage.and_then(non_empty_trimmed);
    let name = raw.name;
    let mut resolved = resolve_raw_plugin_manifest(
        plugin_root,
        RawPluginManifest {
            name: name.clone(),
            version,
            description: description.clone(),
            keywords: raw.keywords,
            skills: Some("./skills".to_string()),
            mcp_servers: Some("./mcp.json".to_string()),
            interface: Some(RawPluginManifestInterface {
                display_name: Some(name),
                short_description: description.clone(),
                long_description: description,
                developer_name,
                category: Some("Other".to_string()),
                website_url: homepage,
                ..RawPluginManifestInterface::default()
            }),
            ..RawPluginManifest::default()
        },
        SkillDiscoveryMode::DirectChildren,
        manifest_path,
        "hooks",
    );

    if let Some(extension_contents) = codex_extension.as_deref() {
        apply_codex_agent_plugin_extension(
            &mut resolved,
            plugin_root,
            manifest_path,
            "extensions.com.openai.hooks",
            extension_contents,
        )?;
    } else if let Some((overlay_path, overlay_contents)) = overlay {
        apply_codex_agent_plugin_extension(
            &mut resolved,
            plugin_root,
            overlay_path,
            "hooks",
            overlay_contents,
        )?;
    }
    Ok(resolved)
}

fn non_empty_trimmed(value: String) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn is_valid_agent_plugin_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && !name.contains("--")
        && !name.contains("..")
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || b".-".contains(&byte))
        && name
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && name
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
}
