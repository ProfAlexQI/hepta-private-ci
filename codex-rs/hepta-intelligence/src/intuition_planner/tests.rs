use hepta_core::ApprovalRequirement;
use hepta_core::IntuitionActionMode;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::RiskTier;
use hepta_core::SessionId;
use hepta_core::ToolExecutionMetadata;
use hepta_core::TopicActivationScore;
use hepta_core::TopicId;
use hepta_core::TopicLabel;

use super::IntuitionCapabilityView;
use super::IntuitionPlan;
use super::IntuitionPlanInput;
use super::IntuitionPolicyBinding;
use super::plan_intuition;

#[test]
fn zero_limit_returns_empty_plan() {
    let score = topic_score("topic-alpha", "read file architecture", 0.72);
    let capabilities = compatibility_capabilities();
    let policies = compatibility_policies();

    let plan = plan(
        "read file architecture",
        &[score],
        &[],
        &capabilities,
        &policies,
        0,
    );

    assert!(plan.workflow_priors.is_empty());
    assert!(plan.skill_decisions.is_empty());
}

#[test]
fn read_intent_deterministically_binds_runtime_compatibility_capability() {
    let score = topic_score("topic-read", "read file architecture notes", 0.72);
    let capabilities = compatibility_capabilities();
    let policies = compatibility_policies();

    let plan = plan(
        "read file architecture notes",
        &[score],
        &[],
        &capabilities,
        &policies,
        1,
    );

    let decision = &plan.skill_decisions[0];
    assert_eq!(decision.skill_id, "read_file");
    assert!(decision.exists_in_registry);
    assert_eq!(decision.missing_capability, None);
    assert_eq!(decision.risk_tier, Some(RiskTier::Medium));
    assert!(decision.requires_confirmation);
    assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
    assert!(
        decision
            .reason
            .as_deref()
            .unwrap_or_default()
            .contains("bound to runtime tool registry entry 'read_file'")
    );
}

#[test]
fn approval_none_prepares_read_without_granting_execution() {
    let score = topic_score("topic-read", "read file architecture notes", 0.72);
    let capabilities = compatibility_capabilities();
    let mut policies = compatibility_policies();
    let read_policy = policies
        .iter_mut()
        .find(|binding| binding.capability_name == "read_file")
        .expect("read policy");
    read_policy.requirement = ApprovalRequirement::None;
    read_policy.reason = "alpha session may preflight read_file suggestions".into();
    read_policy.matched_rule_id = Some("policy-alpha-read".into());

    let plan = plan(
        "read file architecture notes",
        &[score],
        &[],
        &capabilities,
        &policies,
        1,
    );

    let decision = &plan.skill_decisions[0];
    assert_eq!(decision.skill_id, "read_file");
    assert!(!decision.requires_confirmation);
    assert_eq!(decision.action_mode, IntuitionActionMode::Prepare);
    let reason = decision.reason.as_deref().unwrap_or_default();
    assert!(reason.contains("approval=none"));
    assert!(reason.contains("policy_rule=policy-alpha-read"));
    assert!(reason.contains("alpha session may preflight read_file suggestions"));
}

#[test]
fn denied_write_remains_suggest_only() {
    let score = topic_score("topic-write", "create file release notes", 0.68);
    let capabilities = compatibility_capabilities();
    let policies = compatibility_policies();

    let plan = plan(
        "create file release notes",
        &[score],
        &[],
        &capabilities,
        &policies,
        1,
    );

    let decision = &plan.skill_decisions[0];
    assert_eq!(decision.skill_id, "write_file");
    assert_eq!(decision.risk_tier, Some(RiskTier::High));
    assert!(decision.requires_confirmation);
    assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
    let reason = decision.reason.as_deref().unwrap_or_default();
    assert!(reason.contains("approval=deny"));
    assert!(reason.contains("denied by default"));
}

#[test]
fn workflow_ties_are_stably_ordered_by_registry_id() {
    let score = topic_score("topic-tie", "memory rust", 0.50);

    let plan = plan("memory rust", &[score], &[], &[], &[], 1);

    assert_eq!(
        plan.workflow_priors[0].workflow_id,
        "workflow:engineering-change"
    );
    assert_eq!(
        plan.workflow_priors[0].action_mode,
        IntuitionActionMode::Prepare
    );
}

#[test]
fn unknown_workflow_and_skill_stay_gated_with_missing_capabilities() {
    let score = topic_score("topic-opaque", "opaque subject", 0.40);

    let plan = plan("opaque subject", &[score], &[], &[], &[], 1);

    let workflow = &plan.workflow_priors[0];
    assert_eq!(workflow.workflow_id, "workflow-bootstrap:topic-opaque");
    assert!(!workflow.exists_in_registry);
    assert_eq!(
        workflow.missing_capability.as_deref(),
        Some("workflow_registry_binding_pending")
    );
    assert!(workflow.requires_confirmation);
    assert_eq!(workflow.action_mode, IntuitionActionMode::SuggestOnly);

    let skill = &plan.skill_decisions[0];
    assert_eq!(skill.skill_id, "skill-bootstrap:topic-opaque:followup");
    assert!(!skill.exists_in_registry);
    assert_eq!(
        skill.missing_capability.as_deref(),
        Some("bootstrap_skill_registry_binding_pending")
    );
    assert!(skill.requires_confirmation);
    assert_eq!(skill.action_mode, IntuitionActionMode::SuggestOnly);
}

#[test]
fn matching_feedback_preserves_exact_score_delta() {
    let score = topic_score("topic-feedback", "rust worker pipeline", 0.20);
    let baseline = plan(
        "rust worker pipeline",
        std::slice::from_ref(&score),
        &[],
        &[],
        &[],
        1,
    );
    let feedback = IntuitionFeedbackRecord {
        decision_id: Some("feedback-1".into()),
        surface_session_id: SessionId("session-alpha".into()),
        user_intent: "rust worker pipeline".into(),
        outcome: IntuitionFeedbackOutcome::Accepted,
        skill_id: None,
        workflow_id: Some("workflow:engineering-change".into()),
        source_topic_ids: vec![score.topic_id.clone()],
        source_neuron_ids: Vec::new(),
        weight_delta: 0.12,
        observed_outcome: None,
        latency_ms: None,
        cost: None,
        user_correction: None,
        confidence_before: Some(0.50),
        confidence_after: Some(0.62),
        reason: Some("accepted workflow".into()),
        created_at_unix_ms: 1,
    };

    let calibrated = plan("rust worker pipeline", &[score], &[feedback], &[], &[], 1);

    let delta = calibrated.workflow_priors[0].score - baseline.workflow_priors[0].score;
    assert!((delta - 0.12).abs() < f32::EPSILON * 4.0);
}

#[test]
fn planner_never_emits_execution_authority() {
    let scores = vec![
        topic_score("topic-read", "read file architecture", 0.72),
        topic_score("topic-write", "create file release notes", 0.68),
        topic_score("topic-echo", "echo smoke test", 0.60),
    ];
    let capabilities = compatibility_capabilities();
    let mut policies = compatibility_policies();
    policies
        .iter_mut()
        .find(|binding| binding.capability_name == "echo")
        .expect("echo policy")
        .requirement = ApprovalRequirement::None;

    let plan = plan(
        "read file architecture create file release notes echo smoke test",
        &scores,
        &[],
        &capabilities,
        &policies,
        scores.len(),
    );

    assert!(
        plan.workflow_priors
            .iter()
            .all(|prior| prior.action_mode != IntuitionActionMode::ExecuteAllowed)
    );
    assert!(
        plan.skill_decisions
            .iter()
            .all(|decision| decision.action_mode != IntuitionActionMode::ExecuteAllowed)
    );
}

fn plan(
    user_intent: &str,
    topic_scores: &[TopicActivationScore],
    intuition_feedback: &[IntuitionFeedbackRecord],
    capabilities: &[IntuitionCapabilityView],
    policy_bindings: &[IntuitionPolicyBinding],
    limit: usize,
) -> IntuitionPlan {
    plan_intuition(IntuitionPlanInput {
        user_intent,
        topic_scores,
        activations: &[],
        compressed_neurons: &[],
        intuition_feedback,
        capabilities,
        policy_bindings,
        limit,
    })
}

fn topic_score(topic_id: &str, label: &str, score: f32) -> TopicActivationScore {
    TopicActivationScore {
        topic_id: TopicId(topic_id.into()),
        topic_label: TopicLabel(label.into()),
        score,
        matched_terms: Vec::new(),
        reason: Some("test topic route".into()),
    }
}

fn compatibility_capabilities() -> Vec<IntuitionCapabilityView> {
    vec![
        IntuitionCapabilityView {
            name: "echo".into(),
            risk_tier: RiskTier::Low,
            execution_metadata: ToolExecutionMetadata {
                read_only: true,
                destructive: false,
                idempotent: true,
                produces_structured_output: true,
            },
            default_approval_requirement: ApprovalRequirement::None,
        },
        IntuitionCapabilityView {
            name: "read_file".into(),
            risk_tier: RiskTier::Medium,
            execution_metadata: ToolExecutionMetadata {
                read_only: true,
                destructive: false,
                idempotent: true,
                produces_structured_output: true,
            },
            default_approval_requirement: ApprovalRequirement::Ask,
        },
        IntuitionCapabilityView {
            name: "write_file".into(),
            risk_tier: RiskTier::High,
            execution_metadata: ToolExecutionMetadata {
                read_only: false,
                destructive: true,
                idempotent: false,
                produces_structured_output: true,
            },
            default_approval_requirement: ApprovalRequirement::Deny,
        },
    ]
}

fn compatibility_policies() -> Vec<IntuitionPolicyBinding> {
    vec![
        IntuitionPolicyBinding {
            capability_name: "echo".into(),
            requirement: ApprovalRequirement::None,
            reason: "low-risk read-only tool".into(),
            matched_rule_id: None,
        },
        IntuitionPolicyBinding {
            capability_name: "read_file".into(),
            requirement: ApprovalRequirement::Ask,
            reason: "medium-risk tool requires approval".into(),
            matched_rule_id: None,
        },
        IntuitionPolicyBinding {
            capability_name: "write_file".into(),
            requirement: ApprovalRequirement::Deny,
            reason: "high-risk tool denied by default".into(),
            matched_rule_id: None,
        },
    ]
}
