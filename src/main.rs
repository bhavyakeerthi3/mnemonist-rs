use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, Write};

use indexmap::{IndexMap, IndexSet};
use mnemonist::set_ops;
use mnemonist::{
    BiMap, BitSet, BitVector, BkTree, CircularBuffer, CritBitTreeMap, FixedDeque, FixedReverseHeap,
    FixedStack, GeneralizedSuffixArray, HashedArrayTree, Heap, Interval, JsBloomFilter, LinkedList,
    LruCache, MultiArray, MultiMap, MultiMapContainer, MultiSet, Queue, SparseMap, SparseQueueSet,
    SparseSet, Stack, StaticDisjointSet, StaticIntervalTree, SuffixArray, SymSpell, TrieMap,
    Vector,
};
use serde::Deserialize;
use serde_json::{json, Value};

const PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(default)]
    id: Value,
    op: String,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    args: Vec<Value>,
}

#[derive(Default)]
struct ProtocolInvertedIndex {
    documents: Vec<Value>,
    postings: IndexMap<Value, Vec<usize>>,
}

impl ProtocolInvertedIndex {
    fn clear(&mut self) {
        self.documents.clear();
        self.postings.clear();
    }

    fn add(&mut self, document: Value, tokens: Vec<Value>) {
        let index = self.documents.len();
        self.documents.push(document);
        let mut seen = IndexSet::new();
        for token in tokens {
            if seen.insert(token.clone()) {
                self.postings.entry(token).or_default().push(index);
            }
        }
    }

    fn get(&self, tokens: &[Value]) -> Vec<Value> {
        let Some(first) = tokens.first().and_then(|token| self.postings.get(token)) else {
            return Vec::new();
        };
        let mut matches = first.clone();
        for token in &tokens[1..] {
            let Some(posting) = self.postings.get(token) else {
                return Vec::new();
            };
            let accepted: IndexSet<_> = posting.iter().copied().collect();
            matches.retain(|index| accepted.contains(index));
        }
        matches
            .into_iter()
            .map(|index| self.documents[index].clone())
            .collect()
    }
}

struct ProtocolPassjoinIndex {
    k: usize,
    strings: Vec<String>,
}

struct ProtocolKdTree {
    labels: Vec<Value>,
    points: Vec<Vec<f64>>,
}

#[derive(Default)]
struct ProtocolDefaultMap {
    entries: IndexMap<Value, Value>,
}

struct ProtocolFuzzyMultiMap {
    entries: IndexMap<Value, Vec<Value>>,
    set_mode: bool,
}

struct ProtocolComparatorHeap {
    items: Vec<Value>,
    max: bool,
}

impl ProtocolComparatorHeap {
    fn compare(
        &self,
        comparisons: &IndexMap<(Value, Value), i8>,
        left: &Value,
        right: &Value,
    ) -> Result<std::cmp::Ordering, String> {
        let value = comparisons
            .get(&(left.clone(), right.clone()))
            .copied()
            .or_else(|| (left == right).then_some(0))
            .ok_or("missing custom comparator result")?;
        let ordering = value.cmp(&0);
        Ok(if self.max {
            ordering.reverse()
        } else {
            ordering
        })
    }

    fn sift_down(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
        start: usize,
        mut index: usize,
    ) -> Result<(), String> {
        let item = self.items[index].clone();
        while index > start {
            let parent = (index - 1) >> 1;
            if self.compare(comparisons, &item, &self.items[parent])? == std::cmp::Ordering::Less {
                self.items[index] = self.items[parent].clone();
                index = parent;
            } else {
                break;
            }
        }
        self.items[index] = item;
        Ok(())
    }

    fn sift_up(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
        mut index: usize,
    ) -> Result<(), String> {
        let end = self.items.len();
        let start = index;
        let item = self.items[index].clone();
        let mut child = index * 2 + 1;
        while child < end {
            let right = child + 1;
            if right < end
                && self.compare(comparisons, &self.items[child], &self.items[right])?
                    != std::cmp::Ordering::Less
            {
                child = right;
            }
            self.items[index] = self.items[child].clone();
            index = child;
            child = index * 2 + 1;
        }
        self.items[index] = item;
        self.sift_down(comparisons, start, index)
    }

    fn push(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
        item: Value,
    ) -> Result<usize, String> {
        self.items.push(item);
        self.sift_down(comparisons, 0, self.items.len() - 1)?;
        Ok(self.items.len())
    }

    fn pop(&mut self, comparisons: &IndexMap<(Value, Value), i8>) -> Result<Option<Value>, String> {
        let Some(last) = self.items.pop() else {
            return Ok(None);
        };
        if self.items.is_empty() {
            return Ok(Some(last));
        }
        let top = std::mem::replace(&mut self.items[0], last);
        self.sift_up(comparisons, 0)?;
        Ok(Some(top))
    }

    fn replace(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
        item: Value,
    ) -> Result<Value, String> {
        if self.items.is_empty() {
            return Err("mnemonist/heap.replace: cannot pop an empty heap.".to_owned());
        }
        let top = std::mem::replace(&mut self.items[0], item);
        self.sift_up(comparisons, 0)?;
        Ok(top)
    }

    fn pushpop(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
        mut item: Value,
    ) -> Result<Value, String> {
        if !self.items.is_empty()
            && self.compare(comparisons, &self.items[0], &item)? == std::cmp::Ordering::Less
        {
            std::mem::swap(&mut self.items[0], &mut item);
            self.sift_up(comparisons, 0)?;
        }
        Ok(item)
    }

    fn consume(
        &mut self,
        comparisons: &IndexMap<(Value, Value), i8>,
    ) -> Result<Vec<Value>, String> {
        let mut values = Vec::with_capacity(self.items.len());
        while let Some(value) = self.pop(comparisons)? {
            values.push(value);
        }
        Ok(values)
    }

    fn best(&self, comparisons: &IndexMap<(Value, Value), i8>) -> Result<Option<Value>, String> {
        let Some(mut best) = self.items.first().cloned() else {
            return Ok(None);
        };
        for item in self.items.iter().skip(1) {
            if self.compare(comparisons, item, &best)? == std::cmp::Ordering::Less {
                best = item.clone();
            }
        }
        Ok(Some(best))
    }
}

impl ProtocolFuzzyMultiMap {
    fn size(&self) -> usize {
        self.entries.values().map(Vec::len).sum()
    }
    fn set(&mut self, key: Value, value: Value) {
        let values = self.entries.entry(key).or_default();
        if !self.set_mode || !values.contains(&value) {
            values.push(value);
        }
    }
    fn values(&self) -> Vec<Value> {
        self.entries.values().flatten().cloned().collect()
    }
}

impl ProtocolKdTree {
    fn nearest(&self, k: usize, query: &[f64]) -> Vec<Value> {
        let mut indices: Vec<_> = (0..self.points.len()).collect();
        indices.sort_by(|left, right| {
            squared_distance(&self.points[*left], query)
                .total_cmp(&squared_distance(&self.points[*right], query))
                .then_with(|| right.cmp(left))
        });
        indices
            .into_iter()
            .take(k)
            .map(|index| self.labels[index].clone())
            .collect()
    }

    fn nearest_linear(&self, k: usize, query: &[f64]) -> Vec<Value> {
        let mut indices: Vec<_> = (0..self.points.len()).collect();
        indices.sort_by(|left, right| {
            squared_distance(&self.points[*left], query)
                .total_cmp(&squared_distance(&self.points[*right], query))
                .then_with(|| {
                    self.labels[*left]
                        .to_string()
                        .cmp(&self.labels[*right].to_string())
                })
        });
        indices
            .into_iter()
            .take(k)
            .map(|index| self.labels[index].clone())
            .collect()
    }
}

fn squared_distance(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(a, b)| (a - b).powi(2)).sum()
}

impl ProtocolPassjoinIndex {
    fn new(k: usize) -> Self {
        Self {
            k,
            strings: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.strings.clear();
    }

    fn add(&mut self, value: String) {
        self.strings.push(value);
    }

    fn search(&self, query: &str) -> Vec<String> {
        let mut matches = IndexSet::new();
        for candidate in &self.strings {
            if candidate.len().abs_diff(query.len()) <= self.k
                && mnemonist::bk_tree::levenshtein(query, candidate) <= self.k
            {
                matches.insert(candidate.clone());
            }
        }
        matches.into_iter().collect()
    }
}

enum Collection {
    Stack(Stack),
    Queue(Queue),
    LinkedList(LinkedList),
    FixedStack(FixedStack),
    FixedDeque(FixedDeque),
    CircularBuffer(CircularBuffer),
    SparseSet(SparseSet),
    LruCache(LruCache),
    BitSet(BitSet),
    SparseQueueSet(SparseQueueSet),
    SparseMap(SparseMap),
    StaticDisjointSet(StaticDisjointSet),
    MultiArray(MultiArray),
    MultiSet(MultiSet),
    MultiMap(MultiMap),
    BiMap(BiMap),
    Heap(Heap),
    FixedReverseHeap(FixedReverseHeap),
    HashedArrayTree(HashedArrayTree),
    SetOps,
    BloomFilter(JsBloomFilter),
    BitVector(BitVector),
    Vector(Vector),
    SuffixArray(SuffixArray),
    GeneralizedSuffixArray(GeneralizedSuffixArray),
    StaticIntervalTree(StaticIntervalTree),
    TrieMap(TrieMap),
    InvertedIndex(ProtocolInvertedIndex),
    SymSpell(SymSpell),
    BkTree(BkTree),
    PassjoinIndex(ProtocolPassjoinIndex),
    KdTree(ProtocolKdTree),
    DefaultMap(ProtocolDefaultMap),
    DefaultWeakMap(ProtocolDefaultMap),
    FuzzyMap(ProtocolDefaultMap),
    FuzzyMultiMap(ProtocolFuzzyMultiMap),
    VpTree(mnemonist::vp_tree::StringVpTree),
    ComparatorHeap(ProtocolComparatorHeap),
    Sort,
    CritBitTreeMap {
        tree: CritBitTreeMap,
        capacity: Option<usize>,
    },
}

impl Collection {
    fn kind(&self) -> &'static str {
        match self {
            Self::Stack(_) => "stack",
            Self::Queue(_) => "queue",
            Self::LinkedList(_) => "linked-list",
            Self::FixedStack(_) => "fixed-stack",
            Self::FixedDeque(_) => "fixed-deque",
            Self::CircularBuffer(_) => "circular-buffer",
            Self::SparseSet(_) => "sparse-set",
            Self::LruCache(_) => "lru-cache",
            Self::BitSet(_) => "bit-set",
            Self::SparseQueueSet(_) => "sparse-queue-set",
            Self::SparseMap(_) => "sparse-map",
            Self::StaticDisjointSet(_) => "static-disjoint-set",
            Self::MultiArray(_) => "multi-array",
            Self::MultiSet(_) => "multi-set",
            Self::MultiMap(_) => "multi-map",
            Self::BiMap(_) => "bi-map",
            Self::Heap(_) => "heap",
            Self::FixedReverseHeap(_) => "fixed-reverse-heap",
            Self::HashedArrayTree(_) => "hashed-array-tree",
            Self::SetOps => "set-ops",
            Self::BloomFilter(_) => "bloom-filter",
            Self::BitVector(_) => "bit-vector",
            Self::Vector(_) => "vector",
            Self::SuffixArray(_) => "suffix-array",
            Self::GeneralizedSuffixArray(_) => "generalized-suffix-array",
            Self::StaticIntervalTree(_) => "static-interval-tree",
            Self::TrieMap(_) => "trie-map",
            Self::InvertedIndex(_) => "inverted-index",
            Self::SymSpell(_) => "symspell",
            Self::BkTree(_) => "bk-tree",
            Self::PassjoinIndex(_) => "passjoin-index",
            Self::KdTree(_) => "kd-tree",
            Self::DefaultMap(_) => "default-map",
            Self::DefaultWeakMap(_) => "default-weak-map",
            Self::FuzzyMap(_) => "fuzzy-map",
            Self::FuzzyMultiMap(_) => "fuzzy-multi-map",
            Self::VpTree(_) => "vp-tree",
            Self::ComparatorHeap(_) => "comparator-heap",
            Self::Sort => "sort",
            Self::CritBitTreeMap {
                capacity: Some(_), ..
            } => "fixed-critbit-tree-map",
            Self::CritBitTreeMap { capacity: None, .. } => "critbit-tree-map",
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Stack(collection) => collection.size(),
            Self::Queue(collection) => collection.size(),
            Self::LinkedList(collection) => collection.size(),
            Self::FixedStack(collection) => collection.size(),
            Self::FixedDeque(collection) => collection.size(),
            Self::CircularBuffer(collection) => collection.size(),
            Self::SparseSet(collection) => collection.size(),
            Self::LruCache(collection) => collection.size(),
            Self::BitSet(collection) => collection.size(),
            Self::SparseQueueSet(collection) => collection.size(),
            Self::SparseMap(collection) => collection.size(),
            Self::StaticDisjointSet(collection) => collection.size(),
            Self::MultiArray(collection) => collection.size(),
            Self::MultiSet(collection) => collection.size(),
            Self::MultiMap(collection) => collection.size(),
            Self::BiMap(collection) => collection.size(),
            Self::Heap(collection) => collection.size(),
            Self::FixedReverseHeap(collection) => collection.size(),
            Self::HashedArrayTree(collection) => collection.size(),
            Self::SetOps => 0,
            Self::BloomFilter(_) => 0,
            Self::BitVector(collection) => collection.size(),
            Self::Vector(collection) => collection.size(),
            Self::SuffixArray(collection) => collection.length(),
            Self::GeneralizedSuffixArray(collection) => collection.length(),
            Self::StaticIntervalTree(collection) => collection.size(),
            Self::TrieMap(collection) => collection.size(),
            Self::InvertedIndex(collection) => collection.documents.len(),
            Self::SymSpell(collection) => collection.size(),
            Self::BkTree(collection) => collection.size(),
            Self::PassjoinIndex(collection) => collection.strings.len(),
            Self::KdTree(collection) => collection.points.len(),
            Self::DefaultMap(collection) => collection.entries.len(),
            Self::DefaultWeakMap(collection) => collection.entries.len(),
            Self::FuzzyMap(collection) => collection.entries.len(),
            Self::FuzzyMultiMap(collection) => collection.size(),
            Self::VpTree(collection) => collection.size(),
            Self::ComparatorHeap(collection) => collection.items.len(),
            Self::Sort => 0,
            Self::CritBitTreeMap { tree, .. } => tree.size(),
        }
    }

    fn values(&self) -> Vec<Value> {
        match self {
            Self::Stack(collection) => collection.to_array(),
            Self::Queue(collection) => collection.to_array(),
            Self::LinkedList(collection) => collection.to_array(),
            Self::FixedStack(collection) => collection.to_array(),
            Self::FixedDeque(collection) => collection.to_array(),
            Self::CircularBuffer(collection) => collection.to_array(),
            Self::SparseSet(collection) => collection.values().map(|value| json!(value)).collect(),
            Self::LruCache(collection) => collection
                .entries()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::BitSet(collection) => collection.values().map(|value| json!(value)).collect(),
            Self::SparseQueueSet(collection) => {
                collection.values().map(|value| json!(value)).collect()
            }
            Self::SparseMap(collection) => collection
                .entries()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::StaticDisjointSet(collection) => collection
                .clone()
                .compile()
                .into_iter()
                .flatten()
                .map(|value| json!(value))
                .collect(),
            Self::MultiArray(collection) => collection.values().cloned().collect(),
            Self::MultiSet(collection) => collection.values(),
            Self::MultiMap(collection) => collection
                .entries()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::BiMap(collection) => collection
                .entries()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::Heap(collection) => collection.to_array(),
            Self::FixedReverseHeap(collection) => collection.to_array(),
            Self::HashedArrayTree(collection) => collection.to_array(),
            Self::SetOps => Vec::new(),
            Self::BloomFilter(collection) => {
                collection.data().iter().map(|value| json!(value)).collect()
            }
            Self::BitVector(collection) => collection.values().map(|value| json!(value)).collect(),
            Self::Vector(collection) => collection.to_array(),
            Self::SuffixArray(collection) => collection
                .array()
                .iter()
                .map(|value| json!(value))
                .collect(),
            Self::GeneralizedSuffixArray(collection) => collection
                .array()
                .iter()
                .map(|value| json!(value))
                .collect(),
            Self::StaticIntervalTree(collection) => collection
                .intervals()
                .iter()
                .map(|interval| interval.value.clone())
                .collect(),
            Self::TrieMap(collection) => collection
                .find("")
                .into_iter()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::InvertedIndex(collection) => collection.documents.clone(),
            Self::SymSpell(_) => Vec::new(),
            Self::BkTree(collection) => collection
                .values()
                .iter()
                .cloned()
                .map(Value::from)
                .collect(),
            Self::PassjoinIndex(collection) => collection
                .strings
                .iter()
                .cloned()
                .map(Value::from)
                .collect(),
            Self::KdTree(_) => Vec::new(),
            Self::DefaultMap(collection) => collection
                .entries
                .iter()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::DefaultWeakMap(collection) => collection
                .entries
                .iter()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::FuzzyMap(collection) => collection
                .entries
                .iter()
                .map(|(key, value)| json!([key, value]))
                .collect(),
            Self::FuzzyMultiMap(collection) => collection.values(),
            Self::VpTree(_) => Vec::new(),
            Self::ComparatorHeap(collection) => collection.items.clone(),
            Self::Sort => Vec::new(),
            Self::CritBitTreeMap { tree, .. } => tree
                .entries()
                .map(|(key, value)| json!([key, value]))
                .collect(),
        }
    }

    fn call(&mut self, method: &str, args: &[Value]) -> Result<Value, String> {
        match self {
            Self::Stack(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "pop" => Ok(option_result(collection.pop())),
                "peek" => Ok(option_ref_result(collection.peek())),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported stack method: {method}")),
            },
            Self::Queue(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "enqueue" => Ok(value_result(json!(collection.enqueue(argument(args, 0)?)))),
                "dequeue" => Ok(option_result(collection.dequeue())),
                "peek" => Ok(option_ref_result(collection.peek())),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported queue method: {method}")),
            },
            Self::LinkedList(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "unshift" => Ok(value_result(json!(collection.unshift(argument(args, 0)?)))),
                "shift" => Ok(option_result(collection.shift())),
                "first" | "peek" => Ok(option_ref_result(collection.first())),
                "last" => Ok(option_ref_result(collection.last())),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported linked-list method: {method}")),
            },
            Self::FixedStack(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "push" => collection
                    .push(argument(args, 0)?)
                    .map(|size| value_result(json!(size)))
                    .map_err(|error| error.to_string()),
                "pop" => Ok(option_result(collection.pop())),
                "peek" => Ok(option_ref_result(collection.peek())),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported fixed-stack method: {method}")),
            },
            Self::FixedDeque(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "push" => collection
                    .push(argument(args, 0)?)
                    .map(|size| value_result(json!(size)))
                    .map_err(|error| error.to_string()),
                "unshift" => collection
                    .unshift(argument(args, 0)?)
                    .map(|size| value_result(json!(size)))
                    .map_err(|error| error.to_string()),
                "pop" => Ok(option_result(collection.pop())),
                "shift" => Ok(option_result(collection.shift())),
                "peekFirst" => Ok(option_ref_result(collection.peek_first())),
                "peekLast" => Ok(option_ref_result(collection.peek_last())),
                "get" => Ok(option_ref_result(collection.get(index_argument(args)?))),
                "start" => Ok(value_result(json!(collection.start()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported fixed-deque method: {method}")),
            },
            Self::CircularBuffer(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "unshift" => Ok(value_result(json!(collection.unshift(argument(args, 0)?)))),
                "pop" => Ok(option_result(collection.pop())),
                "shift" => Ok(option_result(collection.shift())),
                "peekFirst" => Ok(option_ref_result(collection.peek_first())),
                "peekLast" => Ok(option_ref_result(collection.peek_last())),
                "get" => Ok(option_ref_result(collection.get(index_argument(args)?))),
                "start" => Ok(value_result(json!(collection.start()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "toArray" | "values" => Ok(value_result(json!(collection.to_array()))),
                _ => Err(format!("unsupported circular-buffer method: {method}")),
            },
            Self::SparseSet(collection) => match method {
                "add" => {
                    collection.add(index_argument(args)?);
                    Ok(void_result())
                }
                "has" => Ok(value_result(json!(collection.has(index_argument(args)?)))),
                "delete" => Ok(value_result(json!(collection.delete(index_argument(args)?)))),
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "size" => Ok(value_result(json!(collection.size()))),
                "length" => Ok(value_result(json!(collection.length()))),
                "values" | "toArray" => {
                    Ok(value_result(json!(collection.values().collect::<Vec<_>>())))
                }
                _ => Err(format!("unsupported sparse-set method: {method}")),
            },
            Self::LruCache(collection) => match method {
                "set" => {
                    collection.set(argument(args, 0)?, argument(args, 1)?);
                    Ok(void_result())
                }
                "setpop" => Ok(option_result(collection.setpop(
                    argument(args, 0)?,
                    argument(args, 1)?,
                ).map(|result| {
                    json!({"evicted": result.evicted, "key": result.key, "value": result.value})
                }))),
                "get" => Ok(option_result(collection.get(&argument(args, 0)?))),
                "peek" => Ok(option_ref_result(collection.peek(&argument(args, 0)?))),
                "has" => Ok(value_result(json!(collection.has(&argument(args, 0)?)))),
                "delete" => Ok(value_result(json!(collection.delete(&argument(args, 0)?)))),
                "remove" => Ok(option_result(collection.remove(&argument(args, 0)?))),
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "size" => Ok(value_result(json!(collection.size()))),
                "capacity" => Ok(value_result(json!(collection.capacity()))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" | "toArray" => Ok(value_result(json!(collection
                    .entries()
                    .map(|(key, value)| (key, value))
                    .collect::<Vec<_>>()))),
                _ => Err(format!("unsupported lru-cache method: {method}")),
            },
            Self::BitSet(collection) => match method {
                "set" => {
                    let value = args.get(1).and_then(Value::as_bool).unwrap_or(true);
                    collection.set(index_argument(args)?, value);
                    Ok(void_result())
                }
                "get" => Ok(value_result(json!(collection.get(index_argument(args)?)))),
                "test" => Ok(value_result(json!(collection.test(index_argument(args)?)))),
                "reset" => {
                    collection.reset(index_argument(args)?);
                    Ok(void_result())
                }
                "flip" => {
                    collection.flip(index_argument(args)?);
                    Ok(void_result())
                }
                "rank" => Ok(value_result(json!(collection.rank(index_argument(args)?)))),
                "select" => {
                    let selected = collection.select(index_argument(args)?);
                    if selected < 0 { Ok(undefined_result()) } else { Ok(value_result(json!(selected))) }
                }
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "size" => Ok(value_result(json!(collection.size()))),
                "length" => Ok(value_result(json!(collection.length()))),
                "wordLen" => Ok(value_result(json!(collection.word_len()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().collect::<Vec<_>>()))),
                "toJson" => Ok(value_result(json!(collection.to_json()))),
                _ => Err(format!("unsupported bit-set method: {method}")),
            },
            Self::SparseQueueSet(collection) => match method {
                "enqueue" => Ok(value_result(json!(collection.enqueue(index_argument(args)?)))),
                "dequeue" => Ok(option_result(collection.dequeue().map(|value| json!(value)))),
                "has" => Ok(value_result(json!(collection.has(index_argument(args)?)))),
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "size" => Ok(value_result(json!(collection.size()))),
                "capacity" => Ok(value_result(json!(collection.capacity()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                _ => Err(format!("unsupported sparse-queue-set method: {method}")),
            },
            Self::SparseMap(collection) => match method {
                "set" => { collection.set(index_argument(args)?, argument(args, 1)?); Ok(void_result()) }
                "get" => Ok(option_ref_result(collection.get(index_argument(args)?))),
                "has" => Ok(value_result(json!(collection.has(index_argument(args)?)))),
                "delete" => Ok(value_result(json!(collection.delete(index_argument(args)?)))),
                "clear" => { collection.clear(); Ok(void_result()) }
                "size" => Ok(value_result(json!(collection.size()))),
                "length" => Ok(value_result(json!(collection.length()))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().collect::<Vec<_>>()))),
                _ => Err(format!("unsupported sparse-map method: {method}")),
            },
            Self::StaticDisjointSet(collection) => match method {
                "union" => { collection.union(index_argument(args)?, index_argument(&args[1..])?); Ok(void_result()) }
                "connected" => Ok(value_result(json!(collection.connected(index_argument(args)?, index_argument(&args[1..])?)))),
                "mapping" => Ok(value_result(json!(collection.mapping()))),
                "compile" => Ok(value_result(json!(collection.compile()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "dimension" => Ok(value_result(json!(collection.dimension()))),
                _ => Err(format!("unsupported static-disjoint-set method: {method}")),
            },
            Self::MultiArray(collection) => match method {
                "set" => { collection.set(index_argument(args)?, argument(args, 1)?); Ok(void_result()) }
                "push" => { collection.push(argument(args, 0)?); Ok(void_result()) }
                "get" => Ok(option_result(collection.get(index_argument(args)?).map(|values| json!(values)))),
                "has" => Ok(value_result(json!(collection.has(index_argument(args)?)))),
                "count" => Ok(value_result(json!(collection.count(index_argument(args)?)))),
                "clear" => { collection.clear(); Ok(void_result()) }
                "size" => Ok(value_result(json!(collection.size()))),
                "dimension" => Ok(value_result(json!(collection.dimension()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "valuesAt" => Ok(value_result(json!(collection.values_at(index_argument(args)?).collect::<Vec<_>>()))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "containers" => Ok(value_result(json!(collection.containers().collect::<Vec<_>>()))),
                "associations" => Ok(value_result(json!(collection.associations().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().collect::<Vec<_>>()))),
                _ => Err(format!("unsupported multi-array method: {method}")),
            },
            Self::MultiSet(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "add" => { collection.add(argument(args, 0)?, signed_count(args, 1)?); Ok(void_result()) }
                "set" => { collection.set(argument(args, 0)?, signed_count(args, 1)?); Ok(void_result()) }
                "has" => Ok(value_result(json!(collection.has(&argument(args, 0)?)))),
                "delete" => Ok(value_result(json!(collection.delete(&argument(args, 0)?)))),
                "remove" => { collection.remove(&argument(args, 0)?, signed_count(args, 1)?); Ok(void_result()) }
                "edit" => { collection.edit(&argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "multiplicity" => Ok(value_result(json!(collection.multiplicity(&argument(args, 0)?)))),
                "frequency" => Ok(value_result(json!(collection.frequency(&argument(args, 0)?)))),
                "top" => collection.top(index_argument(args)?).map(|items| value_result(json!(items))).map_err(|error| error.to_string()),
                "values" => Ok(value_result(json!(collection.values()))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "multiplicities" => Ok(value_result(json!(collection.multiplicities().map(|(key, count)| json!([key, count])).collect::<Vec<_>>()))),
                "forEach" => Ok(value_result(json!(collection.values().into_iter().map(|value| json!([value.clone(), value])).collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "dimension" => Ok(value_result(json!(collection.dimension()))),
                _ => Err(format!("unsupported multi-set method: {method}")),
            },
            Self::MultiMap(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "set" => { collection.set(argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "get" => Ok(option_result(collection.get(&argument(args, 0)?).map(|values| json!(values)))),
                "has" => Ok(value_result(json!(collection.has(&argument(args, 0)?)))),
                "contains" => Ok(value_result(json!(collection.contains(&argument(args, 0)?, &argument(args, 1)?)))),
                "delete" => Ok(value_result(json!(collection.delete(&argument(args, 0)?)))),
                "remove" => Ok(value_result(json!(collection.remove(&argument(args, 0)?, &argument(args, 1)?)))),
                "multiplicity" => Ok(value_result(json!(collection.multiplicity(&argument(args, 0)?)))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().map(|(key, value)| json!([key, value])).collect::<Vec<_>>()))),
                "containers" => Ok(value_result(json!(collection.containers().map(|(key, values)| json!([key, values])).collect::<Vec<_>>()))),
                "associations" | "forEachAssociation" => Ok(value_result(json!(collection.associations().map(|(key, values)| json!([key, values])).collect::<Vec<_>>()))),
                "forEach" => Ok(value_result(json!(collection.entries().map(|(key, value)| json!([value, key])).collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "dimension" => Ok(value_result(json!(collection.dimension()))),
                _ => Err(format!("unsupported multi-map method: {method}")),
            },
            Self::BiMap(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "set" => { collection.set(argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "get" => Ok(option_ref_result(collection.get(&argument(args, 0)?))),
                "inverseGet" => Ok(option_ref_result(collection.inverse_get(&argument(args, 0)?))),
                "has" => Ok(value_result(json!(collection.has(&argument(args, 0)?)))),
                "inverseHas" => Ok(value_result(json!(collection.inverse_has(&argument(args, 0)?)))),
                "delete" => Ok(value_result(json!(collection.delete(&argument(args, 0)?)))),
                "inverseDelete" => Ok(value_result(json!(collection.inverse_delete(&argument(args, 0)?)))),
                "keys" => Ok(value_result(json!(collection.keys().collect::<Vec<_>>()))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().map(|(key, value)| json!([key, value])).collect::<Vec<_>>()))),
                "inverseKeys" => Ok(value_result(json!(collection.inverse_keys().collect::<Vec<_>>()))),
                "inverseValues" => Ok(value_result(json!(collection.inverse_values().collect::<Vec<_>>()))),
                "inverseEntries" => Ok(value_result(json!(collection.inverse_entries().map(|(key, value)| json!([key, value])).collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported bi-map method: {method}")),
            },
            Self::Heap(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "peek" => Ok(option_ref_result(collection.peek())),
                "pop" => Ok(option_result(collection.pop())),
                "replace" => collection.replace(argument(args, 0)?).map(value_result).map_err(|error| error.to_string()),
                "pushpop" => Ok(value_result(collection.pushpop(argument(args, 0)?))),
                "consume" => Ok(value_result(json!(collection.consume()))),
                "toArray" => Ok(value_result(json!(collection.to_array()))),
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported heap method: {method}")),
            },
            Self::FixedReverseHeap(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "peek" => Ok(option_ref_result(collection.peek())),
                "consume" => Ok(value_result(json!(collection.consume()))),
                "toArray" => Ok(value_result(json!(collection.to_array()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "capacity" => Ok(value_result(json!(collection.capacity()))),
                _ => Err(format!("unsupported fixed-reverse-heap method: {method}")),
            },
            Self::HashedArrayTree(collection) => match method {
                "set" => collection.set(index_argument(args)?, argument(args, 1)?).then_some(void_result()).ok_or_else(|| "hashed-array-tree index out of bounds".to_owned()),
                "get" => Ok(option_ref_result(collection.get(index_argument(args)?))),
                "grow" => { collection.grow(args.first().and_then(Value::as_u64).and_then(|value| usize::try_from(value).ok()).unwrap_or_else(|| collection.capacity() + collection.block_size())); Ok(void_result()) }
                "resize" => { collection.resize(index_argument(args)?, Value::Null); Ok(void_result()) }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "pop" => Ok(option_result(collection.pop())),
                "length" => Ok(value_result(json!(collection.length()))),
                "capacity" => Ok(value_result(json!(collection.capacity()))),
                _ => Err(format!("unsupported hashed-array-tree method: {method}")),
            },
            Self::SetOps => match method {
                "intersection" => Ok(value_result(json!(set_ops::intersection(&sets_argument(args)?)?.into_iter().collect::<Vec<_>>()))),
                "union" => Ok(value_result(json!(set_ops::union(&sets_argument(args)?)?.into_iter().collect::<Vec<_>>()))),
                "difference" => Ok(value_result(json!(set_ops::difference(&set_argument(args, 0)?, &set_argument(args, 1)?).into_iter().collect::<Vec<_>>()))),
                "symmetricDifference" => Ok(value_result(json!(set_ops::symmetric_difference(&set_argument(args, 0)?, &set_argument(args, 1)?).into_iter().collect::<Vec<_>>()))),
                "isSubset" => Ok(value_result(json!(set_ops::is_subset(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                "isSuperset" => Ok(value_result(json!(set_ops::is_superset(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                "intersectionSize" => Ok(value_result(json!(set_ops::intersection_size(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                "unionSize" => Ok(value_result(json!(set_ops::union_size(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                "jaccard" => Ok(value_result(json!(set_ops::jaccard(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                "overlap" => Ok(value_result(json!(set_ops::overlap(&set_argument(args, 0)?, &set_argument(args, 1)?)))),
                _ => Err(format!("unsupported set-ops method: {method}")),
            },
            Self::BloomFilter(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "add" => { collection.add(argument(args, 0)?.as_str().ok_or("bloom-filter values must be strings")?); Ok(void_result()) }
                "test" => Ok(value_result(json!(collection.test(argument(args, 0)?.as_str().ok_or("bloom-filter values must be strings")?)))),
                "data" => Ok(value_result(json!(collection.data()))),
                "capacity" => Ok(value_result(json!(collection.capacity()))),
                "hashFunctions" => Ok(value_result(json!(collection.hashes()))),
                _ => Err(format!("unsupported bloom-filter method: {method}")),
            },
            Self::BitVector(collection) => match method {
                "set" => { collection.set(index_argument(args)?, argument(args, 1)?.as_bool().ok_or("bit-vector values must be booleans")?); Ok(void_result()) }
                "get" => Ok(value_result(json!(collection.get(index_argument(args)?)))),
                "test" => Ok(value_result(json!(collection.test(index_argument(args)?)))),
                "reset" => { collection.reset(index_argument(args)?); Ok(void_result()) }
                "flip" => { collection.flip(index_argument(args)?); Ok(void_result()) }
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?.as_bool().ok_or("bit-vector values must be booleans")?)))),
                "pop" => Ok(option_result(collection.pop().map(|value| json!(value)))),
                "reallocate" => { collection.reallocate(index_argument(args)?); Ok(void_result()) }
                "resize" => { collection.resize(index_argument(args)?); Ok(void_result()) }
                "rank" => Ok(value_result(json!(collection.rank(index_argument(args)?)))),
                "select" => Ok(value_result(json!(collection.select(index_argument(args)?)))),
                "values" => Ok(value_result(json!(collection.values().collect::<Vec<_>>()))),
                "entries" => Ok(value_result(json!(collection.entries().map(|(index, value)| json!([index, value])).collect::<Vec<_>>()))),
                "toJson" => Ok(value_result(json!(collection.to_json()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "length" => Ok(value_result(json!(collection.length()))),
                _ => Err(format!("unsupported bit-vector method: {method}")),
            },
            Self::Vector(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "set" => collection.set(index_argument(args)?, argument(args, 1)?).then_some(void_result()).ok_or_else(|| "vector index out of bounds".to_owned()),
                "get" => Ok(option_ref_result(collection.get(index_argument(args)?))),
                "push" => Ok(value_result(json!(collection.push(argument(args, 0)?)))),
                "pop" => Ok(option_result(collection.pop())),
                "reallocate" => { collection.reallocate(index_argument(args)?); Ok(void_result()) }
                "resize" => { collection.resize(index_argument(args)?, argument(args, 1).unwrap_or(Value::Null)); Ok(void_result()) }
                "values" => Ok(value_result(json!(collection.to_array()))),
                "length" => Ok(value_result(json!(collection.length()))),
                _ => Err(format!("unsupported vector method: {method}")),
            },
            Self::SuffixArray(collection) => match method {
                "length" => Ok(value_result(json!(collection.length()))),
                "array" => Ok(value_result(json!(collection.array()))),
                _ => Err(format!("unsupported suffix-array method: {method}")),
            },
            Self::GeneralizedSuffixArray(collection) => match method {
                "length" => Ok(value_result(json!(collection.length()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "array" => Ok(value_result(json!(collection.array()))),
                "longestCommonSubsequence" => Ok(value_result(json!(collection.longest_common_subsequence()))),
                _ => Err(format!("unsupported generalized-suffix-array method: {method}")),
            },
            Self::StaticIntervalTree(collection) => match method {
                "size" => Ok(value_result(json!(collection.size()))),
                "height" => Ok(value_result(json!(collection.height()))),
                "queryPoint" => Ok(value_result(json!(collection
                    .query_point(number_argument(args, 0)?)
                    .into_iter()
                    .map(|interval| interval.value.clone())
                    .collect::<Vec<_>>()))),
                "queryInterval" => Ok(value_result(json!(collection
                    .query_interval(number_argument(args, 0)?, number_argument(args, 1)?)
                    .into_iter()
                    .map(|interval| interval.value.clone())
                    .collect::<Vec<_>>()))),
                _ => Err(format!("unsupported static-interval-tree method: {method}")),
            },
            Self::TrieMap(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "set" => { collection.set(argument(args, 0)?.as_str().ok_or("trie-map keys must be strings")?, argument(args, 1)?); Ok(void_result()) }
                "get" => Ok(option_ref_result(collection.get(argument(args, 0)?.as_str().ok_or("trie-map keys must be strings")?))),
                "has" => Ok(value_result(json!(collection.has(argument(args, 0)?.as_str().ok_or("trie-map keys must be strings")?)))),
                "delete" => Ok(value_result(json!(collection.delete(argument(args, 0)?.as_str().ok_or("trie-map keys must be strings")?)))),
                "entries" => Ok(value_result(json!(collection.find(args.first().and_then(Value::as_str).unwrap_or(""))))),
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported trie-map method: {method}")),
            },
            Self::InvertedIndex(collection) => match method {
                "clear" => { collection.clear(); Ok(void_result()) }
                "add" => {
                    let tokens = argument(args, 1)?.as_array().cloned().ok_or("inverted-index tokens must be an array")?;
                    collection.add(argument(args, 0)?, tokens);
                    Ok(void_result())
                }
                "get" => {
                    let token_argument = argument(args, 0)?;
                    let tokens = token_argument
                        .as_array()
                        .ok_or("inverted-index tokens must be an array")?;
                    Ok(value_result(json!(collection.get(tokens))))
                }
                "documents" => Ok(value_result(json!(collection.documents))),
                "tokens" => Ok(value_result(json!(collection.postings.keys().collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(collection.documents.len()))),
                "dimension" => Ok(value_result(json!(collection.postings.len()))),
                _ => Err(format!("unsupported inverted-index method: {method}")),
            },
            Self::SymSpell(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "add" => {
                    let word = argument(args, 0)?
                        .as_str()
                        .ok_or("symspell words must be strings")?
                        .to_owned();
                    collection.add(word);
                    Ok(void_result())
                }
                "search" => {
                    let input = argument(args, 0)?;
                    let input = input.as_str().ok_or("symspell input must be a string")?;
                    Ok(value_result(json!(collection
                        .search(input)
                        .into_iter()
                        .map(|suggestion| json!({
                            "term": suggestion.term,
                            "distance": suggestion.distance,
                            "count": suggestion.count,
                        }))
                        .collect::<Vec<_>>())))
                }
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported symspell method: {method}")),
            },
            Self::BkTree(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "add" => {
                    let value = argument(args, 0)?
                        .as_str()
                        .ok_or("bk-tree items must be strings")?
                        .to_owned();
                    collection.add(value);
                    Ok(void_result())
                }
                "search" => {
                    let radius = index_argument(args)?;
                    let query = argument(args, 1)?;
                    let query = query.as_str().ok_or("bk-tree queries must be strings")?;
                    Ok(value_result(json!(collection
                        .search(query, radius)
                        .into_iter()
                        .map(|(item, distance)| json!({"item": item, "distance": distance}))
                        .collect::<Vec<_>>())))
                }
                "values" => Ok(value_result(json!(collection.values()))),
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported bk-tree method: {method}")),
            },
            Self::PassjoinIndex(collection) => match method {
                "clear" => {
                    collection.clear();
                    Ok(void_result())
                }
                "add" => {
                    let value = argument(args, 0)?
                        .as_str()
                        .ok_or("passjoin-index values must be strings")?
                        .to_owned();
                    collection.add(value);
                    Ok(void_result())
                }
                "search" => {
                    let query = argument(args, 0)?;
                    let query = query
                        .as_str()
                        .ok_or("passjoin-index queries must be strings")?;
                    Ok(value_result(json!(collection.search(query))))
                }
                "values" => Ok(value_result(json!(collection.strings))),
                "size" => Ok(value_result(json!(collection.strings.len()))),
                "comparator" => {
                    let left = argument(args, 0)?;
                    let right = argument(args, 1)?;
                    let left = left.as_str().ok_or("passjoin comparator requires strings")?;
                    let right = right.as_str().ok_or("passjoin comparator requires strings")?;
                    let value = match mnemonist::passjoin_index::comparator(left, right) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };
                    Ok(value_result(json!(value)))
                }
                "partition" => Ok(value_result(json!(mnemonist::passjoin_index::partition(
                    index_argument(args)?,
                    unsigned_argument(args, 1)?,
                )))),
                "segments" => {
                    let value = argument(args, 1)?;
                    let value = value.as_str().ok_or("passjoin segments require a string")?;
                    Ok(value_result(json!(mnemonist::passjoin_index::segments(
                        index_argument(args)?,
                        value,
                    ))))
                }
                "segmentPos" => {
                    let value = argument(args, 2)?;
                    let value = value.as_str().ok_or("passjoin segmentPos requires a string")?;
                    Ok(value_result(json!(mnemonist::passjoin_index::segment_pos(
                        index_argument(args)?,
                        unsigned_argument(args, 1)?,
                        value,
                    ))))
                }
                "multiMatchAwareInterval" => Ok(value_result(json!(
                    mnemonist::passjoin_index::multi_match_aware_interval(
                        signed_argument(args, 0)?,
                        signed_argument(args, 1)?,
                        signed_argument(args, 2)?,
                        signed_argument(args, 3)?,
                        signed_argument(args, 4)?,
                        signed_argument(args, 5)?,
                    )
                ))),
                "multiMatchAwareSubstrings" => {
                    let value = argument(args, 1)?;
                    let value = value
                        .as_str()
                        .ok_or("passjoin substrings require a string")?;
                    Ok(value_result(json!(
                        mnemonist::passjoin_index::multi_match_aware_substrings(
                            signed_argument(args, 0)?,
                            value,
                            signed_argument(args, 2)?,
                            signed_argument(args, 3)?,
                            signed_argument(args, 4)?,
                            signed_argument(args, 5)?,
                        )
                    )))
                }
                _ => Err(format!("unsupported passjoin-index method: {method}")),
            },
            Self::KdTree(collection) => match method {
                "nearest" => {
                    let k = index_argument(args)?;
                    let query = argument(args, 1)?;
                    let query = query
                        .as_array()
                        .ok_or("kd-tree queries must be coordinate arrays")?
                        .iter()
                        .map(|value| value.as_f64().ok_or("kd-tree coordinates must be numeric"))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(value_result(json!(collection.nearest(k, &query))))
                }
                "nearestLinear" => {
                    let k = index_argument(args)?;
                    let query = argument(args, 1)?;
                    let query = query.as_array().ok_or("kd-tree queries must be coordinate arrays")?
                        .iter().map(|value| value.as_f64().ok_or("kd-tree coordinates must be numeric"))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(value_result(json!(collection.nearest_linear(k, &query))))
                }
                "size" => Ok(value_result(json!(collection.points.len()))),
                _ => Err(format!("unsupported kd-tree method: {method}")),
            },
            Self::DefaultWeakMap(collection) => match method {
                "clear" => { collection.entries.clear(); Ok(void_result()) }
                "set" => { collection.entries.insert(argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "peek" => Ok(option_ref_result(collection.entries.get(&argument(args, 0)?))),
                "has" => Ok(value_result(json!(collection.entries.contains_key(&argument(args, 0)?)))),
                "delete" | "release" => Ok(value_result(json!(collection.entries.shift_remove(&argument(args, 0)?).is_some()))),
                "size" => Ok(value_result(json!(collection.entries.len()))),
                _ => Err(format!("unsupported default-weak-map method: {method}")),
            },
            Self::DefaultMap(collection) | Self::FuzzyMap(collection) => match method {
                "clear" => { collection.entries.clear(); Ok(void_result()) }
                "set" => { collection.entries.insert(argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "peek" => Ok(option_ref_result(collection.entries.get(&argument(args, 0)?))),
                "has" => Ok(value_result(json!(collection.entries.contains_key(&argument(args, 0)?)))),
                "delete" => Ok(value_result(json!(collection.entries.shift_remove(&argument(args, 0)?).is_some()))),
                "entries" => Ok(value_result(json!(collection.entries.iter().map(|(key, value)| json!([key, value])).collect::<Vec<_>>()))),
                "keys" => Ok(value_result(json!(collection.entries.keys().collect::<Vec<_>>()))),
                "values" => Ok(value_result(json!(collection.entries.values().collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(collection.entries.len()))),
                _ => Err(format!("unsupported default-map method: {method}")),
            },
            Self::FuzzyMultiMap(collection) => match method {
                "clear" => { collection.entries.clear(); Ok(void_result()) }
                "set" => { collection.set(argument(args, 0)?, argument(args, 1)?); Ok(void_result()) }
                "get" => Ok(collection.entries.get(&argument(args, 0)?).cloned().map(|value| value_result(json!(value))).unwrap_or_else(undefined_result)),
                "has" => Ok(value_result(json!(collection.entries.contains_key(&argument(args, 0)?)))),
                "values" => Ok(value_result(json!(collection.values()))),
                "size" => Ok(value_result(json!(collection.size()))),
                "dimension" => Ok(value_result(json!(collection.entries.len()))),
                _ => Err(format!("unsupported fuzzy-multi-map method: {method}")),
            },
            Self::VpTree(collection) => match method {
                "nearestNeighbors" => {
                    let k = index_argument(args)?;
                    let query = argument(args, 1)?;
                    let query = query
                        .as_str()
                        .ok_or("vp-tree queries must be strings")?;
                    Ok(value_result(json!(collection
                        .nearest_neighbors(k, query)
                        .into_iter()
                        .map(|neighbor| json!({"distance": neighbor.distance, "item": neighbor.item}))
                        .collect::<Vec<_>>())))
                }
                "neighbors" => {
                    let radius = index_argument(args)?;
                    let query = argument(args, 1)?;
                    let query = query
                        .as_str()
                        .ok_or("vp-tree queries must be strings")?;
                    Ok(value_result(json!(collection
                        .neighbors(radius, query)
                        .into_iter()
                        .map(|neighbor| json!({"distance": neighbor.distance, "item": neighbor.item}))
                        .collect::<Vec<_>>())))
                }
                "size" => Ok(value_result(json!(collection.size()))),
                _ => Err(format!("unsupported vp-tree method: {method}")),
            },
            Self::ComparatorHeap(collection) => match method {
                "clear" => {
                    collection.items.clear();
                    Ok(void_result())
                }
                "items" => Ok(value_result(json!(collection.items))),
                "pushCompared" => {
                    let comparisons = comparison_table(args, 1)?;
                    Ok(value_result(json!(collection.push(&comparisons, argument(args, 0)?)?)))
                }
                "peekCompared" => {
                    let comparisons = comparison_table(args, 0)?;
                    Ok(option_result(collection.best(&comparisons)?))
                }
                "popCompared" => {
                    let comparisons = comparison_table(args, 0)?;
                    Ok(option_result(collection.pop(&comparisons)?))
                }
                "replaceCompared" => {
                    let comparisons = comparison_table(args, 1)?;
                    Ok(value_result(collection.replace(&comparisons, argument(args, 0)?)?))
                }
                "pushpopCompared" => {
                    let comparisons = comparison_table(args, 1)?;
                    Ok(value_result(collection.pushpop(&comparisons, argument(args, 0)?)?))
                }
                "consumeCompared" => {
                    let comparisons = comparison_table(args, 0)?;
                    Ok(value_result(json!(collection.consume(&comparisons)?)))
                }
                "toArrayCompared" => {
                    let comparisons = comparison_table(args, 0)?;
                    let mut clone = ProtocolComparatorHeap { items: collection.items.clone(), max: collection.max };
                    Ok(value_result(json!(clone.consume(&comparisons)?)))
                }
                "size" => Ok(value_result(json!(collection.items.len()))),
                _ => Err(format!("unsupported comparator-heap method: {method}")),
            },
            Self::Sort => match method {
                "insertion" | "quick" => Ok(value_result(json!(sort_values(args)?))),
                "insertionIndices" => Ok(value_result(json!(sort_indices(args, false)?))),
                "quickIndices" => Ok(value_result(json!(sort_indices(args, true)?))),
                _ => Err(format!("unsupported sort method: {method}")),
            },
            Self::CritBitTreeMap { tree, capacity } => match method {
                "clear" => { tree.clear(); Ok(void_result()) }
                "set" => {
                    let key_argument = argument(args, 0)?;
                    let key = key_argument.as_str().ok_or("critbit-tree-map keys must be strings")?;
                    if !tree.has(key) && capacity.is_some_and(|capacity| tree.size() >= capacity) {
                        return Err("fixed-critbit-tree-map capacity exceeded".to_owned());
                    }
                    tree.set(key, argument(args, 1)?);
                    Ok(void_result())
                }
                "get" => Ok(option_ref_result(tree.get(argument(args, 0)?.as_str().ok_or("critbit-tree-map keys must be strings")?))),
                "has" => Ok(value_result(json!(tree.has(argument(args, 0)?.as_str().ok_or("critbit-tree-map keys must be strings")?)))),
                "delete" => Ok(value_result(json!(tree.delete(argument(args, 0)?.as_str().ok_or("critbit-tree-map keys must be strings")?)))),
                "entries" => Ok(value_result(json!(tree.entries().map(|(key, value)| json!([key, value])).collect::<Vec<_>>()))),
                "size" => Ok(value_result(json!(tree.size()))),
                _ => Err(format!("unsupported critbit-tree-map method: {method}")),
            },
        }
    }
}

fn argument(args: &[Value], index: usize) -> Result<Value, String> {
    args.get(index)
        .cloned()
        .ok_or_else(|| format!("missing argument at index {index}"))
}

fn comparison_table(args: &[Value], index: usize) -> Result<IndexMap<(Value, Value), i8>, String> {
    let entries = args
        .get(index)
        .and_then(Value::as_array)
        .ok_or("custom comparator table must be an array")?;
    let mut comparisons = IndexMap::new();
    for entry in entries {
        let entry = entry
            .as_array()
            .filter(|entry| entry.len() == 3)
            .ok_or("custom comparator entries must be [left, right, result]")?;
        let result = entry[2]
            .as_i64()
            .ok_or("custom comparator result must be numeric")?;
        comparisons.insert((entry[0].clone(), entry[1].clone()), result.signum() as i8);
    }
    Ok(comparisons)
}

fn index_argument(args: &[Value]) -> Result<usize, String> {
    unsigned_argument(args, 0)
}

fn unsigned_argument(args: &[Value], index: usize) -> Result<usize, String> {
    argument(args, index)?
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| "expected a non-negative integer index".to_owned())
}

fn signed_count(args: &[Value], index: usize) -> Result<isize, String> {
    match args.get(index) {
        None => Ok(1),
        Some(value) => value
            .as_i64()
            .and_then(|value| isize::try_from(value).ok())
            .ok_or_else(|| "expected an integer count".to_owned()),
    }
}

fn signed_argument(args: &[Value], index: usize) -> Result<isize, String> {
    argument(args, index)?
        .as_i64()
        .and_then(|value| isize::try_from(value).ok())
        .ok_or_else(|| "expected an integer argument".to_owned())
}

fn number_argument(args: &[Value], index: usize) -> Result<f64, String> {
    argument(args, index)?
        .as_f64()
        .ok_or_else(|| "expected a numeric interval boundary".to_owned())
}

fn sort_values(args: &[Value]) -> Result<Vec<Value>, String> {
    let mut values = argument(args, 0)?
        .as_array()
        .cloned()
        .ok_or("sort requires an array")?;
    let start = args.get(1).and_then(Value::as_u64).unwrap_or(0) as usize;
    let end = args
        .get(2)
        .and_then(Value::as_u64)
        .unwrap_or(values.len() as u64) as usize;
    if start > end || end > values.len() {
        return Err("sort range is out of bounds".to_owned());
    }
    values[start..end].sort_by(mnemonist::compare::default_compare);
    Ok(values)
}

fn sort_indices(args: &[Value], reverse_ties: bool) -> Result<Vec<usize>, String> {
    let values = argument(args, 0)?
        .as_array()
        .cloned()
        .ok_or("sort requires an array")?;
    let mut indices = argument(args, 1)?
        .as_array()
        .ok_or("sort requires an index array")?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .and_then(|value| usize::try_from(value).ok())
                .ok_or("sort indices must be non-negative integers")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let start = args.get(2).and_then(Value::as_u64).unwrap_or(0) as usize;
    let end = args
        .get(3)
        .and_then(Value::as_u64)
        .unwrap_or(indices.len() as u64) as usize;
    if start > end || end > indices.len() || indices.iter().any(|index| *index >= values.len()) {
        return Err("sort range is out of bounds".to_owned());
    }
    indices[start..end].sort_by(|left, right| {
        let order = mnemonist::compare::default_compare(&values[*left], &values[*right]);
        if reverse_ties && order == std::cmp::Ordering::Equal {
            right.cmp(left)
        } else {
            order
        }
    });
    Ok(indices)
}

fn set_argument(args: &[Value], index: usize) -> Result<IndexSet<Value>, String> {
    argument(args, index)?
        .as_array()
        .map(|values| values.iter().cloned().collect())
        .ok_or_else(|| "expected an array of set values".to_owned())
}

fn sets_argument(args: &[Value]) -> Result<Vec<IndexSet<Value>>, String> {
    argument(args, 0)?
        .as_array()
        .ok_or_else(|| "expected an array of sets".to_owned())?
        .iter()
        .map(|set| {
            set.as_array()
                .map(|values| values.iter().cloned().collect())
                .ok_or_else(|| "expected an array of set values".to_owned())
        })
        .collect()
}

fn value_result(value: Value) -> Value {
    json!({"kind": "value", "value": value})
}

fn option_result(value: Option<Value>) -> Value {
    value.map(value_result).unwrap_or_else(undefined_result)
}

fn option_ref_result(value: Option<&Value>) -> Value {
    value
        .cloned()
        .map(value_result)
        .unwrap_or_else(undefined_result)
}

fn undefined_result() -> Value {
    json!({"kind": "undefined"})
}

fn void_result() -> Value {
    json!({"kind": "void"})
}

fn success(id: Value, result: Value, size: Option<usize>) -> Value {
    let mut response = json!({"id": id, "ok": true, "result": result});
    if let Some(size) = size {
        response["size"] = json!(size);
    }
    response
}

fn failure(id: Value, message: String) -> Value {
    json!({"id": id, "ok": false, "error": message})
}

fn capacity(args: &[Value]) -> Result<usize, String> {
    args.first()
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| "fixed-size collections require a positive integer capacity".to_owned())
}

fn length(args: &[Value]) -> Result<usize, String> {
    args.first()
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| "collections require a non-negative integer length".to_owned())
}

fn create(kind: &str, args: &[Value]) -> Result<Collection, String> {
    match kind {
        "stack" => Ok(Collection::Stack(Stack::new())),
        "queue" => Ok(Collection::Queue(Queue::new())),
        "linked-list" => Ok(Collection::LinkedList(LinkedList::new())),
        "fixed-stack" => FixedStack::new(capacity(args)?)
            .map(Collection::FixedStack)
            .map_err(|error| error.to_string()),
        "fixed-deque" => FixedDeque::new(capacity(args)?)
            .map(Collection::FixedDeque)
            .map_err(|error| error.to_string()),
        "circular-buffer" => CircularBuffer::new(capacity(args)?)
            .map(Collection::CircularBuffer)
            .map_err(|error| error.to_string()),
        "sparse-set" => Ok(Collection::SparseSet(SparseSet::new(length(args)?))),
        "lru-cache" => LruCache::new(capacity(args)?)
            .map(Collection::LruCache)
            .map_err(|error| error.to_string()),
        "bit-set" => Ok(Collection::BitSet(BitSet::new(length(args)?))),
        "sparse-queue-set" => Ok(Collection::SparseQueueSet(SparseQueueSet::new(length(
            args,
        )?))),
        "sparse-map" => Ok(Collection::SparseMap(SparseMap::new(length(args)?))),
        "static-disjoint-set" => Ok(Collection::StaticDisjointSet(StaticDisjointSet::new(
            length(args)?,
        ))),
        "multi-array" => Ok(Collection::MultiArray(MultiArray::new(length(args)?))),
        "multi-set" => Ok(Collection::MultiSet(MultiSet::new())),
        "multi-map" => Ok(Collection::MultiMap(MultiMap::new(
            if args.first().and_then(Value::as_bool).unwrap_or(false) {
                MultiMapContainer::Set
            } else {
                MultiMapContainer::Vec
            },
        ))),
        "bi-map" => Ok(Collection::BiMap(BiMap::new())),
        "heap" => Ok(Collection::Heap(
            if args.first().and_then(Value::as_bool).unwrap_or(false) {
                Heap::new_max()
            } else {
                Heap::new_min()
            },
        )),
        "fixed-reverse-heap" => Ok(Collection::FixedReverseHeap(FixedReverseHeap::new(
            capacity(args)?,
        ))),
        "hashed-array-tree" => Ok(Collection::HashedArrayTree(
            HashedArrayTree::with_dimensions(
                args.get(2)
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(1024),
                args.first()
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(0),
                args.get(1)
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(0),
            ),
        )),
        "set-ops" => Ok(Collection::SetOps),
        "bloom-filter" => JsBloomFilter::new(
            args.first()
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or("bloom-filter requires capacity")?,
            args.get(1).and_then(Value::as_f64).unwrap_or(0.005),
        )
        .map(Collection::BloomFilter),
        "bit-vector" => Ok(Collection::BitVector(BitVector::with_length(length(args)?))),
        "vector" => {
            let capacity = args
                .first()
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .unwrap_or(0);
            let initial_length = args
                .get(1)
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .unwrap_or(0);
            let mut vector = Vector::with_capacity(capacity);
            vector.resize(initial_length, Value::from(0));
            Ok(Collection::Vector(vector))
        }
        "suffix-array" => Ok(Collection::SuffixArray(SuffixArray::new(
            args.first()
                .and_then(Value::as_str)
                .ok_or("suffix-array requires a string")?,
        ))),
        "generalized-suffix-array" => {
            let strings = args
                .first()
                .and_then(Value::as_array)
                .ok_or("generalized-suffix-array requires an array of strings")?;
            let strings: Result<Vec<_>, _> = strings
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .ok_or("generalized-suffix-array requires strings")
                })
                .collect();
            Ok(Collection::GeneralizedSuffixArray(
                GeneralizedSuffixArray::new(&strings?),
            ))
        }
        "static-interval-tree" => {
            let intervals = args
                .first()
                .and_then(Value::as_array)
                .ok_or("static-interval-tree requires intervals")?
                .iter()
                .map(|value| {
                    let pair = value
                        .as_array()
                        .filter(|pair| pair.len() >= 2)
                        .ok_or("intervals must be [start, end] pairs")?;
                    Ok(Interval {
                        start: pair[0].as_f64().ok_or("interval start must be numeric")?,
                        end: pair[1].as_f64().ok_or("interval end must be numeric")?,
                        value: value.clone(),
                    })
                })
                .collect::<Result<Vec<_>, &str>>()?;
            Ok(Collection::StaticIntervalTree(StaticIntervalTree::new(
                intervals,
            )))
        }
        "trie-map" => Ok(Collection::TrieMap(TrieMap::new())),
        "inverted-index" => Ok(Collection::InvertedIndex(ProtocolInvertedIndex::default())),
        "symspell" => {
            let max_distance = args.first().and_then(Value::as_u64).unwrap_or(2);
            let max_distance =
                usize::try_from(max_distance).map_err(|_| "symspell maxDistance is too large")?;
            let verbosity = args.get(1).and_then(Value::as_u64).unwrap_or(2);
            let verbosity = u8::try_from(verbosity).map_err(|_| "symspell verbosity is invalid")?;
            SymSpell::with_options(max_distance, verbosity)
                .map(Collection::SymSpell)
                .map_err(str::to_owned)
        }
        "bk-tree" => Ok(Collection::BkTree(BkTree::new())),
        "passjoin-index" => Ok(Collection::PassjoinIndex(ProtocolPassjoinIndex::new(
            args.first().map_or(Ok(1), |_| capacity(args))?,
        ))),
        "kd-tree" => {
            let labels = args
                .first()
                .and_then(Value::as_array)
                .ok_or("kd-tree requires labels")?;
            let points = args
                .get(1)
                .and_then(Value::as_array)
                .ok_or("kd-tree requires points")?;
            if labels.len() != points.len() {
                return Err("kd-tree labels and points must have equal length".to_owned());
            }
            let points = points
                .iter()
                .map(|point| {
                    point
                        .as_array()
                        .ok_or("kd-tree points must be coordinate arrays")?
                        .iter()
                        .map(|value| value.as_f64().ok_or("kd-tree coordinates must be numeric"))
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Collection::KdTree(ProtocolKdTree {
                labels: labels.clone(),
                points,
            }))
        }
        "default-map" => Ok(Collection::DefaultMap(ProtocolDefaultMap::default())),
        "default-weak-map" => Ok(Collection::DefaultWeakMap(ProtocolDefaultMap::default())),
        "fuzzy-map" => Ok(Collection::FuzzyMap(ProtocolDefaultMap::default())),
        "fuzzy-multi-map" => Ok(Collection::FuzzyMultiMap(ProtocolFuzzyMultiMap {
            entries: IndexMap::new(),
            set_mode: args.first().and_then(Value::as_bool).unwrap_or(false),
        })),
        "vp-tree" => {
            let items = args
                .first()
                .and_then(Value::as_array)
                .ok_or("vp-tree requires an array of strings")?
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(str::to_owned)
                        .ok_or("vp-tree items must be strings")
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Collection::VpTree(mnemonist::vp_tree::StringVpTree::new(
                items,
            )))
        }
        "comparator-heap" => Ok(Collection::ComparatorHeap(ProtocolComparatorHeap {
            items: Vec::new(),
            max: args.first().and_then(Value::as_bool).unwrap_or(false),
        })),
        "sort" => Ok(Collection::Sort),
        "critbit-tree-map" => Ok(Collection::CritBitTreeMap {
            tree: CritBitTreeMap::new(),
            capacity: None,
        }),
        "fixed-critbit-tree-map" => Ok(Collection::CritBitTreeMap {
            tree: CritBitTreeMap::new(),
            capacity: Some(capacity(args)?),
        }),
        _ => Err(format!("unsupported collection kind: {kind}")),
    }
}

fn collection_key(id: &Value) -> Result<&str, String> {
    id.as_str()
        .ok_or_else(|| "collection requests require a string id".to_owned())
}

fn execute(request: Request, collections: &mut HashMap<String, Collection>) -> Value {
    let id = request.id.clone();
    let outcome: Result<(Value, Option<usize>), String> = (|| match request.op.as_str() {
        "hello" => Ok((
            json!({
                "kind": "value",
                "value": {
                    "protocol": "mnemonist-jsonl",
                    "version": PROTOCOL_VERSION,
                    "collections": [
                        "stack",
                        "queue",
                        "linked-list",
                        "fixed-stack",
                        "fixed-deque",
                        "circular-buffer",
                        "sparse-set",
                        "lru-cache",
                        "bit-set",
                        "sparse-queue-set",
                        "sparse-map",
                        "static-disjoint-set",
                        "multi-array",
                        "multi-set",
                        "multi-map",
                        "bi-map",
                        "heap",
                        "fixed-reverse-heap",
                        "hashed-array-tree",
                        "set-ops",
                        "bloom-filter",
                        "bit-vector",
                        "vector",
                        "suffix-array",
                        "generalized-suffix-array",
                        "static-interval-tree",
                        "trie-map",
                        "inverted-index",
                        "symspell",
                        "bk-tree",
                        "passjoin-index",
                        "kd-tree",
                        "default-map",
                        "default-weak-map",
                        "fuzzy-map",
                        "fuzzy-multi-map",
                        "vp-tree",
                        "comparator-heap",
                        "sort",
                        "critbit-tree-map",
                        "fixed-critbit-tree-map"
                    ]
                }
            }),
            None,
        )),
        "create" => {
            let kind = request.kind.as_deref().ok_or("create requires kind")?;
            let key = collection_key(&id)?;
            if collections.contains_key(key) {
                Err(format!("collection already exists: {id}"))
            } else {
                let collection = create(kind, &request.args)?;
                let size = collection.size();
                collections.insert(key.to_owned(), collection);
                Ok((void_result(), Some(size)))
            }
        }
        "call" => {
            let method = request.method.as_deref().ok_or("call requires method")?;
            let key = collection_key(&id)?;
            let collection = collections
                .get_mut(key)
                .ok_or_else(|| format!("unknown collection: {id}"))?;
            let result = collection.call(method, &request.args)?;
            Ok((result, Some(collection.size())))
        }
        "snapshot" => {
            let key = collection_key(&id)?;
            let collection = collections
                .get(key)
                .ok_or_else(|| format!("unknown collection: {id}"))?;
            Ok((
                value_result(json!({"kind": collection.kind(), "values": collection.values()})),
                Some(collection.size()),
            ))
        }
        "drop" => {
            let key = collection_key(&id)?;
            if collections.remove(key).is_some() {
                Ok((void_result(), None))
            } else {
                Err(format!("unknown collection: {id}"))
            }
        }
        _ => Err(format!("unsupported operation: {}", request.op)),
    })();

    match outcome {
        Ok((result, size)) => success(id, result, size),
        Err(message) => failure(id, message),
    }
}

fn print_help() {
    println!("mnemonist JSONL protocol runner");
    println!("Reads one JSON request per stdin line and writes one JSON response per stdout line.");
    println!("Supported collections: stack, queue, linked-list.");
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("--help") | Some("-h") => {
            print_help();
            return;
        }
        Some("--version") | Some("-V") => {
            println!("mnemonist-jsonl {PROTOCOL_VERSION}");
            return;
        }
        Some(argument) => {
            eprintln!("unknown argument: {argument}");
            std::process::exit(2);
        }
        None => {}
    }

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut collections = HashMap::new();

    for line in stdin.lock().lines() {
        let response = match line {
            Ok(line) if line.trim().is_empty() => continue,
            Ok(line) => match serde_json::from_str::<Request>(&line) {
                Ok(request) => execute(request, &mut collections),
                Err(error) => failure(Value::Null, format!("invalid request: {error}")),
            },
            Err(error) => failure(Value::Null, format!("stdin read failure: {error}")),
        };

        if serde_json::to_writer(&mut output, &response).is_err() || writeln!(output).is_err() {
            break;
        }
        if output.flush().is_err() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        id: Value,
        op: &str,
        kind: Option<&str>,
        method: Option<&str>,
        args: Vec<Value>,
    ) -> Request {
        Request {
            id,
            op: op.to_owned(),
            kind: kind.map(str::to_owned),
            method: method.map(str::to_owned),
            args,
        }
    }

    #[test]
    fn protocol_round_trip_preserves_json_null() {
        let mut collections = HashMap::new();

        let created = execute(
            request(json!("stack"), "create", Some("stack"), None, vec![]),
            &mut collections,
        );
        assert_eq!(created["ok"], json!(true));

        execute(
            request(
                json!("stack"),
                "call",
                None,
                Some("push"),
                vec![Value::Null],
            ),
            &mut collections,
        );
        let popped = execute(
            request(json!("stack"), "call", None, Some("pop"), vec![]),
            &mut collections,
        );

        assert_eq!(popped["result"], json!({"kind": "value", "value": null}));
    }

    #[test]
    fn protocol_rejects_non_string_collection_ids() {
        let response = execute(
            request(json!(1), "create", Some("queue"), None, vec![]),
            &mut HashMap::new(),
        );

        assert_eq!(response["ok"], json!(false));
        assert_eq!(
            response["error"],
            json!("collection requests require a string id")
        );
    }
}
