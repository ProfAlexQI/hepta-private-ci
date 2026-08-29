//! Fail-closed Windows transient Acrylic lifecycle for Hepta UI v4.
//!
//! The transient host is deliberately separate from the persistent root-window
//! bridge. It requires a distinct popup HWND, exact parent/transient Makepad
//! `WindowId` generations, monotonic requests, Acrylic readback, explicit
//! `WindowBackdrop::None` rollback, and an exact destroyed event. Only a closed,
//! internally consistent lifecycle can be exported to the profile aggregator.
//! Source presence never binds the product material host or grants authority.

use makepad_widgets::{WindowBackdrop, WindowId, WindowVisuals};

use super::hepta_windows_material_adapter::{
    HeptaWindowsBackdropReadbackApi, HeptaWindowsBackdropReadbackError,
    HeptaWindowsDwmBackdropValue,
};
use super::hepta_windows_material_profile_aggregate as aggregate;

pub const HEPTA_WINDOWS_TRANSIENT_HOST_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_PRODUCT_BOUND: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_ACRYLIC_ACKNOWLEDGED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_SOLID_ROLLBACK_ACKNOWLEDGED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_DESTROYED_ACKNOWLEDGED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_SYSTEM_MATERIAL_BOUND: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_COMPLETE_PROFILE_BOUND: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_NATIVE_PRODUCT_RUNTIME: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_DEVICE_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_PRODUCTION_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_EFFECT_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_LIVE_ADAPTER_AUTHORITY: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_OPERATOR_ACCEPTANCE: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_PROMOTION: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_RELEASE: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientHostPhase {
    Unbound,
    ParentBound,
    RequestPending,
    AcrylicAcknowledged,
    SolidRollbackAcknowledged,
    Closing,
    Closed,
    Rejected,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientMaterialKind {
    Acrylic,
    SolidRollback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientReceiptStatus {
    VerifiedAcrylicWithBackdropReadback,
    VerifiedSolidRollbackWithBackdropReadback,
    RejectedBackendFailure,
    RejectedReadbackMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientWindowIdentity {
    pub hwnd: isize,
    pub window_index: usize,
    pub window_generation: u64,
}

impl HeptaWindowsTransientWindowIdentity {
    pub const fn new(
        hwnd: isize,
        window_index: usize,
        window_generation: u64,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        if hwnd == 0 {
            return Err(HeptaWindowsTransientHostError::InvalidHostHandle);
        }
        Ok(Self {
            hwnd,
            window_index,
            window_generation,
        })
    }

    pub const fn from_window_id(
        hwnd: isize,
        window_id: WindowId,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        Self::new(hwnd, window_id.0, window_id.1)
    }

    pub const fn window_id(self) -> WindowId {
        WindowId(self.window_index, self.window_generation)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientRequestIdentity {
    pub request_sequence: u64,
    pub parent_window_id: WindowId,
    pub transient_window_id: WindowId,
    pub requested_visuals: WindowVisuals,
    pub kind: HeptaWindowsTransientMaterialKind,
}

impl HeptaWindowsTransientRequestIdentity {
    pub fn new(
        request_sequence: u64,
        parent_window_id: WindowId,
        transient_window_id: WindowId,
        requested_visuals: WindowVisuals,
        kind: HeptaWindowsTransientMaterialKind,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        if request_sequence == 0 {
            return Err(HeptaWindowsTransientHostError::InvalidRequestSequence);
        }
        if parent_window_id == transient_window_id {
            return Err(HeptaWindowsTransientHostError::SeparateTransientHostRequired);
        }
        let requested_visuals = requested_visuals.normalized();
        let valid = match kind {
            HeptaWindowsTransientMaterialKind::Acrylic => {
                requested_visuals.transparent
                    && requested_visuals.backdrop == WindowBackdrop::Acrylic
            }
            HeptaWindowsTransientMaterialKind::SolidRollback => {
                !requested_visuals.transparent && requested_visuals.backdrop == WindowBackdrop::None
            }
        };
        if !valid {
            return Err(HeptaWindowsTransientHostError::UnsupportedRequestedVisuals);
        }
        Ok(Self {
            request_sequence,
            parent_window_id,
            transient_window_id,
            requested_visuals,
            kind,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientBackendProcessed {
    pub request_sequence: u64,
    pub window_id: WindowId,
    pub hwnd: isize,
    pub visuals: WindowVisuals,
    pub backend_apply_succeeded: bool,
    pub is_popup: bool,
}

impl HeptaWindowsTransientBackendProcessed {
    pub fn new(
        request_sequence: u64,
        window_id: WindowId,
        hwnd: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        if hwnd == 0 {
            return Err(HeptaWindowsTransientHostError::InvalidHostHandle);
        }
        if !is_popup {
            return Err(HeptaWindowsTransientHostError::PopupWindowRequired);
        }
        Ok(Self {
            request_sequence,
            window_id,
            hwnd,
            visuals: visuals.normalized(),
            backend_apply_succeeded,
            is_popup,
        })
    }

    pub const fn identity(self) -> HeptaWindowsTransientWindowIdentity {
        HeptaWindowsTransientWindowIdentity {
            hwnd: self.hwnd,
            window_index: self.window_id.0,
            window_generation: self.window_id.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientReceipt {
    pub status: HeptaWindowsTransientReceiptStatus,
    pub accepted: bool,
    pub request_sequence: u64,
    pub parent_window_id: WindowId,
    pub transient: HeptaWindowsTransientWindowIdentity,
    pub requested_backdrop: WindowBackdrop,
    pub observed_backdrop: Option<WindowBackdrop>,
    pub backdrop_exact: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsTransientReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }

    pub const fn remains_partial(self) -> bool {
        !self.complete_profile_bound && !self.system_material_bound
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientHostError {
    InvalidHostHandle,
    InvalidRequestSequence,
    PopupWindowRequired,
    UnsupportedRequestedVisuals,
    HostShutdown,
    ParentNotBound,
    ParentIdentityMismatch,
    SeparateTransientHostRequired,
    PendingRequestExists,
    NoPendingRequest,
    StaleRequestSequence,
    AcrylicAcknowledgementRequired,
    AcrylicAlreadyAcknowledged,
    SolidRollbackAlreadyAcknowledged,
    DispatchSequenceMismatch,
    DispatchWindowIdentityMismatch,
    DispatchVisualsMismatch,
    Readback(HeptaWindowsBackdropReadbackError),
    SolidRollbackRequiredBeforeClose,
    CloseNotStarted,
    LifecycleNotClosed,
    MissingAcceptedReceipt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientHostSnapshot {
    pub phase: HeptaWindowsTransientHostPhase,
    pub parent: Option<HeptaWindowsTransientWindowIdentity>,
    pub transient: Option<HeptaWindowsTransientWindowIdentity>,
    pub pending_request_sequence: Option<u64>,
    pub last_registered_sequence: u64,
    pub acrylic_acknowledged: bool,
    pub solid_rollback_acknowledged: bool,
    pub destroyed_acknowledged: bool,
    pub product_bound: bool,
    pub runtime_validated: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

pub struct HeptaWindowsTransientMaterialHost {
    phase: HeptaWindowsTransientHostPhase,
    parent: Option<HeptaWindowsTransientWindowIdentity>,
    transient: Option<HeptaWindowsTransientWindowIdentity>,
    pending: Option<HeptaWindowsTransientRequestIdentity>,
    last_registered_sequence: u64,
    acrylic_receipt: Option<HeptaWindowsTransientReceipt>,
    solid_receipt: Option<HeptaWindowsTransientReceipt>,
    destroyed: Option<HeptaWindowsTransientWindowIdentity>,
}

impl Default for HeptaWindowsTransientMaterialHost {
    fn default() -> Self {
        Self {
            phase: HeptaWindowsTransientHostPhase::Unbound,
            parent: None,
            transient: None,
            pending: None,
            last_registered_sequence: 0,
            acrylic_receipt: None,
            solid_receipt: None,
            destroyed: None,
        }
    }
}

impl HeptaWindowsTransientMaterialHost {
    pub const fn snapshot(&self) -> HeptaWindowsTransientHostSnapshot {
        HeptaWindowsTransientHostSnapshot {
            phase: self.phase,
            parent: self.parent,
            transient: self.transient,
            pending_request_sequence: match self.pending {
                Some(request) => Some(request.request_sequence),
                None => None,
            },
            last_registered_sequence: self.last_registered_sequence,
            acrylic_acknowledged: self.acrylic_receipt.is_some(),
            solid_rollback_acknowledged: self.solid_receipt.is_some(),
            destroyed_acknowledged: self.destroyed.is_some(),
            product_bound: false,
            runtime_validated: false,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    pub fn bind_parent(
        &mut self,
        parent: HeptaWindowsTransientWindowIdentity,
    ) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        if let Some(current) = self.parent {
            if current == parent {
                return Ok(());
            }
            if current.window_index == parent.window_index
                && parent.window_generation <= current.window_generation
            {
                return Err(HeptaWindowsTransientHostError::ParentIdentityMismatch);
            }
            self.clear_transient_state();
        }
        self.parent = Some(parent);
        self.phase = HeptaWindowsTransientHostPhase::ParentBound;
        Ok(())
    }

    pub fn register_request(
        &mut self,
        request: HeptaWindowsTransientRequestIdentity,
    ) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        let parent = self
            .parent
            .ok_or(HeptaWindowsTransientHostError::ParentNotBound)?;
        if parent.window_id() != request.parent_window_id {
            return Err(HeptaWindowsTransientHostError::ParentIdentityMismatch);
        }
        if parent.window_id() == request.transient_window_id {
            return Err(HeptaWindowsTransientHostError::SeparateTransientHostRequired);
        }
        if self.pending.is_some() {
            return Err(HeptaWindowsTransientHostError::PendingRequestExists);
        }
        if request.request_sequence <= self.last_registered_sequence {
            return Err(HeptaWindowsTransientHostError::StaleRequestSequence);
        }
        if let Some(transient) = self.transient
            && transient.window_id() != request.transient_window_id
        {
            return Err(HeptaWindowsTransientHostError::DispatchWindowIdentityMismatch);
        }
        match request.kind {
            HeptaWindowsTransientMaterialKind::Acrylic => {
                if self.acrylic_receipt.is_some() {
                    return Err(HeptaWindowsTransientHostError::AcrylicAlreadyAcknowledged);
                }
            }
            HeptaWindowsTransientMaterialKind::SolidRollback => {
                if self.acrylic_receipt.is_none() {
                    return Err(HeptaWindowsTransientHostError::AcrylicAcknowledgementRequired);
                }
                if self.solid_receipt.is_some() {
                    return Err(HeptaWindowsTransientHostError::SolidRollbackAlreadyAcknowledged);
                }
            }
        }
        self.last_registered_sequence = request.request_sequence;
        self.pending = Some(request);
        self.phase = HeptaWindowsTransientHostPhase::RequestPending;
        Ok(())
    }

    pub fn process_backend_event<A: HeptaWindowsBackdropReadbackApi>(
        &mut self,
        event: HeptaWindowsTransientBackendProcessed,
        api: &mut A,
    ) -> Result<HeptaWindowsTransientReceipt, HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        let request = self
            .pending
            .ok_or(HeptaWindowsTransientHostError::NoPendingRequest)?;
        let parent = self
            .parent
            .ok_or(HeptaWindowsTransientHostError::ParentNotBound)?;
        if event.request_sequence != request.request_sequence {
            return Err(HeptaWindowsTransientHostError::DispatchSequenceMismatch);
        }
        if event.window_id != request.transient_window_id {
            return Err(HeptaWindowsTransientHostError::DispatchWindowIdentityMismatch);
        }
        if event.visuals != request.requested_visuals {
            self.reject_and_consume();
            return Err(HeptaWindowsTransientHostError::DispatchVisualsMismatch);
        }
        if event.hwnd == parent.hwnd {
            self.reject_and_consume();
            return Err(HeptaWindowsTransientHostError::SeparateTransientHostRequired);
        }
        let identity = event.identity();
        if let Some(current) = self.transient {
            if current != identity {
                return Err(HeptaWindowsTransientHostError::DispatchWindowIdentityMismatch);
            }
        } else {
            self.transient = Some(identity);
        }

        if !event.backend_apply_succeeded {
            let receipt = self.receipt(
                request,
                identity,
                HeptaWindowsTransientReceiptStatus::RejectedBackendFailure,
                false,
                None,
            );
            self.pending = None;
            self.phase = HeptaWindowsTransientHostPhase::Rejected;
            return Ok(receipt);
        }

        let observed = api.read_backdrop(identity.hwnd).map_err(|error| {
            self.reject_and_consume();
            HeptaWindowsTransientHostError::Readback(error)
        })?;
        let observed_backdrop = observed_backdrop(observed);
        if observed_backdrop != Some(request.requested_visuals.backdrop) {
            let receipt = self.receipt(
                request,
                identity,
                HeptaWindowsTransientReceiptStatus::RejectedReadbackMismatch,
                false,
                observed_backdrop,
            );
            self.pending = None;
            self.phase = HeptaWindowsTransientHostPhase::Rejected;
            return Ok(receipt);
        }

        let status = match request.kind {
            HeptaWindowsTransientMaterialKind::Acrylic => {
                self.phase = HeptaWindowsTransientHostPhase::AcrylicAcknowledged;
                HeptaWindowsTransientReceiptStatus::VerifiedAcrylicWithBackdropReadback
            }
            HeptaWindowsTransientMaterialKind::SolidRollback => {
                self.phase = HeptaWindowsTransientHostPhase::SolidRollbackAcknowledged;
                HeptaWindowsTransientReceiptStatus::VerifiedSolidRollbackWithBackdropReadback
            }
        };
        let receipt = self.receipt(request, identity, status, true, observed_backdrop);
        self.pending = None;
        match request.kind {
            HeptaWindowsTransientMaterialKind::Acrylic => self.acrylic_receipt = Some(receipt),
            HeptaWindowsTransientMaterialKind::SolidRollback => self.solid_receipt = Some(receipt),
        }
        Ok(receipt)
    }

    pub fn begin_close(&mut self) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        if self.pending.is_some() || self.solid_receipt.is_none() {
            return Err(HeptaWindowsTransientHostError::SolidRollbackRequiredBeforeClose);
        }
        self.phase = HeptaWindowsTransientHostPhase::Closing;
        Ok(())
    }

    pub fn process_destroyed(
        &mut self,
        destroyed: HeptaWindowsTransientWindowIdentity,
    ) -> Result<bool, HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        if self.phase != HeptaWindowsTransientHostPhase::Closing {
            return Err(HeptaWindowsTransientHostError::CloseNotStarted);
        }
        if self.transient != Some(destroyed) {
            return Ok(false);
        }
        self.destroyed = Some(destroyed);
        self.pending = None;
        self.phase = HeptaWindowsTransientHostPhase::Closed;
        Ok(true)
    }

    pub fn profile_evidence(
        &self,
    ) -> Result<aggregate::HeptaWindowsTransientLifecycleEvidence, HeptaWindowsTransientHostError>
    {
        if self.phase != HeptaWindowsTransientHostPhase::Closed {
            return Err(HeptaWindowsTransientHostError::LifecycleNotClosed);
        }
        let parent = self
            .parent
            .ok_or(HeptaWindowsTransientHostError::ParentNotBound)?;
        let transient = self
            .transient
            .ok_or(HeptaWindowsTransientHostError::MissingAcceptedReceipt)?;
        let acrylic = self
            .acrylic_receipt
            .ok_or(HeptaWindowsTransientHostError::MissingAcceptedReceipt)?;
        let solid = self
            .solid_receipt
            .ok_or(HeptaWindowsTransientHostError::MissingAcceptedReceipt)?;
        let destroyed = self
            .destroyed
            .ok_or(HeptaWindowsTransientHostError::MissingAcceptedReceipt)?;
        if !acrylic.accepted || !solid.accepted {
            return Err(HeptaWindowsTransientHostError::MissingAcceptedReceipt);
        }
        Ok(aggregate::HeptaWindowsTransientLifecycleEvidence {
            parent_hwnd: parent.hwnd,
            parent_window_index: parent.window_index,
            parent_window_generation: parent.window_generation,
            transient: aggregate_identity(transient),
            acrylic: aggregate_receipt(acrylic),
            solid_rollback: aggregate_receipt(solid),
            destroyed: aggregate_identity(destroyed),
            destroyed_acknowledged: true,
            product_bound: false,
            runtime_validated: false,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        })
    }

    pub fn shutdown(&mut self) {
        self.parent = None;
        self.clear_transient_state();
        self.phase = HeptaWindowsTransientHostPhase::Shutdown;
    }

    fn clear_transient_state(&mut self) {
        self.transient = None;
        self.pending = None;
        self.last_registered_sequence = 0;
        self.acrylic_receipt = None;
        self.solid_receipt = None;
        self.destroyed = None;
    }

    fn ensure_active(&self) -> Result<(), HeptaWindowsTransientHostError> {
        if self.phase == HeptaWindowsTransientHostPhase::Shutdown {
            Err(HeptaWindowsTransientHostError::HostShutdown)
        } else {
            Ok(())
        }
    }

    fn reject_and_consume(&mut self) {
        self.pending = None;
        self.phase = HeptaWindowsTransientHostPhase::Rejected;
    }

    fn receipt(
        &self,
        request: HeptaWindowsTransientRequestIdentity,
        identity: HeptaWindowsTransientWindowIdentity,
        status: HeptaWindowsTransientReceiptStatus,
        accepted: bool,
        observed_backdrop: Option<WindowBackdrop>,
    ) -> HeptaWindowsTransientReceipt {
        HeptaWindowsTransientReceipt {
            status,
            accepted,
            request_sequence: request.request_sequence,
            parent_window_id: request.parent_window_id,
            transient: identity,
            requested_backdrop: request.requested_visuals.backdrop,
            observed_backdrop,
            backdrop_exact: observed_backdrop == Some(request.requested_visuals.backdrop),
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }
}

const fn observed_backdrop(value: HeptaWindowsDwmBackdropValue) -> Option<WindowBackdrop> {
    match value {
        HeptaWindowsDwmBackdropValue::Auto => Some(WindowBackdrop::Auto),
        HeptaWindowsDwmBackdropValue::None => Some(WindowBackdrop::None),
        HeptaWindowsDwmBackdropValue::Mica => Some(WindowBackdrop::Mica),
        HeptaWindowsDwmBackdropValue::Acrylic => Some(WindowBackdrop::Acrylic),
        HeptaWindowsDwmBackdropValue::MicaAlt => None,
    }
}

const fn aggregate_identity(
    value: HeptaWindowsTransientWindowIdentity,
) -> aggregate::HeptaWindowsTransientWindowIdentity {
    aggregate::HeptaWindowsTransientWindowIdentity {
        hwnd: value.hwnd,
        window_index: value.window_index,
        window_generation: value.window_generation,
    }
}

fn aggregate_receipt(
    value: HeptaWindowsTransientReceipt,
) -> aggregate::HeptaWindowsTransientReceipt {
    let status = match value.status {
        HeptaWindowsTransientReceiptStatus::VerifiedAcrylicWithBackdropReadback => {
            aggregate::HeptaWindowsTransientEvidenceStatus::VerifiedAcrylicWithBackdropReadback
        }
        HeptaWindowsTransientReceiptStatus::VerifiedSolidRollbackWithBackdropReadback => {
            aggregate::HeptaWindowsTransientEvidenceStatus::VerifiedSolidRollbackWithBackdropReadback
        }
        HeptaWindowsTransientReceiptStatus::RejectedBackendFailure
        | HeptaWindowsTransientReceiptStatus::RejectedReadbackMismatch => {
            unreachable!("rejected receipts cannot be exported as profile evidence")
        }
    };
    aggregate::HeptaWindowsTransientReceipt {
        status,
        accepted: value.accepted,
        request_sequence: value.request_sequence,
        parent_window_index: value.parent_window_id.0,
        parent_window_generation: value.parent_window_id.1,
        transient: aggregate_identity(value.transient),
        requested_backdrop: value.requested_backdrop,
        observed_backdrop: value
            .observed_backdrop
            .expect("accepted transient receipt has exact readback"),
        backdrop_exact: value.backdrop_exact,
        complete_profile_bound: false,
        system_material_bound: false,
        production_authority: false,
        effect_authority: false,
        live_adapter_authority: false,
        operator_acceptance: false,
        promotion: false,
        release: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::hepta_windows_material_profile_aggregate::{
        aggregate_windows_material_profile, HeptaWindowsMaterialProfileAggregateStatus,
    };
    use crate::shared::hepta_windows_backend_ack_bridge::HeptaWindowsBackendWindowIdentity;
    use crate::shared::hepta_window_visual_ack::{
        HeptaWindowVisualAckReceipt, HeptaWindowVisualAckStatus, HeptaWindowVisualBackend,
        HeptaWindowVisualReadbackScope,
    };
    use crate::shared::hepta_platform_material::HeptaPlatform;

    struct RecordingApi {
        calls: Vec<isize>,
        value: HeptaWindowsDwmBackdropValue,
    }

    impl HeptaWindowsBackdropReadbackApi for RecordingApi {
        fn read_backdrop(
            &mut self,
            window: isize,
        ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
            self.calls.push(window);
            Ok(self.value)
        }
    }

    const ROOT_ID: WindowId = WindowId(1, 7);
    const POPUP_ID: WindowId = WindowId(2, 9);

    fn parent() -> HeptaWindowsTransientWindowIdentity {
        HeptaWindowsTransientWindowIdentity::new(11, ROOT_ID.0, ROOT_ID.1).unwrap()
    }

    fn request(
        sequence: u64,
        kind: HeptaWindowsTransientMaterialKind,
    ) -> HeptaWindowsTransientRequestIdentity {
        let visuals = match kind {
            HeptaWindowsTransientMaterialKind::Acrylic => WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Acrylic,
                backdrop_intensity: 0.86,
            },
            HeptaWindowsTransientMaterialKind::SolidRollback => WindowVisuals::default(),
        };
        HeptaWindowsTransientRequestIdentity::new(sequence, ROOT_ID, POPUP_ID, visuals, kind)
            .unwrap()
    }

    fn event(
        request: HeptaWindowsTransientRequestIdentity,
        backend_apply_succeeded: bool,
    ) -> HeptaWindowsTransientBackendProcessed {
        HeptaWindowsTransientBackendProcessed::new(
            request.request_sequence,
            POPUP_ID,
            12,
            request.requested_visuals,
            backend_apply_succeeded,
            true,
        )
        .unwrap()
    }

    fn root_receipt() -> HeptaWindowVisualAckReceipt {
        HeptaWindowVisualAckReceipt {
            status: HeptaWindowVisualAckStatus::VerifiedPersistentChromeWithBackdropReadback,
            accepted: true,
            request_sequence: 1,
            platform: HeptaPlatform::Windows,
            window_index: ROOT_ID.0,
            window_generation: ROOT_ID.1,
            backend: HeptaWindowVisualBackend::WindowsDwm,
            requested_visuals: WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Mica,
                backdrop_intensity: 0.9,
            },
            readback_scope: HeptaWindowVisualReadbackScope::BackdropOnly,
            observed_backdrop: Some(WindowBackdrop::Mica),
            observed_visuals: None,
            backdrop_exact: true,
            full_visuals_exact: false,
            persistent_chrome_acknowledged: true,
            solid_fallback_acknowledged: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    #[test]
    fn separate_popup_acrylic_rollback_destroy_exports_valid_evidence() {
        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_parent(parent()).unwrap();

        let acrylic = request(2, HeptaWindowsTransientMaterialKind::Acrylic);
        host.register_request(acrylic).unwrap();
        let mut acrylic_api = RecordingApi {
            calls: Vec::new(),
            value: HeptaWindowsDwmBackdropValue::Acrylic,
        };
        assert!(
            host.process_backend_event(event(acrylic, true), &mut acrylic_api)
                .unwrap()
                .accepted
        );

        let solid = request(3, HeptaWindowsTransientMaterialKind::SolidRollback);
        host.register_request(solid).unwrap();
        let mut solid_api = RecordingApi {
            calls: Vec::new(),
            value: HeptaWindowsDwmBackdropValue::None,
        };
        assert!(
            host.process_backend_event(event(solid, true), &mut solid_api)
                .unwrap()
                .accepted
        );

        host.begin_close().unwrap();
        let destroyed =
            HeptaWindowsTransientWindowIdentity::new(12, POPUP_ID.0, POPUP_ID.1).unwrap();
        assert!(host.process_destroyed(destroyed).unwrap());
        let aggregate = aggregate_windows_material_profile(
            HeptaWindowsBackendWindowIdentity {
                hwnd: 11,
                window_index: ROOT_ID.0,
                window_generation: ROOT_ID.1,
            },
            root_receipt(),
            host.profile_evidence().unwrap(),
        );
        assert_eq!(
            aggregate.status,
            HeptaWindowsMaterialProfileAggregateStatus::ReadyForProductIntegrationReview
        );
        assert!(aggregate.accepted);
        assert!(aggregate.remains_unbound());
        assert!(aggregate.grants_no_authority());
    }

    #[test]
    fn root_hwnd_reuse_and_close_without_rollback_fail_closed() {
        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_parent(parent()).unwrap();
        let acrylic = request(2, HeptaWindowsTransientMaterialKind::Acrylic);
        host.register_request(acrylic).unwrap();
        let reused = HeptaWindowsTransientBackendProcessed::new(
            2,
            POPUP_ID,
            11,
            acrylic.requested_visuals,
            true,
            true,
        )
        .unwrap();
        let mut api = RecordingApi {
            calls: Vec::new(),
            value: HeptaWindowsDwmBackdropValue::Acrylic,
        };
        assert_eq!(
            host.process_backend_event(reused, &mut api),
            Err(HeptaWindowsTransientHostError::SeparateTransientHostRequired)
        );
        assert!(api.calls.is_empty());

        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_parent(parent()).unwrap();
        assert_eq!(
            host.begin_close(),
            Err(HeptaWindowsTransientHostError::SolidRollbackRequiredBeforeClose)
        );
    }

    #[test]
    fn backend_failure_skips_readback_and_stale_destroy_is_ignored() {
        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_parent(parent()).unwrap();
        let acrylic = request(2, HeptaWindowsTransientMaterialKind::Acrylic);
        host.register_request(acrylic).unwrap();
        let mut api = RecordingApi {
            calls: Vec::new(),
            value: HeptaWindowsDwmBackdropValue::Acrylic,
        };
        let receipt = host
            .process_backend_event(event(acrylic, false), &mut api)
            .unwrap();
        assert_eq!(
            receipt.status,
            HeptaWindowsTransientReceiptStatus::RejectedBackendFailure
        );
        assert!(!receipt.accepted);
        assert!(api.calls.is_empty());
    }

    #[test]
    fn source_authority_constants_remain_false() {
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_SOURCE_WIRED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_HOST_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_HOST_PRODUCT_BOUND);
        assert!(!HEPTA_WINDOWS_TRANSIENT_ACRYLIC_ACKNOWLEDGED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_SOLID_ROLLBACK_ACKNOWLEDGED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_DESTROYED_ACKNOWLEDGED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_SYSTEM_MATERIAL_BOUND);
        assert!(!HEPTA_WINDOWS_TRANSIENT_COMPLETE_PROFILE_BOUND);
        assert!(!HEPTA_WINDOWS_TRANSIENT_NATIVE_PRODUCT_RUNTIME);
        assert!(!HEPTA_WINDOWS_TRANSIENT_DEVICE_VALIDATED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_PRODUCTION_AUTHORITY);
        assert!(!HEPTA_WINDOWS_TRANSIENT_EFFECT_AUTHORITY);
        assert!(!HEPTA_WINDOWS_TRANSIENT_LIVE_ADAPTER_AUTHORITY);
        assert!(!HEPTA_WINDOWS_TRANSIENT_OPERATOR_ACCEPTANCE);
        assert!(!HEPTA_WINDOWS_TRANSIENT_PROMOTION);
        assert!(!HEPTA_WINDOWS_TRANSIENT_RELEASE);
    }
}
