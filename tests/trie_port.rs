use mnemonist::Trie;

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    
    trie.add("roman");
    trie.add("romanesque");
    trie.add("romanesco");
    trie.add("romulus");
    trie.add("rubicon");
    trie.add("rubicund");
    
    assert_eq!(trie.size(), 6);
    
    assert!(trie.has("roman"));
    assert!(trie.has("romanesque"));
    assert!(trie.has("romanesco"));
    assert!(trie.has("romulus"));
    assert!(trie.has("rubicon"));
    assert!(trie.has("rubicund"));
    assert!(!trie.has("rom"));
    assert!(!trie.has("roma"));
    
    let mut words = trie.find("roma");
    words.sort();
    assert_eq!(words, vec!["roman", "romanesco", "romanesque"]);
    
    let mut words2 = trie.find("rubic");
    words2.sort();
    assert_eq!(words2, vec!["rubicon", "rubicund"]);
    
    // Add existing
    trie.add("roman");
    assert_eq!(trie.size(), 6);
    
    // Delete
    assert!(trie.delete("roman"));
    assert_eq!(trie.size(), 5);
    assert!(!trie.has("roman"));
    assert!(!trie.delete("roman")); // already deleted
    
    // Add empty
    trie.add("");
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
    
    // From iter
    let from_trie = Trie::from_iter(vec!["hello", "world"]);
    assert_eq!(from_trie.size(), 2);
    assert!(from_trie.has("hello"));
}
