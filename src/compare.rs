use std::cmp::Ordering;

use serde_json::Value;

/// JavaScript-like `<` / `>` comparison for JSON values used in heaps.
pub fn default_compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Null, Value::Null) => Ordering::Equal,
        (Value::Null, _) => Ordering::Less,
        (_, Value::Null) => Ordering::Greater,
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        (Value::Number(x), Value::Number(y)) => {
            let xf = x.as_f64().unwrap_or(f64::NAN);
            let yf = y.as_f64().unwrap_or(f64::NAN);
            xf.partial_cmp(&yf).unwrap_or(Ordering::Equal)
        }
        (Value::String(x), Value::String(y)) => x.cmp(y),
        (Value::Array(x), Value::Array(y)) => {
            let xs = serde_json::to_string(x).unwrap_or_default();
            let ys = serde_json::to_string(y).unwrap_or_default();
            xs.cmp(&ys)
        }
        (Value::Object(x), Value::Object(y)) => {
            let xs = serde_json::to_string(x).unwrap_or_default();
            let ys = serde_json::to_string(y).unwrap_or_default();
            xs.cmp(&ys)
        }
        (Value::Bool(_), _) => Ordering::Less,
        (Value::Number(_), Value::Bool(_)) => Ordering::Greater,
        (Value::Number(_), _) => Ordering::Less,
        (Value::String(_), Value::Bool(_) | Value::Number(_)) => Ordering::Greater,
        (Value::String(_), _) => Ordering::Less,
        (Value::Array(_), _) => Ordering::Less,
        (Value::Object(_), _) => Ordering::Less,
    }
}

pub fn reverse_compare(a: &Value, b: &Value) -> Ordering {
    default_compare(b, a)
}
