use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SparseMap {
    length: usize,
    dense: Vec<usize>,
    sparse: Vec<usize>,
    values: Vec<Value>,
}

impl SparseMap {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            dense: vec![0; length],
            sparse: vec![0; length],
            values: Vec::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn has(&self, member: usize) -> bool {
        if member >= self.length {
            return false;
        }
        let index = self.sparse[member];
        index < self.values.len() && self.dense[index] == member
    }

    pub fn set(&mut self, member: usize, value: Value) {
        if member >= self.length {
            return;
        }
        if self.has(member) {
            let index = self.sparse[member];
            self.values[index] = value;
            return;
        }
        let index = self.values.len();
        self.dense[index] = member;
        self.sparse[member] = index;
        self.values.push(value);
    }

    pub fn get(&self, member: usize) -> Option<&Value> {
        if !self.has(member) {
            return None;
        }
        self.values.get(self.sparse[member])
    }

    pub fn delete(&mut self, member: usize) -> bool {
        if !self.has(member) {
            return false;
        }
        let index = self.sparse[member];
        let last_member = self.dense[self.values.len() - 1];
        self.dense[index] = last_member;
        self.sparse[last_member] = index;
        self.values.swap_remove(index);
        true
    }

    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.dense[..self.values.len()].iter().copied()
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.keys().zip(self.values.iter())
    }

    pub fn for_each<F: FnMut(&Value, usize)>(&self, mut callback: F) {
        for i in 0..self.values.len() {
            callback(&self.values[i], self.dense[i]);
        }
    }
}
