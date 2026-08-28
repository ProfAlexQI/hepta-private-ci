#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hepta_windows_product_host_integration_review::{
        HeptaWindowsProductHostReviewEvidence, HeptaWindowsProvenanceLevel,
        HeptaWindowsReviewBindingBoundary, evaluate_windows_product_host_integration_review,
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum BindMode {
        Success,
        Error,
        Invalid,
    }

    struct FakeBackend {
        bind_mode: BindMode,
        rollback_failures_remaining: usize,
        invalid_rollback: bool,
        bind_calls: usize,
        rollback_calls: usize,
    }

    impl FakeBackend {
        fn success() -> Self {
            Self {
                bind_mode: BindMode::Success,
                rollback_failures_remaining: 0,
                invalid_rollback: false,
                bind_calls: 0,
                rollback_calls: 0,
            }
        }
    }

    impl HeptaWindowsQualificationBackend for FakeBackend {
        fn bind_qualification_verified(
            &mut self,
            identity: HeptaWindowsQualificationWindowIdentity,
        ) -> Result<HeptaWindowsQualificationBindingReceipt, HeptaWindowsQualificationBackendError>
        {
            self.bind_calls += 1;
            if self.bind_mode == BindMode::Error {
                return Err(HeptaWindowsQualificationBackendError::BindFailed);
            }
            Ok(HeptaWindowsQualificationBindingReceipt {
                identity,
                root_mica_exact: self.bind_mode != BindMode::Invalid,
                transient_acrylic_exact: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            })
        }

        fn rollback_qualification_to_solid_verified(
            &mut self,
            identity: HeptaWindowsQualificationWindowIdentity,
        ) -> Result<HeptaWindowsQualificationRollbackReceipt, HeptaWindowsQualificationBackendError>
        {
            self.rollback_calls += 1;
            if self.rollback_failures_remaining > 0 {
                self.rollback_failures_remaining -= 1;
                return Err(HeptaWindowsQualificationBackendError::RollbackFailed);
            }
            Ok(HeptaWindowsQualificationRollbackReceipt {
                identity,
                root_none_exact: !self.invalid_rollback,
                transient_none_exact: true,
                rollback_verified: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            })
        }
    }

    fn review_receipt() -> HeptaWindowsProductHostReviewReceipt {
        evaluate_windows_product_host_integration_review(&HeptaWindowsProductHostReviewEvidence {
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
        })
    }

    fn identity() -> HeptaWindowsQualificationWindowIdentity {
        HeptaWindowsQualificationWindowIdentity {
            root_window_index: 11,
            root_window_generation: 1,
            root_hwnd: 101,
            transient_window_index: 12,
            transient_window_generation: 1,
            transient_hwnd: 102,
        }
    }

    fn request() -> HeptaWindowsQualificationRequest {
        HeptaWindowsQualificationRequest {
            review: review_receipt(),
            review_seal: HeptaWindowsProductHostReviewSeal {
                evidence_candidate_commit: "a".repeat(40),
                evidence_candidate_tree: "b".repeat(40),
                binding_digest: "d".repeat(64),
            },
            implementation_candidate_commit: "c".repeat(40),
            implementation_candidate_tree: "e".repeat(40),
            approval: HeptaWindowsQualificationApproval {
                implementation_candidate_commit: "c".repeat(40),
                implementation_candidate_tree: "e".repeat(40),
                review_binding_digest: "d".repeat(64),
                feature_flag_enabled: true,
                implementation_approved: true,
                operator_accepted: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            },
            identity: identity(),
            preferences: HeptaWindowsQualificationPreferences {
                transparency_allowed: true,
                high_contrast: false,
            },
        }
    }

    #[cfg(not(feature = "hepta_ui_windows_system_material_v4"))]
    #[test]
    fn default_build_cannot_start_device_qualification() {
        let mut host = HeptaWindowsQualificationHost::new(FakeBackend::success());
        assert_eq!(
            host.begin_qualification(&request()),
            Err(HeptaWindowsQualificationError::FeatureNotCompiled)
        );
        assert_eq!(host.phase(), HeptaWindowsQualificationPhase::Dormant);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn qualification_has_no_circular_device_or_rollback_prerequisite() {
        let mut host = HeptaWindowsQualificationHost::new(FakeBackend::success());
        let active = host.begin_qualification(&request()).unwrap();
        assert!(active.qualification_material_active);
        assert!(active.rollback_required);
        assert!(active.remains_non_product());
        let rolled_back = host.rollback_to_solid().unwrap();
        assert!(rolled_back.qualified_unbound);
        assert!(!rolled_back.rollback_required);
        assert!(rolled_back.remains_non_product());
        let evidence = host.qualification_evidence().unwrap();
        assert!(evidence.qualified_unbound);
        assert!(evidence.remains_non_product());
        assert!(evidence.grants_no_authority());
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn review_digest_and_implementation_identity_are_bound() {
        let mut value = request();
        value.approval.review_binding_digest = "f".repeat(64);
        let mut host = HeptaWindowsQualificationHost::new(FakeBackend::success());
        assert_eq!(
            host.begin_qualification(&value),
            Err(HeptaWindowsQualificationError::ReviewDigestMismatch)
        );
        assert_eq!(host.backend_mut().bind_calls, 0);

        let mut value = request();
        value.approval.implementation_candidate_commit = "f".repeat(40);
        let mut host = HeptaWindowsQualificationHost::new(FakeBackend::success());
        assert_eq!(
            host.begin_qualification(&value),
            Err(HeptaWindowsQualificationError::ImplementationCandidateMismatch)
        );
        assert_eq!(host.backend_mut().bind_calls, 0);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn rollback_failure_retains_identity_and_blocks_false_safe_state() {
        let mut backend = FakeBackend::success();
        backend.rollback_failures_remaining = 1;
        let mut host = HeptaWindowsQualificationHost::new(backend);
        host.begin_qualification(&request()).unwrap();
        assert_eq!(
            host.rollback_to_solid(),
            Err(HeptaWindowsQualificationError::RollbackFailed)
        );
        assert_eq!(host.phase(), HeptaWindowsQualificationPhase::RejectedUnsafe);
        assert!(host.requires_rollback());
        assert_eq!(host.active_identity(), Some(identity()));

        let recovered = host.rollback_to_solid().unwrap();
        assert!(recovered.qualified_unbound);
        assert!(!host.requires_rollback());
        assert_eq!(host.active_identity(), None);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn failed_bind_with_failed_rollback_cannot_suspend_or_shutdown_as_unbound() {
        let mut backend = FakeBackend::success();
        backend.bind_mode = BindMode::Error;
        backend.rollback_failures_remaining = 3;
        let mut host = HeptaWindowsQualificationHost::new(backend);
        assert_eq!(
            host.begin_qualification(&request()),
            Err(HeptaWindowsQualificationError::RollbackFailed)
        );
        assert!(host.requires_rollback());
        assert_eq!(host.phase(), HeptaWindowsQualificationPhase::RejectedUnsafe);
        assert_eq!(
            host.suspend(),
            Err(HeptaWindowsQualificationError::RollbackFailed)
        );
        assert_eq!(host.phase(), HeptaWindowsQualificationPhase::RejectedUnsafe);
        assert_eq!(
            host.shutdown(),
            Err(HeptaWindowsQualificationError::RollbackFailed)
        );
        assert_ne!(host.phase(), HeptaWindowsQualificationPhase::Shutdown);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn preference_suspend_and_shutdown_paths_verify_solid_rollback() {
        for operation in 0..4 {
            let mut host = HeptaWindowsQualificationHost::new(FakeBackend::success());
            host.begin_qualification(&request()).unwrap();
            let receipt = match operation {
                0 => host
                    .enforce_preferences(HeptaWindowsQualificationPreferences {
                        transparency_allowed: true,
                        high_contrast: true,
                    })
                    .unwrap(),
                1 => host
                    .enforce_preferences(HeptaWindowsQualificationPreferences {
                        transparency_allowed: false,
                        high_contrast: false,
                    })
                    .unwrap(),
                2 => host.suspend().unwrap(),
                _ => host.shutdown().unwrap(),
            };
            assert!(!receipt.rollback_required);
            assert!(receipt.root_none_exact);
            assert!(receipt.transient_none_exact);
            assert!(receipt.remains_non_product());
            assert!(receipt.grants_no_authority());
        }
    }

    #[test]
    fn all_source_authority_and_product_wiring_constants_remain_false() {
        assert!(!HEPTA_WINDOWS_QUALIFICATION_FEATURE_DEFAULT_ENABLED);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_AUTOMATIC_BINDING_ALLOWED);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_PRODUCT_WIRED);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_LIFECYCLE_WIRED);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_PRODUCT_BOUND);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_PROMOTION);
        assert!(!HEPTA_WINDOWS_QUALIFICATION_RELEASE);
    }
}
