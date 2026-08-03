use mnemonist::multi_map::{MultiMap, MultiMapContainer};
use serde_json::json;

#[test]
fn test_multi_map_basic() {
    let mut map = MultiMap::new(MultiMapContainer::Vec);
    
    map.set(json!("one"), json!(1));
    map.set(json!("one"), json!(2));
    map.set(json!("two"), json!(3));
    
    assert_eq!(map.size(), 3);
    assert_eq!(map.dimension(), 2);
    
    assert!(map.has(&json!("one")));
    assert!(map.has(&json!("two")));
    assert!(!map.has(&json!("three")));
    
    assert!(map.contains(&json!("one"), &json!(1)));
    assert!(map.contains(&json!("one"), &json!(2)));
    assert!(!map.contains(&json!("one"), &json!(3)));
    
    assert_eq!(map.get(&json!("one")), Some(vec![json!(1), json!(2)].as_slice()));
    assert_eq!(map.get(&json!("two")), Some(vec![json!(3)].as_slice()));
    
    assert_eq!(map.multiplicity(&json!("one")), 2);
    assert_eq!(map.multiplicity(&json!("two")), 1);
    assert_eq!(map.multiplicity(&json!("three")), 0);
}

#[test]
fn test_multi_map_delete_remove() {
    let mut map = MultiMap::new(MultiMapContainer::Vec);
    map.set(json!("one"), json!(1));
    map.set(json!("one"), json!(2));
    
    assert!(map.remove(&json!("one"), &json!(1)));
    assert_eq!(map.size(), 1);
    assert_eq!(map.dimension(), 1);
    
    assert!(map.delete(&json!("one")));
    assert_eq!(map.size(), 0);
    assert_eq!(map.dimension(), 0);
}

#[test]
fn test_multi_map_iterators() {
    let mut map = MultiMap::new(MultiMapContainer::Vec);
    map.set(json!("one"), json!(1));
    map.set(json!("one"), json!(2));
    map.set(json!("two"), json!(3));
    
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&json!("one"), &json!("two")]);
    
    let values: Vec<_> = map.values().collect();
    assert_eq!(values, vec![&json!(1), &json!(2), &json!(3)]);
    
    let entries: Vec<_> = map.entries().collect();
    assert_eq!(entries, vec![
        (&json!("one"), &json!(1)),
        (&json!("one"), &json!(2)),
        (&json!("two"), &json!(3))
    ]);
    
    let containers: Vec<_> = map.containers().collect();
    assert_eq!(containers.len(), 2);
    
    let associations: Vec<_> = map.associations().collect();
    assert_eq!(associations.len(), 2);
}

#[test]
fn test_multi_map_for_each() {
    let mut map = MultiMap::new(MultiMapContainer::Vec);
    map.set(json!("one"), json!(1));
    map.set(json!("one"), json!(2));
    
    let mut count = 0;
    map.for_each(|value, key| {
        assert_eq!(key, &json!("one"));
        count += 1;
        assert!(value == &json!(1) || value == &json!(2));
    });
    assert_eq!(count, 2);
    
    let mut assoc_count = 0;
    map.for_each_association(|values, key| {
        assert_eq!(key, &json!("one"));
        assert_eq!(values.len(), 2);
        assoc_count += 1;
    });
    assert_eq!(assoc_count, 1);
}

#[test]
fn test_multi_map_from() {
    let items = vec![
        (json!("one"), json!(1)),
        (json!("one"), json!(2)),
        (json!("two"), json!(3))
    ];
    let map = MultiMap::from(items);
    assert_eq!(map.size(), 3);
    assert_eq!(map.dimension(), 2);
}
