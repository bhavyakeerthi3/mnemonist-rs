#[derive(Debug, Clone, Default)]
pub struct BitVector {
    bits: Vec<u8>,
}

impl BitVector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_length(length: usize) -> Self {
        Self {
            bits: vec![0; length],
        }
    }

    pub fn length(&self) -> usize {
        self.bits.len()
    }

    pub fn size(&self) -> usize {
        self.bits.iter().filter(|bit| **bit == 1).count()
    }

    pub fn push(&mut self, bit: bool) -> usize {
        self.bits.push(u8::from(bit));
        self.bits.len()
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.bits.pop()
    }

    /// Sets the bit at `index` to `bit`. Returns `true` if this actually
    /// changed the stored value (mirrors the `HashSet::insert`-style
    /// "was this newly set" contract used elsewhere in this crate), and
    /// `false` if the index was out of bounds or the bit already held
    /// that value.
    pub fn set(&mut self, index: usize, bit: bool) -> bool {
        if index >= self.bits.len() {
            return false;
        }
        let new_value = u8::from(bit);
        let changed = self.bits[index] != new_value;
        self.bits[index] = new_value;
        changed
    }

    pub fn get(&self, index: usize) -> u8 {
        self.bits.get(index).copied().unwrap_or(0)
    }

    pub fn rank(&self, index: usize) -> usize {
        self.bits[..index.min(self.bits.len())]
            .iter()
            .filter(|bit| **bit == 1)
            .count()
    }

    pub fn select(&self, r: usize) -> isize {
        if r == 0 {
            return -1;
        }
        let mut seen = 0;
        for (index, bit) in self.bits.iter().enumerate() {
            if *bit == 1 {
                seen += 1;
                if seen == r {
                    return index as isize;
                }
            }
        }
        -1
    }

    pub fn values(&self) -> impl Iterator<Item = u8> + '_ {
        self.bits.iter().copied()
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.bits.iter().copied().enumerate()
    }

    pub fn test(&self, index: usize) -> bool {
        self.get(index) == 1
    }

    pub fn reset(&mut self, index: usize) {
        if index < self.bits.len() {
            self.bits[index] = 0;
        }
    }

    pub fn flip(&mut self, index: usize) {
        if index < self.bits.len() {
            self.bits[index] ^= 1;
        }
    }

    pub fn reallocate(&mut self, new_capacity: usize) {
        if new_capacity > self.bits.len() {
            self.bits.reserve_exact(new_capacity - self.bits.len());
        }
    }

    pub fn grow(&mut self, desired_capacity: usize) {
        if desired_capacity > self.bits.capacity() {
            self.bits.reserve(desired_capacity - self.bits.len());
        }
    }

    pub fn resize(&mut self, new_length: usize) {
        self.bits.resize(new_length, 0);
    }

    pub fn capacity(&self) -> usize {
        self.bits.capacity()
    }

    pub fn to_json(&self) -> Vec<u32> {
        let mut words = Vec::new();
        for chunk in self.bits.chunks(32) {
            let mut word: u32 = 0;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit == 1 {
                    word |= 1 << i;
                }
            }
            words.push(word);
        }
        words
    }
}
