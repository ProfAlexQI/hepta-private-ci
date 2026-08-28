use std::{
    env,
    ffi::c_void,
    fs,
    mem::size_of,
    path::PathBuf,
    ptr::{null, null_mut},
    thread,
    time::Duration,
};

use super::hepta_windows_product_host_integration_review::{
    HeptaWindowsProductHostIntegrationPlan, HeptaWindowsProductHostReviewReceipt,
    HeptaWindowsProductHostReviewStatus, HeptaWindowsReviewAuthorityBoundary,
};
use super::hepta_windows_product_host_qualification_host::{
    HeptaWindowsProductHostReviewSeal, HeptaWindowsQualificationApproval,
    HeptaWindowsQualificationBackend, HeptaWindowsQualificationBackendError,
    HeptaWindowsQualificationBindingReceipt, HeptaWindowsQualificationHost,
    HeptaWindowsQualificationPhase, HeptaWindowsQualificationPreferences,
    HeptaWindowsQualificationRequest, HeptaWindowsQualificationRollbackReceipt,
    HeptaWindowsQualificationWindowIdentity,
};

type Hwnd = isize;
type Hinstance = isize;
type Hicon = isize;
type Hcursor = isize;
type Hbrush = isize;
type Hmenu = isize;
type Wparam = usize;
type Lparam = isize;
type Lresult = isize;

const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
const WS_POPUP: u32 = 0x8000_0000;
const WS_VISIBLE: u32 = 0x1000_0000;
const WS_EX_TOOLWINDOW: u32 = 0x0000_0080;
const SW_SHOW: i32 = 5;
const PM_REMOVE: u32 = 0x0001;
const DWMWA_SYSTEMBACKDROP_TYPE: u32 = 38;
const DWMSBT_NONE: i32 = 1;
const DWMSBT_MAINWINDOW: i32 = 2;
const DWMSBT_TRANSIENTWINDOW: i32 = 3;

#[repr(C)]
struct WndClassExW {
    cb_size: u32,
    style: u32,
    wnd_proc: Option<unsafe extern "system" fn(Hwnd, u32, Wparam, Lparam) -> Lresult>,
    cls_extra: i32,
    wnd_extra: i32,
    instance: Hinstance,
    icon: Hicon,
    cursor: Hcursor,
    background: Hbrush,
    menu_name: *const u16,
    class_name: *const u16,
    icon_small: Hicon,
}

#[repr(C)]
#[derive(Default)]
struct Msg {
    hwnd: Hwnd,
    message: u32,
    w_param: Wparam,
    l_param: Lparam,
    time: u32,
    point_x: i32,
    point_y: i32,
    private: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
}

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassExW(class: *const WndClassExW) -> u16;
    fn CreateWindowExW(
        extended_style: u32,
        class_name: *const u16,
        window_name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Hwnd,
        menu: Hmenu,
        instance: Hinstance,
        parameter: *mut c_void,
    ) -> Hwnd;
    fn DestroyWindow(window: Hwnd) -> i32;
    fn DefWindowProcW(window: Hwnd, message: u32, wparam: Wparam, lparam: Lparam) -> Lresult;
    fn ShowWindow(window: Hwnd, command: i32) -> i32;
    fn UpdateWindow(window: Hwnd) -> i32;
    fn PeekMessageW(message: *mut Msg, window: Hwnd, min: u32, max: u32, remove: u32) -> i32;
    fn TranslateMessage(message: *const Msg) -> i32;
    fn DispatchMessageW(message: *const Msg) -> Lresult;
}

#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        window: Hwnd,
        attribute: u32,
        value: *const c_void,
        value_size: u32,
    ) -> i32;
    fn DwmGetWindowAttribute(
        window: Hwnd,
        attribute: u32,
        value: *mut c_void,
        value_size: u32,
    ) -> i32;
}

unsafe extern "system" fn window_proc(
    window: Hwnd,
    message: u32,
    wparam: Wparam,
    lparam: Lparam,
) -> Lresult {
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

struct WindowPair {
    root: Hwnd,
    transient: Hwnd,
}

impl WindowPair {
    fn create() -> Result<Self, String> {
        let class_name = wide("HeptaUiV4QualificationWindow");
        let title = wide("Hepta UI v4 physical material qualification");
        let instance = unsafe { GetModuleHandleW(null()) };
        if instance == 0 {
            return Err("GetModuleHandleW failed".to_string());
        }
        let class = WndClassExW {
            cb_size: size_of::<WndClassExW>() as u32,
            style: 0,
            wnd_proc: Some(window_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance,
            icon: 0,
            cursor: 0,
            background: 0,
            menu_name: null(),
            class_name: class_name.as_ptr(),
            icon_small: 0,
        };
        let atom = unsafe { RegisterClassExW(&class) };
        if atom == 0 {
            return Err("RegisterClassExW failed".to_string());
        }
        let root = unsafe {
            CreateWindowExW(
                0,
                class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100,
                100,
                720,
                480,
                0,
                0,
                instance,
                null_mut(),
            )
        };
        if root == 0 {
            return Err("root CreateWindowExW failed".to_string());
        }
        let transient = unsafe {
            CreateWindowExW(
                WS_EX_TOOLWINDOW,
                class_name.as_ptr(),
                title.as_ptr(),
                WS_POPUP | WS_VISIBLE,
                180,
                180,
                420,
                260,
                root,
                0,
                instance,
                null_mut(),
            )
        };
        if transient == 0 {
            unsafe { DestroyWindow(root) };
            return Err("transient CreateWindowExW failed".to_string());
        }
        unsafe {
            ShowWindow(root, SW_SHOW);
            ShowWindow(transient, SW_SHOW);
            UpdateWindow(root);
            UpdateWindow(transient);
        }
        pump_messages();
        Ok(Self { root, transient })
    }
}

impl Drop for WindowPair {
    fn drop(&mut self) {
        unsafe {
            let _ = DestroyWindow(self.transient);
            let _ = DestroyWindow(self.root);
        }
    }
}
