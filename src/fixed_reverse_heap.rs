use std::cmp::Ordering;

use serde_json::Value;

use crate::compare::default_compare;

#[derive(Debug, Clone)]
pub struct FixedReverseHeap {
    capacity: usize,
    items: Vec<Value>,
}

impl FixedReverseHeap {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push(&mut self, item: Value) -> usize {
        if self.capacity == 0 {
            return 0;
        }

        self.items.push(item);
        self.items.sort_by(default_compare);
        if self.items.len() > self.capacity {
            self.items.pop();
        }
        self.items.len()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.items.first()
    }

    pub fn consume(&mut self) -> Vec<Value> {
        let mut items = std::mem::take(&mut self.items);
        items.sort_by(|a, b| default_compare(a, b));
        items
    }

    pub fn to_array(&self) -> Vec<Value> {
        let mut items = self.items.clone();
        items.sort_by(|a, b| match default_compare(a, b) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        });
        items
    }
}
