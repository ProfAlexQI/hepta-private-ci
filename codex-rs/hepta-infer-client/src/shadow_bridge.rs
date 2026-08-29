//! Typed, non-authoritative product shadow comparison boundary.
//!
//! This module never carries a raw prompt or model output. Product code provides only
//! bounded projection metadata and digests. The primary decision is returned unchanged
//! for every shadow disposition; shadow results are typed signals with no Memory, KG,
//! effect, route, or fleet write authority.

use std::fmt;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_core::TenantId;
use codex_hepta_infer_core::WorkspaceId;
use tokio::time;

use crate::RouteMode;

const BASIS_POINTS: u64 = 10_000;

pub type ShadowBridgeResult<T> = std::result::Result<T, ShadowBridgeError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowBridgeError {
    InvalidConfig,
    InvalidPrimaryDecision,
    InvalidProjection,
    InputFenceMismatch,
}

impl ShadowBridgeError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidConfig => "INF_SHADOW_BRIDGE_CONFIG_INVALID",
            Self::InvalidPrimaryDecision => "INF_SHADOW_PRIMARY_INVALID",
            Self::InvalidProjection => "INF_SHADOW_PROJECTION_INVALID",
            Self::InputFenceMismatch => "INF_SHADOW_INPUT_FENCE_MISMATCH",
        }
    }
}

impl fmt::Display for ShadowBridgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ShadowBridgeError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowExecutionError {
    CapabilityRejected,
    ProtocolRejected,
    RuntimeUnavailable,
}

impl ShadowExecutionError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::CapabilityRejected => "INF_SHADOW_CAPABILITY_REJECTED",
            Self::ProtocolRejected => "INF_SHADOW_PROTOCOL_REJECTED",
            Self::RuntimeUnavailable => "INF_SHADOW_RUNTIME_UNAVAILABLE",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowProposalKind {
    SemanticCandidate,
    ClassificationSignal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShadowComparisonDisposition {
    Matched,
    Diverged,
    BypassedDisabled,
    BypassedKillSwitch,
    BypassedSampling,
    ShadowTimedOut,
    ShadowUnavailable(ShadowExecutionError),
    RejectedFence,
    RejectedBudget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShadowBridgeConfig {
    pub enabled: bool,
    pub allowed_tuple_digest: Digest,
    pub shadow_timeout: Duration,
    pub max_projection_bytes: u64,
    pub max_primary_output_bytes: u64,
    pub max_shadow_output_bytes: u64,
    pub comparison_budget_bytes: u64,
    pub sample_rate_basis_points: u16,
}

impl ShadowBridgeConfig {
    pub fn validate(&self) -> ShadowBridgeResult<()> {
        if self.shadow_timeout.is_zero()
            || self.max_projection_bytes == 0
            || self.max_primary_output_bytes == 0
            || self.max_shadow_output_bytes == 0
            || self.comparison_budget_bytes == 0
            || self.max_shadow_output_bytes > self.comparison_budget_bytes
            || u64::from(self.sample_rate_basis_points) > BASIS_POINTS
        {
            return Err(ShadowBridgeError::InvalidConfig);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShadowInputProjection {
    pub request_id: RequestId,
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub tuple_digest: Digest,
    pub prompt_digest: Digest,
    pub prompt_byte_length: u64,
    pub projection_digest: Digest,
    pub projection_byte_length: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimaryDecision {
    pub request_id: RequestId,
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub tuple_digest: Digest,
    pub output_digest: Digest,
    pub output_byte_length: u64,
}

impl PrimaryDecision {
    pub const fn authoritative(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShadowProposal {
    pub request_id: RequestId,
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub tuple_digest: Digest,
    pub output_digest: Digest,
    pub output_byte_length: u64,
    pub kind: ShadowProposalKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShadowComparisonSignal {
    pub route_mode: RouteMode,
    pub disposition: ShadowComparisonDisposition,
    pub request_id: RequestId,
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub tuple_digest: Digest,
    pub primary_output_digest: Digest,
    pub shadow_output_digest: Option<Digest>,
    pub primary_output_authoritative: bool,
    pub shadow_output_authoritative: bool,
    pub memory_write: bool,
    pub shared_kg_write: bool,
    pub effect_write: bool,
    pub route_write: bool,
    pub fleet_write: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShadowBridgeOutcome {
    pub primary: PrimaryDecision,
    pub signal: ShadowComparisonSignal,
}

#[derive(Clone, Debug)]
pub struct ShadowKillSwitch {
    engaged: Arc<AtomicBool>,
}

impl ShadowKillSwitch {
    pub fn engaged_by_default() -> Self {
        Self {
            engaged: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn explicit_shadow_opt_in() -> Self {
        Self {
            engaged: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn engage(&self) {
        self.engaged.store(true, Ordering::SeqCst);
    }

    pub fn clear_for_explicit_shadow(&self) {
        self.engaged.store(false, Ordering::SeqCst);
    }

    pub fn is_engaged(&self) -> bool {
        self.engaged.load(Ordering::SeqCst)
    }
}

pub struct ProductShadowBridge {
    config: ShadowBridgeConfig,
    kill_switch: ShadowKillSwitch,
}

impl ProductShadowBridge {
    pub fn new(
        config: ShadowBridgeConfig,
        kill_switch: ShadowKillSwitch,
    ) -> ShadowBridgeResult<Self> {
        config.validate()?;
        Ok(Self {
            config,
            kill_switch,
        })
    }

    pub const fn route_mode(&self) -> RouteMode {
        RouteMode::ShadowCompareOnly
    }

    pub const fn config(&self) -> &ShadowBridgeConfig {
        &self.config
    }

    pub fn kill_switch(&self) -> ShadowKillSwitch {
        self.kill_switch.clone()
    }

    pub async fn compare<F, Fut>(
        &self,
        primary: PrimaryDecision,
        projection: ShadowInputProjection,
        shadow_operation: F,
    ) -> ShadowBridgeResult<ShadowBridgeOutcome>
    where
        F: FnOnce(ShadowInputProjection) -> Fut,
        Fut: Future<Output = std::result::Result<ShadowProposal, ShadowExecutionError>>,
    {
        self.validate_inputs(&primary, &projection)?;
        if !self.config.enabled {
            return Ok(self.outcome(
                primary,
                ShadowComparisonDisposition::BypassedDisabled,
                None,
            ));
        }
        if self.kill_switch.is_engaged() {
            return Ok(self.outcome(
                primary,
                ShadowComparisonDisposition::BypassedKillSwitch,
                None,
            ));
        }
        if !self.sampled(&projection) {
            return Ok(self.outcome(
                primary,
                ShadowComparisonDisposition::BypassedSampling,
                None,
            ));
        }

        match time::timeout(
            self.config.shadow_timeout,
            shadow_operation(projection.clone()),
        )
        .await
        {
            Err(_) => Ok(self.outcome(
                primary,
                ShadowComparisonDisposition::ShadowTimedOut,
                None,
            )),
            Ok(Err(error)) => Ok(self.outcome(
                primary,
                ShadowComparisonDisposition::ShadowUnavailable(error),
                None,
            )),
            Ok(Ok(proposal)) => {
                if !proposal_matches_projection(&proposal, &projection) {
                    return Ok(self.outcome(
                        primary,
                        ShadowComparisonDisposition::RejectedFence,
                        None,
                    ));
                }
                if proposal.output_byte_length == 0
                    || proposal.output_byte_length > self.config.max_shadow_output_bytes
                    || proposal.output_byte_length > self.config.comparison_budget_bytes
                {
                    return Ok(self.outcome(
                        primary,
                        ShadowComparisonDisposition::RejectedBudget,
                        None,
                    ));
                }
                let disposition = if proposal.output_digest == primary.output_digest {
                    ShadowComparisonDisposition::Matched
                } else {
                    ShadowComparisonDisposition::Diverged
                };
                Ok(self.outcome(primary, disposition, Some(proposal.output_digest)))
            }
        }
    }

    fn validate_inputs(
        &self,
        primary: &PrimaryDecision,
        projection: &ShadowInputProjection,
    ) -> ShadowBridgeResult<()> {
        if primary.output_byte_length == 0
            || primary.output_byte_length > self.config.max_primary_output_bytes
            || primary.tuple_digest != self.config.allowed_tuple_digest
        {
            return Err(ShadowBridgeError::InvalidPrimaryDecision);
        }
        if projection.prompt_byte_length == 0
            || projection.projection_byte_length == 0
            || projection.projection_byte_length > self.config.max_projection_bytes
            || projection.tuple_digest != self.config.allowed_tuple_digest
        {
            return Err(ShadowBridgeError::InvalidProjection);
        }
        if primary.request_id != projection.request_id
            || primary.tenant_id != projection.tenant_id
            || primary.workspace_id != projection.workspace_id
            || primary.tuple_digest != projection.tuple_digest
        {
            return Err(ShadowBridgeError::InputFenceMismatch);
        }
        Ok(())
    }

    fn sampled(&self, projection: &ShadowInputProjection) -> bool {
        let rate = u64::from(self.config.sample_rate_basis_points);
        if rate == 0 {
            return false;
        }
        if rate == BASIS_POINTS {
            return true;
        }
        stable_sample_hash(projection.projection_digest.as_str().as_bytes()) % BASIS_POINTS < rate
    }

    fn outcome(
        &self,
        primary: PrimaryDecision,
        disposition: ShadowComparisonDisposition,
        shadow_output_digest: Option<Digest>,
    ) -> ShadowBridgeOutcome {
        let signal = ShadowComparisonSignal {
            route_mode: RouteMode::ShadowCompareOnly,
            disposition,
            request_id: primary.request_id.clone(),
            tenant_id: primary.tenant_id.clone(),
            workspace_id: primary.workspace_id.clone(),
            tuple_digest: primary.tuple_digest.clone(),
            primary_output_digest: primary.output_digest.clone(),
            shadow_output_digest,
            primary_output_authoritative: true,
            shadow_output_authoritative: false,
            memory_write: false,
            shared_kg_write: false,
            effect_write: false,
            route_write: false,
            fleet_write: false,
        };
        ShadowBridgeOutcome { primary, signal }
    }
}

fn proposal_matches_projection(
    proposal: &ShadowProposal,
    projection: &ShadowInputProjection,
) -> bool {
    proposal.request_id == projection.request_id
        && proposal.tenant_id == projection.tenant_id
        && proposal.workspace_id == projection.workspace_id
        && proposal.tuple_digest == projection.tuple_digest
}

fn stable_sample_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use super::*;

    fn must<T, E: fmt::Display>(result: std::result::Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn primary() -> PrimaryDecision {
        PrimaryDecision {
            request_id: must(RequestId::parse("request-product-shadow")),
            tenant_id: must(TenantId::parse("tenant-a")),
            workspace_id: must(WorkspaceId::parse("workspace-a")),
            tuple_digest: digest('a'),
            output_digest: digest('b'),
            output_byte_length: 32,
        }
    }

    fn projection() -> ShadowInputProjection {
        let primary = primary();
        ShadowInputProjection {
            request_id: primary.request_id,
            tenant_id: primary.tenant_id,
            workspace_id: primary.workspace_id,
            tuple_digest: primary.tuple_digest,
            prompt_digest: digest('c'),
            prompt_byte_length: 128,
            projection_digest: digest('d'),
            projection_byte_length: 96,
        }
    }

    fn proposal(output_digest: Digest) -> ShadowProposal {
        let primary = primary();
        ShadowProposal {
            request_id: primary.request_id,
            tenant_id: primary.tenant_id,
            workspace_id: primary.workspace_id,
            tuple_digest: primary.tuple_digest,
            output_digest,
            output_byte_length: 24,
            kind: ShadowProposalKind::SemanticCandidate,
        }
    }

    fn config(enabled: bool, sample_rate_basis_points: u16) -> ShadowBridgeConfig {
        ShadowBridgeConfig {
            enabled,
            allowed_tuple_digest: digest('a'),
            shadow_timeout: Duration::from_millis(50),
            max_projection_bytes: 1024,
            max_primary_output_bytes: 1024,
            max_shadow_output_bytes: 1024,
            comparison_budget_bytes: 1024,
            sample_rate_basis_points,
        }
    }

    #[tokio::test]
    async fn disabled_and_kill_switch_paths_do_not_execute_shadow() {
        let calls = Arc::new(AtomicUsize::new(0));
        let bridge = must(ProductShadowBridge::new(
            config(false, 10_000),
            ShadowKillSwitch::explicit_shadow_opt_in(),
        ));
        let observed = calls.clone();
        let outcome = must(
            bridge
                .compare(primary(), projection(), move |_| async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            outcome.signal.disposition,
            ShadowComparisonDisposition::BypassedDisabled
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        let switch = ShadowKillSwitch::engaged_by_default();
        let bridge = must(ProductShadowBridge::new(config(true, 10_000), switch));
        let observed = calls.clone();
        let outcome = must(
            bridge
                .compare(primary(), projection(), move |_| async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            outcome.signal.disposition,
            ShadowComparisonDisposition::BypassedKillSwitch
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn match_and_divergence_are_non_authoritative_typed_signals() {
        let bridge = must(ProductShadowBridge::new(
            config(true, 10_000),
            ShadowKillSwitch::explicit_shadow_opt_in(),
        ));
        let matched = must(
            bridge
                .compare(primary(), projection(), |_| async {
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            matched.signal.disposition,
            ShadowComparisonDisposition::Matched
        );
        assert!(matched.primary.authoritative());
        assert!(matched.signal.primary_output_authoritative);
        assert!(!matched.signal.shadow_output_authoritative);
        assert!(!matched.signal.memory_write);
        assert!(!matched.signal.shared_kg_write);
        assert!(!matched.signal.effect_write);
        assert!(!matched.signal.route_write);
        assert!(!matched.signal.fleet_write);

        let diverged = must(
            bridge
                .compare(primary(), projection(), |_| async {
                    Ok(proposal(digest('e')))
                })
                .await,
        );
        assert_eq!(
            diverged.signal.disposition,
            ShadowComparisonDisposition::Diverged
        );
        assert_eq!(diverged.primary, primary());
    }

    #[tokio::test]
    async fn timeout_and_runtime_failure_are_isolated_from_primary() {
        let bridge = must(ProductShadowBridge::new(
            config(true, 10_000),
            ShadowKillSwitch::explicit_shadow_opt_in(),
        ));
        let timed_out = must(
            bridge
                .compare(primary(), projection(), |_| async {
                    time::sleep(Duration::from_millis(100)).await;
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            timed_out.signal.disposition,
            ShadowComparisonDisposition::ShadowTimedOut
        );
        assert_eq!(timed_out.primary, primary());

        let unavailable = must(
            bridge
                .compare(primary(), projection(), |_| async {
                    Err(ShadowExecutionError::RuntimeUnavailable)
                })
                .await,
        );
        assert_eq!(
            unavailable.signal.disposition,
            ShadowComparisonDisposition::ShadowUnavailable(
                ShadowExecutionError::RuntimeUnavailable
            )
        );
        assert_eq!(unavailable.primary, primary());
    }

    #[tokio::test]
    async fn proposal_fence_and_budget_violations_fail_closed_as_signals() {
        let bridge = must(ProductShadowBridge::new(
            config(true, 10_000),
            ShadowKillSwitch::explicit_shadow_opt_in(),
        ));
        let mut wrong_tenant = proposal(digest('b'));
        wrong_tenant.tenant_id = must(TenantId::parse("tenant-b"));
        let rejected = must(
            bridge
                .compare(primary(), projection(), |_| async { Ok(wrong_tenant) })
                .await,
        );
        assert_eq!(
            rejected.signal.disposition,
            ShadowComparisonDisposition::RejectedFence
        );
        assert!(rejected.signal.shadow_output_digest.is_none());

        let mut oversized = proposal(digest('b'));
        oversized.output_byte_length = 1025;
        let rejected = must(
            bridge
                .compare(primary(), projection(), |_| async { Ok(oversized) })
                .await,
        );
        assert_eq!(
            rejected.signal.disposition,
            ShadowComparisonDisposition::RejectedBudget
        );
        assert!(rejected.signal.shadow_output_digest.is_none());
    }

    #[tokio::test]
    async fn kill_switch_rollback_stops_future_shadow_calls_without_changing_primary() {
        let calls = Arc::new(AtomicUsize::new(0));
        let switch = ShadowKillSwitch::explicit_shadow_opt_in();
        let bridge = must(ProductShadowBridge::new(
            config(true, 10_000),
            switch.clone(),
        ));
        let observed = calls.clone();
        let first = must(
            bridge
                .compare(primary(), projection(), move |_| async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(first.signal.disposition, ShadowComparisonDisposition::Matched);
        switch.engage();
        let observed = calls.clone();
        let rolled_back = must(
            bridge
                .compare(primary(), projection(), move |_| async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            rolled_back.signal.disposition,
            ShadowComparisonDisposition::BypassedKillSwitch
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(rolled_back.primary, primary());
    }

    #[tokio::test]
    async fn deterministic_zero_sampling_never_executes_shadow() {
        let calls = Arc::new(AtomicUsize::new(0));
        let bridge = must(ProductShadowBridge::new(
            config(true, 0),
            ShadowKillSwitch::explicit_shadow_opt_in(),
        ));
        let observed = calls.clone();
        let outcome = must(
            bridge
                .compare(primary(), projection(), move |_| async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    Ok(proposal(digest('b')))
                })
                .await,
        );
        assert_eq!(
            outcome.signal.disposition,
            ShadowComparisonDisposition::BypassedSampling
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
}
