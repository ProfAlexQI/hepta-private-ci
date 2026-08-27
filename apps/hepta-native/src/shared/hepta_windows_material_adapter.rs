//! Windows 11 DWM material adapter for Hepta UI v4.
//!
//! The adapter requires explicit, non-zero host window handles. It binds Mica to
//! the persistent chrome window and transient Acrylic to a separate transient
//! host window. If either call fails, the adapter rolls back and reports no
//! material binding.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile,
};
use super::hepta_platform_material_runtime::{
    HeptaSystemMaterialAdapter, HeptaSystemMaterialError, HeptaSystemMaterialReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsBackdropKind {
    None,
    Mica,
    Acrylic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsWindowHandles {
    pub chrome_window: isize,
    pub transient_window: isize,
}

impl HeptaWindowsWindowHandles {
    pub const fn new(
        chrome_window: isize,
        transient_window: isize,
    ) -> Result<Self, HeptaSystemMaterialError> {
        if chrome_window == 0 || transient_window == 0 {
            return Err(HeptaSystemMaterialError::InvalidHostHandle);
        }
        Ok(Self {
            chrome_window,
            transient_window,
        })
    }
}

/// Minimal DWM call surface used by the Windows adapter. Implementations must
/// return the original HRESULT value on failure.
pub trait HeptaWindowsBackdropApi {
    fn set_backdrop(
        &mut self,
        window: isize,
        kind: HeptaWindowsBackdropKind,
    ) -> Result<(), i32>;
}

pub struct HeptaWindowsMaterialAdapter<A> {
    handles: HeptaWindowsWindowHandles,
    api: A,
    bound: bool,
}

impl<A> HeptaWindowsMaterialAdapter<A> {
    pub const fn new(handles: HeptaWindowsWindowHandles, api: A) -> Self {
        Self {
            handles,
            api,
            bound: false,
        }
    }
}

impl<A: HeptaWindowsBackdropApi> HeptaSystemMaterialAdapter
    for HeptaWindowsMaterialAdapter<A>
{
    fn platform(&self) -> HeptaPlatform {
        HeptaPlatform::Windows
    }

    fn bind(
        &mut self,
        profile: HeptaPlatformMaterialProfile,
    ) -> Result<HeptaSystemMaterialReceipt, HeptaSystemMaterialError> {
        if profile.content != HeptaMaterialRenderer::Solid
            || profile.chrome != HeptaMaterialRenderer::WindowsMica
            || profile.transient != HeptaMaterialRenderer::WindowsAcrylic
            || profile.stable_content_backdrop_layers != 0
            || profile.max_visible_backdrop_layers > 2
        {
            return Err(HeptaSystemMaterialError::AdapterRejectedProfile);
        }

        self.api
            .set_backdrop(self.handles.chrome_window, HeptaWindowsBackdropKind::Mica)
            .map_err(HeptaSystemMaterialError::SystemCallFailed)?;

        if let Err(error) = self.api.set_backdrop(
            self.handles.transient_window,
            HeptaWindowsBackdropKind::Acrylic,
        ) {
            let _ = self.api.set_backdrop(
                self.handles.chrome_window,
                HeptaWindowsBackdropKind::None,
            );
            self.bound = false;
            return Err(HeptaSystemMaterialError::SystemCallFailed(error));
        }

        self.bound = true;
        Ok(HeptaSystemMaterialReceipt {
            platform: HeptaPlatform::Windows,
            chrome: HeptaMaterialRenderer::WindowsMica,
            transient: HeptaMaterialRenderer::WindowsAcrylic,
            system_material_bound: true,
            production_authority: false,
            effect_authority: false,
        })
    }

    fn unbind(&mut self) {
        if !self.bound {
            return;
        }
        let _ = self.api.set_backdrop(
            self.handles.transient_window,
            HeptaWindowsBackdropKind::None,
        );
        let _ = self.api.set_backdrop(
            self.handles.chrome_window,
            HeptaWindowsBackdropKind::None,
        );
        self.bound = false;
    }
}

pub const HEPTA_WINDOWS_MATERIAL_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_RELEASE: bool = false;

#[cfg(target_os = "windows")]
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
#[cfg(target_os = "windows")]
const DWMSBT_AUTO: i32 = 0;
#[cfg(target_os = "windows")]
const DWMSBT_MAINWINDOW: i32 = 2;
#[cfg(target_os = "windows")]
const DWMSBT_TRANSIENTWINDOW: i32 = 3;

#[cfg(target_os = "windows")]
#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        window: isize,
        attribute: u32,
        value: *const std::ffi::c_void,
        value_size: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[derive(Default)]
pub struct HeptaWindowsDwmBackdropApi;

#[cfg(target_os = "windows")]
impl HeptaWindowsBackdropApi for HeptaWindowsDwmBackdropApi {
    fn set_backdrop(
        &mut self,
        window: isize,
        kind: HeptaWindowsBackdropKind,
    ) -> Result<(), i32> {
        use std::mem::size_of;

        if window == 0 {
            return Err(-1);
        }
        let value = match kind {
            HeptaWindowsBackdropKind::None => DWMSBT_AUTO,
            HeptaWindowsBackdropKind::Mica => DWMSBT_MAINWINDOW,
            HeptaWindowsBackdropKind::Acrylic => DWMSBT_TRANSIENTWINDOW,
        };
        let result = unsafe {
            DwmSetWindowAttribute(
                window,
                DWMWA_SYSTEMBACKDROP_TYPE,
                (&value as *const i32).cast(),
                size_of::<i32>() as u32,
            )
        };
        if result >= 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::hepta_platform_material::{
        HeptaPlatformMaterialCapabilities, platform_material_profile_with_capabilities,
    };
    use crate::shared::hepta_platform_material_runtime::{
        HeptaMaterialRuntimePreferences, bind_material_runtime,
    };

    #[derive(Default)]
    struct RecordingApi {
        calls: Vec<(isize, HeptaWindowsBackdropKind)>,
        fail_on: Option<HeptaWindowsBackdropKind>,
    }

    impl HeptaWindowsBackdropApi for RecordingApi {
        fn set_backdrop(
            &mut self,
            window: isize,
            kind: HeptaWindowsBackdropKind,
        ) -> Result<(), i32> {
            self.calls.push((window, kind));
            if self.fail_on == Some(kind) {
                Err(-5)
            } else {
                Ok(())
            }
        }
    }

    fn handles() -> HeptaWindowsWindowHandles {
        HeptaWindowsWindowHandles::new(
            /* chrome_window */ 11,
            /* transient_window */ 12,
        )
        .unwrap()
    }

    fn preferences() -> HeptaMaterialRuntimePreferences {
        HeptaMaterialRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: false,
        }
    }

    #[test]
    fn windows_adapter_binds_and_unbinds_both_material_roles() {
        let mut adapter =
            HeptaWindowsMaterialAdapter::new(handles(), RecordingApi::default());
        let receipt = bind_material_runtime(&mut adapter, preferences()).unwrap();
        assert_eq!(
            receipt,
            HeptaSystemMaterialReceipt {
                platform: HeptaPlatform::Windows,
                chrome: HeptaMaterialRenderer::WindowsMica,
                transient: HeptaMaterialRenderer::WindowsAcrylic,
                system_material_bound: true,
                production_authority: false,
                effect_authority: false,
            }
        );
        assert_eq!(
            adapter.api.calls,
            vec![
                (11, HeptaWindowsBackdropKind::Mica),
                (12, HeptaWindowsBackdropKind::Acrylic),
            ]
        );

        adapter.unbind();
        assert_eq!(
            adapter.api.calls,
            vec![
                (11, HeptaWindowsBackdropKind::Mica),
                (12, HeptaWindowsBackdropKind::Acrylic),
                (12, HeptaWindowsBackdropKind::None),
                (11, HeptaWindowsBackdropKind::None),
            ]
        );
    }

    #[test]
    fn windows_adapter_rolls_back_a_partial_bind() {
        let api = RecordingApi {
            fail_on: Some(HeptaWindowsBackdropKind::Acrylic),
            ..RecordingApi::default()
        };
        let mut adapter = HeptaWindowsMaterialAdapter::new(handles(), api);
        let profile = platform_material_profile_with_capabilities(
            HeptaPlatform::Windows,
            true,
            HeptaPlatformMaterialCapabilities::default(),
        );
        assert_eq!(
            adapter.bind(profile),
            Err(HeptaSystemMaterialError::SystemCallFailed(-5))
        );
        assert_eq!(
            adapter.api.calls,
            vec![
                (11, HeptaWindowsBackdropKind::Mica),
                (12, HeptaWindowsBackdropKind::Acrylic),
                (11, HeptaWindowsBackdropKind::None),
            ]
        );
    }

    #[test]
    fn windows_adapter_rejects_invalid_handles_and_grants_no_authority() {
        assert_eq!(
            HeptaWindowsWindowHandles::new(
                /* chrome_window */ 0,
                /* transient_window */ 12,
            ),
            Err(HeptaSystemMaterialError::InvalidHostHandle)
        );
        assert!(!HEPTA_WINDOWS_MATERIAL_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_MATERIAL_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_MATERIAL_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_MATERIAL_PROMOTION);
        assert!(!HEPTA_WINDOWS_MATERIAL_RELEASE);
    }
}
