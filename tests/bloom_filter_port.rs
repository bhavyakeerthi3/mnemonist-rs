use mnemonist::bloom_filter::BloomFilter;

#[test]
fn test_bloom_filter() {
    let mut bf = BloomFilter::new(100, 0.01);
    
    assert_eq!(bf.size(), 0);
    assert!(bf.capacity() > 100);
    assert!(bf.hashes() > 1);

    // Test add and contains
    bf.add(&"hello");
    bf.add(&"world");

    assert_eq!(bf.size(), 2);
    
    assert!(bf.contains(&"hello"));
    assert!(bf.contains(&"world"));
    
    assert!(!bf.contains(&"goodbye"));
    assert!(!bf.contains(&"earth"));
    
    // Test false positive behavior
    // With 100 capacity and 0.01 error rate, adding a few items shouldn't cause too many false positives
    let mut fp_count = 0;
    for i in 0..100 {
        let val = format!("test{}", i);
        if bf.contains(&val) {
            fp_count += 1;
        }
    }
    // Shouldn't see many false positives
    assert!(fp_count < 10);

    // Test clear
    bf.clear();
    assert_eq!(bf.size(), 0);
    assert!(!bf.contains(&"hello"));
    assert!(!bf.contains(&"world"));
}

#[test]
fn test_bloom_filter_edge_cases() {
    let bf1 = BloomFilter::new(0, 0.5); // capacity will be 1
    assert!(bf1.capacity() > 0);
    
    let bf2 = BloomFilter::new(100, 2.0); // error_rate clamped to 0.9999
    assert!(bf2.capacity() > 0);
}
