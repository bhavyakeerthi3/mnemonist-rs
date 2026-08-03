use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct LruSetPop {
    pub evicted: bool,
    pub key: Value,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct LruCache {
    capacity: usize,
    entries: Vec<(Value, Value)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LruCacheError(pub String);

impl std::fmt::Display for LruCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LruCacheError {}

impl LruCache {
    pub fn new(capacity: usize) -> Result<Self, LruCacheError> {
        if capacity == 0 {
            return Err(LruCacheError(
                "mnemonist/lru-cache: capacity should be positive number.".into(),
            ));
        }

        Ok(Self {
            capacity,
            entries: Vec::with_capacity(capacity),
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn has(&self, key: &Value) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    pub fn set(&mut self, key: Value, value: Value) {
        if let Some(index) = self.position(&key) {
            self.entries.remove(index);
        } else if self.entries.len() == self.capacity {
            self.entries.pop();
        }

        self.entries.insert(0, (key, value));
    }

    pub fn setpop(&mut self, key: Value, value: Value) -> Option<LruSetPop> {
        if let Some(index) = self.position(&key) {
            let (old_key, old_value) = self.entries.remove(index);
            self.entries.insert(0, (key.clone(), value));
            return Some(LruSetPop {
                evicted: false,
                key: old_key,
                value: old_value,
            });
        }

        let evicted = if self.entries.len() == self.capacity {
            self.entries.pop().map(|(old_key, old_value)| LruSetPop {
                evicted: true,
                key: old_key,
                value: old_value,
            })
        } else {
            None
        };

        self.entries.insert(0, (key, value));
        evicted
    }

    pub fn get(&mut self, key: &Value) -> Option<Value> {
        let index = self.position(key)?;
        let (key, value) = self.entries.remove(index);
        let result = value.clone();
        self.entries.insert(0, (key, value));
        Some(result)
    }

    pub fn peek(&self, key: &Value) -> Option<&Value> {
        self.entries
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn delete(&mut self, key: &Value) -> bool {
        let Some(index) = self.position(key) else {
            return false;
        };
        self.entries.remove(index);
        true
    }

    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        let index = self.position(key)?;
        Some(self.entries.remove(index).1)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.entries.iter().map(|(key, _)| key)
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.entries.iter().map(|(_, value)| value)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Value, &Value)> {
        self.entries.iter().map(|(key, value)| (key, value))
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, &Value),
    {
        for (key, value) in &self.entries {
            callback(value, key);
        }
    }

    pub fn from_iter<I>(iterable: I, capacity: Option<usize>) -> Result<Self, LruCacheError>
    where
        I: IntoIterator<Item = (Value, Value)>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        let cap = capacity.unwrap_or_else(|| iter.len());
        let mut cache = Self::new(cap)?;

        for (key, value) in iter {
            cache.set(key, value);
        }

        Ok(cache)
    }

    fn position(&self, key: &Value) -> Option<usize> {
        self.entries.iter().position(|(k, _)| k == key)
    }
}
