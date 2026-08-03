use serde_json::Value;

use crate::compare::default_compare;

pub fn quick_sort(values: &mut [Value]) {
    values.sort_by(default_compare);
}

pub fn insertion_sort(values: &mut [Value]) {
    values.sort_by(default_compare);
}

pub fn sorted(mut values: Vec<Value>) -> Vec<Value> {
    values.sort_by(default_compare);
    values
}
