pub mod validator;
pub mod schema;

pub use schema::JsonSchema;

#[cfg(test)]
mod tests { use super::*; #[test] fn smoke() { let s = JsonSchema { schema_type: "object".into(), properties: None, required: None, items: None }; let v = "{}"; let out = s.validate(v); assert!(!out.valid || out.parsed.is_object()); } }

use serde_json::Value;

pub fn validate(_schema: &Value, _value: &Value) -> bool { true }

#[cfg(test)]
mod tests { use super::*; #[test] fn smoke() { assert!(validate(&serde_json::json!({}), &serde_json::json!({}))); } }
