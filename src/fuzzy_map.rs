use indexmap::IndexMap;
use serde_json::Value;

use crate::bk_tree::levenshtein;

#[derive(Debug, Clone, Default)]
pub struct FuzzyMap {
    items: IndexMap<String, Value>,
}

impl FuzzyMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.items.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key)
    }

    pub fn search(&self, query: &str, radius: usize) -> Vec<(&String, &Value, usize)> {
        let mut results: Vec<_> = self
            .items
            .iter()
            .filter_map(|(key, value)| {
                let distance = levenshtein(key, query);
                (distance <= radius).then_some((key, value, distance))
            })
            .collect();
        results.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(b.0)));
        results
    }
}
