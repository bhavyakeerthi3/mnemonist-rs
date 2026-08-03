//! Port of mnemonist's original Mocha fixed-stack tests (tests/original/fixed-stack.js).

use mnemonist::FixedStack;
use serde_json::json;

#[test]
fn wrong_capacity_should_error() {
    assert!(FixedStack::new(0).is_err());
}

#[test]
fn push_values() {
    let mut stack = FixedStack::new(10).unwrap();
    stack.push(json!("test")).unwrap();
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.capacity(), 10);
}

#[test]
fn exceeding_capacity_should_error() {
    let mut stack = FixedStack::new(1).unwrap();
    stack.push(json!("test")).unwrap();
    assert!(stack.push(json!("test")).is_err());
}

#[test]
fn clear_stack() {
    let mut stack = FixedStack::new(2).unwrap();
    stack.push(json!(2)).unwrap();
    stack.push(json!(3)).unwrap();
    stack.clear();
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn peek() {
    let mut stack = FixedStack::new(2).unwrap();
    assert!(stack.peek().is_none());
    stack.push(json!(1)).unwrap();
    assert_eq!(stack.peek(), Some(&json!(1)));
    stack.push(json!(2)).unwrap();
    assert_eq!(stack.peek(), Some(&json!(2)));
}

#[test]
fn pop_stack() {
    let mut stack = FixedStack::new(3).unwrap();
    stack.push(json!(1)).unwrap();
    stack.push(json!(2)).unwrap();
    stack.push(json!(3)).unwrap();
    assert_eq!(stack.pop(), Some(json!(3)));
    assert_eq!(stack.pop(), Some(json!(2)));
    assert_eq!(stack.pop(), Some(json!(1)));
    assert_eq!(stack.pop(), None);
}

#[test]
fn for_each_stack() {
    let mut stack = FixedStack::new(3).unwrap();
    stack.push(json!(1)).unwrap();
    stack.push(json!(2)).unwrap();
    stack.push(json!(3)).unwrap();

    let mut times = 0;
    stack.for_each(|item, i| {
        assert_eq!(item, &json!(3 - i));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn to_array() {
    let mut stack = FixedStack::new(3).unwrap();
    stack.push(json!(1)).unwrap();
    stack.push(json!(2)).unwrap();
    stack.push(json!(3)).unwrap();
    assert_eq!(stack.to_array(), vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn from_iter() {
    let stack = FixedStack::from_iter(vec![json!(1), json!(2), json!(3)], None).unwrap();
    assert_eq!(stack.to_array(), vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn values_iterator() {
    let stack = FixedStack::from_iter(vec![json!(1), json!(2), json!(3)], Some(45)).unwrap();
    let values: Vec<_> = stack.values().cloned().collect();
    assert_eq!(values, vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn entries_iterator() {
    let stack = FixedStack::from_iter(vec![json!(1), json!(2), json!(3)], Some(5)).unwrap();
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.capacity(), 5);
    let entries: Vec<_> = stack.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(3)), (1, json!(2)), (2, json!(1))]);
}
