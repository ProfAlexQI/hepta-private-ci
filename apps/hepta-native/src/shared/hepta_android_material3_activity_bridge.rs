//! Android Material 3 Activity/JNI bridge contract for Hepta UI v4.
//!
//! The bridge never copies iOS blur semantics. It requires an exact Activity
//! instance, verified dynamic-color availability, bounded animator scale, and
//! separate chrome/sheet views. Missing capability fails closed to solid.

use super::hepta_platform_material::{HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile};
use super::hepta_platform_material_runtime::{HeptaSystemMaterialAdapter, HeptaSystemMaterialError, HeptaSystemMaterialReceipt};

pub const HEPTA_ANDROID_MATERIAL3_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_ANDROID_ACTIVITY_JNI_BRIDGE_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_ANDROID_MATERIAL3_PRODUCT_BOUND: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_DEVICE_VALIDATED: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_PROMOTION: bool = false;
pub const HEPTA_ANDROID_MATERIAL3_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaAndroidTonalRole { Solid, Chrome, Sheet }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaAndroidActivityIdentity {
    pub activity_instance_id: u64,
    pub chrome_view_id: i32,
    pub sheet_view_id: i32,
}
impl HeptaAndroidActivityIdentity {
    pub const fn is_valid(self) -> bool {
        self.activity_instance_id != 0 && self.chrome_view_id > 0 && self.sheet_view_id > 0 && self.chrome_view_id != self.sheet_view_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaAndroidMaterial3Environment {
    pub dynamic_color_available: bool,
    pub animator_duration_scale: f32,
    pub ime_inset: u32,
    pub navigation_inset: u32,
    pub verified: bool,
}
impl HeptaAndroidMaterial3Environment {
    pub fn is_valid(self) -> bool {
        self.verified && self.animator_duration_scale.is_finite() && (0.0..=10.0).contains(&self.animator_duration_scale)
    }
}

/// Activity/JNI surface. A target implementation should use the exact Activity
/// and view references supplied by the Android host. It must not discover a
/// foreground Activity or hold an unbounded local JNI reference.
pub trait HeptaAndroidMaterial3ActivityApi {
    fn environment(&mut self, activity_instance_id: u64) -> Result<HeptaAndroidMaterial3Environment, i32>;
    fn apply_dynamic_colors(&mut self, activity_instance_id: u64) -> Result<(), i32>;
    fn apply_role(&mut self, view_id: i32, role: HeptaAndroidTonalRole) -> Result<(), i32>;
    fn observe_role(&mut self, view_id: i32) -> Result<HeptaAndroidTonalRole, i32>;
}

pub struct HeptaAndroidMaterial3Adapter<A> { identity: HeptaAndroidActivityIdentity, api: A, bound: bool }
impl<A> HeptaAndroidMaterial3Adapter<A> {
    pub const fn new(identity: HeptaAndroidActivityIdentity, api: A) -> Self { Self { identity, api, bound: false } }
    pub const fn is_bound(&self)->bool{self.bound}
    fn rollback(&mut self) where A: HeptaAndroidMaterial3ActivityApi {
        let _=self.api.apply_role(self.identity.sheet_view_id,HeptaAndroidTonalRole::Solid);
        let _=self.api.apply_role(self.identity.chrome_view_id,HeptaAndroidTonalRole::Solid);
        self.bound=false;
    }
}
impl<A:HeptaAndroidMaterial3ActivityApi> HeptaSystemMaterialAdapter for HeptaAndroidMaterial3Adapter<A>{
    fn platform(&self)->HeptaPlatform{HeptaPlatform::Android}
    fn bind(&mut self,profile:HeptaPlatformMaterialProfile)->Result<HeptaSystemMaterialReceipt,HeptaSystemMaterialError>{
        if !self.identity.is_valid(){return Err(HeptaSystemMaterialError::InvalidHostHandle)}
        if profile.content!=HeptaMaterialRenderer::Solid||profile.chrome!=HeptaMaterialRenderer::AndroidTonalChrome||profile.transient!=HeptaMaterialRenderer::AndroidTonalSheet||profile.stable_content_backdrop_layers!=0||profile.max_visible_backdrop_layers>2{return Err(HeptaSystemMaterialError::AdapterRejectedProfile)}
        let env=self.api.environment(self.identity.activity_instance_id).map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if !env.is_valid(){self.rollback();return Err(HeptaSystemMaterialError::SystemApiUnavailable)}
        if !env.dynamic_color_available{self.rollback();return Err(HeptaSystemMaterialError::UnsupportedPlatform)}
        self.api.apply_dynamic_colors(self.identity.activity_instance_id).map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        self.api.apply_role(self.identity.chrome_view_id,HeptaAndroidTonalRole::Chrome).map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if let Err(error)=self.api.apply_role(self.identity.sheet_view_id,HeptaAndroidTonalRole::Sheet){self.rollback();return Err(HeptaSystemMaterialError::SystemCallFailed(error))}
        let exact=self.api.observe_role(self.identity.chrome_view_id).ok()==Some(HeptaAndroidTonalRole::Chrome)&&self.api.observe_role(self.identity.sheet_view_id).ok()==Some(HeptaAndroidTonalRole::Sheet);
        if !exact{self.rollback();return Err(HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial)}
        self.bound=true;
        Ok(HeptaSystemMaterialReceipt{platform:HeptaPlatform::Android,chrome:HeptaMaterialRenderer::AndroidTonalChrome,transient:HeptaMaterialRenderer::AndroidTonalSheet,system_material_bound:true,production_authority:false,effect_authority:false})
    }
    fn unbind(&mut self){self.rollback()}
}

#[cfg(target_os="android")]
pub mod jni_contract {
    pub const DYNAMIC_COLORS_CLASS: &str = "com/google/android/material/color/DynamicColors";
    pub const IS_DYNAMIC_COLOR_AVAILABLE: &str = "isDynamicColorAvailable";
    pub const APPLY_TO_ACTIVITY_IF_AVAILABLE: &str = "applyToActivityIfAvailable";
    pub const SETTINGS_GLOBAL_CLASS: &str = "android/provider/Settings$Global";
    pub const ANIMATOR_DURATION_SCALE: &str = "animator_duration_scale";
    pub const SET_BACKGROUND_COLOR: &str = "setBackgroundColor";
}

#[cfg(test)]
mod tests{
    use super::*;use super::super::hepta_platform_material::{HeptaPlatformMaterialCapabilities,platform_material_profile_with_capabilities};
    #[derive(Default)]struct Fake{env:Option<HeptaAndroidMaterial3Environment>,chrome:Option<HeptaAndroidTonalRole>,sheet:Option<HeptaAndroidTonalRole>,fail_sheet:bool}
    impl HeptaAndroidMaterial3ActivityApi for Fake{fn environment(&mut self,_:u64)->Result<HeptaAndroidMaterial3Environment,i32>{self.env.ok_or(-1)}fn apply_dynamic_colors(&mut self,_:u64)->Result<(),i32>{Ok(())}fn apply_role(&mut self,id:i32,role:HeptaAndroidTonalRole)->Result<(),i32>{if id==12&&self.fail_sheet{return Err(-2)}if id==11{self.chrome=Some(role)}else if id==12{self.sheet=Some(role)}else{return Err(-3)}Ok(())}fn observe_role(&mut self,id:i32)->Result<HeptaAndroidTonalRole,i32>{match id{11=>self.chrome.ok_or(-4),12=>self.sheet.ok_or(-4),_=>Err(-5)}}}
    fn env(dynamic:bool)->HeptaAndroidMaterial3Environment{HeptaAndroidMaterial3Environment{dynamic_color_available:dynamic,animator_duration_scale:1.0,ime_inset:0,navigation_inset:24,verified:true}}
    fn profile()->HeptaPlatformMaterialProfile{platform_material_profile_with_capabilities(HeptaPlatform::Android,true,HeptaPlatformMaterialCapabilities{dynamic_color_available:true})}
    #[test]fn android_binds_tonal_roles_without_blur(){let mut a=HeptaAndroidMaterial3Adapter::new(HeptaAndroidActivityIdentity{activity_instance_id:7,chrome_view_id:11,sheet_view_id:12},Fake{env:Some(env(true)),..Fake::default()});assert!(a.bind(profile()).unwrap().system_material_bound);a.unbind();assert!(!a.is_bound())}
    #[test]fn unavailable_dynamic_color_and_partial_apply_fail_closed(){let mut a=HeptaAndroidMaterial3Adapter::new(HeptaAndroidActivityIdentity{activity_instance_id:7,chrome_view_id:11,sheet_view_id:12},Fake{env:Some(env(false)),..Fake::default()});assert_eq!(a.bind(profile()),Err(HeptaSystemMaterialError::UnsupportedPlatform));let mut a=HeptaAndroidMaterial3Adapter::new(HeptaAndroidActivityIdentity{activity_instance_id:7,chrome_view_id:11,sheet_view_id:12},Fake{env:Some(env(true)),fail_sheet:true,..Fake::default()});assert!(a.bind(profile()).is_err());assert!(!a.is_bound())}
}
