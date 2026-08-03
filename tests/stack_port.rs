//! Port of mnemonist's original Mocha stack tests (tests/original/stack.js).

use mnemonist::Stack;
use serde_json::json;

#[test]
fn push_values() {
    let mut stack = Stack::new();
    stack.push(json!("test"));
    assert_eq!(stack.size(), 1);
}

#[test]
fn clear_stack() {
    let mut stack = Stack::new();
    stack.push(json!(2));
    stack.push(json!(3));
    stack.clear();
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn peek() {
    let mut stack = Stack::new();
    assert!(stack.peek().is_none());

    stack.push(json!(1));
    assert_eq!(stack.peek(), Some(&json!(1)));

    stack.push(json!(2));
    assert_eq!(stack.peek(), Some(&json!(2)));
}

#[test]
fn pop_stack() {
    let mut stack = Stack::new();
    stack.push(json!(1));
    stack.push(json!(2));
    stack.push(json!(3));

    assert_eq!(stack.pop(), Some(json!(3)));
    assert_eq!(stack.pop(), Some(json!(2)));
    assert_eq!(stack.pop(), Some(json!(1)));
    assert_eq!(stack.pop(), None);
}

#[test]
fn for_each_stack() {
    let mut stack = Stack::new();
    stack.push(json!(1));
    stack.push(json!(2));
    stack.push(json!(3));

    let mut times = 0;
    stack.for_each(|item, i| {
        assert_eq!(item, &json!(3 - i));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn to_array() {
    let mut stack = Stack::new();
    stack.push(json!(1));
    stack.push(json!(2));
    stack.push(json!(3));
    assert_eq!(stack.to_array(), vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn from_iter() {
    let stack = Stack::from_iter(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(stack.to_array(), vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn of() {
    let stack = Stack::of(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(stack.to_array(), vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn values_iterator() {
    let stack = Stack::from_iter(vec![json!(1), json!(2), json!(3)]);
    let values: Vec<_> = stack.values().cloned().collect();
    assert_eq!(values, vec![json!(3), json!(2), json!(1)]);
}

#[test]
fn entries_iterator() {
    let stack = Stack::from_iter(vec![json!(1), json!(2), json!(3)]);
    let entries: Vec<_> = stack.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(3)), (1, json!(2)), (2, json!(1))]);
}
