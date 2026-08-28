#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hepta_windows_product_host_integration_review::{
        HeptaWindowsProductHostReviewEvidence, HeptaWindowsProvenanceLevel,
        HeptaWindowsReviewBindingBoundary, evaluate_windows_product_host_integration_review,
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum FakeMode {
        Success,
        BindError,
        InvalidBinding,
        RollbackError,
        InvalidRollback,
    }

    struct FakeBackend {
        mode: FakeMode,
        bind_calls: usize,
        rollback_calls: usize,
    }

    impl FakeBackend {
        fn new(mode: FakeMode) -> Self {
            Self {
                mode,
                bind_calls: 0,
                rollback_calls: 0,
            }
        }
    }

    impl HeptaWindowsProductMaterialBackend for FakeBackend {
        fn bind_verified(
            &mut self,
            identity: HeptaWindowsProductHostWindowIdentity,
        ) -> Result<HeptaWindowsVerifiedMaterialBinding, HeptaWindowsProductMaterialBackendError>
        {
            self.bind_calls += 1;
            if self.mode == FakeMode::BindError {
                return Err(HeptaWindowsProductMaterialBackendError::BindFailed);
            }
            Ok(HeptaWindowsVerifiedMaterialBinding {
                identity,
                root_mica_exact: self.mode != FakeMode::InvalidBinding,
                transient_acrylic_exact: true,
                complete_profile_bound: true,
                system_material_bound: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            })
        }

        fn rollback_to_solid_verified(
            &mut self,
            identity: HeptaWindowsProductHostWindowIdentity,
        ) -> Result<HeptaWindowsVerifiedMaterialRollback, HeptaWindowsProductMaterialBackendError>
        {
            self.rollback_calls += 1;
            if self.mode == FakeMode::RollbackError {
                return Err(HeptaWindowsProductMaterialBackendError::RollbackFailed);
            }
            Ok(HeptaWindowsVerifiedMaterialRollback {
                identity,
                root_none_exact: self.mode != FakeMode::InvalidRollback,
                transient_none_exact: true,
                rollback_verified: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            })
        }
    }

    fn review_receipt() -> HeptaWindowsProductHostReviewReceipt {
        let evidence = HeptaWindowsProductHostReviewEvidence {
            candidate_commit: "a".repeat(40),
            candidate_tree: "b".repeat(40),
            provenance_candidate_commit: "a".repeat(40),
            provenance_candidate_tree: "b".repeat(40),
            qualification_candidate_commit: "a".repeat(40),
            qualification_candidate_tree: "b".repeat(40),
            makepad_revision:
                super::super::hepta_windows_product_host_integration_review::HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION.to_string(),
            provenance_level: HeptaWindowsProvenanceLevel::WindowsRuntime,
            qualification_level: HeptaWindowsProvenanceLevel::WindowsRuntime,
            root_window_index: 1,
            root_window_generation: 7,
            root_hwnd: 11,
            transient_window_index: 2,
            transient_window_generation: 9,
            transient_hwnd: 12,
            root_sequence: 1,
            acrylic_sequence: 2,
            rollback_sequence: 3,
            root_mica_exact: true,
            transient_acrylic_exact: true,
            solid_rollback_exact: true,
            destroyed_acknowledged: true,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
            existing_binding: HeptaWindowsReviewBindingBoundary::default(),
        };
        evaluate_windows_product_host_integration_review(&evidence)
    }

    fn identity() -> HeptaWindowsProductHostWindowIdentity {
        HeptaWindowsProductHostWindowIdentity {
            root_window_index: 1,
            root_window_generation: 7,
            root_hwnd: 11,
            transient_window_index: 2,
            transient_window_generation: 9,
            transient_hwnd: 12,
        }
    }

    fn request() -> HeptaWindowsProductHostActivationRequest {
        HeptaWindowsProductHostActivationRequest {
            review: review_receipt(),
            approval: HeptaWindowsProductHostImplementationApproval {
                candidate_commit: "a".repeat(40),
                candidate_tree: "b".repeat(40),
                review_binding_digest: "c".repeat(64),
                implementation_approved: true,
                operator_accepted: true,
                physical_device_validated: true,
                rollback_drill_validated: true,
                feature_flag_enabled: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            },
            identity: identity(),
            preferences: HeptaWindowsProductHostRuntimePreferences {
                transparency_allowed: true,
                high_contrast: false,
            },
        }
    }

    #[cfg(not(feature = "hepta_ui_windows_system_material_v4"))]
    #[test]
    fn default_build_cannot_activate_product_materials() {
        let mut host = HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::Success));
        assert_eq!(
            host.activate(&request()),
            Err(HeptaWindowsProductHostError::FeatureNotCompiled)
        );
        assert_eq!(host.phase(), HeptaWindowsProductHostPhase::Dormant);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn explicit_approved_activation_binds_without_granting_authority() {
        let mut host = HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::Success));
        let receipt = host.activate(&request()).unwrap();
        assert_eq!(receipt.phase, HeptaWindowsProductHostPhase::Bound);
        assert!(receipt.product_bound);
        assert!(receipt.system_material_bound);
        assert!(receipt.grants_no_authority());
        assert_eq!(host.identity(), Some(identity()));
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn every_governance_prerequisite_is_fail_closed() {
        let mut variants = Vec::new();
        let mut value = request();
        value.approval.feature_flag_enabled = false;
        variants.push((value, HeptaWindowsProductHostError::FeatureDisabled));
        let mut value = request();
        value.approval.implementation_approved = false;
        variants.push((value, HeptaWindowsProductHostError::ImplementationApprovalMissing));
        let mut value = request();
        value.approval.operator_accepted = false;
        variants.push((value, HeptaWindowsProductHostError::OperatorAcceptanceMissing));
        let mut value = request();
        value.approval.physical_device_validated = false;
        variants.push((value, HeptaWindowsProductHostError::DeviceValidationMissing));
        let mut value = request();
        value.approval.rollback_drill_validated = false;
        variants.push((value, HeptaWindowsProductHostError::RollbackDrillMissing));
        let mut value = request();
        value.preferences.transparency_allowed = false;
        variants.push((value, HeptaWindowsProductHostError::TransparencyDisabled));
        let mut value = request();
        value.preferences.high_contrast = true;
        variants.push((value, HeptaWindowsProductHostError::HighContrast));

        for (request, error) in variants {
            let mut host =
                HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::Success));
            assert_eq!(host.activate(&request), Err(error));
            assert_eq!(host.phase(), HeptaWindowsProductHostPhase::Dormant);
        }
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn invalid_binding_rolls_back_and_never_publishes_bound_state() {
        let mut host =
            HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::InvalidBinding));
        assert_eq!(
            host.activate(&request()),
            Err(HeptaWindowsProductHostError::BackendBindingReceiptRejected)
        );
        assert_eq!(host.phase(), HeptaWindowsProductHostPhase::Rejected);
        assert_eq!(host.backend.rollback_calls, 1);
        assert_eq!(host.identity(), None);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn explicit_rollback_suspend_and_shutdown_are_unbound() {
        let mut host = HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::Success));
        host.activate(&request()).unwrap();
        let rollback = host.rollback_to_solid().unwrap();
        assert_eq!(rollback.phase, HeptaWindowsProductHostPhase::SolidFallback);
        assert!(!rollback.product_bound);
        assert!(rollback.grants_no_authority());

        let suspended = host.suspend().unwrap();
        assert_eq!(suspended.phase, HeptaWindowsProductHostPhase::Suspended);
        let shutdown = host.shutdown().unwrap();
        assert_eq!(shutdown.phase, HeptaWindowsProductHostPhase::Shutdown);
        assert_eq!(
            host.activate(&request()),
            Err(HeptaWindowsProductHostError::HostShutdown)
        );
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn rollback_failure_is_rejected_and_authority_constants_remain_false() {
        let mut host =
            HeptaWindowsProductMaterialHost::new(FakeBackend::new(FakeMode::RollbackError));
        host.activate(&request()).unwrap();
        assert_eq!(
            host.rollback_to_solid(),
            Err(HeptaWindowsProductHostError::RollbackFailed)
        );
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_FEATURE_DEFAULT_ENABLED);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_AUTOMATIC_BINDING_ALLOWED);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_PRODUCT_WIRED);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_LIFECYCLE_WIRED);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_PROMOTION);
        assert!(!HEPTA_WINDOWS_PRODUCT_HOST_RELEASE);
    }
}
