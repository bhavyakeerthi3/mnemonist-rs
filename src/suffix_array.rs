//! Suffix Array and Generalized Suffix Array.
//!
//! Port of mnemonist/suffix-array.js — supports construction, search,
//! and the generalized variant with longest-common-subsequence.

#[derive(Debug, Clone)]
pub struct SuffixArray {
    text: String,
    indices: Vec<usize>,
}

impl SuffixArray {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let mut indices: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
        indices.sort_by(|a, b| text[*a..].cmp(&text[*b..]));
        Self { text, indices }
    }

    pub fn length(&self) -> usize {
        self.indices.len()
    }

    pub fn inspect(&self) -> &[usize] {
        &self.indices
    }

    pub fn array(&self) -> &[usize] {
        &self.indices
    }

    pub fn string(&self) -> &str {
        &self.text
    }

    pub fn suffix(&self, rank: usize) -> Option<&str> {
        self.indices.get(rank).map(|index| &self.text[*index..])
    }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let mut positions: Vec<_> = self
            .indices
            .iter()
            .copied()
            .filter(|index| self.text[*index..].starts_with(pattern))
            .collect();
        positions.sort_unstable();
        positions
    }
}

/// Generalized Suffix Array over multiple strings.
///
/// Mirrors mnemonist's GeneralizedSuffixArray: concatenates the input
/// strings with unique sentinel characters (char values below any
/// printable character), builds the suffix array over the combined
/// string, and supports `longest_common_subsequence`.
#[derive(Debug, Clone)]
pub struct GeneralizedSuffixArray {
    strings: Vec<String>,
    combined: String,
    /// For each suffix-array position, which original string does it belong to?
    text_index: Vec<usize>,
    /// The sorted suffix-array indices into `combined`.
    indices: Vec<usize>,
}

impl GeneralizedSuffixArray {
    pub fn new(texts: &[&str]) -> Self {
        let strings: Vec<String> = texts.iter().map(|s| s.to_string()).collect();

        // Build combined string with sentinel separators.
        // We use '\x01', '\x02', … as sentinels (lower than any normal char).
        let mut combined = String::new();
        let mut boundaries: Vec<(usize, usize)> = Vec::new(); // (start, string_index)
        for (i, s) in strings.iter().enumerate() {
            let start = combined.len();
            combined.push_str(s);
            boundaries.push((start, i));
            // Append a unique sentinel between strings only (not after the
            // final one) so the combined length matches upstream mnemonist,
            // e.g. GeneralizedSuffixArray(["banana", "ananas"]).length === 13.
            if i + 1 < strings.len() {
                combined.push(char::from((i + 1) as u8));
            }
        }

        // Build suffix array over the combined string
        let char_indices: Vec<usize> = combined.char_indices().map(|(i, _)| i).collect();
        let mut sorted = char_indices.clone();
        sorted.sort_by(|a, b| combined[*a..].cmp(&combined[*b..]));

        // For each suffix position, determine which original string it belongs to
        let mut text_index = vec![0usize; combined.len()];
        for (i, &(start, si)) in boundaries.iter().enumerate() {
            let end = if i + 1 < boundaries.len() {
                boundaries[i + 1].0
            } else {
                combined.len()
            };
            for pos in start..end {
                text_index[pos] = si;
            }
        }

        // Filter out sentinel-only suffixes from the array, keep all others
        // Actually the original JS includes them — keep all indices.
        Self {
            strings,
            combined,
            text_index,
            indices: sorted,
        }
    }

    pub fn length(&self) -> usize {
        self.indices.len()
    }

    pub fn size(&self) -> usize {
        self.strings.len()
    }

    pub fn array(&self) -> &[usize] {
        &self.indices
    }

    /// Find the longest common subsequence (substring) between the
    /// original strings using the LCP array approach.
    pub fn longest_common_subsequence(&self) -> String {
        if self.strings.len() < 2 {
            return String::new();
        }

        let n = self.indices.len();
        let bytes = self.combined.as_bytes();

        let mut best_len = 0usize;
        let mut best_pos = 0usize;

        // Walk adjacent pairs in the suffix array; when they belong to
        // different original strings, compute their LCP.
        for i in 1..n {
            let a = self.indices[i - 1];
            let b = self.indices[i];

            let ta = self.text_index.get(a).copied().unwrap_or(usize::MAX);
            let tb = self.text_index.get(b).copied().unwrap_or(usize::MAX);

            if ta == tb {
                continue; // same string — skip
            }

            // Compute LCP, but stop at sentinel characters (< 0x20)
            let mut lcp = 0;
            let sa = &bytes[a..];
            let sb = &bytes[b..];
            for (ca, cb) in sa.iter().zip(sb.iter()) {
                if ca != cb || *ca < 0x20 || *cb < 0x20 {
                    break;
                }
                lcp += 1;
            }

            if lcp > best_len {
                best_len = lcp;
                best_pos = a;
            }
        }

        self.combined[best_pos..best_pos + best_len].to_string()
    }
}
