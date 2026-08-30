#![cfg(feature = "p1-3-qualification")]

use std::collections::BTreeMap;

use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaVector as P03QuotaVector;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_AUTHORITY;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_EFFECT_AUTHORITY;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_EXECUTE_ALLOWED;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_G5_ALLOWED;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_OPERATOR_ACCEPTANCE;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_PRODUCTION_CALLER;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_PRODUCTION_WRITER;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_PROMOTION;
use codex_hepta_authbus_p1_3_qualification::AUTHBUS_P1_3_QUALIFICATION_ONLY;
use codex_hepta_authbus_qualification::QualificationQuota;
use codex_hepta_contracts::AUTHBUS_QUOTA_DIMENSION_COUNT;
use codex_hepta_contracts::AUTHBUS_QUOTA_DIMENSIONS;
use codex_hepta_contracts::AUTHBUS_QUOTA_REGISTRY_SCHEMA;
use codex_hepta_contracts::AUTHBUS_QUOTA_REGISTRY_SHA256;
use codex_hepta_contracts::CanonicalQuotaLimits;
use codex_hepta_contracts::CanonicalQuotaVector;
use codex_hepta_contracts::LegacyQuotaVectorV0;
use codex_hepta_contracts::LegacyRequestCountPolicy;
use codex_hepta_contracts::QuotaProjection;
use codex_hepta_contracts::QuotaProjectionError;
use codex_hepta_contracts::QuotaVector;
use codex_hepta_contracts::authbus_quota_registry_digest;
use codex_hepta_contracts::migrate_legacy_quota;
use codex_hepta_contracts::validate_authbus_quota_registry;
use pretty_assertions::assert_eq;

#[test]
fn authority_boundary_remains_closed() {
    assert!(AUTHBUS_P1_3_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P1_3_AUTHORITY);
    assert!(!AUTHBUS_P1_3_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P1_3_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P1_3_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P1_3_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P1_3_PROMOTION);
    assert!(!AUTHBUS_P1_3_G5_ALLOWED);
    assert!(!AUTHBUS_P1_3_EXECUTE_ALLOWED);
}

#[test]
fn p0_3_scheduler_reexports_the_contract_owned_type() {
    let p03 = P03QuotaVector::new(1, 2, 300, 1, 400, 512);
    let contract: CanonicalQuotaVector = p03;
    assert_eq!(contract.request_count, 1);
    assert_eq!(
        contract.digest(),
        CanonicalQuotaVector::new(1, 2, 300, 1, 400, 512).digest()
    );
}

#[test]
fn descriptor_registry_owns_every_projection_surface() {
    validate_authbus_quota_registry().expect("valid quota registry");
    assert_eq!(
        AUTHBUS_QUOTA_DIMENSIONS.len(),
        AUTHBUS_QUOTA_DIMENSION_COUNT
    );
    assert_eq!(
        AUTHBUS_QUOTA_REGISTRY_SCHEMA,
        "hepta.authbus.quota-registry.v1"
    );
    assert_eq!(
        authbus_quota_registry_digest().as_str(),
        AUTHBUS_QUOTA_REGISTRY_SHA256
    );

    let vector = CanonicalQuotaVector::new(1, 2, 300, 1, 400, 512);
    assert_eq!(
        vector.project(QuotaProjection::Wire),
        BTreeMap::from([
            ("concurrency", 1),
            ("context", 512),
            ("day_budget", 400),
            ("request_count", 1),
            ("rpm", 2),
            ("tpm", 300),
        ])
    );
    assert_eq!(
        vector.project(QuotaProjection::SqliteLimit),
        BTreeMap::from([
            ("limit_concurrency", 1),
            ("limit_context", 512),
            ("limit_day_budget", 400),
            ("limit_request_count", 1),
            ("limit_rpm", 2),
            ("limit_tpm", 300),
        ])
    );
    assert_eq!(
        vector.project(QuotaProjection::SqliteReserved),
        BTreeMap::from([
            ("reserved_concurrency", 1),
            ("reserved_context", 512),
            ("reserved_day_budget", 400),
            ("reserved_request_count", 1),
            ("reserved_rpm", 2),
            ("reserved_tpm", 300),
        ])
    );
    assert_eq!(
        vector.project(QuotaProjection::SqliteUsed),
        BTreeMap::from([
            ("used_concurrency", 1),
            ("used_context", 512),
            ("used_day_budget", 400),
            ("used_request_count", 1),
            ("used_rpm", 2),
            ("used_tpm", 300),
        ])
    );
    assert_eq!(
        vector.project(QuotaProjection::Receipt),
        vector.project(QuotaProjection::Wire)
    );
    assert_eq!(
        vector.project(QuotaProjection::Metric),
        BTreeMap::from([
            ("quota_concurrency", 1),
            ("quota_context", 512),
            ("quota_day_budget", 400),
            ("quota_request_count", 1),
            ("quota_rpm", 2),
            ("quota_tpm", 300),
        ])
    );

    let serialized = serde_json::to_value(vector).expect("serialize canonical quota");
    assert_eq!(serialized.as_object().expect("object").len(), 6);
    assert_eq!(serialized["request_count"], 1);
}

#[test]
fn b4_legacy_projection_requires_an_explicit_request_count_policy() {
    let legacy = QuotaVector::new(2, 300, 1, 400, 512);
    assert_eq!(
        legacy.try_into_canonical(LegacyRequestCountPolicy::RejectMissing),
        Err(QuotaProjectionError::MissingRequestCount)
    );
    let canonical = legacy
        .try_into_canonical(LegacyRequestCountPolicy::AssumeOnePerPermit)
        .expect("explicit B4 migration");
    assert_eq!(canonical.request_count, 1);
    assert_eq!(QuotaVector::try_from_canonical(canonical), Ok(legacy));
    assert_eq!(
        QuotaVector::try_from_canonical(CanonicalQuotaVector::new(2, 2, 300, 1, 400, 512)),
        Err(QuotaProjectionError::LossyLegacyDowngrade { request_count: 2 })
    );
}

#[test]
fn p0_2_storage_projection_requires_the_same_explicit_policy() {
    let legacy = QualificationQuota {
        rpm: 2,
        tpm: 300,
        concurrency: 1,
        day_budget: 400,
        context: 512,
    };
    assert_eq!(
        legacy.try_into_canonical(LegacyRequestCountPolicy::RejectMissing),
        Err(QuotaProjectionError::MissingRequestCount)
    );
    let canonical = legacy
        .try_into_canonical(LegacyRequestCountPolicy::AssumeOnePerPermit)
        .expect("explicit P0.2 migration");
    assert_eq!(canonical.request_count, 1);
    assert_eq!(
        QualificationQuota::try_from_canonical(canonical),
        Ok(legacy)
    );
    assert_eq!(
        QualificationQuota::try_from_canonical(CanonicalQuotaVector::new(0, 2, 300, 1, 400, 512)),
        Err(QuotaProjectionError::LossyLegacyDowngrade { request_count: 0 })
    );
}

#[test]
fn migration_receipt_binds_the_target_registry() {
    let legacy = LegacyQuotaVectorV0 {
        rpm: 2,
        tpm: 300,
        concurrency: 1,
        day_budget: 400,
        context: 512,
    };
    let (canonical, receipt) =
        migrate_legacy_quota(legacy, LegacyRequestCountPolicy::AssumeOnePerPermit)
            .expect("migration");
    assert_eq!(canonical.request_count, 1);
    assert!(receipt.request_count_assumed);
    assert_eq!(receipt.target_schema, AUTHBUS_QUOTA_REGISTRY_SCHEMA);
    assert_eq!(
        receipt.target_registry_sha256,
        authbus_quota_registry_digest()
    );
}

#[test]
fn unknown_dimensions_and_arithmetic_overflow_fail_closed() {
    let capacity = CanonicalQuotaVector::new(10, 10, 1_000, 4, 10_000, 20_000);
    let requested = CanonicalQuotaVector::new(1, 1, 100, 1, 100, 512);
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
    assert_eq!(
        request_only.marketability(),
        UsageVectorMarketability::RequestCountOnly
    );
    assert_eq!(request_only.try_to_legacy_max_uses(), Ok(7));
    assert_eq!(
        UsageVector::default().validate_for_admission(),
        Err(QuotaProjectionError::EmptyUsageVector)
    );

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
    assert_eq!(
        unknown.marketability(),
        UsageVectorMarketability::NotMarketableUnknown
    );

    let full = UsageVector::known(1, 2, 300, 1, 400, 512);
    assert_eq!(full.marketability(), UsageVectorMarketability::FullVector);
    assert_eq!(
        full.try_to_legacy_max_uses(),
        Err(QuotaProjectionError::ScalarCompatibilityViolation)
    );

    assert!(
        AUTHBUS_QUOTA_DIMENSIONS.iter().all(|descriptor| {
            descriptor.hold_rounding.as_str() == "integer_round_up_before_hold"
        })
    );
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
