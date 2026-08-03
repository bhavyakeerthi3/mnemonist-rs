use mnemonist::{
    sort, BitVector, BkTree, BloomFilter, CritBitTreeMap, DefaultMap, FixedReverseHeap, FuzzyMap,
    FuzzyMultiMap, HashedArrayTree, InvertedIndex, KdTree, MultiArray, MultiMap, MultiMapContainer,
    PassjoinIndex, SparseMap, SparseQueueSet, StaticIntervalTree, SuffixArray, SymSpell, Trie,
    TrieMap, Vector, VpTree,
};
use serde_json::{json, Value};

#[test]
fn vector_and_hashed_array_tree_cover_dynamic_array_surface() {
    let mut vector = Vector::with_capacity(2);
    assert_eq!(vector.push(json!("a")), 1);
    assert_eq!(vector.push(json!("b")), 2);
    assert_eq!(vector.get(1), Some(&json!("b")));
    assert!(vector.set(1, json!("c")));
    assert_eq!(vector.to_array(), vec![json!("a"), json!("c")]);
    assert_eq!(vector.pop(), Some(json!("c")));

    let mut hat = HashedArrayTree::new(4);
    hat.push(json!(1));
    hat.push(json!(2));
    hat.grow(8);
    assert_eq!(hat.block_size(), 4);
    assert_eq!(hat.get(0), Some(&json!(1)));
    assert!(hat.set(1, json!(3)));
    assert_eq!(hat.to_array(), vec![json!(1), json!(3)]);
    hat.resize(4, json!(0));
    assert_eq!(hat.to_array(), vec![json!(1), json!(3), json!(0), json!(0)]);
}

#[test]
fn default_map_multimap_and_multiarray_preserve_order() {
    let mut defaults = DefaultMap::new(json!([]));
    assert_eq!(defaults.get(json!("missing")), &json!([]));
    defaults.set(json!("one"), json!(1));
    assert!(defaults.has(&json!("one")));
    assert_eq!(
        defaults.keys().cloned().collect::<Vec<_>>(),
        vec![json!("missing"), json!("one")]
    );

    let mut multi = MultiMap::new(MultiMapContainer::Vec);
    multi.set(json!("k"), json!(1));
    multi.set(json!("k"), json!(2));
    multi.set(json!("z"), json!(3));
    assert_eq!(multi.size(), 3);
    assert_eq!(multi.dimension(), 2);
    assert!(multi.contains(&json!("k"), &json!(2)));
    assert_eq!(multi.get(&json!("k")).unwrap(), &[json!(1), json!(2)]);
    assert!(multi.remove(&json!("k"), &json!(1)));
    assert_eq!(multi.get(&json!("k")).unwrap(), &[json!(2)]);

    let mut unique = MultiMap::new(MultiMapContainer::Set);
    unique.set(json!("k"), json!(1));
    unique.set(json!("k"), json!(1));
    assert_eq!(unique.dimension(), 1);

    let mut array = MultiArray::new(3);
    array.set(0, json!("a"));
    array.set(2, json!("z"));
    assert_eq!(array.width(), 3);
    assert_eq!(array.dimension(), 3);
    assert_eq!(array.get(2).unwrap(), &[json!("z")]);
}

#[test]
fn sparse_map_queue_set_bit_vector_and_fixed_reverse_heap_work() {
    let mut map = SparseMap::new(10);
    map.set(3, json!("three"));
    map.set(6, json!("six"));
    map.set(3, json!("trois"));
    assert_eq!(map.size(), 2);
    assert_eq!(map.get(3), Some(&json!("trois")));
    assert!(map.delete(6));
    assert_eq!(map.keys().collect::<Vec<_>>(), vec![3]);

    let mut queue = SparseQueueSet::new(10);
    assert!(queue.enqueue(3));
    assert!(queue.enqueue(6));
    assert!(!queue.enqueue(3));
    assert_eq!(queue.values().collect::<Vec<_>>(), vec![3, 6]);
    assert_eq!(queue.dequeue(), Some(3));
    assert!(!queue.has(3));
    assert!(queue.has(6));

    let mut bits = BitVector::new();
    bits.push(true);
    bits.push(false);
    bits.push(true);
    assert_eq!(bits.size(), 2);
    assert_eq!(bits.rank(3), 2);
    assert_eq!(bits.select(2), 2);
    assert!(bits.set(1, true));
    assert_eq!(bits.values().collect::<Vec<_>>(), vec![1, 1, 1]);

    let mut heap = FixedReverseHeap::new(3);
    heap.push(json!(5));
    heap.push(json!(1));
    heap.push(json!(3));
    heap.push(json!(2));
    assert_eq!(heap.size(), 3);
    assert_eq!(heap.to_array(), vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn bloom_bk_fuzzy_passjoin_and_symspell_cover_string_matching() {
    let mut bloom = BloomFilter::new(100, 0.01);
    bloom.add(&"mnemonist");
    assert!(bloom.contains(&"mnemonist"));
    assert!(bloom.capacity() > 0);

    let mut bk = BkTree::new();
    bk.add("book");
    bk.add("back");
    bk.add("books");
    assert_eq!(
        bk.search("book", 1),
        vec![("book".into(), 0), ("books".into(), 1)]
    );

    let mut fuzzy = FuzzyMap::new();
    fuzzy.set("book", json!(1));
    fuzzy.set("back", json!(2));
    assert_eq!(fuzzy.search("books", 1)[0].1, &json!(1));

    let mut fuzzy_multi = FuzzyMultiMap::new();
    fuzzy_multi.set("book", json!("a"));
    fuzzy_multi.set("book", json!("b"));
    assert_eq!(fuzzy_multi.dimension(), 1);
    assert_eq!(
        fuzzy_multi.search("books", 1)[0].1,
        &[json!("a"), json!("b")]
    );

    let mut passjoin = PassjoinIndex::new();
    passjoin.add("book", json!("book"));
    passjoin.add("back", json!("back"));
    assert_eq!(passjoin.search("books", 1)[0].1, &json!("book"));

    let mut sym = SymSpell::new();
    sym.add("hello");
    sym.add("yellow");
    assert_eq!(sym.lookup("hell", 1), vec![("hello".into(), 1)]);
}

#[test]
fn trie_critbit_suffix_and_sort_helpers_are_usable() {
    let mut trie = Trie::new();
    trie.add("car");
    trie.add("cart");
    trie.add("dog");
    assert!(trie.has("car"));
    assert_eq!(
        trie.find("car"),
        vec!["car".to_string(), "cart".to_string()]
    );

    let mut trie_map = TrieMap::new();
    trie_map.set("car", json!(1));
    trie_map.set("cart", json!(2));
    trie_map.update("car", |current| {
        json!(current.unwrap().as_i64().unwrap() + 10)
    });
    assert_eq!(trie_map.get("car"), Some(&json!(11)));
    assert_eq!(trie_map.find("car").len(), 2);

    let mut crit = CritBitTreeMap::new();
    crit.set("alpha", json!(1));
    crit.set("alpine", json!(2));
    crit.set("beta", json!(3));
    assert_eq!(
        crit.keys_with_prefix("alp").cloned().collect::<Vec<_>>(),
        vec!["alpha".to_string(), "alpine".to_string()]
    );

    let suffix = SuffixArray::new("banana");
    assert_eq!(suffix.search("ana"), vec![1, 3]);
    assert_eq!(suffix.suffix(0), Some("a"));

    let sorted = sort::sorted(vec![json!(3), json!(1), json!(2)]);
    assert_eq!(sorted, vec![json!(1), json!(2), json!(3)]);
}

#[test]
fn inverted_interval_and_spatial_indexes_return_expected_matches() {
    let mut index = InvertedIndex::new();
    index.add("a", "rust data structures", json!("doc-a"));
    index.add("b", "javascript data structures", json!("doc-b"));
    index.add("c", "rust compiler", json!("doc-c"));
    assert_eq!(index.search("rust data"), vec![&json!("doc-a")]);

    let tree = StaticIntervalTree::new([
        mnemonist::Interval {
            start: 0.0,
            end: 5.0,
            value: json!("a"),
        },
        mnemonist::Interval {
            start: 3.0,
            end: 10.0,
            value: json!("b"),
        },
    ]);
    assert_eq!(
        tree.query_point(4.0)
            .into_iter()
            .map(|interval| interval.value.clone())
            .collect::<Vec<_>>(),
        vec![json!("a"), json!("b")]
    );

    let kd = KdTree::new([
        (vec![0.0, 0.0], json!("origin")),
        (vec![10.0, 0.0], json!("far")),
    ]);
    assert_eq!(kd.nearest(&[1.0, 1.0], 1)[0].0, &json!("origin"));

    let vp = VpTree::new([
        (vec![0.0, 0.0], json!("origin")),
        (vec![10.0, 0.0], json!("far")),
    ]);
    assert_eq!(vp.nearest(&[9.0, 0.0], 1)[0].0, &json!("far"));
}

#[test]
fn aliases_for_lru_and_fibonacci_are_available() {
    let mut lru = mnemonist::LruMap::new(2).unwrap();
    lru.set(json!("a"), json!(1));
    lru.set(json!("b"), json!(2));
    lru.set(json!("c"), json!(3));
    assert_eq!(
        lru.entries()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Vec<(Value, Value)>>(),
        vec![(json!("c"), json!(3)), (json!("b"), json!(2))]
    );

    let mut fib = mnemonist::FibonacciHeap::new_min();
    fib.push(json!(2));
    fib.push(json!(1));
    assert_eq!(fib.pop(), Some(json!(1)));
}
