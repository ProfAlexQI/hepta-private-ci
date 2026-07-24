use hepta_memory::DurableIntegrityKey;
use hepta_runtime::RuntimeKernel;

fn integrity_key() -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([0x51; 32])
}

#[tokio::test]
async fn read_only_canary_persists_intent_and_terminal_receipt_across_reopen() {
    let root = tempfile::tempdir().expect("runtime root");
    let database = root.path().join("outcomes.sqlite3");
    let attempt_id = {
        let runtime = RuntimeKernel::bootstrap_with_durable_outcomes(&database, integrity_key())
            .expect("durable runtime");
        runtime
            .switch_model_in_session("native-gateway:runtime-kernel-canary", "demo/demo-chat")
            .expect("isolated demo model");
        let result = runtime
            .run_demo_turn_in_session(
                "native-gateway:runtime-kernel-canary",
                r#"Use the echo tool with arguments exactly {"text":"canary-binding"}. Do not answer directly."#,
            )
            .await
            .expect("canary turn");
        let receipt = result.execution_receipt.expect("execution receipt");

        assert_eq!(result.invoked_tool.as_deref(), Some("echo"));
        assert!(receipt.durable_intent_recorded);
        assert!(!receipt.effect_plan_recorded);
        assert!(receipt.provider_effect_ack_hash.is_none());
        assert_eq!(receipt.terminal_status, "succeeded");
        assert!(
            runtime
                .terminal_receipt_recorded(&receipt.attempt_id)
                .expect("terminal readback")
        );
        assert!(
            runtime
                .pending_execution_intents()
                .expect("pending intents")
                .is_empty()
        );
        receipt.attempt_id
    };

    let reopened = RuntimeKernel::open_with_durable_outcomes(&database, integrity_key())
        .expect("reopened durable runtime");
    assert!(
        reopened
            .terminal_receipt_recorded(&attempt_id)
            .expect("restart terminal readback")
    );
    assert!(
        reopened
            .pending_execution_intents()
            .expect("restart pending intents")
            .is_empty()
    );
}
