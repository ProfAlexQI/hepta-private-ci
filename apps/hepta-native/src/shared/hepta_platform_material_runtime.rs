//! Runtime selection seam for Hepta UI v4 platform materials.
//!
//! The selector resolves the current compilation target and user accessibility
//! preferences into a bounded semantic profile. System APIs remain behind an
//! explicit adapter trait; the default adapter is fail-closed and solid.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile,
    platform_material_profile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMaterialRuntimePreferences {
    pub transparency_allowed: bool,
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub dynamic_color_available: bool,
}

impl Default for HeptaMaterialRuntimePreferences {
    fn default() -> Self {
        Self {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialBindingStatus {
    SolidFallback,
    SemanticIntentOnly,
    SystemMaterialBound,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaResolvedMaterialRuntime {
    pub platform: HeptaPlatform,
    pub profile: HeptaPlatformMaterialProfile,
    pub status: HeptaMaterialBindingStatus,
    pub high_contrast: bool,
    pub reduced_motion: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaSystemMaterialError {
    UnsupportedPlatform,
    SystemApiUnavailable,
    UserTransparencyDisabled,
    AdapterRejectedProfile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaSystemMaterialReceipt {
    pub platform: HeptaPlatform,
    pub chrome: HeptaMaterialRenderer,
    pub transient: HeptaMaterialRenderer,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
}

pub trait HeptaSystemMaterialAdapter {
    fn platform(&self) -> HeptaPlatform;

    fn bind(
        &mut self,
        profile: HeptaPlatformMaterialProfile,
    ) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError>;

    fn unbind(&mut self);
}

#[derive(Default)]
pub struct HeptaUnboundSystemMaterialAdapter;

impl HeptaSystemMaterialAdapter for HeptaUnboundSystemMaterialAdapter {
    fn platform(&self) -> HeptaPlatform { current_platform() }

    fn bind(
        &mut self,
        _profile: HeptaPlatformMaterialProfile,
    ) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
        Err(HeptaSystemMaterialError::SystemApiUnavailable)
    }

    fn unbind(&mut self) {}
}

pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_PROMOTION: bool = false;

pub const fn current_platform() -> HeptaPlatform {
    if cfg!(target_os = "windows") {
        HeptaPlatform::Windows
    } else if cfg!(target_os = "macos") {
        HeptaPlatform::MacOs
    } else if cfg!(target_os = "ios") {
        HeptaPlatform::Ios
    } else if cfg!(target_os = "android") {
        HeptaPlatform::Android
    } else if cfg!(target_arch = "wasm32") {
        HeptaPlatform::Web
    } else if cfg!(target_os = "linux") {
        HeptaPlatform::Linux
    } else {
        HeptaPlatform::Unknown
    }
}

pub const fn resolve_material_runtime(
    platform: HeptaPlatform,
    preferences: HeptaMaterialRuntimePreferences,
    system_adapter_bound: bool,
) -> HeptaResolvedMaterialRuntime {
    let transparency_allowed = preferences.transparency_allowed && !preferences.high_contrast;
    let profile = platform_material_profile(platform, transparency_allowed);
    let status = if !profile.transparency_enabled {
        HeptaMaterialBindingStatus::SolidFallback
    } else if system_adapter_bound {
        HeptaMaterialBindingStatus::SystemMaterialBound
    } else {
        HeptaMaterialBindingStatus::SemanticIntentOnly
    };

    HeptaResolvedMaterialRuntime {
        platform,
        profile,
        status,
        high_contrast: preferences.high_contrast,
        reduced_motion: preferences.reduced_motion,
    }
}

pub fn bind_material_runtime<A: HeptaSystemMaterialAdapter>(
    adapter: &mut A,
    preferences: HeptaMaterialRuntimePreferences,
) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
    if !preferences.transparency_allowed || preferences.high_contrast {
        adapter.unbind();
        return Err(HeptaSystemMaterialError::UserTransparencyDisabled);
    }

    let platform = adapter.platform();
    let profile = platform_material_profile(platform, true);
    if !profile.transparency_enabled {
        adapter.unbind();
        return Err(HeptaSystemMaterialError::UnsupportedPlatform);
    }

    let receipt = adapter.bind(profile)?;
    if receipt.platform != platform
        || receipt.chrome != profile.chrome
        || receipt.transient != profile.transient
        || receipt.production_authority
        || receipt.effect_authority
    {
        adapter.unbind();
        return Err(HeptaSystemMaterialError::AdapterRejectedProfile);
    }
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_material_resolution_is_bounded_and_fail_closed() {
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_EFFECT_AUTHORITY);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_PROMOTION);

        let preferences = HeptaMaterialRuntimePreferences::default();
        let intent = resolve_material_runtime(HeptaPlatform::Windows, preferences, false);
        assert_eq!(intent.status, HeptaMaterialBindingStatus::SemanticIntentOnly);
        assert_eq!(intent.profile.content, HeptaMaterialRenderer::Solid);
        assert!(intent.profile.max_visible_backdrop_layers <= 2);

        let high_contrast = resolve_material_runtime(
            HeptaPlatform::Windows,
            HeptaMaterialRuntimePreferences { high_contrast: true, ..preferences },
            true,
        );
        assert_eq!(high_contrast.status, HeptaMaterialBindingStatus::SolidFallback);
        assert!(!high_contrast.profile.transparency_enabled);

        let mut unbound = HeptaUnboundSystemMaterialAdapter;
        assert_eq!(
            bind_material_runtime(&mut unbound, preferences),
            Err(HeptaSystemMaterialError::SystemApiUnavailable),
        );
    }
}
