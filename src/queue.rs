use serde_json::Value;

/// FIFO queue mirroring mnemonist's offset-compacting `Queue` semantics.
#[derive(Debug, Clone, Default)]
pub struct Queue {
    items: Vec<Value>,
    offset: usize,
    size: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.offset = 0;
        self.size = 0;
    }

    pub fn enqueue(&mut self, item: Value) -> usize {
        self.items.push(item);
        self.size += 1;
        self.size
    }

    pub fn dequeue(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }

        let item = self.items.get(self.offset).cloned()?;

        self.offset += 1;
        if self.offset * 2 >= self.items.len() {
            self.items = self.items[self.offset..].to_vec();
            self.offset = 0;
        }

        self.size -= 1;
        Some(item)
    }

    pub fn peek(&self) -> Option<&Value> {
        if self.size == 0 {
            return None;
        }
        self.items.get(self.offset)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, usize),
    {
        for (j, item) in self.items[self.offset..].iter().enumerate() {
            callback(item, j);
        }
    }

    pub fn to_array(&self) -> Vec<Value> {
        self.items[self.offset..].to_vec()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items[self.offset..].iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.items[self.offset..].iter().enumerate()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let mut queue = Self::new();
        for value in iterable {
            queue.enqueue(value);
        }
        queue
    }

    pub fn of(items: impl IntoIterator<Item = Value>) -> Self {
        Self::from_iter(items)
    }
}
