//! Port of mnemonist's original StaticDisjointSet tests.

use mnemonist::StaticDisjointSet;

#[test]
fn union_find_mapping_and_compile() {
    let mut sets = StaticDisjointSet::new(10);

    sets.union(0, 1);
    sets.union(1, 5);
    sets.union(0, 7);
    sets.union(8, 9);
    sets.union(2, 3);
    sets.union(2, 4);

    assert_eq!(sets.size(), 10);
    assert_eq!(sets.dimension(), 4);
    assert!(sets.connected(1, 7));
    assert!(!sets.connected(6, 0));
    assert_eq!(sets.mapping(), vec![0, 0, 1, 1, 1, 0, 2, 0, 3, 3]);
    assert_eq!(
        sets.compile(),
        vec![vec![0, 1, 5, 7], vec![2, 3, 4], vec![6], vec![8, 9]]
    );
}
