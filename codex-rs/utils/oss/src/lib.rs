//! OSS provider utilities shared between TUI and exec.

use codex_core::config::Config;
use codex_model_provider_info::LMSTUDIO_OSS_PROVIDER_ID;
use codex_model_provider_info::OLLAMA_OSS_PROVIDER_ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OssProviderKind {
    LmStudio,
    Ollama,
}

fn provider_kind(provider_id: &str) -> Result<OssProviderKind, std::io::Error> {
    match provider_id {
        LMSTUDIO_OSS_PROVIDER_ID => Ok(OssProviderKind::LmStudio),
        OLLAMA_OSS_PROVIDER_ID => Ok(OssProviderKind::Ollama),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Unknown OSS provider `{provider_id}`; refusing to skip readiness checks. Expected one of: {LMSTUDIO_OSS_PROVIDER_ID}, {OLLAMA_OSS_PROVIDER_ID}."
            ),
        )),
    }
}

/// Returns the default model for a given OSS provider.
pub fn get_default_model_for_oss_provider(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        LMSTUDIO_OSS_PROVIDER_ID => Some(codex_lmstudio::DEFAULT_OSS_MODEL),
        OLLAMA_OSS_PROVIDER_ID => Some(codex_ollama::DEFAULT_OSS_MODEL),
        _ => None,
    }
}

/// Ensures the specified OSS provider is ready (model present and service reachable).
///
/// Unknown providers fail closed rather than silently skipping setup.
pub async fn ensure_oss_provider_ready(
    provider_id: &str,
    config: &Config,
) -> Result<(), std::io::Error> {
    match provider_kind(provider_id)? {
        OssProviderKind::LmStudio => {
            codex_lmstudio::ensure_oss_ready(config)
                .await
                .map_err(|error| std::io::Error::other(format!("OSS setup failed: {error}")))?;
        }
        OssProviderKind::Ollama => {
            let client = codex_ollama::OllamaClient::try_from_oss_provider(config).await?;
            codex_ollama::ensure_responses_supported(&client).await?;
            codex_ollama::ensure_oss_ready(config, &client)
                .await
                .map_err(|error| std::io::Error::other(format!("OSS setup failed: {error}")))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_model_for_provider_lmstudio() {
        let result = get_default_model_for_oss_provider(LMSTUDIO_OSS_PROVIDER_ID);
        assert_eq!(result, Some(codex_lmstudio::DEFAULT_OSS_MODEL));
    }

    #[test]
    fn test_get_default_model_for_provider_ollama() {
        let result = get_default_model_for_oss_provider(OLLAMA_OSS_PROVIDER_ID);
        assert_eq!(result, Some(codex_ollama::DEFAULT_OSS_MODEL));
    }

    #[test]
    fn test_get_default_model_for_provider_unknown() {
        let result = get_default_model_for_oss_provider("unknown-provider");
        assert_eq!(result, None);
    }

    #[test]
    fn unknown_provider_fails_closed() {
        let error = provider_kind("unknown-provider")
            .expect_err("unknown providers must fail closed before setup");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains("refusing to skip readiness checks"));
    }
}
