use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiMapContainer {
    Vec,
    Set,
}

#[derive(Debug, Clone)]
pub struct MultiMap {
    items: IndexMap<Value, Vec<Value>>,
    size: usize,
    container: MultiMapContainer,
}

impl Default for MultiMap {
    fn default() -> Self {
        Self::new(MultiMapContainer::Vec)
    }
}

impl MultiMap {
    pub fn new(container: MultiMapContainer) -> Self {
        Self {
            items: IndexMap::new(),
            size: 0,
            container,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dimension(&self) -> usize {
        self.items.len()
    }

    pub fn set(&mut self, key: Value, value: Value) {
        let values = self.items.entry(key).or_default();
        if self.container == MultiMapContainer::Set && values.contains(&value) {
            return;
        }
        values.push(value);
        self.size += 1;
    }

    pub fn has(&self, key: &Value) -> bool {
        self.items.contains_key(key)
    }

    pub fn contains(&self, key: &Value, value: &Value) -> bool {
        self.items
            .get(key)
            .map(|values| values.contains(value))
            .unwrap_or(false)
    }

    pub fn get(&self, key: &Value) -> Option<&[Value]> {
        self.items.get(key).map(Vec::as_slice)
    }

    pub fn delete(&mut self, key: &Value) -> bool {
        let Some(values) = self.items.shift_remove(key) else {
            return false;
        };
        self.size -= values.len();
        true
    }

    pub fn remove(&mut self, key: &Value, value: &Value) -> bool {
        let Some(values) = self.items.get_mut(key) else {
            return false;
        };
        let Some(index) = values.iter().position(|item| item == value) else {
            return false;
        };
        values.remove(index);
        self.size -= 1;
        if values.is_empty() {
            self.items.shift_remove(key);
        }
        true
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.items.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.values().flat_map(|values| values.iter())
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Value, &Value)> {
        self.items
            .iter()
            .flat_map(|(key, values)| values.iter().map(move |value| (key, value)))
    }

    pub fn containers(&self) -> impl Iterator<Item = (&Value, &[Value])> {
        self.items
            .iter()
            .map(|(key, values)| (key, values.as_slice()))
    }

    pub fn multiplicity(&self, key: &Value) -> usize {
        self.items.get(key).map(|v| v.len()).unwrap_or(0)
    }

    pub fn associations(&self) -> impl Iterator<Item = (&Value, &[Value])> {
        self.containers()
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, &Value),
    {
        for (key, values) in &self.items {
            for value in values {
                callback(value, key);
            }
        }
    }

    pub fn for_each_association<F>(&self, mut callback: F)
    where
        F: FnMut(&[Value], &Value),
    {
        for (key, values) in &self.items {
            callback(values.as_slice(), key);
        }
    }

    pub fn from<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (Value, Value)>,
    {
        let mut map = Self::new(MultiMapContainer::Vec);
        for (k, v) in iterable {
            map.set(k, v);
        }
        map
    }
}
