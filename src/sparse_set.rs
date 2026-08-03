#[derive(Debug, Clone)]
pub struct SparseSet {
    length: usize,
    dense: Vec<usize>,
    sparse: Vec<usize>,
    size: usize,
}

impl SparseSet {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            dense: vec![0; length],
            sparse: vec![0; length],
            size: 0,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn has(&self, member: usize) -> bool {
        if member >= self.length {
            return false;
        }
        let index = self.sparse[member];
        index < self.size && self.dense[index] == member
    }

    pub fn add(&mut self, member: usize) {
        if member >= self.length || self.has(member) {
            return;
        }

        self.dense[self.size] = member;
        self.sparse[member] = self.size;
        self.size += 1;
    }

    pub fn delete(&mut self, member: usize) -> bool {
        if !self.has(member) {
            return false;
        }

        let index = self.sparse[member];
        let last = self.dense[self.size - 1];
        self.dense[index] = last;
        self.sparse[last] = index;
        self.size -= 1;
        true
    }

    pub fn values(&self) -> impl Iterator<Item = usize> + '_ {
        self.dense[..self.size].iter().copied()
    }
}
