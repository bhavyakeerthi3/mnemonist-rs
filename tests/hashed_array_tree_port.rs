use mnemonist::hashed_array_tree::HashedArrayTree;
use serde_json::json;

#[test]
fn test_hashed_array_tree_methods() {
    let mut tree = HashedArrayTree::new(4);
    assert_eq!(tree.block_size(), 4);
    assert_eq!(tree.size(), 0);
    assert_eq!(tree.length(), 0);

    tree.push(json!("a"));
    tree.push(json!("b"));
    assert_eq!(tree.size(), 2);
    assert_eq!(tree.get(0), Some(&json!("a")));
    assert_eq!(tree.get(1), Some(&json!("b")));
    assert_eq!(tree.get(2), None);

    assert!(tree.set(1, json!("c")));
    assert_eq!(tree.get(1), Some(&json!("c")));
    assert!(!tree.set(5, json!("d")));

    assert_eq!(tree.pop(), Some(json!("c")));
    assert_eq!(tree.size(), 1);

    tree.push(json!("e"));
    let values: Vec<serde_json::Value> = tree.values().cloned().collect();
    assert_eq!(values, vec![json!("a"), json!("e")]);

    let arr = tree.to_array();
    assert_eq!(arr, vec![json!("a"), json!("e")]);

    tree.clear();
    assert_eq!(tree.size(), 0);
    assert_eq!(tree.pop(), None);
}

#[test]
fn test_hashed_array_tree_grow_and_resize() {
    let mut tree = HashedArrayTree::new(4);
    tree.push(json!(1));
    tree.grow(10);
    assert!(tree.capacity() >= 10);
    assert_eq!(tree.size(), 1);

    tree.resize(5, json!(0));
    assert_eq!(tree.size(), 5);
    assert_eq!(tree.get(4), Some(&json!(0)));

    tree.resize(2, json!(0));
    assert_eq!(tree.size(), 2);
    assert_eq!(tree.get(1), Some(&json!(0)));
    assert_eq!(tree.get(2), None);
}
