use hepta_runtime::EffectBroker;
use hepta_runtime::EffectPlan;
use hepta_runtime::ExactExecutionAuthority;
use hepta_runtime::ExecutionAdmission;
use hepta_runtime::ExecutionAdmissionError;
use hepta_runtime::ExecutionIngress;
use hepta_runtime::ProviderEffectAck;
use hepta_runtime::TerminalEffectReceipt;
use sha2::Digest;
use sha2::Sha256;

pub(crate) struct McpExecutionAdmission {
    broker: EffectBroker,
    effect_plan_hash: String,
    operation: String,
}

impl McpExecutionAdmission {
    pub(crate) fn new(
        operation: &str,
        request_binding: &str,
        workspace_binding: &str,
        session_binding: &str,
        payload: &str,
    ) -> Result<Self, ExecutionAdmissionError> {
        let caller_hash = digest_hex("mcp-caller", &[request_binding]);
        let workspace_hash = digest_hex("mcp-workspace", &[workspace_binding]);
        let session_hash = digest_hex("mcp-session", &[session_binding]);
        let payload_hash = content_hash("mcp-payload", &[payload]);
        let authority =
            ExactExecutionAuthority::new(caller_hash.clone(), workspace_hash, session_hash)?;
        let admission = ExecutionAdmission::new(
            ExecutionIngress::McpServer,
            operation,
            authority,
            caller_hash.clone(),
            payload_hash.clone(),
        )?;
        let plan = EffectPlan::new(
            admission.admission_hash(),
            "thread_submission",
            session_binding,
            payload_hash,
            caller_hash,
        )?;
        let effect_plan_hash = plan.effect_plan_hash().to_string();
        let mut broker = EffectBroker::admit(admission);
        broker.record_effect_plan(plan)?;
        Ok(Self {
            broker,
            effect_plan_hash,
            operation: operation.to_string(),
        })
    }

    pub(crate) fn complete(
        &mut self,
        provider_receipt: &str,
        terminal_status: &str,
    ) -> Result<(), ExecutionAdmissionError> {
        let ack = ProviderEffectAck::new(
            &self.effect_plan_hash,
            "codex-thread-manager",
            content_hash("mcp-provider-receipt", &[provider_receipt]),
        )?;
        let ack_hash = ack.ack_hash().to_string();
        self.broker.record_provider_ack(ack)?;
        let receipt = TerminalEffectReceipt::terminal(
            ack_hash,
            terminal_status,
            content_hash("mcp-terminal-runtime", &[&self.operation, provider_receipt]),
            content_hash(
                "mcp-terminal-evidence",
                &[terminal_status, provider_receipt],
            ),
        )?;
        self.broker.record_terminal_receipt(receipt)
    }
}

fn digest_hex(domain: &str, values: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    for value in values {
        hasher.update((value.len() as u64).to_be_bytes());
        hasher.update(value.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn content_hash(domain: &str, values: &[&str]) -> String {
    format!("sha256:{}", digest_hex(domain, values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_admission_reaches_terminal_after_provider_acceptance() {
        let mut admission = McpExecutionAdmission::new(
            "mcp-session-start",
            "request-1",
            "/tmp",
            "new-thread",
            "hello",
        )
        .unwrap();
        admission
            .complete("thread-submit-accepted", "succeeded")
            .unwrap();
    }

    #[test]
    fn mcp_admission_denies_duplicate_provider_completion() {
        let mut admission = McpExecutionAdmission::new(
            "mcp-session-interrupt",
            "request-2",
            "/tmp",
            "thread-2",
            "interrupt",
        )
        .unwrap();
        admission
            .complete("interrupt-accepted", "succeeded")
            .unwrap();
        assert!(
            admission
                .complete("interrupt-accepted", "succeeded")
                .is_err()
        );
    }
}
