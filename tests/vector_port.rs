use mnemonist::vector::Vector;
use serde_json::json;

#[test]
fn test_vector_methods() {
    let mut vec = Vector::new();
    assert_eq!(vec.size(), 0);
    assert_eq!(vec.length(), 0);

    vec.push(json!(10));
    assert_eq!(vec.size(), 1);
    assert_eq!(vec.get(0), Some(&json!(10)));

    vec.push(json!(20));
    assert_eq!(vec.size(), 2);
    assert_eq!(vec.get(1), Some(&json!(20)));

    assert_eq!(vec.pop(), Some(json!(20)));
    assert_eq!(vec.size(), 1);

    assert!(vec.set(0, json!(15)));
    assert_eq!(vec.get(0), Some(&json!(15)));
    assert!(!vec.set(5, json!(100)));

    vec.push(json!(25));
    let values: Vec<serde_json::Value> = vec.values().cloned().collect();
    assert_eq!(values, vec![json!(15), json!(25)]);

    let entries: Vec<(usize, serde_json::Value)> = vec.entries().map(|(k, v)| (k, v.clone())).collect();
    assert_eq!(entries, vec![(0, json!(15)), (1, json!(25))]);

    let arr = vec.to_array();
    assert_eq!(arr, vec![json!(15), json!(25)]);

    vec.clear();
    assert_eq!(vec.size(), 0);
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_vector_initialization_and_capacity() {
    let mut vec = Vector::with_capacity(10);
    assert!(vec.capacity() >= 10);
    assert_eq!(vec.size(), 0);

    vec.push(json!(1));
    vec.reallocate(5);
    assert!(vec.capacity() >= 5);
    assert_eq!(vec.size(), 1);

    vec.grow(20);
    assert!(vec.capacity() >= 20);
    assert_eq!(vec.size(), 1);

    vec.resize(5, json!(0));
    assert_eq!(vec.size(), 5);
    assert_eq!(vec.get(4), Some(&json!(0)));
    
    vec.reallocate(2);
    assert_eq!(vec.size(), 2);
    assert!(vec.capacity() >= 2);

    let vec2 = Vector::with_initial_length(3, json!("a"));
    assert_eq!(vec2.size(), 3);
    assert_eq!(vec2.get(2), Some(&json!("a")));

    let vec3 = Vector::from_iter(vec![json!(1), json!(2), json!(3)]);
    assert_eq!(vec3.size(), 3);
    assert_eq!(vec3.get(1), Some(&json!(2)));
}
