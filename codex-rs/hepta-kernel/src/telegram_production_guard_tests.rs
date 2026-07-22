use super::*;

#[test]
fn kernel_model_timeout_policy_clamps_and_defaults() {
    assert_eq!(
        hepta_kernel_telegram_model_timeout(None),
        Duration::from_millis(DEFAULT_TELEGRAM_MODEL_TIMEOUT_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_model_timeout(Some(1)),
        Duration::from_millis(MIN_TELEGRAM_MODEL_TIMEOUT_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_model_timeout(Some(999_999_999)),
        Duration::from_millis(MAX_TELEGRAM_MODEL_TIMEOUT_MS)
    );
    assert_eq!(hepta_kernel_telegram_model_timeout_ms(Some(2_500)), 2_500);
}

#[test]
fn kernel_production_guard_policies_are_bounded() {
    assert_eq!(
        hepta_kernel_telegram_typing_keepalive_interval_policy(None),
        Duration::from_millis(DEFAULT_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_typing_keepalive_interval_policy(Some(1)),
        Duration::from_millis(1_000)
    );
    assert_eq!(
        hepta_kernel_telegram_typing_keepalive_interval_policy(Some(999_999)),
        Duration::from_millis(MAX_TELEGRAM_TYPING_KEEPALIVE_INTERVAL_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_read_max_attempts_policy(None),
        DEFAULT_TELEGRAM_READ_MAX_ATTEMPTS
    );
    assert_eq!(hepta_kernel_telegram_read_max_attempts_policy(Some(0)), 1);
    assert_eq!(
        hepta_kernel_telegram_read_max_attempts_policy(Some(999)),
        MAX_TELEGRAM_READ_MAX_ATTEMPTS
    );
    assert_eq!(
        hepta_kernel_telegram_read_retry_backoff_policy(None),
        Duration::from_millis(DEFAULT_TELEGRAM_READ_RETRY_BACKOFF_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_read_retry_backoff_policy(Some(999_999)),
        Duration::from_millis(MAX_TELEGRAM_READ_RETRY_BACKOFF_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_send_min_interval_policy(None),
        Duration::ZERO
    );
    assert_eq!(
        hepta_kernel_telegram_send_min_interval_policy(Some(999_999)),
        Duration::from_millis(MAX_TELEGRAM_SEND_MIN_INTERVAL_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_send_max_attempts_policy(None),
        DEFAULT_TELEGRAM_SEND_MAX_ATTEMPTS
    );
    assert_eq!(hepta_kernel_telegram_send_max_attempts_policy(Some(0)), 1);
    assert_eq!(
        hepta_kernel_telegram_send_max_attempts_policy(Some(999)),
        MAX_TELEGRAM_SEND_MAX_ATTEMPTS
    );
    assert_eq!(
        hepta_kernel_telegram_send_retry_backoff_policy(None),
        Duration::from_millis(DEFAULT_TELEGRAM_SEND_RETRY_BACKOFF_MS)
    );
    assert_eq!(
        hepta_kernel_telegram_send_retry_backoff_policy(Some(999_999)),
        Duration::from_millis(MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS)
    );
}

#[test]
fn kernel_telegram_production_guard_status_is_bounded() {
    let direct = build_hepta_kernel_telegram_production_guard_status(
        HeptaKernelTelegramProductionGuardStatusInput {
            read_max_attempts_env: "READ_MAX",
            read_max_attempts: 2,
            read_retry_backoff_env: "READ_BACKOFF",
            read_retry_backoff_ms: 500,
            typing_keepalive_env: "TYPING",
            typing_keepalive_enabled: true,
            typing_keepalive_interval_ms: 4_000,
            model_timeout_env: "MODEL_TIMEOUT",
            model_timeout_ms: 120_000,
            model_failure_fallback_env: "MODEL_FALLBACK",
            model_failure_fallback_enabled: true,
            send_min_interval_env: "SEND_MIN",
            send_min_interval_ms: 1_000,
            send_max_attempts_env: "SEND_MAX",
            send_max_attempts: 3,
            send_retry_backoff_env: "SEND_BACKOFF",
            send_retry_backoff_ms: 700,
        },
    );

    assert!(direct.retry_transient_read_errors);
    assert!(direct.retry_transient_send_errors);
    assert_eq!(
        direct.rate_limit_scope,
        "in-process per chat id; reset on gateway restart"
    );
    assert!(!direct.raw_token_exposed);

    let from_policy = build_hepta_kernel_telegram_production_guard_status_from_policy(
        HeptaKernelTelegramProductionGuardPolicyInput {
            read_max_attempts_env: "READ_MAX",
            read_max_attempts: Some(999),
            read_retry_backoff_env: "READ_BACKOFF",
            read_retry_backoff_ms: Some(999_999),
            typing_keepalive_env: "TYPING",
            typing_keepalive_enabled: true,
            typing_keepalive_interval_ms: Some(1),
            model_timeout_env: "MODEL_TIMEOUT",
            model_timeout_ms: Some(1),
            model_failure_fallback_env: "MODEL_FALLBACK",
            model_failure_fallback_enabled: true,
            send_min_interval_env: "SEND_MIN",
            send_min_interval_ms: Some(999_999),
            send_max_attempts_env: "SEND_MAX",
            send_max_attempts: Some(0),
            send_retry_backoff_env: "SEND_BACKOFF",
            send_retry_backoff_ms: Some(999_999),
        },
    );

    assert_eq!(
        from_policy.read_max_attempts,
        MAX_TELEGRAM_READ_MAX_ATTEMPTS
    );
    assert_eq!(
        from_policy.read_retry_backoff_ms,
        MAX_TELEGRAM_READ_RETRY_BACKOFF_MS
    );
    assert_eq!(from_policy.typing_keepalive_interval_ms, 1_000);
    assert_eq!(from_policy.model_timeout_ms, MIN_TELEGRAM_MODEL_TIMEOUT_MS);
    assert_eq!(
        from_policy.send_min_interval_ms,
        MAX_TELEGRAM_SEND_MIN_INTERVAL_MS
    );
    assert_eq!(from_policy.send_max_attempts, 1);
    assert_eq!(
        from_policy.send_retry_backoff_ms,
        MAX_TELEGRAM_SEND_RETRY_BACKOFF_MS
    );
    assert!(!from_policy.raw_token_exposed);
}
