use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const LN2_SQUARED: f64 = std::f64::consts::LN_2 * std::f64::consts::LN_2;

#[derive(Debug, Clone)]
pub struct JsBloomFilter {
    capacity: usize,
    hashes: usize,
    data: Vec<u8>,
}

impl JsBloomFilter {
    pub fn new(capacity: usize, error_rate: f64) -> Result<Self, String> {
        if capacity == 0 {
            return Err("mnemonist/BloomFilter.constructor: `capacity` option should be a positive integer.".into());
        }
        if !error_rate.is_finite() || error_rate <= 0.0 {
            return Err("mnemonist/BloomFilter.constructor: `errorRate` option should be a positive float.".into());
        }
        let length = ((-(capacity as f64) * error_rate.ln() / LN2_SQUARED) / 8.0) as usize;
        let hashes = ((length * 8) as f64 / capacity as f64 * std::f64::consts::LN_2) as usize;
        Ok(Self { capacity, hashes, data: vec![0; length] })
    }

    pub fn capacity(&self) -> usize { self.capacity }
    pub fn hashes(&self) -> usize { self.hashes }
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn clear(&mut self) { self.data.fill(0); }

    pub fn add(&mut self, value: &str) {
        for seed in 0..self.hashes {
            let index = murmurhash3((seed as u32).wrapping_mul(0xfba4c795), value) as usize % (self.data.len() * 8);
            self.data[index >> 3] |= 1 << (7 & index);
        }
    }

    pub fn test(&self, value: &str) -> bool {
        (0..self.hashes).all(|seed| {
            let index = murmurhash3((seed as u32).wrapping_mul(0xfba4c795), value) as usize % (self.data.len() * 8);
            self.data[index >> 3] & (1 << (7 & index)) != 0
        })
    }
}

fn murmurhash3(seed: u32, value: &str) -> u32 {
    let data: Vec<u16> = value.encode_utf16().collect();
    let mut hash = seed;
    let mut i = 0;
    while i + 3 < data.len() {
        let mut k1 = data[i] as u32 | (data[i + 1] as u32) << 8 | (data[i + 2] as u32) << 16 | (data[i + 3] as u32) << 24;
        k1 = k1.wrapping_mul(0xcc9e2d51).rotate_left(15).wrapping_mul(0x1b873593);
        hash ^= k1;
        hash = sum32(hash.rotate_left(13).wrapping_mul(5), 0x6b64e654);
        i += 4;
    }
    let mut k1 = 0;
    match data.len() & 3 {
        3 => k1 ^= (data[i + 2] as u32) << 16,
        _ => {}
    }
    if data.len() & 3 >= 2 { k1 ^= (data[i + 1] as u32) << 8; }
    if data.len() & 3 >= 1 {
        k1 ^= data[i] as u32;
        k1 = k1.wrapping_mul(0xcc9e2d51).rotate_left(15).wrapping_mul(0x1b873593);
        hash ^= k1;
    }
    hash ^= data.len() as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^ (hash >> 16)
}

fn sum32(a: u32, b: u32) -> u32 {
    (a & 0xffff)
        .wrapping_add(b >> 16)
        .wrapping_add((((a >> 16).wrapping_add(b)) & 0xffff) << 16)
}

#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: Vec<bool>,
    hashes: usize,
    inserted: usize,
}

impl BloomFilter {
    pub fn new(capacity: usize, error_rate: f64) -> Self {
        let capacity = capacity.max(1);
        let error_rate = error_rate.clamp(0.0001, 0.9999);
        let m = (-(capacity as f64) * error_rate.ln() / 2f64.ln().powi(2)).ceil() as usize;
        let k = ((m as f64 / capacity as f64) * 2f64.ln()).ceil() as usize;

        Self {
            bits: vec![false; m.max(1)],
            hashes: k.max(1),
            inserted: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.bits.len()
    }

    pub fn hashes(&self) -> usize {
        self.hashes
    }

    pub fn size(&self) -> usize {
        self.inserted
    }

    pub fn clear(&mut self) {
        self.bits.fill(false);
        self.inserted = 0;
    }

    pub fn add<T: Hash>(&mut self, value: &T) {
        let indexes = self.indexes(value);
        for index in indexes {
            self.bits[index] = true;
        }
        self.inserted += 1;
    }

    pub fn contains<T: Hash>(&self, value: &T) -> bool {
        self.indexes(value)
            .into_iter()
            .all(|index| self.bits[index])
    }

    fn indexes<T: Hash>(&self, value: &T) -> Vec<usize> {
        let h1 = hash_with_seed(value, 0);
        let h2 = hash_with_seed(value, 1).max(1);
        (0..self.hashes)
            .map(|i| (h1.wrapping_add((i as u64).wrapping_mul(h2)) as usize) % self.bits.len())
            .collect()
    }
}

fn hash_with_seed<T: Hash>(value: &T, seed: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    value.hash(&mut hasher);
    hasher.finish()
}
