use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct CritBitTreeMap {
    items: BTreeMap<String, Value>,
}

impl CritBitTreeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.items.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.items.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.items.remove(key).is_some()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.items.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.values()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.items.iter()
    }

    pub fn keys_with_prefix<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = &'a String> {
        self.items
            .range(prefix.to_string()..)
            .map(|(key, _)| key)
            .take_while(move |key| key.starts_with(prefix))
    }
}
