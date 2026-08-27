//! Platform renderer decisions for Hepta UI v4 materials.
//!
//! This module maps semantic material roles to bounded platform renderer intents.
//! It invokes no operating-system API and grants no production, effect,
//! live-adapter, operator-acceptance, promotion, or release authority.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaPlatform {
    Windows,
    MacOs,
    Ios,
    Android,
    Web,
    Linux,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMaterialRenderer {
    Solid,
    WindowsMica,
    WindowsAcrylic,
    MacSystemChrome,
    MacSystemPopover,
    IosSystemGlass,
    IosSystemSheet,
    AndroidTonalChrome,
    AndroidTonalSheet,
    WebBackdropChrome,
    WebBackdropTransient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaPlatformMaterialProfile {
    pub environment: HeptaMaterialRenderer,
    pub content: HeptaMaterialRenderer,
    pub chrome: HeptaMaterialRenderer,
    pub transient: HeptaMaterialRenderer,
    pub max_visible_backdrop_layers: u8,
    pub stable_content_backdrop_layers: u8,
    pub transparency_enabled: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeptaPlatformMaterialCapabilities {
    pub dynamic_color_available: bool,
}

pub const HEPTA_PLATFORM_MATERIAL_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_PROMOTION: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_RELEASE: bool = false;

const fn solid_profile() -> HeptaPlatformMaterialProfile {
    HeptaPlatformMaterialProfile {
        environment: HeptaMaterialRenderer::Solid,
        content: HeptaMaterialRenderer::Solid,
        chrome: HeptaMaterialRenderer::Solid,
        transient: HeptaMaterialRenderer::Solid,
        max_visible_backdrop_layers: 0,
        stable_content_backdrop_layers: 0,
        transparency_enabled: false,
    }
}

pub const fn platform_material_profile(
    platform: HeptaPlatform,
    transparency_allowed: bool,
) -> HeptaPlatformMaterialProfile {
    platform_material_profile_with_capabilities(
        platform,
        transparency_allowed,
        HeptaPlatformMaterialCapabilities {
            dynamic_color_available: true,
        },
    )
}

pub const fn platform_material_profile_with_capabilities(
    platform: HeptaPlatform,
    transparency_allowed: bool,
    capabilities: HeptaPlatformMaterialCapabilities,
) -> HeptaPlatformMaterialProfile {
    if !transparency_allowed {
        return solid_profile();
    }

    let (environment, chrome, transient) = match platform {
        HeptaPlatform::Windows => (
            HeptaMaterialRenderer::WindowsMica,
            HeptaMaterialRenderer::WindowsMica,
            HeptaMaterialRenderer::WindowsAcrylic,
        ),
        HeptaPlatform::MacOs => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::MacSystemChrome,
            HeptaMaterialRenderer::MacSystemPopover,
        ),
        HeptaPlatform::Ios => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::IosSystemGlass,
            HeptaMaterialRenderer::IosSystemSheet,
        ),
        HeptaPlatform::Android if capabilities.dynamic_color_available => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::AndroidTonalChrome,
            HeptaMaterialRenderer::AndroidTonalSheet,
        ),
        HeptaPlatform::Android | HeptaPlatform::Linux | HeptaPlatform::Unknown => {
            return solid_profile();
        }
        HeptaPlatform::Web => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::WebBackdropChrome,
            HeptaMaterialRenderer::WebBackdropTransient,
        ),
    };

    HeptaPlatformMaterialProfile {
        environment,
        content: HeptaMaterialRenderer::Solid,
        chrome,
        transient,
        max_visible_backdrop_layers: 2,
        stable_content_backdrop_layers: 0,
        transparency_enabled: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_profiles_are_bounded_and_fail_closed() {
        for platform in [
            HeptaPlatform::Windows,
            HeptaPlatform::MacOs,
            HeptaPlatform::Ios,
            HeptaPlatform::Android,
            HeptaPlatform::Web,
            HeptaPlatform::Linux,
            HeptaPlatform::Unknown,
        ] {
            let profile = platform_material_profile(platform, true);
            assert_eq!(profile.content, HeptaMaterialRenderer::Solid);
            assert_eq!(profile.stable_content_backdrop_layers, 0);
            assert!(profile.max_visible_backdrop_layers <= 2);
            assert_eq!(platform_material_profile(platform, false), solid_profile());
        }
    }

    #[test]
    fn stable_content_is_always_solid() {
        for platform in [
            HeptaPlatform::Windows,
            HeptaPlatform::MacOs,
            HeptaPlatform::Ios,
            HeptaPlatform::Android,
            HeptaPlatform::Web,
            HeptaPlatform::Linux,
            HeptaPlatform::Unknown,
        ] {
            let profile = platform_material_profile(platform, true);
            assert_eq!(profile.content, HeptaMaterialRenderer::Solid);
            assert_eq!(profile.stable_content_backdrop_layers, 0);
        }
    }

    #[test]
    fn disabled_transparency_forces_solid_fallback() {
        for platform in [
            HeptaPlatform::Windows,
            HeptaPlatform::MacOs,
            HeptaPlatform::Ios,
            HeptaPlatform::Android,
            HeptaPlatform::Web,
            HeptaPlatform::Linux,
            HeptaPlatform::Unknown,
        ] {
            assert_eq!(platform_material_profile(platform, false), solid_profile());
        }
    }

    #[test]
    fn android_dynamic_color_unavailable_forces_solid_fallback() {
        let profile = platform_material_profile_with_capabilities(
            HeptaPlatform::Android,
            true,
            HeptaPlatformMaterialCapabilities {
                dynamic_color_available: false,
            },
        );
        assert_eq!(profile, solid_profile());
    }

    #[test]
    fn semantic_resolution_never_grants_authority() {
        assert!(!HEPTA_PLATFORM_MATERIAL_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_EFFECT_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_PLATFORM_MATERIAL_PROMOTION);
        assert!(!HEPTA_PLATFORM_MATERIAL_RELEASE);
    }
}
