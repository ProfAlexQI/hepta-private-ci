pub struct HeptaWindowsProductHostCoordinator<B> {
    host: HeptaWindowsProductMaterialHost<B>,
    active_review_binding_digest: Option<String>,
}

impl<B: HeptaWindowsProductMaterialBackend> HeptaWindowsProductHostCoordinator<B> {
    pub const fn new(backend: B) -> Self {
        Self {
            host: HeptaWindowsProductMaterialHost::new(backend),
            active_review_binding_digest: None,
        }
    }

    pub fn phase(&self) -> HeptaWindowsProductHostPhase {
        self.host.phase()
    }

    pub fn activate_explicit(
        &mut self,
        seal: &HeptaWindowsProductHostEvidenceSeal,
        preferences: HeptaWindowsProductHostRuntimePreferences,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        let request = seal.activation_request(preferences);
        let receipt = self.host.activate(&request)?;
        self.active_review_binding_digest = Some(seal.review_binding_digest().to_string());
        Ok(receipt)
    }

    pub fn reconcile_preferences(
        &mut self,
        preferences: HeptaWindowsProductHostRuntimePreferences,
    ) -> Result<Option<HeptaWindowsProductHostRuntimeReceipt>, HeptaWindowsProductHostWiringError>
    {
        if self.host.phase() != HeptaWindowsProductHostPhase::Bound {
            return Ok(None);
        }
        if !preferences.transparency_allowed || preferences.high_contrast {
            let receipt = self.host.rollback_to_solid()?;
            self.active_review_binding_digest = None;
            return Ok(Some(receipt));
        }
        Ok(None)
    }

    pub fn suspend(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        let receipt = self.host.suspend()?;
        self.active_review_binding_digest = None;
        Ok(receipt)
    }

    pub fn shutdown(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        let receipt = self.host.shutdown()?;
        self.active_review_binding_digest = None;
        Ok(receipt)
    }

    pub fn active_review_binding_digest(&self) -> Option<&str> {
        self.active_review_binding_digest.as_deref()
    }
}

#[cfg(target_os = "windows")]
pub struct HeptaWindowsDefaultProductHostCoordinator {
    inner: HeptaWindowsProductHostCoordinator<
        HeptaWindowsProductDwmBackend<HeptaWindowsDwmBackdropApi>,
    >,
}

#[cfg(target_os = "windows")]
impl Default for HeptaWindowsDefaultProductHostCoordinator {
    fn default() -> Self {
        Self {
            inner: HeptaWindowsProductHostCoordinator::new(
                HeptaWindowsProductDwmBackend::new(HeptaWindowsDwmBackdropApi),
            ),
        }
    }
}

#[cfg(target_os = "windows")]
impl HeptaWindowsDefaultProductHostCoordinator {
    pub fn phase(&self) -> HeptaWindowsProductHostPhase {
        self.inner.phase()
    }

    pub fn activate_explicit(
        &mut self,
        seal: &HeptaWindowsProductHostEvidenceSeal,
        preferences: HeptaWindowsProductHostRuntimePreferences,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        self.inner.activate_explicit(seal, preferences)
    }

    pub fn reconcile_preferences(
        &mut self,
        preferences: HeptaWindowsProductHostRuntimePreferences,
    ) -> Result<Option<HeptaWindowsProductHostRuntimeReceipt>, HeptaWindowsProductHostWiringError>
    {
        self.inner.reconcile_preferences(preferences)
    }

    pub fn suspend(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        self.inner.suspend()
    }

    pub fn shutdown(
        &mut self,
    ) -> Result<HeptaWindowsProductHostRuntimeReceipt, HeptaWindowsProductHostWiringError> {
        self.inner.shutdown()
    }
}

fn native_handle(value: u64) -> Result<isize, ()> {
    isize::try_from(value).map_err(|_| ())
}

fn git_object_id(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

