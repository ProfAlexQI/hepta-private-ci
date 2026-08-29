//! Transactional macOS AppKit material adapter contract for Hepta UI v4.
//!
//! AppKit object discovery remains host-owned. The adapter accepts exact non-zero
//! NSView identities, validates accessibility preferences, applies chrome and
//! transient materials transactionally, verifies scoped observations, and rolls
//! back to solid on every partial or rejected bind. It grants no authority.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile,
};
use super::hepta_platform_material_runtime::{
    HeptaSystemMaterialAdapter, HeptaSystemMaterialError, HeptaSystemMaterialReceipt,
};

pub const HEPTA_MACOS_APPKIT_SOURCE_IMPLEMENTED: bool = true;
pub const HEPTA_MACOS_APPKIT_PRODUCT_BOUND: bool = false;
pub const HEPTA_MACOS_APPKIT_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_MACOS_APPKIT_DEVICE_VALIDATED: bool = false;
pub const HEPTA_MACOS_APPKIT_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_MACOS_APPKIT_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_MACOS_APPKIT_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_MACOS_APPKIT_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_MACOS_APPKIT_PROMOTION: bool = false;
pub const HEPTA_MACOS_APPKIT_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMacosAppKitMaterial {
    Solid,
    Sidebar,
    Popover,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMacosAppKitHostIdentity {
    pub chrome_view: isize,
    pub transient_view: isize,
}

impl HeptaMacosAppKitHostIdentity {
    pub const fn is_valid(self) -> bool {
        self.chrome_view != 0 && self.transient_view != 0 && self.chrome_view != self.transient_view
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMacosAccessibilitySnapshot {
    pub reduce_transparency: bool,
    pub increase_contrast: bool,
    pub reduce_motion: bool,
    pub window_active: bool,
    pub verified: bool,
}

impl HeptaMacosAccessibilitySnapshot {
    pub const fn permits_material(self) -> bool {
        self.verified && !self.reduce_transparency && !self.increase_contrast
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaMacosAppKitError {
    InvalidHost,
    PreferenceUnavailable,
    PreferenceRequiresSolid,
    ApplyFailed(i32),
    ObservationFailed(i32),
    ObservationMismatch,
}

/// Host-owned AppKit bridge. Implementations should use NSVisualEffectView
/// material/state/blending APIs on the main thread and must never discover a
/// different window or view by title, global enumeration, or foreground state.
pub trait HeptaMacosAppKitApi {
    fn preferences(&mut self) -> Result<HeptaMacosAccessibilitySnapshot, i32>;
    fn apply_material(
        &mut self,
        view: isize,
        material: HeptaMacosAppKitMaterial,
        active: bool,
    ) -> Result<(), i32>;
    fn observe_material(&mut self, view: isize) -> Result<HeptaMacosAppKitMaterial, i32>;
}

pub struct HeptaMacosAppKitAdapter<A> {
    host: HeptaMacosAppKitHostIdentity,
    api: A,
    bound: bool,
}

impl<A> HeptaMacosAppKitAdapter<A> {
    pub const fn new(host: HeptaMacosAppKitHostIdentity, api: A) -> Self {
        Self {
            host,
            api,
            bound: false,
        }
    }

    pub const fn is_bound(&self) -> bool {
        self.bound
    }

    pub fn scoped_receipt(&mut self) -> Result<HeptaMacosAppKitScopedReceipt, HeptaMacosAppKitError>
    where
        A: HeptaMacosAppKitApi,
    {
        if !self.host.is_valid() {
            return Err(HeptaMacosAppKitError::InvalidHost);
        }
        let chrome = self
            .api
            .observe_material(self.host.chrome_view)
            .map_err(HeptaMacosAppKitError::ObservationFailed)?;
        let transient = self
            .api
            .observe_material(self.host.transient_view)
            .map_err(HeptaMacosAppKitError::ObservationFailed)?;
        let exact = chrome == HeptaMacosAppKitMaterial::Sidebar
            && transient == HeptaMacosAppKitMaterial::Popover;
        Ok(HeptaMacosAppKitScopedReceipt {
            chrome,
            transient,
            exact,
            complete_profile_bound: false,
            product_bound: false,
            production_authority: false,
            effect_authority: false,
        })
    }

    fn rollback(&mut self)
    where
        A: HeptaMacosAppKitApi,
    {
        let _ = self.api.apply_material(
            self.host.transient_view,
            HeptaMacosAppKitMaterial::Solid,
            false,
        );
        let _ = self.api.apply_material(
            self.host.chrome_view,
            HeptaMacosAppKitMaterial::Solid,
            false,
        );
        self.bound = false;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaMacosAppKitScopedReceipt {
    pub chrome: HeptaMacosAppKitMaterial,
    pub transient: HeptaMacosAppKitMaterial,
    pub exact: bool,
    pub complete_profile_bound: bool,
    pub product_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
}

impl<A: HeptaMacosAppKitApi> HeptaSystemMaterialAdapter for HeptaMacosAppKitAdapter<A> {
    fn platform(&self) -> HeptaPlatform {
        HeptaPlatform::MacOs
    }

    fn bind(
        &mut self,
        profile: HeptaPlatformMaterialProfile,
    ) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
        if !self.host.is_valid() {
            return Err(HeptaSystemMaterialError::InvalidHostHandle);
        }
        if profile.content != HeptaMaterialRenderer::Solid
            || profile.chrome != HeptaMaterialRenderer::MacSystemChrome
            || profile.transient != HeptaMaterialRenderer::MacSystemPopover
            || profile.stable_content_backdrop_layers != 0
            || profile.max_visible_backdrop_layers > 2
        {
            return Err(HeptaSystemMaterialError::AdapterRejectedProfile);
        }
        let preferences = self
            .api
            .preferences()
            .map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if !preferences.verified {
            self.rollback();
            return Err(HeptaSystemMaterialError::SystemApiUnavailable);
        }
        if !preferences.permits_material() {
            self.rollback();
            return Err(HeptaSystemMaterialError::UserTransparencyDisabled);
        }
        self.api
            .apply_material(
                self.host.chrome_view,
                HeptaMacosAppKitMaterial::Sidebar,
                preferences.window_active,
            )
            .map_err(HeptaSystemMaterialError::SystemCallFailed)?;
        if let Err(error) = self.api.apply_material(
            self.host.transient_view,
            HeptaMacosAppKitMaterial::Popover,
            preferences.window_active,
        ) {
            self.rollback();
            return Err(HeptaSystemMaterialError::SystemCallFailed(error));
        }
        let scoped = match self.scoped_receipt() {
            Ok(receipt) if receipt.exact => receipt,
            _ => {
                self.rollback();
                return Err(HeptaSystemMaterialError::AdapterDidNotBindSystemMaterial);
            }
        };
        debug_assert!(scoped.exact);
        self.bound = true;
        Ok(HeptaSystemMaterialReceipt {
            platform: HeptaPlatform::MacOs,
            chrome: HeptaMaterialRenderer::MacSystemChrome,
            transient: HeptaMaterialRenderer::MacSystemPopover,
            system_material_bound: true,
            production_authority: false,
            effect_authority: false,
        })
    }

    fn unbind(&mut self) {
        self.rollback();
    }
}

#[cfg(test)]
mod tests {
    use super::super::hepta_platform_material::{
        HeptaPlatformMaterialCapabilities, platform_material_profile_with_capabilities,
    };
    use super::*;

    #[derive(Default)]
    struct FakeApi {
        chrome: HeptaMacosAppKitMaterial,
        transient: HeptaMacosAppKitMaterial,
        reject_popover: bool,
        preferences: Option<HeptaMacosAccessibilitySnapshot>,
    }

    impl Default for HeptaMacosAppKitMaterial {
        fn default() -> Self {
            Self::Solid
        }
    }

    impl HeptaMacosAppKitApi for FakeApi {
        fn preferences(&mut self) -> Result<HeptaMacosAccessibilitySnapshot, i32> {
            self.preferences.ok_or(-1)
        }
        fn apply_material(
            &mut self,
            view: isize,
            material: HeptaMacosAppKitMaterial,
            _active: bool,
        ) -> Result<(), i32> {
            if material == HeptaMacosAppKitMaterial::Popover && self.reject_popover {
                return Err(-2);
            }
            if view == 11 {
                self.chrome = material;
            } else if view == 12 {
                self.transient = material;
            } else {
                return Err(-3);
            }
            Ok(())
        }
        fn observe_material(&mut self, view: isize) -> Result<HeptaMacosAppKitMaterial, i32> {
            match view {
                11 => Ok(self.chrome),
                12 => Ok(self.transient),
                _ => Err(-4),
            }
        }
    }

    fn profile() -> HeptaPlatformMaterialProfile {
        platform_material_profile_with_capabilities(
            HeptaPlatform::MacOs,
            true,
            HeptaPlatformMaterialCapabilities::default(),
        )
    }

    fn allowed() -> HeptaMacosAccessibilitySnapshot {
        HeptaMacosAccessibilitySnapshot {
            reduce_transparency: false,
            increase_contrast: false,
            reduce_motion: false,
            window_active: true,
            verified: true,
        }
    }

    #[test]
    fn macos_binds_exact_roles_transactionally() {
        let mut adapter = HeptaMacosAppKitAdapter::new(
            HeptaMacosAppKitHostIdentity {
                chrome_view: 11,
                transient_view: 12,
            },
            FakeApi {
                preferences: Some(allowed()),
                ..FakeApi::default()
            },
        );
        let receipt = adapter.bind(profile()).unwrap();
        assert!(receipt.system_material_bound);
        assert!(adapter.scoped_receipt().unwrap().exact);
        adapter.unbind();
        assert!(!adapter.is_bound());
    }

    #[test]
    fn preference_or_partial_failure_rolls_back() {
        let mut adapter = HeptaMacosAppKitAdapter::new(
            HeptaMacosAppKitHostIdentity {
                chrome_view: 11,
                transient_view: 12,
            },
            FakeApi {
                preferences: Some(HeptaMacosAccessibilitySnapshot {
                    reduce_transparency: true,
                    ..allowed()
                }),
                ..FakeApi::default()
            },
        );
        assert_eq!(
            adapter.bind(profile()),
            Err(HeptaSystemMaterialError::UserTransparencyDisabled)
        );
        let mut adapter = HeptaMacosAppKitAdapter::new(
            HeptaMacosAppKitHostIdentity {
                chrome_view: 11,
                transient_view: 12,
            },
            FakeApi {
                preferences: Some(allowed()),
                reject_popover: true,
                ..FakeApi::default()
            },
        );
        assert!(adapter.bind(profile()).is_err());
        assert!(!adapter.is_bound());
    }
}
