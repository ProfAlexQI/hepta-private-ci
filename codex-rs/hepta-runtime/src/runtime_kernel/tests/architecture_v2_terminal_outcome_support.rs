struct AuthorizationFixture {
    authorization: HeptaKernelSafetyAuthorization,
    capability: hepta_contracts::CapabilityManifestRef,
}

fn authorization(label: &str, state_hash: &str) -> AuthorizationFixture {
    let policy_rules = vec![PolicyRule {
        id: "terminal-outcome-test".into(),
        session_id: None,
        provider_name: None,
        tool_name: None,
        risk_tier: None,
        requirement: ApprovalRequirement::None,
        reason: "terminal outcome fixture".into(),
    }];
    let policy_stamp = HeptaKernelPolicyEvidence::snapshot_for_rules(
        Revision::new(1),
        &policy_rules,
        &[],
    )
    .expect("fixture policy snapshot");
    let observation = ObservationSnapshot::new(
        ObservationId::new(format!("tool-observation:session-{label}:echo")),
        Revision::new(1),
        ContentHash::new(format!("sha256:observation-{label}")),
        PrincipalId::new("runtime:observer"),
        Vec::new(),
    );
    let context = FrozenTurnContext::new(
        observation.reference(),
        stamp(state_hash),
        policy_stamp,
        stamp("sha256:catalog"),
        stamp("sha256:preference"),
    );
    let descriptor = CapabilityDescriptor::new(
        CapabilityId::new("tool:echo"),
        Revision::new(1),
        ContentHash::new("sha256:manifest"),
        context.capability_catalog().clone(),
        PrincipalId::new("runtime:tool-host"),
        "tool.echo",
    );
    let capability = descriptor.reference();
    let payload_hash = ContentHash::new(format!("sha256:payload-{label}"));
    let policy = HeptaKernelPolicyEvidence::new(
        context.policy().clone(),
        format!("session-{label}"),
        "test-provider",
        "echo",
        RiskTier::Low,
        policy_rules,
        Vec::new(),
        PolicyDecision {
            requirement: ApprovalRequirement::None,
            reason: "terminal outcome fixture".into(),
            matched_rule_id: Some("terminal-outcome-test".into()),
        },
    );
    let evidence = HeptaKernelAdmissionEvidence::from_tool_policy(
        policy,
        descriptor,
        payload_hash.clone(),
    );
    let requester = PrincipalId::new("model:test-provider/demo");
    let request = CapabilityRequest::try_new(
        CapabilityRequestId::new(format!("request-{label}")),
        ContentHash::new(format!("sha256:request-{label}")),
        capability.clone(),
        requester.clone(),
        context.clone(),
        payload_hash.clone(),
    )
    .expect("request");
    let candidate = JointCandidate::try_new(
        CandidateId::new(format!("candidate-{label}")),
        Revision::new(1),
        ContentHash::new(format!("sha256:candidate-{label}")),
        context.clone(),
        ContentHash::new("sha256:action"),
        evidence
            .policy_decision_hash()
            .expect("fixture policy serializes")
            .clone(),
        payload_hash,
        vec![requester],
        vec![request.reference()],
    )
    .expect("candidate");
    let gate = HeptaKernelSafetyGate::new();
    let admission = gate.admit_candidate(
        AdmissionId::new(format!("admission-{label}")),
        Revision::new(1),
        &candidate,
        PrincipalId::new("kernel:safety-gate"),
        &evidence,
    );
    let authorization = gate
        .authorize_at_commit(
            AuthorizationId::new(format!("authorization-{label}")),
            Revision::new(1),
            &admission,
            admission.binding(),
            &candidate,
            candidate.payload_set_hash(),
            &context,
            PrincipalId::new("kernel:safety-gate"),
            ContentHash::new(format!("sha256:scope-{label}")),
        )
        .expect("authorization");
    AuthorizationFixture {
        authorization,
        capability,
    }
}
