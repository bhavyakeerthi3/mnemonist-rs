use indexmap::IndexMap;
use serde_json::Value;

use crate::bk_tree::levenshtein;

pub fn comparator(left: &str, right: &str) -> std::cmp::Ordering {
    right
        .chars()
        .count()
        .cmp(&left.chars().count())
        .then_with(|| left.cmp(right))
}

pub fn partition(k: usize, length: usize) -> Vec<(usize, usize)> {
    let segments = k + 1;
    let small = length / segments;
    let large = small + 1;
    let large_count = length - small * segments;
    let small_count = segments - large_count;
    let mut result = Vec::with_capacity(segments);

    for index in 0..small_count {
        result.push((index * small, small));
    }
    let offset = small_count * small;
    for index in 0..large_count {
        result.push((offset + index * large, large));
    }
    result
}

pub fn segments(k: usize, value: &str) -> Vec<String> {
    let characters: Vec<_> = value.chars().collect();
    partition(k, characters.len())
        .into_iter()
        .map(|(start, length)| characters[start..start + length].iter().collect())
        .collect()
}

pub fn segment_pos(k: usize, index: usize, value: &str) -> usize {
    partition(k, value.chars().count())[index].0
}

pub fn multi_match_aware_interval(
    k: isize,
    delta: isize,
    index: isize,
    string_length: isize,
    position: isize,
    segment_length: isize,
) -> (isize, isize) {
    let start = (position - index)
        .max(position + delta - (k - index))
        .max(0);
    let end = (position + index)
        .min(position + delta + (k - index))
        .min(string_length - segment_length);
    (start, end)
}

pub fn multi_match_aware_substrings(
    k: isize,
    value: &str,
    target_length: isize,
    index: isize,
    position: isize,
    segment_length: isize,
) -> Vec<String> {
    let characters: Vec<_> = value.chars().collect();
    let (start, end) = multi_match_aware_interval(
        k,
        characters.len() as isize - target_length,
        index,
        characters.len() as isize,
        position,
        segment_length,
    );
    let mut result = Vec::new();
    let mut previous = None;
    for offset in start..=end {
        let substring: String = characters[offset as usize..(offset + segment_length) as usize]
            .iter()
            .collect();
        if previous.as_ref() != Some(&substring) {
            previous = Some(substring.clone());
            result.push(substring);
        }
    }
    result
}

#[derive(Debug, Clone, Default)]
pub struct PassjoinIndex {
    records: IndexMap<String, Value>,
}

impl PassjoinIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.records.len()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn add(&mut self, key: impl Into<String>, value: Value) {
        self.records.insert(key.into(), value);
    }

    pub fn search(&self, query: &str, radius: usize) -> Vec<(&String, &Value, usize)> {
        let mut results: Vec<_> = self
            .records
            .iter()
            .filter_map(|(key, value)| {
                let distance = levenshtein(key, query);
                (distance <= radius).then_some((key, value, distance))
            })
            .collect();
        results.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(b.0)));
        results
    }
}
