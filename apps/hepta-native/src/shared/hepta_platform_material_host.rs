//! Fail-closed host lifecycle for Hepta UI v4 platform materials.
//!
//! The host owns transactional adapter binding, accessibility fallbacks, and
//! lifecycle cleanup. Operating-system calls remain behind
//! `HeptaSystemMaterialAdapter`; this module grants no production, effect,
//! live-adapter, operator-acceptance, promotion, or release authority.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile,
    platform_material_profile,
};
use super::hepta_platform_material_runtime::{
    HeptaMaterialRuntimePreferences, HeptaSystemMaterialAdapter, HeptaSystemMaterialError,
    HeptaSystemMaterialReceipt,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMaterialHostCapabilities {
    pub system_material_available: bool,
    pub dynamic_color_available: bool,
}

impl Default for HeptaMaterialHostCapabilities {
    fn default() -> Self {
        Self {
            system_material_available: false,
            dynamic_color_available: false,
        }
    }
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

        if preferences.high_contrast {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::HighContrast,
            ));
        }
        if !preferences.transparency_allowed {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::UserTransparencyDisabled,
            ));
        }

        let profile = platform_material_profile(platform, true);
        if !profile.transparency_enabled {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::UnsupportedPlatform,
            ));
        }
        if platform == HeptaPlatform::Android
            && !(capabilities.dynamic_color_available && preferences.dynamic_color_available)
        {
            adapter.unbind();
            return Ok(self.install_solid(
                platform,
                HeptaMaterialHostPhase::SolidFallback,
                HeptaMaterialFallbackReason::DynamicColorUnavailable,
            ));
        }
        if !capabilities.system_material_available {
            adapter.unbind();
            self.snapshot = semantic_snapshot(self.generation, platform, profile);
            return Ok(self.snapshot);
        }

        let receipt = match adapter.bind(profile) {
            Ok(receipt) => receipt,
            Err(error) => {
                adapter.unbind();
                self.install_solid(
                    platform,
                    HeptaMaterialHostPhase::SolidFallback,
                    HeptaMaterialFallbackReason::SystemMaterialUnavailable,
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
        profile: platform_material_profile(platform, false),
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

    #[derive(Clone, Copy)]
    enum FakeMode {
        Success,
        Error,
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
                FakeMode::Error => Err(HeptaSystemMaterialError::SystemApiUnavailable),
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
    fn disabled_transparency_uses_solid_without_binding() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
        let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, FakeMode::Success);
        let snapshot = host
            .apply(
                &mut adapter,
                HeptaMaterialRuntimePreferences {
                    transparency_allowed: false,
                    ..transparent_preferences()
                },
                available_capabilities(),
            )
            .unwrap();

        assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SolidFallback);
        assert_eq!(snapshot.profile.content, HeptaMaterialRenderer::Solid);
        assert!(!snapshot.profile.transparency_enabled);
        assert_eq!(adapter.bind_count, 0);
        assert!(snapshot.grants_no_authority());
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
        assert!(snapshot.grants_no_authority());
    }

    #[test]
    fn valid_receipt_binds_a_bounded_profile() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
        let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, FakeMode::Success);
        let snapshot = host
            .apply(
                &mut adapter,
                transparent_preferences(),
                available_capabilities(),
            )
            .unwrap();

        assert_eq!(snapshot.phase, HeptaMaterialHostPhase::SystemMaterialBound);
        assert!(snapshot.system_material_bound);
        assert_eq!(snapshot.profile.content, HeptaMaterialRenderer::Solid);
        assert_eq!(snapshot.profile.stable_content_backdrop_layers, 0);
        assert!(snapshot.profile.max_visible_backdrop_layers <= 2);
        assert_eq!(adapter.bind_count, 1);
        assert!(snapshot.grants_no_authority());
    }

    #[test]
    fn invalid_receipt_rolls_back_to_solid() {
        for mode in [FakeMode::InvalidAuthority, FakeMode::InvalidProfile] {
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
            assert_eq!(adapter.unbind_count, 1);
            assert!(host.snapshot().grants_no_authority());
        }
    }

    #[test]
    fn adapter_failure_and_suspend_are_fail_closed() {
        let mut host = HeptaPlatformMaterialHost::new(HeptaPlatform::Windows);
        let mut adapter = FakeAdapter::new(HeptaPlatform::Windows, FakeMode::Error);
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

        let suspended = host.suspend(&mut adapter);
        assert_eq!(suspended.phase, HeptaMaterialHostPhase::Suspended);
        assert!(!suspended.profile.transparency_enabled);
        assert!(suspended.grants_no_authority());
    }
}
