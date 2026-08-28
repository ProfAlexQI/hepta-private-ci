mod client;

pub use client::LMStudioClient;
use codex_core::config::Config;

/// Default OSS model to use when `--oss` is passed without an explicit `-m`.
pub const DEFAULT_OSS_MODEL: &str = "openai/gpt-oss-20b";

/// Prepare the local OSS environment when `--oss` is selected.
///
/// Readiness is fail-closed: the function returns only after the configured
/// model is present and a minimal Responses request has successfully loaded it.
pub async fn ensure_oss_ready(config: &Config) -> std::io::Result<()> {
    let model = match config.model.as_ref() {
        Some(model) => model,
        None => DEFAULT_OSS_MODEL,
    };

    let lmstudio_client = LMStudioClient::try_from_provider(config).await?;
    let models = lmstudio_client.fetch_models().await?;
    if !models.iter().any(|candidate| candidate == model) {
        lmstudio_client.download_model(model).await?;
    }

    // Do not detach this operation. A successful return is the readiness fence
    // consumed by callers, so load errors must propagate to them.
    lmstudio_client.load_model(model).await
}
