use std::cmp::Ordering;

use serde_json::Value;

use crate::compare::{default_compare, reverse_compare};

type CompareFn = fn(&Value, &Value) -> Ordering;

fn sift_down(compare: CompareFn, heap: &mut [Value], start_index: usize, mut i: usize) {
    let item = heap[i].clone();
    while i > start_index {
        let parent_index = (i - 1) >> 1;
        let parent = &heap[parent_index];
        if compare(&item, parent) == Ordering::Less {
            heap[i] = parent.clone();
            i = parent_index;
            continue;
        }
        break;
    }
    heap[i] = item;
}

fn sift_up(compare: CompareFn, heap: &mut [Value], mut i: usize) {
    let end_index = heap.len();
    let start_index = i;
    let item = heap[i].clone();
    let mut child_index = 2 * i + 1;

    while child_index < end_index {
        let right_index = child_index + 1;
        if right_index < end_index
            && compare(&heap[child_index], &heap[right_index]) != Ordering::Less
        {
            child_index = right_index;
        }
        heap[i] = heap[child_index].clone();
        i = child_index;
        child_index = 2 * i + 1;
    }

    heap[i] = item;
    sift_down(compare, heap, start_index, i);
}

fn heap_push(compare: CompareFn, heap: &mut Vec<Value>, item: Value) {
    heap.push(item);
    let last = heap.len() - 1;
    sift_down(compare, heap, 0, last);
}

fn heap_pop(compare: CompareFn, heap: &mut Vec<Value>) -> Option<Value> {
    let last_item = heap.pop()?;
    if heap.is_empty() {
        return Some(last_item);
    }
    let item = heap[0].clone();
    heap[0] = last_item;
    sift_up(compare, heap, 0);
    Some(item)
}

fn heap_replace(compare: CompareFn, heap: &mut Vec<Value>, item: Value) -> Result<Value, String> {
    if heap.is_empty() {
        return Err("mnemonist/heap.replace: cannot pop an empty heap.".into());
    }
    let popped = heap[0].clone();
    heap[0] = item;
    sift_up(compare, heap, 0);
    Ok(popped)
}

fn heap_pushpop(compare: CompareFn, heap: &mut [Value], mut item: Value) -> Value {
    if !heap.is_empty() && compare(&heap[0], &item) == Ordering::Less {
        std::mem::swap(&mut heap[0], &mut item);
        sift_up(compare, heap, 0);
    }
    item
}

pub fn heapify(compare: CompareFn, array: &mut [Value]) {
    let n = array.len();
    if n == 0 {
        return;
    }
    let mut i = n >> 1;
    while i > 0 {
        i -= 1;
        sift_up(compare, array, i);
    }
}

pub fn consume(compare: CompareFn, heap: &mut Vec<Value>) -> Vec<Value> {
    let l = heap.len();
    let mut array = Vec::with_capacity(l);
    for _ in 0..l {
        if let Some(v) = heap_pop(compare, heap) {
            array.push(v);
        }
    }
    array
}

#[derive(Debug, Clone)]
pub struct Heap {
    items: Vec<Value>,
    size: usize,
    compare: CompareFn,
}

impl Heap {
    pub fn new_min() -> Self {
        Self {
            items: Vec::new(),
            size: 0,
            compare: default_compare,
        }
    }

    pub fn new_max() -> Self {
        Self {
            items: Vec::new(),
            size: 0,
            compare: reverse_compare,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn push(&mut self, item: Value) -> usize {
        heap_push(self.compare, &mut self.items, item);
        self.size += 1;
        self.size
    }

    pub fn peek(&self) -> Option<&Value> {
        self.items.first()
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.size > 0 {
            self.size -= 1;
        }
        heap_pop(self.compare, &mut self.items)
    }

    pub fn replace(&mut self, item: Value) -> Result<Value, String> {
        let popped = heap_replace(self.compare, &mut self.items, item)?;
        Ok(popped)
    }

    pub fn pushpop(&mut self, item: Value) -> Value {
        heap_pushpop(self.compare, &mut self.items, item)
    }

    pub fn consume(&mut self) -> Vec<Value> {
        self.size = 0;
        consume(self.compare, &mut self.items)
    }

    pub fn to_array(&self) -> Vec<Value> {
        consume(self.compare, &mut self.items.clone())
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn from_iter<I>(iterable: I, max: bool) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let compare = if max {
            reverse_compare
        } else {
            default_compare
        };
        let mut items: Vec<Value> = iterable.into_iter().collect();
        heapify(compare, &mut items);
        let size = items.len();
        Self {
            items,
            size,
            compare,
        }
    }

    pub fn nsmallest(n: usize, iterable: impl IntoIterator<Item = Value>) -> Vec<Value> {
        nsmallest(default_compare, n, iterable)
    }

    pub fn nlargest(n: usize, iterable: impl IntoIterator<Item = Value>) -> Vec<Value> {
        nlargest(default_compare, n, iterable)
    }
}

fn nsmallest(
    compare: CompareFn,
    n: usize,
    iterable: impl IntoIterator<Item = Value>,
) -> Vec<Value> {
    let items: Vec<Value> = iterable.into_iter().collect();
    if n == 0 || items.is_empty() {
        return Vec::new();
    }
    if n == 1 {
        return vec![items.iter().cloned().min_by(|a, b| compare(a, b)).unwrap()];
    }
    if n >= items.len() {
        let mut sorted = items;
        sorted.sort_by(|a, b| compare(a, b));
        return sorted;
    }

    let mut result = items[..n].to_vec();
    heapify(reverse_compare, &mut result);
    for item in items.into_iter().skip(n) {
        if reverse_compare(&item, &result[0]) == Ordering::Greater {
            let _ = heap_replace(reverse_compare, &mut result, item);
        }
    }
    result.sort_by(|a, b| compare(a, b));
    result
}

fn nlargest(compare: CompareFn, n: usize, iterable: impl IntoIterator<Item = Value>) -> Vec<Value> {
    let items: Vec<Value> = iterable.into_iter().collect();
    if n == 0 || items.is_empty() {
        return Vec::new();
    }
    if n == 1 {
        return vec![items.iter().cloned().max_by(|a, b| compare(a, b)).unwrap()];
    }
    if n >= items.len() {
        let mut sorted = items;
        sorted.sort_by(|a, b| reverse_compare(a, b));
        return sorted;
    }

    let mut result = items[..n].to_vec();
    heapify(compare, &mut result);
    for item in items.into_iter().skip(n) {
        if compare(&item, &result[0]) == Ordering::Greater {
            let _ = heap_replace(compare, &mut result, item);
        }
    }
    result.sort_by(|a, b| reverse_compare(a, b));
    result
}

pub fn heapify_in_place(array: &mut [Value]) {
    heapify(default_compare, array);
}

pub fn consume_slice(array: &mut Vec<Value>) -> Vec<Value> {
    consume(default_compare, array)
}
