//! Differential fuzz harness (Port Mortem template).
//!
//! Run: cargo test --test fuzz_harness
//!
//! Compares Stack/Queue behavior against reference vectors derived from
//! the original JavaScript test fixtures.

use mnemonist::{BitVector, LruCache, Queue, Stack};
use serde_json::{json, Value};

fn fuzz_stack(ops: &[(char, Option<i64>)]) -> Vec<Value> {
    let mut stack = Stack::new();
    let mut trace = Vec::new();

    for (op, val) in ops {
        match op {
            'p' => {
                stack.push(json!(val.unwrap()));
                trace.push(json!({"op": "push", "size": stack.size()}));
            }
            'o' => {
                let v = stack.pop();
                trace.push(json!({"op": "pop", "value": v, "size": stack.size()}));
            }
            'k' => {
                trace.push(json!({"op": "peek", "value": stack.peek().cloned()}));
            }
            'c' => {
                stack.clear();
                trace.push(json!({"op": "clear", "size": stack.size()}));
            }
            _ => {}
        }
    }
    trace.push(json!({"final": stack.to_array()}));
    trace
}

fn fuzz_queue(ops: &[(char, Option<i64>)]) -> Vec<Value> {
    let mut queue = Queue::new();
    let mut trace = Vec::new();

    for (op, val) in ops {
        match op {
            'e' => {
                queue.enqueue(json!(val.unwrap()));
                trace.push(json!({"op": "enqueue", "size": queue.size()}));
            }
            'd' => {
                let v = queue.dequeue();
                trace.push(json!({"op": "dequeue", "value": v, "size": queue.size()}));
            }
            'k' => {
                trace.push(json!({"op": "peek", "value": queue.peek().cloned()}));
            }
            'c' => {
                queue.clear();
                trace.push(json!({"op": "clear", "size": queue.size()}));
            }
            _ => {}
        }
    }
    trace.push(json!({"final": queue.to_array()}));
    trace
}

#[test]
fn stack_differential_smoke() {
    let ops = vec![
        ('p', Some(1)),
        ('p', Some(2)),
        ('k', None),
        ('o', None),
        ('p', Some(3)),
        ('c', None),
    ];
    let trace = fuzz_stack(&ops);
    assert!(trace.len() > 1);
    assert_eq!(trace.last().unwrap()["final"], json!([]));
}

#[test]
fn queue_differential_smoke() {
    let ops = vec![
        ('e', Some(10)),
        ('e', Some(20)),
        ('d', None),
        ('k', None),
        ('e', Some(30)),
    ];
    let trace = fuzz_queue(&ops);
    assert_eq!(trace.last().unwrap()["final"], json!([20, 30]));
}

#[test]
fn stack_queue_independent_traces() {
    // Property: stack LIFO vs queue FIFO on same push sequence
    let mut stack = Stack::new();
    let mut queue = Queue::new();
    for i in 0..5 {
        stack.push(json!(i));
        queue.enqueue(json!(i));
    }
    assert_ne!(stack.to_array(), queue.to_array());
    assert_eq!(
        stack.to_array(),
        vec![json!(4), json!(3), json!(2), json!(1), json!(0)]
    );
    assert_eq!(
        queue.to_array(),
        vec![json!(0), json!(1), json!(2), json!(3), json!(4)]
    );
}

#[test]
fn lru_matches_a_small_reference_model_over_a_mixed_trace() {
    let mut cache = LruCache::new(4).unwrap();
    let mut model: Vec<(Value, Value)> = Vec::new();

    for step in 0..128 {
        let key = json!(step % 7);
        match step % 4 {
            0 | 1 => {
                let value = json!(step);
                cache.set(key.clone(), value.clone());
                if let Some(index) = model.iter().position(|(existing, _)| existing == &key) {
                    model.remove(index);
                } else if model.len() == 4 {
                    model.pop();
                }
                model.insert(0, (key, value));
            }
            2 => {
                let expected = model
                    .iter()
                    .position(|(existing, _)| existing == &key)
                    .map(|index| model.remove(index));
                if let Some((stored_key, stored_value)) = expected.clone() {
                    model.insert(0, (stored_key, stored_value));
                }
                assert_eq!(cache.get(&key), expected.map(|(_, value)| value));
            }
            _ => {
                let expected = model
                    .iter()
                    .position(|(existing, _)| existing == &key)
                    .map(|index| model.remove(index).1);
                assert_eq!(cache.remove(&key), expected);
            }
        }

        let entries = cache
            .entries()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>();
        assert_eq!(entries, model);
        assert!(entries.len() <= cache.capacity());
        assert!(entries
            .iter()
            .enumerate()
            .all(|(index, (key, _))| entries[..index].iter().all(|(prior, _)| prior != key)));
    }
}

#[test]
fn bit_vector_rank_select_invariants_survive_mixed_updates() {
    let mut vector = BitVector::new();
    for index in 0..128 {
        vector.push(index % 3 == 0);
    }

    for step in 0..256 {
        let index = (step * 37) % vector.length();
        match step % 3 {
            0 => {
                vector.set(index, step % 2 == 0);
            }
            1 => vector.flip(index),
            _ => vector.reset(index),
        }

        let values = vector.values().collect::<Vec<_>>();
        let size = values.iter().filter(|bit| **bit == 1).count();
        assert_eq!(vector.size(), size);
        for end in 0..=values.len() {
            assert_eq!(vector.rank(end), values[..end].iter().filter(|bit| **bit == 1).count());
        }
        let selected = values
            .iter()
            .enumerate()
            .filter_map(|(index, bit)| (*bit == 1).then_some(index as isize))
            .collect::<Vec<_>>();
        assert_eq!(vector.select(0), -1);
        for (rank, index) in selected.iter().enumerate() {
            assert_eq!(vector.select(rank + 1), *index);
        }
        assert_eq!(vector.select(selected.len() + 1), -1);
    }
}
