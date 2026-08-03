use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct Trie {
    words: BTreeSet<String>,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn clear(&mut self) {
        self.words.clear();
    }

    pub fn add(&mut self, word: impl Into<String>) -> bool {
        self.words.insert(word.into())
    }

    pub fn has(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    pub fn delete(&mut self, word: &str) -> bool {
        self.words.remove(word)
    }

    pub fn find(&self, prefix: &str) -> Vec<String> {
        self.words
            .range(prefix.to_string()..)
            .take_while(|word| word.starts_with(prefix))
            .cloned()
            .collect()
    }

    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.words.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.words.iter()
    }

    pub fn prefixes(&self) -> impl Iterator<Item = &String> {
        self.words.iter()
    }

    pub fn keys_with_prefix<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = &'a String> + 'a {
        self.words
            .range(prefix.to_string()..)
            .take_while(move |word| word.starts_with(prefix))
    }

    pub fn from_iter<I, S>(iterable: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut trie = Self::new();
        for word in iterable {
            trie.add(word);
        }
        trie
    }
}
