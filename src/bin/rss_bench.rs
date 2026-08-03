use std::io::{self, Write};

use mnemonist::{Queue, Stack};
use serde_json::json;

const DEFAULT_ITEMS: usize = 200_000;

fn item_count() -> usize {
    std::env::var("MNEMONIST_RSS_ITEMS")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_ITEMS)
}

fn main() {
    let items = item_count();
    let mut stack = Stack::new();
    let mut queue = Queue::new();

    for i in 0..items {
        stack.push(json!(i));
        queue.enqueue(json!(i));
    }

    // Keep the post-workload allocations live until the sampler acknowledges them.
    println!(
        "ready items={items} stack={} queue={}",
        stack.size(),
        queue.size()
    );
    io::stdout().flush().expect("flush RSS readiness marker");

    let mut acknowledgement = String::new();
    io::stdin()
        .read_line(&mut acknowledgement)
        .expect("read RSS sampler acknowledgement");

    drop((stack, queue));
}
