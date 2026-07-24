use hepta_contracts::CandidateId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::CapabilityRequest;
use hepta_contracts::CapabilityRequestId;
use hepta_contracts::ContentHash;
use hepta_contracts::ContractError;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::JointCandidate;
use hepta_contracts::PrincipalId;
use hepta_contracts::RevisionStamp;
use sha2::Digest;
use sha2::Sha256;

/// Exact capability proposal inputs owned by the intelligence boundary.
///
/// Runtime freezes the context and capability host supplies the exact manifest;
/// intelligence binds those inputs into one joint action/metacontrol candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCandidateProposalInput {
    pub context: FrozenTurnContext,
    pub capability: CapabilityManifestRef,
    pub requester: PrincipalId,
    pub payload_hash: ContentHash,
    pub metacontrol_hash: ContentHash,
    pub contributors: Vec<PrincipalId>,
}

/// Builds one exact tool candidate without performing policy or execution.
///
/// Identifiers and envelope hashes are derived from the complete supplied
/// bindings. Kernel admission still independently computes its own safety
/// binding and never trusts the candidate digest as authorization.
pub fn propose_tool_candidate(
    input: ToolCandidateProposalInput,
) -> Result<JointCandidate, ContractError> {
    let ToolCandidateProposalInput {
        context,
        capability,
        requester,
        payload_hash,
        metacontrol_hash,
        mut contributors,
    } = input;

    if !contributors
        .iter()
        .any(|contributor| contributor == &requester)
    {
        contributors.push(requester.clone());
    }

    let mut request_parts = capability_parts(&capability);
    request_parts.extend(context_parts(&context));
    request_parts.push(requester.as_str().to_string());
    request_parts.push(payload_hash.as_str().to_string());
    let request_hash = hash_parts("hepta.intelligence.capability-request.v1", &request_parts);
    let request = CapabilityRequest::try_new(
        CapabilityRequestId::new(format!("request:{}", request_hash.as_str())),
        request_hash,
        capability,
        requester,
        context.clone(),
        payload_hash.clone(),
    )?;

    let action_hash = hash_parts(
        "hepta.intelligence.tool-action.v1",
        &[
            request.capability().id().as_str().to_string(),
            request.capability().manifest_hash().as_str().to_string(),
            payload_hash.as_str().to_string(),
        ],
    );
    let request_ref = request.reference();
    let mut candidate_parts = context_parts(&context);
    candidate_parts.extend([
        action_hash.as_str().to_string(),
        metacontrol_hash.as_str().to_string(),
        payload_hash.as_str().to_string(),
        request_ref.request_hash().as_str().to_string(),
    ]);
    candidate_parts.extend(
        contributors
            .iter()
            .map(|contributor| contributor.as_str().to_string()),
    );
    let candidate_hash = hash_parts("hepta.intelligence.joint-candidate.v1", &candidate_parts);

    JointCandidate::try_new(
        CandidateId::new(format!("candidate:{}", candidate_hash.as_str())),
        context.observation().revision(),
        candidate_hash,
        context,
        action_hash,
        metacontrol_hash,
        payload_hash,
        contributors,
        vec![request_ref],
    )
}

fn capability_parts(capability: &CapabilityManifestRef) -> Vec<String> {
    vec![
        capability.id().as_str().to_string(),
        capability.manifest_hash().as_str().to_string(),
        capability.catalog().content_hash().as_str().to_string(),
        capability.revision().get().to_string(),
        capability.catalog().revision().get().to_string(),
    ]
}

fn context_parts(context: &FrozenTurnContext) -> Vec<String> {
    let observation = context.observation();
    let mut parts = vec![
        observation.id().as_str().to_string(),
        observation.content_hash().as_str().to_string(),
        observation.revision().get().to_string(),
    ];
    push_stamp_parts(&mut parts, context.state());
    push_stamp_parts(&mut parts, context.policy());
    push_stamp_parts(&mut parts, context.capability_catalog());
    push_stamp_parts(&mut parts, context.preference());
    parts
}

fn push_stamp_parts(parts: &mut Vec<String>, stamp: &RevisionStamp) {
    parts.push(stamp.revision().get().to_string());
    parts.push(stamp.content_hash().as_str().to_string());
}

fn hash_parts(domain: &str, parts: &[String]) -> ContentHash {
    let mut digest = Sha256::new();
    update_part(&mut digest, domain);
    for part in parts {
        update_part(&mut digest, part);
    }
    ContentHash::new(format!("sha256:{:x}", digest.finalize()))
}

fn update_part(digest: &mut Sha256, part: &str) {
    digest.update((part.len() as u64).to_be_bytes());
    digest.update(part.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use hepta_contracts::CapabilityId;
    use hepta_contracts::ObservationId;
    use hepta_contracts::ObservationRef;
    use hepta_contracts::Revision;

    #[test]
    fn proposal_binds_payload_context_and_metacontrol() -> Result<(), ContractError> {
        let base = proposal_input("payload-a", "meta-a");
        let candidate = propose_tool_candidate(base.clone())?;
        let payload_drift = propose_tool_candidate(ToolCandidateProposalInput {
            payload_hash: ContentHash::new("payload-b"),
            ..base.clone()
        })?;
        let context_drift = propose_tool_candidate(ToolCandidateProposalInput {
            context: frozen_context("policy-b"),
            ..base.clone()
        })?;
        let metacontrol_drift = propose_tool_candidate(ToolCandidateProposalInput {
            metacontrol_hash: ContentHash::new("meta-b"),
            ..base
        })?;

        assert_ne!(candidate.content_hash(), payload_drift.content_hash());
        assert_ne!(candidate.content_hash(), context_drift.content_hash());
        assert_ne!(candidate.content_hash(), metacontrol_drift.content_hash());
        assert_eq!(candidate.capability_requests().len(), 1);
        assert_eq!(
            candidate.capability_requests()[0].payload_hash(),
            candidate.payload_set_hash()
        );
        Ok(())
    }

    #[test]
    fn proposal_defaults_contributor_to_requester() -> Result<(), ContractError> {
        let candidate = propose_tool_candidate(proposal_input("payload-a", "meta-a"))?;
        assert_eq!(candidate.contributors(), &[PrincipalId::new("model:demo")]);
        Ok(())
    }

    #[test]
    fn proposal_always_includes_the_requester() -> Result<(), ContractError> {
        let mut input = proposal_input("payload-a", "meta-a");
        input.contributors = vec![PrincipalId::new("agent:planner")];

        let candidate = propose_tool_candidate(input)?;

        assert_eq!(
            candidate.contributors(),
            &[
                PrincipalId::new("agent:planner"),
                PrincipalId::new("model:demo")
            ]
        );
        Ok(())
    }

    #[test]
    fn hash_framing_is_stable_across_target_pointer_widths() {
        assert_eq!(
            hash_parts(
                "hepta.test.hash-parts.v1",
                &["a".to_string(), "bc".to_string()]
            )
            .as_str(),
            "sha256:445c344458bd23633e477f37034854388d1b6dc4457aa67216350f0a09b93c63"
        );
    }

    fn proposal_input(payload: &str, metacontrol: &str) -> ToolCandidateProposalInput {
        let context = frozen_context("policy-a");
        ToolCandidateProposalInput {
            capability: CapabilityManifestRef::new(
                CapabilityId::new("tool:read_file"),
                Revision::new(1),
                ContentHash::new("manifest-a"),
                context.capability_catalog().clone(),
            ),
            context,
            requester: PrincipalId::new("model:demo"),
            payload_hash: ContentHash::new(payload),
            metacontrol_hash: ContentHash::new(metacontrol),
            contributors: Vec::new(),
        }
    }

    fn frozen_context(policy_hash: &str) -> FrozenTurnContext {
        FrozenTurnContext::new(
            ObservationRef::new(
                ObservationId::new("observation-a"),
                Revision::new(7),
                ContentHash::new("observation-a"),
            ),
            RevisionStamp::new(Revision::new(3), ContentHash::new("state-a")),
            RevisionStamp::new(Revision::new(4), ContentHash::new(policy_hash)),
            RevisionStamp::new(Revision::new(5), ContentHash::new("catalog-a")),
            RevisionStamp::new(Revision::new(6), ContentHash::new("preference-a")),
        )
    }
}
