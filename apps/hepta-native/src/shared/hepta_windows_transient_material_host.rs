//! Governed Windows transient-material host for Hepta UI v4.
//!
//! Persistent root-window Mica and transient Acrylic are deliberately separate
//! lifecycles. This host accepts only an exact popup `WindowId`, obtains its
//! explicit HWND from the correlated Makepad backend event, requires the HWND
//! and full `WindowId` to differ from the persistent root, and validates only
//! `DWMWA_SYSTEMBACKDROP_TYPE` readback.
//!
//! Source presence does not bind a product host or grant complete-profile,
//! effect, production, operator, promotion, or release authority.

use makepad_widgets::{WindowBackdrop, WindowVisuals};

use super::hepta_windows_backend_ack_bridge::HeptaWindowsBackendWindowIdentity;
use super::hepta_windows_material_adapter::{
    HeptaWindowsBackdropReadbackApi, HeptaWindowsBackdropReadbackError,
    HeptaWindowsDwmBackdropValue,
};

pub const HEPTA_WINDOWS_TRANSIENT_HOST_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_POPUP_REQUIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_SEPARATE_HWND_REQUIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_FOCUS_LOSS_CLEANUP_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_CLOSE_CLEANUP_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_ROLLBACK_SOURCE_WIRED: bool = true;
pub const HEPTA_WINDOWS_TRANSIENT_HOST_BOUND: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_RUNTIME_VALIDATED: bool = false;
pub const HEPTA_WINDOWS_TRANSIENT_BACKDROP_READBACK_VALIDATED: bool = false;
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
    RootBound,
    TransientObserved,
    AcrylicPending,
    AcrylicAcknowledged,
    CleanupPending,
    SolidAcknowledged,
    RollbackRequired,
    ClosePending,
    Closed,
    Rejected,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientCleanupReason {
    FocusLost,
    Close,
    Rollback,
    Suspend,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientRequestKind {
    Acrylic,
    SolidCleanup(HeptaWindowsTransientCleanupReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientFrameworkWindowIdentity {
    pub window_index: usize,
    pub window_generation: u64,
    pub is_popup: bool,
}

impl HeptaWindowsTransientFrameworkWindowIdentity {
    pub const fn new(
        window_index: usize,
        window_generation: u64,
        is_popup: bool,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        if !is_popup {
            return Err(HeptaWindowsTransientHostError::PopupRequired);
        }
        Ok(Self {
            window_index,
            window_generation,
            is_popup,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaWindowsTransientNativeWindowIdentity {
    pub hwnd: isize,
    pub window_index: usize,
    pub window_generation: u64,
}

impl HeptaWindowsTransientNativeWindowIdentity {
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientRequest {
    pub sequence: u64,
    pub window: HeptaWindowsTransientFrameworkWindowIdentity,
    pub kind: HeptaWindowsTransientRequestKind,
    pub visuals: WindowVisuals,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientVisualsProcessed {
    pub request_sequence: u64,
    pub window_index: usize,
    pub window_generation: u64,
    pub hwnd: isize,
    pub visuals: WindowVisuals,
    pub backend_apply_succeeded: bool,
    pub is_popup: bool,
}

impl HeptaWindowsTransientVisualsProcessed {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_sequence: u64,
        window_index: usize,
        window_generation: u64,
        hwnd: isize,
        visuals: WindowVisuals,
        backend_apply_succeeded: bool,
        is_popup: bool,
    ) -> Result<Self, HeptaWindowsTransientHostError> {
        if hwnd == 0 {
            return Err(HeptaWindowsTransientHostError::InvalidHostHandle);
        }
        if !is_popup {
            return Err(HeptaWindowsTransientHostError::PopupRequired);
        }
        Ok(Self {
            request_sequence,
            window_index,
            window_generation,
            hwnd,
            visuals: visuals.normalized(),
            backend_apply_succeeded,
            is_popup,
        })
    }

    pub const fn native_identity(self) -> HeptaWindowsTransientNativeWindowIdentity {
        HeptaWindowsTransientNativeWindowIdentity {
            hwnd: self.hwnd,
            window_index: self.window_index,
            window_generation: self.window_generation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientAckStatus {
    VerifiedAcrylicWithBackdropReadback,
    VerifiedSolidCleanupWithBackdropReadback,
    RejectedBackendFailure,
    RejectedReadbackMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientAckReceipt {
    pub status: HeptaWindowsTransientAckStatus,
    pub accepted: bool,
    pub sequence: u64,
    pub window_index: usize,
    pub window_generation: u64,
    pub hwnd: isize,
    pub request_kind: HeptaWindowsTransientRequestKind,
    pub requested_visuals: WindowVisuals,
    pub observed_backdrop: Option<HeptaWindowsDwmBackdropValue>,
    pub backdrop_exact: bool,
    pub acrylic_backdrop_acknowledged: bool,
    pub solid_cleanup_acknowledged: bool,
    pub cleanup_reason: Option<HeptaWindowsTransientCleanupReason>,
    pub close_ready: bool,
    pub full_visuals_exact: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub device_validated: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsTransientAckReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }

    pub const fn remains_partial(self) -> bool {
        !self.full_visuals_exact
            && !self.transient_system_material_bound
            && !self.complete_profile_bound
            && !self.system_material_bound
            && !self.native_product_runtime
            && !self.device_validated
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientDestroyReceipt {
    pub accepted: bool,
    pub exact_window_identity: bool,
    pub cleanup_confirmed_before_close: bool,
    pub closed: bool,
    pub window_index: usize,
    pub window_generation: u64,
    pub hwnd: isize,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub device_validated: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

impl HeptaWindowsTransientDestroyReceipt {
    pub const fn grants_no_authority(self) -> bool {
        !self.production_authority
            && !self.effect_authority
            && !self.live_adapter_authority
            && !self.operator_acceptance
            && !self.promotion
            && !self.release
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeptaWindowsTransientHostSnapshot {
    pub phase: HeptaWindowsTransientHostPhase,
    pub root: Option<HeptaWindowsBackendWindowIdentity>,
    pub transient_framework: Option<HeptaWindowsTransientFrameworkWindowIdentity>,
    pub transient_native: Option<HeptaWindowsTransientNativeWindowIdentity>,
    pub pending_sequence: Option<u64>,
    pub last_sequence: u64,
    pub last_ack_accepted: Option<bool>,
    pub host_bound: bool,
    pub runtime_validated: bool,
    pub transient_system_material_bound: bool,
    pub complete_profile_bound: bool,
    pub system_material_bound: bool,
    pub native_product_runtime: bool,
    pub device_validated: bool,
    pub production_authority: bool,
    pub effect_authority: bool,
    pub live_adapter_authority: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeptaWindowsTransientHostError {
    InvalidHostHandle,
    PopupRequired,
    HostShutdown,
    RootNotBound,
    TransientNotObserved,
    RootHandleReuse,
    RootWindowIdentityReuse,
    StaleWindowGeneration,
    PendingRequestExists,
    NoPendingRequest,
    StaleRequestSequence,
    InvalidPhase,
    WindowIdentityMismatch,
    DispatchSequenceMismatch,
    DispatchVisualsMismatch,
    CleanupRequiredBeforeClose,
    Readback(HeptaWindowsBackdropReadbackError),
}

pub struct HeptaWindowsTransientMaterialHost {
    phase: HeptaWindowsTransientHostPhase,
    root: Option<HeptaWindowsBackendWindowIdentity>,
    transient_framework: Option<HeptaWindowsTransientFrameworkWindowIdentity>,
    transient_native: Option<HeptaWindowsTransientNativeWindowIdentity>,
    pending: Option<HeptaWindowsTransientRequest>,
    last_sequence: u64,
    last_receipt: Option<HeptaWindowsTransientAckReceipt>,
}

impl Default for HeptaWindowsTransientMaterialHost {
    fn default() -> Self {
        Self {
            phase: HeptaWindowsTransientHostPhase::Unbound,
            root: None,
            transient_framework: None,
            transient_native: None,
            pending: None,
            last_sequence: 0,
            last_receipt: None,
        }
    }
}

impl HeptaWindowsTransientMaterialHost {
    pub const fn phase(&self) -> HeptaWindowsTransientHostPhase {
        self.phase
    }

    pub const fn pending_request(&self) -> Option<HeptaWindowsTransientRequest> {
        self.pending
    }

    pub const fn last_receipt(&self) -> Option<HeptaWindowsTransientAckReceipt> {
        self.last_receipt
    }

    pub const fn snapshot(&self) -> HeptaWindowsTransientHostSnapshot {
        HeptaWindowsTransientHostSnapshot {
            phase: self.phase,
            root: self.root,
            transient_framework: self.transient_framework,
            transient_native: self.transient_native,
            pending_sequence: match self.pending {
                Some(request) => Some(request.sequence),
                None => None,
            },
            last_sequence: self.last_sequence,
            last_ack_accepted: match self.last_receipt {
                Some(receipt) => Some(receipt.accepted),
                None => None,
            },
            host_bound: false,
            runtime_validated: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            native_product_runtime: false,
            device_validated: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    pub fn bind_root(
        &mut self,
        root: HeptaWindowsBackendWindowIdentity,
    ) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        if root.hwnd == 0 {
            return Err(HeptaWindowsTransientHostError::InvalidHostHandle);
        }
        if let Some(current) = self.root {
            if current == root {
                return Ok(());
            }
            if current.window_index == root.window_index
                && root.window_generation <= current.window_generation
            {
                return Err(HeptaWindowsTransientHostError::StaleWindowGeneration);
            }
            if self.pending.is_some() {
                return Err(HeptaWindowsTransientHostError::PendingRequestExists);
            }
            self.clear_transient();
        }
        self.root = Some(root);
        self.phase = HeptaWindowsTransientHostPhase::RootBound;
        Ok(())
    }

    pub fn observe_transient_window(
        &mut self,
        identity: HeptaWindowsTransientFrameworkWindowIdentity,
    ) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        let root = self
            .root
            .ok_or(HeptaWindowsTransientHostError::RootNotBound)?;
        if !identity.is_popup {
            return Err(HeptaWindowsTransientHostError::PopupRequired);
        }
        if identity.window_index == root.window_index
            && identity.window_generation == root.window_generation
        {
            return Err(HeptaWindowsTransientHostError::RootWindowIdentityReuse);
        }
        if let Some(current) = self.transient_framework {
            if current == identity {
                return Ok(());
            }
            if self.pending.is_some() {
                return Err(HeptaWindowsTransientHostError::PendingRequestExists);
            }
            if current.window_index == identity.window_index
                && identity.window_generation <= current.window_generation
            {
                return Err(HeptaWindowsTransientHostError::StaleWindowGeneration);
            }
        }
        self.transient_framework = Some(identity);
        self.transient_native = None;
        self.last_receipt = None;
        self.phase = HeptaWindowsTransientHostPhase::TransientObserved;
        Ok(())
    }

    pub fn begin_acrylic(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(sequence, HeptaWindowsTransientRequestKind::Acrylic)
    }

    pub fn begin_focus_loss_cleanup(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(
            sequence,
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::FocusLost,
            ),
        )
    }

    pub fn begin_close_cleanup(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(
            sequence,
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::Close,
            ),
        )
    }

    pub fn begin_rollback_cleanup(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(
            sequence,
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::Rollback,
            ),
        )
    }

    pub fn begin_suspend_cleanup(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(
            sequence,
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::Suspend,
            ),
        )
    }

    pub fn begin_shutdown_cleanup(
        &mut self,
        sequence: u64,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.begin_request(
            sequence,
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::Shutdown,
            ),
        )
    }

    pub fn process_backend_event<A: HeptaWindowsBackdropReadbackApi>(
        &mut self,
        event: HeptaWindowsTransientVisualsProcessed,
        api: &mut A,
    ) -> Result<HeptaWindowsTransientAckReceipt, HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        let request = self
            .pending
            .ok_or(HeptaWindowsTransientHostError::NoPendingRequest)?;
        let framework = self
            .transient_framework
            .ok_or(HeptaWindowsTransientHostError::TransientNotObserved)?;
        let root = self
            .root
            .ok_or(HeptaWindowsTransientHostError::RootNotBound)?;

        if event.request_sequence != request.sequence {
            return Err(HeptaWindowsTransientHostError::DispatchSequenceMismatch);
        }
        if event.window_index != framework.window_index
            || event.window_generation != framework.window_generation
            || event.window_index != request.window.window_index
            || event.window_generation != request.window.window_generation
        {
            return Err(HeptaWindowsTransientHostError::WindowIdentityMismatch);
        }
        if event.visuals.normalized() != request.visuals.normalized() {
            return Err(HeptaWindowsTransientHostError::DispatchVisualsMismatch);
        }

        let native = event.native_identity();
        if native.hwnd == root.hwnd {
            return Err(HeptaWindowsTransientHostError::RootHandleReuse);
        }
        if native.window_index == root.window_index
            && native.window_generation == root.window_generation
        {
            return Err(HeptaWindowsTransientHostError::RootWindowIdentityReuse);
        }
        if let Some(current) = self.transient_native {
            if current != native {
                return Err(HeptaWindowsTransientHostError::WindowIdentityMismatch);
            }
        } else {
            self.transient_native = Some(native);
        }

        self.pending = None;
        if !event.backend_apply_succeeded {
            self.phase = match request.kind {
                HeptaWindowsTransientRequestKind::Acrylic => {
                    HeptaWindowsTransientHostPhase::RollbackRequired
                }
                HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                    HeptaWindowsTransientHostPhase::Rejected
                }
            };
            let receipt = self.receipt(
                request,
                native,
                HeptaWindowsTransientAckStatus::RejectedBackendFailure,
                false,
                None,
            );
            self.last_receipt = Some(receipt);
            return Ok(receipt);
        }

        let observed = match api.read_backdrop(native.hwnd) {
            Ok(value) => value,
            Err(error) => {
                self.phase = match request.kind {
                    HeptaWindowsTransientRequestKind::Acrylic => {
                        HeptaWindowsTransientHostPhase::RollbackRequired
                    }
                    HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                        HeptaWindowsTransientHostPhase::Rejected
                    }
                };
                self.last_receipt = None;
                return Err(HeptaWindowsTransientHostError::Readback(error));
            }
        };
        let expected = match request.kind {
            HeptaWindowsTransientRequestKind::Acrylic => {
                HeptaWindowsDwmBackdropValue::Acrylic
            }
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                HeptaWindowsDwmBackdropValue::None
            }
        };
        if observed != expected {
            self.phase = match request.kind {
                HeptaWindowsTransientRequestKind::Acrylic => {
                    HeptaWindowsTransientHostPhase::RollbackRequired
                }
                HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                    HeptaWindowsTransientHostPhase::Rejected
                }
            };
            let receipt = self.receipt(
                request,
                native,
                HeptaWindowsTransientAckStatus::RejectedReadbackMismatch,
                false,
                Some(observed),
            );
            self.last_receipt = Some(receipt);
            return Ok(receipt);
        }

        self.phase = match request.kind {
            HeptaWindowsTransientRequestKind::Acrylic => {
                HeptaWindowsTransientHostPhase::AcrylicAcknowledged
            }
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                HeptaWindowsTransientHostPhase::SolidAcknowledged
            }
        };
        let status = match request.kind {
            HeptaWindowsTransientRequestKind::Acrylic => {
                HeptaWindowsTransientAckStatus::VerifiedAcrylicWithBackdropReadback
            }
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                HeptaWindowsTransientAckStatus::VerifiedSolidCleanupWithBackdropReadback
            }
        };
        let receipt = self.receipt(request, native, status, true, Some(observed));
        self.last_receipt = Some(receipt);
        Ok(receipt)
    }

    pub fn mark_close_requested(&mut self) -> Result<(), HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        let Some(receipt) = self.last_receipt else {
            return Err(HeptaWindowsTransientHostError::CleanupRequiredBeforeClose);
        };
        if self.phase != HeptaWindowsTransientHostPhase::SolidAcknowledged
            || !receipt.accepted
            || receipt.cleanup_reason != Some(HeptaWindowsTransientCleanupReason::Close)
            || !receipt.solid_cleanup_acknowledged
        {
            return Err(HeptaWindowsTransientHostError::CleanupRequiredBeforeClose);
        }
        self.phase = HeptaWindowsTransientHostPhase::ClosePending;
        Ok(())
    }

    pub fn process_destroyed(
        &mut self,
        destroyed: HeptaWindowsTransientNativeWindowIdentity,
    ) -> HeptaWindowsTransientDestroyReceipt {
        let expected = self.transient_native;
        let exact = expected == Some(destroyed);
        let cleanup_confirmed = self.phase == HeptaWindowsTransientHostPhase::ClosePending
            && self.last_receipt.is_some_and(|receipt| {
                receipt.accepted
                    && receipt.cleanup_reason
                        == Some(HeptaWindowsTransientCleanupReason::Close)
                    && receipt.solid_cleanup_acknowledged
            });
        let accepted = exact && cleanup_confirmed;
        if accepted {
            self.clear_transient();
            self.phase = HeptaWindowsTransientHostPhase::Closed;
        } else if exact {
            self.phase = HeptaWindowsTransientHostPhase::Rejected;
        }
        HeptaWindowsTransientDestroyReceipt {
            accepted,
            exact_window_identity: exact,
            cleanup_confirmed_before_close: cleanup_confirmed,
            closed: accepted,
            window_index: destroyed.window_index,
            window_generation: destroyed.window_generation,
            hwnd: destroyed.hwnd,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            native_product_runtime: false,
            device_validated: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    pub fn shutdown(&mut self) -> Result<(), HeptaWindowsTransientHostError> {
        if self.phase == HeptaWindowsTransientHostPhase::Shutdown {
            return Ok(());
        }
        if matches!(
            self.phase,
            HeptaWindowsTransientHostPhase::AcrylicPending
                | HeptaWindowsTransientHostPhase::AcrylicAcknowledged
                | HeptaWindowsTransientHostPhase::CleanupPending
                | HeptaWindowsTransientHostPhase::RollbackRequired
                | HeptaWindowsTransientHostPhase::ClosePending
        ) {
            return Err(HeptaWindowsTransientHostError::CleanupRequiredBeforeClose);
        }
        self.clear_transient();
        self.root = None;
        self.phase = HeptaWindowsTransientHostPhase::Shutdown;
        Ok(())
    }

    fn begin_request(
        &mut self,
        sequence: u64,
        kind: HeptaWindowsTransientRequestKind,
    ) -> Result<HeptaWindowsTransientRequest, HeptaWindowsTransientHostError> {
        self.ensure_active()?;
        if self.pending.is_some() {
            return Err(HeptaWindowsTransientHostError::PendingRequestExists);
        }
        if sequence <= self.last_sequence {
            return Err(HeptaWindowsTransientHostError::StaleRequestSequence);
        }
        let window = self
            .transient_framework
            .ok_or(HeptaWindowsTransientHostError::TransientNotObserved)?;
        let allowed = match kind {
            HeptaWindowsTransientRequestKind::Acrylic => matches!(
                self.phase,
                HeptaWindowsTransientHostPhase::TransientObserved
                    | HeptaWindowsTransientHostPhase::SolidAcknowledged
            ),
            HeptaWindowsTransientRequestKind::SolidCleanup(
                HeptaWindowsTransientCleanupReason::Rollback,
            ) => self.phase == HeptaWindowsTransientHostPhase::RollbackRequired,
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                self.phase == HeptaWindowsTransientHostPhase::AcrylicAcknowledged
            }
        };
        if !allowed {
            return Err(HeptaWindowsTransientHostError::InvalidPhase);
        }
        let visuals = match kind {
            HeptaWindowsTransientRequestKind::Acrylic => WindowVisuals {
                transparent: true,
                backdrop: WindowBackdrop::Acrylic,
                backdrop_intensity: 0.88,
            }
            .normalized(),
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => WindowVisuals::default(),
        };
        let request = HeptaWindowsTransientRequest {
            sequence,
            window,
            kind,
            visuals,
        };
        self.pending = Some(request);
        self.last_sequence = sequence;
        self.last_receipt = None;
        self.phase = match kind {
            HeptaWindowsTransientRequestKind::Acrylic => {
                HeptaWindowsTransientHostPhase::AcrylicPending
            }
            HeptaWindowsTransientRequestKind::SolidCleanup(_) => {
                HeptaWindowsTransientHostPhase::CleanupPending
            }
        };
        Ok(request)
    }

    fn receipt(
        &self,
        request: HeptaWindowsTransientRequest,
        native: HeptaWindowsTransientNativeWindowIdentity,
        status: HeptaWindowsTransientAckStatus,
        accepted: bool,
        observed_backdrop: Option<HeptaWindowsDwmBackdropValue>,
    ) -> HeptaWindowsTransientAckReceipt {
        let cleanup_reason = match request.kind {
            HeptaWindowsTransientRequestKind::Acrylic => None,
            HeptaWindowsTransientRequestKind::SolidCleanup(reason) => Some(reason),
        };
        HeptaWindowsTransientAckReceipt {
            status,
            accepted,
            sequence: request.sequence,
            window_index: native.window_index,
            window_generation: native.window_generation,
            hwnd: native.hwnd,
            request_kind: request.kind,
            requested_visuals: request.visuals.normalized(),
            observed_backdrop,
            backdrop_exact: accepted,
            acrylic_backdrop_acknowledged: accepted
                && request.kind == HeptaWindowsTransientRequestKind::Acrylic,
            solid_cleanup_acknowledged: accepted && cleanup_reason.is_some(),
            cleanup_reason,
            close_ready: accepted
                && cleanup_reason == Some(HeptaWindowsTransientCleanupReason::Close),
            full_visuals_exact: false,
            transient_system_material_bound: false,
            complete_profile_bound: false,
            system_material_bound: false,
            native_product_runtime: false,
            device_validated: false,
            production_authority: false,
            effect_authority: false,
            live_adapter_authority: false,
            operator_acceptance: false,
            promotion: false,
            release: false,
        }
    }

    fn ensure_active(&self) -> Result<(), HeptaWindowsTransientHostError> {
        if self.phase == HeptaWindowsTransientHostPhase::Shutdown {
            Err(HeptaWindowsTransientHostError::HostShutdown)
        } else {
            Ok(())
        }
    }

    fn clear_transient(&mut self) {
        self.transient_framework = None;
        self.transient_native = None;
        self.pending = None;
        self.last_receipt = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RecordingReadbackApi {
        calls: Vec<isize>,
        result: Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError>,
    }

    impl HeptaWindowsBackdropReadbackApi for RecordingReadbackApi {
        fn read_backdrop(
            &mut self,
            window: isize,
        ) -> Result<HeptaWindowsDwmBackdropValue, HeptaWindowsBackdropReadbackError> {
            self.calls.push(window);
            self.result
        }
    }

    fn root(generation: u64) -> HeptaWindowsBackendWindowIdentity {
        HeptaWindowsBackendWindowIdentity {
            hwnd: 101,
            window_index: 1,
            window_generation: generation,
        }
    }

    fn popup(generation: u64) -> HeptaWindowsTransientFrameworkWindowIdentity {
        HeptaWindowsTransientFrameworkWindowIdentity::new(2, generation, true).unwrap()
    }

    fn processed(
        request: HeptaWindowsTransientRequest,
        hwnd: isize,
        succeeded: bool,
    ) -> HeptaWindowsTransientVisualsProcessed {
        HeptaWindowsTransientVisualsProcessed::new(
            request.sequence,
            request.window.window_index,
            request.window.window_generation,
            hwnd,
            request.visuals,
            succeeded,
            true,
        )
        .unwrap()
    }

    fn host() -> HeptaWindowsTransientMaterialHost {
        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_root(root(7)).unwrap();
        host.observe_transient_window(popup(9)).unwrap();
        host
    }

    #[test]
    fn transient_window_must_be_a_distinct_popup() {
        assert_eq!(
            HeptaWindowsTransientFrameworkWindowIdentity::new(2, 9, false),
            Err(HeptaWindowsTransientHostError::PopupRequired)
        );
        let mut host = HeptaWindowsTransientMaterialHost::default();
        host.bind_root(root(7)).unwrap();
        assert_eq!(
            host.observe_transient_window(
                HeptaWindowsTransientFrameworkWindowIdentity::new(1, 7, true).unwrap(),
            ),
            Err(HeptaWindowsTransientHostError::RootWindowIdentityReuse)
        );
        host.observe_transient_window(popup(9)).unwrap();
        let request = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Acrylic),
        };
        assert_eq!(
            host.process_backend_event(processed(request, 101, true), &mut api),
            Err(HeptaWindowsTransientHostError::RootHandleReuse)
        );
        assert!(api.calls.is_empty());
    }

    #[test]
    fn acrylic_readback_is_partial_and_authority_free() {
        let mut host = host();
        let request = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Acrylic),
        };
        let receipt = host
            .process_backend_event(processed(request, 202, true), &mut api)
            .unwrap();
        assert_eq!(
            receipt.status,
            HeptaWindowsTransientAckStatus::VerifiedAcrylicWithBackdropReadback
        );
        assert!(receipt.accepted);
        assert!(receipt.acrylic_backdrop_acknowledged);
        assert!(receipt.backdrop_exact);
        assert!(receipt.remains_partial());
        assert!(receipt.grants_no_authority());
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::AcrylicAcknowledged);
        assert_eq!(api.calls, vec![202]);
    }

    #[test]
    fn focus_loss_cleanup_is_solid_and_allows_a_new_acrylic_cycle() {
        let mut host = host();
        let acrylic = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Acrylic),
        };
        host.process_backend_event(processed(acrylic, 202, true), &mut api)
            .unwrap();

        let cleanup = host.begin_focus_loss_cleanup(2).unwrap();
        assert_eq!(cleanup.visuals, WindowVisuals::default());
        api.result = Ok(HeptaWindowsDwmBackdropValue::None);
        let receipt = host
            .process_backend_event(processed(cleanup, 202, true), &mut api)
            .unwrap();
        assert_eq!(
            receipt.cleanup_reason,
            Some(HeptaWindowsTransientCleanupReason::FocusLost)
        );
        assert!(receipt.solid_cleanup_acknowledged);
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::SolidAcknowledged);

        let next = host.begin_acrylic(3).unwrap();
        assert_eq!(next.visuals.backdrop, WindowBackdrop::Acrylic);
    }

    #[test]
    fn acrylic_failure_requires_explicit_rollback_before_reuse() {
        let mut host = host();
        let acrylic = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Acrylic),
        };
        let rejected = host
            .process_backend_event(processed(acrylic, 202, false), &mut api)
            .unwrap();
        assert_eq!(
            rejected.status,
            HeptaWindowsTransientAckStatus::RejectedBackendFailure
        );
        assert!(!rejected.accepted);
        assert!(api.calls.is_empty());
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::RollbackRequired);
        assert_eq!(
            host.begin_acrylic(2),
            Err(HeptaWindowsTransientHostError::InvalidPhase)
        );

        let rollback = host.begin_rollback_cleanup(2).unwrap();
        api.result = Ok(HeptaWindowsDwmBackdropValue::None);
        let receipt = host
            .process_backend_event(processed(rollback, 202, true), &mut api)
            .unwrap();
        assert_eq!(
            receipt.cleanup_reason,
            Some(HeptaWindowsTransientCleanupReason::Rollback)
        );
        assert!(receipt.solid_cleanup_acknowledged);
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::SolidAcknowledged);
    }

    #[test]
    fn close_requires_solid_ack_and_exact_destroy_identity() {
        let mut host = host();
        let acrylic = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Acrylic),
        };
        host.process_backend_event(processed(acrylic, 202, true), &mut api)
            .unwrap();
        assert_eq!(
            host.mark_close_requested(),
            Err(HeptaWindowsTransientHostError::CleanupRequiredBeforeClose)
        );

        let cleanup = host.begin_close_cleanup(2).unwrap();
        api.result = Ok(HeptaWindowsDwmBackdropValue::None);
        host.process_backend_event(processed(cleanup, 202, true), &mut api)
            .unwrap();
        host.mark_close_requested().unwrap();

        let stale = host.process_destroyed(
            HeptaWindowsTransientNativeWindowIdentity::new(303, 2, 9).unwrap(),
        );
        assert!(!stale.accepted);
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::ClosePending);

        let closed = host.process_destroyed(
            HeptaWindowsTransientNativeWindowIdentity::new(202, 2, 9).unwrap(),
        );
        assert!(closed.accepted);
        assert!(closed.cleanup_confirmed_before_close);
        assert!(closed.closed);
        assert!(closed.grants_no_authority());
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::Closed);
    }

    #[test]
    fn readback_mismatch_requires_rollback_and_shutdown_is_terminal() {
        let mut host = host();
        let acrylic = host.begin_acrylic(1).unwrap();
        let mut api = RecordingReadbackApi {
            calls: Vec::new(),
            result: Ok(HeptaWindowsDwmBackdropValue::Mica),
        };
        let receipt = host
            .process_backend_event(processed(acrylic, 202, true), &mut api)
            .unwrap();
        assert_eq!(
            receipt.status,
            HeptaWindowsTransientAckStatus::RejectedReadbackMismatch
        );
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::RollbackRequired);
        assert_eq!(
            host.shutdown(),
            Err(HeptaWindowsTransientHostError::CleanupRequiredBeforeClose)
        );
        let rollback = host.begin_rollback_cleanup(2).unwrap();
        api.result = Ok(HeptaWindowsDwmBackdropValue::None);
        host.process_backend_event(processed(rollback, 202, true), &mut api)
            .unwrap();
        host.shutdown().unwrap();
        assert_eq!(host.phase(), HeptaWindowsTransientHostPhase::Shutdown);
        assert_eq!(
            host.bind_root(root(8)),
            Err(HeptaWindowsTransientHostError::HostShutdown)
        );
    }

    #[test]
    fn source_claim_constants_remain_fail_closed() {
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_SOURCE_WIRED);
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_POPUP_REQUIRED);
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_SEPARATE_HWND_REQUIRED);
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_FOCUS_LOSS_CLEANUP_SOURCE_WIRED);
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_CLOSE_CLEANUP_SOURCE_WIRED);
        assert!(HEPTA_WINDOWS_TRANSIENT_HOST_ROLLBACK_SOURCE_WIRED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_HOST_BOUND);
        assert!(!HEPTA_WINDOWS_TRANSIENT_RUNTIME_VALIDATED);
        assert!(!HEPTA_WINDOWS_TRANSIENT_BACKDROP_READBACK_VALIDATED);
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
