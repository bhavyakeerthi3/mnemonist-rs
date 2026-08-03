use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct BiMap {
    left: IndexMap<Value, Value>,
    right: IndexMap<Value, Value>,
}

impl BiMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
    }

    pub fn size(&self) -> usize {
        self.left.len()
    }

    pub fn set(&mut self, key: Value, value: Value) {
        if let Some(current_value) = self.left.get(&key) {
            if current_value == &value {
                return;
            }
            self.right.shift_remove(current_value);
        }

        if let Some(current_key) = self.right.get(&value) {
            if current_key == &key {
                return;
            }
            self.left.shift_remove(current_key);
        }

        self.left.insert(key.clone(), value.clone());
        self.right.insert(value, key);
    }

    pub fn delete(&mut self, key: &Value) -> bool {
        let Some(value) = self.left.shift_remove(key) else {
            return false;
        };
        self.right.shift_remove(&value);
        true
    }

    pub fn inverse_delete(&mut self, value: &Value) -> bool {
        let Some(key) = self.right.shift_remove(value) else {
            return false;
        };
        self.left.shift_remove(&key);
        true
    }

    pub fn has(&self, key: &Value) -> bool {
        self.left.contains_key(key)
    }

    pub fn inverse_has(&self, value: &Value) -> bool {
        self.right.contains_key(value)
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.left.get(key)
    }

    pub fn inverse_get(&self, value: &Value) -> Option<&Value> {
        self.right.get(value)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.left.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.left.values()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Value, &Value)> {
        self.left.iter()
    }

    pub fn inverse_keys(&self) -> impl Iterator<Item = &Value> {
        self.right.keys()
    }

    pub fn inverse_values(&self) -> impl Iterator<Item = &Value> {
        self.right.values()
    }

    pub fn inverse_entries(&self) -> impl Iterator<Item = (&Value, &Value)> {
        self.right.iter()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (Value, Value)>,
    {
        let mut map = Self::new();
        for (key, value) in iterable {
            map.set(key, value);
        }
        map
    }
}
