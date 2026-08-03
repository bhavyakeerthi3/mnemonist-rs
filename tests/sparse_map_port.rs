use mnemonist::sparse_map::SparseMap;
use serde_json::json;

#[test]
fn test_sparse_map_methods() {
    let mut map = SparseMap::new(10);
    assert_eq!(map.length(), 10);
    assert_eq!(map.size(), 0);

    map.set(2, json!("two"));
    assert_eq!(map.size(), 1);
    assert!(map.has(2));
    assert_eq!(map.get(2), Some(&json!("two")));
    assert!(!map.has(3));
    assert_eq!(map.get(3), None);

    map.set(5, json!("five"));
    assert_eq!(map.size(), 2);
    assert!(map.has(5));
    assert_eq!(map.get(5), Some(&json!("five")));

    // update
    map.set(2, json!("two_updated"));
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(2), Some(&json!("two_updated")));

    // out of bounds
    assert!(!map.has(15));
    assert_eq!(map.get(15), None);

    // delete
    assert!(map.delete(2));
    assert_eq!(map.size(), 1);
    assert!(!map.has(2));
    assert_eq!(map.get(2), None);

    // delete non-existent
    assert!(!map.delete(2));
    assert!(!map.delete(15));

    // keys, values, entries
    map.set(7, json!("seven"));
    let mut keys: Vec<usize> = map.keys().collect();
    keys.sort();
    assert_eq!(keys, vec![5, 7]);

    let mut values: Vec<serde_json::Value> = map.values().cloned().collect();
    values.sort_by(|a, b| a.as_str().unwrap().cmp(b.as_str().unwrap()));
    assert_eq!(values, vec![json!("five"), json!("seven")]);

    let mut entries: Vec<(usize, serde_json::Value)> = map.entries().map(|(k, v)| (k, v.clone())).collect();
    entries.sort_by_key(|(k, _)| *k);
    assert_eq!(entries, vec![(5, json!("five")), (7, json!("seven"))]);

    // clear
    map.clear();
    assert_eq!(map.size(), 0);
    assert_eq!(map.length(), 10);
    assert!(!map.has(5));
    assert_eq!(map.get(5), None);
    assert_eq!(map.keys().count(), 0);
}
