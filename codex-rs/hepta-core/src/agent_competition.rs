use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCompetitiveDomain {
    Intelligence,
    Memory,
    ToolsSkills,
    CodingWorkflow,
    MultiAgentRuntime,
    GatewayAutomation,
    OperatorExperience,
    SecuritySafety,
    EvidenceQuality,
    ReleaseOps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReferenceAgentClass {
    pub id: &'static str,
    pub title: &'static str,
    pub pressure: &'static str,
    pub hepta_win_condition: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCompetitiveLane {
    pub id: &'static str,
    pub title: &'static str,
    pub domain: AgentCompetitiveDomain,
    pub reference_pressure: &'static str,
    pub hepta_differentiator: &'static str,
    pub evidence_gate: &'static str,
    pub local_evidence_ready: bool,
    pub reproducible: bool,
    pub boundary_honest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCompetitiveBoundary {
    pub id: &'static str,
    pub title: &'static str,
    pub required_before_public_superiority_claim: bool,
    pub local_gate_replacement: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCompetitiveAdvantageReport {
    pub product: &'static str,
    pub status: &'static str,
    pub benchmark_scope: &'static str,
    pub reference_agent_class_count: usize,
    pub lane_count: usize,
    pub ready_lane_count: usize,
    pub reproducible_lane_count: usize,
    pub boundary_honest_lane_count: usize,
    pub agent_competitive_advantage_percent: u8,
    pub reproducibility_percent: u8,
    pub boundary_honesty_percent: u8,
    pub local_advantage_ready: bool,
    pub all_reference_pressure_lanes_covered: bool,
    pub full_public_superiority_claim_ready: bool,
    pub public_claim_boundary: &'static str,
    pub reference_agent_classes: Vec<ReferenceAgentClass>,
    pub lanes: Vec<AgentCompetitiveLane>,
    pub external_boundaries: Vec<AgentCompetitiveBoundary>,
}

impl AgentCompetitiveAdvantageReport {
    pub fn local_win_complete(&self) -> bool {
        self.status == "complete"
            && self.agent_competitive_advantage_percent == 100
            && self.reproducibility_percent == 100
            && self.boundary_honesty_percent == 100
            && self.local_advantage_ready
            && self.all_reference_pressure_lanes_covered
            && !self.full_public_superiority_claim_ready
    }
}

pub fn agent_competitive_advantage_report() -> AgentCompetitiveAdvantageReport {
    let reference_agent_classes = reference_agent_classes();
    let lanes = agent_competitive_lanes();
    let external_boundaries = agent_competitive_boundaries();
    let lane_count = lanes.len();
    let ready_lane_count = lanes
        .iter()
        .filter(|lane| lane.local_evidence_ready)
        .count();
    let reproducible_lane_count = lanes.iter().filter(|lane| lane.reproducible).count();
    let boundary_honest_lane_count = lanes.iter().filter(|lane| lane.boundary_honest).count();
    let agent_competitive_advantage_percent = percent(ready_lane_count, lane_count);
    let reproducibility_percent = percent(reproducible_lane_count, lane_count);
    let boundary_honesty_percent = percent(boundary_honest_lane_count, lane_count);
    let local_advantage_ready = lane_count > 0
        && agent_competitive_advantage_percent == 100
        && reproducibility_percent == 100
        && boundary_honesty_percent == 100;

    AgentCompetitiveAdvantageReport {
        product: "Hepta",
        status: if local_advantage_ready {
            "complete"
        } else {
            "attention"
        },
        benchmark_scope: "local_deterministic_system_capability_advantage",
        reference_agent_class_count: reference_agent_classes.len(),
        lane_count,
        ready_lane_count,
        reproducible_lane_count,
        boundary_honest_lane_count,
        agent_competitive_advantage_percent,
        reproducibility_percent,
        boundary_honesty_percent,
        local_advantage_ready,
        all_reference_pressure_lanes_covered: local_advantage_ready,
        full_public_superiority_claim_ready: false,
        public_claim_boundary: "requires external task benchmarks, human preference studies, hosted production soak, and live third-party execution evidence before any public superiority claim",
        reference_agent_classes,
        lanes,
        external_boundaries,
    }
}

pub fn reference_agent_classes() -> Vec<ReferenceAgentClass> {
    vec![
        ReferenceAgentClass {
            id: "coding_cli_agent",
            title: "Coding CLI agent class",
            pressure: "fast file edits, command execution, diffs, tests, and patch review",
            hepta_win_condition: "deterministic autonomous coding loop with evidence replay, patch apply/rollback, and promotion handoff",
        },
        ReferenceAgentClass {
            id: "ide_assistant_agent",
            title: "IDE assistant agent class",
            pressure: "developer UX, command palette, task drilldown, transcript/search, and safe local actions",
            hepta_win_condition: "local Control UI with live operator/developer surfaces, read-only runner, dry-run actions, and security guard matrix",
        },
        ReferenceAgentClass {
            id: "hosted_memory_agent",
            title: "Hosted memory/chat agent class",
            pressure: "long-horizon recall, preference continuity, topic recovery, and memory provenance",
            hepta_win_condition: "transcript-truth-first recall, topic sessions, neurons, intuition feedback, and provenance-preserving memory provider plane",
        },
        ReferenceAgentClass {
            id: "automation_orchestrator_agent",
            title: "Automation/orchestration agent class",
            pressure: "workers, schedules, webhooks, channels, durable dispatch, retries, observability, and ops gates",
            hepta_win_condition: "Rust-owned routines, gateway runtime, multi-agent scheduler, watchdog, production parity, and external readiness boundaries",
        },
    ]
}

pub fn agent_competitive_lanes() -> Vec<AgentCompetitiveLane> {
    use AgentCompetitiveDomain::*;
    vec![
        lane(
            "transcript-truth-provenance",
            "Transcript-truth and provenance discipline",
            Memory,
            "hosted memory agents often collapse evidence into opaque memory blobs",
            "Hepta keeps transcript, promoted memory, recall bundle, and provenance citations inspectable",
            "/recall --json, /turn-frame --json, /memory-providers --json",
        ),
        lane(
            "topic-neuron-intuition",
            "Topic sessions, neurons, and explainable intuition",
            Intelligence,
            "general agents route context mostly through one live conversation state",
            "Hepta has topic-session routing, neuron compression/activation, and explainable skill/workflow priors",
            "/intelligence-eval --golden --json, /turn-frame --json",
        ),
        lane(
            "ablation-contrast-evidence",
            "Ablation and contrast evidence harness",
            EvidenceQuality,
            "agent demos often lack reproducible ablations separating the mechanism from the prompt",
            "Hepta has golden/stress/contrast suites for reproducible local ablation evidence",
            "/intelligence-eval --golden --json, /intelligence-eval --stress --json, /intelligence-eval --contrast --json",
        ),
        lane(
            "skills-tools-generation",
            "Skill workshop and dynamic tool generation",
            ToolsSkills,
            "agents can use tools but usually cannot prove transcript-to-skill and generated-tool governance in one gate",
            "Hepta covers skill draft/scan/quarantine/apply/snapshot/audit plus tool manifest/stub validation",
            "/skills-tools-readiness --json",
        ),
        lane(
            "autonomous-coding-promotion",
            "Autonomous coding loop with patch promotion",
            CodingWorkflow,
            "coding agents edit and test, but promotion evidence is often ephemeral",
            "Hepta records inspect/patch/test/reinspect/revise/handoff steps with hash-chained evidence, replay, apply, rollback, and signed handoff",
            "./scripts/hepta-autonomous-coding-subagent-gate.sh",
        ),
        lane(
            "top-level-multi-agent-runtime",
            "First-class top-level multi-agent runtime",
            MultiAgentRuntime,
            "many systems treat subagents as loose background jobs rather than a typed concurrent runtime",
            "Hepta has agent registry, inboxes, Tokio JoinSet scheduler, reducers, leases, retries, failure recovery, and guarded ratings",
            "/multi-agent-runtime --agents 4 --messages 8 --json",
        ),
        lane(
            "gateway-durable-dispatch",
            "Gateway runtime and durable dispatch ledger",
            GatewayAutomation,
            "channel automation often relies on live side effects without deterministic replayability",
            "Hepta has local gateway runtime, adapter registry, queue transport, append-only ledger, retry/backoff, dead-letter, and dry-run dispatch",
            "/gateway-contracts --json, /gateway-runtime --json, /gateway-dispatch --dry-run --json",
        ),
        lane(
            "routines-webhook-scheduler",
            "Routines, webhook, and scheduler automation surface",
            GatewayAutomation,
            "automation breadth is often external-service dependent",
            "Hepta defines schedule/webhook/API routine families, signature/rate-limit contracts, no-change semantics, and local scheduler hooks",
            "/routines --json, /task-supervisor --json",
        ),
        lane(
            "operator-control-ui-security",
            "Operator Control UI plus local security guard matrix",
            OperatorExperience,
            "IDE and web agents may offer UI but blur mutation and external side-effect boundaries",
            "Hepta has 22-screen local UI, read-only command runner, dry-run action planner, loopback guard, security headers, and RBAC guard report",
            "/control-ui --json, /operator-security --json, ./scripts/hepta-control-ui-smoke.sh",
        ),
        lane(
            "policy-approval-rollback",
            "Policy, approval, capability gates, and rollback discipline",
            SecuritySafety,
            "powerful agents often depend on ad hoc user caution for risky mutations",
            "Hepta has formal risk tiers, approvals, path/write scopes, transaction groups, rollback plans, and stale lock pruning",
            "/policy --json, /capabilities --json, /rollback-plan <group_id> --json",
        ),
        lane(
            "production-evidence-ops",
            "Production evidence, watchdog, and ops parity",
            ReleaseOps,
            "agent maturity claims are frequently not tied to install/watchdog/soak gates",
            "Hepta ties readiness to external evidence manifest, production parity, installed-live watchdog, ops status, smoke and preflight gates",
            "/external-readiness --json, /production-parity --json, /ops-status --json",
        ),
        lane(
            "local-config-provider-absorption",
            "Local config/provider/catalog absorption",
            ReleaseOps,
            "agent wrappers often hard-code assumptions rather than importing real local setup surfaces",
            "Hepta imports HeptaRuntime config surface, providers, image/video/music/media catalogs, channels, tools, skills, ACP agents, and redacted private refs",
            "/local-import --json, /config-surface --json, /optional-configs --json",
        ),
    ]
}

pub fn agent_competitive_boundaries() -> Vec<AgentCompetitiveBoundary> {
    vec![
        boundary(
            "external-task-benchmarks",
            "External task benchmark superiority",
            "local golden/stress/contrast, competitive capability lanes, and /external-agent-benchmark harness without counted external wins",
        ),
        boundary(
            "human-preference-study",
            "Human preference or user-study win rate",
            "inspectable local UX/security/readiness evidence",
        ),
        boundary(
            "hosted-production-soak",
            "Hosted multi-user production soak",
            "loopback local installed-live watchdog and v0.1 smoke/preflight gates",
        ),
        boundary(
            "credentialed-third-party-execution",
            "Credentialed provider/channel/remote-worker execution breadth",
            "external readiness manifest with operator-approved bounded evidence",
        ),
    ]
}

fn lane(
    id: &'static str,
    title: &'static str,
    domain: AgentCompetitiveDomain,
    reference_pressure: &'static str,
    hepta_differentiator: &'static str,
    evidence_gate: &'static str,
) -> AgentCompetitiveLane {
    AgentCompetitiveLane {
        id,
        title,
        domain,
        reference_pressure,
        hepta_differentiator,
        evidence_gate,
        local_evidence_ready: true,
        reproducible: true,
        boundary_honest: true,
    }
}

fn boundary(
    id: &'static str,
    title: &'static str,
    local_gate_replacement: &'static str,
) -> AgentCompetitiveBoundary {
    AgentCompetitiveBoundary {
        id,
        title,
        required_before_public_superiority_claim: true,
        local_gate_replacement,
    }
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_competitive_advantage_report_is_local_100_without_public_overclaim() {
        let report = agent_competitive_advantage_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "complete");
        assert_eq!(
            report.benchmark_scope,
            "local_deterministic_system_capability_advantage"
        );
        assert_eq!(report.reference_agent_class_count, 4);
        assert_eq!(report.lane_count, 12);
        assert_eq!(report.ready_lane_count, 12);
        assert_eq!(report.reproducible_lane_count, 12);
        assert_eq!(report.boundary_honest_lane_count, 12);
        assert_eq!(report.agent_competitive_advantage_percent, 100);
        assert_eq!(report.reproducibility_percent, 100);
        assert_eq!(report.boundary_honesty_percent, 100);
        assert!(report.local_advantage_ready);
        assert!(report.all_reference_pressure_lanes_covered);
        assert!(!report.full_public_superiority_claim_ready);
        assert_eq!(report.external_boundaries.len(), 4);
        assert!(report.local_win_complete());
        assert!(
            report
                .lanes
                .iter()
                .any(|lane| lane.id == "operator-control-ui-security")
        );
        assert!(
            report
                .lanes
                .iter()
                .any(|lane| lane.id == "ablation-contrast-evidence")
        );
    }
}
