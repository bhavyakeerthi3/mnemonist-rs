use std::cmp::Ordering;

use serde_json::Value;

use crate::compare::{default_compare, reverse_compare};

type Compare = fn(&Value, &Value) -> Ordering;

#[derive(Debug, Clone)]
struct Node {
    item: Value,
    degree: usize,
    parent: Option<usize>,
    child: Option<usize>,
    left: usize,
    right: usize,
}

/// A safe, arena-backed Fibonacci heap. Nodes form circular root and child
/// lists through indices, avoiding raw pointers while retaining consolidation.
#[derive(Debug, Clone)]
pub struct FibonacciHeap {
    nodes: Vec<Node>,
    root: Option<usize>,
    best: Option<usize>,
    size: usize,
    compare: Compare,
}

impl FibonacciHeap {
    pub fn new_min() -> Self {
        Self::new(default_compare)
    }

    pub fn new_max() -> Self {
        Self::new(reverse_compare)
    }

    pub fn new(compare: Compare) -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            best: None,
            size: 0,
            compare,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = None;
        self.best = None;
        self.size = 0;
    }

    pub fn push(&mut self, item: Value) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            item,
            degree: 0,
            parent: None,
            child: None,
            left: index,
            right: index,
        });
        self.add_root(index);
        self.size += 1;
        self.size
    }

    pub fn peek(&self) -> Option<&Value> {
        self.best.map(|index| &self.nodes[index].item)
    }

    pub fn pop(&mut self) -> Option<Value> {
        let minimum = self.best?;
        let item = self.nodes[minimum].item.clone();

        if let Some(child) = self.nodes[minimum].child {
            let children = self.list(child);
            self.nodes[minimum].child = None;
            self.nodes[minimum].degree = 0;
            for child in children {
                self.nodes[child].parent = None;
                self.isolate(child);
                self.add_root(child);
            }
        }

        self.remove_root(minimum);
        self.size -= 1;
        if self.root.is_some() {
            self.consolidate();
        } else {
            self.best = None;
        }
        Some(item)
    }

    pub fn from_iter<I>(iterable: I, max: bool) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let mut heap = if max {
            Self::new_max()
        } else {
            Self::new_min()
        };
        for item in iterable {
            heap.push(item);
        }
        heap
    }

    pub fn to_array(&self) -> Vec<Value> {
        let mut copy = self.clone();
        let mut items = Vec::with_capacity(copy.size);
        while let Some(item) = copy.pop() {
            items.push(item);
        }
        items
    }

    pub fn consume(&mut self) -> Vec<Value> {
        let mut items = Vec::with_capacity(self.size);
        while let Some(item) = self.pop() {
            items.push(item);
        }
        items
    }

    fn add_root(&mut self, node: usize) {
        self.nodes[node].parent = None;
        match self.root {
            None => {
                self.root = Some(node);
                self.best = Some(node);
            }
            Some(root) => {
                self.insert_after(root, node);
                if self.better(node, self.best.expect("a root list has a best node")) {
                    self.best = Some(node);
                }
            }
        }
    }

    fn consolidate(&mut self) {
        let roots = self.list(self.root.expect("consolidation requires a root list"));
        let mut by_degree: Vec<Option<usize>> = vec![None; self.size.ilog2() as usize + 3];

        for mut node in roots {
            if self.nodes[node].parent.is_some() {
                continue;
            }
            let mut degree = self.nodes[node].degree;
            loop {
                if degree >= by_degree.len() {
                    by_degree.resize(degree + 1, None);
                }
                let Some(other) = by_degree[degree].take() else {
                    by_degree[degree] = Some(node);
                    break;
                };
                if self.better(other, node) {
                    self.link(node, other);
                    node = other;
                } else {
                    self.link(other, node);
                }
                degree = self.nodes[node].degree;
            }
        }

        self.root = None;
        self.best = None;
        for node in by_degree.into_iter().flatten() {
            self.isolate(node);
            self.nodes[node].parent = None;
            self.add_root(node);
        }
    }

    fn link(&mut self, child: usize, parent: usize) {
        self.remove_root(child);
        self.isolate(child);
        self.nodes[child].parent = Some(parent);
        match self.nodes[parent].child {
            None => self.nodes[parent].child = Some(child),
            Some(first_child) => self.insert_after(first_child, child),
        }
        self.nodes[parent].degree += 1;
    }

    fn remove_root(&mut self, node: usize) {
        if self.nodes[node].right == node {
            self.root = None;
        } else {
            if self.root == Some(node) {
                self.root = Some(self.nodes[node].right);
            }
            self.unlink(node);
        }
        self.isolate(node);
    }

    fn list(&self, first: usize) -> Vec<usize> {
        let mut nodes = vec![first];
        let mut current = self.nodes[first].right;
        while current != first {
            nodes.push(current);
            current = self.nodes[current].right;
        }
        nodes
    }

    fn isolate(&mut self, node: usize) {
        self.nodes[node].left = node;
        self.nodes[node].right = node;
    }

    fn insert_after(&mut self, anchor: usize, node: usize) {
        let right = self.nodes[anchor].right;
        self.nodes[node].left = anchor;
        self.nodes[node].right = right;
        self.nodes[anchor].right = node;
        self.nodes[right].left = node;
    }

    fn unlink(&mut self, node: usize) {
        let left = self.nodes[node].left;
        let right = self.nodes[node].right;
        self.nodes[left].right = right;
        self.nodes[right].left = left;
    }

    fn better(&self, left: usize, right: usize) -> bool {
        (self.compare)(&self.nodes[left].item, &self.nodes[right].item) == Ordering::Less
    }
}

impl Default for FibonacciHeap {
    fn default() -> Self {
        Self::new_min()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::FibonacciHeap;

    #[test]
    fn consolidates_roots_and_keeps_minimum_order() {
        let mut heap = FibonacciHeap::new_min();
        for value in [7, 3, 18, 39, 10, 1, 12, 4, 2] {
            heap.push(json!(value));
        }

        assert_eq!(heap.pop(), Some(json!(1)));
        assert!(heap.nodes.iter().any(|node| node.degree > 0));
        assert_eq!(
            heap.consume(),
            vec![
                json!(2),
                json!(3),
                json!(4),
                json!(7),
                json!(10),
                json!(12),
                json!(18),
                json!(39)
            ]
        );
    }

    #[test]
    fn max_heap_reverses_priority_without_changing_structure() {
        let mut heap = FibonacciHeap::new_max();
        for value in [2, 5, 1, 4] {
            heap.push(json!(value));
        }

        assert_eq!(heap.consume(), vec![json!(5), json!(4), json!(2), json!(1)]);
    }
}
