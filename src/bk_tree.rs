#[derive(Debug, Clone)]
pub struct BkTree {
    words: Vec<String>,
}

impl BkTree {
    pub fn new() -> Self {
        Self { words: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn clear(&mut self) {
        self.words.clear();
    }

    pub fn add(&mut self, word: impl Into<String>) {
        let word = word.into();
        if !self.words.contains(&word) {
            self.words.push(word);
        }
    }

    pub fn search(&self, query: &str, radius: usize) -> Vec<(String, usize)> {
        let mut results: Vec<_> = self
            .words
            .iter()
            .map(|word| (word.clone(), levenshtein(word, query)))
            .filter(|(_, distance)| *distance <= radius)
            .collect();
        results.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        results
    }

    pub fn values(&self) -> &[String] {
        &self.words
    }

    pub fn from_iter<I, S>(iterable: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut tree = Self::new();
        for word in iterable {
            tree.add(word);
        }
        tree
    }
}

impl Default for BkTree {
    fn default() -> Self {
        Self::new()
    }
}

pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut costs: Vec<usize> = (0..=b.chars().count()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut last = i;
        costs[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let old = costs[j + 1];
            costs[j + 1] = if ca == cb {
                last
            } else {
                1 + last.min(costs[j]).min(costs[j + 1])
            };
            last = old;
        }
    }
    *costs.last().unwrap_or(&0)
}
