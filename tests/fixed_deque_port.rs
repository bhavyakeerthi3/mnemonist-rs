//! Port of mnemonist's original Mocha fixed-deque tests (tests/original/fixed-deque.js).

use mnemonist::FixedDeque;
use serde_json::json;

#[test]
fn wrong_capacity_should_error() {
    assert!(FixedDeque::new(0).is_err());
}

#[test]
fn push_values() {
    let mut deque = FixedDeque::new(10).unwrap();
    deque.push(json!("test")).unwrap();
    assert_eq!(deque.size(), 1);
    assert_eq!(deque.capacity(), 10);
}

#[test]
fn exceeding_capacity_should_error() {
    let mut deque = FixedDeque::new(1).unwrap();
    deque.push(json!("test")).unwrap();
    assert!(deque.push(json!("test")).is_err());
    assert!(deque.unshift(json!("test")).is_err());
}

#[test]
fn clear_deque() {
    let mut deque = FixedDeque::new(2).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();
    deque.clear();
    assert_eq!(deque.size(), 0);
    assert_eq!(deque.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn peek() {
    let mut deque = FixedDeque::new(3).unwrap();
    assert!(deque.peek_first().is_none());
    assert!(deque.peek_last().is_none());

    deque.push(json!(1)).unwrap();
    assert_eq!(deque.peek_first(), Some(&json!(1)));
    assert_eq!(deque.peek_last(), Some(&json!(1)));

    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();

    assert_eq!(deque.peek_first(), Some(&json!(1)));
    assert_eq!(deque.peek_last(), Some(&json!(3)));
    assert_eq!(deque.get(0), Some(&json!(1)));
    assert_eq!(deque.get(1), Some(&json!(2)));
    assert_eq!(deque.get(2), Some(&json!(3)));
    assert!(deque.get(3).is_none());
}

#[test]
fn pop_deque() {
    let mut deque = FixedDeque::new(3).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();

    assert_eq!(deque.pop(), Some(json!(3)));
    assert_eq!(deque.pop(), Some(json!(2)));
    assert_eq!(deque.pop(), Some(json!(1)));
    assert_eq!(deque.pop(), None);
    assert_eq!(deque.size(), 0);

    deque.push(json!(4)).unwrap();
    assert_eq!(deque.size(), 1);
    assert_eq!(deque.peek_last(), Some(&json!(4)));

    let mut deque2 = FixedDeque::new(6).unwrap();
    deque2.push(json!(1)).unwrap();
    deque2.push(json!(2)).unwrap();
    deque2.push(json!(3)).unwrap();
    deque2.unshift(json!(4)).unwrap();
    deque2.unshift(json!(5)).unwrap();
    deque2.unshift(json!(6)).unwrap();

    assert_eq!(deque2.pop(), Some(json!(3)));
    assert_eq!(deque2.size(), 5);
}

#[test]
fn shift_deque() {
    let mut deque = FixedDeque::new(3).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();

    assert_eq!(deque.shift(), Some(json!(1)));
    assert_eq!(deque.shift(), Some(json!(2)));
    assert_eq!(deque.shift(), Some(json!(3)));
    assert_eq!(deque.size(), 0);

    deque.push(json!(4)).unwrap();
    deque.push(json!(5)).unwrap();
    assert_eq!(deque.size(), 2);
    assert_eq!(deque.pop(), Some(json!(5)));
    assert_eq!(deque.shift(), Some(json!(4)));
}

#[test]
fn unshift_deque() {
    let mut deque = FixedDeque::new(6).unwrap();
    deque.push(json!(10)).unwrap();
    deque.push(json!(11)).unwrap();
    deque.push(json!(12)).unwrap();

    assert_eq!(deque.unshift(json!(13)).unwrap(), 4);
    assert_eq!(deque.unshift(json!(14)).unwrap(), 5);
    assert_eq!(deque.unshift(json!(15)).unwrap(), 6);
    assert_eq!(deque.size(), 6);
    assert_eq!(deque.start(), 3);

    assert_eq!(deque.pop(), Some(json!(12)));
    assert_eq!(deque.shift(), Some(json!(15)));
}

#[test]
fn consistent_over_time() {
    let mut deque = FixedDeque::new(3).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.pop();
    assert_eq!(deque.to_array(), vec![json!(1)]);

    deque.push(json!(3)).unwrap();
    deque.push(json!(4)).unwrap();
    assert_eq!(deque.to_array(), vec![json!(1), json!(3), json!(4)]);

    deque.shift();
    deque.shift();
    assert_eq!(deque.to_array(), vec![json!(4)]);
    deque.pop();
    assert_eq!(deque.to_array(), Vec::<serde_json::Value>::new());

    deque.push(json!(5)).unwrap();
    deque.push(json!(6)).unwrap();
    assert_eq!(deque.to_array(), vec![json!(5), json!(6)]);

    deque.shift();
    assert_eq!(deque.to_array(), vec![json!(6)]);
}

#[test]
fn for_each_deque() {
    let mut deque = FixedDeque::new(3).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();

    let mut times = 0;
    deque.for_each(|item, i| {
        assert_eq!(item, &json!(i + 1));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn to_array() {
    let mut deque = FixedDeque::new(3).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();
    assert_eq!(deque.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn from_iter() {
    let deque = FixedDeque::from_iter(vec![json!(1), json!(2), json!(3)], None).unwrap();
    assert_eq!(deque.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn values_iterator() {
    let deque = FixedDeque::from_iter(vec![json!(1), json!(2), json!(3)], Some(45)).unwrap();
    let values: Vec<_> = deque.values().cloned().collect();
    assert_eq!(values, vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn entries_iterator() {
    let deque = FixedDeque::from_iter(vec![json!(1), json!(2), json!(3)], Some(5)).unwrap();
    assert_eq!(deque.size(), 3);
    assert_eq!(deque.capacity(), 5);
    let entries: Vec<_> = deque.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(1)), (1, json!(2)), (2, json!(3))]);
}

#[test]
fn tricky_situations() {
    let mut deque = FixedDeque::new(6).unwrap();
    deque.push(json!(1)).unwrap();
    deque.push(json!(2)).unwrap();
    deque.push(json!(3)).unwrap();

    assert_eq!(deque.unshift(json!(4)).unwrap(), 4);
    assert_eq!(deque.unshift(json!(5)).unwrap(), 5);

    assert_eq!(deque.peek_first(), Some(&json!(5)));
    assert_eq!(deque.peek_last(), Some(&json!(3)));
    assert_eq!(deque.get(1), Some(&json!(4)));
    assert_eq!(deque.size(), 5);
    assert_eq!(deque.start(), 4);

    assert_eq!(deque.pop(), Some(json!(3)));
    assert_eq!(deque.shift(), Some(json!(5)));
    assert_eq!(deque.unshift(json!(5)).unwrap(), 4);
    assert_eq!(deque.peek_first(), Some(&json!(5)));
}
