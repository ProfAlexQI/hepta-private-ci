use crate::DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS;
use crate::DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS;
use crate::DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS;
use crate::DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS;
use crate::DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS;
use crate::DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
use crate::MAX_TELEGRAM_MODEL_TIMEOUT_MS;
use crate::MAX_TELEGRAM_READ_MAX_ATTEMPTS;
use crate::MAX_TELEGRAM_READ_RETRY_BACKOFF_MS;
use crate::MAX_TELEGRAM_SEND_MAX_ATTEMPTS;
use crate::MAX_TELEGRAM_SEND_MIN_INTERVAL_MS;
use crate::MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS;
use crate::MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS;
use crate::MIN_TELEGRAM_MODEL_TIMEOUT_MS;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelTelegramProductionGuardStatus {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: u64,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: u64,
    pub retry_transient_read_errors: bool,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: u64,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: u64,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: u64,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: u64,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: u64,
    pub retry_transient_send_errors: bool,
    pub rate_limit_scope: &'static str,
    pub raw_token_exposed: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct HeptaKernelTelegramProductionGuardStatusInput {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: u64,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: u64,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: u64,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: u64,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: u64,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: u64,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct HeptaKernelTelegramProductionGuardPolicyInput {
    pub read_max_attempts_env: &'static str,
    pub read_max_attempts: Option<u64>,
    pub read_retry_backoff_env: &'static str,
    pub read_retry_backoff_ms: Option<u64>,
    pub typing_keepalive_env: &'static str,
    pub typing_keepalive_enabled: bool,
    pub typing_keepalive_interval_ms: Option<u64>,
    pub model_timeout_env: &'static str,
    pub model_timeout_ms: Option<u64>,
    pub model_failure_fallback_env: &'static str,
    pub model_failure_fallback_enabled: bool,
    pub send_min_interval_env: &'static str,
    pub send_min_interval_ms: Option<u64>,
    pub send_max_attempts_env: &'static str,
    pub send_max_attempts: Option<u64>,
    pub send_retry_backoff_env: &'static str,
    pub send_retry_backoff_ms: Option<u64>,
}

pub fn hepta_kernel_telegram_model_timeout_ms(value_ms: Option<u64>) -> u64 {
    value_ms
        .map(|value| value.clamp(MIN_TELEGRAM_MODEL_TIMEOUT_MS, MAX_TELEGRAM_MODEL_TIMEOUT_MS))
        .unwrap_or(DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS)
}

pub fn hepta_kernel_telegram_model_timeout(value_ms: Option<u64>) -> Duration {
    Duration::from_millis(hepta_kernel_telegram_model_timeout_ms(value_ms))
}

pub fn hepta_kernel_telegram_typing_keepalive_interval_policy(value_ms: Option<u64>) -> Duration {
    Duration::from_millis(
        value_ms
            .map(|ms| ms.clamp(1_000, MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS))
            .unwrap_or(DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS),
    )
}

pub fn hepta_kernel_telegram_read_max_attempts_policy(value: Option<u64>) -> u64 {
    value
        .map(|attempts| attempts.clamp(1, MAX_TELEGRAM_READ_MAX_ATTEMPTS))
        .unwrap_or(DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS)
}

pub fn hepta_kernel_telegram_read_retry_backoff_policy(value_ms: Option<u64>) -> Duration {
    Duration::from_millis(
        value_ms
            .map(|ms| ms.min(MAX_TELEGRAM_READ_RETRY_BACKOFF_MS))
            .unwrap_or(DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS),
    )
}

pub fn hepta_kernel_telegram_send_min_interval_policy(value_ms: Option<u64>) -> Duration {
    Duration::from_millis(
        value_ms
            .map(|ms| ms.min(MAX_TELEGRAM_SEND_MIN_INTERVAL_MS))
            .unwrap_or(0),
    )
}

pub fn hepta_kernel_telegram_send_max_attempts_policy(value: Option<u64>) -> u64 {
    value
        .map(|attempts| attempts.clamp(1, MAX_TELEGRAM_SEND_MAX_ATTEMPTS))
        .unwrap_or(DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS)
}

pub fn hepta_kernel_telegram_send_retry_backoff_policy(value_ms: Option<u64>) -> Duration {
    Duration::from_millis(
        value_ms
            .map(|ms| ms.min(MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS))
            .unwrap_or(DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS),
    )
}

pub fn build_hepta_kernel_telegram_production_guard_status(
    input: HeptaKernelTelegramProductionGuardStatusInput,
) -> HeptaKernelTelegramProductionGuardStatus {
    HeptaKernelTelegramProductionGuardStatus {
        read_max_attempts_env: input.read_max_attempts_env,
        read_max_attempts: input.read_max_attempts,
        read_retry_backoff_env: input.read_retry_backoff_env,
        read_retry_backoff_ms: input.read_retry_backoff_ms,
        retry_transient_read_errors: true,
        typing_keepalive_env: input.typing_keepalive_env,
        typing_keepalive_enabled: input.typing_keepalive_enabled,
        typing_keepalive_interval_ms: input.typing_keepalive_interval_ms,
        model_timeout_env: input.model_timeout_env,
        model_timeout_ms: input.model_timeout_ms,
        model_failure_fallback_env: input.model_failure_fallback_env,
        model_failure_fallback_enabled: input.model_failure_fallback_enabled,
        send_min_interval_env: input.send_min_interval_env,
        send_min_interval_ms: input.send_min_interval_ms,
        send_max_attempts_env: input.send_max_attempts_env,
        send_max_attempts: input.send_max_attempts,
        send_retry_backoff_env: input.send_retry_backoff_env,
        send_retry_backoff_ms: input.send_retry_backoff_ms,
        retry_transient_send_errors: true,
        rate_limit_scope: "in-process per chat id; reset on gateway restart",
        raw_token_exposed: false,
    }
}

pub fn build_hepta_kernel_telegram_production_guard_status_from_policy(
    input: HeptaKernelTelegramProductionGuardPolicyInput,
) -> HeptaKernelTelegramProductionGuardStatus {
    build_hepta_kernel_telegram_production_guard_status(
        HeptaKernelTelegramProductionGuardStatusInput {
            read_max_attempts_env: input.read_max_attempts_env,
            read_max_attempts: hepta_kernel_telegram_read_max_attempts_policy(
                input.read_max_attempts,
            ),
            read_retry_backoff_env: input.read_retry_backoff_env,
            read_retry_backoff_ms: duration_millis_u64(
                hepta_kernel_telegram_read_retry_backoff_policy(input.read_retry_backoff_ms),
            ),
            typing_keepalive_env: input.typing_keepalive_env,
            typing_keepalive_enabled: input.typing_keepalive_enabled,
            typing_keepalive_interval_ms: duration_millis_u64(
                hepta_kernel_telegram_typing_keepalive_interval_policy(
                    input.typing_keepalive_interval_ms,
                ),
            ),
            model_timeout_env: input.model_timeout_env,
            model_timeout_ms: hepta_kernel_telegram_model_timeout_ms(input.model_timeout_ms),
            model_failure_fallback_env: input.model_failure_fallback_env,
            model_failure_fallback_enabled: input.model_failure_fallback_enabled,
            send_min_interval_env: input.send_min_interval_env,
            send_min_interval_ms: duration_millis_u64(
                hepta_kernel_telegram_send_min_interval_policy(input.send_min_interval_ms),
            ),
            send_max_attempts_env: input.send_max_attempts_env,
            send_max_attempts: hepta_kernel_telegram_send_max_attempts_policy(
                input.send_max_attempts,
            ),
            send_retry_backoff_env: input.send_retry_backoff_env,
            send_retry_backoff_ms: duration_millis_u64(
                hepta_kernel_telegram_send_retry_backoff_policy(input.send_retry_backoff_ms),
            ),
        },
    )
}

fn duration_millis_u64(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

#[cfg(test)]
#[path = "telegram_production_guard_tests.rs"]
mod tests;
