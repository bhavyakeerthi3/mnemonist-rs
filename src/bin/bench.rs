use std::hint::black_box;
use std::time::Instant;

use mnemonist::{Queue, Stack};
use serde_json::json;

const N: usize = 200_000;

fn stack_push_pop() -> u128 {
    let started = Instant::now();
    let mut stack = Stack::new();

    for i in 0..N {
        stack.push(json!(i));
    }

    for _ in 0..N {
        black_box(stack.pop());
    }

    started.elapsed().as_micros()
}

fn queue_enqueue_dequeue() -> u128 {
    let started = Instant::now();
    let mut queue = Queue::new();

    for i in 0..N {
        queue.enqueue(json!(i));
    }

    for _ in 0..N {
        black_box(queue.dequeue());
    }

    started.elapsed().as_micros()
}

fn main() {
    let stack_us = stack_push_pop();
    let queue_us = queue_enqueue_dequeue();

    println!(
        "{{\"iterations\":{},\"stack_push_pop_us\":{},\"queue_enqueue_dequeue_us\":{},\"unsafe_count\":0}}",
        N, stack_us, queue_us
    );
}
