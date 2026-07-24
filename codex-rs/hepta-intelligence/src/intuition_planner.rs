//! Pure compatibility planner for legacy intuition workflow and skill previews.
//!
//! Runtime hydrates topic, neuron, feedback, capability, and policy projections.
//! Intelligence owns the deterministic generation, ranking, and binding rules.
//! The returned action modes are planning-only: this module never emits
//! [`IntuitionActionMode::ExecuteAllowed`] and cannot authorize execution.

use hepta_core::ApprovalRequirement;
use hepta_core::HeptaNeuron;
use hepta_core::IntuitionActionMode;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::NeuronActivation;
use hepta_core::RiskTier;
use hepta_core::SkillActivationDecision;
use hepta_core::ToolExecutionMetadata;
use hepta_core::TopicActivationScore;
use hepta_core::TopicLabel;
use hepta_core::WorkflowPrior;

use crate::compute_intuition_feedback_delta;

/// Capability metadata projected into the cognition boundary for planning.
///
/// This is deliberately not an execution handle or authorization witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntuitionCapabilityView {
    pub name: String,
    pub risk_tier: RiskTier,
    pub execution_metadata: ToolExecutionMetadata,
    pub default_approval_requirement: ApprovalRequirement,
}

/// Evaluated policy metadata projected into the cognition boundary.
///
/// The planner uses this only to rank and explain gated previews. Kernel
/// admission and commit authorization remain separate authority boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntuitionPolicyBinding {
    pub capability_name: String,
    pub requirement: ApprovalRequirement,
    pub reason: String,
    pub matched_rule_id: Option<String>,
}

/// Hydrated immutable inputs for one compatibility intuition plan.
#[derive(Debug, Clone, Copy)]
pub struct IntuitionPlanInput<'a> {
    pub user_intent: &'a str,
    pub topic_scores: &'a [TopicActivationScore],
    pub activations: &'a [NeuronActivation],
    pub compressed_neurons: &'a [HeptaNeuron],
    pub intuition_feedback: &'a [IntuitionFeedbackRecord],
    pub capabilities: &'a [IntuitionCapabilityView],
    pub policy_bindings: &'a [IntuitionPolicyBinding],
    pub limit: usize,
}

/// Planning-only workflow and skill projections produced by intelligence.
#[derive(Debug, Clone, PartialEq)]
pub struct IntuitionPlan {
    pub workflow_priors: Vec<WorkflowPrior>,
    pub skill_decisions: Vec<SkillActivationDecision>,
}

/// Produces one deterministic workflow/skill plan without side effects.
///
/// `SuggestOnly` and `Prepare` are the strongest action modes this compatibility
/// planner can emit. Neither mode is execution authority.
pub fn plan_intuition(input: IntuitionPlanInput<'_>) -> IntuitionPlan {
    let workflow_priors = build_bootstrap_workflow_priors(
        input.user_intent,
        input.topic_scores,
        input.activations,
        input.compressed_neurons,
        input.intuition_feedback,
        input.limit,
    );
    let skill_decisions = build_bootstrap_skill_decisions(
        input.user_intent,
        input.topic_scores,
        input.activations,
        input.compressed_neurons,
        &workflow_priors,
        input.intuition_feedback,
        input.capabilities,
        input.policy_bindings,
        input.limit,
    );

    debug_assert!(
        workflow_priors
            .iter()
            .all(|prior| prior.action_mode != IntuitionActionMode::ExecuteAllowed)
    );
    debug_assert!(
        skill_decisions
            .iter()
            .all(|decision| decision.action_mode != IntuitionActionMode::ExecuteAllowed)
    );

    IntuitionPlan {
        workflow_priors,
        skill_decisions,
    }
}

fn build_bootstrap_workflow_priors(
    user_intent: &str,
    topic_scores: &[TopicActivationScore],
    activations: &[NeuronActivation],
    compressed_neurons: &[HeptaNeuron],
    intuition_feedback: &[IntuitionFeedbackRecord],
    limit: usize,
) -> Vec<WorkflowPrior> {
    if limit == 0 {
        return Vec::new();
    }

    topic_scores
        .iter()
        .take(limit)
        .map(|score| {
            let neuron_score = activations
                .iter()
                .find(|activation| activation.topic_id == score.topic_id)
                .map(|activation| activation.final_score)
                .unwrap_or(score.score);
            let neuron = compressed_neurons
                .iter()
                .find(|neuron| neuron.topic_id == score.topic_id);
            let neuron_prior = neuron.and_then(|neuron| neuron.workflow_priors.first());
            let neuron_policy = neuron
                .map(|neuron| neuron.compression_policy_version.as_str())
                .filter(|policy| !policy.trim().is_empty())
                .unwrap_or("none");
            let ranked_workflow = rank_intuition_workflow_candidate(
                user_intent,
                score,
                neuron_prior,
                &default_intuition_workflow_registry(),
            );
            let base_score = neuron_prior
                .map(|prior| ((score.score + neuron_score + prior.score) / 3.0).clamp(0.0, 1.0))
                .unwrap_or_else(|| ((score.score + neuron_score) / 2.0).clamp(0.0, 1.0));
            let feedback_delta = compute_intuition_feedback_delta(
                intuition_feedback,
                Some(&score.topic_id),
                neuron.map(|neuron| &neuron.neuron_id),
                None,
                Some(&ranked_workflow.workflow_id),
            );

            WorkflowPrior {
                workflow_id: ranked_workflow.workflow_id,
                score: (base_score + ranked_workflow.rank_bonus + feedback_delta).clamp(0.0, 1.0),
                exists_in_registry: ranked_workflow.registry_binding.exists_in_registry,
                missing_capability: ranked_workflow.registry_binding.missing_capability,
                requires_confirmation: ranked_workflow.registry_binding.requires_confirmation,
                action_mode: ranked_workflow.registry_binding.action_mode,
                source_topic_ids: vec![score.topic_id.clone()],
                source_neuron_ids: neuron
                    .map(|neuron| vec![neuron.neuron_id.clone()])
                    .unwrap_or_default(),
                reason: Some(format!(
                    "workflow registry ranked a prior for topic '{}' (routing {:.2}, neuron {:.2}, registry_rank {:.2}, prior {}, feedback {:+.2}, neuron_policy {}, {})",
                    score.topic_label.0,
                    score.score,
                    neuron_score,
                    ranked_workflow.registry_affinity,
                    neuron_prior
                        .map(|prior| format!("{:.2}", prior.score))
                        .unwrap_or_else(|| "none".into()),
                    feedback_delta,
                    neuron_policy,
                    ranked_workflow.registry_binding.reason,
                )),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkflowDescriptor {
    workflow_id: &'static str,
    label: &'static str,
    keywords: &'static [&'static str],
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
}

#[derive(Debug, Clone, PartialEq)]
struct RankedIntuitionWorkflowCandidate {
    workflow_id: String,
    registry_binding: IntuitionWorkflowRegistryBinding,
    registry_affinity: f32,
    rank_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntuitionWorkflowRegistryBinding {
    exists_in_registry: bool,
    missing_capability: Option<String>,
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
    reason: String,
}

fn default_intuition_workflow_registry() -> Vec<WorkflowDescriptor> {
    vec![
        WorkflowDescriptor {
            workflow_id: "workflow:memory-review",
            label: "Memory and provenance review",
            keywords: &[
                "memory",
                "recall",
                "context",
                "provenance",
                "adaptive",
                "remember",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:engineering-change",
            label: "Engineering implementation lane",
            keywords: &[
                "rust",
                "worker",
                "pipeline",
                "implementation",
                "code",
                "router",
                "neuron",
                "intelligence",
                "hepta",
                "lane",
                "agent",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:file-inspection",
            label: "File inspection and evidence gathering",
            keywords: &[
                "read",
                "inspect",
                "open",
                "show",
                "cat",
                "file",
                "architecture",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:file-change",
            label: "File mutation planning",
            keywords: &[
                "write",
                "create",
                "append",
                "overwrite",
                "save",
                "edit",
                "patch",
                "release notes",
            ],
            requires_confirmation: true,
            action_mode: IntuitionActionMode::SuggestOnly,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:tool-smoke-test",
            label: "Low-risk tool smoke test",
            keywords: &["echo", "repeat", "smoke test", "test tool"],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
    ]
}

fn rank_intuition_workflow_candidate(
    user_intent: &str,
    score: &TopicActivationScore,
    neuron_prior: Option<&WorkflowPrior>,
    registry: &[WorkflowDescriptor],
) -> RankedIntuitionWorkflowCandidate {
    if let Some(prior) =
        neuron_prior.filter(|prior| !prior.workflow_id.starts_with("workflow-bootstrap:"))
    {
        let binding = bind_intuition_workflow_to_registry(&prior.workflow_id, registry);
        return RankedIntuitionWorkflowCandidate {
            workflow_id: prior.workflow_id.clone(),
            registry_affinity: if binding.exists_in_registry { 1.0 } else { 0.0 },
            rank_bonus: if binding.exists_in_registry {
                0.10
            } else {
                0.0
            },
            registry_binding: binding,
        };
    }

    let mut candidates = registry
        .iter()
        .map(|descriptor| {
            let affinity = score_workflow_descriptor_for_intent(descriptor, user_intent, score);
            (descriptor, affinity)
        })
        .filter(|(_, affinity)| *affinity > 0.0)
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.workflow_id.cmp(right.0.workflow_id))
    });

    if let Some((descriptor, affinity)) = candidates.first() {
        let binding = bind_intuition_workflow_to_registry(descriptor.workflow_id, registry);
        return RankedIntuitionWorkflowCandidate {
            workflow_id: descriptor.workflow_id.to_string(),
            registry_binding: binding,
            registry_affinity: *affinity,
            rank_bonus: (affinity * 0.10).min(0.10),
        };
    }

    let fallback_id = neuron_prior
        .map(|prior| prior.workflow_id.clone())
        .unwrap_or_else(|| format!("workflow-bootstrap:{}", score.topic_id.0));
    RankedIntuitionWorkflowCandidate {
        workflow_id: fallback_id.clone(),
        registry_binding: bind_intuition_workflow_to_registry(&fallback_id, registry),
        registry_affinity: 0.0,
        rank_bonus: 0.0,
    }
}

fn score_workflow_descriptor_for_intent(
    descriptor: &WorkflowDescriptor,
    user_intent: &str,
    score: &TopicActivationScore,
) -> f32 {
    let haystack = format!(
        "{} {} {}",
        user_intent.to_ascii_lowercase(),
        score.topic_label.0.to_ascii_lowercase(),
        score.matched_terms.join(" ").to_ascii_lowercase(),
    );
    let matched_count = descriptor
        .keywords
        .iter()
        .filter(|keyword| haystack.contains(**keyword))
        .count();
    if matched_count == 0 {
        0.0
    } else {
        ((matched_count as f32) * 0.18 + score.score * 0.20).min(1.0)
    }
}

fn bind_intuition_workflow_to_registry(
    workflow_id: &str,
    registry: &[WorkflowDescriptor],
) -> IntuitionWorkflowRegistryBinding {
    if let Some(descriptor) = registry
        .iter()
        .find(|descriptor| descriptor.workflow_id == workflow_id)
    {
        return IntuitionWorkflowRegistryBinding {
            exists_in_registry: true,
            missing_capability: None,
            requires_confirmation: descriptor.requires_confirmation,
            action_mode: descriptor.action_mode,
            reason: format!(
                "bound to workflow registry entry '{}' ({}, action={}, requires_confirmation={})",
                descriptor.workflow_id,
                descriptor.label,
                format_intuition_action_mode(descriptor.action_mode),
                descriptor.requires_confirmation,
            ),
        };
    }

    IntuitionWorkflowRegistryBinding {
        exists_in_registry: false,
        missing_capability: Some("workflow_registry_binding_pending".into()),
        requires_confirmation: true,
        action_mode: IntuitionActionMode::SuggestOnly,
        reason: format!(
            "no workflow registry entry matched '{}'; prior remains suggest-only",
            workflow_id,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_bootstrap_skill_decisions(
    user_intent: &str,
    topic_scores: &[TopicActivationScore],
    activations: &[NeuronActivation],
    compressed_neurons: &[HeptaNeuron],
    workflow_priors: &[WorkflowPrior],
    intuition_feedback: &[IntuitionFeedbackRecord],
    capabilities: &[IntuitionCapabilityView],
    policy_bindings: &[IntuitionPolicyBinding],
    limit: usize,
) -> Vec<SkillActivationDecision> {
    if limit == 0 {
        return Vec::new();
    }

    topic_scores
        .iter()
        .enumerate()
        .take(limit)
        .map(|(index, score)| {
            let matching_activation = activations
                .iter()
                .find(|activation| activation.topic_id == score.topic_id);
            let neuron_ids = matching_activation
                .map(|activation| vec![activation.neuron_id.clone()])
                .unwrap_or_default();
            let workflow_id = workflow_priors
                .iter()
                .find(|prior| prior.source_topic_ids.contains(&score.topic_id))
                .or_else(|| workflow_priors.get(index))
                .map(|prior| prior.workflow_id.clone());
            let neuron = compressed_neurons
                .iter()
                .find(|neuron| neuron.topic_id == score.topic_id);
            let skill_prior = neuron.and_then(|neuron| neuron.skill_priors.first());
            let neuron_policy = neuron
                .map(|neuron| neuron.compression_policy_version.as_str())
                .filter(|policy| !policy.trim().is_empty())
                .unwrap_or("none");
            let activation_score = matching_activation
                .map(|activation| ((score.score + activation.final_score) / 2.0).clamp(0.0, 1.0))
                .unwrap_or(score.score);
            let base_skill_score = skill_prior
                .map(|prior| ((activation_score + prior.score) / 2.0).clamp(0.0, 1.0))
                .unwrap_or(activation_score);
            let preferred_skill_id = skill_prior
                .map(|prior| prior.skill_id.clone())
                .unwrap_or_else(|| format!("skill-bootstrap:{}:followup", score.topic_id.0));
            let ranked_skill = rank_intuition_skill_candidate(
                &preferred_skill_id,
                user_intent,
                &score.topic_label,
                capabilities,
                policy_bindings,
            );
            let skill_id = ranked_skill.skill_id;
            let registry_binding = ranked_skill.registry_binding;
            let feedback_delta = compute_intuition_feedback_delta(
                intuition_feedback,
                Some(&score.topic_id),
                neuron.map(|neuron| &neuron.neuron_id),
                Some(&skill_id),
                workflow_id.as_deref(),
            );
            let skill_score = (base_skill_score + ranked_skill.rank_bonus + feedback_delta)
                .clamp(0.0, 1.0);

            SkillActivationDecision {
                skill_id,
                workflow_id,
                score: skill_score,
                exists_in_registry: registry_binding.exists_in_registry,
                missing_capability: registry_binding.missing_capability,
                risk_tier: registry_binding.risk_tier,
                requires_confirmation: registry_binding.requires_confirmation,
                action_mode: registry_binding.action_mode,
                source_topic_ids: vec![score.topic_id.clone()],
                source_neuron_ids: neuron_ids,
                reason: Some(format!(
                    "policy-aware intuition ranked a follow-up skill for topic '{}' (routing {:.2}, activation {:.2}, skill {:.2}, registry_rank {:.2}, policy_rank {:.2}, feedback {:+.2}{}, neuron_policy {}, {})",
                    score.topic_label.0,
                    score.score,
                    activation_score,
                    skill_score,
                    ranked_skill.registry_affinity,
                    ranked_skill.policy_affinity,
                    feedback_delta,
                    skill_prior
                        .map(|prior| format!(", compressed neuron prior {:.2}", prior.score))
                        .unwrap_or_default(),
                    neuron_policy,
                    registry_binding.reason,
                )),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntuitionSkillRegistryBinding {
    exists_in_registry: bool,
    missing_capability: Option<String>,
    risk_tier: Option<RiskTier>,
    policy_requirement: ApprovalRequirement,
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
    reason: String,
}

#[derive(Debug, Clone, PartialEq)]
struct RankedIntuitionSkillCandidate {
    skill_id: String,
    registry_binding: IntuitionSkillRegistryBinding,
    registry_affinity: f32,
    policy_affinity: f32,
    rank_bonus: f32,
}

fn rank_intuition_skill_candidate(
    preferred_skill_id: &str,
    user_intent: &str,
    topic_label: &TopicLabel,
    capabilities: &[IntuitionCapabilityView],
    policy_bindings: &[IntuitionPolicyBinding],
) -> RankedIntuitionSkillCandidate {
    if let Some(capability) =
        find_registered_capability_for_skill_id(preferred_skill_id, capabilities)
    {
        let policy = find_policy_binding_for_capability(&capability.name, policy_bindings);
        let registry_binding =
            bind_intuition_skill_to_runtime_registry(&capability.name, capabilities, policy);
        let policy_affinity = policy_requirement_affinity(registry_binding.policy_requirement());
        return RankedIntuitionSkillCandidate {
            skill_id: capability.name.clone(),
            registry_binding,
            registry_affinity: 1.0,
            policy_affinity,
            rank_bonus: (0.08 + policy_affinity * 0.04).min(0.12),
        };
    }

    let mut candidates = capabilities
        .iter()
        .filter_map(|capability| {
            let registry_affinity =
                score_registered_capability_for_intent(capability, user_intent, topic_label);
            (registry_affinity > 0.0).then(|| {
                let policy = find_policy_binding_for_capability(&capability.name, policy_bindings);
                let policy_affinity = policy
                    .map(|policy| policy_requirement_affinity(policy.requirement))
                    .unwrap_or_else(|| {
                        policy_requirement_affinity(capability.default_approval_requirement)
                    });
                let safety_affinity = safety_affinity_for_capability(capability);
                let total = registry_affinity + policy_affinity * 0.18 + safety_affinity * 0.12;
                (
                    capability,
                    policy,
                    registry_affinity,
                    policy_affinity,
                    total,
                )
            })
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .4
            .total_cmp(&left.4)
            .then_with(|| left.0.name.cmp(&right.0.name))
    });

    if let Some((capability, policy, registry_affinity, policy_affinity, _)) = candidates.first() {
        let registry_binding =
            bind_intuition_skill_to_runtime_registry(&capability.name, capabilities, *policy);
        return RankedIntuitionSkillCandidate {
            skill_id: capability.name.clone(),
            registry_binding,
            registry_affinity: *registry_affinity,
            policy_affinity: *policy_affinity,
            rank_bonus: (registry_affinity * 0.08 + policy_affinity * 0.04).min(0.12),
        };
    }

    RankedIntuitionSkillCandidate {
        skill_id: preferred_skill_id.to_string(),
        registry_binding: bind_intuition_skill_to_runtime_registry(
            preferred_skill_id,
            capabilities,
            None,
        ),
        registry_affinity: 0.0,
        policy_affinity: 0.0,
        rank_bonus: 0.0,
    }
}

fn bind_intuition_skill_to_runtime_registry(
    skill_id: &str,
    capabilities: &[IntuitionCapabilityView],
    policy_binding: Option<&IntuitionPolicyBinding>,
) -> IntuitionSkillRegistryBinding {
    if let Some(capability) = find_registered_capability_for_skill_id(skill_id, capabilities) {
        let requirement = policy_binding
            .map(|binding| binding.requirement)
            .unwrap_or(capability.default_approval_requirement);
        let requires_confirmation = requirement != ApprovalRequirement::None;
        let action_mode = if requires_confirmation {
            IntuitionActionMode::SuggestOnly
        } else if capability.execution_metadata.read_only
            && capability.execution_metadata.idempotent
        {
            IntuitionActionMode::Prepare
        } else {
            IntuitionActionMode::SuggestOnly
        };

        return IntuitionSkillRegistryBinding {
            exists_in_registry: true,
            missing_capability: None,
            risk_tier: Some(capability.risk_tier),
            policy_requirement: requirement,
            requires_confirmation,
            action_mode,
            reason: format!(
                "bound to runtime tool registry entry '{}' (risk={}, approval={}, policy_rule={}, policy_reason=\"{}\")",
                capability.name,
                format_skill_registry_risk_tier(capability.risk_tier),
                format_approval_requirement(requirement),
                policy_binding
                    .and_then(|binding| binding.matched_rule_id.as_deref())
                    .unwrap_or("default"),
                summarize_line(
                    policy_binding
                        .map(|binding| binding.reason.as_str())
                        .unwrap_or("tool default approval requirement"),
                    72,
                ),
            ),
        };
    }

    IntuitionSkillRegistryBinding {
        exists_in_registry: false,
        missing_capability: Some("bootstrap_skill_registry_binding_pending".into()),
        risk_tier: Some(RiskTier::Low),
        policy_requirement: ApprovalRequirement::Ask,
        requires_confirmation: true,
        action_mode: IntuitionActionMode::SuggestOnly,
        reason: format!(
            "no runtime registry entry matched skill '{}'; suggestion remains gated",
            skill_id,
        ),
    }
}

impl IntuitionSkillRegistryBinding {
    fn policy_requirement(&self) -> ApprovalRequirement {
        self.policy_requirement
    }
}

fn find_policy_binding_for_capability<'a>(
    capability_name: &str,
    policy_bindings: &'a [IntuitionPolicyBinding],
) -> Option<&'a IntuitionPolicyBinding> {
    policy_bindings
        .iter()
        .find(|binding| binding.capability_name == capability_name)
}

fn policy_requirement_affinity(requirement: ApprovalRequirement) -> f32 {
    match requirement {
        ApprovalRequirement::None => 1.0,
        ApprovalRequirement::Ask => 0.55,
        ApprovalRequirement::Deny => 0.05,
    }
}

fn safety_affinity_for_capability(capability: &IntuitionCapabilityView) -> f32 {
    let mut score = 0.0_f32;
    if capability.execution_metadata.read_only {
        score += 0.45;
    }
    if capability.execution_metadata.idempotent {
        score += 0.35;
    }
    if !capability.execution_metadata.destructive {
        score += 0.20;
    }
    score.min(1.0)
}

fn find_registered_capability_for_skill_id<'a>(
    skill_id: &str,
    capabilities: &'a [IntuitionCapabilityView],
) -> Option<&'a IntuitionCapabilityView> {
    let normalized = normalize_skill_tool_selector(skill_id);
    capabilities
        .iter()
        .find(|capability| capability.name == normalized)
}

fn normalize_skill_tool_selector(skill_id: &str) -> &str {
    skill_id
        .strip_prefix("tool:")
        .or_else(|| skill_id.strip_prefix("skill-tool:"))
        .or_else(|| skill_id.strip_prefix("runtime-tool:"))
        .unwrap_or(skill_id)
}

fn score_registered_capability_for_intent(
    capability: &IntuitionCapabilityView,
    user_intent: &str,
    topic_label: &TopicLabel,
) -> f32 {
    let intent_haystack = format!(
        "{} {}",
        user_intent.to_ascii_lowercase(),
        topic_label.0.to_ascii_lowercase(),
    );

    match capability.name.as_str() {
        "read_file"
            if contains_any(
                &intent_haystack,
                &["read", "inspect", "open", "show", "cat"],
            ) =>
        {
            if intent_haystack.contains("file") || intent_haystack.contains("path") {
                1.0
            } else {
                0.62
            }
        }
        "write_file"
            if contains_any(
                &intent_haystack,
                &[
                    "write",
                    "save",
                    "create",
                    "append",
                    "overwrite",
                    "edit",
                    "patch",
                ],
            ) =>
        {
            if intent_haystack.contains("file") || intent_haystack.contains("path") {
                1.0
            } else {
                0.66
            }
        }
        "echo"
            if contains_any(
                &intent_haystack,
                &["echo", "repeat", "smoke test", "test tool"],
            ) =>
        {
            0.82
        }
        _ => 0.0,
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn format_approval_requirement(requirement: ApprovalRequirement) -> &'static str {
    match requirement {
        ApprovalRequirement::None => "none",
        ApprovalRequirement::Ask => "ask",
        ApprovalRequirement::Deny => "deny",
    }
}

fn format_skill_registry_risk_tier(risk_tier: RiskTier) -> &'static str {
    match risk_tier {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn format_intuition_action_mode(mode: IntuitionActionMode) -> &'static str {
    match mode {
        IntuitionActionMode::SuggestOnly => "suggest_only",
        IntuitionActionMode::Prepare => "prepare",
        IntuitionActionMode::ExecuteAllowed => "execute_allowed",
    }
}

fn summarize_line(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests;
