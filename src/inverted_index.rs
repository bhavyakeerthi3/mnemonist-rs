use indexmap::{IndexMap, IndexSet};
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct InvertedIndex {
    documents: IndexMap<String, Value>,
    postings: IndexMap<String, IndexSet<String>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.documents.len()
    }

    pub fn clear(&mut self) {
        self.documents.clear();
        self.postings.clear();
    }

    pub fn add(&mut self, id: impl Into<String>, text: &str, value: Value) {
        let id = id.into();
        self.documents.insert(id.clone(), value);
        for token in tokenize(text) {
            self.postings.entry(token).or_default().insert(id.clone());
        }
    }

    pub fn get(&self, id: &str) -> Option<&Value> {
        self.documents.get(id)
    }

    pub fn search(&self, query: &str) -> Vec<&Value> {
        let mut ids: Option<IndexSet<String>> = None;

        for token in tokenize(query) {
            let Some(posting) = self.postings.get(&token) else {
                return Vec::new();
            };
            ids = Some(match ids {
                Some(current) => current
                    .intersection(posting)
                    .cloned()
                    .collect::<IndexSet<String>>(),
                None => posting.clone(),
            });
        }

        ids.unwrap_or_default()
            .iter()
            .filter_map(|id| self.documents.get(id))
            .collect()
    }
}

fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_lowercase())
}
