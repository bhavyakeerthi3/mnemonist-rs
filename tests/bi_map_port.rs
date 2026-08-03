//! Port of mnemonist's original BiMap tests (tests/original/bi-map.js).

use mnemonist::BiMap;
use serde_json::json;

#[test]
fn set_keys() {
    let mut map = BiMap::new();
    map.set(json!("one"), json!("hello"));
    map.set(json!("two"), json!("world"));

    assert_eq!(map.size(), 2);
    assert_eq!(map.inverse_entries().count(), 2);
}

#[test]
fn handle_constraints() {
    let mut map = BiMap::new();
    map.set(json!("one"), json!("hello"));
    map.set(json!("two"), json!("world"));

    map.set(json!("two"), json!("monde"));
    assert_eq!(map.size(), 2);

    map.set(json!("three"), json!("monde"));
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(&json!("two")), None);
    assert_eq!(map.get(&json!("three")), Some(&json!("monde")));

    let mut map = BiMap::new();
    map.set(json!("A"), json!("B"));
    map.set(json!("C"), json!("D"));
    map.set(json!("A"), json!("D"));
    assert_eq!(map.size(), 1);
    assert_eq!(map.get(&json!("A")), Some(&json!("D")));
}

#[test]
fn has_delete_clear_get() {
    let mut map = BiMap::new();
    map.set(json!("one"), json!("hello"));
    map.set(json!("two"), json!("world"));

    assert!(map.has(&json!("one")));
    assert!(!map.has(&json!("three")));
    assert_eq!(map.get(&json!("one")), Some(&json!("hello")));
    assert_eq!(map.inverse_get(&json!("hello")), Some(&json!("one")));

    assert!(map.delete(&json!("one")));
    assert_eq!(map.size(), 1);
    assert!(!map.has(&json!("one")));
    assert!(!map.inverse_has(&json!("hello")));

    map.clear();
    assert_eq!(map.size(), 0);
}

#[test]
fn iterators_keep_insertion_order() {
    let mut map = BiMap::new();
    map.set(json!("one"), json!("hello"));
    map.set(json!("two"), json!("world"));

    assert_eq!(
        map.keys().cloned().collect::<Vec<_>>(),
        vec![json!("one"), json!("two")]
    );
    assert_eq!(
        map.values().cloned().collect::<Vec<_>>(),
        vec![json!("hello"), json!("world")]
    );
    assert_eq!(
        map.entries()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>(),
        vec![
            (json!("one"), json!("hello")),
            (json!("two"), json!("world"))
        ]
    );
    assert_eq!(
        map.inverse_entries()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>(),
        vec![
            (json!("hello"), json!("one")),
            (json!("world"), json!("two"))
        ]
    );
}

#[test]
fn from_iter() {
    let map = BiMap::from_iter([
        (json!("one"), json!("hello")),
        (json!("two"), json!("world")),
    ]);

    assert_eq!(map.size(), 2);
    assert_eq!(map.get(&json!("one")), Some(&json!("hello")));
}
