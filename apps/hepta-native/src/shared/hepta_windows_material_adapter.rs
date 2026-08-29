//! Windows 11 DWM material adapter for Hepta UI v4.
//!
//! The full-profile adapter requires explicit, non-zero persistent and transient
//! host handles. The readback API is separate so a persistent-root acknowledgement
//! can prove only the DWM system-backdrop type without pretending to read Makepad
//! transparency or intensity state.

use super::hepta_platform_material::{
    HeptaMaterialRenderer, HeptaPlatform, HeptaPlatformMaterialProfile,
};
use super::hepta_platform_material_runtime::{
    HeptaSystemMaterialAdapter, HeptaSystemMaterialError, HeptaSystemMaterialReceipt,
};

const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_AUTO: i32 = 0;
const DWMSBT_NONE: i32 = 1;
const DWMSBT_MAINWINDOW: i32 = 2;
const DWMSBT_TRANSIENTWINDOW: i32 = 3;
const DWMSBT_TABBEDWINDOW: i32 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsBackdropKind {
    None,
    Mica,
    Acrylic,
}

impl HeptaWindowsBackdropKind {
    pub const fn dwm_value(self) -> i32 {
        match self {
            Self::None => DWMSBT_NONE,
            Self::Mica => DWMSBT_MAINWINDOW,
            Self::Acrylic => DWMSBT_TRANSIENTWINDOW,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsDwmBackdropValue {
    Auto,
    None,
    Mica,
    Acrylic,
    MicaAlt,
}

impl HeptaWindowsDwmBackdropValue {
    pub const fn dwm_value(self) -> i32 {
        match self {
            Self::Auto => DWMSBT_AUTO,
            Self::None => DWMSBT_NONE,
            Self::Mica => DWMSBT_MAINWINDOW,
            Self::Acrylic => DWMSBT_TRANSIENTWINDOW,
            Self::MicaAlt => DWMSBT_TABBEDWINDOW,
        }
    }

    pub const fn from_dwm_value(value: i32) -> Option<Self> {
        match value {
            DWMSBT_AUTO => Some(Self::Auto),
            DWMSBT_NONE => Some(Self::None),
            DWMSBT_MAINWINDOW => Some(Self::Mica),
            DWMSBT_TRANSIENTWINDOW => Some(Self::Acrylic),
            DWMSBT_TABBEDWINDOW => Some(Self::MicaAlt),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsBackdropReadbackError {
    InvalidHostHandle,
    SystemCallFailed(i32),
    UnknownBackdropValue(i32),
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

/// Minimal DWM write surface used by the full Windows material adapter.
pub trait HeptaWindowsBackdropApi {
    fn set_backdrop(&mut self, window: isize, kind: HeptaWindowsBackdropKind) -> Result<(), i32>;
}

/// DWM readback surface used by the root-window acknowledgement producer.
/// It returns only the system backdrop type. It cannot prove Makepad's
/// transparency flag or backdrop intensity.
pub trait HeptaWindowsBackdropReadbackApi {
    fn read_backdrop(
        &mut self,
        window: isize,
    ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError>;
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

impl<A: HeptaWindowsBackdropApi> HeptaSystemMaterialAdapter for HeptaWindowsMaterialAdapter<A> {
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
            let _ = self
                .api
                .set_backdrop(self.handles.chrome_window, HeptaWindowsBackdropKind::None);
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
        let _ = self
            .api
            .set_backdrop(self.handles.chrome_window, HeptaWindowsBackdropKind::None);
        self.bound = false;
    }
}

pub const HEPTA_WINDOWS_MATERIAL_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_MATERIAL_RELEASE: bool = false;

#[cfg(target_os = "windows")]
#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        window: isize,
        attribute: u32,
        value: *const std::ffi::c_void,
        value_size: u32,
    ) -> i32;

    fn DwmGetWindowAttribute(
        window: isize,
        attribute: u32,
        value: *mut std::ffi::c_void,
        value_size: u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[derive(Default)]
pub struct HeptaWindowsDwmBackdropApi;

#[cfg(target_os = "windows")]
impl HeptaWindowsBackdropApi for HeptaWindowsDwmBackdropApi {
    fn set_backdrop(&mut self, window: isize, kind: HeptaWindowsBackdropKind) -> Result<(), i32> {
        use std::mem::size_of;

        if window == 0 {
            return Err(-1);
        }
        let value = kind.dwm_value();
        let result = unsafe {
            DwmSetWindowAttribute(
                window,
                DWMWA_SYSTEMBACKDROP_TYPE,
                (&value as *const i32).cast(),
                size_of::<i32>() as u32,
            )
        };
        if result >= 0 { Ok(()) } else { Err(result) }
    }
}

#[cfg(target_os = "windows")]
impl HeptaWindowsBackdropReadbackApi for HeptaWindowsDwmBackdropApi {
    fn read_backdrop(
        &mut self,
        window: isize,
    ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
        use std::mem::size_of;

        if window == 0 {
            return Err(HeptaWindowsBackdropReadbackError::InvalidHostHandle);
        }
        let mut value = DWMSBT_AUTO;
        let result = unsafe {
            DwmGetWindowAttribute(
                window,
                DWMWA_SYSTEMBACKDROP_TYPE,
                (&mut value as *mut i32).cast(),
                size_of::<i32>() as u32,
            )
        };
        if result < 0 {
            return Err(HeptaWindowsBackdropReadbackError::SystemCallFailed(result));
        }
        HeptaWindowsDwmBackdropValue::from_dwm_value(value).ok_or(
            HeptaWindowsBackdropReadbackError::UnknownBackdropValue(value),
        )
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
        HeptaWindowsWindowHandles::new(11, 12).unwrap()
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
        let mut adapter = HeptaWindowsMaterialAdapter::new(handles(), RecordingApi::default());
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
    fn dwm_none_is_not_auto_and_all_known_values_round_trip() {
        assert_eq!(HeptaWindowsDwmBackdropValue::Auto.dwm_value(), 0);
        assert_eq!(HeptaWindowsBackdropKind::None.dwm_value(), 1);
        assert_ne!(
            HeptaWindowsBackdropKind::None.dwm_value(),
            HeptaWindowsDwmBackdropValue::Auto.dwm_value()
        );
        for value in [
            HeptaWindowsDwmBackdropValue::Auto,
            HeptaWindowsDwmBackdropValue::None,
            HeptaWindowsDwmBackdropValue::Mica,
            HeptaWindowsDwmBackdropValue::Acrylic,
            HeptaWindowsDwmBackdropValue::MicaAlt,
        ] {
            assert_eq!(
                HeptaWindowsDwmBackdropValue::from_dwm_value(value.dwm_value()),
                Some(value)
            );
        }
        assert_eq!(HeptaWindowsDwmBackdropValue::from_dwm_value(99), None);
    }

    #[test]
    fn windows_adapter_rejects_invalid_handles_and_grants_no_authority() {
        assert_eq!(
            HeptaWindowsWindowHandles::new(0, 12),
            Err(HeptaSystemMaterialError::InvalidHostHandle)
        );
        assert!(!HEPTA_WINDOWS_MATERIAL_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_MATERIAL_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_MATERIAL_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_MATERIAL_PROMOTION);
        assert!(!HEPTA_WINDOWS_MATERIAL_RELEASE);
    }
}
