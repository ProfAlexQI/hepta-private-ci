#![forbid(unsafe_code)]

#[path = "../../../apps/hepta-native/src/shared/hepta_android_material3_activity_bridge.rs"]
pub mod hepta_android_material3_activity_bridge;
#[path = "../../../apps/hepta-native/src/shared/hepta_ios_uikit_material_adapter.rs"]
pub mod hepta_ios_uikit_material_adapter;
#[path = "../../../apps/hepta-native/src/shared/hepta_macos_appkit_material_adapter.rs"]
pub mod hepta_macos_appkit_material_adapter;
#[path = "../../../apps/hepta-native/src/shared/hepta_platform_material.rs"]
pub mod hepta_platform_material;
#[path = "../../../apps/hepta-native/src/shared/hepta_platform_material_runtime.rs"]
pub mod hepta_platform_material_runtime;
#[path = "../../../apps/hepta-native/src/shared/hepta_system_preferences.rs"]
pub mod hepta_system_preferences;
#[path = "../../../apps/hepta-native/src/shared/hepta_ui_explicit_activation_caller.rs"]
pub mod hepta_ui_explicit_activation_caller;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_source_closure_constants_remain_non_authoritative() {
        assert!(hepta_macos_appkit_material_adapter::HEPTA_MACOS_APPKIT_SOURCE_IMPLEMENTED);
        assert!(hepta_ios_uikit_material_adapter::HEPTA_IOS_UIKIT_SOURCE_IMPLEMENTED);
        assert!(
            hepta_android_material3_activity_bridge::HEPTA_ANDROID_MATERIAL3_SOURCE_IMPLEMENTED
        );
        assert!(hepta_ui_explicit_activation_caller::HEPTA_UI_EXPLICIT_CALLER_SOURCE_IMPLEMENTED);
        assert!(!hepta_macos_appkit_material_adapter::HEPTA_MACOS_APPKIT_PRODUCT_BOUND);
        assert!(!hepta_ios_uikit_material_adapter::HEPTA_IOS_UIKIT_PRODUCT_BOUND);
        assert!(!hepta_android_material3_activity_bridge::HEPTA_ANDROID_MATERIAL3_PRODUCT_BOUND);
        assert!(!hepta_ui_explicit_activation_caller::HEPTA_UI_EXPLICIT_CALLER_REGISTERED);
    }
}
