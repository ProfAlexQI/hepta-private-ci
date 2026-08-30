//! Canonical six-dimensional quota registry for AuthBus.
//!
//! This module owns field order, external names, projection keys and migration
//! policy. Legacy five-dimensional vectors remain compatibility projections;
//! they cannot supply or discard `request_count` without an explicit policy.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::Sha256Digest;

/// Stable schema name for the first canonical AuthBus quota registry.
pub const AUTHBUS_QUOTA_REGISTRY_SCHEMA: &str = "hepta.authbus.quota-registry.v1";
/// Stable integer version for the first canonical AuthBus quota registry.
pub const AUTHBUS_QUOTA_REGISTRY_VERSION: u32 = 1;
/// Canonical source registry required by the B2 projection binding.
pub const AUTHBUS_QUOTA_SOURCE_REGISTRY_REF: &str =
    "OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry";
/// SHA-256 of the canonical v1.3 registry bound by the active projection.
pub const AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256: &str =
    "cda83c4776d4c2b3c2851474e476e775d6ca26fa815373083aac47fdfd0c89f5";
/// Exact domain projection that owns UsageVector semantics.
pub const AUTHBUS_QUOTA_SOURCE_DOMAIN_REF: &str = "OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml#/execution_closure_v1_3/usage_vector";
/// Declared lossless transform from the canonical registry into this module.
pub const AUTHBUS_QUOTA_PROJECTION_TRANSFORM: &str =
    "generated_domain_projection:AUTHBUS.11-v1.3:usage_vector";
/// Semantic revision bound by the generated projection.
pub const AUTHBUS_QUOTA_SEMANTIC_REVISION: &str = "AUTHBUS.11-v1.3";
/// Number of dimensions in the canonical AuthBus quota vector.
pub const AUTHBUS_QUOTA_DIMENSION_COUNT: usize = 6;
/// Digest of the committed descriptor registry, using the encoding implemented
/// by [`authbus_quota_registry_digest`].
pub const AUTHBUS_QUOTA_REGISTRY_SHA256: &str =
    "dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1";

/// Stable quota dimension identifier.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum QuotaDimension {
    /// Number of admitted requests.
    RequestCount = 0,
    /// Requests per minute.
    Rpm = 1,
    /// Tokens per minute.
    Tpm = 2,
    /// Concurrent in-flight requests.
    Concurrency = 3,
    /// Provider or owner budget consumed in a day.
    DayBudget = 4,
    /// Context tokens reserved or consumed.
    Context = 5,
}

impl QuotaDimension {
    /// Canonical dimension order. This order is part of the schema digest.
    pub const ALL: [Self; AUTHBUS_QUOTA_DIMENSION_COUNT] = [
        Self::RequestCount,
        Self::Rpm,
        Self::Tpm,
        Self::Concurrency,
        Self::DayBudget,
        Self::Context,
    ];

    /// Return this dimension's stable descriptor.
    pub const fn descriptor(self) -> &'static QuotaDimensionDescriptor {
        &AUTHBUS_QUOTA_DIMENSIONS[self as usize]
    }
}

/// Unit carried by a quota dimension.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaUnit {
    /// Absolute request count.
    Requests,
    /// Requests in a declared window.
    RequestsPerWindow,
    /// Tokens in a declared window.
    TokensPerWindow,
    /// Active operations held concurrently.
    ActiveOperations,
    /// Provider-defined integer minimum units.
    ProviderDefinedIntegerUnits,
}

impl QuotaUnit {
    /// Stable unit name included in the registry digest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requests => "requests",
            Self::RequestsPerWindow => "requests_per_window",
            Self::TokensPerWindow => "tokens_per_window",
            Self::ActiveOperations => "active_operations",
            Self::ProviderDefinedIntegerUnits => "provider_defined_integer_units",
        }
    }
}

/// Lifecycle semantics for one quota dimension.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaDimensionLifecycle {
    /// Monotonic cumulative counter.
    Cumulative,
    /// Counter bound to an explicit window identity.
    Windowed,
    /// Active hold that is released rather than consumed.
    ActiveHold,
    /// Per-request bound that is not aggregate spend by default.
    PerRequestBound,
}

impl QuotaDimensionLifecycle {
    /// Stable lifecycle name included in the registry digest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cumulative => "cumulative",
            Self::Windowed => "windowed",
            Self::ActiveHold => "active_hold",
            Self::PerRequestBound => "per_request_bound",
        }
    }
}

/// Deterministic integer rounding bound by the metering contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaRounding {
    /// Estimate is rounded up before a hold is admitted.
    IntegerRoundUpBeforeHold,
    /// Terminal quantity is recorded exactly in integer minimum units.
    IntegerExactOnFinalize,
}

impl QuotaRounding {
    /// Stable rounding name included in the registry digest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IntegerRoundUpBeforeHold => "integer_round_up_before_hold",
            Self::IntegerExactOnFinalize => "integer_exact_on_finalize",
        }
    }
}

/// One canonical descriptor. Every wire, SQLite, receipt and metric name is
/// declared here rather than duplicated by individual AuthBus tranches.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuotaDimensionDescriptor {
    /// Stable dimension identifier.
    pub dimension: QuotaDimension,
    /// Zero-based canonical order.
    pub ordinal: u8,
    /// Canonical internal key.
    pub canonical_key: &'static str,
    /// Versioned wire key.
    pub wire_key: &'static str,
    /// SQLite column used for a configured limit.
    pub sqlite_limit_column: &'static str,
    /// SQLite column used for a held reservation.
    pub sqlite_reserved_column: &'static str,
    /// SQLite column used for terminal or accumulated usage.
    pub sqlite_used_column: &'static str,
    /// Key emitted into durable receipts.
    pub receipt_key: &'static str,
    /// Stable metrics suffix.
    pub metric_key: &'static str,
    /// Semantic unit.
    pub unit: QuotaUnit,
    /// Counter lifecycle.
    pub lifecycle: QuotaDimensionLifecycle,
    /// Whether a durable window identity is required.
    pub window_identity_required: bool,
    /// Rounding applied before a hold.
    pub hold_rounding: QuotaRounding,
    /// Rounding applied at terminal finalization.
    pub finalize_rounding: QuotaRounding,
    /// Whether the five-dimensional v0 projection carried this dimension.
    pub present_in_legacy_v0: bool,
}

/// The only AuthBus quota field registry.
pub const AUTHBUS_QUOTA_DIMENSIONS: [QuotaDimensionDescriptor; AUTHBUS_QUOTA_DIMENSION_COUNT] = [
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::RequestCount,
        ordinal: 0,
        canonical_key: "request_count",
        wire_key: "request_count",
        sqlite_limit_column: "limit_request_count",
        sqlite_reserved_column: "reserved_request_count",
        sqlite_used_column: "used_request_count",
        receipt_key: "request_count",
        metric_key: "quota_request_count",
        unit: QuotaUnit::Requests,
        lifecycle: QuotaDimensionLifecycle::Cumulative,
        window_identity_required: false,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: false,
    },
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::Rpm,
        ordinal: 1,
        canonical_key: "rpm",
        wire_key: "rpm",
        sqlite_limit_column: "limit_rpm",
        sqlite_reserved_column: "reserved_rpm",
        sqlite_used_column: "used_rpm",
        receipt_key: "rpm",
        metric_key: "quota_rpm",
        unit: QuotaUnit::RequestsPerWindow,
        lifecycle: QuotaDimensionLifecycle::Windowed,
        window_identity_required: true,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: true,
    },
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::Tpm,
        ordinal: 2,
        canonical_key: "tpm",
        wire_key: "tpm",
        sqlite_limit_column: "limit_tpm",
        sqlite_reserved_column: "reserved_tpm",
        sqlite_used_column: "used_tpm",
        receipt_key: "tpm",
        metric_key: "quota_tpm",
        unit: QuotaUnit::TokensPerWindow,
        lifecycle: QuotaDimensionLifecycle::Windowed,
        window_identity_required: true,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: true,
    },
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::Concurrency,
        ordinal: 3,
        canonical_key: "concurrency",
        wire_key: "concurrency",
        sqlite_limit_column: "limit_concurrency",
        sqlite_reserved_column: "reserved_concurrency",
        sqlite_used_column: "used_concurrency",
        receipt_key: "concurrency",
        metric_key: "quota_concurrency",
        unit: QuotaUnit::ActiveOperations,
        lifecycle: QuotaDimensionLifecycle::ActiveHold,
        window_identity_required: true,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: true,
    },
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::DayBudget,
        ordinal: 4,
        canonical_key: "day_budget",
        wire_key: "day_budget",
        sqlite_limit_column: "limit_day_budget",
        sqlite_reserved_column: "reserved_day_budget",
        sqlite_used_column: "used_day_budget",
        receipt_key: "day_budget",
        metric_key: "quota_day_budget",
        unit: QuotaUnit::ProviderDefinedIntegerUnits,
        lifecycle: QuotaDimensionLifecycle::Windowed,
        window_identity_required: true,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: true,
    },
    QuotaDimensionDescriptor {
        dimension: QuotaDimension::Context,
        ordinal: 5,
        canonical_key: "context",
        wire_key: "context",
        sqlite_limit_column: "limit_context",
        sqlite_reserved_column: "reserved_context",
        sqlite_used_column: "used_context",
        receipt_key: "context",
        metric_key: "quota_context",
        unit: QuotaUnit::ProviderDefinedIntegerUnits,
        lifecycle: QuotaDimensionLifecycle::PerRequestBound,
        window_identity_required: false,
        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,
        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,
        present_in_legacy_v0: true,
    },
];

/// Projection surface selected from the canonical descriptor registry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaProjection {
    /// Versioned wire field names.
    Wire,
    /// SQLite configured-limit columns.
    SqliteLimit,
    /// SQLite held-reservation columns.
    SqliteReserved,
    /// SQLite terminal/accumulated-use columns.
    SqliteUsed,
    /// Durable receipt keys.
    Receipt,
    /// Stable metric suffixes.
    Metric,
}

impl QuotaDimensionDescriptor {
    /// Return the field name for one projection surface.
    pub const fn projection_key(self, projection: QuotaProjection) -> &'static str {
        match projection {
            QuotaProjection::Wire => self.wire_key,
            QuotaProjection::SqliteLimit => self.sqlite_limit_column,
            QuotaProjection::SqliteReserved => self.sqlite_reserved_column,
            QuotaProjection::SqliteUsed => self.sqlite_used_column,
            QuotaProjection::Receipt => self.receipt_key,
            QuotaProjection::Metric => self.metric_key,
        }
    }
}

/// Registry validation error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaRegistryError {
    /// Registry length differs from the schema constant.
    InvalidDimensionCount,
    /// Descriptor ordinal or enum value differs from its array position.
    InvalidOrdinal,
    /// At least one projection contains a duplicate key.
    DuplicateProjectionKey,
    /// Source registry reference differs from the canonical B2 binding.
    SourceRegistryRefMismatch,
    /// Source registry digest differs from the canonical B2 binding.
    SourceRegistryDigestMismatch,
    /// Domain projection reference differs from the active UsageVector source.
    SourceDomainRefMismatch,
    /// Projection transform differs from the declared lossless transform.
    ProjectionTransformMismatch,
    /// Semantic revision differs from AUTHBUS.11-v1.3.
    SemanticRevisionMismatch,
    /// The committed descriptor digest is stale.
    RegistryDigestMismatch,
}

/// Validate descriptor order and key uniqueness.
pub fn validate_authbus_quota_registry() -> Result<(), QuotaRegistryError> {
    if AUTHBUS_QUOTA_DIMENSIONS.len() != AUTHBUS_QUOTA_DIMENSION_COUNT {
        return Err(QuotaRegistryError::InvalidDimensionCount);
    }

    for (index, descriptor) in AUTHBUS_QUOTA_DIMENSIONS.iter().enumerate() {
        if usize::from(descriptor.ordinal) != index || descriptor.dimension as usize != index {
            return Err(QuotaRegistryError::InvalidOrdinal);
        }
    }

    for projection in [
        QuotaProjection::Wire,
        QuotaProjection::SqliteLimit,
        QuotaProjection::SqliteReserved,
        QuotaProjection::SqliteUsed,
        QuotaProjection::Receipt,
        QuotaProjection::Metric,
    ] {
        let mut keys = BTreeSet::new();
        if !AUTHBUS_QUOTA_DIMENSIONS
            .iter()
            .all(|descriptor| keys.insert(descriptor.projection_key(projection)))
        {
            return Err(QuotaRegistryError::DuplicateProjectionKey);
        }
    }

    if authbus_quota_registry_digest().as_str() != AUTHBUS_QUOTA_REGISTRY_SHA256 {
        return Err(QuotaRegistryError::RegistryDigestMismatch);
    }

    Ok(())
}

/// Validate the canonical registry, digest, domain and transform binding.
pub fn validate_authbus_quota_source_binding(
    source_registry_ref: &str,
    source_registry_sha256: &str,
    source_domain_ref: &str,
    projection_transform: &str,
    semantic_revision: &str,
) -> Result<(), QuotaRegistryError> {
    if source_registry_ref != AUTHBUS_QUOTA_SOURCE_REGISTRY_REF {
        return Err(QuotaRegistryError::SourceRegistryRefMismatch);
    }
    if source_registry_sha256 != AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256 {
        return Err(QuotaRegistryError::SourceRegistryDigestMismatch);
    }
    if source_domain_ref != AUTHBUS_QUOTA_SOURCE_DOMAIN_REF {
        return Err(QuotaRegistryError::SourceDomainRefMismatch);
    }
    if projection_transform != AUTHBUS_QUOTA_PROJECTION_TRANSFORM {
        return Err(QuotaRegistryError::ProjectionTransformMismatch);
    }
    if semantic_revision != AUTHBUS_QUOTA_SEMANTIC_REVISION {
        return Err(QuotaRegistryError::SemanticRevisionMismatch);
    }
    Ok(())
}

/// Compute the digest that binds source identity, field order, units, lifecycle,
/// rounding, projection names and legacy presence.
pub fn authbus_quota_registry_digest() -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, AUTHBUS_QUOTA_REGISTRY_SCHEMA);
    bytes.extend_from_slice(&AUTHBUS_QUOTA_REGISTRY_VERSION.to_be_bytes());
    push_text(&mut bytes, AUTHBUS_QUOTA_SOURCE_REGISTRY_REF);
    push_text(&mut bytes, AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256);
    push_text(&mut bytes, AUTHBUS_QUOTA_SOURCE_DOMAIN_REF);
    push_text(&mut bytes, AUTHBUS_QUOTA_PROJECTION_TRANSFORM);
    push_text(&mut bytes, AUTHBUS_QUOTA_SEMANTIC_REVISION);
    for descriptor in AUTHBUS_QUOTA_DIMENSIONS {
        bytes.push(descriptor.ordinal);
        push_text(&mut bytes, descriptor.canonical_key);
        push_text(&mut bytes, descriptor.wire_key);
        push_text(&mut bytes, descriptor.sqlite_limit_column);
        push_text(&mut bytes, descriptor.sqlite_reserved_column);
        push_text(&mut bytes, descriptor.sqlite_used_column);
        push_text(&mut bytes, descriptor.receipt_key);
        push_text(&mut bytes, descriptor.metric_key);
        push_text(&mut bytes, descriptor.unit.as_str());
        push_text(&mut bytes, descriptor.lifecycle.as_str());
        bytes.push(u8::from(descriptor.window_identity_required));
        push_text(&mut bytes, descriptor.hold_rounding.as_str());
        push_text(&mut bytes, descriptor.finalize_rounding.as_str());
        bytes.push(u8::from(descriptor.present_in_legacy_v0));
    }
    Sha256Digest::for_bytes(&bytes)
}

/// Explicit policy required when a legacy vector has no request-count field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LegacyRequestCountPolicy {
    /// Reject the vector because request count is unknown.
    RejectMissing,
    /// Interpret one legacy reservation as exactly one admitted request.
    AssumeOnePerPermit,
}

/// Error returned by a compatibility projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaProjectionError {
    /// A legacy vector omitted request count and no assumption was authorized.
    MissingRequestCount,
    /// A six-dimensional vector cannot be represented by the legacy shape.
    LossyLegacyDowngrade {
        /// Request count that would be discarded.
        request_count: u64,
    },
    /// A dimension is explicitly unknown and cannot be admitted.
    UnknownDimension {
        /// Dimension that is unknown.
        dimension: QuotaDimension,
    },
    /// A declared known-only projection omitted a dimension.
    NotDeclaredDimension {
        /// Dimension that was not declared.
        dimension: QuotaDimension,
    },
    /// Two vectors disagree on whether a dimension is declared.
    DeclarationMismatch {
        /// Dimension whose declaration state differs.
        dimension: QuotaDimension,
    },
    /// Component-wise arithmetic overflowed.
    ArithmeticOverflow {
        /// Dimension that overflowed.
        dimension: QuotaDimension,
    },
    /// No declared dimension carries a non-zero quantity.
    EmptyUsageVector,
    /// A scalar max-uses projection would discard another dimension.
    ScalarCompatibilityViolation,
}

/// Explicit state of one UsageVector quantity. Absence is not zero and an
/// unknown observation is not silently admitted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", content = "value", rename_all = "snake_case")]
pub enum QuotaQuantity {
    /// The provider or contract did not declare this dimension.
    #[default]
    NotDeclared,
    /// The dimension exists but its value is not known.
    ExplicitUnknown,
    /// A known integer minimum-unit quantity.
    Known(u64),
}

impl QuotaQuantity {
    fn declaration_matches(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::NotDeclared, Self::NotDeclared)
                | (Self::ExplicitUnknown, Self::ExplicitUnknown)
                | (Self::Known(_), Self::Known(_))
        )
    }

    fn checked_add(
        self,
        other: Self,
        dimension: QuotaDimension,
    ) -> Result<Self, QuotaProjectionError> {
        match (self, other) {
            (Self::NotDeclared, Self::NotDeclared) => Ok(Self::NotDeclared),
            (Self::Known(left), Self::Known(right)) => left
                .checked_add(right)
                .map(Self::Known)
                .ok_or(QuotaProjectionError::ArithmeticOverflow { dimension }),
            (Self::ExplicitUnknown, _) | (_, Self::ExplicitUnknown) => {
                Err(QuotaProjectionError::UnknownDimension { dimension })
            }
            _ => Err(QuotaProjectionError::DeclarationMismatch { dimension }),
        }
    }

    const fn is_zero_or_not_declared(self) -> bool {
        matches!(self, Self::NotDeclared | Self::Known(0))
    }
}

/// Whether a UsageVector may be advertised without overstating its authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsageVectorMarketability {
    /// All six dimensions are explicitly known.
    FullVector,
    /// Only request_count is authoritative; no broader quota claim is allowed.
    RequestCountOnly,
    /// A lossless partial vector is present but cannot be advertised as full.
    PartialVector,
    /// At least one declared dimension is unknown.
    NotMarketableUnknown,
}

/// Canonical six-dimensional wire value. Every component explicitly preserves
/// known, unknown and not-declared state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageVector {
    /// Number of admitted requests.
    pub request_count: QuotaQuantity,
    /// Requests in the bound window.
    pub rpm: QuotaQuantity,
    /// Tokens in the bound window.
    pub tpm: QuotaQuantity,
    /// Active operations.
    pub concurrency: QuotaQuantity,
    /// Provider-defined daily budget units.
    pub day_budget: QuotaQuantity,
    /// Provider-defined per-request context units.
    pub context: QuotaQuantity,
}

impl UsageVector {
    /// Construct a fully known six-dimensional vector.
    pub const fn known(
        request_count: u64,
        rpm: u64,
        tpm: u64,
        concurrency: u64,
        day_budget: u64,
        context: u64,
    ) -> Self {
        Self {
            request_count: QuotaQuantity::Known(request_count),
            rpm: QuotaQuantity::Known(rpm),
            tpm: QuotaQuantity::Known(tpm),
            concurrency: QuotaQuantity::Known(concurrency),
            day_budget: QuotaQuantity::Known(day_budget),
            context: QuotaQuantity::Known(context),
        }
    }

    /// Decode a legacy scalar max_uses as request_count only.
    pub const fn request_count_only(max_uses: u64) -> Self {
        Self {
            request_count: QuotaQuantity::Known(max_uses),
            rpm: QuotaQuantity::NotDeclared,
            tpm: QuotaQuantity::NotDeclared,
            concurrency: QuotaQuantity::NotDeclared,
            day_budget: QuotaQuantity::NotDeclared,
            context: QuotaQuantity::NotDeclared,
        }
    }

    /// Return a component in canonical descriptor order.
    pub const fn quantity(self, dimension: QuotaDimension) -> QuotaQuantity {
        match dimension {
            QuotaDimension::RequestCount => self.request_count,
            QuotaDimension::Rpm => self.rpm,
            QuotaDimension::Tpm => self.tpm,
            QuotaDimension::Concurrency => self.concurrency,
            QuotaDimension::DayBudget => self.day_budget,
            QuotaDimension::Context => self.context,
        }
    }

    fn components(self) -> [QuotaQuantity; AUTHBUS_QUOTA_DIMENSION_COUNT] {
        [
            self.request_count,
            self.rpm,
            self.tpm,
            self.concurrency,
            self.day_budget,
            self.context,
        ]
    }

    fn from_components(values: [QuotaQuantity; AUTHBUS_QUOTA_DIMENSION_COUNT]) -> Self {
        Self {
            request_count: values[0],
            rpm: values[1],
            tpm: values[2],
            concurrency: values[3],
            day_budget: values[4],
            context: values[5],
        }
    }

    /// Reject explicit unknown quantities while retaining not-declared state.
    pub fn validate_declared_shape(self) -> Result<(), QuotaProjectionError> {
        for dimension in QuotaDimension::ALL {
            if matches!(self.quantity(dimension), QuotaQuantity::ExplicitUnknown) {
                return Err(QuotaProjectionError::UnknownDimension { dimension });
            }
        }
        Ok(())
    }

    /// Admission additionally requires at least one known non-zero quantity.
    pub fn validate_for_admission(self) -> Result<(), QuotaProjectionError> {
        self.validate_declared_shape()?;
        if !self
            .components()
            .into_iter()
            .any(|value| matches!(value, QuotaQuantity::Known(quantity) if quantity > 0))
        {
            return Err(QuotaProjectionError::EmptyUsageVector);
        }
        Ok(())
    }

    /// Return true when both vectors declare exactly the same dimensions.
    pub fn declarations_match(self, other: Self) -> bool {
        self.components()
            .into_iter()
            .zip(other.components())
            .all(|(left, right)| left.declaration_matches(right))
    }

    /// Checked component-wise addition with fail-closed declaration semantics.
    pub fn checked_add(self, other: Self) -> Result<Self, QuotaProjectionError> {
        let mut values = [QuotaQuantity::NotDeclared; AUTHBUS_QUOTA_DIMENSION_COUNT];
        for (index, dimension) in QuotaDimension::ALL.into_iter().enumerate() {
            values[index] = self
                .quantity(dimension)
                .checked_add(other.quantity(dimension), dimension)?;
        }
        Ok(Self::from_components(values))
    }

    /// Classify how narrowly this vector may be advertised.
    pub fn marketability(self) -> UsageVectorMarketability {
        if self
            .components()
            .into_iter()
            .any(|value| matches!(value, QuotaQuantity::ExplicitUnknown))
        {
            return UsageVectorMarketability::NotMarketableUnknown;
        }
        if matches!(self.request_count, QuotaQuantity::Known(value) if value > 0)
            && self.rpm.is_zero_or_not_declared()
            && self.tpm.is_zero_or_not_declared()
            && self.concurrency.is_zero_or_not_declared()
            && self.day_budget.is_zero_or_not_declared()
            && self.context.is_zero_or_not_declared()
        {
            return UsageVectorMarketability::RequestCountOnly;
        }
        if self
            .components()
            .into_iter()
            .all(|value| matches!(value, QuotaQuantity::Known(_)))
        {
            UsageVectorMarketability::FullVector
        } else {
            UsageVectorMarketability::PartialVector
        }
    }

    /// Encode back to max_uses only when no other dimension is present or non-zero.
    pub fn try_to_legacy_max_uses(self) -> Result<u64, QuotaProjectionError> {
        let QuotaQuantity::Known(request_count) = self.request_count else {
            return Err(QuotaProjectionError::ScalarCompatibilityViolation);
        };
        if self.rpm.is_zero_or_not_declared()
            && self.tpm.is_zero_or_not_declared()
            && self.concurrency.is_zero_or_not_declared()
            && self.day_budget.is_zero_or_not_declared()
            && self.context.is_zero_or_not_declared()
        {
            Ok(request_count)
        } else {
            Err(QuotaProjectionError::ScalarCompatibilityViolation)
        }
    }
}

/// Explicit five-dimensional compatibility value. It is not schema authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LegacyQuotaVectorV0 {
    /// Requests per minute.
    pub rpm: u64,
    /// Tokens per minute.
    pub tpm: u64,
    /// Concurrent in-flight requests.
    pub concurrency: u64,
    /// Provider or owner daily budget.
    pub day_budget: u64,
    /// Context token count.
    pub context: u64,
}

/// Canonical six-dimensional quota value.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanonicalQuotaVector {
    /// Number of admitted requests.
    pub request_count: u64,
    /// Requests per minute.
    pub rpm: u64,
    /// Tokens per minute.
    pub tpm: u64,
    /// Concurrent in-flight requests.
    pub concurrency: u64,
    /// Provider or owner daily budget.
    pub day_budget: u64,
    /// Context token count.
    pub context: u64,
}

impl CanonicalQuotaVector {
    /// Construct a complete six-dimensional vector.
    pub const fn new(
        request_count: u64,
        rpm: u64,
        tpm: u64,
        concurrency: u64,
        day_budget: u64,
        context: u64,
    ) -> Self {
        Self {
            request_count,
            rpm,
            tpm,
            concurrency,
            day_budget,
            context,
        }
    }

    /// Upgrade a legacy five-dimensional vector under an explicit policy.
    pub fn from_legacy(
        value: LegacyQuotaVectorV0,
        policy: LegacyRequestCountPolicy,
    ) -> Result<Self, QuotaProjectionError> {
        let request_count = match policy {
            LegacyRequestCountPolicy::RejectMissing => {
                return Err(QuotaProjectionError::MissingRequestCount);
            }
            LegacyRequestCountPolicy::AssumeOnePerPermit => 1,
        };
        Ok(Self::new(
            request_count,
            value.rpm,
            value.tpm,
            value.concurrency,
            value.day_budget,
            value.context,
        ))
    }

    /// Upgrade legacy dimensions under an explicit request-count policy.
    pub fn from_legacy_dimensions(
        rpm: u64,
        tpm: u64,
        concurrency: u64,
        day_budget: u64,
        context: u64,
        policy: LegacyRequestCountPolicy,
    ) -> Result<Self, QuotaProjectionError> {
        Self::from_legacy(
            LegacyQuotaVectorV0 {
                rpm,
                tpm,
                concurrency,
                day_budget,
                context,
            },
            policy,
        )
    }

    /// Downgrade only when request count is exactly one and no information is
    /// lost.
    pub fn try_to_legacy(self) -> Result<LegacyQuotaVectorV0, QuotaProjectionError> {
        if self.request_count != 1 {
            return Err(QuotaProjectionError::LossyLegacyDowngrade {
                request_count: self.request_count,
            });
        }
        Ok(LegacyQuotaVectorV0 {
            rpm: self.rpm,
            tpm: self.tpm,
            concurrency: self.concurrency,
            day_budget: self.day_budget,
            context: self.context,
        })
    }

    /// Return a value by stable dimension.
    pub const fn value(self, dimension: QuotaDimension) -> u64 {
        match dimension {
            QuotaDimension::RequestCount => self.request_count,
            QuotaDimension::Rpm => self.rpm,
            QuotaDimension::Tpm => self.tpm,
            QuotaDimension::Concurrency => self.concurrency,
            QuotaDimension::DayBudget => self.day_budget,
            QuotaDimension::Context => self.context,
        }
    }

    /// Return values in canonical registry order.
    pub const fn values(self) -> [u64; AUTHBUS_QUOTA_DIMENSION_COUNT] {
        [
            self.request_count,
            self.rpm,
            self.tpm,
            self.concurrency,
            self.day_budget,
            self.context,
        ]
    }

    /// Whether every dimension is zero.
    pub const fn is_zero(self) -> bool {
        self.request_count == 0
            && self.rpm == 0
            && self.tpm == 0
            && self.concurrency == 0
            && self.day_budget == 0
            && self.context == 0
    }

    /// Checked component-wise addition.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            request_count: self.request_count.checked_add(other.request_count)?,
            rpm: self.rpm.checked_add(other.rpm)?,
            tpm: self.tpm.checked_add(other.tpm)?,
            concurrency: self.concurrency.checked_add(other.concurrency)?,
            day_budget: self.day_budget.checked_add(other.day_budget)?,
            context: self.context.checked_add(other.context)?,
        })
    }

    /// Checked component-wise subtraction.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self {
            request_count: self.request_count.checked_sub(other.request_count)?,
            rpm: self.rpm.checked_sub(other.rpm)?,
            tpm: self.tpm.checked_sub(other.tpm)?,
            concurrency: self.concurrency.checked_sub(other.concurrency)?,
            day_budget: self.day_budget.checked_sub(other.day_budget)?,
            context: self.context.checked_sub(other.context)?,
        })
    }

    /// Whether every component fits within the supplied vector.
    pub const fn fits_within(self, limit: Self) -> bool {
        self.request_count <= limit.request_count
            && self.rpm <= limit.rpm
            && self.tpm <= limit.tpm
            && self.concurrency <= limit.concurrency
            && self.day_budget <= limit.day_budget
            && self.context <= limit.context
    }

    /// Convert an in-flight reservation to terminal usage.
    pub const fn terminal_usage(self) -> Self {
        Self {
            concurrency: 0,
            ..self
        }
    }

    /// Project values through one registry-owned external naming surface.
    pub fn project(self, projection: QuotaProjection) -> BTreeMap<&'static str, u64> {
        AUTHBUS_QUOTA_DIMENSIONS
            .iter()
            .map(|descriptor| {
                (
                    descriptor.projection_key(projection),
                    self.value(descriptor.dimension),
                )
            })
            .collect()
    }

    /// Digest registry identity and all values in canonical order.
    pub fn digest(self) -> Sha256Digest {
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.quota-vector.v1");
        push_text(&mut bytes, AUTHBUS_QUOTA_REGISTRY_SHA256);
        for value in self.values() {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        Sha256Digest::for_bytes(&bytes)
    }
}

/// Canonical six-dimensional quota limits. `None` is unknown and always
/// denies admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CanonicalQuotaLimits {
    /// Request-count limit.
    pub request_count: Option<u64>,
    /// Requests-per-minute limit.
    pub rpm: Option<u64>,
    /// Tokens-per-minute limit.
    pub tpm: Option<u64>,
    /// Concurrency limit.
    pub concurrency: Option<u64>,
    /// Daily budget limit.
    pub day_budget: Option<u64>,
    /// Context-token limit.
    pub context: Option<u64>,
}

impl CanonicalQuotaLimits {
    /// Construct fully known limits from a vector.
    pub const fn known(value: CanonicalQuotaVector) -> Self {
        Self {
            request_count: Some(value.request_count),
            rpm: Some(value.rpm),
            tpm: Some(value.tpm),
            concurrency: Some(value.concurrency),
            day_budget: Some(value.day_budget),
            context: Some(value.context),
        }
    }

    /// Construct a compatibility view with unknown request count.
    pub const fn unknown_request_count(value: CanonicalQuotaVector) -> Self {
        Self {
            request_count: None,
            rpm: Some(value.rpm),
            tpm: Some(value.tpm),
            concurrency: Some(value.concurrency),
            day_budget: Some(value.day_budget),
            context: Some(value.context),
        }
    }

    /// Whether every canonical dimension is bounded.
    pub const fn is_fully_known(self) -> bool {
        self.request_count.is_some()
            && self.rpm.is_some()
            && self.tpm.is_some()
            && self.concurrency.is_some()
            && self.day_budget.is_some()
            && self.context.is_some()
    }

    /// Fail-closed admission check over used, held and requested values.
    pub fn can_hold(
        self,
        used: CanonicalQuotaVector,
        held: CanonicalQuotaVector,
        requested: CanonicalQuotaVector,
    ) -> bool {
        fn dimension(limit: Option<u64>, used: u64, held: u64, requested: u64) -> bool {
            let Some(limit) = limit else {
                return false;
            };
            used.checked_add(held)
                .and_then(|committed| committed.checked_add(requested))
                .is_some_and(|total| total <= limit)
        }

        dimension(
            self.request_count,
            used.request_count,
            held.request_count,
            requested.request_count,
        ) && dimension(self.rpm, used.rpm, held.rpm, requested.rpm)
            && dimension(self.tpm, used.tpm, held.tpm, requested.tpm)
            && dimension(
                self.concurrency,
                used.concurrency,
                held.concurrency,
                requested.concurrency,
            )
            && dimension(
                self.day_budget,
                used.day_budget,
                held.day_budget,
                requested.day_budget,
            )
            && dimension(self.context, used.context, held.context, requested.context)
    }

    /// Project optional limits through one registry-owned naming surface.
    pub fn project(self, projection: QuotaProjection) -> BTreeMap<&'static str, Option<u64>> {
        AUTHBUS_QUOTA_DIMENSIONS
            .iter()
            .map(|descriptor| {
                let value = match descriptor.dimension {
                    QuotaDimension::RequestCount => self.request_count,
                    QuotaDimension::Rpm => self.rpm,
                    QuotaDimension::Tpm => self.tpm,
                    QuotaDimension::Concurrency => self.concurrency,
                    QuotaDimension::DayBudget => self.day_budget,
                    QuotaDimension::Context => self.context,
                };
                (descriptor.projection_key(projection), value)
            })
            .collect()
    }
}

/// Evidence emitted when a legacy vector is upgraded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotaMigrationReceipt {
    /// Legacy schema name.
    pub source_schema: &'static str,
    /// Canonical target schema name.
    pub target_schema: &'static str,
    /// Whether request count was supplied by an explicit assumption.
    pub request_count_assumed: bool,
    /// Digest of the canonical descriptor registry.
    pub target_registry_sha256: Sha256Digest,
}

/// Upgrade a legacy value and return a digest-bound migration receipt.
pub fn migrate_legacy_quota(
    value: LegacyQuotaVectorV0,
    policy: LegacyRequestCountPolicy,
) -> Result<(CanonicalQuotaVector, QuotaMigrationReceipt), QuotaProjectionError> {
    let canonical = CanonicalQuotaVector::from_legacy(value, policy)?;
    Ok((
        canonical,
        QuotaMigrationReceipt {
            source_schema: "hepta.authbus.quota-vector.v0",
            target_schema: AUTHBUS_QUOTA_REGISTRY_SCHEMA,
            request_count_assumed: matches!(policy, LegacyRequestCountPolicy::AssumeOnePerPermit),
            target_registry_sha256: authbus_quota_registry_digest(),
        },
    ))
}

fn push_text(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

impl From<CanonicalQuotaVector> for UsageVector {
    fn from(value: CanonicalQuotaVector) -> Self {
        Self::known(
            value.request_count,
            value.rpm,
            value.tpm,
            value.concurrency,
            value.day_budget,
            value.context,
        )
    }
}

impl TryFrom<UsageVector> for CanonicalQuotaVector {
    type Error = QuotaProjectionError;

    fn try_from(value: UsageVector) -> Result<Self, Self::Error> {
        fn known(
            value: QuotaQuantity,
            dimension: QuotaDimension,
        ) -> Result<u64, QuotaProjectionError> {
            match value {
                QuotaQuantity::Known(value) => Ok(value),
                QuotaQuantity::ExplicitUnknown => {
                    Err(QuotaProjectionError::UnknownDimension { dimension })
                }
                QuotaQuantity::NotDeclared => {
                    Err(QuotaProjectionError::NotDeclaredDimension { dimension })
                }
            }
        }

        Ok(Self::new(
            known(value.request_count, QuotaDimension::RequestCount)?,
            known(value.rpm, QuotaDimension::Rpm)?,
            known(value.tpm, QuotaDimension::Tpm)?,
            known(value.concurrency, QuotaDimension::Concurrency)?,
            known(value.day_budget, QuotaDimension::DayBudget)?,
            known(value.context, QuotaDimension::Context)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_order_projection_names_and_digest_are_stable() {
        validate_authbus_quota_registry().expect("valid registry");
        assert_eq!(
            authbus_quota_registry_digest().as_str(),
            AUTHBUS_QUOTA_REGISTRY_SHA256
        );
        assert_eq!(
            QuotaDimension::ALL.map(|dimension| dimension.descriptor().canonical_key),
            [
                "request_count",
                "rpm",
                "tpm",
                "concurrency",
                "day_budget",
                "context",
            ]
        );
    }

    #[test]
    fn every_projection_is_registry_derived() {
        let value = CanonicalQuotaVector::new(1, 2, 3, 4, 5, 6);
        assert_eq!(
            value.project(QuotaProjection::Wire),
            BTreeMap::from([
                ("concurrency", 4),
                ("context", 6),
                ("day_budget", 5),
                ("request_count", 1),
                ("rpm", 2),
                ("tpm", 3),
            ])
        );
        assert_eq!(
            value.project(QuotaProjection::SqliteReserved),
            BTreeMap::from([
                ("reserved_concurrency", 4),
                ("reserved_context", 6),
                ("reserved_day_budget", 5),
                ("reserved_request_count", 1),
                ("reserved_rpm", 2),
                ("reserved_tpm", 3),
            ])
        );
        assert_eq!(
            value.project(QuotaProjection::Receipt),
            value.project(QuotaProjection::Wire)
        );
        assert_eq!(
            value.project(QuotaProjection::Metric),
            BTreeMap::from([
                ("quota_concurrency", 4),
                ("quota_context", 6),
                ("quota_day_budget", 5),
                ("quota_request_count", 1),
                ("quota_rpm", 2),
                ("quota_tpm", 3),
            ])
        );
    }

    #[test]
    fn legacy_upgrade_and_downgrade_are_never_implicit() {
        let legacy = LegacyQuotaVectorV0 {
            rpm: 2,
            tpm: 300,
            concurrency: 1,
            day_budget: 400,
            context: 512,
        };
        assert_eq!(
            CanonicalQuotaVector::from_legacy(legacy, LegacyRequestCountPolicy::RejectMissing),
            Err(QuotaProjectionError::MissingRequestCount)
        );
        let canonical =
            CanonicalQuotaVector::from_legacy(legacy, LegacyRequestCountPolicy::AssumeOnePerPermit)
                .expect("explicit migration");
        assert_eq!(canonical.request_count, 1);
        assert_eq!(canonical.try_to_legacy(), Ok(legacy));
        assert_eq!(
            CanonicalQuotaVector::new(2, 2, 300, 1, 400, 512).try_to_legacy(),
            Err(QuotaProjectionError::LossyLegacyDowngrade { request_count: 2 })
        );
    }

    #[test]
    fn unknown_limit_and_overflow_fail_closed() {
        let capacity = CanonicalQuotaVector::new(10, 10, 100, 2, 1_000, 2_000);
        let requested = CanonicalQuotaVector::new(1, 1, 20, 1, 100, 200);
        assert!(CanonicalQuotaLimits::known(capacity).can_hold(
            CanonicalQuotaVector::default(),
            CanonicalQuotaVector::default(),
            requested
        ));
        assert!(
            !CanonicalQuotaLimits::unknown_request_count(capacity).can_hold(
                CanonicalQuotaVector::default(),
                CanonicalQuotaVector::default(),
                requested
            )
        );
        assert!(
            CanonicalQuotaVector::new(u64::MAX, 0, 0, 0, 0, 0)
                .checked_add(CanonicalQuotaVector::new(1, 0, 0, 0, 0, 0))
                .is_none()
        );
    }
}
