use mnemonist::multi_array::MultiArray;
use serde_json::json;

#[test]
fn test_multi_array_basic() {
    let mut arr = MultiArray::new(2);
    assert_eq!(arr.dimension(), 2);
    assert_eq!(arr.size(), 0);
    
    arr.set(0, json!("a"));
    arr.set(0, json!("b"));
    arr.set(1, json!("c"));
    
    assert_eq!(arr.dimension(), 2);
    assert_eq!(arr.size(), 3);
    
    assert!(arr.has(0));
    assert!(arr.has(1));
    assert!(!arr.has(2));
    
    assert_eq!(arr.count(0), 2);
    assert_eq!(arr.count(1), 1);
    assert_eq!(arr.count(2), 0);
    
    assert_eq!(arr.get(0), Some(vec![json!("a"), json!("b")].as_slice()));
}

#[test]
fn test_multi_array_push() {
    let mut arr = MultiArray::new(1);
    arr.set(0, json!("a"));
    
    arr.push(json!("b"));
    
    assert_eq!(arr.dimension(), 2);
    assert_eq!(arr.size(), 2);
    assert_eq!(arr.get(1), Some(vec![json!("b")].as_slice()));
}

#[test]
fn test_multi_array_set_autogrow() {
    let mut arr = MultiArray::new(1);
    arr.set(3, json!("a"));
    
    assert_eq!(arr.dimension(), 4);
    assert_eq!(arr.size(), 1);
    assert_eq!(arr.get(3), Some(vec![json!("a")].as_slice()));
    assert!(!arr.has(1));
}

#[test]
fn test_multi_array_iterators() {
    let mut arr = MultiArray::new(2);
    arr.set(0, json!("a"));
    arr.set(0, json!("b"));
    arr.set(1, json!("c"));
    
    let keys: Vec<_> = arr.keys().collect();
    assert_eq!(keys, vec![0, 1]);
    
    let values: Vec<_> = arr.values().collect();
    assert_eq!(values, vec![&json!("a"), &json!("b"), &json!("c")]);
    
    let values_at: Vec<_> = arr.values_at(0).collect();
    assert_eq!(values_at, vec![&json!("b"), &json!("a")]);
    
    let entries: Vec<_> = arr.entries().collect();
    assert_eq!(entries, vec![
        (0, &json!("b")),
        (0, &json!("a")),
        (1, &json!("c"))
    ]);
    
    let containers: Vec<_> = arr.containers().collect();
    assert_eq!(containers.len(), 2);
    
    let associations: Vec<_> = arr.associations().collect();
    assert_eq!(associations.len(), 2);
}
