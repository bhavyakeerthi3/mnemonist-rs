use mnemonist::bit_vector::BitVector;

#[test]
fn test_bit_vector() {
    let mut bv = BitVector::with_length(10);
    assert_eq!(bv.length(), 10);
    assert_eq!(bv.size(), 0);

    bv.set(3, true);
    bv.set(7, true);

    assert_eq!(bv.get(3), 1);
    assert_eq!(bv.get(7), 1);
    assert_eq!(bv.get(0), 0);
    assert_eq!(bv.size(), 2);

    assert!(bv.test(3));
    assert!(!bv.test(0));

    bv.reset(3);
    assert_eq!(bv.get(3), 0);
    assert_eq!(bv.size(), 1);

    bv.flip(7);
    assert_eq!(bv.get(7), 0);
    assert_eq!(bv.size(), 0);

    bv.flip(1);
    assert_eq!(bv.get(1), 1);

    bv.push(true);
    assert_eq!(bv.length(), 11);
    assert_eq!(bv.get(10), 1);

    assert_eq!(bv.pop(), Some(1));
    assert_eq!(bv.length(), 10);
    assert_eq!(bv.pop(), Some(0));
    assert_eq!(bv.length(), 9);

    bv.resize(100);
    assert_eq!(bv.length(), 100);
    assert_eq!(bv.get(99), 0);

    bv.set(64, true);
    assert_eq!(bv.get(64), 1);
    
    let words = bv.to_json();
    // length is 100, which requires 4 32-bit words (32, 32, 32, 4)
    assert_eq!(words.len(), 4);
    assert_eq!(words[2], 1); // bit 64 is in the 3rd word, at index 0
    assert_eq!(words[0], 2); // bit 1 is set earlier
}

#[test]
fn test_rank_select() {
    let mut bv = BitVector::with_length(10);
    bv.set(1, true);
    bv.set(5, true);
    bv.set(8, true);
    
    assert_eq!(bv.rank(0), 0);
    assert_eq!(bv.rank(1), 0);
    assert_eq!(bv.rank(2), 1);
    assert_eq!(bv.rank(5), 1);
    assert_eq!(bv.rank(6), 2);
    assert_eq!(bv.rank(10), 3);
    
    assert_eq!(bv.select(0), -1);
    assert_eq!(bv.select(1), 1);
    assert_eq!(bv.select(2), 5);
    assert_eq!(bv.select(3), 8);
    assert_eq!(bv.select(4), -1);
}

#[test]
fn test_capacity() {
    let mut bv = BitVector::new();
    bv.grow(100);
    assert!(bv.capacity() >= 100);
    bv.reallocate(200);
    assert!(bv.capacity() >= 200);
}
