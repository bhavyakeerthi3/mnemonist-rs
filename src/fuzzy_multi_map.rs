use indexmap::IndexMap;
use serde_json::Value;

use crate::bk_tree::levenshtein;

#[derive(Debug, Clone, Default)]
pub struct FuzzyMultiMap {
    items: IndexMap<String, Vec<Value>>,
}

impl FuzzyMultiMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.items.values().map(Vec::len).sum()
    }

    pub fn dimension(&self) -> usize {
        self.items.len()
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.items.entry(key.into()).or_default().push(value);
    }

    pub fn get(&self, key: &str) -> Option<&[Value]> {
        self.items.get(key).map(Vec::as_slice)
    }

    pub fn search(&self, query: &str, radius: usize) -> Vec<(&String, &[Value], usize)> {
        let mut results: Vec<_> = self
            .items
            .iter()
            .filter_map(|(key, values)| {
                let distance = levenshtein(key, query);
                (distance <= radius).then_some((key, values.as_slice(), distance))
            })
            .collect();
        results.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(b.0)));
        results
    }
}
