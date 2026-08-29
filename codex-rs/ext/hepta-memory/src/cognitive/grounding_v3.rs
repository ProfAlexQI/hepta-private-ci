//! P0.3 source-only tool input contract for durably grounded KG facts.
//!
//! This module validates the JSON shape that a future
//! `hepta_cognitive.*_grounded_v3` executor will consume. It is compiled and
//! tested, but deliberately not registered with `ToolContributor` until P0.1
//! and P0.2 receive executable qualification.

use codex_hepta_contracts::Sha256Digest;
use codex_hepta_memory::FactEvidenceSpanDraft;
use codex_hepta_memory::GroundedFactKind;
use codex_hepta_memory::GroundedKgFactSetDraft;
use codex_hepta_memory::KgEntityFactDraft;
use codex_hepta_memory::KgFactSetDraft;
use codex_hepta_memory::KgRelationFactDraft;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

pub(crate) const GROUNDED_TOOL_V3_SCHEMA_VERSION: u32 = 3;
pub(crate) const GROUNDED_TOOL_V3_REGISTERED: bool = false;
pub(crate) const GROUNDED_TOOL_V3_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const GROUNDED_TOOL_V3_EXTERNAL_EFFECTS: bool = false;
pub(crate) const GROUNDED_TOOL_V3_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const GROUNDED_TOOL_V3_PROMOTION: bool = false;

const MAX_ENTITIES: usize = 64;
const MAX_RELATIONS: usize = 128;
const MAX_SPANS_PER_FACT: usize = 4;
const MAX_TOTAL_SPANS: usize = 768;
const MAX_KEY_BYTES: usize = 256;
const MAX_TYPE_BYTES: usize = 128;
const MAX_LABEL_BYTES: usize = 1024;
const MAX_RELATION_BYTES: usize = 128;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedToolV3Input {
    #[serde(default)]
    pub(crate) entities: Vec<GroundedEntityV3>,
    #[serde(default)]
    pub(crate) relations: Vec<GroundedRelationV3>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedEntityV3 {
    pub(crate) key: String,
    pub(crate) entity_type: String,
    pub(crate) label: String,
    pub(crate) evidence: Vec<EvidenceSpanV3>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct GroundedRelationV3 {
    pub(crate) key: String,
    pub(crate) from_entity_key: String,
    pub(crate) to_entity_key: String,
    pub(crate) relation: String,
    pub(crate) evidence: Vec<EvidenceSpanV3>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceSpanV3 {
    pub(crate) start_byte: u32,
    pub(crate) end_byte: u32,
    pub(crate) sha256: String,
}

pub(crate) fn grounded_tool_v3_schema() -> Value {
    let evidence = || {
        json!({
            "type": "array",
            "minItems": 1,
            "maxItems": MAX_SPANS_PER_FACT,
            "items": {
                "type": "object",
                "properties": {
                    "start_byte": { "type": "integer", "minimum": 0 },
                    "end_byte": { "type": "integer", "minimum": 1 },
                    "sha256": {
                        "type": "string",
                        "minLength": 64,
                        "maxLength": 64,
                        "pattern": "^[0-9a-f]{64}$"
                    }
                },
                "required": ["start_byte", "end_byte", "sha256"],
                "additionalProperties": false
            }
        })
    };
    json!({
        "type": "object",
        "properties": {
            "entities": {
                "type": "array",
                "maxItems": MAX_ENTITIES,
                "items": {
                    "type": "object",
                    "properties": {
                        "key": { "type": "string", "minLength": 1, "maxLength": MAX_KEY_BYTES },
                        "entity_type": { "type": "string", "minLength": 1, "maxLength": MAX_TYPE_BYTES },
                        "label": { "type": "string", "minLength": 1, "maxLength": MAX_LABEL_BYTES },
                        "evidence": evidence()
                    },
                    "required": ["key", "entity_type", "label", "evidence"],
                    "additionalProperties": false
                }
            },
            "relations": {
                "type": "array",
                "maxItems": MAX_RELATIONS,
                "items": {
                    "type": "object",
                    "properties": {
                        "key": { "type": "string", "minLength": 1, "maxLength": MAX_KEY_BYTES },
                        "from_entity_key": { "type": "string", "minLength": 1, "maxLength": MAX_KEY_BYTES },
                        "to_entity_key": { "type": "string", "minLength": 1, "maxLength": MAX_KEY_BYTES },
                        "relation": { "type": "string", "minLength": 1, "maxLength": MAX_RELATION_BYTES },
                        "evidence": evidence()
                    },
                    "required": [
                        "key",
                        "from_entity_key",
                        "to_entity_key",
                        "relation",
                        "evidence"
                    ],
                    "additionalProperties": false
                }
            }
        },
        "required": ["entities", "relations"],
        "additionalProperties": false
    })
}

pub(crate) fn prepare_grounded_tool_v3(
    source_content: &str,
    input: GroundedToolV3Input,
) -> Result<GroundedKgFactSetDraft, String> {
    if input.entities.len() > MAX_ENTITIES || input.relations.len() > MAX_RELATIONS {
        return Err("grounded v3 input exceeds entity or relation limits".to_string());
    }
    let total_spans = input
        .entities
        .iter()
        .map(|entity| entity.evidence.len())
        .chain(
            input
                .relations
                .iter()
                .map(|relation| relation.evidence.len()),
        )
        .try_fold(0usize, |total, count| total.checked_add(count))
        .ok_or_else(|| "grounded v3 evidence count overflow".to_string())?;
    if total_spans > MAX_TOTAL_SPANS {
        return Err("grounded v3 input exceeds total evidence limit".to_string());
    }

    let mut entities = Vec::with_capacity(input.entities.len());
    let mut relations = Vec::with_capacity(input.relations.len());
    let mut evidence = Vec::with_capacity(total_spans);
    for entity in input.entities {
        validate_text(&entity.key, MAX_KEY_BYTES, "entity key")?;
        validate_text(&entity.entity_type, MAX_TYPE_BYTES, "entity type")?;
        validate_text(&entity.label, MAX_LABEL_BYTES, "entity label")?;
        append_evidence(
            source_content,
            GroundedFactKind::Entity,
            entity.key.as_str(),
            entity.evidence,
            &mut evidence,
        )?;
        entities.push(KgEntityFactDraft {
            key: entity.key,
            entity_type: entity.entity_type,
            label: entity.label,
        });
    }
    for relation in input.relations {
        validate_text(&relation.key, MAX_KEY_BYTES, "relation key")?;
        validate_text(
            &relation.from_entity_key,
            MAX_KEY_BYTES,
            "relation source key",
        )?;
        validate_text(
            &relation.to_entity_key,
            MAX_KEY_BYTES,
            "relation target key",
        )?;
        validate_text(&relation.relation, MAX_RELATION_BYTES, "relation predicate")?;
        append_evidence(
            source_content,
            GroundedFactKind::Relation,
            relation.key.as_str(),
            relation.evidence,
            &mut evidence,
        )?;
        relations.push(KgRelationFactDraft {
            key: relation.key,
            from_entity_key: relation.from_entity_key,
            to_entity_key: relation.to_entity_key,
            relation: relation.relation,
        });
    }
    if entities.is_empty() && relations.is_empty() {
        return Err("grounded v3 input must contain at least one fact".to_string());
    }
    Ok(GroundedKgFactSetDraft {
        facts: KgFactSetDraft {
            entities,
            relations,
        },
        evidence,
    })
}

fn append_evidence(
    source_content: &str,
    kind: GroundedFactKind,
    fact_key: &str,
    spans: Vec<EvidenceSpanV3>,
    output: &mut Vec<FactEvidenceSpanDraft>,
) -> Result<(), String> {
    if spans.is_empty() || spans.len() > MAX_SPANS_PER_FACT {
        return Err(format!(
            "{} fact `{fact_key}` must contain 1..={MAX_SPANS_PER_FACT} evidence spans",
            kind.as_str()
        ));
    }
    for span in spans {
        let start = usize::try_from(span.start_byte)
            .map_err(|_| "evidence start byte exceeds usize".to_string())?;
        let end = usize::try_from(span.end_byte)
            .map_err(|_| "evidence end byte exceeds usize".to_string())?;
        if start >= end || end > source_content.len() {
            return Err(format!(
                "{} fact `{fact_key}` evidence range is outside source content",
                kind.as_str()
            ));
        }
        if !source_content.is_char_boundary(start) || !source_content.is_char_boundary(end) {
            return Err(format!(
                "{} fact `{fact_key}` evidence range splits a UTF-8 character",
                kind.as_str()
            ));
        }
        let digest = Sha256Digest::parse(span.sha256)
            .map_err(|error| format!("invalid evidence SHA-256: {error}"))?;
        let actual = Sha256Digest::for_bytes(&source_content.as_bytes()[start..end]);
        if digest != actual {
            return Err(format!(
                "{} fact `{fact_key}` evidence digest does not match source bytes",
                kind.as_str()
            ));
        }
        output.push(FactEvidenceSpanDraft::new(
            kind,
            fact_key,
            span.start_byte,
            span.end_byte,
            digest,
        ));
    }
    Ok(())
}

fn validate_text(value: &str, maximum: usize, label: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > maximum || value.as_bytes().contains(&0) {
        return Err(format!("{label} must contain 1..={maximum} non-NUL bytes"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(source: &str, needle: &str) -> EvidenceSpanV3 {
        let start = source.find(needle).expect("needle");
        let end = start + needle.len();
        EvidenceSpanV3 {
            start_byte: u32::try_from(start).expect("start"),
            end_byte: u32::try_from(end).expect("end"),
            sha256: Sha256Digest::for_bytes(&source.as_bytes()[start..end])
                .as_str()
                .to_string(),
        }
    }

    #[test]
    fn schema_requires_evidence_for_each_fact() {
        let schema = grounded_tool_v3_schema();
        assert_eq!(schema["required"], json!(["entities", "relations"]));
        assert_eq!(
            schema["properties"]["entities"]["items"]["required"],
            json!(["key", "entity_type", "label", "evidence"])
        );
        assert_eq!(
            schema["properties"]["relations"]["items"]["required"],
            json!([
                "key",
                "from_entity_key",
                "to_entity_key",
                "relation",
                "evidence"
            ])
        );
        const { assert!(!GROUNDED_TOOL_V3_REGISTERED); }
        const { assert!(!GROUNDED_TOOL_V3_PRODUCTION_AUTHORITY); }
    }

    #[test]
    fn valid_v3_input_produces_grounded_fact_set() {
        let source = "Project Aurora uses Rust.";
        let input = GroundedToolV3Input {
            entities: vec![
                GroundedEntityV3 {
                    key: "aurora".to_string(),
                    entity_type: "project".to_string(),
                    label: "Project Aurora".to_string(),
                    evidence: vec![span(source, "Project Aurora uses Rust")],
                },
                GroundedEntityV3 {
                    key: "rust".to_string(),
                    entity_type: "language".to_string(),
                    label: "Rust".to_string(),
                    evidence: vec![span(source, "Project Aurora uses Rust")],
                },
            ],
            relations: vec![GroundedRelationV3 {
                key: "aurora-uses-rust".to_string(),
                from_entity_key: "aurora".to_string(),
                to_entity_key: "rust".to_string(),
                relation: "uses".to_string(),
                evidence: vec![span(source, "Project Aurora uses Rust")],
            }],
        };
        let grounded = prepare_grounded_tool_v3(source, input).expect("grounded");
        assert_eq!(grounded.facts.entities.len(), 2);
        assert_eq!(grounded.facts.relations.len(), 1);
        assert_eq!(grounded.evidence.len(), 3);
    }

    #[test]
    fn v3_rejects_missing_or_drifted_evidence() {
        let source = "Project Aurora uses Rust.";
        let missing = GroundedToolV3Input {
            entities: vec![GroundedEntityV3 {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
                evidence: Vec::new(),
            }],
            relations: Vec::new(),
        };
        assert!(prepare_grounded_tool_v3(source, missing).is_err());

        let mut drifted = span(source, "Project Aurora");
        drifted.sha256 = Sha256Digest::for_bytes(b"wrong").as_str().to_string();
        let drifted = GroundedToolV3Input {
            entities: vec![GroundedEntityV3 {
                key: "aurora".to_string(),
                entity_type: "project".to_string(),
                label: "Project Aurora".to_string(),
                evidence: vec![drifted],
            }],
            relations: Vec::new(),
        };
        assert!(prepare_grounded_tool_v3(source, drifted).is_err());
    }
}
