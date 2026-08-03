use mnemonist::{
    BitVector, BloomFilter, DefaultWeakMap, FibonacciHeap, FixedCritBitTreeMap, FixedReverseHeap,
    InvertedIndex, LruCacheWithDelete, LruMapWithDelete, MultiArray, MultiMap, MultiMapContainer,
    SemiDynamicTrie, StaticIntervalTree, SuffixArray, Trie, TrieMap, Vector,
};
use serde_json::{json, Value};

#[test]
fn vector_bitvector_and_multiarray_cover_mutation_edges() {
    let vector = Vector::from_iter([json!(1), json!(2), json!(3)]);
    assert_eq!(vector.size(), 3);
    assert_eq!(
        vector
            .entries()
            .map(|(i, value)| (i, value.clone()))
            .collect::<Vec<(usize, Value)>>(),
        vec![(0, json!(1)), (1, json!(2)), (2, json!(3))]
    );

    let mut bits = BitVector::with_length(3);
    assert_eq!(bits.values().collect::<Vec<_>>(), vec![0, 0, 0]);
    assert!(bits.set(0, true));
    assert!(bits.set(2, true));
    assert_eq!(bits.rank(2), 1);
    assert_eq!(bits.pop(), Some(1));
    assert_eq!(bits.length(), 2);

    let mut array = MultiArray::new(2);
    array.set(0, json!("left"));
    array.set(1, json!("right"));
    assert_eq!(
        array
            .containers()
            .map(|container| container.to_vec())
            .collect::<Vec<_>>(),
        vec![vec![json!("left")], vec![json!("right")]]
    );
    array.clear();
    assert_eq!(array.dimension(), 2);
}

#[test]
fn map_aliases_and_multimap_delete_paths_are_covered() {
    let mut defaults = DefaultWeakMap::new(json!("factory"));
    assert_eq!(defaults.get(json!("x")), &json!("factory"));
    assert!(defaults.delete(&json!("x")));
    assert_eq!(defaults.size(), 0);

    let mut multi = MultiMap::new(MultiMapContainer::Vec);
    multi.set(json!("a"), json!(1));
    multi.set(json!("a"), json!(2));
    multi.set(json!("b"), json!(3));
    assert!(multi.delete(&json!("a")));
    assert_eq!(multi.size(), 1);
    assert_eq!(multi.dimension(), 1);
    assert_eq!(
        multi
            .containers()
            .map(|(key, values)| (key.clone(), values.to_vec()))
            .collect::<Vec<_>>(),
        vec![(json!("b"), vec![json!(3)])]
    );
}

#[test]
fn lru_with_delete_aliases_match_delete_remove_semantics() {
    let mut cache = LruCacheWithDelete::new(3).unwrap();
    cache.set(json!("one"), json!("uno"));
    cache.set(json!("two"), json!("dos"));
    cache.set(json!("three"), json!("tres"));
    assert!(cache.delete(&json!("two")));
    assert_eq!(
        cache
            .entries()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<_>>(),
        vec![
            (json!("three"), json!("tres")),
            (json!("one"), json!("uno"))
        ]
    );
    assert_eq!(cache.remove(&json!("one")), Some(json!("uno")));
    assert_eq!(cache.remove(&json!("missing")), None);

    let mut map = LruMapWithDelete::new(2).unwrap();
    map.set(json!("a"), json!(1));
    map.set(json!("b"), json!(2));
    map.get(&json!("a"));
    map.set(json!("c"), json!(3));
    assert!(!map.has(&json!("b")));
    assert_eq!(map.peek(&json!("a")), Some(&json!(1)));
}

#[test]
fn critbit_trie_and_semidynamic_alias_cover_prefix_delete() {
    let mut crit = FixedCritBitTreeMap::new();
    crit.set("abc", json!(1));
    crit.set("abd", json!(2));
    crit.set("z", json!(3));
    assert!(crit.has("abc"));
    assert_eq!(crit.get("abd"), Some(&json!(2)));
    assert!(crit.delete("z"));
    assert_eq!(
        crit.keys_with_prefix("ab").cloned().collect::<Vec<_>>(),
        vec!["abc".to_string(), "abd".to_string()]
    );

    let mut trie = Trie::from_iter(["tea", "team", "toast"]);
    assert!(trie.delete("toast"));
    assert_eq!(
        trie.find("tea"),
        vec!["tea".to_string(), "team".to_string()]
    );

    let mut semi = SemiDynamicTrie::new();
    semi.add("alpha");
    semi.add("alpine");
    assert_eq!(
        semi.find("alp"),
        vec!["alpha".to_string(), "alpine".to_string()]
    );

    let mut trie_map = TrieMap::new();
    trie_map.set("tea", json!(1));
    trie_map.set("team", json!(2));
    assert!(trie_map.delete("team"));
    assert_eq!(trie_map.find("tea"), vec![("tea".to_string(), json!(1))]);
}

#[test]
fn bloom_inverted_suffix_interval_and_heaps_cover_more_paths() {
    let mut bloom = BloomFilter::new(10, 0.05);
    bloom.add(&"alpha");
    assert!(bloom.contains(&"alpha"));
    bloom.clear();
    assert_eq!(bloom.size(), 0);

    let mut index = InvertedIndex::new();
    index.add("one", "alpha beta", json!(1));
    index.add("two", "beta gamma", json!(2));
    assert_eq!(index.get("one"), Some(&json!(1)));
    assert_eq!(index.search("beta").len(), 2);
    index.clear();
    assert_eq!(index.size(), 0);

    let suffix = SuffixArray::new("mississippi");
    assert_eq!(suffix.length(), 11);
    assert!(suffix.inspect().contains(&0));
    assert_eq!(suffix.search("issi"), vec![1, 4]);

    let intervals = StaticIntervalTree::new([
        mnemonist::Interval {
            start: 0.0,
            end: 1.0,
            value: json!("a"),
        },
        mnemonist::Interval {
            start: 2.0,
            end: 4.0,
            value: json!("b"),
        },
    ]);
    assert_eq!(
        intervals
            .query_interval(0.5, 2.5)
            .into_iter()
            .map(|interval| interval.value.clone())
            .collect::<Vec<_>>(),
        vec![json!("a"), json!("b")]
    );

    let mut fixed = FixedReverseHeap::new(2);
    fixed.push(json!(3));
    fixed.push(json!(1));
    fixed.push(json!(2));
    assert_eq!(fixed.consume(), vec![json!(1), json!(2)]);

    let mut fib = FibonacciHeap::from_iter([json!(4), json!(2), json!(3)], false);
    assert_eq!(fib.pop(), Some(json!(2)));
}
