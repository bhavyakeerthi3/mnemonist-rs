#[derive(Debug, Clone)]
pub struct BitSet {
    length: usize,
    words: Vec<u32>,
    size: usize,
}

impl BitSet {
    pub fn new(length: usize) -> Self {
        let mut set = Self {
            length,
            words: Vec::new(),
            size: 0,
        };
        set.clear();
        set
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.words = vec![0; self.length.div_ceil(32)];
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn word_len(&self) -> usize {
        self.words.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.length {
            return;
        }

        let byte_index = index >> 5;
        let pos = index & 31;
        let mask = 1u32 << pos;
        let was_set = self.words[byte_index] & mask != 0;

        if value {
            self.words[byte_index] |= mask;
            if !was_set {
                self.size += 1;
            }
        } else {
            self.words[byte_index] &= !mask;
            if was_set {
                self.size -= 1;
            }
        }
    }

    pub fn reset(&mut self, index: usize) {
        self.set(index, false);
    }

    pub fn flip(&mut self, index: usize) {
        if index >= self.length {
            return;
        }

        let byte_index = index >> 5;
        let pos = index & 31;
        let mask = 1u32 << pos;

        if self.words[byte_index] & mask == 0 {
            self.words[byte_index] |= mask;
            self.size += 1;
        } else {
            self.words[byte_index] &= !mask;
            self.size -= 1;
        }
    }

    pub fn get(&self, index: usize) -> u8 {
        if index >= self.length {
            return 0;
        }
        let byte_index = index >> 5;
        let pos = index & 31;
        ((self.words[byte_index] >> pos) & 1) as u8
    }

    pub fn test(&self, index: usize) -> bool {
        self.get(index) == 1
    }

    pub fn rank(&self, index: usize) -> usize {
        if self.size == 0 {
            return 0;
        }

        let index = index.min(self.length);
        let full_words = index >> 5;
        let pos = index & 31;
        let mut rank = self.words[..full_words]
            .iter()
            .map(|word| word.count_ones() as usize)
            .sum();

        if pos > 0 && full_words < self.words.len() {
            rank += (self.words[full_words] & ((1u32 << pos) - 1)).count_ones() as usize;
        }

        rank
    }

    pub fn select(&self, r: usize) -> isize {
        if self.size == 0 || r == 0 || r > self.size {
            return -1;
        }

        let mut count = 0;
        for index in 0..self.length {
            if self.test(index) {
                count += 1;
                if count == r {
                    return index as isize;
                }
            }
        }

        -1
    }

    pub fn values(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.length).map(|index| self.get(index))
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        (0..self.length).map(|index| (index, self.get(index)))
    }

    pub fn to_json(&self) -> Vec<u32> {
        self.words.clone()
    }
}
