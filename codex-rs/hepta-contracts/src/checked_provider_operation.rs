//! Final-payload, revocation-checked provider operation boundary.
//!
//! The lower-level provider coordinator remains private to this crate. Product
//! callers can only obtain this wrapper, which owns distinct provider-dispatch
//! and external-effect capabilities. Every physical dispatch builds the final
//! wire payload first, verifies both capabilities with the B0 trusted clock and
//! durable claim store, persists both witnesses as one caller-owned record, and
//! only then crosses the raw adapter boundary.
//!
//! A claim or witness failure is fail-closed: no provider call is attempted and
//! no automatic retry is permitted. A timeout or lost acknowledgement after the
//! adapter is crossed remains `Indeterminate` and recovery is lookup-only.

use crate::AuthorityError;
use crate::Authorized;
use crate::ExternalEffectCapability;
use crate::OperationPhase;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseFinalCheck;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::ProviderDispatchCapability;
use crate::ProviderEffectAdapter;
use crate::ProviderEffectCoordinator;
use crate::ProviderEffectIntent;
use crate::ProviderEffectState;
use crate::RevocationRevision;
use crate::RuntimeAuthorityContext;
use crate::Sha256Digest;
use crate::VerifiedUseError;
use crate::VerifiedUseWitness;
use crate::provider_operation;
use crate::verified_use::PhysicalUseClaimReceipt;
use crate::verified_use::PhysicalUseClaimRequest;
use crate::verified_use::PhysicalUseClaimStore;
use crate::verified_use::PhysicalUseClaimStoreError;
use crate::verified_use::TrustedPhysicalClock;
use crate::verify_capability_use;
use crate::verify_physical_capability_use;

pub const B1_PROVIDER_BOUNDARY_RUNTIME_REGISTERED: bool = false;
pub const B1_PROVIDER_BOUNDARY_PRODUCTION_CALLER: bool = false;
pub const B1_PROVIDER_BOUNDARY_PRODUCTION_WRITER: bool = false;
pub const B1_PROVIDER_BOUNDARY_MODEL_INVOCATION: bool = false;
pub const B1_PROVIDER_BOUNDARY_PROVIDER_DISPATCH: bool = false;
pub const B1_PROVIDER_BOUNDARY_EXTERNAL_EFFECT: bool = false;
pub const B1_PROVIDER_BOUNDARY_OPERATOR_ACCEPTANCE: bool = false;
pub const B1_PROVIDER_BOUNDARY_PROMOTION: bool = false;
pub const B1_PROVIDER_BOUNDARY_RELEASE: bool = false;

/// Checked provider boundary with separate provider-dispatch and external-effect
/// authority. The verifier is retained so every dispatch and reconciliation
/// revalidates current authority instead of trusting construction-time state.
pub struct ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    inner: provider_operation::ProviderOperationCoordinator<A>,
    provider_dispatch: Authorized<ProviderDispatchCapability>,
    external_effect: Authorized<ExternalEffectCapability>,
    provider_runtime_authority: RuntimeAuthorityContext,
    effect_runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

include!("checked_provider_operation_parts/impl_construction.rs");
include!("checked_provider_operation_parts/impl_dispatch.rs");
include!("checked_provider_operation_parts/impl_reconcile.rs");
include!("checked_provider_operation_parts/helpers.rs");
include!("checked_provider_operation_parts/tests.rs");
