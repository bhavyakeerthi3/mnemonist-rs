use mnemonist::TrieMap;
use serde_json::json;

#[test]
fn test_trie_map() {
    let mut trie = TrieMap::new();
    
    trie.set("roman", json!(1));
    trie.set("romanesque", json!(2));
    trie.set("romanesco", json!(3));
    trie.set("romulus", json!(4));
    trie.set("rubicon", json!(5));
    trie.set("rubicund", json!(6));
    
    assert_eq!(trie.size(), 6);
    
    assert!(trie.has("roman"));
    assert_eq!(trie.get("roman"), Some(&json!(1)));
    assert!(!trie.has("rom"));
    assert_eq!(trie.get("rom"), None);
    
    let mut found = trie.find("roma");
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(found, vec![
        ("roman".to_string(), json!(1)),
        ("romanesco".to_string(), json!(3)),
        ("romanesque".to_string(), json!(2)),
    ]);
    
    // Add existing
    trie.set("roman", json!(10));
    assert_eq!(trie.size(), 6);
    assert_eq!(trie.get("roman"), Some(&json!(10)));
    
    // Update
    trie.update("roman", |v| match v {
        Some(val) => json!(val.as_i64().unwrap() + 5),
        None => json!(0),
    });
    assert_eq!(trie.get("roman"), Some(&json!(15)));
    
    // Delete
    assert!(trie.delete("roman"));
    assert_eq!(trie.size(), 5);
    assert!(!trie.has("roman"));
    assert!(!trie.delete("roman")); // already deleted
    
    // Empty
    trie.set("", json!(0));
    assert_eq!(trie.size(), 6);
    assert!(trie.has(""));
    
    // Iterators
    let mut prefixes: Vec<_> = trie.prefixes().cloned().collect();
    prefixes.sort();
    let mut keys: Vec<_> = trie.keys().cloned().collect();
    keys.sort();
    assert_eq!(prefixes, keys);
    
    let mut keys_roma: Vec<_> = trie.keys_with_prefix("roma").cloned().collect();
    keys_roma.sort();
    assert_eq!(keys_roma, vec!["romanesco", "romanesque"]);
    
    let mut vals_roma: Vec<_> = trie.values_with_prefix("roma").cloned().collect();
    vals_roma.sort_by(|a, b| a.as_i64().unwrap().cmp(&b.as_i64().unwrap()));
    assert_eq!(vals_roma, vec![json!(2), json!(3)]);
    
    let mut entries_roma: Vec<_> = trie.entries_with_prefix("roma")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    entries_roma.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(entries_roma, vec![
        ("romanesco".to_string(), json!(3)),
        ("romanesque".to_string(), json!(2)),
    ]);
    
    // From iter
    let from_map = TrieMap::from_iter(vec![
        ("hello", json!(1)),
        ("world", json!(2)),
    ]);
    assert_eq!(from_map.size(), 2);
    assert_eq!(from_map.get("hello"), Some(&json!(1)));
}
