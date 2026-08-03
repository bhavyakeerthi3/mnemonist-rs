use serde_json::Value;

#[derive(Debug, Clone)]
pub struct HashedArrayTree {
    block_size: usize,
    capacity: usize,
    items: Vec<Value>,
}

impl HashedArrayTree {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size: block_size.max(1),
            capacity: 0,
            items: Vec::new(),
        }
    }

    pub fn with_dimensions(block_size: usize, initial_capacity: usize, initial_length: usize) -> Self {
        let mut tree = Self::new(block_size);
        tree.grow(initial_capacity.max(initial_length));
        tree.items.resize(initial_length, Value::Null);
        tree
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn length(&self) -> usize {
        self.size()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push(&mut self, value: Value) -> usize {
        if self.items.len() == self.capacity {
            self.grow(self.capacity + self.block_size);
        }
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

    pub fn grow(&mut self, capacity: usize) {
        if capacity <= self.capacity {
            return;
        }
        self.capacity = capacity.div_ceil(self.block_size) * self.block_size;
        self.items.reserve(self.capacity.saturating_sub(self.items.len()));
    }

    pub fn resize(&mut self, size: usize, value: Value) {
        if size > self.capacity {
            self.grow(size);
        }
        self.items.resize(size, value);
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.iter()
    }

    pub fn to_array(&self) -> Vec<Value> {
        self.items.clone()
    }
}
