use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SparseQueueSet {
    length: usize,
    dense: Vec<usize>,
    sparse: Vec<usize>,
    queue: VecDeque<usize>,
}

impl SparseQueueSet {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            dense: vec![0; length],
            sparse: vec![0; length],
            queue: VecDeque::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.length
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn has(&self, member: usize) -> bool {
        if member >= self.length {
            return false;
        }
        let index = self.sparse[member];
        index < self.queue.len() && self.dense[index] == member
    }

    pub fn enqueue(&mut self, member: usize) -> bool {
        if member >= self.length || self.has(member) {
            return false;
        }
        let index = self.queue.len();
        self.dense[index] = member;
        self.sparse[member] = index;
        self.queue.push_back(member);
        true
    }

    pub fn dequeue(&mut self) -> Option<usize> {
        let member = self.queue.pop_front()?;
        self.rebuild_sparse();
        Some(member)
    }

    pub fn values(&self) -> impl Iterator<Item = usize> + '_ {
        self.queue.iter().copied()
    }

    pub fn for_each<F: FnMut(usize)>(&self, mut callback: F) {
        for &member in &self.queue {
            callback(member);
        }
    }

    fn rebuild_sparse(&mut self) {
        for (index, member) in self.queue.iter().copied().enumerate() {
            self.dense[index] = member;
            self.sparse[member] = index;
        }
    }
}

