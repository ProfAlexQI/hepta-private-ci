pub struct HeptaWindowsQualificationHost<B> {
    backend: B,
    phase: HeptaWindowsQualificationPhase,
    generation: u64,
    active_identity: Option<HeptaWindowsQualificationWindowIdentity>,
    last_identity: Option<HeptaWindowsQualificationWindowIdentity>,
    review_seal: Option<HeptaWindowsProductHostReviewSeal>,
    implementation_candidate_commit: Option<String>,
    implementation_candidate_tree: Option<String>,
    rollback_required: bool,
    binding_verified: bool,
    rollback_verified: bool,
}

impl<B: HeptaWindowsQualificationBackend> HeptaWindowsQualificationHost<B> {
    pub const fn new(backend: B) -> Self {
        Self {
            backend,
            phase: HeptaWindowsQualificationPhase::Dormant,
            generation: 0,
            active_identity: None,
            last_identity: None,
            review_seal: None,
            implementation_candidate_commit: None,
            implementation_candidate_tree: None,
            rollback_required: false,
            binding_verified: false,
            rollback_verified: false,
        }
    }

    pub const fn phase(&self) -> HeptaWindowsQualificationPhase {
        self.phase
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn active_identity(&self) -> Option<HeptaWindowsQualificationWindowIdentity> {
        self.active_identity
    }

    pub const fn requires_rollback(&self) -> bool {
        self.rollback_required
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn begin_qualification(
        &mut self,
        request: &HeptaWindowsQualificationRequest,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        self.validate_request(request)?;
        self.generation = self.generation.saturating_add(1);
        self.active_identity = Some(request.identity);
        self.last_identity = Some(request.identity);
        self.review_seal = Some(request.review_seal.clone());
        self.implementation_candidate_commit =
            Some(request.implementation_candidate_commit.clone());
        self.implementation_candidate_tree = Some(request.implementation_candidate_tree.clone());
        self.rollback_required = true;
        self.binding_verified = false;
        self.rollback_verified = false;

        let binding = match self.backend.bind_qualification_verified(request.identity) {
            Ok(binding) => binding,
            Err(_) => {
                return self.recover_failed_bind(
                    request.identity,
                    HeptaWindowsQualificationError::BackendBindFailed,
                );
            }
        };
        if !valid_binding(binding, request.identity) {
            return self.recover_failed_bind(
                request.identity,
                HeptaWindowsQualificationError::BackendBindingReceiptRejected,
            );
        }

        self.phase = HeptaWindowsQualificationPhase::QualificationBound;
        self.binding_verified = true;
        Ok(self.receipt(
            true,
            true,
            binding.root_mica_exact,
            binding.transient_acrylic_exact,
            false,
            false,
            request.approval.implementation_approved,
            request.approval.operator_accepted,
        ))
    }

    pub fn rollback_to_solid(
        &mut self,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        if self.phase == HeptaWindowsQualificationPhase::Shutdown {
            return Err(HeptaWindowsQualificationError::HostShutdown);
        }
        if !self.rollback_required {
            return Err(HeptaWindowsQualificationError::RollbackNotRequired);
        }
        let identity = self
            .active_identity
            .ok_or(HeptaWindowsQualificationError::RollbackRequired)?;
        self.generation = self.generation.saturating_add(1);
        let rollback = match self
            .backend
            .rollback_qualification_to_solid_verified(identity)
        {
            Ok(rollback) => rollback,
            Err(_) => {
                self.phase = HeptaWindowsQualificationPhase::RejectedUnsafe;
                return Err(HeptaWindowsQualificationError::RollbackFailed);
            }
        };
        if !valid_rollback(rollback, identity) {
            self.phase = HeptaWindowsQualificationPhase::RejectedUnsafe;
            return Err(HeptaWindowsQualificationError::RollbackReceiptRejected);
        }

        self.rollback_required = false;
        self.rollback_verified = true;
        self.active_identity = None;
        self.phase = if self.binding_verified {
            HeptaWindowsQualificationPhase::QualifiedUnbound
        } else {
            HeptaWindowsQualificationPhase::RejectedSafe
        };
        Ok(self.receipt(
            true,
            false,
            self.binding_verified,
            self.binding_verified,
            rollback.root_none_exact,
            rollback.transient_none_exact,
            true,
            true,
        ))
    }

    pub fn enforce_preferences(
        &mut self,
        preferences: HeptaWindowsQualificationPreferences,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        if !preferences.transparency_allowed || preferences.high_contrast {
            return self.rollback_to_solid();
        }
        if self.rollback_required {
            return Ok(self.receipt(
                true,
                true,
                self.binding_verified,
                self.binding_verified,
                false,
                false,
                true,
                true,
            ));
        }
        Err(HeptaWindowsQualificationError::RollbackNotRequired)
    }

    pub fn suspend(
        &mut self,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        if self.phase == HeptaWindowsQualificationPhase::Shutdown {
            return Err(HeptaWindowsQualificationError::HostShutdown);
        }
        if self.rollback_required {
            self.rollback_to_solid()?;
        }
        self.generation = self.generation.saturating_add(1);
        self.phase = HeptaWindowsQualificationPhase::Suspended;
        Ok(self.safe_unbound_receipt(HeptaWindowsQualificationPhase::Suspended))
    }

    pub fn shutdown(
        &mut self,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        if self.phase == HeptaWindowsQualificationPhase::Shutdown {
            return Ok(self.safe_unbound_receipt(HeptaWindowsQualificationPhase::Shutdown));
        }
        if self.rollback_required {
            self.rollback_to_solid()?;
        }
        self.generation = self.generation.saturating_add(1);
        self.phase = HeptaWindowsQualificationPhase::Shutdown;
        Ok(self.safe_unbound_receipt(HeptaWindowsQualificationPhase::Shutdown))
    }

    pub fn qualification_evidence(
        &self,
    ) -> Result<HeptaWindowsQualificationEvidence, HeptaWindowsQualificationError> {
        if self.phase != HeptaWindowsQualificationPhase::QualifiedUnbound
            || !self.binding_verified
            || !self.rollback_verified
            || self.rollback_required
            || self.active_identity.is_some()
        {
            return Err(HeptaWindowsQualificationError::QualificationEvidenceUnavailable);
        }
        let identity = self
            .last_identity
            .ok_or(HeptaWindowsQualificationError::QualificationEvidenceUnavailable)?;
        let seal = self
            .review_seal
            .as_ref()
            .ok_or(HeptaWindowsQualificationError::QualificationEvidenceUnavailable)?;
        Ok(HeptaWindowsQualificationEvidence {
            implementation_candidate_commit: self
                .implementation_candidate_commit
                .clone()
                .ok_or(HeptaWindowsQualificationError::QualificationEvidenceUnavailable)?,
            implementation_candidate_tree: self
                .implementation_candidate_tree
                .clone()
                .ok_or(HeptaWindowsQualificationError::QualificationEvidenceUnavailable)?,
            review_evidence_candidate_commit: seal.evidence_candidate_commit.clone(),
            review_evidence_candidate_tree: seal.evidence_candidate_tree.clone(),
            review_binding_digest: seal.binding_digest.clone(),
            identity,
            root_mica_exact: true,
            transient_acrylic_exact: true,
            root_none_exact: true,
            transient_none_exact: true,
            qualified_unbound: true,
            product_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            promotion: false,
            release: false,
        })
    }

    fn validate_request(
        &self,
        request: &HeptaWindowsQualificationRequest,
    ) -> Result<(), HeptaWindowsQualificationError> {
        if !hepta_windows_qualification_feature_compiled() {
            return Err(HeptaWindowsQualificationError::FeatureNotCompiled);
        }
        if !request.approval.feature_flag_enabled {
            return Err(HeptaWindowsQualificationError::FeatureDisabled);
        }
        if self.phase == HeptaWindowsQualificationPhase::Shutdown {
            return Err(HeptaWindowsQualificationError::HostShutdown);
        }
        if self.rollback_required || self.active_identity.is_some() {
            return Err(HeptaWindowsQualificationError::RollbackRequired);
        }
        if !request.review.accepted
            || request.review.status
                != HeptaWindowsProductHostReviewStatus::EligibleForImplementationReview
            || !request.review.remains_review_only()
            || !request.review.grants_no_authority()
            || request.review.plan.feature_flag_default_enabled
            || request.review.plan.automatic_binding_allowed
            || !request.review.plan.operator_acceptance_required
            || !request.review.plan.device_validation_required
            || !request.review.plan.rollback_to_solid_required
            || !request.review.plan.high_contrast_recheck_required
            || !request.review.plan.transparency_preference_recheck_required
            || !request.review.plan.suspend_shutdown_unbind_required
        {
            return Err(HeptaWindowsQualificationError::ReviewEnvelopeRejected);
        }
        if !request.review_seal.is_valid() {
            return Err(HeptaWindowsQualificationError::ReviewSealInvalid);
        }
        if !git_object_id(&request.implementation_candidate_commit)
            || !git_object_id(&request.implementation_candidate_tree)
            || !git_object_id(&request.approval.implementation_candidate_commit)
            || !git_object_id(&request.approval.implementation_candidate_tree)
        {
            return Err(HeptaWindowsQualificationError::ImplementationCandidateInvalid);
        }
        if request.approval.implementation_candidate_commit
            != request.implementation_candidate_commit
            || request.approval.implementation_candidate_tree
                != request.implementation_candidate_tree
        {
            return Err(HeptaWindowsQualificationError::ImplementationCandidateMismatch);
        }
        if request.approval.review_binding_digest != request.review_seal.binding_digest
            || !sha256_hex(&request.approval.review_binding_digest)
        {
            return Err(HeptaWindowsQualificationError::ReviewDigestMismatch);
        }
        if !request.approval.implementation_approved {
            return Err(HeptaWindowsQualificationError::ImplementationApprovalMissing);
        }
        if !request.approval.operator_accepted {
            return Err(HeptaWindowsQualificationError::OperatorAcceptanceMissing);
        }
        if !request.approval.authority.grants_none() {
            return Err(HeptaWindowsQualificationError::AuthorityEscape);
        }
        if !request.identity.is_valid() {
            return Err(HeptaWindowsQualificationError::WindowIdentityInvalid);
        }
        if !request.preferences.transparency_allowed {
            return Err(HeptaWindowsQualificationError::TransparencyDisabled);
        }
        if request.preferences.high_contrast {
            return Err(HeptaWindowsQualificationError::HighContrast);
        }
        Ok(())
    }

    fn recover_failed_bind(
        &mut self,
        identity: HeptaWindowsQualificationWindowIdentity,
        original_error: HeptaWindowsQualificationError,
    ) -> Result<HeptaWindowsQualificationRuntimeReceipt, HeptaWindowsQualificationError> {
        match self
            .backend
            .rollback_qualification_to_solid_verified(identity)
        {
            Ok(rollback) if valid_rollback(rollback, identity) => {
                self.rollback_required = false;
                self.active_identity = None;
                self.rollback_verified = true;
                self.phase = HeptaWindowsQualificationPhase::RejectedSafe;
                Err(original_error)
            }
            Ok(_) => {
                self.phase = HeptaWindowsQualificationPhase::RejectedUnsafe;
                Err(HeptaWindowsQualificationError::RollbackReceiptRejected)
            }
            Err(_) => {
                self.phase = HeptaWindowsQualificationPhase::RejectedUnsafe;
                Err(HeptaWindowsQualificationError::RollbackFailed)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn receipt(
        &self,
        accepted: bool,
        material_active: bool,
        root_mica_exact: bool,
        transient_acrylic_exact: bool,
        root_none_exact: bool,
        transient_none_exact: bool,
        implementation_approved: bool,
        operator_accepted: bool,
    ) -> HeptaWindowsQualificationRuntimeReceipt {
        HeptaWindowsQualificationRuntimeReceipt {
            phase: self.phase,
            generation: self.generation,
            accepted,
            qualification_material_active: material_active,
            root_mica_exact,
            transient_acrylic_exact,
            root_none_exact,
            transient_none_exact,
            rollback_required: self.rollback_required,
            qualified_unbound: self.phase == HeptaWindowsQualificationPhase::QualifiedUnbound,
            implementation_approved,
            operator_acceptance_verified: operator_accepted,
            product_host_may_bind: false,
            product_bound: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            native_product_runtime: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            promotion: false,
            release: false,
        }
    }

    fn safe_unbound_receipt(
        &self,
        phase: HeptaWindowsQualificationPhase,
    ) -> HeptaWindowsQualificationRuntimeReceipt {
        debug_assert!(!self.rollback_required);
        debug_assert!(self.active_identity.is_none());
        HeptaWindowsQualificationRuntimeReceipt {
            phase,
            generation: self.generation,
            accepted: true,
            qualification_material_active: false,
            root_mica_exact: self.binding_verified,
            transient_acrylic_exact: self.binding_verified,
            root_none_exact: self.rollback_verified,
            transient_none_exact: self.rollback_verified,
            rollback_required: false,
            qualified_unbound: self.binding_verified && self.rollback_verified,
            implementation_approved: false,
            operator_acceptance_verified: false,
            product_host_may_bind: false,
            product_bound: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            native_product_runtime: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            promotion: false,
            release: false,
        }
    }
}

fn valid_binding(
    receipt: HeptaWindowsQualificationBindingReceipt,
    expected: HeptaWindowsQualificationWindowIdentity,
) -> bool {
    receipt.identity == expected
        && receipt.root_mica_exact
        && receipt.transient_acrylic_exact
        && receipt.authority.grants_none()
}

fn valid_rollback(
    receipt: HeptaWindowsQualificationRollbackReceipt,
    expected: HeptaWindowsQualificationWindowIdentity,
) -> bool {
    receipt.identity == expected
        && receipt.root_none_exact
        && receipt.transient_none_exact
        && receipt.rollback_verified
        && receipt.authority.grants_none()
}
