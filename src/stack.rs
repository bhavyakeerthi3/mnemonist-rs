use serde_json::Value;

/// LIFO stack mirroring mnemonist's `Stack` API semantics.
#[derive(Debug, Clone, Default)]
pub struct Stack {
    items: Vec<Value>,
    size: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn push(&mut self, item: Value) -> usize {
        self.items.push(item);
        self.size += 1;
        self.size
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&Value> {
        if self.size == 0 {
            return None;
        }
        self.items.get(self.size - 1)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, usize),
    {
        for (i, item) in self.items.iter().rev().enumerate() {
            callback(item, i);
        }
    }

    pub fn to_array(&self) -> Vec<Value> {
        self.items.iter().rev().cloned().collect()
    }

    pub fn values(&self) -> impl DoubleEndedIterator<Item = &Value> {
        self.items.iter().rev()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.items.iter().rev().enumerate()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let mut stack = Self::new();
        for value in iterable {
            stack.push(value);
        }
        stack
    }

    pub fn of(items: impl IntoIterator<Item = Value>) -> Self {
        Self::from_iter(items)
    }
}
