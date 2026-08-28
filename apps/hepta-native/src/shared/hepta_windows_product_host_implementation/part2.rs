pub struct HeptaWindowsProductMaterialHost<B> {
    backend: B,
    phase: HeptaWindowsProductHostPhase,
    generation: u64,
    identity: Option<HeptaWindowsProductHostWindowIdentity>,
}

impl<B: HeptaWindowsProductMaterialBackend> HeptaWindowsProductMaterialHost<B> {
    pub const fn new(backend: B) -> Self {
        Self {
            backend,
            phase: HeptaWindowsProductHostPhase::Dormant,
            generation: 0,
            identity: None,
        }
    }

    pub const fn phase(&self) -> HeptaWindowsProductHostPhase {
        self.phase
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn identity(&self) -> Option<HeptaWindowsProductHostWindowIdentity> {
        self.identity
    }

    pub fn activate(
        &mut self,
        request: &HeptaWindowsProductHostActivationRequest,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostError> {
        self.validate_activation_request(request)?;
        self.generation = self.generation.saturating_add(1);

        let binding = match self.backend.bind_verified(request.identity) {
            Ok(binding) => binding,
            Err(_) => {
                let _ = self.backend.rollback_to_solid_verified(request.identity);
                self.phase = HeptaWindowsProductHostPhase::Rejected;
                self.identity = None;
                return Err(HeptaWindowsProductHostError::BackendBindFailed);
            }
        };

        if !valid_binding(binding, request.identity) {
            let _ = self.backend.rollback_to_solid_verified(request.identity);
            self.phase = HeptaWindowsProductHostPhase::Rejected;
            self.identity = None;
            return Err(HeptaWindowsProductHostError::BackendBindingReceiptRejected);
        }

        self.phase = HeptaWindowsProductHostPhase::Bound;
        self.identity = Some(request.identity);
        Ok(self.runtime_receipt(true, request))
    }

    pub fn rollback_to_solid(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostError> {
        if self.phase == HeptaWindowsProductHostPhase::Shutdown {
            return Err(HeptaWindowsProductHostError::HostShutdown);
        }
        let identity = self
            .identity
            .ok_or(HeptaWindowsProductHostError::RollbackFailed)?;
        self.generation = self.generation.saturating_add(1);
        let rollback = match self.backend.rollback_to_solid_verified(identity) {
            Ok(rollback) => rollback,
            Err(_) => {
                self.phase = HeptaWindowsProductHostPhase::Rejected;
                self.identity = None;
                return Err(HeptaWindowsProductHostError::RollbackFailed);
            }
        };
        if !valid_rollback(rollback, identity) {
            self.phase = HeptaWindowsProductHostPhase::Rejected;
            self.identity = None;
            return Err(HeptaWindowsProductHostError::RollbackReceiptRejected);
        }
        self.phase = HeptaWindowsProductHostPhase::SolidFallback;
        self.identity = None;
        Ok(self.unbound_receipt(HeptaWindowsProductHostPhase::SolidFallback))
    }

    pub fn suspend(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostError> {
        if self.phase == HeptaWindowsProductHostPhase::Shutdown {
            return Err(HeptaWindowsProductHostError::HostShutdown);
        }
        if self.phase == HeptaWindowsProductHostPhase::Bound {
            self.rollback_to_solid()?;
        }
        self.generation = self.generation.saturating_add(1);
        self.phase = HeptaWindowsProductHostPhase::Suspended;
        self.identity = None;
        Ok(self.unbound_receipt(HeptaWindowsProductHostPhase::Suspended))
    }

    pub fn shutdown(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostError> {
        if self.phase == HeptaWindowsProductHostPhase::Bound {
            self.rollback_to_solid()?;
        }
        self.generation = self.generation.saturating_add(1);
        self.phase = HeptaWindowsProductHostPhase::Shutdown;
        self.identity = None;
        Ok(self.unbound_receipt(HeptaWindowsProductHostPhase::Shutdown))
    }

    fn validate_activation_request(
        &self,
        request: &HeptaWindowsProductHostActivationRequest,
    ) -> Result<(), HeptaWindowsProductHostError> {
        if !hepta_windows_product_host_feature_compiled() {
            return Err(HeptaWindowsProductHostError::FeatureNotCompiled);
        }
        if !request.approval.feature_flag_enabled {
            return Err(HeptaWindowsProductHostError::FeatureDisabled);
        }
        if self.phase == HeptaWindowsProductHostPhase::Shutdown {
            return Err(HeptaWindowsProductHostError::HostShutdown);
        }
        if self.phase == HeptaWindowsProductHostPhase::Bound {
            return Err(HeptaWindowsProductHostError::AlreadyBound);
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
            return Err(HeptaWindowsProductHostError::ReviewEnvelopeRejected);
        }
        if !request.approval.implementation_approved {
            return Err(HeptaWindowsProductHostError::ImplementationApprovalMissing);
        }
        if !request.approval.operator_accepted {
            return Err(HeptaWindowsProductHostError::OperatorAcceptanceMissing);
        }
        if !request.approval.physical_device_validated {
            return Err(HeptaWindowsProductHostError::DeviceValidationMissing);
        }
        if !request.approval.rollback_drill_validated {
            return Err(HeptaWindowsProductHostError::RollbackDrillMissing);
        }
        if !git_object_id(&request.approval.candidate_commit)
            || !git_object_id(&request.approval.candidate_tree)
        {
            return Err(HeptaWindowsProductHostError::CandidateIdentityInvalid);
        }
        if !sha256_hex(&request.approval.review_binding_digest) {
            return Err(HeptaWindowsProductHostError::ReviewDigestInvalid);
        }
        if !request.approval.authority.grants_none() {
            return Err(HeptaWindowsProductHostError::AuthorityEscape);
        }
        if !request.identity.is_valid() {
            return Err(HeptaWindowsProductHostError::WindowIdentityInvalid);
        }
        if !request.preferences.transparency_allowed {
            return Err(HeptaWindowsProductHostError::TransparencyDisabled);
        }
        if request.preferences.high_contrast {
            return Err(HeptaWindowsProductHostError::HighContrast);
        }
        Ok(())
    }

    fn runtime_receipt(
        &self,
        accepted: bool,
        request: &HeptaWindowsProductHostActivationRequest,
    ) -> HeptaWindowsProductHostRuntimeReceipt {
        HeptaWindowsProductHostRuntimeReceipt {
            phase: self.phase,
            accepted,
            generation: self.generation,
            implementation_approved: request.approval.implementation_approved,
            operator_acceptance_verified: request.approval.operator_accepted,
            device_validation_verified: request.approval.physical_device_validated,
            rollback_drill_verified: request.approval.rollback_drill_validated,
            product_host_may_bind: accepted,
            product_bound: accepted,
            transient_system_material_bound: accepted,
            complete_profile_bound: accepted,
            system_material_bound: accepted,
            native_product_runtime: accepted,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            promotion: false,
            release: false,
        }
    }

    fn unbound_receipt(
        &self,
        phase: HeptaWindowsProductHostPhase,
    ) -> HeptaWindowsProductHostRuntimeReceipt {
        HeptaWindowsProductHostRuntimeReceipt {
            phase,
            accepted: true,
            generation: self.generation,
            implementation_approved: false,
            operator_acceptance_verified: false,
            device_validation_verified: false,
            rollback_drill_verified: false,
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
    binding: HeptaWindowsVerifiedMaterialBinding,
    expected: HeptaWindowsProductHostWindowIdentity,
) -> bool {
    binding.identity == expected
        && binding.root_mica_exact
        && binding.transient_acrylic_exact
        && binding.complete_profile_bound
        && binding.system_material_bound
        && binding.authority.grants_none()
}

fn valid_rollback(
    rollback: HeptaWindowsVerifiedMaterialRollback,
    expected: HeptaWindowsProductHostWindowIdentity,
) -> bool {
    rollback.identity == expected
        && rollback.root_none_exact
        && rollback.transient_none_exact
        && rollback.rollback_verified
        && rollback.authority.grants_none()
}

fn git_object_id(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
