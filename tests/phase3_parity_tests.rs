use mnemonist::sparse_queue_set::SparseQueueSet;
use mnemonist::sparse_map::SparseMap;
use mnemonist::multi_map::{MultiMap, MultiMapContainer};
use mnemonist::multi_array::MultiArray;
use serde_json::json;

// --- SparseQueueSet Tests ---

#[test]
fn test_sparse_queue_set_basics() {
    let mut sqs = SparseQueueSet::new(5);
    assert_eq!(sqs.capacity(), 5);
    assert_eq!(sqs.length(), 5);
    assert_eq!(sqs.size(), 0);

    assert_eq!(sqs.enqueue(2), true);
    assert_eq!(sqs.enqueue(3), true);
    assert_eq!(sqs.enqueue(2), false); // duplicate
    assert_eq!(sqs.size(), 2);

    assert_eq!(sqs.has(2), true);
    assert_eq!(sqs.has(3), true);
    assert_eq!(sqs.has(4), false);
    assert_eq!(sqs.has(5), false); // out of bounds
}

#[test]
fn test_sparse_queue_set_dequeue() {
    let mut sqs = SparseQueueSet::new(5);
    sqs.enqueue(2);
    sqs.enqueue(3);
    sqs.enqueue(1);

    assert_eq!(sqs.dequeue(), Some(2));
    assert_eq!(sqs.has(2), false);
    assert_eq!(sqs.size(), 2);

    assert_eq!(sqs.dequeue(), Some(3));
    assert_eq!(sqs.dequeue(), Some(1));
    assert_eq!(sqs.dequeue(), None);
}

#[test]
fn test_sparse_queue_set_clear() {
    let mut sqs = SparseQueueSet::new(5);
    sqs.enqueue(1);
    sqs.enqueue(2);
    sqs.clear();
    assert_eq!(sqs.size(), 0);
    assert_eq!(sqs.has(1), false);
    assert_eq!(sqs.dequeue(), None);
}

#[test]
fn test_sparse_queue_set_wrap_around() {
    let mut sqs = SparseQueueSet::new(4);
    for _ in 0..13 {
        sqs.enqueue(2);
        sqs.enqueue(3);
        sqs.enqueue(1);
        sqs.dequeue();
        sqs.dequeue();
        sqs.dequeue();
    }
    assert_eq!(sqs.size(), 0);
    
    sqs.enqueue(0);
    sqs.enqueue(1);
    sqs.enqueue(2);
    sqs.enqueue(3);
    assert_eq!(sqs.size(), 4);
    assert_eq!(sqs.dequeue(), Some(0));
    assert_eq!(sqs.dequeue(), Some(1));
    assert_eq!(sqs.dequeue(), Some(2));
    assert_eq!(sqs.dequeue(), Some(3));
}

#[test]
fn test_sparse_queue_set_values() {
    let mut sqs = SparseQueueSet::new(5);
    sqs.enqueue(4);
    sqs.enqueue(1);
    sqs.enqueue(3);
    let vals: Vec<usize> = sqs.values().collect();
    assert_eq!(vals, vec![4, 1, 3]);
}

// --- SparseMap Tests ---

#[test]
fn test_sparse_map_basics() {
    let mut sm = SparseMap::new(5);
    assert_eq!(sm.length(), 5);
    assert_eq!(sm.size(), 0);

    sm.set(2, json!("two"));
    assert_eq!(sm.size(), 1);
    assert_eq!(sm.has(2), true);
    assert_eq!(sm.get(2), Some(&json!("two")));
    assert_eq!(sm.get(3), None);

    // out of bounds
    sm.set(10, json!("ten"));
    assert_eq!(sm.size(), 1);
}

#[test]
fn test_sparse_map_overwrite() {
    let mut sm = SparseMap::new(5);
    sm.set(2, json!("two"));
    sm.set(2, json!("deux"));
    assert_eq!(sm.size(), 1);
    assert_eq!(sm.get(2), Some(&json!("deux")));
}

#[test]
fn test_sparse_map_delete() {
    let mut sm = SparseMap::new(5);
    sm.set(1, json!("one"));
    sm.set(2, json!("two"));
    sm.set(3, json!("three"));
    
    assert_eq!(sm.delete(2), true);
    assert_eq!(sm.delete(2), false);
    assert_eq!(sm.size(), 2);
    assert_eq!(sm.has(2), false);

    sm.set(2, json!("two_again"));
    assert_eq!(sm.size(), 3);
    assert_eq!(sm.get(2), Some(&json!("two_again")));
}

#[test]
fn test_sparse_map_clear() {
    let mut sm = SparseMap::new(5);
    sm.set(1, json!("one"));
    sm.clear();
    assert_eq!(sm.size(), 0);
    assert_eq!(sm.has(1), false);
    assert_eq!(sm.get(1), None);
}

#[test]
fn test_sparse_map_iterators() {
    let mut sm = SparseMap::new(5);
    sm.set(2, json!("two"));
    sm.set(0, json!("zero"));

    let keys: Vec<usize> = sm.keys().collect();
    assert_eq!(keys, vec![2, 0]);

    let vals: Vec<_> = sm.values().cloned().collect();
    assert_eq!(vals, vec![json!("two"), json!("zero")]);

    let entries: Vec<_> = sm.entries().map(|(k, v)| (k, v.clone())).collect();
    assert_eq!(entries, vec![(2, json!("two")), (0, json!("zero"))]);
}

// --- MultiMap Tests ---

#[test]
fn test_multi_map_basics() {
    let mut mm = MultiMap::new(MultiMapContainer::Vec);
    assert_eq!(mm.size(), 0);
    assert_eq!(mm.dimension(), 0);

    mm.set(json!("color"), json!("red"));
    mm.set(json!("color"), json!("blue"));
    mm.set(json!("shape"), json!("circle"));

    assert_eq!(mm.size(), 3);
    assert_eq!(mm.dimension(), 2);
    assert_eq!(mm.has(&json!("color")), true);
    assert_eq!(mm.has(&json!("size")), false);
    
    assert_eq!(mm.multiplicity(&json!("color")), 2);
    assert_eq!(mm.multiplicity(&json!("shape")), 1);
    assert_eq!(mm.multiplicity(&json!("size")), 0);
}

#[test]
fn test_multi_map_contains_get() {
    let mut mm = MultiMap::new(MultiMapContainer::Vec);
    mm.set(json!("k"), json!("v1"));
    mm.set(json!("k"), json!("v2"));

    assert_eq!(mm.contains(&json!("k"), &json!("v1")), true);
    assert_eq!(mm.contains(&json!("k"), &json!("v3")), false);
    assert_eq!(mm.contains(&json!("k2"), &json!("v1")), false);

    assert_eq!(mm.get(&json!("k")), Some([json!("v1"), json!("v2")].as_slice()));
    assert_eq!(mm.get(&json!("k2")), None);
}

#[test]
fn test_multi_map_set_container() {
    let mut mm = MultiMap::new(MultiMapContainer::Set);
    mm.set(json!("k"), json!("v1"));
    mm.set(json!("k"), json!("v1")); // duplicate
    mm.set(json!("k"), json!("v2"));

    assert_eq!(mm.size(), 2);
    assert_eq!(mm.multiplicity(&json!("k")), 2);
    assert_eq!(mm.get(&json!("k")), Some([json!("v1"), json!("v2")].as_slice()));
}

#[test]
fn test_multi_map_remove_delete_clear() {
    let mut mm = MultiMap::new(MultiMapContainer::Vec);
    mm.set(json!("k"), json!("v1"));
    mm.set(json!("k"), json!("v2"));
    mm.set(json!("k2"), json!("v3"));

    assert_eq!(mm.remove(&json!("k"), &json!("v1")), true);
    assert_eq!(mm.remove(&json!("k"), &json!("v3")), false);
    assert_eq!(mm.size(), 2);

    assert_eq!(mm.delete(&json!("k")), true);
    assert_eq!(mm.delete(&json!("k")), false);
    assert_eq!(mm.size(), 1);
    assert_eq!(mm.has(&json!("k")), false);

    mm.clear();
    assert_eq!(mm.size(), 0);
    assert_eq!(mm.dimension(), 0);
}

#[test]
fn test_multi_map_iterators() {
    let mut mm = MultiMap::new(MultiMapContainer::Vec);
    mm.set(json!("k1"), json!("v1"));
    mm.set(json!("k1"), json!("v2"));
    mm.set(json!("k2"), json!("v3"));

    let keys: Vec<_> = mm.keys().cloned().collect();
    assert_eq!(keys, vec![json!("k1"), json!("k2")]);

    let vals: Vec<_> = mm.values().cloned().collect();
    assert_eq!(vals, vec![json!("v1"), json!("v2"), json!("v3")]);

    let entries: Vec<_> = mm.entries().map(|(k, v)| (k.clone(), v.clone())).collect();
    assert_eq!(entries, vec![
        (json!("k1"), json!("v1")),
        (json!("k1"), json!("v2")),
        (json!("k2"), json!("v3"))
    ]);

    let containers: Vec<_> = mm.containers().map(|(k, v)| (k.clone(), v.to_vec())).collect();
    assert_eq!(containers, vec![
        (json!("k1"), vec![json!("v1"), json!("v2")]),
        (json!("k2"), vec![json!("v3")])
    ]);
}

#[test]
fn test_multi_map_for_each() {
    let mut mm = MultiMap::new(MultiMapContainer::Vec);
    mm.set(json!("k1"), json!("v1"));
    mm.set(json!("k1"), json!("v2"));

    let mut out = vec![];
    mm.for_each(|v, k| out.push((k.clone(), v.clone())));
    assert_eq!(out, vec![
        (json!("k1"), json!("v1")),
        (json!("k1"), json!("v2"))
    ]);

    let mut assoc_out = vec![];
    mm.for_each_association(|v, k| assoc_out.push((k.clone(), v.to_vec())));
    assert_eq!(assoc_out, vec![
        (json!("k1"), vec![json!("v1"), json!("v2")])
    ]);
}

#[test]
fn test_multi_map_from() {
    let iter = vec![
        (json!("k1"), json!("v1")),
        (json!("k1"), json!("v2")),
        (json!("k2"), json!("v3"))
    ];
    let mm = MultiMap::from(iter);
    assert_eq!(mm.size(), 3);
    assert_eq!(mm.dimension(), 2);
}

// --- MultiArray Tests ---

#[test]
fn test_multi_array_basics() {
    let mut ma = MultiArray::new(3);
    assert_eq!(ma.size(), 0);
    assert_eq!(ma.dimension(), 3); // aka width

    ma.set(0, json!("v1"));
    ma.set(0, json!("v2"));
    ma.set(2, json!("v3"));

    assert_eq!(ma.size(), 3);
    assert_eq!(ma.has(0), true);
    assert_eq!(ma.has(1), false);
    assert_eq!(ma.has(2), true);
    
    assert_eq!(ma.count(0), 2);
    assert_eq!(ma.count(1), 0);
    
    assert_eq!(ma.get(0), Some([json!("v1"), json!("v2")].as_slice()));
    assert_eq!(ma.get(1), Some([].as_slice())); // Empty slice for initialized but empty vec
}

#[test]
fn test_multi_array_push() {
    let mut ma = MultiArray::new(0);
    ma.push(json!("v1"));
    ma.push(json!("v2"));
    
    assert_eq!(ma.dimension(), 2);
    assert_eq!(ma.size(), 2);
    assert_eq!(ma.get(0), Some([json!("v1")].as_slice()));
    assert_eq!(ma.get(1), Some([json!("v2")].as_slice()));
}

#[test]
fn test_multi_array_sparse_indices() {
    let mut ma = MultiArray::new(0);
    ma.set(2, json!("v3"));
    assert_eq!(ma.dimension(), 3);
    assert_eq!(ma.size(), 1);
    assert_eq!(ma.count(0), 0);
    assert_eq!(ma.count(2), 1);
}

#[test]
fn test_multi_array_clear() {
    let mut ma = MultiArray::new(2);
    ma.set(0, json!("v1"));
    ma.set(1, json!("v2"));
    ma.clear();
    assert_eq!(ma.size(), 0);
    assert_eq!(ma.dimension(), 2);
    assert_eq!(ma.count(0), 0);
}

#[test]
fn test_multi_array_iterators() {
    let mut ma = MultiArray::new(2);
    ma.set(0, json!("v1"));
    ma.set(0, json!("v2"));
    ma.set(1, json!("v3"));

    let keys: Vec<_> = ma.keys().collect();
    assert_eq!(keys, vec![0, 1]);

    let vals: Vec<_> = ma.values().cloned().collect();
    assert_eq!(vals, vec![json!("v1"), json!("v2"), json!("v3")]);

    let vals_at_0: Vec<_> = ma.values_at(0).cloned().collect();
    assert_eq!(vals_at_0, vec![json!("v2"), json!("v1")]); // values_at uses rev()

    let entries: Vec<_> = ma.entries().map(|(k, v)| (k, v.clone())).collect();
    assert_eq!(entries, vec![
        (0, json!("v2")), // rev() in entries
        (0, json!("v1")),
        (1, json!("v3"))
    ]);

    let containers: Vec<_> = ma.containers().map(|v| v.to_vec()).collect();
    assert_eq!(containers, vec![
        vec![json!("v1"), json!("v2")],
        vec![json!("v3")]
    ]);
    
    let assocs: Vec<_> = ma.associations().map(|(i, c)| (i, c.to_vec())).collect();
    assert_eq!(assocs, vec![
        (0, vec![json!("v1"), json!("v2")]),
        (1, vec![json!("v3")])
    ]);
}
