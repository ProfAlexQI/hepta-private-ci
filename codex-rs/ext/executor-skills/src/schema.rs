use schemars::JsonSchema;
use schemars::r#gen::SchemaSettings;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

pub(crate) fn input_schema_for<T: JsonSchema>() -> Value {
    schema_for::<T>(false)
}

pub(crate) fn output_schema_for<T: JsonSchema>() -> Value {
    schema_for::<T>(true)
}

fn schema_for<T: JsonSchema>(option_add_null_type: bool) -> Value {
    let schema = SchemaSettings::draft2019_09()
        .with(|settings| {
            settings.inline_subschemas = true;
            settings.option_add_null_type = option_add_null_type;
        })
        .into_generator()
        .into_root_schema_for::<T>();
    let Value::Object(mut object) = serde_json::to_value(schema).unwrap_or_else(|_| {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false,
        })
    }) else {
        return json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false,
        });
    };
    let mut result = Map::new();
    for key in [
        "properties",
        "required",
        "type",
        "additionalProperties",
        "$defs",
        "definitions",
    ] {
        if let Some(value) = object.remove(key) {
            result.insert(key.to_string(), value);
        }
    }
    Value::Object(result)
}
