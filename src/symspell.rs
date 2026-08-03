use std::collections::{HashMap, VecDeque};

use indexmap::{IndexMap, IndexSet};

use crate::bk_tree::levenshtein;

#[derive(Debug, Clone, Default)]
struct DictionaryEntry {
    suggestions: IndexSet<usize>,
    count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymSpellSuggestion {
    pub term: String,
    pub distance: usize,
    pub count: usize,
}

/// A delete-based spelling index compatible with Mnemonist's SymSpell API.
///
/// The dictionary deliberately uses insertion-ordered maps and sets. Upstream
/// exposes the discovery order of equally good suggestions, so sorting results
/// after lookup would be observably different.
#[derive(Debug, Clone)]
pub struct SymSpell {
    max_distance: usize,
    verbosity: u8,
    size: usize,
    dictionary: IndexMap<String, DictionaryEntry>,
    max_length: usize,
    words: Vec<String>,
}

impl Default for SymSpell {
    fn default() -> Self {
        Self::with_options(2, 2).expect("default SymSpell options are valid")
    }
}

impl SymSpell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(max_distance: usize, verbosity: u8) -> Result<Self, &'static str> {
        if max_distance == 0 {
            return Err("max_distance must be greater than zero");
        }
        if verbosity > 2 {
            return Err("verbosity must be 0, 1, or 2");
        }
        Ok(Self {
            max_distance,
            verbosity,
            size: 0,
            dictionary: IndexMap::new(),
            max_length: 0,
            words: Vec::new(),
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn max_distance(&self) -> usize {
        self.max_distance
    }

    pub fn verbosity(&self) -> u8 {
        self.verbosity
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.dictionary.clear();
        self.max_length = 0;
        self.words.clear();
    }

    pub fn add(&mut self, word: impl Into<String>) -> &mut Self {
        let word = word.into();
        let is_new_dictionary_key = !self.dictionary.contains_key(&word);
        let count = {
            let target = self.dictionary.entry(word.clone()).or_default();
            target.count += 1;
            target.count
        };

        if is_new_dictionary_key {
            self.max_length = self.max_length.max(string_length(&word));
        }

        if count == 1 {
            let index = self.words.len();
            self.words.push(word.clone());

            for deleted in edits(&word, self.max_distance) {
                if let Some(target) = self.dictionary.get_mut(&deleted) {
                    if !target.suggestions.contains(&index) {
                        add_lowest_distance(&self.words, self.verbosity, target, index, &deleted);
                    }
                } else {
                    let mut target = DictionaryEntry::default();
                    target.suggestions.insert(index);
                    self.dictionary.insert(deleted, target);
                }
            }
        }

        self.size += 1;
        self
    }

    pub fn search(&self, input: &str) -> Vec<SymSpellSuggestion> {
        let length = string_length(input);
        if length.saturating_sub(self.max_distance) > self.max_length {
            return Vec::new();
        }

        let mut candidates = VecDeque::from([input.to_owned()]);
        let mut candidate_set = IndexSet::new();
        let mut suggestion_set = IndexSet::new();
        let mut suggestions: Vec<SymSpellSuggestion> = Vec::new();

        while let Some(candidate) = candidates.pop_front() {
            let candidate_length = string_length(&candidate);
            if self.verbosity < 2
                && !suggestions.is_empty()
                && length - candidate_length > suggestions[0].distance
            {
                break;
            }

            if let Some(target) = self.dictionary.get(&candidate) {
                if target.count > 0 && suggestion_set.insert(candidate.clone()) {
                    suggestions.push(SymSpellSuggestion {
                        term: candidate.clone(),
                        distance: length - candidate_length,
                        count: target.count,
                    });
                    if self.verbosity < 2 && length == candidate_length {
                        break;
                    }
                }

                for index in &target.suggestions {
                    let term = &self.words[*index];
                    if !suggestion_set.insert(term.clone()) {
                        continue;
                    }
                    let steps = if input == term {
                        0
                    } else {
                        damerau_levenshtein(term, input)
                    };
                    if self.verbosity < 2
                        && !suggestions.is_empty()
                        && suggestions[0].distance > steps
                    {
                        suggestions.clear();
                    }
                    if self.verbosity < 2
                        && !suggestions.is_empty()
                        && steps > suggestions[0].distance
                    {
                        continue;
                    }
                    if steps <= self.max_distance {
                        if let Some(entry) = self.dictionary.get(term) {
                            suggestions.push(SymSpellSuggestion {
                                term: term.clone(),
                                distance: steps,
                                count: entry.count,
                            });
                        }
                    }
                }
            }

            if length - candidate_length < self.max_distance {
                if self.verbosity < 2
                    && !suggestions.is_empty()
                    && length - candidate_length >= suggestions[0].distance
                {
                    continue;
                }
                for deleted in one_character_deletes(&candidate) {
                    if candidate_set.insert(deleted.clone()) {
                        candidates.push_back(deleted);
                    }
                }
            }
        }

        if self.verbosity == 0 {
            suggestions.truncate(1);
        }
        suggestions
    }

    /// Retains the small radius-based helper exposed by the initial Rust port.
    pub fn lookup(&self, query: &str, radius: usize) -> Vec<(String, usize)> {
        let mut results: Vec<_> = self
            .words
            .iter()
            .map(|word| (word.clone(), levenshtein(word, query)))
            .filter(|(_, distance)| *distance <= radius)
            .collect();
        results.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
        results
    }
}

fn string_length(value: &str) -> usize {
    value.chars().count()
}

fn one_character_deletes(word: &str) -> Vec<String> {
    let characters: Vec<_> = word.chars().collect();
    (0..characters.len())
        .map(|index| {
            characters
                .iter()
                .enumerate()
                .filter_map(|(current, character)| (current != index).then_some(*character))
                .collect()
        })
        .collect()
}

fn edits(word: &str, max_distance: usize) -> IndexSet<String> {
    fn visit(word: &str, distance: usize, max_distance: usize, deletes: &mut IndexSet<String>) {
        let distance = distance + 1;
        if string_length(word) > 1 {
            for deleted in one_character_deletes(word) {
                if deletes.insert(deleted.clone()) && distance < max_distance {
                    visit(&deleted, distance, max_distance, deletes);
                }
            }
        }
    }

    let mut deletes = IndexSet::new();
    visit(word, 0, max_distance, &mut deletes);
    deletes
}

fn add_lowest_distance(
    words: &[String],
    verbosity: u8,
    target: &mut DictionaryEntry,
    index: usize,
    deleted: &str,
) {
    let deleted_length = string_length(deleted);
    if verbosity < 2 && !target.suggestions.is_empty() {
        let first = target.suggestions[0];
        if string_length(&words[first]) - deleted_length
            > string_length(&words[index]) - deleted_length
        {
            target.suggestions.clear();
            target.count = 0;
        }
    }
    if verbosity == 2
        || target.suggestions.is_empty()
        || string_length(&words[target.suggestions[0]]) - deleted_length
            >= string_length(&words[index]) - deleted_length
    {
        target.suggestions.insert(index);
    }
}

fn damerau_levenshtein(left: &str, right: &str) -> usize {
    let left: Vec<_> = left.chars().collect();
    let right: Vec<_> = right.chars().collect();
    let infinity = left.len() + right.len();
    let mut matrix = vec![vec![0; right.len() + 2]; left.len() + 2];
    let mut last_seen = HashMap::new();

    matrix[0][0] = infinity;
    for left_index in 0..=left.len() {
        matrix[left_index + 1][1] = left_index;
        matrix[left_index + 1][0] = infinity;
    }
    for right_index in 0..=right.len() {
        matrix[1][right_index + 1] = right_index;
        matrix[0][right_index + 1] = infinity;
    }
    for character in left.iter().chain(&right) {
        last_seen.entry(*character).or_insert(0usize);
    }

    for left_index in 1..=left.len() {
        let mut last_matching_right = 0;
        for right_index in 1..=right.len() {
            let previous_left = last_seen[&right[right_index - 1]];
            let previous_right = last_matching_right;
            if left[left_index - 1] == right[right_index - 1] {
                matrix[left_index + 1][right_index + 1] = matrix[left_index][right_index];
                last_matching_right = right_index;
            } else {
                matrix[left_index + 1][right_index + 1] = (matrix[left_index][right_index] + 1)
                    .min(matrix[left_index + 1][right_index] + 1)
                    .min(matrix[left_index][right_index + 1] + 1);
            }
            matrix[left_index + 1][right_index + 1] = matrix[left_index + 1][right_index + 1].min(
                matrix[previous_left][previous_right]
                    + (left_index - previous_left - 1)
                    + 1
                    + (right_index - previous_right - 1),
            );
        }
        last_seen.insert(left[left_index - 1], left_index);
    }

    matrix[left.len() + 1][right.len() + 1]
}

#[cfg(test)]
mod tests {
    use super::{damerau_levenshtein, SymSpell};

    #[test]
    fn preserves_mnemonist_suggestion_order_and_counts() {
        let mut index = SymSpell::new();
        for word in [
            "Hello", "Mello", "John", "Book", "Back", "World", "Hello", "Jello", "Hell", "Trello",
        ] {
            index.add(word);
        }

        assert_eq!(index.size(), 10);
        assert_eq!(
            index
                .search("ello")
                .into_iter()
                .map(|suggestion| (suggestion.term, suggestion.distance, suggestion.count))
                .collect::<Vec<_>>(),
            vec![
                ("Hello".into(), 1, 2),
                ("Mello".into(), 1, 1),
                ("Jello".into(), 1, 1),
                ("Trello".into(), 2, 1),
                ("Hell".into(), 2, 1),
            ]
        );
    }

    #[test]
    fn retains_suggestions_shared_by_multiple_delete_paths() {
        let mut index = SymSpell::with_options(2, 2).unwrap();
        for word in ["bem", "bwwhwcp", "wmw", ""] {
            index.add(word);
        }

        assert!(
            index.dictionary["b"].suggestions.contains(&0),
            "delete dictionary lost bem: {:?}",
            index.dictionary["b"]
        );
        assert_eq!(damerau_levenshtein("bem", "mb"), 2);
        assert!(
            index
                .search("mb")
                .iter()
                .any(|suggestion| suggestion.term == "bem"),
            "search lost bem: {:?}",
            index.search("mb")
        );
    }
}
