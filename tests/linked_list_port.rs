//! Port of mnemonist's original Mocha linked-list tests (tests/original/linked-list.js).

use mnemonist::LinkedList;
use serde_json::json;

#[test]
fn push_values() {
    let mut list = LinkedList::new();
    list.push(json!("test"));
    assert_eq!(list.size(), 1);
}

#[test]
fn unshift_and_push() {
    let mut list = LinkedList::new();
    list.push(json!(2));
    list.push(json!(3));
    list.unshift(json!(1));
    assert_eq!(list.size(), 3);
    assert_eq!(list.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn clear_list() {
    let mut list = LinkedList::new();
    list.push(json!(2));
    list.push(json!(3));
    list.clear();
    assert_eq!(list.size(), 0);
    assert_eq!(list.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn first_and_last() {
    let mut list = LinkedList::new();
    assert!(list.first().is_none());
    assert!(list.last().is_none());

    list.push(json!("hello"));
    assert_eq!(list.first(), Some(&json!("hello")));
    assert_eq!(list.last(), Some(&json!("hello")));

    list.push(json!("world"));
    assert_eq!(list.first(), Some(&json!("hello")));
    assert_eq!(list.last(), Some(&json!("world")));
    assert_eq!(list.first(), list.peek());
}

#[test]
fn shift_list() {
    let mut list = LinkedList::new();
    list.push(json!(1));
    list.push(json!(2));
    list.push(json!(3));

    assert_eq!(list.shift(), Some(json!(1)));
    assert_eq!(list.shift(), Some(json!(2)));
    assert_eq!(list.shift(), Some(json!(3)));
    assert_eq!(list.shift(), None);
}

#[test]
fn for_each_list() {
    let mut list = LinkedList::new();
    list.push(json!(1));
    list.push(json!(2));
    list.push(json!(3));

    let mut times = 0;
    list.for_each(|item, i| {
        assert_eq!(item, &json!(i + 1));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn from_iter() {
    let list = LinkedList::from_iter(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(list.size(), 3);
    assert_eq!(list.last(), Some(&json!(3)));
}

#[test]
fn values_iterator() {
    let list = LinkedList::from_iter(vec![json!(1), json!(2), json!(3)]);
    let values: Vec<_> = list.values().cloned().collect();
    assert_eq!(values, vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn entries_iterator() {
    let list = LinkedList::from_iter(vec![json!(1), json!(2), json!(3)]);
    let entries: Vec<_> = list.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(1)), (1, json!(2)), (2, json!(3))]);
}
