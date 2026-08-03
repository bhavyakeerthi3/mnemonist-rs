//! Port of mnemonist heap tests (tests/original/heap.js) — default comparator path.

use mnemonist::heap::{consume_slice, heapify_in_place, Heap};
use serde_json::json;

#[test]
fn push_and_size() {
    let mut heap = Heap::new_min();
    heap.push(json!("hello"));
    heap.push(json!("world"));
    assert_eq!(heap.size(), 2);
}

#[test]
fn peek() {
    let mut heap = Heap::new_min();
    assert!(heap.peek().is_none());

    heap.push(json!(3));
    heap.push(json!(24));
    assert_eq!(heap.peek(), Some(&json!(3)));

    heap.push(json!(1));
    assert_eq!(heap.peek(), Some(&json!(1)));
}

#[test]
fn pop_sequence() {
    let mut heap = Heap::new_min();
    heap.push(json!(3));
    heap.push(json!(34));
    heap.push(json!(1));
    heap.push(json!(2));

    assert_eq!(heap.size(), 4);
    assert_eq!(heap.pop(), Some(json!(1)));
    assert_eq!(heap.size(), 3);
    assert_eq!(heap.pop(), Some(json!(2)));
    assert_eq!(heap.pop(), Some(json!(3)));
    assert_eq!(heap.pop(), Some(json!(34)));
    assert_eq!(heap.pop(), None);
    assert_eq!(heap.size(), 0);
}

#[test]
fn replace_and_pushpop() {
    let mut heap = Heap::new_min();
    assert!(heap.replace(json!(3)).is_err());

    heap.push(json!(3));
    let popped = heap.replace(json!(56)).unwrap();
    assert_eq!(heap.size(), 1);
    assert_eq!(popped, json!(3));
    assert_eq!(heap.peek(), Some(&json!(56)));

    let mut heap = Heap::new_min();
    assert_eq!(heap.pushpop(json!(3)), json!(3));
    assert_eq!(heap.size(), 0);

    heap.push(json!(4));
    heap.push(json!(5));
    let popped = heap.pushpop(json!(6));
    assert_eq!(heap.size(), 2);
    assert_eq!(popped, json!(4));
    assert_eq!(heap.to_array(), vec![json!(5), json!(6)]);
}

#[test]
fn max_heap() {
    let mut heap = Heap::new_max();
    heap.push(json!(3));
    heap.push(json!(34));
    heap.push(json!(1));
    heap.push(json!(2));

    assert_eq!(heap.size(), 4);
    assert_eq!(heap.pop(), Some(json!(34)));
    assert_eq!(heap.pop(), Some(json!(3)));
    assert_eq!(heap.pop(), Some(json!(2)));
    assert_eq!(heap.pop(), Some(json!(1)));
}

#[test]
fn from_and_to_array() {
    let heap = Heap::from_iter([json!(23), json!(1), json!(34), json!(5)], false);
    assert_eq!(
        heap.to_array(),
        vec![json!(1), json!(5), json!(23), json!(34)]
    );

    let heap = Heap::from_iter([json!(23), json!(1), json!(34), json!(5)], true);
    assert_eq!(
        heap.to_array(),
        vec![json!(34), json!(23), json!(5), json!(1)]
    );
}

#[test]
fn from_set_like() {
    let heap = Heap::from_iter([json!(45), json!(56), json!(23)], false);
    assert_eq!(heap.size(), 3);
    assert_eq!(heap.peek(), Some(&json!(23)));
}

#[test]
fn heapify_and_consume_slice() {
    let mut array = vec![
        json!(3),
        json!(5),
        json!(1),
        json!(56),
        json!(0),
        json!(13),
        json!(4),
    ];
    heapify_in_place(&mut array);
    let sorted = consume_slice(&mut array);
    assert_eq!(
        sorted,
        vec![
            json!(0),
            json!(1),
            json!(3),
            json!(4),
            json!(5),
            json!(13),
            json!(56)
        ]
    );
}

#[test]
fn consume_instance() {
    let mut heap = Heap::new_min();
    heap.push(json!(45));
    heap.push(json!(-3));
    heap.push(json!(0));
    let array = heap.consume();
    assert_eq!(heap.size(), 0);
    assert_eq!(array, vec![json!(-3), json!(0), json!(45)]);
}

#[test]
fn nsmallest_and_nlargest() {
    let array = vec![
        json!(5),
        json!(2),
        json!(4),
        json!(8),
        json!(9),
        json!(1),
        json!(45),
        json!(134),
        json!(-34),
        json!(4),
        json!(-1),
        json!(0),
    ];

    assert_eq!(Heap::nsmallest(1, array.clone()), vec![json!(-34)]);
    assert_eq!(
        Heap::nsmallest(3, array.clone()),
        vec![json!(-34), json!(-1), json!(0)]
    );
    assert_eq!(Heap::nlargest(1, array.clone()), vec![json!(134)]);
    assert_eq!(
        Heap::nlargest(3, array.clone()),
        vec![json!(134), json!(45), json!(9)]
    );

    let unique: Vec<_> = array
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(Heap::nsmallest(1, unique.clone()), vec![json!(-34)]);
    assert_eq!(Heap::nlargest(1, unique.clone()), vec![json!(134)]);
}
