use mnemonist::sparse_queue_set::SparseQueueSet;

#[test]
fn test_sparse_queue_set_methods() {
    let mut set = SparseQueueSet::new(10);
    assert_eq!(set.capacity(), 10);
    assert_eq!(set.length(), 10);
    assert_eq!(set.size(), 0);

    assert!(set.enqueue(2));
    assert_eq!(set.size(), 1);
    assert!(set.has(2));
    assert!(!set.has(3));

    // enqueue existing
    assert!(!set.enqueue(2));
    assert_eq!(set.size(), 1);

    assert!(set.enqueue(5));
    assert_eq!(set.size(), 2);
    assert!(set.has(5));

    // out of bounds
    assert!(!set.has(15));
    assert!(!set.enqueue(15));

    // values
    let values: Vec<usize> = set.values().collect();
    assert_eq!(values, vec![2, 5]);

    // dequeue
    assert_eq!(set.dequeue(), Some(2));
    assert_eq!(set.size(), 1);
    assert!(!set.has(2));
    assert!(set.has(5));
    
    // values after dequeue
    let values2: Vec<usize> = set.values().collect();
    assert_eq!(values2, vec![5]);

    assert_eq!(set.dequeue(), Some(5));
    assert_eq!(set.size(), 0);
    assert!(!set.has(5));
    assert_eq!(set.dequeue(), None);

    // clear
    set.enqueue(1);
    set.enqueue(4);
    assert_eq!(set.size(), 2);
    set.clear();
    assert_eq!(set.size(), 0);
    assert!(!set.has(1));
    assert_eq!(set.dequeue(), None);
}

#[test]
fn test_sparse_queue_set_wrapping() {
    // Should not break when wrapping around (simulate by full enqueue/dequeue cycle)
    let mut set = SparseQueueSet::new(3);
    assert!(set.enqueue(0));
    assert!(set.enqueue(1));
    assert!(set.enqueue(2));
    assert!(!set.enqueue(0));
    
    assert_eq!(set.dequeue(), Some(0));
    assert!(!set.has(0));
    
    // enqueue again should work without issues
    assert!(set.enqueue(0));
    assert!(set.has(0));
    
    assert_eq!(set.dequeue(), Some(1));
    assert_eq!(set.dequeue(), Some(2));
    assert_eq!(set.dequeue(), Some(0));
    assert_eq!(set.dequeue(), None);
}
