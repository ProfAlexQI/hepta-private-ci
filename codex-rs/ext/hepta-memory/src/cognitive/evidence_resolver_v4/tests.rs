use super::*;

fn quote(value: &str, occurrence: u32) -> EvidenceLocatorV4 {
    EvidenceLocatorV4::ExactQuote(ExactQuoteLocatorV4 {
        quote: value.to_string(),
        occurrence,
    })
}

fn segment(segment_id: &str) -> EvidenceLocatorV4 {
    EvidenceLocatorV4::SourceSegment(SourceSegmentLocatorV4 {
        segment_id: segment_id.to_string(),
    })
}

fn entity_input(label: &str, evidence: Vec<EvidenceLocatorV4>) -> GroundedToolV4Input {
    entity_input_with("aurora", "project", label, evidence)
}

fn entity_input_with(
    key: &str,
    entity_type: &str,
    label: &str,
    evidence: Vec<EvidenceLocatorV4>,
) -> GroundedToolV4Input {
    GroundedToolV4Input {
        entities: vec![GroundedEntityV4 {
            key: key.to_string(),
            entity_type: entity_type.to_string(),
            label: label.to_string(),
            evidence,
        }],
        relations: Vec::new(),
    }
}

#[test]
fn v4_schema_accepts_host_resolvable_selectors_only() {
    let schema = grounded_tool_v4_schema();
    let encoded = schema.to_string();
    assert!(encoded.contains("quote"));
    assert!(encoded.contains("occurrence"));
    assert!(encoded.contains("segment_id"));
    assert!(!encoded.contains("start_byte"));
    assert!(!encoded.contains("end_byte"));
    assert!(!encoded.contains("sha256"));
    const _: () = {
        assert!(!GROUNDED_TOOL_V4_REGISTERED);
        assert!(!GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY);
        assert!(!MODEL_SUPPLIED_BYTE_OFFSETS);
        assert!(!MODEL_SUPPLIED_DIGESTS);
    };
}

#[test]
fn exact_quote_resolves_unicode_occurrence_and_host_digest() {
    let source = "项目 Aurora 使用 Rust。项目 Aurora 使用 Rust。";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let resolved = resolver
        .prepare_grounded_tool_v4(entity_input(
            "项目 Aurora",
            vec![quote("项目 Aurora 使用 Rust", 1)],
        ))
        .expect("resolved");
    let expected_start = source.rfind("项目 Aurora 使用 Rust").expect("second quote");
    let span = &resolved.grounded.evidence[0];
    assert_eq!(
        usize::try_from(span.start_byte).expect("start"),
        expected_start
    );
    assert_eq!(
        span.evidence_sha256,
        Sha256Digest::for_bytes(
            &source.as_bytes()[expected_start..expected_start + "项目 Aurora 使用 Rust".len()]
        )
    );
    assert!(resolved.resolution.host_resolved_byte_offsets);
    assert!(resolved.resolution.host_resolved_digests);
    resolved.resolution.validate().expect("receipt");
}

#[test]
fn exact_quote_occurrence_is_fail_closed() {
    let source = "Aurora Aurora";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let error = resolver
        .prepare_grounded_tool_v4(entity_input("Aurora", vec![quote("Aurora", 2)]))
        .expect_err("missing occurrence");
    assert!(error.contains("does not exist"));
}

#[test]
fn host_segment_id_resolves_without_model_offsets_or_digest() {
    let source = "Project Aurora uses Rust for deployment.";
    let start = source.find("Project Aurora uses Rust").expect("segment");
    let end = start + "Project Aurora uses Rust".len();
    let resolver = HostEvidenceResolverV1::new(
        source,
        &[SourceSegmentDraftV1::new(
            u32::try_from(start).expect("start"),
            u32::try_from(end).expect("end"),
        )],
    )
    .expect("resolver");
    let descriptor = &resolver.segment_descriptors()[0];
    let resolved = resolver
        .prepare_grounded_tool_v4(entity_input(
            "Project Aurora",
            vec![segment(descriptor.segment_id.as_str())],
        ))
        .expect("resolved");
    let span = &resolved.grounded.evidence[0];
    assert_eq!(span.start_byte, descriptor.start_byte);
    assert_eq!(span.end_byte, descriptor.end_byte);
    assert_eq!(span.evidence_sha256, descriptor.evidence_sha256);
    assert_eq!(
        resolved.resolution.source_content_sha256.as_str(),
        resolver.source_content_sha256().as_str()
    );
    assert_eq!(
        resolved.resolution.segment_catalog_sha256.as_str(),
        resolver.segment_catalog_sha256().as_str()
    );
}

#[test]
fn segment_ids_are_bound_to_the_exact_source() {
    let first_source = "Project Aurora uses Rust.";
    let first = HostEvidenceResolverV1::new(first_source, &[SourceSegmentDraftV1::new(0, 20)])
        .expect("first resolver");
    let foreign_id = first.segment_descriptors()[0].segment_id.as_str();
    let second_source = "Project Borealis uses Go.";
    let second = HostEvidenceResolverV1::new(second_source, &[SourceSegmentDraftV1::new(0, 21)])
        .expect("second resolver");
    let error = second
        .prepare_grounded_tool_v4(entity_input("Project Borealis", vec![segment(foreign_id)]))
        .expect_err("foreign segment must fail");
    assert!(error.contains("unknown source segment"));
}

#[test]
fn duplicate_and_overlapping_ranges_are_rejected_per_fact() {
    let source = "Project Aurora uses Rust.";
    let start = source.find("Project Aurora").expect("segment");
    let end = start + "Project Aurora".len();
    let resolver = HostEvidenceResolverV1::new(
        source,
        &[SourceSegmentDraftV1::new(
            u32::try_from(start).expect("start"),
            u32::try_from(end).expect("end"),
        )],
    )
    .expect("resolver");
    let segment_id = resolver.segment_descriptors()[0].segment_id.as_str();
    let duplicate = resolver
        .prepare_grounded_tool_v4(entity_input(
            "Project Aurora",
            vec![quote("Project Aurora", 0), segment(segment_id)],
        ))
        .expect_err("duplicate range");
    assert!(duplicate.contains("duplicate evidence ranges"));

    let overlapping = resolver
        .prepare_grounded_tool_v4(entity_input(
            "Project Aurora",
            vec![quote("Project Aurora", 0), quote("Aurora uses", 0)],
        ))
        .expect_err("overlap");
    assert!(overlapping.contains("overlapping evidence ranges"));
}

#[test]
fn receipt_is_digest_bound_and_contains_no_source_body() {
    let source = "Project Aurora uses Rust. secret-source-marker";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let mut resolved = resolver
        .prepare_grounded_tool_v4(entity_input(
            "Project Aurora",
            vec![quote("Project Aurora uses Rust", 0)],
        ))
        .expect("resolved");
    let encoded = serde_json::to_string(&resolved.resolution).expect("json");
    assert!(!encoded.contains(source));
    assert!(!encoded.contains("secret-source-marker"));
    resolved.resolution.model_supplied_digests = true;
    assert!(resolved.resolution.validate().is_err());
}

#[test]
fn relation_endpoints_must_reference_declared_entities() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let input = GroundedToolV4Input {
        entities: vec![GroundedEntityV4 {
            key: "aurora".to_string(),
            entity_type: "project".to_string(),
            label: "Project Aurora".to_string(),
            evidence: vec![quote("Project Aurora", 0)],
        }],
        relations: vec![GroundedRelationV4 {
            key: "aurora-uses-rust".to_string(),
            from_entity_key: "aurora".to_string(),
            to_entity_key: "rust".to_string(),
            relation: "uses".to_string(),
            evidence: vec![quote("Project Aurora uses Rust", 0)],
        }],
    };
    let error = resolver
        .prepare_grounded_tool_v4(input)
        .expect_err("unknown endpoint must fail");
    assert!(error.contains("references an unknown entity key"));
}

#[test]
fn resolution_receipt_binds_the_complete_tool_input() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let first = resolver
        .prepare_grounded_tool_v4(entity_input(
            "Project Aurora",
            vec![quote("Project Aurora", 0)],
        ))
        .expect("first");
    let second = resolver
        .prepare_grounded_tool_v4(entity_input_with(
            "aurora",
            "initiative",
            "Project Aurora",
            vec![quote("Project Aurora", 0)],
        ))
        .expect("second");
    assert_ne!(
        first.resolution.tool_input_sha256,
        second.resolution.tool_input_sha256
    );
    assert_ne!(
        first.resolution.receipt_sha256,
        second.resolution.receipt_sha256
    );
}

#[test]
fn exact_quote_occurrences_support_deterministic_overlaps() {
    let source = "aaaa";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let resolved = resolver
        .prepare_grounded_tool_v4(entity_input_with(
            "pair",
            "token",
            "aa",
            vec![quote("aa", 2)],
        ))
        .expect("third overlapping match");
    let span = &resolved.grounded.evidence[0];
    assert_eq!(span.start_byte, 2);
    assert_eq!(span.end_byte, 4);
}

#[test]
fn exact_quote_validation_is_bounded_and_fail_closed() {
    let source = "Project Aurora";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    for locator in [
        quote(" ", 0),
        quote("Project\0Aurora", 0),
        quote("Project Aurora", MAX_QUOTE_OCCURRENCE + 1),
    ] {
        assert!(
            resolver
                .prepare_grounded_tool_v4(entity_input("Project Aurora", vec![locator]))
                .is_err()
        );
    }
    let oversized = "x".repeat(MAX_QUOTE_BYTES + 1);
    assert!(
        resolver
            .prepare_grounded_tool_v4(entity_input(
                "Project Aurora",
                vec![quote(oversized.as_str(), 0)],
            ))
            .is_err()
    );
}

#[test]
fn source_segment_catalog_is_deterministic_and_rejects_duplicate_ranges() {
    let source = "Project Aurora uses Rust.";
    let first_start = source.find("Project Aurora").expect("first");
    let first_end = first_start + "Project Aurora".len();
    let second_start = source.find("Rust").expect("second");
    let second_end = second_start + "Rust".len();
    let first = SourceSegmentDraftV1::new(
        u32::try_from(first_start).expect("start"),
        u32::try_from(first_end).expect("end"),
    );
    let second = SourceSegmentDraftV1::new(
        u32::try_from(second_start).expect("start"),
        u32::try_from(second_end).expect("end"),
    );
    let ordered = HostEvidenceResolverV1::new(source, &[first, second]).expect("ordered");
    let reversed = HostEvidenceResolverV1::new(source, &[second, first]).expect("reversed");
    assert_eq!(
        ordered.segment_descriptors(),
        reversed.segment_descriptors()
    );
    assert_eq!(
        ordered.segment_catalog_sha256(),
        reversed.segment_catalog_sha256()
    );
    let duplicate = HostEvidenceResolverV1::new(source, &[first, first])
        .err()
        .expect("duplicate range must fail");
    assert!(duplicate.contains("duplicate segment range"));
}

#[test]
fn malformed_and_unknown_segment_ids_are_rejected() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let uppercase = "A".repeat(64);
    let unknown = "0".repeat(64);
    for segment_id in [
        "not-a-segment".to_string(),
        format!("{SOURCE_SEGMENT_ID_PREFIX}{uppercase}"),
        format!("{SOURCE_SEGMENT_ID_PREFIX}{unknown}"),
    ] {
        assert!(
            resolver
                .prepare_grounded_tool_v4(entity_input(
                    "Project Aurora",
                    vec![segment(segment_id.as_str())],
                ))
                .is_err()
        );
    }
}

#[test]
fn different_facts_may_share_the_same_resolved_source_span() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let shared = quote("Project Aurora uses Rust", 0);
    let resolved = resolver
        .prepare_grounded_tool_v4(GroundedToolV4Input {
            entities: vec![
                GroundedEntityV4 {
                    key: "aurora".to_string(),
                    entity_type: "project".to_string(),
                    label: "Project Aurora".to_string(),
                    evidence: vec![shared.clone()],
                },
                GroundedEntityV4 {
                    key: "rust".to_string(),
                    entity_type: "language".to_string(),
                    label: "Rust".to_string(),
                    evidence: vec![shared],
                },
            ],
            relations: Vec::new(),
        })
        .expect("shared evidence across facts");
    assert_eq!(resolved.grounded.evidence.len(), 2);
    assert_eq!(
        resolved.grounded.evidence[0].start_byte,
        resolved.grounded.evidence[1].start_byte
    );
}

#[test]
fn duplicate_fact_keys_are_rejected_before_lowering() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    let duplicate_entity = GroundedToolV4Input {
        entities: vec![
            GroundedEntityV4 {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
                evidence: vec![quote("Project Aurora", 0)],
            },
            GroundedEntityV4 {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
                evidence: vec![quote("Project Aurora", 0)],
            },
        ],
        relations: Vec::new(),
    };
    assert!(
        resolver
            .prepare_grounded_tool_v4(duplicate_entity)
            .expect_err("duplicate entity")
            .contains("duplicate entity key")
    );

    let duplicate_relation = GroundedToolV4Input {
        entities: vec![
            GroundedEntityV4 {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
                evidence: vec![quote("Project Aurora", 0)],
            },
            GroundedEntityV4 {
                key: "rust".to_string(),
                entity_type: "language".to_string(),
                label: "Rust".to_string(),
                evidence: vec![quote("Rust", 0)],
            },
        ],
        relations: vec![
            GroundedRelationV4 {
                key: "aurora-uses-rust".to_string(),
                from_entity_key: "aurora".to_string(),
                to_entity_key: "rust".to_string(),
                relation: "uses".to_string(),
                evidence: vec![quote("Project Aurora uses Rust", 0)],
            },
            GroundedRelationV4 {
                key: "aurora-uses-rust".to_string(),
                from_entity_key: "aurora".to_string(),
                to_entity_key: "rust".to_string(),
                relation: "uses".to_string(),
                evidence: vec![quote("Project Aurora uses Rust", 0)],
            },
        ],
    };
    assert!(
        resolver
            .prepare_grounded_tool_v4(duplicate_relation)
            .expect_err("duplicate relation")
            .contains("duplicate relation key")
    );
}

#[test]
fn selector_counts_are_bounded_before_resolution() {
    let source = "Project Aurora uses Rust.";
    let resolver = HostEvidenceResolverV1::new(source, &[]).expect("resolver");
    assert!(
        resolver
            .prepare_grounded_tool_v4(entity_input("Project Aurora", Vec::new()))
            .expect_err("missing selector")
            .contains("must contain 1..=")
    );
    assert!(
        resolver
            .prepare_grounded_tool_v4(entity_input(
                "Project Aurora",
                vec![quote("Project Aurora", 0); MAX_SPANS_PER_FACT + 1],
            ))
            .expect_err("per-fact selector limit")
            .contains("must contain 1..=")
    );
    assert!(
        resolver
            .prepare_grounded_tool_v4(entity_input(
                "Project Aurora",
                vec![quote("Project Aurora", 0); MAX_TOTAL_SPANS + 1],
            ))
            .expect_err("total selector limit")
            .contains("total evidence limit")
    );
}

#[test]
fn wrapper_prepares_the_same_host_owned_contract() {
    let source = "Project Aurora uses Rust.";
    let resolved = prepare_grounded_tool_v4(
        source,
        &[],
        entity_input("Project Aurora", vec![quote("Project Aurora", 0)]),
    )
    .expect("wrapper");
    resolved.resolution.validate().expect("receipt");
    assert_eq!(resolved.resolution.selector_count, 1);
}

#[test]
fn source_and_segment_limits_are_fail_closed() {
    let oversized_source = "x".repeat(MAX_SOURCE_BYTES + 1);
    assert!(HostEvidenceResolverV1::new(oversized_source.as_str(), &[]).is_err());
    assert!(HostEvidenceResolverV1::new("abc", &[SourceSegmentDraftV1::new(0, 0)]).is_err());
    let too_many = vec![SourceSegmentDraftV1::new(0, 1); MAX_SOURCE_SEGMENTS + 1];
    assert!(HostEvidenceResolverV1::new("abc", &too_many).is_err());
}

#[test]
fn source_segment_ranges_require_utf8_boundaries() {
    let source = "项目 Aurora";
    let error = HostEvidenceResolverV1::new(source, &[SourceSegmentDraftV1::new(1, 4)])
        .err()
        .expect("split UTF-8 must fail");
    assert!(error.contains("splits a UTF-8 character"));
}
