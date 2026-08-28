pub struct HeptaWindowsProductDwmBackend<A> {
    api: A,
}

impl<A> HeptaWindowsProductDwmBackend<A> {
    pub const fn new(api: A) -> Self {
        Self { api }
    }

    pub fn into_inner(self) -> A {
        self.api
    }

    fn rollback_best_effort(&mut self, identity: HeptaWindowsProductHostWindowIdentity)
    where
        A: HeptaWindowsBackdropApi,
    {
        if let (Ok(root), Ok(transient)) =
            (native_handle(identity.root_hwnd), native_handle(identity.transient_hwnd))
        {
            let _ = self
                .api
                .set_backdrop(transient, HeptaWindowsBackdropKind::None);
            let _ = self.api.set_backdrop(root, HeptaWindowsBackdropKind::None);
        }
    }
}

impl<A> HeptaWindowsProductMaterialBackend for HeptaWindowsProductDwmBackend<A>
where
    A: HeptaWindowsBackdropApi + HeptaWindowsBackdropReadbackApi,
{
    fn bind_verified(
        &mut self,
        identity: HeptaWindowsProductHostWindowIdentity,
    ) -> Result<HeptaWindowsVerifiedMaterialBinding, HeptaWindowsProductMaterialBackendError> {
        if !identity.is_valid() {
            return Err(HeptaWindowsProductMaterialBackendError::BindFailed);
        }
        let root = native_handle(identity.root_hwnd)
            .map_err(|_| HeptaWindowsProductMaterialBackendError::BindFailed)?;
        let transient = native_handle(identity.transient_hwnd)
            .map_err(|_| HeptaWindowsProductMaterialBackendError::BindFailed)?;

        if self
            .api
            .set_backdrop(root, HeptaWindowsBackdropKind::Mica)
            .is_err()
            || self.api.read_backdrop(root).ok()
                != Some(HeptaWindowsDwmBackdropValue::Mica)
        {
            self.rollback_best_effort(identity);
            return Err(HeptaWindowsProductMaterialBackendError::BindFailed);
        }

        if self
            .api
            .set_backdrop(transient, HeptaWindowsBackdropKind::Acrylic)
            .is_err()
            || self.api.read_backdrop(transient).ok()
                != Some(HeptaWindowsDwmBackdropValue::Acrylic)
        {
            self.rollback_best_effort(identity);
            return Err(HeptaWindowsProductMaterialBackendError::BindFailed);
        }

        Ok(HeptaWindowsVerifiedMaterialBinding {
            identity,
            root_mica_exact: true,
            transient_acrylic_exact: true,
            complete_profile_bound: true,
            system_material_bound: true,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        })
    }

    fn rollback_to_solid_verified(
        &mut self,
        identity: HeptaWindowsProductHostWindowIdentity,
    ) -> Result<HeptaWindowsVerifiedMaterialRollback, HeptaWindowsProductMaterialBackendError> {
        if !identity.is_valid() {
            return Err(HeptaWindowsProductMaterialBackendError::RollbackFailed);
        }
        let root = native_handle(identity.root_hwnd)
            .map_err(|_| HeptaWindowsProductMaterialBackendError::RollbackFailed)?;
        let transient = native_handle(identity.transient_hwnd)
            .map_err(|_| HeptaWindowsProductMaterialBackendError::RollbackFailed)?;

        let transient_set = self
            .api
            .set_backdrop(transient, HeptaWindowsBackdropKind::None)
            .is_ok();
        let root_set = self
            .api
            .set_backdrop(root, HeptaWindowsBackdropKind::None)
            .is_ok();
        let transient_exact = transient_set
            && self.api.read_backdrop(transient).ok()
                == Some(HeptaWindowsDwmBackdropValue::None);
        let root_exact = root_set
            && self.api.read_backdrop(root).ok()
                == Some(HeptaWindowsDwmBackdropValue::None);
        if !transient_exact || !root_exact {
            self.rollback_best_effort(identity);
            return Err(HeptaWindowsProductMaterialBackendError::RollbackFailed);
        }

        Ok(HeptaWindowsVerifiedMaterialRollback {
            identity,
            root_none_exact: true,
            transient_none_exact: true,
            rollback_verified: true,
            authority: HeptaWindowsReviewAuthorityBoundary::default(),
        })
    }
}
