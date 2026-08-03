use std::collections::VecDeque;

use serde_json::Value;

/// Singly-linked-list semantics using `VecDeque` for safe O(1) ends.
#[derive(Debug, Clone, Default)]
pub struct LinkedList {
    items: VecDeque<Value>,
    size: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn first(&self) -> Option<&Value> {
        self.items.front()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.first()
    }

    pub fn last(&self) -> Option<&Value> {
        self.items.back()
    }

    pub fn push(&mut self, item: Value) -> usize {
        self.items.push_back(item);
        self.size += 1;
        self.size
    }

    pub fn unshift(&mut self, item: Value) -> usize {
        self.items.push_front(item);
        self.size += 1;
        self.size
    }

    pub fn shift(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.items.pop_front()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, usize),
    {
        for (i, item) in self.items.iter().enumerate() {
            callback(item, i);
        }
    }

    pub fn to_array(&self) -> Vec<Value> {
        self.items.iter().cloned().collect()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.items.iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.items.iter().enumerate()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let mut list = Self::new();
        for value in iterable {
            list.push(value);
        }
        list
    }
}
