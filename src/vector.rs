use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct Vector {
    items: Vec<Value>,
}

impl Vector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn length(&self) -> usize {
        self.size()
    }

    pub fn with_initial_length(length: usize, value: Value) -> Self {
        Self {
            items: vec![value; length],
        }
    }

    pub fn reallocate(&mut self, new_capacity: usize) {
        let current_len = self.items.len();
        if new_capacity < current_len {
            self.items.truncate(new_capacity);
        }
        self.items.shrink_to_fit();
        if new_capacity > self.items.capacity() {
            let additional = new_capacity - self.items.len();
            self.items.reserve_exact(additional);
        }
    }

    pub fn grow(&mut self, desired_capacity: usize) {
        if desired_capacity > self.items.capacity() {
            let additional = desired_capacity - self.items.len();
            self.items.reserve_exact(additional);
        }
    }

    pub fn resize(&mut self, new_length: usize, value: Value) {
        self.items.resize(new_length, value);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push(&mut self, value: Value) -> usize {
        self.items.push(value);
        self.items.len()
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.items.pop()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.items.get(index)
    }

    pub fn set(&mut self, index: usize, value: Value) -> bool {
        if index >= self.items.len() {
            return false;
        }
        self.items[index] = value;
        true
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.items.iter().enumerate()
    }

    pub fn to_array(&self) -> Vec<Value> {
        self.items.clone()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        Self {
            items: iterable.into_iter().collect(),
        }
    }
}
