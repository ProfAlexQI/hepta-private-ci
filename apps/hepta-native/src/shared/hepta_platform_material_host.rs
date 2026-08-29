//! Fail-closed host lifecycle for Hepta UI v4 platform materials.
//!
//! The host consolidates semantic resolution, verified system preferences,
//! transactional adapter binding, rollback, suspension, and shutdown. Concrete
//! operating-system calls remain behind `HeptaSystemMaterialAdapter`.
//!
//! A host snapshot is descriptive evidence only. It never grants network,
//! mutation, effect, live-adapter, production, operator-acceptance, promotion,
//! or release authority.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialCapabilities,
    HeptaPlatformMaterialProfile, platform_material_profile_with_capabilities,
};
use super::hepta_platform_material_runtime::{
    HeptaMaterialRuntimePreferences, HeptaSystemMaterialAdapter, HeptaSystemMaterialError,
    HeptaSystemMaterialReceipt, bind_material_runtime,
};
use super::hepta_system_preferences::HeptaSystemPreferenceSnapshot;

pub const HEPTA_MATERIAL_HOST_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_HOST_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_HOST_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_MATERIAL_HOST_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_MATERIAL_HOST_PROMOTION: bool = false;
pub const HEPTA_MATERIAL_HOST_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialHostPhase {
    SolidFallback,
    SemanticIntentOnly,
    SystemMaterialBound,
    Suspended,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialFallbackReason {
    UserTransparencyDisabled,
    HighContrast,
    UnsupportedPlatform,
    DynamicColorUnavailable,
    SystemMaterialUnavailable,
    AdapterReceiptRejected,
    LifecycleSuspended,
    LifecycleShutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialHostError {
    Adapter(HeptaSystemMaterialError),
    AdapterReceiptRejected,
    HostShutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMaterialHostSnapshot {
    pub generation: u64,
    pub platform: HeptaPlatform,
    pub phase: HeptaMaterialHostPhase,
    pub profile: HeptaPlatformMaterialProfile,
    pub system_material_bound: bool,
    pub fallback_reason: Option<HeptaMaterialFallbackReason>,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaMaterialHostSnapshot {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeptaMaterialHostCapabilities {
    /// True only when the caller has a concrete adapter and all host objects
    /// required by that adapter for the requested profile.
    pub system_material_available: bool,
    /// Android dynamic color must be independently verified by the Activity
    /// bridge. Other platforms ignore this field.
    pub dynamic_color_available: bool,
}

#[derive(Debug)]
pub struct HeptaPlatformMaterialHost {
    generation: u64,
    snapshot: HeptaMaterialHostSnapshot,
}

impl HeptaPlatformMaterialHost {
    pub fn new(platform: HeptaPlatform) -> Self {
        Self {
            generation: 0,
            snapshot: solid_snapshot(
                0,
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                Some(HeptaMaterialFallbackReason::SystemMaterialUnavailable),
            ),
        }
    }

    pub const fn snapshot(&self) -> HeptaMaterialHostSnapshot {
        self.snapshot
    }

    pub fn apply_snapshot<A: HeptaSystemMaterialAdapter>(
        &mut self,
        adapter: &mut A,
        preferences: HeptaSystemPreferenceSnapshot,
        capabilities: HeptaMaterialHostCapabilities,
    ) -> Result<HeptaMaterialHostSnapshot, HeptaMaterialHostError> {
        self.apply(adapter, preferences.preferences, capabilities)
    }

    pub fn apply<A: HeptaSystemMaterialAdapter>(
        &mut self,
        adapter: &mut A,
        preferences: HeptaMaterialRuntimePreferences,
        capabilities: HeptaMaterialHostCapabilities,
    ) -> Result<HeptaMaterialHostSnapshot, HeptaMaterialHostError> {
        if self.snapshot.phase == HeptaMaterialHostPhase::Shutdown {
            return Err(HeptaMaterialHostError::HostShutdown);
        }

        self.generation = self.generation.saturating_add(1);
        let platform = adapter.platform();
        let effective_preferences = HeptaMaterialRuntimePreferences {
            dynamic_color_available: if platform == HeptaPlatform::Android {
                preferences.dynamic_color_available && capabilities.dynamic_color_available
            } else {
                preferences.dynamic_color_available
            },
            ..preferences
        };

        if effective_preferences.high_contrast {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::HighContrast,
            ));
        }
        if !effective_preferences.transparency_allowed {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::UserTransparencyDisabled,
            ));
        }

        let profile = platform_material_profile_with_capabilities(
            platform,
            true,
            HeptaPlatformMaterialCapabilities {
                dynamic_color_available: effective_preferences.dynamic_color_available,
            },
        );
        if !profile.transparency_enabled {
            adapter.unbind();
            let reason = if platform == HeptaPlatform::Android
                && !effective_preferences.dynamic_color_available
            {
                HeptaMaterialFallbackReason::DynamicColorUnavailable
            } else {
                HeptaMaterialFallbackReason::UnsupportedPlatform
            };
            return Ok(self.install_solid(platform, HeptaMaterialHostPhase::SolidFallback, reason));
        }

        if !capabilities.system_material_available {
            // Clear a previous bind before publishing a semantic-only state.
            adapter.unbind();
            self.snapshot = semantic_snapshot(self.generation, platform, profile);
            return Ok(self.snapshot);
        }

        let receipt = match bind_material_runtime(adapter, effective_preferences) {
            Ok(receipt) => receipt,
            Err(
                HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial
                | HeptaSystemMaterialError::AdapterRejectedProfile,
            ) => {
                adapter.unbind();
                self.install_solid(
                    platform,
                    HeptaMaterialHostPhase::SolidFallback,
                    HeptaMaterialFallbackReason::AdapterReceiptRejected,
                );
                return Err(HeptaMaterialHostError::AdapterReceiptRejected);
            }
            Err(error) => {
                adapter.unbind();
                self.install_solid(
                    platform,
                    HeptaMaterialHostPhase::SolidFallback,
                    fallback_reason_for_error(error),
                );
                return Err(HeptaMaterialHostError::Adapter(error));
            }
        };

        if !receipt_is_valid(receipt, platform, profile) {
            adapter.unbind();
            self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::AdapterReceiptRejected,
            );
            return Err(HeptaMaterialHostError::AdapterReceiptRejected);
        }

        self.snapshot = HeptaMaterialHostSnapshot {
            generation: self.generation,
            platform,
            phase: HeptaMaterialHostPhase::SystemMaterialBound,
            profile,
            system_material_bound: true,
            fallback_reason: None,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        };
        Ok(self.snapshot)
    }

    pub fn suspend<A: HeptaSystemMaterialAdapter>(
        &mut self,
        adapter: &mut A,
    ) -> HeptaMaterialHostSnapshot {
        adapter.unbind();
        self.generation = self.generation.saturating_add(1);
        self.install_solid(
            adapter.platform(),
            HeptaMaterialHostPhase::Suspended,
            HeptaMaterialFallbackReason::LifecycleSuspended,
        )
    }

    pub fn shutdown<A: HeptaSystemMaterialAdapter>(
        &mut self,
        adapter: &mut A,
    ) -> HeptaMaterialHostSnapshot {
        adapter.unbind();
        self.generation = self.generation.saturating_add(1);
        self.install_solid(
            adapter.platform(),
            HeptaMaterialHostPhase::Shutdown,
            HeptaMaterialFallbackReason::LifecycleShutdown,
        )
    }

    fn install_solid(
        &mut self,
        platform: HeptaPlatform,
        phase: HeptaMaterialHostPhase,
        reason: HeptaMaterialFallbackReason,
    ) -> HeptaMaterialHostSnapshot {
        self.snapshot = solid_snapshot(self.generation, platform, phase, Some(reason));
        self.snapshot
    }
}

const fn fallback_reason_for_error(error: HeptaSystemMaterialError) -> HeptaMaterialFallbackReason {
    match error {
        HeptaSystemMaterialError::UserTransparencyDisabled => {
            HeptaMaterialFallbackReason::UserTransparencyDisabled
        }
        HeptaSystemMaterialError::UnsupportedPlatform => {
            HeptaMaterialFallbackReason::UnsupportedPlatform
        }
        HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial
        | HeptaSystemMaterialError::AdapterRejectedProfile => {
            HeptaMaterialFallbackReason::AdapterReceiptRejected
        }
        HeptaSystemMaterialError::SystemApiUnavailable
        | HeptaSystemMaterialError::InvalidHostHandle
        | HeptaSystemMaterialError::SystemCallFailed(_) => {
            HeptaMaterialFallbackReason::SystemMaterialUnavailable
        }
    }
}

fn receipt_is_valid(
    receipt: HeptaSystemMaterialReceipt,
    platform: HeptaPlatform,
    profile: HeptaPlatformMaterialProfile,
) -> bool {
    receipt.platform == platform
        && receipt.chrome == profile.chrome
        && receipt.transient == profile.transient
        && receipt.system_material_bound
        && !receipt.production_authority
        && !receipt.effect_authority
        && profile.content == HeptaMaterialRenderer::Solid
        && profile.stable_content_backdrop_layers == 0
        && profile.max_visible_backdrop_layers <= 2
}

const fn semantic_snapshot(
    generation: u64,
    platform: HeptaPlatform,
    profile: HeptaPlatformMaterialProfile,
) -> HeptaMaterialHostSnapshot {
    HeptaMaterialHostSnapshot {
        generation,
        platform,
        phase: HeptaMaterialHostPhase::SemanticIntentOnly,
        profile,
        system_material_bound: false,
        fallback_reason: Some(HeptaMaterialFallbackReason::SystemMaterialUnavailable),
        production_authority: false,
        effect_authority: false,
        live_adapter_authority: false,
        operator_acceptance: false,
        promotion: false,
        release: false,
    }
}

const fn solid_snapshot(
    generation: u64,
    platform: HeptaPlatform,
    phase: HeptaMaterialHostPhase,
    fallback_reason: Option<HeptaMaterialFallbackReason>,
) -> HeptaMaterialHostSnapshot {
    HeptaMaterialHostSnapshot {
        generation,
        platform,
        phase,
        profile: platform_material_profile_with_capabilities(
            platform,
            false,
            HeptaPlatformMaterialCapabilities {
                dynamic_color_available: false,
            },
        ),
        system_material_bound: false,
        fallback_reason,
        production_authority: false,
        effect_authority: false,
        live_adapter_authority: false,
        operator_acceptance: false,
        promotion: false,
        release: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hepta_system_preferences::HeptaSystemPreferenceSnapshot;

    #[derive(Clone, Copy)]
    enum FakeMode {
        Success,
        Error(HeptaSystemMaterialError),
        UnboundReceipt,
        InvalidAuthority,
        InvalidProfile,
    }

    struct FakeAdapter {
        platform: HeptaPlatform,
        mode: FakeMode,
        bind_count: usize,
        unbind_count: usize,
    }

    impl FakeAdapter {
        fn new(platform: HeptaPlatform, mode: FakeMode) -> Self {
            Self {
                platform,
                mode,
                bind_count: 0,
                unbind_count: 0,
            }
        }
    }

    impl HeptaSystemMaterialAdapter for FakeAdapter {
        fn platform(&self) -> HeptaPlatform {
            self.platform
        }

        fn bind(
            &mut self,
            profile: HeptaPlatformMaterialProfile,
        ) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
            self.bind_count += 1;
            match self.mode {
                FakeMode::Error(error) => Err(error),
                FakeMode::UnboundReceipt => Ok(HeptaSystemMaterialReceipt {
                    platform: self.platform,
                    chrome: profile.chrome,
                    transient: profile.transient,
                    system_material_bound: false,
                    production_authority: false,
                    effect_authority: false,
                }),
                FakeMode::InvalidAuthority => Ok(HeptaSystemMaterialReceipt {
                    platform: self.platform,
                    chrome: profile.chrome,
                    transient: profile.transient,
                    system_material_bound: true,
                    production_authority: true,
                    effect_authority: false,
                }),
                FakeMode::InvalidProfile => Ok(HeptaSystemMaterialReceipt {
                    platform: self.platform,
                    chrome: HeptaMaterialRenderer::Solid,
                    transient: HeptaMaterialRenderer::Solid,
                    system_material_bound: true,
                    production_authority: false,
                    effect_authority: false,
                }),
                FakeMode::Success => Ok(HeptaSystemMaterialReceipt {
                    platform: self.platform,
                    chrome: profile.chrome,
                    transient: profile.transient,
                    system_material_bound: true,
                    production_authority: false,
                    effect_authority: false,
                }),
            }
        }

        fn unbind(&mut self) {
            self.unbind_count += 1;
        }
    }

    fn transparent_preferences() -> HeptaMaterialRuntimePreferences {
        HeptaMaterialRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: true,
        }
    }

    fn available_capabilities() -> HeptaMaterialHostCapabilities {
        HeptaMaterialHostCapabilities {
            system_material_available: true,
            dynamic_color_available: true,
        }
    }

    #[test]
    fn disabled_transparency_and_high_contrast_are_solid_without_binding() {
        for preferences in [
            HeptaMaterialRuntimePreferences {
                transparency_allowed: false,
                ..transparent_preferences()
            },
            HeptaMaterialRuntimePreferences {
                high_contrast: true,
                ..transparent_preferences()
            },
        ] {
            let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
            let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, FakeMode::Success);
            let snapshot = host
                .apply(&mut adapter, preferences, available_capabilities())
                .unwrap();

            assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SolidFallback);
            assert_eq!(snapshot.profile.content, HeptaMaterialRenderer::Solid);
            assert!(!snapshot.profile.transparency_enabled);
            assert_eq!(adapter.bind_count, 0);
            assert_eq!(adapter.unbind_count, 1);
            assert!(snapshot.grants_no_authority());
        }
    }

    #[test]
    fn unavailable_system_material_keeps_semantic_intent_unbound() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::MacOs);
        let mut adapter = FakeAdapter::new(HeptaPlatform::MacOs, FakeMode::Success);
        let snapshot = host
            .apply(
                &mut adapter,
                transparent_preferences(),
                HeptaMaterialHostCapabilities::default(),
            )
            .unwrap();

        assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SemanticIntentOnly);
        assert!(!snapshot.system_material_bound);
        assert_eq!(adapter.bind_count, 0);
        assert_eq!(adapter.unbind_count, 1);
        assert!(snapshot.grants_no_authority());
    }

    #[test]
    fn android_requires_both_preference_and_host_dynamic_color_evidence() {
        for capabilities in [
            HeptaMaterialHostCapabilities {
                system_material_available: true,
                dynamic_color_available: false,
            },
            HeptaMaterialHostCapabilities {
                system_material_available: false,
                dynamic_color_available: true,
            },
        ] {
            let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Android);
            let mut adapter = FakeAdapter::new(HeptaPlatform::Android, FakeMode::Success);
            let snapshot = host
                .apply(&mut adapter, transparent_preferences(), capabilities)
                .unwrap();

            if capabilities.dynamic_color_available {
                assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SemanticIntentOnly);
            } else {
                assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SolidFallback);
                assert_eq!(
                    snapshot.fallback_reason,
                    Some(HeptaMaterialFallbackReason::DynamicColorUnavailable)
                );
            }
            assert!(!snapshot.system_material_bound);
            assert_eq!(adapter.bind_count, 0);
            assert!(snapshot.grants_no_authority());
        }
    }

    #[test]
    fn valid_receipt_binds_a_bounded_profile_from_a_system_snapshot() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
        let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, FakeMode::Success);
        let snapshot = host
            .apply_snapshot(
                &mut adapter,
                HeptaSystemPreferenceSnapshot::from_host(transparent_preferences()),
                available_capabilities(),
            )
            .unwrap();

        assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SystemMaterialBound);
        assert!(snapshot.system_material_bound);
        assert_eq!(snapshot.profile.content, HeptaMaterialRenderer::Solid);
        assert_eq!(snapshot.profile.stable_content_backdrop_layers, 0);
        assert!(snapshot.profile.max_visible_backdrop_layers <= 2);
        assert_eq!(adapter.bind_count, 1);
        assert_eq!(adapter.unbind_count, 0);
        assert!(snapshot.grants_no_authority());
    }

    #[test]
    fn invalid_receipts_roll_back_to_solid() {
        for mode in [
            FakeMode::UnboundReceipt,
            FakeMode::InvalidAuthority,
            FakeMode::InvalidProfile,
        ] {
            let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
            let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, mode);
            assert_eq!(
                host.apply(
                    &mut adapter,
                    transparent_preferences(),
                    available_capabilities(),
                ),
                Err(HeptaMaterialHostError::AdapterReceiptRejected),
            );
            assert_eq!(host.snapshot().phase, HeptaMaterialHostPhase::SolidFallback);
            assert!(!host.snapshot().profile.transparency_enabled);
            assert!(adapter.unbind_count >= 1);
            assert!(host.snapshot().grants_no_authority());
        }
    }

    #[test]
    fn adapter_failure_suspend_and_shutdown_are_fail_closed() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
        let mut adapter = FakeAdapter::new(
            HeptaPlatform::Windows,
            FakeMode::Error(HeptaSystemMaterialError::SystemApiUnavailable),
        );
        assert_eq!(
            host.apply(
                &mut adapter,
                transparent_preferences(),
                available_capabilities(),
            ),
            Err(HeptaMaterialHostError::Adapter(
                HeptaSystemMaterialError::SystemApiUnavailable,
            )),
        );
        assert_eq!(host.snapshot().phase, HeptaMaterialHostPhase::SolidFallback);
        assert!(host.snapshot().grants_no_authority());

        let failed_generation = host.snapshot().generation;
        let suspended = host.suspend(&mut adapter);
        assert_eq!(suspended.phase, HeptaMaterialHostPhase::Suspended);
        assert!(suspended.generation > failed_generation);
        assert!(!suspended.profile.transparency_enabled);

        let shutdown = host.shutdown(&mut adapter);
        assert_eq!(shutdown.phase, HeptaMaterialHostPhase::Shutdown);
        assert!(shutdown.grants_no_authority());
        assert_eq!(
            host.apply(
                &mut adapter,
                transparent_preferences(),
                available_capabilities(),
            ),
            Err(HeptaMaterialHostError::HostShutdown),
        );
    }

    #[test]
    fn host_authority_constants_remain_false() {
        assert!(!HEPTA_MATERIAL_HOST_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_MATERIAL_HOST_EFFECT_AUTHORITY);
        assert!(!HEPTA_MATERIAL_HOST_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_MATERIAL_HOST_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_MATERIAL_HOST_PROMOTION);
        assert!(!HEPTA_MATERIAL_HOST_RELEASE);
    }
}
