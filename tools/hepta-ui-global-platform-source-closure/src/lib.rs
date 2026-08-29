#![forbid(unsafe_code)]

#[path = "../../../apps/hepta-native/src/shared/hepta_platform_material.rs"]
pub mod hepta_platform_material;

/// Safe, host-independent preference shim for the isolated source-closure harness.
///
/// The product module contains target FFI probes. This qualification crate tests
/// material resolution and adapter contracts only, so it must not compile or
/// execute UIKit, Win32, AppKit, or JNI discovery code. Every implicit probe is
/// therefore fail-closed; tests may still inject an explicit host snapshot.
pub mod hepta_system_preferences {
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
                transparency_allowed: false,
                high_contrast: false,
                reduced_motion: true,
                dynamic_color_available: false,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum HeptaSystemPreferenceSource {
        WindowsSystem,
        IosAccessibility,
        HostProvided,
        Unavailable,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum HeptaSystemPreferenceProbeStatus {
        Verified,
        Partial,
        HostProvided,
        Unavailable,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct HeptaSystemPreferenceSnapshot {
        pub preferences: HeptaMaterialRuntimePreferences,
        pub source: HeptaSystemPreferenceSource,
        pub status: HeptaSystemPreferenceProbeStatus,
    }

    impl Default for HeptaSystemPreferenceSnapshot {
        fn default() -> Self {
            Self::fail_closed()
        }
    }

    impl HeptaSystemPreferenceSnapshot {
        pub const fn fail_closed() -> Self {
            Self {
                preferences: HeptaMaterialRuntimePreferences {
                    transparency_allowed: false,
                    high_contrast: false,
                    reduced_motion: true,
                    dynamic_color_available: false,
                },
                source: HeptaSystemPreferenceSource::Unavailable,
                status: HeptaSystemPreferenceProbeStatus::Unavailable,
            }
        }

        pub const fn from_host(preferences: HeptaMaterialRuntimePreferences) -> Self {
            Self {
                preferences,
                source: HeptaSystemPreferenceSource::HostProvided,
                status: HeptaSystemPreferenceProbeStatus::HostProvided,
            }
        }
    }

    pub trait HeptaSystemPreferenceProbe {
        fn probe(&self) -> HeptaSystemPreferenceSnapshot;
    }

    #[derive(Default)]
    pub struct HeptaFailClosedPreferenceProbe;

    impl HeptaSystemPreferenceProbe for HeptaFailClosedPreferenceProbe {
        fn probe(&self) -> HeptaSystemPreferenceSnapshot {
            HeptaSystemPreferenceSnapshot::fail_closed()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct HeptaHostPreferenceProbe {
        snapshot: HeptaSystemPreferenceSnapshot,
    }

    impl HeptaHostPreferenceProbe {
        pub const fn new(preferences: HeptaMaterialRuntimePreferences) -> Self {
            Self {
                snapshot: HeptaSystemPreferenceSnapshot::from_host(preferences),
            }
        }
    }

    impl HeptaSystemPreferenceProbe for HeptaHostPreferenceProbe {
        fn probe(&self) -> HeptaSystemPreferenceSnapshot {
            self.snapshot
        }
    }

    pub const HEPTA_SYSTEM_PREFERENCE_NETWORK_AUTHORITY: bool = false;
    pub const HEPTA_SYSTEM_PREFERENCE_EFFECT_AUTHORITY: bool = false;
    pub const HEPTA_SYSTEM_PREFERENCE_PRODUCTION_AUTHORITY: bool = false;
    pub const HEPTA_SYSTEM_PREFERENCE_PROMOTION: bool = false;
    pub const HEPTA_SYSTEM_PREFERENCE_RELEASE: bool = false;

    pub const fn current_system_preferences() -> HeptaSystemPreferenceSnapshot {
        HeptaSystemPreferenceSnapshot::fail_closed()
    }
}

#[path = "../../../apps/hepta-native/src/shared/hepta_platform_material_runtime.rs"]
pub mod hepta_platform_material_runtime;
#[path = "../../../apps/hepta-native/src/shared/hepta_macos_appkit_material_adapter.rs"]
pub mod hepta_macos_appkit_material_adapter;
#[path = "../../../apps/hepta-native/src/shared/hepta_ios_uikit_material_adapter.rs"]
pub mod hepta_ios_uikit_material_adapter;
#[path = "../../../apps/hepta-native/src/shared/hepta_android_material3_activity_bridge.rs"]
pub mod hepta_android_material3_activity_bridge;
#[path = "../../../apps/hepta-native/src/shared/hepta_ui_explicit_activation_caller.rs"]
pub mod hepta_ui_explicit_activation_caller;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_source_closure_constants_remain_non_authoritative() {
        assert!(hepta_macos_appkit_material_adapter::HEPTA_MACOS_APPKIT_SOURCE_IMPLEMENTED);
        assert!(hepta_ios_uikit_material_adapter::HEPTA_IOS_UIKIT_SOURCE_IMPLEMENTED);
        assert!(hepta_android_material3_activity_bridge::HEPTA_ANDROID_MATERIAL3_SOURCE_IMPLEMENTED);
        assert!(hepta_ui_explicit_activation_caller::HEPTA_UI_EXPLICIT_CALLER_SOURCE_IMPLEMENTED);
        assert!(!hepta_macos_appkit_material_adapter::HEPTA_MACOS_APPKIT_PRODUCT_BOUND);
        assert!(!hepta_ios_uikit_material_adapter::HEPTA_IOS_UIKIT_PRODUCT_BOUND);
        assert!(!hepta_android_material3_activity_bridge::HEPTA_ANDROID_MATERIAL3_PRODUCT_BOUND);
        assert!(!hepta_ui_explicit_activation_caller::HEPTA_UI_EXPLICIT_CALLER_REGISTERED);
        assert_eq!(
            hepta_system_preferences::current_system_preferences(),
            hepta_system_preferences::HeptaSystemPreferenceSnapshot::fail_closed()
        );
    }
}
