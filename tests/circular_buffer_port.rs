//! Port of mnemonist's original Mocha circular-buffer tests (tests/original/circular-buffer.js).

use mnemonist::CircularBuffer;
use serde_json::json;

#[test]
fn wrong_capacity_should_error() {
    assert!(CircularBuffer::new(0).is_err());
}

#[test]
fn push_values() {
    let mut buffer = CircularBuffer::new(10).unwrap();
    buffer.push(json!("test"));
    assert_eq!(buffer.size(), 1);
    assert_eq!(buffer.capacity(), 10);
}

#[test]
fn wrap_on_push() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));
    buffer.push(json!(4));
    assert_eq!(buffer.to_array(), vec![json!(2), json!(3), json!(4)]);
    assert_eq!(buffer.size(), 3);

    buffer.push(json!(5));
    assert_eq!(buffer.to_array(), vec![json!(3), json!(4), json!(5)]);

    buffer.push(json!(6));
    assert_eq!(buffer.to_array(), vec![json!(4), json!(5), json!(6)]);

    buffer.push(json!(7));
    buffer.push(json!(8));
    assert_eq!(buffer.to_array(), vec![json!(6), json!(7), json!(8)]);
    assert_eq!(buffer.size(), 3);
}

#[test]
fn wrap_on_unshift() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.unshift(json!(1));
    buffer.unshift(json!(2));
    buffer.unshift(json!(3));
    buffer.unshift(json!(4));
    assert_eq!(buffer.to_array(), vec![json!(4), json!(3), json!(2)]);

    buffer.unshift(json!(5));
    assert_eq!(buffer.to_array(), vec![json!(5), json!(4), json!(3)]);

    buffer.unshift(json!(6));
    assert_eq!(buffer.to_array(), vec![json!(6), json!(5), json!(4)]);

    buffer.unshift(json!(7));
    buffer.unshift(json!(8));
    assert_eq!(buffer.to_array(), vec![json!(8), json!(7), json!(6)]);
    assert_eq!(buffer.size(), 3);
}

#[test]
fn clear_buffer() {
    let mut buffer = CircularBuffer::new(2).unwrap();
    buffer.push(json!(2));
    buffer.push(json!(3));
    buffer.clear();
    assert_eq!(buffer.size(), 0);
    assert_eq!(buffer.to_array(), Vec::<serde_json::Value>::new());
}

#[test]
fn peek() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    assert!(buffer.peek_first().is_none());
    assert!(buffer.peek_last().is_none());

    buffer.push(json!(1));
    assert_eq!(buffer.peek_first(), Some(&json!(1)));
    assert_eq!(buffer.peek_last(), Some(&json!(1)));

    buffer.push(json!(2));
    buffer.push(json!(3));
    assert_eq!(buffer.peek_first(), Some(&json!(1)));
    assert_eq!(buffer.peek_last(), Some(&json!(3)));
    assert_eq!(buffer.get(0), Some(&json!(1)));
    assert_eq!(buffer.get(1), Some(&json!(2)));
    assert_eq!(buffer.get(2), Some(&json!(3)));
    assert!(buffer.get(3).is_none());
}

#[test]
fn peek_last_issue_223() {
    let mut buffer = CircularBuffer::new(2).unwrap();
    buffer.push(json!(true));
    buffer.push(json!(true));
    buffer.push(json!(true));
    buffer.push(json!(false));
    buffer.push(json!(true));

    assert_eq!(buffer.to_array(), vec![json!(false), json!(true)]);
    assert_eq!(buffer.peek_first(), Some(&json!(false)));
    assert_eq!(buffer.peek_last(), Some(&json!(true)));
    assert_eq!(buffer.get(1), Some(&json!(true)));
}

#[test]
fn pop_buffer() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));

    assert_eq!(buffer.pop(), Some(json!(3)));
    assert_eq!(buffer.pop(), Some(json!(2)));
    assert_eq!(buffer.pop(), Some(json!(1)));
    assert_eq!(buffer.pop(), None);
    assert_eq!(buffer.size(), 0);

    buffer.push(json!(4));
    assert_eq!(buffer.size(), 1);
    assert_eq!(buffer.peek_last(), Some(&json!(4)));

    let mut buffer2 = CircularBuffer::new(6).unwrap();
    buffer2.push(json!(1));
    buffer2.push(json!(2));
    buffer2.push(json!(3));
    buffer2.unshift(json!(4));
    buffer2.unshift(json!(5));
    buffer2.unshift(json!(6));
    assert_eq!(buffer2.pop(), Some(json!(3)));
    assert_eq!(buffer2.size(), 5);
}

#[test]
fn shift_buffer() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));

    assert_eq!(buffer.shift(), Some(json!(1)));
    assert_eq!(buffer.shift(), Some(json!(2)));
    assert_eq!(buffer.shift(), Some(json!(3)));
    assert_eq!(buffer.size(), 0);

    buffer.push(json!(4));
    buffer.push(json!(5));
    assert_eq!(buffer.size(), 2);
    assert_eq!(buffer.pop(), Some(json!(5)));
    assert_eq!(buffer.shift(), Some(json!(4)));
}

#[test]
fn unshift_buffer() {
    let mut buffer = CircularBuffer::new(6).unwrap();
    buffer.push(json!(10));
    buffer.push(json!(11));
    buffer.push(json!(12));

    assert_eq!(buffer.unshift(json!(13)), 4);
    assert_eq!(buffer.unshift(json!(14)), 5);
    assert_eq!(buffer.unshift(json!(15)), 6);
    assert_eq!(buffer.size(), 6);
    assert_eq!(buffer.start(), 3);

    assert_eq!(buffer.pop(), Some(json!(12)));
    assert_eq!(buffer.shift(), Some(json!(15)));
}

#[test]
fn consistent_over_time() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.pop();
    assert_eq!(buffer.to_array(), vec![json!(1)]);

    buffer.push(json!(3));
    buffer.push(json!(4));
    assert_eq!(buffer.to_array(), vec![json!(1), json!(3), json!(4)]);

    buffer.shift();
    buffer.shift();
    assert_eq!(buffer.to_array(), vec![json!(4)]);
    buffer.pop();
    assert_eq!(buffer.to_array(), Vec::<serde_json::Value>::new());

    buffer.push(json!(5));
    buffer.push(json!(6));
    assert_eq!(buffer.to_array(), vec![json!(5), json!(6)]);

    buffer.shift();
    assert_eq!(buffer.to_array(), vec![json!(6)]);
}

#[test]
fn for_each_buffer() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));

    let mut times = 0;
    buffer.for_each(|item, i| {
        assert_eq!(item, &json!(i + 1));
        times += 1;
    });
    assert_eq!(times, 3);
}

#[test]
fn to_array() {
    let mut buffer = CircularBuffer::new(3).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));
    assert_eq!(buffer.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn from_iter() {
    let buffer = CircularBuffer::from_iter(vec![json!(1), json!(2), json!(3)], None).unwrap();
    assert_eq!(buffer.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn values_iterator() {
    let buffer = CircularBuffer::from_iter(vec![json!(1), json!(2), json!(3)], Some(45)).unwrap();
    let values: Vec<_> = buffer.values().cloned().collect();
    assert_eq!(values, vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn entries_iterator() {
    let buffer = CircularBuffer::from_iter(vec![json!(1), json!(2), json!(3)], Some(5)).unwrap();
    assert_eq!(buffer.size(), 3);
    assert_eq!(buffer.capacity(), 5);
    let entries: Vec<_> = buffer.entries().map(|(i, v)| (i, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(1)), (1, json!(2)), (2, json!(3))]);
}

#[test]
fn tricky_situations() {
    let mut buffer = CircularBuffer::new(6).unwrap();
    buffer.push(json!(1));
    buffer.push(json!(2));
    buffer.push(json!(3));

    assert_eq!(buffer.unshift(json!(4)), 4);
    assert_eq!(buffer.unshift(json!(5)), 5);

    assert_eq!(buffer.peek_first(), Some(&json!(5)));
    assert_eq!(buffer.peek_last(), Some(&json!(3)));
    assert_eq!(buffer.get(1), Some(&json!(4)));
    assert_eq!(buffer.size(), 5);
    assert_eq!(buffer.start(), 4);

    assert_eq!(buffer.pop(), Some(json!(3)));
    assert_eq!(buffer.shift(), Some(json!(5)));
    assert_eq!(buffer.unshift(json!(5)), 4);
    assert_eq!(buffer.peek_first(), Some(&json!(5)));
}
