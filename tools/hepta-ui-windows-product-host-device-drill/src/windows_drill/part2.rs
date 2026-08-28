#[derive(Clone, Copy)]
struct DwmBackend {
    root: Hwnd,
    transient: Hwnd,
}

impl DwmBackend {
    fn set_and_read(window: Hwnd, value: i32) -> Result<bool, HeptaWindowsQualificationBackendError> {
        let set_result = unsafe {
            DwmSetWindowAttribute(
                window,
                DWMWA_SYSTEMBACKDROP_TYPE,
                (&value as *const i32).cast(),
                size_of::<i32>() as u32,
            )
        };
        if set_result < 0 {
            return Err(HeptaWindowsQualificationBackendError::BindFailed);
        }
        pump_messages();
        thread::sleep(Duration::from_millis(120));
        let mut observed = 0i32;
        let get_result = unsafe {
            DwmGetWindowAttribute(
                window,
                DWMWA_SYSTEMBACKDROP_TYPE,
                (&mut observed as *mut i32).cast(),
                size_of::<i32>() as u32,
            )
        };
        if get_result < 0 {
            return Err(HeptaWindowsQualificationBackendError::BindFailed);
        }
        Ok(observed == value)
    }
}

impl HeptaWindowsQualificationBackend for DwmBackend {
    fn bind_qualification_verified(
        &mut self,
        identity: HeptaWindowsQualificationWindowIdentity,
    ) -> Result<HeptaWindowsQualificationBindingReceipt, HeptaWindowsQualificationBackendError>
    {
        let root_mica_exact = Self::set_and_read(self.root, DWMSBT_MAINWINDOW)?;
        let transient_acrylic_exact =
            Self::set_and_read(self.transient, DWMSBT_TRANSIENTWINDOW)?;
        Ok(HeptaWindowsQualificationBindingReceipt {
            identity,
            root_mica_exact,
            transient_acrylic_exact,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        })
    }

    fn rollback_qualification_to_solid_verified(
        &mut self,
        identity: HeptaWindowsQualificationWindowIdentity,
    ) -> Result<HeptaWindowsQualificationRollbackReceipt, HeptaWindowsQualificationBackendError>
    {
        let transient_none_exact =
            Self::set_and_read(self.transient, DWMSBT_NONE)
                .map_err(|_| HeptaWindowsQualificationBackendError::RollbackFailed)?;
        let root_none_exact = Self::set_and_read(self.root, DWMSBT_NONE)
            .map_err(|_| HeptaWindowsQualificationBackendError::RollbackFailed)?;
        Ok(HeptaWindowsQualificationRollbackReceipt {
            identity,
            root_none_exact,
            transient_none_exact,
            rollback_verified: root_none_exact && transient_none_exact,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        })
    }
}

fn pump_messages() {
    let mut message = Msg::default();
    unsafe {
        while PeekMessageW(&mut message, 0, 0, 0, PM_REMOVE) != 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn required(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("missing environment variable {name}"))
}

fn true_env(name: &str) -> Result<bool, String> {
    Ok(required(name)? == "true")
}
