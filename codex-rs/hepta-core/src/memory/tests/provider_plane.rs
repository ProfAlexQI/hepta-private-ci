use super::*;

#[test]
fn memory_provider_plane_native_default_is_contract_ready() {
    let report = MemoryProviderPlaneReport::native_default();

    assert_eq!(report.provider_count, 2);
    assert_eq!(report.active_provider_count, 1);
    assert_eq!(report.external_provider_count, 1);
    assert_eq!(report.active_external_provider_count, 0);
    assert!(report.builtin_present);
    assert!(report.exactly_one_external_active_or_none);
    assert!(report.context_fencing_required);
    assert!(report.all_active_providers_prefetch);
    assert!(report.all_active_providers_sync);
    assert!(report.provenance_required);
    assert!(report.deletion_path_available);
    assert!(report.contract_ready());
    assert!(
        report
            .capabilities
            .contains(&MemoryProviderCapability::SemanticSearch)
    );
    assert!(
        report
            .capabilities
            .contains(&MemoryProviderCapability::Conclusions)
    );
}

#[test]
fn memory_provider_plane_rejects_multiple_active_external_providers() {
    let report = MemoryProviderPlaneReport::from_providers(vec![
        MemoryProviderDescriptor::builtin(),
        MemoryProviderDescriptor::external_slot("external-a", MemoryProviderStatus::Active),
        MemoryProviderDescriptor::external_slot("external-b", MemoryProviderStatus::Active),
    ]);

    assert_eq!(report.active_external_provider_count, 2);
    assert!(!report.exactly_one_external_active_or_none);
    assert!(!report.contract_ready());
}

#[test]
fn memory_provider_plane_roundtrips_through_json() {
    let report = MemoryProviderPlaneReport::native_default();
    let json = serde_json::to_string(&report).expect("provider plane should serialize");
    let parsed: MemoryProviderPlaneReport =
        serde_json::from_str(&json).expect("provider plane should deserialize");

    assert_eq!(parsed, report);
    assert!(json.contains("context_fencing_required"));
    assert!(json.contains("external-user-modeling-slot"));
}
