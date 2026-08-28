#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OLD_REGISTRY_DIGEST = "f6702be58c499d853d273f3174a2556481a3f5b4284cd9cd0b0a247160d7ac08"
SOURCE_REGISTRY_REF = "OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry"
SOURCE_REGISTRY_DIGEST = "cda83c4776d4c2b3c2851474e476e775d6ca26fa815373083aac47fdfd0c89f5"
SOURCE_DOMAIN_REF = (
    "OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml"
    "#/execution_closure_v1_3/usage_vector"
)
PROJECTION_TRANSFORM = "generated_domain_projection:AUTHBUS.11-v1.3:usage_vector"
SEMANTIC_REVISION = "AUTHBUS.11-v1.3"


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def write(path: str, content: str) -> None:
    target = ROOT / path
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8")


def replace_once(source: str, before: str, after: str, label: str) -> str:
    count = source.count(before)
    if count != 1:
        raise SystemExit(f"{label}: expected one replacement anchor, found {count}")
    return source.replace(before, after, 1)


def sub_once(source: str, pattern: str, replacement: str, label: str) -> str:
    updated, count = re.subn(pattern, replacement, source, count=1, flags=re.DOTALL)
    if count != 1:
        raise SystemExit(f"{label}: expected one regex anchor, found {count}")
    return updated


def push_text(buffer: bytearray, value: str) -> None:
    encoded = value.encode("utf-8")
    buffer.extend(len(encoded).to_bytes(8, "big"))
    buffer.extend(encoded)


descriptors = (
    (0, "request_count", "request_count", "limit_request_count", "reserved_request_count", "used_request_count", "request_count", "quota_request_count", "requests", "cumulative", False, "integer_round_up_before_hold", "integer_exact_on_finalize", False),
    (1, "rpm", "rpm", "limit_rpm", "reserved_rpm", "used_rpm", "rpm", "quota_rpm", "requests_per_window", "windowed", True, "integer_round_up_before_hold", "integer_exact_on_finalize", True),
    (2, "tpm", "tpm", "limit_tpm", "reserved_tpm", "used_tpm", "tpm", "quota_tpm", "tokens_per_window", "windowed", True, "integer_round_up_before_hold", "integer_exact_on_finalize", True),
    (3, "concurrency", "concurrency", "limit_concurrency", "reserved_concurrency", "used_concurrency", "concurrency", "quota_concurrency", "active_operations", "active_hold", True, "integer_round_up_before_hold", "integer_exact_on_finalize", True),
    (4, "day_budget", "day_budget", "limit_day_budget", "reserved_day_budget", "used_day_budget", "day_budget", "quota_day_budget", "provider_defined_integer_units", "windowed", True, "integer_round_up_before_hold", "integer_exact_on_finalize", True),
    (5, "context", "context", "limit_context", "reserved_context", "used_context", "context", "quota_context", "provider_defined_integer_units", "per_request_bound", False, "integer_round_up_before_hold", "integer_exact_on_finalize", True),
)

digest_bytes = bytearray()
push_text(digest_bytes, "hepta.authbus.quota-registry.v1")
digest_bytes.extend((1).to_bytes(4, "big"))
for value in (SOURCE_REGISTRY_REF, SOURCE_REGISTRY_DIGEST, SOURCE_DOMAIN_REF, PROJECTION_TRANSFORM, SEMANTIC_REVISION):
    push_text(digest_bytes, value)
for descriptor in descriptors:
    ordinal, *values = descriptor
    digest_bytes.append(ordinal)
    for value in values[:9]:
        push_text(digest_bytes, value)
    digest_bytes.append(1 if values[9] else 0)
    push_text(digest_bytes, values[10])
    push_text(digest_bytes, values[11])
    digest_bytes.append(1 if values[12] else 0)
NEW_REGISTRY_DIGEST = hashlib.sha256(digest_bytes).hexdigest()

registry_path = "codex-rs/hepta-contracts/src/quota_registry.rs"
registry = read(registry_path)
registry = replace_once(
    registry,
    'pub const AUTHBUS_QUOTA_REGISTRY_VERSION: u32 = 1;\n',
    'pub const AUTHBUS_QUOTA_REGISTRY_VERSION: u32 = 1;\n'
    '/// Canonical source registry required by the B2 projection binding.\n'
    f'pub const AUTHBUS_QUOTA_SOURCE_REGISTRY_REF: &str = "{SOURCE_REGISTRY_REF}";\n'
    '/// SHA-256 of the canonical v1.3 registry bound by the active projection.\n'
    f'pub const AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256: &str = "{SOURCE_REGISTRY_DIGEST}";\n'
    '/// Exact domain projection that owns UsageVector semantics.\n'
    f'pub const AUTHBUS_QUOTA_SOURCE_DOMAIN_REF: &str = "{SOURCE_DOMAIN_REF}";\n'
    '/// Declared lossless transform from the canonical registry into this module.\n'
    f'pub const AUTHBUS_QUOTA_PROJECTION_TRANSFORM: &str = "{PROJECTION_TRANSFORM}";\n'
    '/// Semantic revision bound by the generated projection.\n'
    f'pub const AUTHBUS_QUOTA_SEMANTIC_REVISION: &str = "{SEMANTIC_REVISION}";\n',
    "quota source metadata",
)
registry = registry.replace(OLD_REGISTRY_DIGEST, NEW_REGISTRY_DIGEST)
registry = sub_once(
    registry,
    r"/// Unit carried by a quota dimension\..*?\n\n/// One canonical descriptor\.",
    '''/// Unit carried by a quota dimension.
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

/// One canonical descriptor.''',
    "quota units, lifecycle and rounding",
)
registry = replace_once(
    registry,
    '''    /// Semantic unit.
    pub unit: QuotaUnit,
    /// Whether the five-dimensional v0 projection carried this dimension.
    pub present_in_legacy_v0: bool,
''',
    '''    /// Semantic unit.
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
''',
    "quota descriptor semantic fields",
)
entry_replacements = (
    ("unit: QuotaUnit::Requests,", "unit: QuotaUnit::Requests,\n        lifecycle: QuotaDimensionLifecycle::Cumulative,\n        window_identity_required: false,"),
    ("unit: QuotaUnit::RequestsPerMinute,", "unit: QuotaUnit::RequestsPerWindow,\n        lifecycle: QuotaDimensionLifecycle::Windowed,\n        window_identity_required: true,"),
    ("unit: QuotaUnit::TokensPerMinute,", "unit: QuotaUnit::TokensPerWindow,\n        lifecycle: QuotaDimensionLifecycle::Windowed,\n        window_identity_required: true,"),
    ("unit: QuotaUnit::ConcurrentRequests,", "unit: QuotaUnit::ActiveOperations,\n        lifecycle: QuotaDimensionLifecycle::ActiveHold,\n        window_identity_required: true,"),
    ("unit: QuotaUnit::BudgetUnitsPerDay,", "unit: QuotaUnit::ProviderDefinedIntegerUnits,\n        lifecycle: QuotaDimensionLifecycle::Windowed,\n        window_identity_required: true,"),
    ("unit: QuotaUnit::Tokens,", "unit: QuotaUnit::ProviderDefinedIntegerUnits,\n        lifecycle: QuotaDimensionLifecycle::PerRequestBound,\n        window_identity_required: false,"),
)
for before, after in entry_replacements:
    registry = replace_once(registry, before, after, f"descriptor entry {before}")
registry = registry.replace(
    "        present_in_legacy_v0:",
    "        hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold,\n"
    "        finalize_rounding: QuotaRounding::IntegerExactOnFinalize,\n"
    "        present_in_legacy_v0:",
)
if registry.count("hold_rounding: QuotaRounding::IntegerRoundUpBeforeHold") != 6:
    raise SystemExit("expected six hold-rounding descriptor fields")
registry = replace_once(
    registry,
    '''    /// At least one projection contains a duplicate key.
    DuplicateProjectionKey,
''',
    '''    /// At least one projection contains a duplicate key.
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
''',
    "quota registry errors",
)
registry = replace_once(
    registry,
    '''    Ok(())
}

/// Compute the digest that binds field order, names, units and legacy presence.
pub fn authbus_quota_registry_digest() -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, AUTHBUS_QUOTA_REGISTRY_SCHEMA);
    bytes.extend_from_slice(&AUTHBUS_QUOTA_REGISTRY_VERSION.to_be_bytes());
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
        bytes.push(u8::from(descriptor.present_in_legacy_v0));
    }
    Sha256Digest::for_bytes(&bytes)
}
''',
    '''    if authbus_quota_registry_digest().as_str() != AUTHBUS_QUOTA_REGISTRY_SHA256 {
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
''',
    "quota registry validation and digest",
)
registry = replace_once(
    registry,
    '''    LossyLegacyDowngrade {
        /// Request count that would be discarded.
        request_count: u64,
    },
}

/// Explicit five-dimensional compatibility value. It is not schema authority.
''',
    '''    LossyLegacyDowngrade {
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", content = "value", rename_all = "snake_case")]
pub enum QuotaQuantity {
    /// The provider or contract did not declare this dimension.
    NotDeclared,
    /// The dimension exists but its value is not known.
    ExplicitUnknown,
    /// A known integer minimum-unit quantity.
    Known(u64),
}

impl Default for QuotaQuantity {
    fn default() -> Self {
        Self::NotDeclared
    }
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
        [self.request_count, self.rpm, self.tpm, self.concurrency, self.day_budget, self.context]
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
''',
    "explicit UsageVector semantics",
)
registry += '''

impl From<CanonicalQuotaVector> for UsageVector {
    fn from(value: CanonicalQuotaVector) -> Self {
        Self::known(value.request_count, value.rpm, value.tpm, value.concurrency, value.day_budget, value.context)
    }
}

impl TryFrom<UsageVector> for CanonicalQuotaVector {
    type Error = QuotaProjectionError;

    fn try_from(value: UsageVector) -> Result<Self, Self::Error> {
        fn known(value: QuotaQuantity, dimension: QuotaDimension) -> Result<u64, QuotaProjectionError> {
            match value {
                QuotaQuantity::Known(value) => Ok(value),
                QuotaQuantity::ExplicitUnknown => Err(QuotaProjectionError::UnknownDimension { dimension }),
                QuotaQuantity::NotDeclared => Err(QuotaProjectionError::NotDeclaredDimension { dimension }),
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
'''
write(registry_path, registry)

contracts_lib_path = "codex-rs/hepta-contracts/src/lib.rs"
contracts_lib = read(contracts_lib_path)
contracts_lib = replace_once(
    contracts_lib,
    "pub use quota_registry::QuotaProjectionError;\n",
    "pub use quota_registry::QuotaProjectionError;\n"
    "pub use quota_registry::QuotaQuantity;\n"
    "pub use quota_registry::QuotaRounding;\n"
    "pub use quota_registry::QuotaDimensionLifecycle;\n"
    "pub use quota_registry::UsageVector;\n"
    "pub use quota_registry::UsageVectorMarketability;\n"
    "pub use quota_registry::AUTHBUS_QUOTA_PROJECTION_TRANSFORM;\n"
    "pub use quota_registry::AUTHBUS_QUOTA_SEMANTIC_REVISION;\n"
    "pub use quota_registry::AUTHBUS_QUOTA_SOURCE_DOMAIN_REF;\n"
    "pub use quota_registry::AUTHBUS_QUOTA_SOURCE_REGISTRY_REF;\n"
    "pub use quota_registry::AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256;\n",
    "quota registry exports",
)
contracts_lib = replace_once(
    contracts_lib,
    "pub use quota_registry::validate_authbus_quota_registry;\n",
    "pub use quota_registry::validate_authbus_quota_registry;\n"
    "pub use quota_registry::validate_authbus_quota_source_binding;\n",
    "quota source validator export",
)
contracts_lib = replace_once(
    contracts_lib,
    "pub use authbus::b2::QuotaReservation;\n",
    "pub use authbus::b2::QuotaReservation;\n"
    "pub use authbus::b2::QuotaReservationV1_3;\n"
    "pub use authbus::b2::QuotaWindowBindingV1_3;\n",
    "B2 canonical reservation exports",
)
write(contracts_lib_path, contracts_lib)

b2_path = "codex-rs/hepta-contracts/src/authbus_b2.rs"
b2 = read(b2_path)
b2 = replace_once(
    b2,
    "/// Quota hold created after admission and before a physical effect.\n",
    "/// Decode-only four-dimensional quota hold retained for compatibility.\n"
    "/// New v1.3 encoders use [`QuotaReservationV1_3`] and [`crate::UsageVector`].\n",
    "legacy B2 decode-only marker",
)
b2 += '''

/// One durable window identity carried by a canonical v1.3 reservation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuotaWindowBindingV1_3 {
    pub window_kind: String,
    pub starts_at_unix_seconds: u64,
    pub ends_at_unix_seconds: u64,
    pub timezone_or_offset: String,
}

impl QuotaWindowBindingV1_3 {
    fn validate(&self) -> Result<(), AuthBusContractError> {
        validate_text(&self.window_kind, "quota window kind", 128)?;
        validate_text(&self.timezone_or_offset, "quota window timezone or offset", 128)?;
        validate_window(self.starts_at_unix_seconds, self.ends_at_unix_seconds)
    }
}

/// Canonical v1.3 reservation projection. Every quantity uses the same
/// six-dimensional UsageVector declaration state, and source registry identity
/// is carried on the wire so stale or lossy projections fail closed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct QuotaReservationV1_3 {
    pub schema_version: u32,
    pub reservation_id: String,
    pub command_id: String,
    pub idempotency_key: String,
    pub resource_id: String,
    pub resource_digest: Sha256Digest,
    pub quota_domain: String,
    pub window_bindings: Vec<QuotaWindowBindingV1_3>,
    pub estimated_vector: crate::UsageVector,
    pub safety_margin_vector: crate::UsageVector,
    pub held_vector: crate::UsageVector,
    pub consumed_vector: crate::UsageVector,
    pub remaining_vector: crate::UsageVector,
    pub state: QuotaReservationState,
    pub issued_at_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub payload_digest: Sha256Digest,
    pub policy_digest: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
    pub expected_revision: u64,
    pub revision: u64,
    pub prior_state_digest: Sha256Digest,
    pub state_digest: Sha256Digest,
    pub source_registry_ref: String,
    pub source_registry_sha256: String,
    pub source_domain_ref: String,
    pub projection_transform: String,
    pub semantic_revision: String,
    #[serde(default)]
    pub authority: bool,
}

impl QuotaReservationV1_3 {
    pub fn validate(&self) -> Result<(), AuthBusContractError> {
        validate_schema(self.schema_version, "QuotaReservationV1_3")?;
        validate_id(&self.reservation_id, "reservation id")?;
        validate_id(&self.command_id, "reservation command id")?;
        validate_id(&self.idempotency_key, "reservation idempotency key")?;
        validate_id(&self.resource_id, "reservation resource id")?;
        validate_digest(&self.resource_digest, "reservation resource digest")?;
        validate_text(&self.quota_domain, "reservation quota domain", 512)?;
        if self.window_bindings.len() > MAX_LIST_ITEMS {
            return Err(error("too many quota window bindings"));
        }
        for window in &self.window_bindings {
            window.validate()?;
        }
        self.estimated_vector
            .validate_for_admission()
            .map_err(|_| error("estimated UsageVector is not admission-safe"))?;
        self.held_vector
            .validate_for_admission()
            .map_err(|_| error("held UsageVector is not admission-safe"))?;
        for vector in [self.safety_margin_vector, self.consumed_vector, self.remaining_vector] {
            vector
                .validate_declared_shape()
                .map_err(|_| error("UsageVector contains an explicit unknown"))?;
        }
        for vector in [self.safety_margin_vector, self.held_vector, self.consumed_vector, self.remaining_vector] {
            if !self.estimated_vector.declarations_match(vector) {
                return Err(error("UsageVector declaration mismatch"));
            }
        }
        if !matches!(
            self.consumed_vector.quantity(crate::QuotaDimension::Concurrency),
            crate::QuotaQuantity::NotDeclared | crate::QuotaQuantity::Known(0)
        ) {
            return Err(error("concurrency is released and cannot be consumed"));
        }
        let requires_window = [
            crate::QuotaDimension::Rpm,
            crate::QuotaDimension::Tpm,
            crate::QuotaDimension::Concurrency,
            crate::QuotaDimension::DayBudget,
        ]
        .into_iter()
        .any(|dimension| {
            matches!(
                self.estimated_vector.quantity(dimension),
                crate::QuotaQuantity::Known(value) if value > 0
            )
        });
        if requires_window && self.window_bindings.is_empty() {
            return Err(error("windowed UsageVector requires a durable window identity"));
        }
        validate_window(self.issued_at_unix_seconds, self.expires_at_unix_seconds)?;
        validate_digest(&self.payload_digest, "reservation payload digest")?;
        validate_digest(&self.policy_digest, "reservation policy digest")?;
        validate_epochs(
            self.authority_epoch,
            self.owner_epoch,
            self.generation,
            &self.fencing_token_sha256,
        )?;
        validate_revision_pair(self.expected_revision, self.revision)?;
        validate_digest(&self.prior_state_digest, "reservation prior state digest")?;
        validate_digest(&self.state_digest, "reservation state digest")?;
        crate::validate_authbus_quota_source_binding(
            &self.source_registry_ref,
            &self.source_registry_sha256,
            &self.source_domain_ref,
            &self.projection_transform,
            &self.semantic_revision,
        )
        .map_err(|_| error("quota registry projection binding mismatch"))?;
        validate_authority(self.authority)
    }
}

impl_contract_methods!(QuotaReservationV1_3, "quota-reservation-v1-3");
'''
write(b2_path, b2)

p13_tests_path = "codex-rs/hepta-authbus-p1-3-qualification/tests/p1_3.rs"
p13_tests = read(p13_tests_path)
p13_tests += '''

#[test]
fn usage_vector_preserves_absent_unknown_rounding_and_scalar_semantics() {
    use codex_hepta_contracts::AUTHBUS_QUOTA_DIMENSIONS;
    use codex_hepta_contracts::AUTHBUS_QUOTA_PROJECTION_TRANSFORM;
    use codex_hepta_contracts::AUTHBUS_QUOTA_SEMANTIC_REVISION;
    use codex_hepta_contracts::AUTHBUS_QUOTA_SOURCE_DOMAIN_REF;
    use codex_hepta_contracts::AUTHBUS_QUOTA_SOURCE_REGISTRY_REF;
    use codex_hepta_contracts::AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256;
    use codex_hepta_contracts::QuotaDimension;
    use codex_hepta_contracts::QuotaProjectionError;
    use codex_hepta_contracts::QuotaQuantity;
    use codex_hepta_contracts::QuotaRegistryError;
    use codex_hepta_contracts::UsageVector;
    use codex_hepta_contracts::UsageVectorMarketability;
    use codex_hepta_contracts::validate_authbus_quota_source_binding;

    let request_only = UsageVector::request_count_only(7);
    assert_eq!(request_only.marketability(), UsageVectorMarketability::RequestCountOnly);
    assert_eq!(request_only.try_to_legacy_max_uses(), Ok(7));
    assert_eq!(UsageVector::default().validate_for_admission(), Err(QuotaProjectionError::EmptyUsageVector));

    let unknown = UsageVector {
        request_count: QuotaQuantity::ExplicitUnknown,
        ..UsageVector::default()
    };
    assert_eq!(
        unknown.validate_declared_shape(),
        Err(QuotaProjectionError::UnknownDimension {
            dimension: QuotaDimension::RequestCount,
        })
    );
    assert_eq!(unknown.marketability(), UsageVectorMarketability::NotMarketableUnknown);

    let full = UsageVector::known(1, 2, 300, 1, 400, 512);
    assert_eq!(full.marketability(), UsageVectorMarketability::FullVector);
    assert_eq!(full.try_to_legacy_max_uses(), Err(QuotaProjectionError::ScalarCompatibilityViolation));

    assert!(AUTHBUS_QUOTA_DIMENSIONS.iter().all(|descriptor| {
        descriptor.hold_rounding.as_str() == "integer_round_up_before_hold"
    }));
    assert!(AUTHBUS_QUOTA_DIMENSIONS.iter().all(|descriptor| {
        descriptor.finalize_rounding.as_str() == "integer_exact_on_finalize"
    }));
    assert_eq!(
        validate_authbus_quota_source_binding(
            AUTHBUS_QUOTA_SOURCE_REGISTRY_REF,
            AUTHBUS_QUOTA_SOURCE_REGISTRY_SHA256,
            AUTHBUS_QUOTA_SOURCE_DOMAIN_REF,
            AUTHBUS_QUOTA_PROJECTION_TRANSFORM,
            AUTHBUS_QUOTA_SEMANTIC_REVISION,
        ),
        Ok(())
    );
    assert_eq!(
        validate_authbus_quota_source_binding(
            AUTHBUS_QUOTA_SOURCE_REGISTRY_REF,
            "00",
            AUTHBUS_QUOTA_SOURCE_DOMAIN_REF,
            AUTHBUS_QUOTA_PROJECTION_TRANSFORM,
            AUTHBUS_QUOTA_SEMANTIC_REVISION,
        ),
        Err(QuotaRegistryError::SourceRegistryDigestMismatch)
    );
}

#[test]
fn b2_exports_the_canonical_six_dimension_reservation_projection() {
    assert!(std::mem::size_of::<codex_hepta_contracts::QuotaReservationV1_3>() > 0);
    assert!(std::mem::size_of::<codex_hepta_contracts::QuotaWindowBindingV1_3>() > 0);
}
'''
write(p13_tests_path, p13_tests)

verify_path = "scripts/verify-authbus-p1-3.py"
verify = read(verify_path).replace(OLD_REGISTRY_DIGEST, NEW_REGISTRY_DIGEST)
verify += f'''

# Semantic B2 closure: source registry binding, explicit unknown/absence,
# deterministic rounding, lifecycle and canonical six-dimensional wire shape.
_semantic_root = Path(__file__).resolve().parents[1]
_semantic_registry = (_semantic_root / "codex-rs/hepta-contracts/src/quota_registry.rs").read_text(encoding="utf-8")
_semantic_b2 = (_semantic_root / "codex-rs/hepta-contracts/src/authbus_b2.rs").read_text(encoding="utf-8")
_semantic_contract = (_semantic_root / "docs/hepta-vnext/dropbox-current-2026-08-27/root/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml").read_text(encoding="utf-8")
_semantic_canonical = (_semantic_root / "docs/hepta-vnext/dropbox-current-2026-08-27/root/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml").read_text(encoding="utf-8")
assert 'pub struct UsageVector' in _semantic_registry
assert 'pub enum QuotaQuantity' in _semantic_registry
assert 'ExplicitUnknown' in _semantic_registry
assert 'NotDeclared' in _semantic_registry
assert 'pub enum QuotaDimensionLifecycle' in _semantic_registry
assert 'pub enum QuotaRounding' in _semantic_registry
assert 'integer_round_up_before_hold' in _semantic_registry
assert 'integer_exact_on_finalize' in _semantic_registry
assert '{SOURCE_REGISTRY_REF}' in _semantic_registry
assert '{SOURCE_REGISTRY_DIGEST}' in _semantic_registry
assert '{SOURCE_DOMAIN_REF}' in _semantic_registry
assert '{PROJECTION_TRANSFORM}' in _semantic_registry
assert '{SEMANTIC_REVISION}' in _semantic_registry
assert 'pub struct QuotaReservationV1_3' in _semantic_b2
assert 'Decode-only four-dimensional quota hold retained for compatibility.' in _semantic_b2
assert 'absent_dimension: not_declared' in _semantic_contract
assert 'unknown_representation: explicit_unknown' in _semantic_contract
assert 'rounding: integer_round_up_before_hold_and_integer_exact_on_finalize' in _semantic_contract
assert 'mixed_vector_scalar_encoding: forbidden' in _semantic_contract
assert 'status: REQUIRED_AT_B2' in _semantic_canonical
assert 'projection_must_record: [source_registry_ref, source_registry_digest, projection_transform]' in _semantic_canonical
'''
write(verify_path, verify)

for path in (
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_DEVELOPMENT_PLAN_2026-08-29.md",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.md",
):
    content = read(path).replace(OLD_REGISTRY_DIGEST, NEW_REGISTRY_DIGEST)
    content += '''

## Canonical B2 semantic closure

The executable tranche also binds the registry to the active AUTHBUS.11-v1.3
source registry and domain projection. The canonical wire value distinguishes
`known`, `explicit_unknown`, and `not_declared`; absence is never converted to
zero. Descriptor metadata owns exact units, lifecycle, window requirements,
integer round-up before hold, and integer-exact terminal finalization. The old
four-dimensional B2 reservation remains decode-only compatibility material;
new source uses `QuotaReservationV1_3` and the six-dimensional `UsageVector`.
A request-count-only scalar may be decoded or advertised only as
request-count-only and cannot imply rpm, tpm, concurrency, day-budget, or
context authority.
'''
    write(path, content)

status_json_path = "docs/hepta-vnext/authbus/AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.json"
status = json.loads(read(status_json_path))
status.update(
    {
        "b2_canonical_usage_vector": True,
        "explicit_unknown_preserved": True,
        "not_declared_preserved": True,
        "rounding_bound": True,
        "lifecycle_bound": True,
        "source_registry_bound": True,
        "legacy_b2_decode_only": True,
        "request_count_only_marketability": True,
        "registry_sha256": NEW_REGISTRY_DIGEST,
    }
)
write(status_json_path, json.dumps(status, indent=2, sort_keys=True) + "\n")

for path in (
    "codex-rs/hepta-contracts/src/quota_registry.rs",
    "codex-rs/hepta-authbus-p1-3-qualification/README.md",
    "codex-rs/hepta-authbus-p1-3-qualification/src/lib.rs",
    "codex-rs/hepta-authbus-p1-3-qualification/tests/p1_3.rs",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_DEVELOPMENT_PLAN_2026-08-29.md",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.md",
    "scripts/verify-authbus-p1-3.py",
):
    content = read(path)
    if OLD_REGISTRY_DIGEST in content:
        write(path, content.replace(OLD_REGISTRY_DIGEST, NEW_REGISTRY_DIGEST))

print("applied_authbus_p1_3_semantic_completion=1")
print(f"authbus_p1_3_registry_sha256={NEW_REGISTRY_DIGEST}")
print("closed_b2_four_dimension_wire_gap=1")
print("closed_absent_unknown_rounding_binding_gap=1")
print("closed_source_registry_projection_binding_gap=1")
