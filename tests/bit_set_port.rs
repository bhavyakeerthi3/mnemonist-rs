//! Port of mnemonist's original BitSet tests (tests/original/bit-set.js).

use mnemonist::BitSet;

#[test]
fn create_and_set_bits() {
    let mut set = BitSet::new(74);
    assert_eq!(set.length(), 74);
    assert_eq!(set.word_len(), 3);
    assert_eq!(set.size(), 0);

    set = BitSet::new(17);
    set.set(13, true);
    assert_eq!(set.size(), 1);
    assert_eq!(set.get(13), 1);
    assert!(set.test(13));
    assert_eq!(set.get(2), 0);

    set.set(2, true);
    assert_eq!(set.size(), 2);
    set.set(2, false);
    assert_eq!(set.size(), 1);

    set.flip(3);
    assert_eq!(set.size(), 2);
    assert!(set.test(3));
    set.flip(3);
    assert_eq!(set.size(), 1);
    assert!(!set.test(3));
}

#[test]
fn count_reset_rank_and_select() {
    let mut set = BitSet::new(32);
    for i in 0..32 {
        set.set(i, true);
        assert_eq!(set.size(), i + 1);
    }

    set.reset(31);
    assert_eq!(set.size(), 31);

    let mut set = BitSet::new(8010);
    for i in (0..8000).step_by(100) {
        set.set(i, true);
    }
    assert_eq!(set.rank(0), 0);
    assert_eq!(set.rank(2000), 20);
    assert_eq!(set.rank(4000), 40);
    assert_eq!(set.rank(6000), 60);
    assert_eq!(set.rank(8000), 80);

    let mut set = BitSet::new(11);
    for i in [1, 3, 4, 5, 9, 10] {
        set.set(i, true);
    }
    assert_eq!(set.rank(set.length()), 6);
    assert_eq!(set.select(1), 1);
    assert_eq!(set.select(2), 3);
    assert_eq!(set.select(6), 10);
    assert_eq!(set.select(7), -1);
}

#[test]
fn iterators_and_json() {
    let mut set = BitSet::new(10);
    set.set(2, true);
    set.set(8, true);
    set.set(9, true);

    let array = vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 1];
    assert_eq!(set.values().collect::<Vec<_>>(), array);
    assert_eq!(
        set.entries().collect::<Vec<_>>(),
        array
            .iter()
            .enumerate()
            .map(|(i, bit)| (i, *bit))
            .collect::<Vec<_>>()
    );
    assert_eq!(set.to_json(), vec![772]);

    let set = BitSet::new(64);
    assert_eq!(set.entries().count(), 64);
}
