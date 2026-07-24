use super::FramedHash;
use hepta_contracts::CapabilityDescriptor;
use hepta_contracts::ContentHash;
use hepta_contracts::JointCandidate;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_core::ApprovalRequirement;
use hepta_core::PolicyDecision;
use hepta_core::PolicyRule;
use hepta_core::RiskTier;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

const POLICY_SNAPSHOT_DOMAIN: &str = "hepta.runtime.policy-snapshot.v1";
const POLICY_DECISION_DOMAIN: &str = "hepta.runtime.tool-metacontrol.v1";
const POLICY_EVIDENCE_DOMAIN: &str = "hepta.kernel.safety-gate.policy-evidence.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PolicyEvidenceFailure {
    SnapshotMismatch,
    Malformed,
    EvaluationContextMismatch,
    DecisionMismatch,
}

/// Complete policy snapshot and evaluation facts independently replayed by the kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaKernelPolicyEvidence {
    snapshot: RevisionStamp,
    session_id: String,
    provider_name: String,
    tool_name: String,
    risk_tier: RiskTier,
    default_rules: Vec<PolicyRule>,
    custom_rules: Vec<PolicyRule>,
    presented_decision: PolicyDecision,
    decision_hash: Option<ContentHash>,
}

impl HeptaKernelPolicyEvidence {
    /// Freezes all policy inputs needed to replay one exact tool decision.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        snapshot: RevisionStamp,
        session_id: impl Into<String>,
        provider_name: impl Into<String>,
        tool_name: impl Into<String>,
        risk_tier: RiskTier,
        default_rules: Vec<PolicyRule>,
        custom_rules: Vec<PolicyRule>,
        presented_decision: PolicyDecision,
    ) -> Self {
        let decision_hash = canonical_hash(
            POLICY_DECISION_DOMAIN,
            serde_json::to_value(&presented_decision).ok(),
        );
        Self {
            snapshot,
            session_id: session_id.into(),
            provider_name: provider_name.into(),
            tool_name: tool_name.into(),
            risk_tier,
            default_rules,
            custom_rules,
            presented_decision,
            decision_hash,
        }
    }

    /// Computes the exact policy stamp expected for the supplied rule snapshot.
    pub fn snapshot_for_rules(
        revision: Revision,
        default_rules: &[PolicyRule],
        custom_rules: &[PolicyRule],
    ) -> Option<RevisionStamp> {
        let hash = policy_snapshot_hash(default_rules, custom_rules)?;
        Some(RevisionStamp::new(revision, hash))
    }

    /// Returns the full policy snapshot stamped into the candidate context.
    pub fn snapshot(&self) -> &RevisionStamp {
        &self.snapshot
    }

    /// Returns the independently hashable presented policy decision.
    pub fn decision_hash(&self) -> Option<&ContentHash> {
        self.decision_hash.as_ref()
    }

    /// Returns the complete caller-presented decision.
    pub fn presented_decision(&self) -> &PolicyDecision {
        &self.presented_decision
    }

    pub(super) fn assess(
        &self,
        candidate: &JointCandidate,
        capability: &CapabilityDescriptor,
    ) -> Result<ApprovalRequirement, PolicyEvidenceFailure> {
        if policy_snapshot_hash(&self.default_rules, &self.custom_rules).as_ref()
            != Some(self.snapshot.content_hash())
            || self.snapshot != *candidate.context().policy()
        {
            return Err(PolicyEvidenceFailure::SnapshotMismatch);
        }
        if self.is_malformed() {
            return Err(PolicyEvidenceFailure::Malformed);
        }
        if !self.context_matches(candidate, capability) {
            return Err(PolicyEvidenceFailure::EvaluationContextMismatch);
        }
        let expected = self.evaluate().ok_or(PolicyEvidenceFailure::Malformed)?;
        if expected != self.presented_decision
            || self.decision_hash() != Some(candidate.metacontrol_hash())
        {
            return Err(PolicyEvidenceFailure::DecisionMismatch);
        }
        Ok(expected.requirement)
    }

    pub(super) fn evidence_hash(&self) -> ContentHash {
        let mut hash = FramedHash::new(POLICY_EVIDENCE_DOMAIN);
        hash.number("snapshot.revision", self.snapshot.revision().get());
        hash.text(
            "snapshot.content_hash",
            self.snapshot.content_hash().as_str(),
        );
        hash.text("evaluation.session_id", &self.session_id);
        hash.text("evaluation.provider_name", &self.provider_name);
        hash.text("evaluation.tool_name", &self.tool_name);
        hash.text("evaluation.risk_tier", risk_name(self.risk_tier));
        hash.text(
            "presented.decision_hash",
            self.decision_hash
                .as_ref()
                .map_or("encoding-error", ContentHash::as_str),
        );
        hash.finish()
    }

    fn is_malformed(&self) -> bool {
        self.session_id.trim().is_empty()
            || self.provider_name.trim().is_empty()
            || self.tool_name.trim().is_empty()
            || self.presented_decision.reason.trim().is_empty()
            || self
                .presented_decision
                .matched_rule_id
                .as_ref()
                .is_some_and(|rule| rule.trim().is_empty())
            || self.decision_hash.is_none()
            || self
                .default_rules
                .iter()
                .chain(&self.custom_rules)
                .any(rule_malformed)
    }

    fn context_matches(
        &self,
        candidate: &JointCandidate,
        capability: &CapabilityDescriptor,
    ) -> bool {
        candidate.context().observation().id().as_str()
            == format!("tool-observation:{}:{}", self.session_id, self.tool_name)
            && capability.id().as_str() == format!("tool:{}", self.tool_name)
            && candidate.capability_requests().iter().all(|request| {
                request
                    .requester()
                    .as_str()
                    .starts_with(&format!("model:{}/", self.provider_name))
            })
    }

    fn evaluate(&self) -> Option<PolicyDecision> {
        let mut best: Option<(&PolicyRule, RuleSortKey)> = None;
        for (index, rule) in self.default_rules.iter().enumerate() {
            consider_rule(&mut best, rule, false, index, self);
        }
        for (index, rule) in self.custom_rules.iter().enumerate() {
            consider_rule(&mut best, rule, true, index, self);
        }
        Some(match best {
            Some((rule, _)) => PolicyDecision {
                requirement: rule.requirement,
                reason: decision_reason(rule, &self.tool_name),
                matched_rule_id: Some(rule.id.clone()),
            },
            None => PolicyDecision {
                requirement: requirement_for_risk(self.risk_tier),
                reason: format!(
                    "fallback risk policy for {} ({})",
                    self.tool_name,
                    risk_name(self.risk_tier)
                ),
                matched_rule_id: None,
            },
        })
    }
}

type RuleSortKey = (u8, usize, u8, u8, u8, u8, usize);

fn consider_rule<'a>(
    best: &mut Option<(&'a PolicyRule, RuleSortKey)>,
    rule: &'a PolicyRule,
    custom: bool,
    index: usize,
    evidence: &HeptaKernelPolicyEvidence,
) {
    if !matches_rule(rule, evidence) {
        return;
    }
    let score = rule_sort_key(rule, custom, index);
    if best.as_ref().is_none_or(|(_, current)| score > *current) {
        *best = Some((rule, score));
    }
}

fn matches_rule(rule: &PolicyRule, evidence: &HeptaKernelPolicyEvidence) -> bool {
    rule.session_id
        .as_deref()
        .is_none_or(|value| value == evidence.session_id)
        && rule
            .provider_name
            .as_deref()
            .is_none_or(|value| value == evidence.provider_name)
        && rule
            .tool_name
            .as_deref()
            .is_none_or(|value| value == evidence.tool_name)
        && rule
            .risk_tier
            .is_none_or(|value| value == evidence.risk_tier)
}

fn rule_sort_key(rule: &PolicyRule, custom: bool, index: usize) -> RuleSortKey {
    let selectors = [
        rule.session_id.is_some(),
        rule.provider_name.is_some(),
        rule.tool_name.is_some(),
        rule.risk_tier.is_some(),
    ];
    (
        u8::from(custom),
        selectors.into_iter().filter(|selected| *selected).count(),
        u8::from(rule.session_id.is_some()),
        u8::from(rule.tool_name.is_some()),
        u8::from(rule.provider_name.is_some()),
        u8::from(rule.risk_tier.is_some()),
        index,
    )
}

fn rule_malformed(rule: &PolicyRule) -> bool {
    rule.id.trim().is_empty()
        || rule.reason.trim().is_empty()
        || [
            rule.session_id.as_deref(),
            rule.provider_name.as_deref(),
            rule.tool_name.as_deref(),
        ]
        .into_iter()
        .flatten()
        .any(|selector| selector.trim().is_empty())
}

fn decision_reason(rule: &PolicyRule, tool_name: &str) -> String {
    match rule.id.as_str() {
        "default-risk-low" => format!("{tool_name} is low risk"),
        "default-risk-medium" => {
            format!("{tool_name} is medium risk and requires explicit approval")
        }
        "default-risk-high" => format!("{tool_name} is high risk and denied by default"),
        "default-tool-exec" => "exec requires explicit approval".into(),
        _ => rule.reason.clone(),
    }
}

const fn requirement_for_risk(risk: RiskTier) -> ApprovalRequirement {
    match risk {
        RiskTier::Low => ApprovalRequirement::None,
        RiskTier::Medium => ApprovalRequirement::Ask,
        RiskTier::High => ApprovalRequirement::Deny,
    }
}

const fn risk_name(risk: RiskTier) -> &'static str {
    match risk {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn policy_snapshot_hash(
    default_rules: &[PolicyRule],
    custom_rules: &[PolicyRule],
) -> Option<ContentHash> {
    canonical_hash(
        POLICY_SNAPSHOT_DOMAIN,
        Some(json!({
            "custom_rules": custom_rules,
            "default_rules": default_rules,
        })),
    )
}

fn canonical_hash(domain: &str, value: Option<Value>) -> Option<ContentHash> {
    let canonical = serde_json::to_string(&canonical_value(value?)).ok()?;
    let mut hash = FramedHash::new(domain);
    hash.text("canonical_json", &canonical);
    Some(hash.finish())
}

fn canonical_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_value).collect()),
        Value::Object(values) => {
            let mut entries = values.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            let mut canonical = Map::new();
            for (key, value) in entries {
                canonical.insert(key, canonical_value(value));
            }
            Value::Object(canonical)
        }
        scalar => scalar,
    }
}
