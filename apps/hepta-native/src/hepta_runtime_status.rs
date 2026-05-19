//! Read-only runtime status model for Hepta Native.
//!
//! This module is intentionally local and side-effect free. It gives the native
//! desktop/mobile UI a small, testable status substrate for Hepta's native
//! OpenClaw-parity runtime capabilities without calling OpenClaw Gateway,
//! Matrix send APIs, model providers, tool executors, or approval mutation paths.

use crate::hepta_fixture_smoke::sample_current_codex_fixture_smoke_report;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaRuntimeStatusKind {
    Ready,
    PreviewOnly,
    Gated,
    Blocked,
}

impl HeptaRuntimeStatusKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PreviewOnly => "preview_only",
            Self::Gated => "gated",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaRuntimeStatusItem {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: HeptaRuntimeStatusKind,
    pub evidence: &'static str,
    pub external_mutation_enabled: bool,
}

impl HeptaRuntimeStatusItem {
    pub fn operator_line(&self) -> String {
        format!(
            "{} · {} · {} · external_mutation_enabled={}",
            self.label,
            self.kind.label(),
            self.evidence,
            self.external_mutation_enabled,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaM4ProductReadinessItem {
    pub capability_id: &'static str,
    pub title: &'static str,
    pub product_ready: bool,
    pub missing_gates: &'static [&'static str],
}

impl HeptaM4ProductReadinessItem {
    pub fn operator_line(&self) -> String {
        format!(
            "{} · product_ready={} · missing={}",
            self.title,
            self.product_ready,
            self.missing_gates.join(",")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaRuntimeStatusSnapshot {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub capability_maturity_summary: &'static str,
    pub items: Vec<HeptaRuntimeStatusItem>,
    pub m4_product_readiness: Vec<HeptaM4ProductReadinessItem>,
}

impl HeptaRuntimeStatusSnapshot {
    pub fn ready_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.kind == HeptaRuntimeStatusKind::Ready)
            .count()
    }

    pub fn preview_only_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.kind == HeptaRuntimeStatusKind::PreviewOnly)
            .count()
    }

    pub fn gated_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.kind == HeptaRuntimeStatusKind::Gated)
            .count()
    }

    pub fn blocked_count(&self) -> usize {
        self.items
            .iter()
            .filter(|item| item.kind == HeptaRuntimeStatusKind::Blocked)
            .count()
    }

    pub fn summary_line(&self) -> String {
        format!(
            "{} ready · {} preview-only · {} gated · {} blocked",
            self.ready_count(),
            self.preview_only_count(),
            self.gated_count(),
            self.blocked_count(),
        )
    }

    pub fn m4_product_ready_count(&self) -> usize {
        self.m4_product_readiness
            .iter()
            .filter(|item| item.product_ready)
            .count()
    }

    pub fn m4_missing_gate_count(&self, gate: &str) -> usize {
        self.m4_product_readiness
            .iter()
            .filter(|item| item.missing_gates.contains(&gate))
            .count()
    }

    pub fn m4_readiness_line(&self) -> String {
        format!(
            "{}/{} M4 product-ready · missing live_adapter={} · visible_in_native=true",
            self.m4_product_ready_count(),
            self.m4_product_readiness.len(),
            self.m4_missing_gate_count("live_adapter"),
        )
    }
}

pub fn sample_runtime_status_snapshot() -> HeptaRuntimeStatusSnapshot {
    let fixture_smoke = sample_current_codex_fixture_smoke_report();
    HeptaRuntimeStatusSnapshot {
        title: "Runtime event plane",
        subtitle: "Local Matrix-heart status snapshot for Hepta-owned native capability parity; no OpenClaw Gateway status request is performed.",
        capability_maturity_summary: "23/23 runtime capability domains are at least M2-local-adapter; 20/23 are M4 product-ready, including 7/7 Hepta-unique domains and 13/13 OpenClaw required absorb domains. External channel sends, external providers, external ACP processes, and Gateway mutations remain separately gated.",
        items: vec![
            HeptaRuntimeStatusItem {
                id: "matrix-heart-fixture",
                label: "Matrix-heart fixture",
                kind: HeptaRuntimeStatusKind::Ready,
                evidence: "m.hepta.* cards render from local fixture events",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "current-codex-fixture-smoke",
                label: "Current codex-rs fixture smoke",
                kind: if fixture_smoke.ready() {
                    HeptaRuntimeStatusKind::Ready
                } else {
                    HeptaRuntimeStatusKind::Gated
                },
                evidence: "bounded fixture smoke verifies current codex-rs runtime bridge event, known m.hepta.* event types, redaction, and false Gateway/provider/channel/process side-effect flags",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "runtime-m2-coverage",
                label: "Runtime M2+ local-adapter coverage",
                kind: HeptaRuntimeStatusKind::Ready,
                evidence: "runtime capability matrix reports all 23 capability domains at least M2-local-adapter, all 7 Hepta-unique domains at M4-product-ready, and all 13 OpenClaw required absorb domains at M4-product-ready",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "session-task-tool-ledgers",
                label: "Session/task/tool ledgers",
                kind: HeptaRuntimeStatusKind::Ready,
                evidence: "session transcript, inbound router, task board, delivery queue, tool invocation, model router, memory context, scheduler, process supervisor, and agent harness stores are local-only",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "composer-action-bridge",
                label: "Composer/action bridge",
                kind: HeptaRuntimeStatusKind::PreviewOnly,
                evidence: "reserved commands stage dry-run previews only",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "approval-confirmation",
                label: "Approval confirmation",
                kind: HeptaRuntimeStatusKind::PreviewOnly,
                evidence: "exact payload modal records local acknowledgement only",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "action-outbox",
                label: "Action outbox",
                kind: HeptaRuntimeStatusKind::Ready,
                evidence: "queue lanes expose staged, blocked, payload-inspected, and evidence states",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "m3-live-gates",
                label: "M3 live adapter gates",
                kind: HeptaRuntimeStatusKind::Gated,
                evidence: "sessions, agent harness, task registry, delivery queue, inbound routing, output directives, tool invocation, approval broker, process supervisor, scheduler, memory context, model provider, and config store have confirmed local handoff gates; external sends, external model/provider invocation, ACP process start, task mutation, and external scheduler/session delivery remain disabled until policy/readback wiring is complete",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "m4-product-readiness",
                label: "M4 product readiness",
                kind: HeptaRuntimeStatusKind::Gated,
                evidence: "Native runtime status surfaces all 20 M4 product gates; runtime-kernel, Matrix-heart, action-outbox, worker-task-board, topic-neuron-intuition with neuron compression v2 and neuron-intuition calibration, memory-intelligence-readiness plus runtime handoff/store readback/prompt assembly/live-turn preflight/turn dispatch/provider rehearsal/installed telemetry/activation cutover/provider-router activation planning/runtime adapter readback/neuron compression v2 readback/neuron-intuition calibration readback/live-canary approval readback/local-fixture canary execution readback/external-provider canary preflight readback/external-provider send approval readback/external-provider send runtime adapter readback/external-provider send intent wording readback/local MLX-Qwen exact-marker live canary readback/local MLX-Qwen bounded-context canary readback/memory live-turn context adapter readback/live-session opt-in receipt readback/live-session apply adapter per-turn receipt readback/live-session attachment ledger readback/retention-store durable readback/production mount gate readback/shadow dispatch gate readback, compat-quarantine, sessions, agent, delivery, approval, task, inbound, output, config-store, tool-invocation-policy, memory-context-citation, scheduler-cron-wake, process-followup-supervisor, and model-provider-routing domains are product-ready, 0/20 domains lack live_adapter, and UI visibility is present here",
                external_mutation_enabled: false,
            },
            HeptaRuntimeStatusItem {
                id: "mobile-packaging",
                label: "Mobile packaging",
                kind: HeptaRuntimeStatusKind::Ready,
                evidence: "cargo-makepad installed; Android APK and iOS simulator build smokes validated, UI remains side-effect-free",
                external_mutation_enabled: false,
            },
        ],
        m4_product_readiness: vec![
            HeptaM4ProductReadinessItem {
                capability_id: "runtime-kernel",
                title: "Rust-native RuntimeKernel",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "matrix-heart-native-client",
                title: "Robrix Matrix-heart desktop/mobile client substrate",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "action-outbox-payload-inspection",
                title: "Action outbox and exact payload inspection",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "worker-task-board",
                title: "Worker task board, leases, promotion ledger",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "topic-neuron-intuition-runtime",
                title: "Topic graph, neuron state, intuition/model-router feedback",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "memory-intelligence-readiness",
                title: "Hepta Intelligence memory readiness aggregate",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "compat-quarantine",
                title: "Rust-native HeptaRuntime compatibility quarantine",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "sessions-transcripts-status",
                title: "Sessions, transcripts, session status",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "agent-subagent-acp",
                title: "Agent, subagent, ACP run orchestration",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "task-registry-status-delivery",
                title: "Task registry, status, delivery lifecycle",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "delivery-queue-message-routing",
                title: "Message routing and durable delivery queue",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "inbound-routing",
                title: "Inbound text/event routing and provenance redaction",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "output-directives-silent-replies",
                title: "Output directives, attachments, silent replies",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "tool-invocation-policy",
                title: "Tool invocation, policy, sandbox, approval gating",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "approval-runtime",
                title: "Approval runtime and resume broker",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "process-followup-supervisor",
                title: "Process follow-up and durable process supervisor",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "scheduler-cron-wake",
                title: "Scheduler, cron, wake and reminders",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "memory-context-citation",
                title: "Memory search/get, context delivery, citation boundaries",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "model-provider-routing",
                title: "Model provider registry and routing",
                product_ready: true,
                missing_gates: &[],
            },
            HeptaM4ProductReadinessItem {
                capability_id: "config-status-update-lifecycle",
                title: "Config/status/restart/update lifecycle",
                product_ready: true,
                missing_gates: &[],
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_runtime_status_is_read_only_and_operator_readable() {
        let snapshot = sample_runtime_status_snapshot();
        assert_eq!(snapshot.title, "Runtime event plane");
        assert_eq!(snapshot.ready_count(), 6);
        assert_eq!(snapshot.preview_only_count(), 2);
        assert_eq!(snapshot.gated_count(), 2);
        assert_eq!(snapshot.blocked_count(), 0);
        assert!(snapshot.capability_maturity_summary.contains("23/23"));
        assert!(snapshot
            .capability_maturity_summary
            .contains("at least M2-local-adapter"));
        assert!(snapshot
            .capability_maturity_summary
            .contains("20/23 are M4 product-ready"));
        assert!(snapshot
            .capability_maturity_summary
            .contains("7/7 Hepta-unique domains"));
        assert!(snapshot
            .capability_maturity_summary
            .contains("13/13 OpenClaw required absorb domains"));
        assert!(snapshot
            .items
            .iter()
            .all(|item| !item.external_mutation_enabled));
        assert!(snapshot
            .summary_line()
            .contains("6 ready · 2 preview-only · 2 gated · 0 blocked"));
    }

    #[test]
    fn current_codex_fixture_smoke_is_visible_in_runtime_status() {
        let snapshot = sample_runtime_status_snapshot();
        let smoke = snapshot
            .items
            .iter()
            .find(|item| item.id == "current-codex-fixture-smoke")
            .expect("current codex fixture smoke should be present");

        assert_eq!(smoke.kind, HeptaRuntimeStatusKind::Ready);
        assert!(!smoke.external_mutation_enabled);
        assert!(smoke
            .evidence
            .contains("current codex-rs runtime bridge event"));
        assert!(smoke
            .evidence
            .contains("false Gateway/provider/channel/process"));
    }

    #[test]
    fn native_runtime_parity_is_not_live_ready_until_m3_gates_open() {
        let snapshot = sample_runtime_status_snapshot();
        let m2 = snapshot
            .items
            .iter()
            .find(|item| item.id == "runtime-m2-coverage")
            .expect("M2 coverage status should be present");
        assert_eq!(m2.kind, HeptaRuntimeStatusKind::Ready);
        assert!(m2.evidence.contains("23 capability domains"));
        assert!(m2.evidence.contains("7 Hepta-unique domains"));
        assert!(m2.evidence.contains("13 OpenClaw required absorb domains"));
        assert!(m2.evidence.contains("M4-product-ready"));

        let gate = snapshot
            .items
            .iter()
            .find(|item| item.id == "m3-live-gates")
            .expect("M3 gate status should be present");
        assert_eq!(gate.kind, HeptaRuntimeStatusKind::Gated);
        assert!(!gate.external_mutation_enabled);
        assert!(gate.evidence.contains("remain disabled"));
        assert!(gate.evidence.contains("confirmed local handoff gates"));
        assert!(gate.evidence.contains("sessions"));
        assert!(gate.evidence.contains("memory context"));
        assert!(gate.evidence.contains("model provider"));
        assert!(gate.evidence.contains("policy/readback"));
    }

    #[test]
    fn native_runtime_status_surfaces_m4_product_gate_readiness() {
        let snapshot = sample_runtime_status_snapshot();
        assert_eq!(snapshot.m4_product_readiness.len(), 20);
        assert_eq!(snapshot.m4_product_ready_count(), 20);
        assert_eq!(snapshot.m4_missing_gate_count("live_adapter"), 0);
        assert!(snapshot
            .m4_readiness_line()
            .contains("20/20 M4 product-ready"));
        assert!(snapshot
            .m4_readiness_line()
            .contains("visible_in_native=true"));

        let m4 = snapshot
            .items
            .iter()
            .find(|item| item.id == "m4-product-readiness")
            .expect("M4 product readiness status should be present");
        assert_eq!(m4.kind, HeptaRuntimeStatusKind::Gated);
        assert!(!m4.external_mutation_enabled);
        assert!(m4.evidence.contains("all 20 M4 product gates"));
        assert!(m4.evidence.contains("runtime-kernel"));
        assert!(m4.evidence.contains("worker-task-board"));
        assert!(m4.evidence.contains("topic-neuron-intuition"));
        assert!(m4.evidence.contains("memory-intelligence-readiness"));
        assert!(m4.evidence.contains("compat-quarantine"));
        assert!(m4.evidence.contains("tool-invocation-policy"));
        assert!(m4.evidence.contains("memory-context-citation"));
        assert!(m4.evidence.contains("scheduler-cron-wake"));
        assert!(m4.evidence.contains("process-followup-supervisor"));
        assert!(m4.evidence.contains("model-provider-routing"));
        assert!(m4.evidence.contains("agent"));

        let inbound = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "inbound-routing")
            .expect("inbound product readiness should be visible");
        assert!(inbound.product_ready);
        assert!(inbound.missing_gates.is_empty());
        assert!(inbound.operator_line().contains("product_ready=true"));

        let output_directives = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "output-directives-silent-replies")
            .expect("output directive product readiness should be visible");
        assert!(output_directives.product_ready);
        assert!(output_directives.missing_gates.is_empty());

        let delivery = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "delivery-queue-message-routing")
            .expect("delivery queue product readiness should be visible");
        assert!(delivery.product_ready);
        assert!(delivery.missing_gates.is_empty());

        let sessions = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "sessions-transcripts-status")
            .expect("session transcript product readiness should be visible");
        assert!(sessions.product_ready);
        assert!(sessions.missing_gates.is_empty());

        let config = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "config-status-update-lifecycle")
            .expect("config product readiness should be visible");
        assert!(config.product_ready);
        assert!(config.missing_gates.is_empty());

        let tool_invocation = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "tool-invocation-policy")
            .expect("tool invocation product readiness should be visible");
        assert!(tool_invocation.product_ready);
        assert!(tool_invocation.missing_gates.is_empty());

        let approval = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "approval-runtime")
            .expect("approval product readiness should be visible");
        assert!(approval.product_ready);
        assert!(approval.missing_gates.is_empty());

        let task_registry = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "task-registry-status-delivery")
            .expect("task registry product readiness should be visible");
        assert!(task_registry.product_ready);
        assert!(task_registry.missing_gates.is_empty());

        let memory_context = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "memory-context-citation")
            .expect("memory context product readiness should be visible");
        assert!(memory_context.product_ready);
        assert!(memory_context.missing_gates.is_empty());

        let memory_intelligence = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "memory-intelligence-readiness")
            .expect("memory intelligence product readiness should be visible");
        assert!(memory_intelligence.product_ready);
        assert!(memory_intelligence.missing_gates.is_empty());

        let scheduler = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "scheduler-cron-wake")
            .expect("scheduler product readiness should be visible");
        assert!(scheduler.product_ready);
        assert!(scheduler.missing_gates.is_empty());

        let process = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "process-followup-supervisor")
            .expect("process supervisor product readiness should be visible");
        assert!(process.product_ready);
        assert!(process.missing_gates.is_empty());

        let model_provider = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "model-provider-routing")
            .expect("model provider product readiness should be visible");
        assert!(model_provider.product_ready);
        assert!(model_provider.missing_gates.is_empty());

        let agent_harness = snapshot
            .m4_product_readiness
            .iter()
            .find(|item| item.capability_id == "agent-subagent-acp")
            .expect("agent harness product readiness should be visible");
        assert!(agent_harness.product_ready);
        assert!(agent_harness.missing_gates.is_empty());
    }

    #[test]
    fn mobile_packaging_gate_stays_explicit() {
        let snapshot = sample_runtime_status_snapshot();
        let blocker = snapshot
            .items
            .iter()
            .find(|item| item.id == "mobile-packaging")
            .expect("mobile packaging blocker should be present");
        assert_eq!(blocker.kind, HeptaRuntimeStatusKind::Ready);
        assert!(blocker.evidence.contains("cargo-makepad installed"));
        assert!(blocker
            .evidence
            .contains("Android APK and iOS simulator build smokes validated"));
    }
}
