//! Port of mnemonist's original LRUCache/LRUMap tests.

use mnemonist::{LruCache, LruSetPop};
use serde_json::json;

fn entries(cache: &LruCache) -> Vec<(serde_json::Value, serde_json::Value)> {
    cache
        .entries()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn values(cache: &LruCache) -> Vec<serde_json::Value> {
    cache.values().cloned().collect()
}

#[test]
fn invalid_capacity_errors() {
    assert!(LruCache::new(0).is_err());
}

#[test]
fn set_get_peek_and_eviction_order() {
    let mut cache = LruCache::new(3).unwrap();
    assert_eq!(cache.capacity(), 3);

    cache.set(json!("one"), json!(1));
    cache.set(json!("two"), json!(2));

    assert_eq!(cache.size(), 2);
    assert_eq!(
        entries(&cache),
        vec![(json!("two"), json!(2)), (json!("one"), json!(1))]
    );

    cache.set(json!("three"), json!(3));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("three"), json!(3)),
            (json!("two"), json!(2)),
            (json!("one"), json!(1))
        ]
    );

    cache.set(json!("four"), json!(4));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("four"), json!(4)),
            (json!("three"), json!(3)),
            (json!("two"), json!(2))
        ]
    );

    cache.set(json!("two"), json!(5));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("two"), json!(5)),
            (json!("four"), json!(4)),
            (json!("three"), json!(3))
        ]
    );

    assert!(cache.has(&json!("four")));
    assert!(!cache.has(&json!("one")));
    assert_eq!(cache.get(&json!("one")), None);
    assert_eq!(cache.get(&json!("four")), Some(json!(4)));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("four"), json!(4)),
            (json!("two"), json!(5)),
            (json!("three"), json!(3))
        ]
    );

    assert_eq!(cache.get(&json!("three")), Some(json!(3)));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("three"), json!(3)),
            (json!("four"), json!(4)),
            (json!("two"), json!(5))
        ]
    );

    assert_eq!(cache.peek(&json!("two")), Some(&json!(5)));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("three"), json!(3)),
            (json!("four"), json!(4)),
            (json!("two"), json!(5))
        ]
    );
}

#[test]
fn clear_and_capacity_one() {
    let mut cache = LruCache::new(3).unwrap();
    cache.set(json!("one"), json!(1));
    cache.set(json!("two"), json!(2));
    cache.set(json!("one"), json!(3));
    assert_eq!(
        entries(&cache),
        vec![(json!("one"), json!(3)), (json!("two"), json!(2))]
    );

    assert_eq!(cache.get(&json!("two")), Some(json!(2)));
    assert_eq!(
        entries(&cache),
        vec![(json!("two"), json!(2)), (json!("one"), json!(3))]
    );

    cache.clear();
    assert_eq!(cache.capacity(), 3);
    assert_eq!(cache.size(), 0);

    let mut cache = LruCache::new(1).unwrap();
    cache.set(json!("one"), json!(1));
    cache.set(json!("two"), json!(2));
    cache.set(json!("three"), json!(3));
    assert_eq!(entries(&cache), vec![(json!("three"), json!(3))]);
    assert_eq!(cache.get(&json!("one")), None);
    assert_eq!(cache.get(&json!("three")), Some(json!(3)));
}

#[test]
fn iterators_setpop_and_from_iter() {
    let mut cache = LruCache::new(3).unwrap();
    cache.set(json!("one"), json!(1));
    cache.set(json!("two"), json!(2));
    cache.set(json!("three"), json!(3));

    assert_eq!(
        cache.keys().cloned().collect::<Vec<_>>(),
        vec![json!("three"), json!("two"), json!("one")]
    );
    assert_eq!(values(&cache), vec![json!(3), json!(2), json!(1)]);

    let pop_result = cache.setpop(json!("four"), json!(4));
    assert_eq!(
        pop_result,
        Some(LruSetPop {
            evicted: true,
            key: json!("one"),
            value: json!(1)
        })
    );
    assert_eq!(values(&cache), vec![json!(4), json!(3), json!(2)]);

    let pop_result = cache.setpop(json!("three"), json!(10));
    assert_eq!(
        pop_result,
        Some(LruSetPop {
            evicted: false,
            key: json!("three"),
            value: json!(3)
        })
    );
    assert_eq!(values(&cache), vec![json!(10), json!(4), json!(2)]);

    let cache =
        LruCache::from_iter([(json!("one"), json!(1)), (json!("two"), json!(2))], None).unwrap();
    assert_eq!(
        entries(&cache),
        vec![(json!("two"), json!(2)), (json!("one"), json!(1))]
    );
}

#[test]
fn delete_and_remove_maintain_order() {
    let mut cache = LruCache::new(5).unwrap();
    cache.set(json!("one"), json!("uno"));
    cache.set(json!("two"), json!("dos"));
    cache.set(json!("three"), json!("tres"));
    cache.set(json!("four"), json!("cuatro"));
    cache.set(json!("five"), json!("cinco"));
    cache.get(&json!("one"));
    cache.set(json!("six"), json!("seis"));

    assert_eq!(
        entries(&cache),
        vec![
            (json!("six"), json!("seis")),
            (json!("one"), json!("uno")),
            (json!("five"), json!("cinco")),
            (json!("four"), json!("cuatro")),
            (json!("three"), json!("tres"))
        ]
    );

    assert!(cache.delete(&json!("five")));
    assert!(!cache.delete(&json!("not_here")));
    cache.set(json!("one"), json!("rast"));
    assert_eq!(
        entries(&cache),
        vec![
            (json!("one"), json!("rast")),
            (json!("six"), json!("seis")),
            (json!("four"), json!("cuatro")),
            (json!("three"), json!("tres"))
        ]
    );

    assert_eq!(cache.remove(&json!("six")), Some(json!("seis")));
    assert_eq!(cache.remove(&json!("not_here")), None);
    assert_eq!(
        entries(&cache),
        vec![
            (json!("one"), json!("rast")),
            (json!("four"), json!("cuatro")),
            (json!("three"), json!("tres"))
        ]
    );
}
