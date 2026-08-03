use std::thread;

use mnemonist::{Queue, Stack};
use serde_json::json;

#[test]
fn independent_collections_survive_parallel_soak_without_cross_talk() {
    const WORKERS: usize = 8;
    const OPERATIONS: usize = 25_000;

    let workers: Vec<_> = (0..WORKERS)
        .map(|worker| {
            thread::spawn(move || {
                let mut stack = Stack::new();
                let mut queue = Queue::new();

                for value in 0..OPERATIONS {
                    stack.push(json!([worker, value]));
                    queue.enqueue(json!([worker, value]));
                }

                for value in (0..OPERATIONS).rev() {
                    assert_eq!(stack.pop(), Some(json!([worker, value])));
                }
                for value in 0..OPERATIONS {
                    assert_eq!(queue.dequeue(), Some(json!([worker, value])));
                }
                assert_eq!(stack.size(), 0);
                assert_eq!(queue.size(), 0);
            })
        })
        .collect();

    for worker in workers {
        worker
            .join()
            .expect("parallel collection worker should not panic");
    }
}
