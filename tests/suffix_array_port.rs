use mnemonist::suffix_array::{SuffixArray, GeneralizedSuffixArray};

// ── SuffixArray (mirrors original suffix-array.js) ─────────────────

#[test]
fn suffix_array_banana() {
    let sa = SuffixArray::new("banana");

    assert_eq!(sa.length(), 6);
    assert_eq!(sa.string(), "banana");
    // sorted suffixes: a(5) ana(3) anana(1) banana(0) na(4) nana(2)
    assert_eq!(sa.array(), &[5, 3, 1, 0, 4, 2]);
    assert_eq!(sa.inspect(), &[5, 3, 1, 0, 4, 2]);

    assert_eq!(sa.suffix(0), Some("a"));
    assert_eq!(sa.suffix(1), Some("ana"));
    assert_eq!(sa.suffix(2), Some("anana"));
    assert_eq!(sa.suffix(3), Some("banana"));
    assert_eq!(sa.suffix(4), Some("na"));
    assert_eq!(sa.suffix(5), Some("nana"));
}

#[test]
fn suffix_array_long_string() {
    // mirrors: new SuffixArray('This is a long string.')
    let sa = SuffixArray::new("This is a long string.");
    assert_eq!(sa.length(), 22);
    assert_eq!(
        sa.array(),
        &[7, 4, 9, 14, 21, 0, 8, 13, 20, 1, 18, 5, 2, 10, 12, 19, 11, 17, 6, 3, 15, 16]
    );
}

#[test]
fn suffix_array_search_ana() {
    let sa = SuffixArray::new("banana");
    let res = sa.search("ana");
    assert_eq!(res, vec![1, 3]);
}

#[test]
fn suffix_array_search_nan() {
    let sa = SuffixArray::new("banana");
    // "nan" only matches at position 2 ("nana"); position 4 ("na") is too short
    let res = sa.search("nan");
    assert_eq!(res, vec![2]);
}

#[test]
fn suffix_array_search_b() {
    let sa = SuffixArray::new("banana");
    assert_eq!(sa.search("b"), vec![0]);
}

#[test]
fn suffix_array_search_miss() {
    let sa = SuffixArray::new("banana");
    assert_eq!(sa.search("x"), Vec::<usize>::new());
}

// ── GeneralizedSuffixArray (mirrors original suffix-array.js) ──────

#[test]
fn generalized_suffix_array_banana_ananas() {
    let gsa = GeneralizedSuffixArray::new(&["banana", "ananas"]);

    assert_eq!(gsa.length(), 13);
    assert_eq!(gsa.size(), 2);
    assert_eq!(
        gsa.array(),
        &[6, 5, 3, 1, 7, 9, 11, 0, 4, 2, 8, 10, 12]
    );
}

#[test]
fn generalized_suffix_array_lcs() {
    let gsa = GeneralizedSuffixArray::new(&["banana", "ananas"]);
    assert_eq!(gsa.longest_common_subsequence(), "anana");

    let gsa2 = GeneralizedSuffixArray::new(&["abcd", "cdef"]);
    assert_eq!(gsa2.longest_common_subsequence(), "cd");
}
