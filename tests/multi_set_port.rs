//! Port of mnemonist's original MultiSet tests (tests/original/multi-set.js).

use mnemonist::MultiSet;
use serde_json::json;

#[test]
fn add_remove_delete_and_counts() {
    let mut set = MultiSet::new();
    set.add(json!("hello"), 1);
    set.add(json!("hello"), 1);
    set.add(json!("world"), 1);

    assert_eq!(set.size(), 3);
    assert_eq!(set.dimension(), 2);
    assert!(set.has(&json!("hello")));
    assert_eq!(set.multiplicity(&json!("hello")), 2);

    set.add(json!("hello"), 0);
    assert_eq!(set.size(), 3);

    set.add(json!("hello"), -1);
    assert_eq!(set.multiplicity(&json!("hello")), 1);

    set.remove(&json!("hello"), 16);
    assert_eq!(set.multiplicity(&json!("hello")), 0);
    assert!(!set.has(&json!("hello")));

    assert!(set.delete(&json!("world")));
    assert_eq!(set.size(), 0);
    assert_eq!(set.dimension(), 0);
}

#[test]
fn frequency_clear_and_set() {
    let mut set = MultiSet::new();
    set.add(json!("apple"), 5);
    set.add(json!("pear"), 2);
    set.add(json!("melon"), 3);

    assert_eq!(set.frequency(&json!("apple")), 0.5);

    set.clear();
    assert_eq!(set.size(), 0);
    assert_eq!(set.frequency(&json!("apple")), 0.0);

    set.set(json!("hello"), 4);
    assert_eq!(set.size(), 4);
    assert_eq!(set.dimension(), 1);
    set.set(json!("hello"), 0);
    assert_eq!(set.size(), 0);
    assert!(!set.has(&json!("hello")));
}

#[test]
fn edit_merges_counts() {
    let mut set = MultiSet::new();
    set.edit(&json!("a"), json!("b"));
    assert_eq!(set.size(), 0);

    set.add(json!("a"), 1);
    set.edit(&json!("a"), json!("b"));
    assert_eq!(
        set.multiplicities()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>(),
        vec![(json!("b"), 1)]
    );

    set.add(json!("c"), 1);
    set.edit(&json!("b"), json!("c"));
    assert_eq!(set.multiplicity(&json!("c")), 2);
    assert_eq!(set.dimension(), 1);
}

#[test]
fn iterators_and_top() {
    let mut set = MultiSet::new();
    set.add(json!("hello"), 2);
    set.add(json!("world"), 1);

    assert_eq!(
        set.values(),
        vec![json!("hello"), json!("hello"), json!("world")]
    );
    assert_eq!(
        set.keys().cloned().collect::<Vec<_>>(),
        vec![json!("hello"), json!("world")]
    );
    assert_eq!(
        set.multiplicities()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>(),
        vec![(json!("hello"), 2), (json!("world"), 1)]
    );
    assert_eq!(set.top(1).unwrap(), vec![(json!("hello"), 2)]);
    assert!(set.top(0).is_err());
}

#[test]
fn subset_and_superset() {
    let letters = MultiSet::from_iter([
        json!("a"),
        json!("a"),
        json!("a"),
        json!("b"),
        json!("c"),
        json!("d"),
        json!("d"),
    ]);
    let less = MultiSet::from_iter([json!("a"), json!("a"), json!("b"), json!("c")]);
    let other = MultiSet::from_iter([json!("z"), json!("k")]);
    let overlapping =
        MultiSet::from_iter([json!("a"), json!("a"), json!("a"), json!("a"), json!("c")]);

    assert!(MultiSet::is_superset(&letters, &less));
    assert!(!MultiSet::is_superset(&less, &letters));
    assert!(!MultiSet::is_superset(&letters, &other));
    assert!(!MultiSet::is_superset(&overlapping, &letters));
    assert!(MultiSet::is_subset(&less, &letters));
    assert!(!MultiSet::is_subset(&letters, &overlapping));
}
