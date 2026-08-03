use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DefaultMap {
    items: IndexMap<Value, Value>,
    default: Value,
}

impl DefaultMap {
    pub fn new(default: Value) -> Self {
        Self {
            items: IndexMap::new(),
            default,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn has(&self, key: &Value) -> bool {
        self.items.contains_key(key)
    }

    pub fn get(&mut self, key: Value) -> &Value {
        self.items
            .entry(key)
            .or_insert_with(|| self.default.clone())
    }

    pub fn peek(&self, key: &Value) -> Option<&Value> {
        self.items.get(key)
    }

    pub fn set(&mut self, key: Value, value: Value) {
        self.items.insert(key, value);
    }

    pub fn delete(&mut self, key: &Value) -> bool {
        self.items.shift_remove(key).is_some()
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.items.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.values()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Value, &Value)> {
        self.items.iter()
    }
}
