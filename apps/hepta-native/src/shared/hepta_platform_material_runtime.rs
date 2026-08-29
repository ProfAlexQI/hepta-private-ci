//! Runtime selection seam for Hepta UI v4 platform materials.
//!
//! The selector resolves the current compilation target and verified system
//! preferences into a bounded semantic profile. System APIs remain behind an
//! explicit adapter trait; the default adapter is fail-closed and solid.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialCapabilities,
    HeptaPlatformMaterialProfile, platform_material_profile_with_capabilities,
};
pub use super::hepta_system_preferences::HeptaMaterialRuntimePreferences;
use super::hepta_system_preferences::{HeptaSystemPreferenceSnapshot, current_system_preferences};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaSystemMaterialBinding {
    Unbound,
    Bound,
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
    pub dynamic_color_available: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaCurrentMaterialRuntime {
    pub resolved: HeptaResolvedMaterialRuntime,
    pub preferences: HeptaSystemPreferenceSnapshot,
}

impl Default for HeptaCurrentMaterialRuntime {
    fn default() -> Self {
        resolve_material_runtime_from_snapshot(
            HeptaPlatform::Unknown,
            HeptaSystemPreferenceSnapshot::fail_closed(),
            HeptaSystemMaterialBinding::Unbound,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaSystemMaterialError {
    UnsupportedPlatform,
    SystemApiUnavailable,
    UserTransparencyDisabled,
    InvalidHostHandle,
    AdapterDidNotBindSystemMaterial,
    AdapterRejectedProfile,
    SystemCallFailed(i32),
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

/// Binds one verified semantic profile to operating-system material APIs.
///
/// Implementations must reject unsupported profiles, leave stable content
/// solid, roll back partial binds, and return authority fields as `false`.
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
    fn platform(&self) -> HeptaPlatform {
        current_platform()
    }

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
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_PROMOTION: bool = false;
pub const HEPTA_SYSTEM_MATERIAL_RUNTIME_RELEASE: bool = false;

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
    binding: HeptaSystemMaterialBinding,
) -> HeptaResolvedMaterialRuntime {
    let transparency_allowed = preferences.transparency_allowed && !preferences.high_contrast;
    let profile = platform_material_profile_with_capabilities(
        platform,
        transparency_allowed,
        HeptaPlatformMaterialCapabilities {
            dynamic_color_available: preferences.dynamic_color_available,
        },
    );
    let status = if !profile.transparency_enabled {
        HeptaMaterialBindingStatus::SolidFallback
    } else if let HeptaSystemMaterialBinding::Bound = binding {
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
        dynamic_color_available: preferences.dynamic_color_available,
    }
}

pub fn resolve_material_runtime_from_snapshot(
    platform: HeptaPlatform,
    preferences: HeptaSystemPreferenceSnapshot,
    binding: HeptaSystemMaterialBinding,
) -> HeptaCurrentMaterialRuntime {
    HeptaCurrentMaterialRuntime {
        resolved: resolve_material_runtime(platform, preferences.preferences, binding),
        preferences,
    }
}

pub fn resolve_current_material_runtime(
    binding: HeptaSystemMaterialBinding,
) -> HeptaCurrentMaterialRuntime {
    resolve_material_runtime_from_snapshot(
        current_platform(),
        current_system_preferences(),
        binding,
    )
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
    let profile = platform_material_profile_with_capabilities(
        platform,
        true,
        HeptaPlatformMaterialCapabilities {
            dynamic_color_available: preferences.dynamic_color_available,
        },
    );
    if !profile.transparency_enabled {
        adapter.unbind();
        return Err(HeptaSystemMaterialError::UnsupportedPlatform);
    }

    let receipt = adapter.bind(profile)?;
    if receipt.platform != platform
        || receipt.chrome != profile.chrome
        || receipt.transient != profile.transient
        || !receipt.system_material_bound
        || receipt.production_authority
        || receipt.effect_authority
    {
        adapter.unbind();
        return Err(if !receipt.system_material_bound {
            HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial
        } else {
            HeptaSystemMaterialError::AdapterRejectedProfile
        });
    }
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transparent_preferences() -> HeptaMaterialRuntimePreferences {
        HeptaMaterialRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: true,
        }
    }

    #[test]
    fn runtime_material_resolution_is_bounded_and_fail_closed() {
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_EFFECT_AUTHORITY);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_PROMOTION);
        assert!(!HEPTA_SYSTEM_MATERIAL_RUNTIME_RELEASE);

        let intent = resolve_material_runtime(
            HeptaPlatform::Windows,
            transparent_preferences(),
            HeptaSystemMaterialBinding::Unbound,
        );
        assert_eq!(
            intent.status,
            HeptaMaterialBindingStatus::SemanticIntentOnly
        );
        assert_eq!(intent.profile.content, HeptaMaterialRenderer::Solid);
        assert!(intent.profile.max_visible_backdrop_layers <= 2);

        let high_contrast = resolve_material_runtime(
            HeptaPlatform::Windows,
            HeptaMaterialRuntimePreferences {
                high_contrast: true,
                ..transparent_preferences()
            },
            HeptaSystemMaterialBinding::Bound,
        );
        assert_eq!(
            high_contrast.status,
            HeptaMaterialBindingStatus::SolidFallback
        );
        assert!(!high_contrast.profile.transparency_enabled);

        let mut unbound = HeptaUnboundSystemMaterialAdapter;
        assert_eq!(
            bind_material_runtime(&mut unbound, transparent_preferences()),
            Err(HeptaSystemMaterialError::SystemApiUnavailable),
        );
    }

    #[test]
    fn android_requires_verified_dynamic_color() {
        let unavailable = resolve_material_runtime(
            HeptaPlatform::Android,
            HeptaMaterialRuntimePreferences {
                dynamic_color_available: false,
                ..transparent_preferences()
            },
            HeptaSystemMaterialBinding::Unbound,
        );
        assert_eq!(
            unavailable.status,
            HeptaMaterialBindingStatus::SolidFallback
        );

        let available = resolve_material_runtime(
            HeptaPlatform::Android,
            transparent_preferences(),
            HeptaSystemMaterialBinding::Unbound,
        );
        assert_eq!(
            available.status,
            HeptaMaterialBindingStatus::SemanticIntentOnly
        );
        assert_eq!(
            available.profile.chrome,
            HeptaMaterialRenderer::AndroidTonalChrome
        );
    }

    #[test]
    fn current_runtime_defaults_to_a_fail_closed_snapshot() {
        let runtime = HeptaCurrentMaterialRuntime::default();
        assert_eq!(
            runtime.resolved.status,
            HeptaMaterialBindingStatus::SolidFallback
        );
        assert!(!runtime.preferences.preferences.transparency_allowed);
    }
}
