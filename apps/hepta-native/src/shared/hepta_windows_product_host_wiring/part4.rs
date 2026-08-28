#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use super::super::hepta_windows_product_host_integration_review::{
        HeptaWindowsProductHostReviewEvidence, HeptaWindowsProvenanceLevel,
        HeptaWindowsReviewBindingBoundary, evaluate_windows_product_host_integration_review,
    };
    use super::super::hepta_windows_material_adapter::HeptaWindowsBackdropReadbackError;

    #[derive(Default)]
    struct FakeApi {
        values: BTreeMap<isize, HeptaWindowsDwmBackdropValue>,
        fail_set: Option<HeptaWindowsBackdropKind>,
        fail_read: bool,
    }

    impl HeptaWindowsBackdropApi for FakeApi {
        fn set_backdrop(
            &mut self,
            window: isize,
            kind: HeptaWindowsBackdropKind,
        ) -> Result<(), i32> {
            if self.fail_set == Some(kind) {
                return Err(-5);
            }
            self.values.insert(
                window,
                match kind {
                    HeptaWindowsBackdropKind::None => HeptaWindowsDwmBackdropValue::None,
                    HeptaWindowsBackdropKind::Mica => HeptaWindowsDwmBackdropValue::Mica,
                    HeptaWindowsBackdropKind::Acrylic => {
                        HeptaWindowsDwmBackdropValue::Acrylic
                    }
                },
            );
            Ok(())
        }
    }

    impl HeptaWindowsBackdropReadbackApi for FakeApi {
        fn read_backdrop(
            &mut self,
            window: isize,
        ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
            if self.fail_read {
                return Err(HeptaWindowsBackdropReadbackError::SystemCallFailed(-6));
            }
            self.values
                .get(&window)
                .copied()
                .ok_or(HeptaWindowsBackdropReadbackError::UnknownBackdropValue(-1))
        }
    }

    fn review() -> HeptaWindowsProductHostReviewReceipt {
        evaluate_windows_product_host_integration_review(
            &HeptaWindowsProductHostReviewEvidence {
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
            },
        )
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

    fn runtime_identity() -> HeptaWindowsProductHostReviewedRuntimeIdentity {
        HeptaWindowsProductHostReviewedRuntimeIdentity {
            window: identity(),
            root_sequence: 1,
            acrylic_sequence: 2,
            rollback_sequence: 3,
            destroyed_acknowledged: true,
        }
    }

    fn seal_result() -> Result<HeptaWindowsProductHostEvidenceSeal, HeptaWindowsProductHostWiringError> {
        HeptaWindowsProductHostEvidenceSeal::try_new(
            HeptaWindowsProductHostBuildIdentity {
                commit: "c".repeat(40),
                tree: "d".repeat(40),
            },
            review(),
            HeptaWindowsProductHostImplementationApproval {
                candidate_commit: "c".repeat(40),
                candidate_tree: "d".repeat(40),
                review_binding_digest: "e".repeat(64),
                implementation_approved: true,
                operator_accepted: true,
                physical_device_validated: true,
                rollback_drill_validated: true,
                feature_flag_enabled: true,
                authority: HeptaWindowsReviewAuthorityBoundary::default(),
            },
            HeptaWindowsProductHostReviewEnvelopeSeal {
                candidate_commit: "a".repeat(40),
                candidate_tree: "b".repeat(40),
                makepad_revision: HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION.to_string(),
                binding_digest: "e".repeat(64),
                source_run_id: 17,
                runtime_identity: runtime_identity(),
            },
            HeptaWindowsProductHostDeviceQualificationSeal {
                implementation_commit: "c".repeat(40),
                implementation_tree: "d".repeat(40),
                review_binding_digest: "e".repeat(64),
                operator_acceptance_digest: "f".repeat(64),
                device_qualification_digest: "1".repeat(64),
                feature_enabled: true,
                physical_device_validated: true,
                rollback_drill_validated: true,
                high_contrast_fallback_validated: true,
                transparency_disabled_fallback_validated: true,
                suspend_rollback_validated: true,
                shutdown_rollback_validated: true,
                final_state_unbound: true,
                runtime_identity: runtime_identity(),
            },
        )
    }

    fn seal() -> HeptaWindowsProductHostEvidenceSeal {
        seal_result().unwrap()
    }

    fn preferences() -> HeptaWindowsProductHostRuntimePreferences {
        HeptaWindowsProductHostRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
        }
    }

    #[test]
    fn exact_dwm_backend_binds_and_rolls_back_both_handles() {
        let mut backend = HeptaWindowsProductDwmBackend::new(FakeApi::default());
        let binding = backend.bind_verified(identity()).unwrap();
        assert!(binding.root_mica_exact);
        assert!(binding.transient_acrylic_exact);
        assert!(binding.authority.grants_none());
        let rollback = backend.rollback_to_solid_verified(identity()).unwrap();
        assert!(rollback.root_none_exact);
        assert!(rollback.transient_none_exact);
        assert!(rollback.rollback_verified);
    }

    #[test]
    fn evidence_seal_rejects_candidate_digest_and_runtime_drift() {
        let mut approval = HeptaWindowsProductHostImplementationApproval {
            candidate_commit: "0".repeat(40),
            candidate_tree: "d".repeat(40),
            review_binding_digest: "e".repeat(64),
            implementation_approved: true,
            operator_accepted: true,
            physical_device_validated: true,
            rollback_drill_validated: true,
            feature_flag_enabled: true,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        };
        let base = HeptaWindowsProductHostBuildIdentity {
            commit: "c".repeat(40),
            tree: "d".repeat(40),
        };
        let envelope = HeptaWindowsProductHostReviewEnvelopeSeal {
            candidate_commit: "a".repeat(40),
            candidate_tree: "b".repeat(40),
            makepad_revision: HEPTA_WINDOWS_REVIEW_MAKEPAD_REVISION.to_string(),
            binding_digest: "e".repeat(64),
            source_run_id: 17,
            runtime_identity: runtime_identity(),
        };
        let device = HeptaWindowsProductHostDeviceQualificationSeal {
            implementation_commit: "c".repeat(40),
            implementation_tree: "d".repeat(40),
            review_binding_digest: "e".repeat(64),
            operator_acceptance_digest: "f".repeat(64),
            device_qualification_digest: "1".repeat(64),
            feature_enabled: true,
            physical_device_validated: true,
            rollback_drill_validated: true,
            high_contrast_fallback_validated: true,
            transparency_disabled_fallback_validated: true,
            suspend_rollback_validated: true,
            shutdown_rollback_validated: true,
            final_state_unbound: true,
            runtime_identity: runtime_identity(),
        };
        assert_eq!(
            HeptaWindowsProductHostEvidenceSeal::try_new(
                base.clone(), review(), approval.clone(), envelope.clone(), device.clone(),
            ),
            Err(HeptaWindowsProductHostWiringError::ApprovalCandidateDrift)
        );
        approval.candidate_commit = "c".repeat(40);
        approval.review_binding_digest = "0".repeat(64);
        assert_eq!(
            HeptaWindowsProductHostEvidenceSeal::try_new(
                base.clone(), review(), approval.clone(), envelope.clone(), device.clone(),
            ),
            Err(HeptaWindowsProductHostWiringError::ReviewDigestDrift)
        );
        approval.review_binding_digest = "e".repeat(64);
        let mut bad_device = device;
        bad_device.runtime_identity.rollback_sequence = 4;
        assert_eq!(
            HeptaWindowsProductHostEvidenceSeal::try_new(
                base, review(), approval, envelope, bad_device,
            ),
            Err(HeptaWindowsProductHostWiringError::RuntimeIdentityDrift)
        );
    }

    #[cfg(not(feature = "hepta_ui_windows_system_material_v4"))]
    #[test]
    fn default_feature_remains_dormant() {
        let mut coordinator =
            HeptaWindowsProductHostCoordinator::new(HeptaWindowsProductDwmBackend::new(
                FakeApi::default(),
            ));
        assert_eq!(
            coordinator.activate_explicit(&seal(), preferences()),
            Err(HeptaWindowsProductHostWiringError::Host(
                HeptaWindowsProductHostError::FeatureNotCompiled
            ))
        );
        assert_eq!(coordinator.phase(), HeptaWindowsProductHostPhase::Dormant);
    }

    #[cfg(feature = "hepta_ui_windows_system_material_v4")]
    #[test]
    fn explicit_activation_and_preference_reconcile_are_transactional() {
        let mut coordinator =
            HeptaWindowsProductHostCoordinator::new(HeptaWindowsProductDwmBackend::new(
                FakeApi::default(),
            ));
        let bound = coordinator
            .activate_explicit(&seal(), preferences())
            .unwrap();
        assert!(bound.product_bound);
        assert!(bound.system_material_bound);
        assert!(bound.grants_no_authority());
        assert_eq!(
            coordinator.active_review_binding_digest(),
            Some("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee")
        );

        let fallback = coordinator
            .reconcile_preferences(HeptaWindowsProductHostRuntimePreferences {
                transparency_allowed: true,
                high_contrast: true,
            })
            .unwrap()
            .unwrap();
        assert_eq!(fallback.phase, HeptaWindowsProductHostPhase::SolidFallback);
        assert!(!fallback.product_bound);
        assert!(fallback.grants_no_authority());
    }

    #[test]
    fn source_constants_never_grant_authority() {
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_FEATURE_DEFAULT_ENABLED);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_AUTOMATIC_ACTIVATION);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCT_CALLER_REGISTERED);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_DEVICE_VALIDATED);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCT_BOUND);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_PROMOTION);
        assert!(!HEPTA_WINDOWS_PRODUCT_WIRING_RELEASE);
    }
}
