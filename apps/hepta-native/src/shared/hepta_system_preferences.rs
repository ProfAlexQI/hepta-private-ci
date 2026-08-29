//! System accessibility and material-preference probing for Hepta UI v4.
//!
//! Every unsupported or unavailable probe fails closed to solid surfaces and
//! reduced motion. Host applications may inject an explicit snapshot when the
//! platform requires an activity, view, or window object that this shared layer
//! does not own.

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

/// Provides a bounded preference snapshot without granting material, effect, or
/// production authority. Implementations must fail closed when a value cannot
/// be verified.
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

pub fn current_system_preferences() -> HeptaSystemPreferenceSnapshot {
    #[cfg(target_os = "windows")]
    {
        return windows::probe();
    }

    #[cfg(target_os = "ios")]
    {
        return ios::probe();
    }

    HeptaSystemPreferenceSnapshot::fail_closed()
}

#[cfg(target_os = "windows")]
mod windows {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::ptr::null_mut;

    use super::{
        HeptaMaterialRuntimePreferences, HeptaSystemPreferenceProbeStatus,
        HeptaSystemPreferenceSnapshot, HeptaSystemPreferenceSource,
    };

    const SPI_GETHIGHCONTRAST: u32 = 0x0042;
    const SPI_GETCLIENTAREAANIMATION: u32 = 0x1042;
    const HCF_HIGHCONTRASTON: u32 = 0x0000_0001;
    const RRF_RT_REG_DWORD: u32 = 0x0000_0018;
    const ERROR_SUCCESS: i32 = 0;
    const HKEY_CURRENT_USER: isize = 0x8000_0001_u32 as i32 as isize;

    #[repr(C)]
    struct HighContrastW {
        cb_size: u32,
        dw_flags: u32,
        default_scheme: *mut u16,
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn SystemParametersInfoW(
            action: u32,
            parameter: u32,
            value: *mut c_void,
            update: u32,
        ) -> i32;
    }

    #[link(name = "advapi32")]
    unsafe extern "system" {
        fn RegGetValueW(
            key: isize,
            sub_key: *const u16,
            value_name: *const u16,
            flags: u32,
            value_type: *mut u32,
            data: *mut c_void,
            data_size: *mut u32,
        ) -> i32;
    }

    pub(super) fn probe() -> HeptaSystemPreferenceSnapshot {
        let transparency = transparency_allowed();
        let high_contrast = high_contrast_enabled();
        let client_animation = client_area_animation_enabled();
        let verified_count = [
            transparency.is_some(),
            high_contrast.is_some(),
            client_animation.is_some(),
        ]
        .into_iter()
        .filter(|verified| *verified)
        .count();
        let status = match verified_count {
            3 => HeptaSystemPreferenceProbeStatus::Verified,
            1 | 2 => HeptaSystemPreferenceProbeStatus::Partial,
            _ => HeptaSystemPreferenceProbeStatus::Unavailable,
        };
        let high_contrast = high_contrast.unwrap_or(false);

        HeptaSystemPreferenceSnapshot {
            preferences: HeptaMaterialRuntimePreferences {
                transparency_allowed: transparency.unwrap_or(false) && !high_contrast,
                high_contrast,
                reduced_motion: client_animation.map(|enabled| !enabled).unwrap_or(true),
                dynamic_color_available: false,
            },
            source: if verified_count == 0 {
                HeptaSystemPreferenceSource::Unavailable
            } else {
                HeptaSystemPreferenceSource::WindowsSystem
            },
            status,
        }
    }

    fn high_contrast_enabled() -> Option<bool> {
        let mut value = HighContrastW {
            cb_size: size_of::<HighContrastW>() as u32,
            dw_flags: 0,
            default_scheme: null_mut(),
        };
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETHIGHCONTRAST,
                value.cb_size,
                (&mut value as *mut HighContrastW).cast(),
                0,
            )
        };
        (result != 0).then_some(value.dw_flags & HCF_HIGHCONTRASTON != 0)
    }

    fn client_area_animation_enabled() -> Option<bool> {
        let mut enabled = 0_i32;
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETCLIENTAREAANIMATION,
                0,
                (&mut enabled as *mut i32).cast(),
                0,
            )
        };
        (result != 0).then_some(enabled != 0)
    }

    fn transparency_allowed() -> Option<bool> {
        let sub_key = wide("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");
        let value_name = wide("EnableTransparency");
        let mut value = 0_u32;
        let mut data_size = size_of::<u32>() as u32;
        let result = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                sub_key.as_ptr(),
                value_name.as_ptr(),
                RRF_RT_REG_DWORD,
                null_mut(),
                (&mut value as *mut u32).cast(),
                &mut data_size,
            )
        };
        (result == ERROR_SUCCESS).then_some(value != 0)
    }

    fn wide(value: &str) -> Vec<u16> {
        value.encode_utf16().chain(Some(0)).collect()
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use super::{
        HeptaMaterialRuntimePreferences, HeptaSystemPreferenceProbeStatus,
        HeptaSystemPreferenceSnapshot, HeptaSystemPreferenceSource,
    };

    #[link(name = "UIKit", kind = "framework")]
    unsafe extern "C" {
        fn UIAccessibilityIsReduceTransparencyEnabled() -> i8;
        fn UIAccessibilityIsReduceMotionEnabled() -> i8;
        fn UIAccessibilityDarkerSystemColorsEnabled() -> i8;
    }

    pub(super) fn probe() -> HeptaSystemPreferenceSnapshot {
        let reduce_transparency = unsafe { UIAccessibilityIsReduceTransparencyEnabled() != 0 };
        let reduced_motion = unsafe { UIAccessibilityIsReduceMotionEnabled() != 0 };
        let high_contrast = unsafe { UIAccessibilityDarkerSystemColorsEnabled() != 0 };

        HeptaSystemPreferenceSnapshot {
            preferences: HeptaMaterialRuntimePreferences {
                transparency_allowed: !reduce_transparency && !high_contrast,
                high_contrast,
                reduced_motion,
                dynamic_color_available: false,
            },
            source: HeptaSystemPreferenceSource::IosAccessibility,
            status: HeptaSystemPreferenceProbeStatus::Verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_preferences_fail_closed() {
        let snapshot = HeptaSystemPreferenceSnapshot::fail_closed();
        assert_eq!(snapshot, HeptaSystemPreferenceSnapshot::default());
        assert!(!snapshot.preferences.transparency_allowed);
        assert!(snapshot.preferences.reduced_motion);
        assert!(!snapshot.preferences.dynamic_color_available);
    }

    #[test]
    fn host_preferences_are_explicit_and_round_trip() {
        let preferences = HeptaMaterialRuntimePreferences {
            transparency_allowed: true,
            high_contrast: false,
            reduced_motion: false,
            dynamic_color_available: true,
        };
        let probe = HeptaHostPreferenceProbe::new(preferences);
        assert_eq!(
            probe.probe(),
            HeptaSystemPreferenceSnapshot {
                preferences,
                source: HeptaSystemPreferenceSource::HostProvided,
                status: HeptaSystemPreferenceProbeStatus::HostProvided,
            }
        );
    }

    #[test]
    fn preference_probe_never_grants_authority() {
        assert!(!HEPTA_SYSTEM_PREFERENCE_NETWORK_AUTHORITY);
        assert!(!HEPTA_SYSTEM_PREFERENCE_EFFECT_AUTHORITY);
        assert!(!HEPTA_SYSTEM_PREFERENCE_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_SYSTEM_PREFERENCE_PROMOTION);
        assert!(!HEPTA_SYSTEM_PREFERENCE_RELEASE);
    }
}
