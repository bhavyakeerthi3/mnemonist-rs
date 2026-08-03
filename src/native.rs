use indexmap::IndexSet;
use napi::{Error, Result};
use napi_derive::napi;
use serde_json::Value;

use crate::{
    set_ops, BiMap, BitSet, BitVector, CircularBuffer, FixedDeque, FixedDequeError,
    FixedReverseHeap, FixedStack, FixedStackError, Heap, LinkedList, LruCache, LruCacheError, MultiArray,
    MultiMap, MultiMapContainer, MultiSet, Queue, SparseMap, SparseQueueSet, SparseSet, Stack,
    StaticDisjointSet, Vector,
};

fn fixed_stack_error(error: FixedStackError) -> Error {
    Error::from_reason(error.to_string())
}

fn fixed_deque_error(error: FixedDequeError) -> Error {
    Error::from_reason(error.to_string())
}

fn lru_cache_error(error: LruCacheError) -> Error {
    Error::from_reason(error.0)
}

fn to_index_set(values: Vec<Value>) -> IndexSet<Value> {
    values.into_iter().collect()
}

fn from_index_set(values: IndexSet<Value>) -> Vec<Value> {
    values.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Stack
// ---------------------------------------------------------------------------

#[napi]
pub struct StackInner {
    inner: Stack,
}

#[napi]
impl StackInner {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Stack::new(),
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> u32 {
        self.inner.push(item) as u32
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

// ---------------------------------------------------------------------------
// Queue
// ---------------------------------------------------------------------------

#[napi]
pub struct QueueInner {
    inner: Queue,
}

#[napi]
impl QueueInner {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Queue::new(),
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn enqueue(&mut self, item: Value) -> u32 {
        self.inner.enqueue(item) as u32
    }

    #[napi]
    pub fn dequeue(&mut self) -> Option<Value> {
        self.inner.dequeue()
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

// ---------------------------------------------------------------------------
// LinkedList
// ---------------------------------------------------------------------------

#[napi]
pub struct LinkedListInner {
    inner: LinkedList,
}

#[napi]
impl LinkedListInner {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> u32 {
        self.inner.push(item) as u32
    }

    #[napi]
    pub fn unshift(&mut self, item: Value) -> u32 {
        self.inner.unshift(item) as u32
    }

    #[napi]
    pub fn shift(&mut self) -> Option<Value> {
        self.inner.shift()
    }

    #[napi]
    pub fn first(&self) -> Option<Value> {
        self.inner.first().cloned()
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn last(&self) -> Option<Value> {
        self.inner.last().cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

// ---------------------------------------------------------------------------
// FixedStack
// ---------------------------------------------------------------------------

#[napi]
pub struct FixedStackInner {
    inner: FixedStack,
}

#[napi]
impl FixedStackInner {
    #[napi(constructor)]
    pub fn new(capacity: u32) -> Result<Self> {
        Ok(Self {
            inner: FixedStack::new(capacity as usize).map_err(fixed_stack_error)?,
        })
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> Result<u32> {
        self.inner
            .push(item)
            .map(|size| size as u32)
            .map_err(fixed_stack_error)
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

// ---------------------------------------------------------------------------
// FixedDeque
// ---------------------------------------------------------------------------

#[napi]
pub struct FixedDequeInner {
    inner: FixedDeque,
}

#[napi]
impl FixedDequeInner {
    #[napi(constructor)]
    pub fn new(capacity: u32) -> Result<Self> {
        Ok(Self {
            inner: FixedDeque::new(capacity as usize).map_err(fixed_deque_error)?,
        })
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> Result<u32> {
        self.inner
            .push(item)
            .map(|size| size as u32)
            .map_err(fixed_deque_error)
    }

    #[napi]
    pub fn unshift(&mut self, item: Value) -> Result<u32> {
        self.inner
            .unshift(item)
            .map(|size| size as u32)
            .map_err(fixed_deque_error)
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn shift(&mut self) -> Option<Value> {
        self.inner.shift()
    }

    #[napi]
    pub fn peek_first(&self) -> Option<Value> {
        self.inner.peek_first().cloned()
    }

    #[napi]
    pub fn peek_last(&self) -> Option<Value> {
        self.inner.peek_last().cloned()
    }

    #[napi]
    pub fn get(&self, index: u32) -> Option<Value> {
        self.inner.get(index as usize).cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn start(&self) -> u32 {
        self.inner.start() as u32
    }
}

// ---------------------------------------------------------------------------
// CircularBuffer
// ---------------------------------------------------------------------------

#[napi]
pub struct CircularBufferInner {
    inner: CircularBuffer,
}

#[napi]
impl CircularBufferInner {
    #[napi(constructor)]
    pub fn new(capacity: u32) -> Result<Self> {
        Ok(Self {
            inner: CircularBuffer::new(capacity as usize).map_err(fixed_deque_error)?,
        })
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> u32 {
        self.inner.push(item) as u32
    }

    #[napi]
    pub fn unshift(&mut self, item: Value) -> u32 {
        self.inner.unshift(item) as u32
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn shift(&mut self) -> Option<Value> {
        self.inner.shift()
    }

    #[napi]
    pub fn peek_first(&self) -> Option<Value> {
        self.inner.peek_first().cloned()
    }

    #[napi]
    pub fn peek_last(&self) -> Option<Value> {
        self.inner.peek_last().cloned()
    }

    #[napi]
    pub fn get(&self, index: u32) -> Option<Value> {
        self.inner.get(index as usize).cloned()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn start(&self) -> u32 {
        self.inner.start() as u32
    }
}

// ---------------------------------------------------------------------------
// Heap
// ---------------------------------------------------------------------------

#[napi]
pub struct HeapInner {
    inner: Heap,
}

#[napi]
impl HeapInner {
    #[napi(constructor)]
    pub fn new(max: bool) -> Self {
        Self {
            inner: if max {
                Heap::new_max()
            } else {
                Heap::new_min()
            },
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, item: Value) -> u32 {
        self.inner.push(item) as u32
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn replace(&mut self, item: Value) -> Result<Value> {
        self.inner.replace(item).map_err(Error::from_reason)
    }

    #[napi]
    pub fn pushpop(&mut self, item: Value) -> Value {
        self.inner.pushpop(item)
    }

    #[napi]
    pub fn consume(&mut self) -> Vec<Value> {
        self.inner.consume()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

#[napi(js_name = "heapFrom")]
pub fn heap_from(items: Vec<Value>, max: bool) -> HeapInner {
    HeapInner {
        inner: Heap::from_iter(items, max),
    }
}

#[napi(js_name = "heapNsmallest")]
pub fn heap_nsmallest(n: u32, items: Vec<Value>) -> Vec<Value> {
    Heap::nsmallest(n as usize, items)
}

#[napi(js_name = "heapNlargest")]
pub fn heap_nlargest(n: u32, items: Vec<Value>) -> Vec<Value> {
    Heap::nlargest(n as usize, items)
}

// ---------------------------------------------------------------------------
// Set operations (free functions)
// ---------------------------------------------------------------------------

#[napi(js_name = "setIntersection")]
pub fn set_intersection(sets: Vec<Vec<Value>>) -> Result<Vec<Value>> {
    let sets: Vec<_> = sets.into_iter().map(to_index_set).collect();
    set_ops::intersection(&sets)
        .map(from_index_set)
        .map_err(Error::from_reason)
}

#[napi(js_name = "setUnion")]
pub fn set_union(sets: Vec<Vec<Value>>) -> Result<Vec<Value>> {
    let sets: Vec<_> = sets.into_iter().map(to_index_set).collect();
    set_ops::union(&sets)
        .map(from_index_set)
        .map_err(Error::from_reason)
}

#[napi(js_name = "setDifference")]
pub fn set_difference(a: Vec<Value>, b: Vec<Value>) -> Vec<Value> {
    from_index_set(set_ops::difference(&to_index_set(a), &to_index_set(b)))
}

#[napi(js_name = "setSymmetricDifference")]
pub fn set_symmetric_difference(a: Vec<Value>, b: Vec<Value>) -> Vec<Value> {
    from_index_set(set_ops::symmetric_difference(
        &to_index_set(a),
        &to_index_set(b),
    ))
}

#[napi(js_name = "setIsSubset")]
pub fn set_is_subset(a: Vec<Value>, b: Vec<Value>) -> bool {
    set_ops::is_subset(&to_index_set(a), &to_index_set(b))
}

#[napi(js_name = "setIsSuperset")]
pub fn set_is_superset(a: Vec<Value>, b: Vec<Value>) -> bool {
    set_ops::is_superset(&to_index_set(a), &to_index_set(b))
}

#[napi(js_name = "setIntersectionSize")]
pub fn set_intersection_size(a: Vec<Value>, b: Vec<Value>) -> u32 {
    set_ops::intersection_size(&to_index_set(a), &to_index_set(b)) as u32
}

#[napi(js_name = "setUnionSize")]
pub fn set_union_size(a: Vec<Value>, b: Vec<Value>) -> u32 {
    set_ops::union_size(&to_index_set(a), &to_index_set(b)) as u32
}

#[napi(js_name = "setJaccard")]
pub fn set_jaccard(a: Vec<Value>, b: Vec<Value>) -> f64 {
    set_ops::jaccard(&to_index_set(a), &to_index_set(b))
}

#[napi(js_name = "setOverlap")]
pub fn set_overlap(a: Vec<Value>, b: Vec<Value>) -> f64 {
    set_ops::overlap(&to_index_set(a), &to_index_set(b))
}

// ===========================================================================
// PHASE 1 — Mechanical bridges
// ===========================================================================

// ---------------------------------------------------------------------------
// StaticDisjointSet
// ---------------------------------------------------------------------------

#[napi]
pub struct StaticDisjointSetInner {
    inner: StaticDisjointSet,
}

#[napi]
impl StaticDisjointSetInner {
    #[napi(constructor)]
    pub fn new(size: u32) -> Self {
        Self {
            inner: StaticDisjointSet::new(size as usize),
        }
    }

    #[napi]
    pub fn union(&mut self, x: u32, y: u32) {
        self.inner.union(x as usize, y as usize);
    }

    #[napi]
    pub fn connected(&mut self, x: u32, y: u32) -> bool {
        self.inner.connected(x as usize, y as usize)
    }

    #[napi]
    pub fn mapping(&mut self) -> Vec<u32> {
        self.inner.mapping().into_iter().map(|v| v as u32).collect()
    }

    #[napi]
    pub fn compile(&mut self) -> Vec<Vec<u32>> {
        self.inner
            .compile()
            .into_iter()
            .map(|group| group.into_iter().map(|v| v as u32).collect())
            .collect()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn dimension(&self) -> u32 {
        self.inner.dimension() as u32
    }
}

// ---------------------------------------------------------------------------
// SparseSet
// ---------------------------------------------------------------------------

#[napi]
pub struct SparseSetInner {
    inner: SparseSet,
}

#[napi]
impl SparseSetInner {
    #[napi(constructor)]
    pub fn new(length: u32) -> Self {
        Self {
            inner: SparseSet::new(length as usize),
        }
    }

    #[napi]
    pub fn add(&mut self, member: u32) {
        self.inner.add(member as usize);
    }

    #[napi]
    pub fn has(&self, member: u32) -> bool {
        self.inner.has(member as usize)
    }

    #[napi]
    pub fn delete(&mut self, member: u32) -> bool {
        self.inner.delete(member as usize)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn values(&self) -> Vec<u32> {
        self.inner.values().map(|v| v as u32).collect()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.length() as u32
    }
}

// ---------------------------------------------------------------------------
// SparseQueueSet
// ---------------------------------------------------------------------------

#[napi]
pub struct SparseQueueSetInner {
    inner: SparseQueueSet,
}

#[napi]
impl SparseQueueSetInner {
    #[napi(constructor)]
    pub fn new(length: u32) -> Self {
        Self {
            inner: SparseQueueSet::new(length as usize),
        }
    }

    #[napi]
    pub fn enqueue(&mut self, member: u32) -> bool {
        self.inner.enqueue(member as usize)
    }

    #[napi]
    pub fn dequeue(&mut self) -> Option<u32> {
        self.inner.dequeue().map(|v| v as u32)
    }

    #[napi]
    pub fn has(&self, member: u32) -> bool {
        self.inner.has(member as usize)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn values(&self) -> Vec<u32> {
        self.inner.values().map(|v| v as u32).collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<u32> {
        let mut result = Vec::new();
        self.inner.for_each(|member| result.push(member as u32));
        result
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn capacity(&self) -> u32 {
        self.inner.capacity() as u32
    }
}

// ---------------------------------------------------------------------------
// BitSet
// ---------------------------------------------------------------------------

#[napi]
pub struct BitSetInner {
    inner: BitSet,
}

#[napi]
impl BitSetInner {
    #[napi(constructor)]
    pub fn new(length: u32) -> Self {
        Self {
            inner: BitSet::new(length as usize),
        }
    }

    #[napi]
    pub fn set(&mut self, index: u32, value: Option<bool>) {
        self.inner.set(index as usize, value.unwrap_or(true));
    }

    #[napi]
    pub fn get(&self, index: u32) -> u8 {
        self.inner.get(index as usize)
    }

    #[napi]
    pub fn test(&self, index: u32) -> bool {
        self.inner.test(index as usize)
    }

    #[napi]
    pub fn reset(&mut self, index: u32) {
        self.inner.reset(index as usize);
    }

    #[napi]
    pub fn flip(&mut self, index: u32) {
        self.inner.flip(index as usize);
    }

    #[napi]
    pub fn rank(&self, index: u32) -> u32 {
        self.inner.rank(index as usize) as u32
    }

    #[napi]
    pub fn select(&self, r: u32) -> Option<u32> {
        let result = self.inner.select(r as usize);
        if result < 0 {
            None
        } else {
            Some(result as u32)
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn values(&self) -> Vec<u8> {
        self.inner.values().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(i, v)| {
                Value::Array(vec![
                    Value::Number(i.into()),
                    Value::Number(v.into()),
                ])
            })
            .collect()
    }

    #[napi]
    pub fn to_json(&self) -> Vec<u32> {
        self.inner.to_json()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.length() as u32
    }

    #[napi(getter)]
    pub fn word_len(&self) -> u32 {
        self.inner.word_len() as u32
    }
}

// ---------------------------------------------------------------------------
// SparseMap
// ---------------------------------------------------------------------------

#[napi]
pub struct SparseMapInner {
    inner: SparseMap,
}

#[napi]
impl SparseMapInner {
    #[napi(constructor)]
    pub fn new(length: u32) -> Self {
        Self {
            inner: SparseMap::new(length as usize),
        }
    }

    #[napi]
    pub fn set(&mut self, member: u32, value: Value) {
        self.inner.set(member as usize, value);
    }

    #[napi]
    pub fn get(&self, member: u32) -> Option<Value> {
        self.inner.get(member as usize).cloned()
    }

    #[napi]
    pub fn has(&self, member: u32) -> bool {
        self.inner.has(member as usize)
    }

    #[napi]
    pub fn delete(&mut self, member: u32) -> bool {
        self.inner.delete(member as usize)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn keys(&self) -> Vec<u32> {
        self.inner.keys().map(|v| v as u32).collect()
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(k, v)| {
                Value::Array(vec![
                    Value::Number(k.into()),
                    v.clone(),
                ])
            })
            .collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<Value> {
        let mut result = Vec::new();
        self.inner.for_each(|value, key| {
            result.push(Value::Array(vec![
                value.clone(),
                Value::Number(key.into()),
            ]));
        });
        result
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.length() as u32
    }
}

// ---------------------------------------------------------------------------
// Vector
// ---------------------------------------------------------------------------

#[napi]
pub struct VectorInner {
    inner: Vector,
}

#[napi]
impl VectorInner {
    #[napi(constructor)]
    pub fn new(capacity: u32, initial_length: u32) -> Self {
        let logical_capacity = (capacity as usize).max(initial_length as usize);
        let mut inner = Vector::with_capacity(logical_capacity);
        if initial_length > 0 {
            inner.resize(initial_length as usize, Value::Number(0.into()));
        }
        Self { inner }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn set(&mut self, index: u32, value: Value) -> bool {
        self.inner.set(index as usize, value)
    }

    #[napi]
    pub fn get(&self, index: u32) -> Option<Value> {
        self.inner.get(index as usize).cloned()
    }

    #[napi]
    pub fn push(&mut self, value: Value) -> u32 {
        self.inner.push(value) as u32
    }

    #[napi]
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    #[napi]
    pub fn reallocate(&mut self, capacity: u32) {
        self.inner.reallocate(capacity as usize);
    }

    #[napi]
    pub fn resize(&mut self, length: u32) {
        self.inner.resize(length as usize, Value::Number(0.into()));
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(index, value)| {
                Value::Array(vec![Value::Number((index as u64).into()), value.clone()])
            })
            .collect()
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.length() as u32
    }
}

// ---------------------------------------------------------------------------
// BitVector
// ---------------------------------------------------------------------------

#[napi]
pub struct BitVectorInner {
    inner: BitVector,
}

#[napi]
impl BitVectorInner {
    #[napi(constructor)]
    pub fn new(initial_length: u32) -> Self {
        let length = if initial_length == 0 {
            0
        } else {
            initial_length as usize
        };
        let mut bv = BitVector::with_length(length);
        // Ensure minimum capacity of 32 bits (1 word) if length > 0
        if length > 0 && bv.capacity() < 32 {
            bv.grow(32);
        }
        Self { inner: bv }
    }

    #[napi]
    pub fn set(&mut self, index: u32, value: Option<bool>) {
        self.inner.set(index as usize, value.unwrap_or(true));
    }

    #[napi]
    pub fn get(&self, index: u32) -> Option<u8> {
        let result = self.inner.get(index as usize);
        // Out-of-bounds: Rust returns 0, but upstream JS returns undefined
        // We need the caller to distinguish: if index >= length, return None (→ undefined)
        if index as usize >= self.inner.length() {
            None
        } else {
            Some(result)
        }
    }

    #[napi]
    pub fn test(&self, index: u32) -> bool {
        self.inner.test(index as usize)
    }

    #[napi]
    pub fn reset(&mut self, index: u32) {
        self.inner.reset(index as usize);
    }

    #[napi]
    pub fn flip(&mut self, index: u32) {
        self.inner.flip(index as usize);
    }

    #[napi]
    pub fn push(&mut self, bit: bool) -> u32 {
        self.inner.push(bit) as u32
    }

    #[napi]
    pub fn pop(&mut self) -> Option<u8> {
        self.inner.pop()
    }

    #[napi]
    pub fn rank(&self, index: u32) -> u32 {
        self.inner.rank(index as usize) as u32
    }

    #[napi]
    pub fn select(&self, r: u32) -> Option<u32> {
        let result = self.inner.select(r as usize);
        if result < 0 {
            None
        } else {
            Some(result as u32)
        }
    }

    #[napi]
    pub fn reallocate(&mut self, new_capacity: u32) {
        self.inner.reallocate(new_capacity as usize);
    }

    #[napi]
    pub fn grow(&mut self, desired_capacity: u32) {
        self.inner.grow(desired_capacity as usize);
    }

    #[napi]
    pub fn resize(&mut self, new_length: u32) {
        self.inner.resize(new_length as usize);
    }

    #[napi]
    pub fn to_json(&self) -> Vec<u32> {
        self.inner.to_json()
    }

    #[napi]
    pub fn values(&self) -> Vec<u8> {
        self.inner.values().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(i, v)| {
                Value::Array(vec![
                    Value::Number(i.into()),
                    Value::Number(v.into()),
                ])
            })
            .collect()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn length(&self) -> u32 {
        self.inner.length() as u32
    }

    #[napi(getter)]
    pub fn capacity(&self) -> u32 {
        self.inner.capacity() as u32
    }
}

// ---------------------------------------------------------------------------
// MultiSet
// ---------------------------------------------------------------------------

#[napi]
pub struct MultiSetInner {
    inner: MultiSet,
}

#[napi]
impl MultiSetInner {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: MultiSet::new(),
        }
    }

    #[napi]
    pub fn add(&mut self, item: Value, count: Option<f64>) -> Result<()> {
        let count = count.unwrap_or(1.0);
        if count != count as isize as f64 {
            return Err(Error::from_reason(
                "mnemonist/multi-set.add: given count is not a number.",
            ));
        }
        self.inner.add(item, count as isize);
        Ok(())
    }

    #[napi]
    pub fn set(&mut self, item: Value, count: f64) -> Result<()> {
        if count != count as isize as f64 {
            return Err(Error::from_reason(
                "mnemonist/multi-set.set: given count is not a number.",
            ));
        }
        self.inner.set(item, count as isize);
        Ok(())
    }

    #[napi]
    pub fn has(&self, item: Value) -> bool {
        self.inner.has(&item)
    }

    #[napi]
    pub fn delete(&mut self, item: Value) -> bool {
        self.inner.delete(&item)
    }

    #[napi]
    pub fn remove(&mut self, item: Value, count: Option<f64>) -> Result<()> {
        let count = count.unwrap_or(1.0);
        if count != count as isize as f64 {
            return Err(Error::from_reason(
                "mnemonist/multi-set.remove: given count is not a number.",
            ));
        }
        self.inner.remove(&item, count as isize);
        Ok(())
    }

    #[napi]
    pub fn edit(&mut self, from: Value, to: Value) {
        self.inner.edit(&from, to);
    }

    #[napi]
    pub fn multiplicity(&self, item: Value) -> u32 {
        self.inner.multiplicity(&item) as u32
    }

    #[napi]
    pub fn frequency(&self, item: Value) -> f64 {
        self.inner.frequency(&item)
    }

    #[napi]
    pub fn top(&self, n: u32) -> Result<Vec<Vec<Value>>> {
        self.inner
            .top(n as usize)
            .map(|entries| {
                entries
                    .into_iter()
                    .map(|(key, count)| vec![key, Value::Number(count.into())])
                    .collect()
            })
            .map_err(|e| Error::from_reason(e))
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values()
    }

    #[napi]
    pub fn keys(&self) -> Vec<Value> {
        self.inner.keys().cloned().collect()
    }

    #[napi]
    pub fn multiplicities(&self) -> Vec<Vec<Value>> {
        self.inner
            .multiplicities()
            .map(|(key, count)| vec![key.clone(), Value::Number((*count).into())])
            .collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<Vec<Value>> {
        let mut result = Vec::new();
        for (value, count) in self.inner.multiplicities() {
            for _ in 0..*count {
                result.push(vec![value.clone(), value.clone()]);
            }
        }
        result
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn dimension(&self) -> u32 {
        self.inner.dimension() as u32
    }
}

#[napi(js_name = "multiSetFrom")]
pub fn multi_set_from(items: Vec<Value>) -> MultiSetInner {
    MultiSetInner {
        inner: MultiSet::from_iter(items),
    }
}

#[napi(js_name = "multiSetIsSubset")]
pub fn multi_set_is_subset(a_items: Vec<Value>, b_items: Vec<Value>) -> bool {
    let a = MultiSet::from_iter(a_items);
    let b = MultiSet::from_iter(b_items);
    MultiSet::is_subset(&a, &b)
}

#[napi(js_name = "multiSetIsSuperset")]
pub fn multi_set_is_superset(a_items: Vec<Value>, b_items: Vec<Value>) -> bool {
    let a = MultiSet::from_iter(a_items);
    let b = MultiSet::from_iter(b_items);
    MultiSet::is_superset(&a, &b)
}

// ---------------------------------------------------------------------------
// MultiArray
// ---------------------------------------------------------------------------

#[napi]
pub struct MultiArrayInner {
    inner: MultiArray,
}

#[napi]
impl MultiArrayInner {
    #[napi(constructor)]
    pub fn new(width: u32) -> Self {
        Self {
            inner: MultiArray::new(width as usize),
        }
    }

    #[napi]
    pub fn set(&mut self, index: u32, value: Value) {
        self.inner.set(index as usize, value);
    }

    #[napi]
    pub fn get(&self, index: u32) -> Option<Vec<Value>> {
        self.inner.get(index as usize).map(|slice| slice.to_vec())
    }

    #[napi]
    pub fn has(&self, index: u32) -> bool {
        self.inner.has(index as usize)
    }

    #[napi]
    pub fn count(&self, index: u32) -> u32 {
        self.inner.count(index as usize) as u32
    }

    #[napi]
    pub fn push(&mut self, value: Value) {
        self.inner.push(value);
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn values_at(&self, index: u32) -> Vec<Value> {
        self.inner.values_at(index as usize).cloned().collect()
    }

    #[napi]
    pub fn containers(&self) -> Vec<Vec<Value>> {
        self.inner.containers().map(|c| c.to_vec()).collect()
    }

    #[napi]
    pub fn keys(&self) -> Vec<u32> {
        self.inner.keys().map(|k| k as u32).collect()
    }

    #[napi]
    pub fn associations(&self) -> Vec<Value> {
        self.inner
            .associations()
            .map(|(i, c)| {
                Value::Array(vec![
                    Value::Number(i.into()),
                    Value::Array(c.to_vec()),
                ])
            })
            .collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(i, v)| {
                Value::Array(vec![
                    Value::Number(i.into()),
                    v.clone(),
                ])
            })
            .collect()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn dimension(&self) -> u32 {
        self.inner.dimension() as u32
    }
}

// ---------------------------------------------------------------------------
// BiMap
// ---------------------------------------------------------------------------

#[napi]
pub struct BiMapInner {
    inner: BiMap,
}

#[napi]
impl BiMapInner {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: BiMap::new(),
        }
    }

    #[napi]
    pub fn set(&mut self, key: Value, value: Value) {
        self.inner.set(key, value);
    }

    #[napi]
    pub fn get(&self, key: Value) -> Option<Value> {
        self.inner.get(&key).cloned()
    }

    #[napi]
    pub fn has(&self, key: Value) -> bool {
        self.inner.has(&key)
    }

    #[napi]
    pub fn delete(&mut self, key: Value) -> bool {
        self.inner.delete(&key)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn inverse_get(&self, value: Value) -> Option<Value> {
        self.inner.inverse_get(&value).cloned()
    }

    #[napi]
    pub fn inverse_has(&self, value: Value) -> bool {
        self.inner.inverse_has(&value)
    }

    #[napi]
    pub fn keys(&self) -> Vec<Value> {
        self.inner.keys().cloned().collect()
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), v.clone()])
            })
            .collect()
    }

    #[napi]
    pub fn inverse_keys(&self) -> Vec<Value> {
        self.inner.inverse_keys().cloned().collect()
    }

    #[napi]
    pub fn inverse_values(&self) -> Vec<Value> {
        self.inner.inverse_values().cloned().collect()
    }

    #[napi]
    pub fn inverse_entries(&self) -> Vec<Value> {
        self.inner
            .inverse_entries()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), v.clone()])
            })
            .collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), v.clone()])
            })
            .collect()
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

#[napi(js_name = "biMapFrom")]
pub fn bi_map_from(items: Vec<Vec<Value>>) -> Result<BiMapInner> {
    let mut native_items = Vec::new();
    for item in items {
        if item.len() != 2 {
            return Err(Error::from_reason("biMapFrom: entries must have length 2"));
        }
        native_items.push((item[0].clone(), item[1].clone()));
    }
    Ok(BiMapInner {
        inner: BiMap::from_iter(native_items),
    })
}

// ---------------------------------------------------------------------------
// MultiMap
// ---------------------------------------------------------------------------

#[napi]
pub struct MultiMapInner {
    inner: MultiMap,
}

#[napi]
impl MultiMapInner {
    #[napi(constructor)]
    pub fn new(use_set: bool) -> Self {
        Self {
            inner: MultiMap::new(if use_set {
                MultiMapContainer::Set
            } else {
                MultiMapContainer::Vec
            }),
        }
    }

    #[napi]
    pub fn set(&mut self, key: Value, value: Value) {
        self.inner.set(key, value);
    }

    #[napi]
    pub fn get(&self, key: Value) -> Option<Vec<Value>> {
        self.inner.get(&key).map(|slice| slice.to_vec())
    }

    #[napi]
    pub fn has(&self, key: Value) -> bool {
        self.inner.has(&key)
    }

    #[napi]
    pub fn contains(&self, key: Value, value: Value) -> bool {
        self.inner.contains(&key, &value)
    }

    #[napi]
    pub fn delete(&mut self, key: Value) -> bool {
        self.inner.delete(&key)
    }

    #[napi]
    pub fn remove(&mut self, key: Value, value: Value) -> bool {
        self.inner.remove(&key, &value)
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn keys(&self) -> Vec<Value> {
        self.inner.keys().cloned().collect()
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), v.clone()])
            })
            .collect()
    }

    #[napi]
    pub fn containers(&self) -> Vec<Value> {
        self.inner
            .containers()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), Value::Array(v.to_vec())])
            })
            .collect()
    }

    #[napi]
    pub fn associations(&self) -> Vec<Value> {
        self.inner
            .associations()
            .map(|(k, v)| {
                Value::Array(vec![k.clone(), Value::Array(v.to_vec())])
            })
            .collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<Value> {
        let mut result = Vec::new();
        self.inner.for_each(|value, key| {
            result.push(Value::Array(vec![value.clone(), key.clone()]));
        });
        result
    }

    #[napi]
    pub fn for_each_association(&self) -> Vec<Value> {
        let mut result = Vec::new();
        self.inner.for_each_association(|values, key| {
            result.push(Value::Array(vec![key.clone(), Value::Array(values.to_vec())]));
        });
        result
    }

    #[napi]
    pub fn multiplicity(&self, key: Value) -> u32 {
        self.inner.multiplicity(&key) as u32
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    #[napi(getter)]
    pub fn dimension(&self) -> u32 {
        self.inner.dimension() as u32
    }
}

#[napi(js_name = "multiMapFrom")]
pub fn multi_map_from(items: Vec<(Value, Value)>) -> MultiMapInner {
    MultiMapInner {
        inner: MultiMap::from(items),
    }
}

// ---------------------------------------------------------------------------
// LruCache (+ LruMap, LruCacheWithDelete, LruMapWithDelete)
// ---------------------------------------------------------------------------

#[napi(object)]
pub struct LruSetPopResult {
    pub evicted: bool,
    pub key: Value,
    pub value: Value,
}

#[napi]
pub struct LruCacheInner {
    inner: LruCache,
}

#[napi]
impl LruCacheInner {
    #[napi(constructor)]
    pub fn new(capacity: f64) -> Result<Self> {
        // Reject non-integer, non-positive capacities (matching upstream behavior)
        if capacity != capacity as usize as f64 || capacity <= 0.0 {
            return Err(Error::from_reason(
                "mnemonist/lru-cache: capacity should be positive number.",
            ));
        }
        Ok(Self {
            inner: LruCache::new(capacity as usize).map_err(lru_cache_error)?,
        })
    }

    #[napi]
    pub fn set(&mut self, key: Value, value: Value) {
        self.inner.set(key, value);
    }

    #[napi]
    pub fn get(&mut self, key: Value) -> Option<Value> {
        self.inner.get(&key)
    }

    #[napi]
    pub fn peek(&self, key: Value) -> Option<Value> {
        self.inner.peek(&key).cloned()
    }

    #[napi]
    pub fn has(&self, key: Value) -> bool {
        self.inner.has(&key)
    }

    #[napi]
    pub fn delete(&mut self, key: Value) -> bool {
        self.inner.delete(&key)
    }

    #[napi]
    pub fn remove(&mut self, key: Value) -> Option<Value> {
        self.inner.remove(&key)
    }

    #[napi]
    pub fn setpop(&mut self, key: Value, value: Value) -> Option<LruSetPopResult> {
        self.inner.setpop(key, value).map(|sp| LruSetPopResult {
            evicted: sp.evicted,
            key: sp.key,
            value: sp.value,
        })
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn keys(&self) -> Vec<Value> {
        self.inner.keys().cloned().collect()
    }

    #[napi]
    pub fn values(&self) -> Vec<Value> {
        self.inner.values().cloned().collect()
    }

    #[napi]
    pub fn entries(&self) -> Vec<Value> {
        self.inner
            .entries()
            .map(|(k, v)| Value::Array(vec![k.clone(), v.clone()]))
            .collect()
    }

    #[napi]
    pub fn for_each(&self) -> Vec<Value> {
        let mut result = Vec::new();
        self.inner.for_each(|value, key| {
            result.push(Value::Array(vec![value.clone(), key.clone()]));
        });
        result
    }

    #[napi(getter)]
    pub fn capacity(&self) -> u32 {
        self.inner.capacity() as u32
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}

#[napi(js_name = "lruCacheFrom")]
pub fn lru_cache_from(items: Vec<Vec<Value>>, capacity: Option<u32>) -> Result<LruCacheInner> {
    let cap = capacity.map(|c| c as usize);
    let mut native_items = Vec::new();
    for item in items {
        if item.len() != 2 {
            return Err(Error::from_reason("lruCacheFrom: entries must have length 2"));
        }
        native_items.push((item[0].clone(), item[1].clone()));
    }
    let cache = LruCache::from_iter(native_items, cap).map_err(lru_cache_error)?;
    Ok(LruCacheInner { inner: cache })
}

// ---------------------------------------------------------------------------
// FixedReverseHeap
// ---------------------------------------------------------------------------

#[napi]
pub struct FixedReverseHeapInner {
    inner: FixedReverseHeap,
}

#[napi]
impl FixedReverseHeapInner {
    #[napi(constructor)]
    pub fn new(capacity: u32) -> Self {
        Self {
            inner: FixedReverseHeap::new(capacity as usize),
        }
    }

    #[napi]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[napi]
    pub fn push(&mut self, value: Value) -> u32 {
        self.inner.push(value) as u32
    }

    #[napi]
    pub fn peek(&self) -> Option<Value> {
        self.inner.peek().cloned()
    }

    #[napi]
    pub fn consume(&mut self) -> Vec<Value> {
        self.inner.consume()
    }

    #[napi]
    pub fn to_array(&self) -> Vec<Value> {
        self.inner.to_array()
    }

    #[napi(getter)]
    pub fn capacity(&self) -> u32 {
        self.inner.capacity() as u32
    }

    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }
}
