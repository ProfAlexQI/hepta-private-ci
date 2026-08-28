use serde_json::json;

use super::*;

pub(super) fn grounded_tool_v4_schema_impl() -> serde_json::Value {
    let evidence_locator = || {
        json!({
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "quote": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": MAX_QUOTE_BYTES
                        },
                        "occurrence": {
                            "type": "integer",
                            "minimum": 0,
                            "maximum": MAX_QUOTE_OCCURRENCE
                        }
                    },
                    "required": ["quote", "occurrence"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "segment_id": {
                            "type": "string",
                            "minLength": 82,
                            "maxLength": 82,
                            "pattern": "^source-segment:v1:[0-9a-f]{64}$"
                        }
                    },
                    "required": ["segment_id"],
                    "additionalProperties": false
                }
            ]
        })
    };
    let evidence = || {
        json!({
            "type": "array",
            "minItems": 1,
            "maxItems": MAX_SPANS_PER_FACT,
            "items": evidence_locator()
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
                        "entity_type": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": MAX_TYPE_BYTES
                        },
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
                        "from_entity_key": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": MAX_KEY_BYTES
                        },
                        "to_entity_key": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": MAX_KEY_BYTES
                        },
                        "relation": {
                            "type": "string",
                            "minLength": 1,
                            "maxLength": MAX_RELATION_BYTES
                        },
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
