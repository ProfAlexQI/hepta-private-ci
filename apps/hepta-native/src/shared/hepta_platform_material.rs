//! Platform renderer decisions for Hepta UI v4 materials.
//!
//! This module maps semantic material roles to bounded platform renderer intents.
//! It invokes no operating-system API and grants no production, effect,
//! live-adapter, operator-acceptance, or promotion authority.

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

pub const HEPTA_PLATFORM_MATERIAL_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_PLATFORM_MATERIAL_PROMOTION: bool = false;

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
        HeptaPlatform::Android => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::AndroidTonalChrome,
            HeptaMaterialRenderer::AndroidTonalSheet,
        ),
        HeptaPlatform::Web => (
            HeptaMaterialRenderer::Solid,
            HeptaMaterialRenderer::WebBackdropChrome,
            HeptaMaterialRenderer::WebBackdropTransient,
        ),
        HeptaPlatform::Linux | HeptaPlatform::Unknown => return solid_profile(),
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
        assert!(!HEPTA_PLATFORM_MATERIAL_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_EFFECT_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_PLATFORM_MATERIAL_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_PLATFORM_MATERIAL_PROMOTION);

        for platform in [
            HeptaPlatform::Windows,
            HeptaPlatform::MacOs,
            HeptaPlatform::Ios,
            HeptaPlatform::Android,
            HeptaPlatform::Web,
            HeptaPlatform::Linux,
            HeptaPlatform::Unknown,
        ] {
            let transparent = platform_material_profile(platform, true);
            assert_eq!(transparent.content, HeptaMaterialRenderer::Solid);
            assert_eq!(transparent.stable_content_backdrop_layers, 0);
            assert!(transparent.max_visible_backdrop_layers <= 2);
            assert_eq!(platform_material_profile(platform, false), solid_profile());
        }
    }
}
