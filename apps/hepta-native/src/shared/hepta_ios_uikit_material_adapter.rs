//! Transactional UIKit navigation/sheet material adapter contract for Hepta UI v4.
//!
//! The host supplies exact UIView/controller identities and bounded layout metrics.
//! The adapter verifies accessibility and Dynamic Type inputs, applies separate
//! navigation and sheet materials, rolls back partial binds, and grants no authority.

use super::hepta_platform_material::{HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile};
use super::hepta_platform_material_runtime::{HeptaSystemMaterialAdapter, HeptaSystemMaterialError, HeptaSystemMaterialReceipt};

pub const HEPTA_IOS_UIKIT_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_IOS_UIKIT_PRODUCT_BOUND: bool = false;
pub const HEPTA_IOS_UIKIT_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_IOS_UIKIT_DEVICE_VALIDATED: bool = false;
pub const HEPTA_IOS_UIKIT_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_IOS_UIKIT_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_IOS_UIKIT_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_IOS_UIKIT_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_IOS_UIKIT_PROMOTION: bool = false;
pub const HEPTA_IOS_UIKIT_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaIosUIKitMaterial { Solid, NavigationGlass, SystemSheet }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaIosUIKitHostIdentity {
    pub navigation_view: isize,
    pub sheet_view_controller: isize,
}
impl HeptaIosUIKitHostIdentity {
    pub const fn is_valid(self) -> bool {
        self.navigation_view != 0 && self.sheet_view_controller != 0 && self.navigation_view != self.sheet_view_controller
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaIosUIKitEnvironment {
    pub reduce_transparency: bool,
    pub darker_system_colors: bool,
    pub reduce_motion: bool,
    pub dynamic_type_scale: f32,
    pub safe_area_top: f32,
    pub safe_area_bottom: f32,
    pub keyboard_inset: f32,
    pub verified: bool,
}
impl HeptaIosUIKitEnvironment {
    pub fn is_valid(self) -> bool {
        self.verified
            && self.dynamic_type_scale.is_finite()
            && (1.0..=2.5).contains(&self.dynamic_type_scale)
            && [self.safe_area_top, self.safe_area_bottom, self.keyboard_inset]
                .into_iter().all(|value| value.is_finite() && value >= 0.0)
    }
    pub fn permits_material(self) -> bool {
        self.is_valid() && !self.reduce_transparency && !self.darker_system_colors
    }
}

pub trait HeptaIosUIKitApi {
    fn environment(&mut self) -> Result<HeptaIosUIKitEnvironment, i32>;
    fn apply_material(&mut self, host: isize, material: HeptaIosUIKitMaterial) -> Result<(), i32>;
    fn observe_material(&mut self, host: isize) -> Result<HeptaIosUIKitMaterial, i32>;
}

pub struct HeptaIosUIKitAdapter<A> { host: HeptaIosUIKitHostIdentity, api: A, bound: bool }
impl<A> HeptaIosUIKitAdapter<A> {
    pub const fn new(host: HeptaIosUIKitHostIdentity, api: A) -> Self { Self { host, api, bound: false } }
    pub const fn is_bound(&self) -> bool { self.bound }
    fn rollback(&mut self) where A: HeptaIosUIKitApi {
        let _ = self.api.apply_material(self.host.sheet_view_controller, HeptaIosUIKitMaterial::Solid);
        let _ = self.api.apply_material(self.host.navigation_view, HeptaIosUIKitMaterial::Solid);
        self.bound = false;
    }
}

impl<A: HeptaIosUIKitApi> HeptaSystemMaterialAdapter for HeptaIosUIKitAdapter<A> {
    fn platform(&self) -> HeptaPlatform { HeptaPlatform::Ios }
    fn bind(&mut self, profile: HeptaPlatformMaterialProfile) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
        if !self.host.is_valid() { return Err(HeptaSystemMaterialError::InvalidHostHandle); }
        if profile.content != HeptaMaterialRenderer::Solid
            || profile.chrome != HeptaMaterialRenderer::IosSystemGlass
            || profile.transient != HeptaMaterialRenderer::IosSystemSheet
            || profile.stable_content_backdrop_layers != 0
            || profile.max_visible_backdrop_layers > 2 {
            return Err(HeptaSystemMaterialError::AdapterRejectedProfile);
        }
        let environment = self.api.environment().map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if !environment.is_valid() { self.rollback(); return Err(HeptaSystemMaterialError::SystemApiUnavailable); }
        if !environment.permits_material() { self.rollback(); return Err(HeptaSystemMaterialError::UserTransparencyDisabled); }
        self.api.apply_material(self.host.navigation_view, HeptaIosUIKitMaterial::NavigationGlass)
            .map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if let Err(error) = self.api.apply_material(self.host.sheet_view_controller, HeptaIosUIKitMaterial::SystemSheet) {
            self.rollback(); return Err(HeptaSystemMaterialError::SystemCallFailed(error));
        }
        let exact = self.api.observe_material(self.host.navigation_view).ok() == Some(HeptaIosUIKitMaterial::NavigationGlass)
            && self.api.observe_material(self.host.sheet_view_controller).ok() == Some(HeptaIosUIKitMaterial::SystemSheet);
        if !exact { self.rollback(); return Err(HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial); }
        self.bound = true;
        Ok(HeptaSystemMaterialReceipt { platform: HeptaPlatform::Ios, chrome: HeptaMaterialRenderer::IosSystemGlass, transient: HeptaMaterialRenderer::IosSystemSheet, system_material_bound: true, production_authority: false, effect_authority: false })
    }
    fn unbind(&mut self) { self.rollback(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hepta_platform_material::{HeptaPlatformMaterialCapabilities, platform_material_profile_with_capabilities};
    #[derive(Default)] struct FakeApi { nav: Option<HeptaIosUIKitMaterial>, sheet: Option<HeptaIosUIKitMaterial>, env: Option<HeptaIosUIKitEnvironment>, fail_sheet: bool }
    impl HeptaIosUIKitApi for FakeApi {
        fn environment(&mut self)->Result<HeptaIosUIKitEnvironment,i32>{self.env.ok_or(-1)}
        fn apply_material(&mut self, host:isize, material:HeptaIosUIKitMaterial)->Result<(),i32>{if host==12 && self.fail_sheet{return Err(-2)}; if host==11{self.nav=Some(material)}else if host==12{self.sheet=Some(material)}else{return Err(-3)};Ok(())}
        fn observe_material(&mut self,host:isize)->Result<HeptaIosUIKitMaterial,i32>{match host{11=>self.nav.ok_or(-4),12=>self.sheet.ok_or(-4),_=>Err(-5)}}
    }
    fn env()->HeptaIosUIKitEnvironment{HeptaIosUIKitEnvironment{reduce_transparency:false,darker_system_colors:false,reduce_motion:false,dynamic_type_scale:2.0,safe_area_top:47.0,safe_area_bottom:34.0,keyboard_inset:0.0,verified:true}}
    fn profile()->HeptaPlatformMaterialProfile{platform_material_profile_with_capabilities(HeptaPlatform::Ios,true,HeptaPlatformMaterialCapabilities::default())}
    #[test] fn ios_binds_and_rolls_back(){let mut a=HeptaIosUIKitAdapter::new(HeptaIosUIKitHostIdentity{navigation_view:11,sheet_view_controller:12},FakeApi{env:Some(env()),..FakeApi::default()});assert!(a.bind(profile()).unwrap().system_material_bound);a.unbind();assert!(!a.is_bound());}
    #[test] fn ios_accessibility_and_partial_bind_fail_closed(){let mut denied=env();denied.reduce_transparency=true;let mut a=HeptaIosUIKitAdapter::new(HeptaIosUIKitHostIdentity{navigation_view:11,sheet_view_controller:12},FakeApi{env:Some(denied),..FakeApi::default()});assert_eq!(a.bind(profile()),Err(HeptaSystemMaterialError::UserTransparencyDisabled));let mut a=HeptaIosUIKitAdapter::new(HeptaIosUIKitHostIdentity{navigation_view:11,sheet_view_controller:12},FakeApi{env:Some(env()),fail_sheet:true,..FakeApi::default()});assert!(a.bind(profile()).is_err());assert!(!a.is_bound());}
}
