fn accepted_review() -> HeptaWindowsProductHostReviewReceipt {
    HeptaWindowsProductHostReviewReceipt {
        status: HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview,
        accepted: true,
        eligible_for_implementation_review: true,
        plan: HeptaWindowsProductHostIntegrationPlan::default(),
        implementation_approved: false,
        product_host_may_bind: false,
        product_bound: false,
        transient_system_material_bound: false,
        complete_profile_bound: false,
        system_material_bound: false,
        native_product_runtime: false,
        device_validated: false,
        production_authority: false,
        effect_authority: false,
        live_adapter_authority: false,
        operator_acceptance: false,
        promotion: false,
        release: false,
    }
}

fn request(
    identity: HeptaWindowsQualificationWindowIdentity,
) -> Result<HeptaWindowsQualificationRequest, String> {
    let implementation_candidate_commit = required("HEPTA_CANDIDATE_COMMIT")?;
    let implementation_candidate_tree = required("HEPTA_CANDIDATE_TREE")?;
    let binding_digest = required("HEPTA_REVIEW_BINDING_DIGEST")?;
    Ok(HeptaWindowsQualificationRequest {
        review: accepted_review(),
        review_seal: HeptaWindowsProductHostReviewSeal {
            evidence_candidate_commit: required("HEPTA_REVIEW_CANDIDATE_COMMIT")?,
            evidence_candidate_tree: required("HEPTA_REVIEW_CANDIDATE_TREE")?,
            binding_digest: binding_digest.clone(),
        },
        implementation_candidate_commit: implementation_candidate_commit.clone(),
        implementation_candidate_tree: implementation_candidate_tree.clone(),
        approval: HeptaWindowsQualificationApproval {
            implementation_candidate_commit,
            implementation_candidate_tree,
            review_binding_digest: binding_digest,
            feature_flag_enabled: true,
            implementation_approved: true_env("HEPTA_IMPLEMENTATION_REVIEW_APPROVED")?,
            operator_accepted: true_env("HEPTA_OPERATOR_ACCEPTED")?,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        },
        identity,
        preferences: HeptaWindowsQualificationPreferences {
            transparency_allowed: true,
            high_contrast: false,
        },
    })
}
