use mnemonist::bit_vector::BitVector;
use mnemonist::bloom_filter::BloomFilter;
use mnemonist::trie::Trie;
use mnemonist::trie_map::TrieMap;
use serde_json::json;

// ==========================================
// BitVector Tests
// ==========================================

#[test]
fn test_bitvector_create() {
    let bv = BitVector::with_length(100);
    assert_eq!(bv.length(), 100);
    assert!(bv.capacity() >= 100);
    assert_eq!(bv.size(), 0);
}

#[test]
fn test_bitvector_set_size_increments() {
    let mut bv = BitVector::with_length(10);
    assert_eq!(bv.size(), 0);
    assert_eq!(bv.set(2, true), true);
    assert_eq!(bv.size(), 1);
    assert_eq!(bv.set(5, true), true);
    assert_eq!(bv.size(), 2);
    // Setting an already set bit should not increment size
    assert_eq!(bv.set(2, true), false);
    assert_eq!(bv.size(), 2);
}

#[test]
fn test_bitvector_set_zero_decrements() {
    let mut bv = BitVector::with_length(10);
    bv.set(2, true);
    bv.set(5, true);
    assert_eq!(bv.size(), 2);
    
    // Unset
    bv.set(2, false);
    assert_eq!(bv.size(), 1);
    assert_eq!(bv.get(2), 0);
    assert_eq!(bv.test(2), false);
}

#[test]
fn test_bitvector_flip() {
    let mut bv = BitVector::with_length(10);
    bv.set(2, true);
    assert_eq!(bv.size(), 1);
    
    bv.flip(2);
    assert_eq!(bv.size(), 0);
    assert_eq!(bv.get(2), 0);
    
    bv.flip(2);
    assert_eq!(bv.size(), 1);
    assert_eq!(bv.get(2), 1);
}

#[test]
fn test_bitvector_count_set_bits() {
    let mut bv = BitVector::with_length(32);
    for i in 0..32 {
        bv.set(i, true);
        assert_eq!(bv.size(), i + 1);
    }
}

#[test]
fn test_bitvector_count_set_bits_when_flipping() {
    let mut bv = BitVector::with_length(32);
    for i in 0..32 {
        bv.flip(i);
        assert_eq!(bv.size(), i + 1);
    }
    for i in 0..32 {
        bv.flip(i);
        assert_eq!(bv.size(), 31 - i);
    }
}

#[test]
fn test_bitvector_reset() {
    let mut bv = BitVector::with_length(10);
    bv.set(3, true);
    bv.reset(3);
    assert_eq!(bv.size(), 0);
    assert_eq!(bv.get(3), 0);
}

#[test]
fn test_bitvector_rank() {
    let mut bv = BitVector::with_length(8010);
    for i in 0..81 {
        bv.set(i * 100, true);
    }
    
    assert_eq!(bv.rank(50), 1);
    assert_eq!(bv.rank(100), 1);
    assert_eq!(bv.rank(101), 2);
    assert_eq!(bv.rank(200), 2);
    assert_eq!(bv.rank(201), 3);
}

#[test]
fn test_bitvector_select() {
    let mut bv = BitVector::with_length(100);
    bv.set(10, true);
    bv.set(20, true);
    bv.set(30, true);
    
    assert_eq!(bv.select(0), -1);
    assert_eq!(bv.select(1), 10);
    assert_eq!(bv.select(2), 20);
    assert_eq!(bv.select(3), 30);
    assert_eq!(bv.select(4), -1);
}

#[test]
fn test_bitvector_iteration() {
    let mut bv = BitVector::with_length(5);
    bv.set(1, true);
    bv.set(3, true);
    
    let values: Vec<u8> = bv.values().collect();
    assert_eq!(values, vec![0, 1, 0, 1, 0]);
    
    let entries: Vec<(usize, u8)> = bv.entries().collect();
    assert_eq!(entries, vec![(0, 0), (1, 1), (2, 0), (3, 1), (4, 0)]);
}

#[test]
fn test_bitvector_length_divisible_by_32_iteration() {
    let mut bv = BitVector::with_length(32);
    bv.set(31, true);
    let values: Vec<u8> = bv.values().collect();
    assert_eq!(values.len(), 32);
    assert_eq!(values[31], 1);
}

#[test]
fn test_bitvector_out_of_bounds() {
    let bv = BitVector::with_length(5);
    assert_eq!(bv.get(10), 0);
    assert_eq!(bv.test(10), false);
}

#[test]
fn test_bitvector_push_pop() {
    let mut bv = BitVector::new();
    assert_eq!(bv.length(), 0);
    
    assert_eq!(bv.push(true), 1);
    assert_eq!(bv.length(), 1);
    assert_eq!(bv.size(), 1);
    
    assert_eq!(bv.push(false), 2);
    assert_eq!(bv.length(), 2);
    assert_eq!(bv.size(), 1);
    
    assert_eq!(bv.pop(), Some(0));
    assert_eq!(bv.length(), 1);
    assert_eq!(bv.size(), 1);
    
    assert_eq!(bv.pop(), Some(1));
    assert_eq!(bv.length(), 0);
    assert_eq!(bv.size(), 0);
    
    assert_eq!(bv.pop(), None);
}

#[test]
fn test_bitvector_resize_grow_reallocate() {
    let mut bv = BitVector::with_length(10);
    bv.set(9, true);
    
    bv.resize(20);
    assert_eq!(bv.length(), 20);
    assert_eq!(bv.get(9), 1);
    
    bv.grow(50);
    assert!(bv.capacity() >= 50);
    
    bv.reallocate(100);
    assert!(bv.capacity() >= 100);
}

#[test]
fn test_bitvector_to_json() {
    let mut bv = BitVector::with_length(33);
    bv.set(0, true);
    bv.set(32, true);
    let json_vec = bv.to_json();
    assert!(json_vec.len() >= 2);
    assert_eq!(json_vec[0], 1); // 1st bit
    assert_eq!(json_vec[1], 1); // 33rd bit (1st bit of 2nd block)
}

// ==========================================
// BloomFilter Tests
// ==========================================

#[test]
fn test_bloom_filter_settings() {
    let bf = BloomFilter::new(3, 0.01);
    // Based on capacity 3 and 0.01 error rate, we expect specific hashes and capacity.
    assert!(bf.hashes() > 0);
    assert!(bf.capacity() >= 3);
}

#[test]
fn test_bloom_filter_add_contains() {
    let mut bf = BloomFilter::new(100, 0.01);
    assert_eq!(bf.size(), 0);
    
    bf.add(&"hello");
    bf.add(&"world");
    
    // Note: Due to the probabilistic nature, size might not be exactly deterministic in all impls,
    // but typically size is tracked for unique added or assumed count.
    assert!(bf.contains(&"hello"));
    assert!(bf.contains(&"world"));
    assert!(!bf.contains(&"not_in_filter"));
}

#[test]
fn test_bloom_filter_false_positive_rate() {
    let mut bf = BloomFilter::new(1000, 0.05);
    for i in 0..1000 {
        bf.add(&format!("item{}", i));
    }
    
    let mut false_positives = 0;
    for i in 1000..2000 {
        if bf.contains(&format!("item{}", i)) {
            false_positives += 1;
        }
    }
    
    // We expect around 50 false positives (5%). 
    // We'll assert it's within a reasonable margin.
    assert!(false_positives < 100);
}

#[test]
fn test_bloom_filter_clear() {
    let mut bf = BloomFilter::new(100, 0.01);
    bf.add(&"hello");
    assert!(bf.contains(&"hello"));
    
    bf.clear();
    assert_eq!(bf.size(), 0);
    assert!(!bf.contains(&"hello"));
}

// ==========================================
// Trie Tests
// ==========================================

#[test]
fn test_trie_add_size() {
    let mut trie = Trie::new();
    assert_eq!(trie.size(), 0);
    
    assert_eq!(trie.add("hello"), true);
    assert_eq!(trie.size(), 1);
    
    assert_eq!(trie.add("world"), true);
    assert_eq!(trie.size(), 2);
}

#[test]
fn test_trie_duplicates() {
    let mut trie = Trie::new();
    trie.add("hello");
    assert_eq!(trie.size(), 1);
    
    assert_eq!(trie.add("hello"), false);
    assert_eq!(trie.size(), 1);
}

#[test]
fn test_trie_empty_string() {
    let mut trie = Trie::new();
    assert_eq!(trie.add(""), true);
    assert_eq!(trie.size(), 1);
    assert!(trie.has(""));
}

#[test]
fn test_trie_delete() {
    let mut trie = Trie::new();
    trie.add("hello");
    trie.add("hell");
    
    assert!(trie.has("hello"));
    assert_eq!(trie.delete("hello"), true);
    assert!(!trie.has("hello"));
    assert!(trie.has("hell")); // prefix should remain as a word
    
    assert_eq!(trie.delete("nonexistent"), false);
    assert_eq!(trie.size(), 1);
}

#[test]
fn test_trie_find_prefix() {
    let mut trie = Trie::new();
    trie.add("apple");
    trie.add("app");
    trie.add("application");
    trie.add("banana");
    
    let mut res = trie.find("app");
    res.sort();
    assert_eq!(res, vec!["app".to_string(), "apple".to_string(), "application".to_string()]);
    
    let all = trie.find("");
    assert_eq!(all.len(), 4);
}

#[test]
fn test_trie_iteration() {
    let mut trie = Trie::new();
    trie.add("cat");
    trie.add("car");
    
    let mut keys: Vec<String> = trie.keys().cloned().collect();
    keys.sort();
    assert_eq!(keys, vec!["car".to_string(), "cat".to_string()]);
    
    // Per upstream mnemonist, Trie#.prefixes is an alias of Trie#.keys:
    // both enumerate the full words stored in the trie, not intermediate
    // substrings such as "c" or "ca".
    let mut prefixes: Vec<String> = trie.prefixes().cloned().collect();
    prefixes.sort();
    assert_eq!(prefixes, vec!["car".to_string(), "cat".to_string()]);
    
    let mut keys_with: Vec<String> = trie.keys_with_prefix("ca").cloned().collect();
    keys_with.sort();
    assert_eq!(keys_with, vec!["car".to_string(), "cat".to_string()]);
}

#[test]
fn test_trie_from_iter() {
    let words = vec!["dog".to_string(), "deer".to_string()];
    let trie = Trie::from_iter(words.into_iter());
    assert_eq!(trie.size(), 2);
    assert!(trie.has("dog"));
    assert!(trie.has("deer"));
}

// ==========================================
// TrieMap Tests
// ==========================================

#[test]
fn test_triemap_set_size() {
    let mut tm = TrieMap::new();
    assert_eq!(tm.size(), 0);
    
    tm.set("key1", json!(10));
    assert_eq!(tm.size(), 1);
    assert_eq!(tm.get("key1"), Some(&json!(10)));
    assert!(tm.has("key1"));
}

#[test]
fn test_triemap_overwrite() {
    let mut tm = TrieMap::new();
    tm.set("key1", json!(10));
    tm.set("key1", json!(20)); // overwrite
    
    assert_eq!(tm.size(), 1);
    assert_eq!(tm.get("key1"), Some(&json!(20)));
}

#[test]
fn test_triemap_update() {
    let mut tm = TrieMap::new();
    tm.set("counter", json!(1));
    tm.update("counter", |v| {
        let current = v.and_then(|val| val.as_i64()).unwrap_or(0);
        json!(current + 1)
    });
    
    assert_eq!(tm.get("counter"), Some(&json!(2)));
}

#[test]
fn test_triemap_delete() {
    let mut tm = TrieMap::new();
    tm.set("key1", json!("value1"));
    tm.set("key2", json!("value2"));
    
    assert_eq!(tm.delete("key1"), true);
    assert_eq!(tm.size(), 1);
    assert!(!tm.has("key1"));
    assert_eq!(tm.delete("key1"), false);
}

#[test]
fn test_triemap_find_prefix() {
    let mut tm = TrieMap::new();
    tm.set("app", json!(1));
    tm.set("apple", json!(2));
    tm.set("banana", json!(3));
    
    let mut res = tm.find("app");
    res.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].0, "app");
    assert_eq!(res[1].0, "apple");
}

#[test]
fn test_triemap_iteration() {
    let mut tm = TrieMap::new();
    tm.set("a", json!(1));
    tm.set("b", json!(2));
    
    let mut entries: Vec<(String, serde_json::Value)> = tm.entries().map(|(k, v)| (k.clone(), v.clone())).collect();
    entries.sort_by(|x, y| x.0.cmp(&y.0));
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].0, "a");
    
    let values: Vec<serde_json::Value> = tm.values().cloned().collect();
    // values might not be sorted depending on iter implementation, but we know what's in there
    assert!(values.contains(&json!(1)));
    assert!(values.contains(&json!(2)));
}

#[test]
fn test_triemap_from_iter() {
    let pairs = vec![
        ("one".to_string(), json!(1)),
        ("two".to_string(), json!(2))
    ];
    let tm = TrieMap::from_iter(pairs.into_iter());
    assert_eq!(tm.size(), 2);
    assert_eq!(tm.get("one"), Some(&json!(1)));
    assert_eq!(tm.get("two"), Some(&json!(2)));
}
