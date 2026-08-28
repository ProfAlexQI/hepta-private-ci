mod client;
mod sha256;

pub use client::LMStudioClient;
use codex_core::config::Config;

/// Default OSS model to use when `--oss` is passed without an explicit `-m`.
pub const DEFAULT_OSS_MODEL: &str = "openai/gpt-oss-20b";

/// Prepare the local LM Studio environment when `--oss` is selected.
///
/// Readiness is observation-only: a normal inference startup cannot download a
/// model. The function succeeds only after the configured model is already
/// present and a bounded minimal Responses request has loaded it.
pub async fn ensure_oss_ready(config: &Config) -> std::io::Result<()> {
    let model = config.model.as_deref().unwrap_or(DEFAULT_OSS_MODEL);
    client::validate_model_identifier(model)?;
    let client = LMStudioClient::try_from_provider(config).await?;
    let models = client.fetch_models().await?;
    if !models.iter().any(|candidate| candidate == model) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                concat!(
                    "LMSTUDIO_MODEL_NOT_INSTALLED model={model}; ",
                    "automatic model installation is disabled. ",
                    "Install the model explicitly in LM Studio and retry."
                )
            ),
        ));
    }
    client.load_model(model).await
}
