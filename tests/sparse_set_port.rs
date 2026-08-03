//! Port of mnemonist's original SparseSet tests (tests/original/sparse-set.js).

use mnemonist::SparseSet;

#[test]
fn add_has_delete_clear_and_values() {
    let mut set = SparseSet::new(10);
    set.add(3);
    set.add(4);
    set.add(3);

    assert_eq!(set.size(), 2);
    assert_eq!(set.length(), 10);
    assert!(set.has(3));
    assert!(!set.has(1));

    assert!(set.delete(3));
    assert!(!set.delete(3));
    assert_eq!(set.size(), 1);

    for i in 0..6 {
        set.add(i);
    }
    set.clear();
    assert_eq!(set.size(), 0);
    assert!(!set.has(1));
}

#[test]
fn values_preserve_dense_order() {
    let mut set = SparseSet::new(10);
    set.add(3);
    set.add(6);
    set.add(9);

    assert_eq!(set.values().collect::<Vec<_>>(), vec![3, 6, 9]);
}
