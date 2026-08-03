//! Port of mnemonist's original Mocha queue tests (tests/original/queue.js).

use mnemonist::Queue;
use serde_json::json;

#[test]
fn enqueue_values() {
    let mut queue = Queue::new();
    queue.enqueue(json!("test"));
    assert_eq!(queue.size(), 1);
}

#[test]
fn clear_queue() {
    let mut queue = Queue::new();
    queue.enqueue(json!(2));
    queue.enqueue(json!(3));
    queue.clear();
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn peek() {
    let mut queue = Queue::new();
    assert!(queue.peek().is_none());

    queue.enqueue(json!(1));
    assert_eq!(queue.peek(), Some(&json!(1)));

    queue.enqueue(json!(2));
    assert_eq!(queue.peek(), Some(&json!(1)));
}

#[test]
fn dequeue() {
    let mut queue = Queue::new();
    queue.enqueue(json!(1));
    queue.enqueue(json!(2));
    queue.enqueue(json!(3));

    assert_eq!(queue.dequeue(), Some(json!(1)));
    assert_eq!(queue.dequeue(), Some(json!(2)));
    assert_eq!(queue.dequeue(), Some(json!(3)));
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn for_each_queue() {
    let mut queue = Queue::new();
    queue.enqueue(json!(1));
    queue.enqueue(json!(2));
    queue.enqueue(json!(3));

    let mut times = 0;
    queue.for_each(|item, i| {
        assert_eq!(item, &json!(i + 1));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn to_array() {
    let mut queue = Queue::new();
    queue.enqueue(json!(1));
    queue.enqueue(json!(2));
    queue.enqueue(json!(3));
    assert_eq!(queue.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn from_iter() {
    let queue = Queue::from_iter(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(queue.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn of() {
    let queue = Queue::of(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(queue.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn values_iterator() {
    let queue = Queue::from_iter(vec![json!(1), json!(2), json!(3)]);
    let values: Vec<_> = queue.values().cloned().collect();
    assert_eq!(values, vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn entries_iterator() {
    let queue = Queue::from_iter(vec![json!(1), json!(2), json!(3)]);
    let entries: Vec<_> = queue.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(1)), (1, json!(2)), (2, json!(3))]);
}

#[test]
fn compaction_after_many_dequeues() {
    let mut queue = Queue::new();
    for i in 0..100 {
        queue.enqueue(json!(i));
    }
    for _ in 0..90 {
        queue.dequeue();
    }
    queue.enqueue(json!(999));
    assert_eq!(queue.size(), 11);
    assert_eq!(queue.peek(), Some(&json!(90)));
}
