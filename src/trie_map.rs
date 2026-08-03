use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
struct Node {
    value: Option<Value>,
    children: IndexMap<char, Node>,
}

#[derive(Debug, Clone, Default)]
pub struct TrieMap {
    root: Node,
    entries: IndexMap<String, Value>,
}

impl TrieMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_iter<I, K>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (K, Value)>,
        K: Into<String>,
    {
        let mut trie = Self::new();
        for (key, value) in iterable {
            trie.set(key, value);
        }
        trie
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.root = Node::default();
        self.entries.clear();
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        let key = key.into();
        let mut node = &mut self.root;
        for token in key.chars() {
            node = node.children.entry(token).or_default();
        }
        node.value = Some(value.clone());
        self.entries.insert(key, value);
    }

    pub fn update<F>(&mut self, key: impl Into<String>, updater: F)
    where
        F: FnOnce(Option<Value>) -> Value,
    {
        let key = key.into();
        self.set(key.clone(), updater(self.get(&key).cloned()));
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if !self.entries.contains_key(key) {
            return false;
        }
        delete(&mut self.root, &mut key.chars());
        self.entries.shift_remove(key);
        true
    }

    pub fn find(&self, prefix: &str) -> Vec<(String, Value)> {
        let mut node = &self.root;
        for token in prefix.chars() {
            let Some(child) = node.children.get(&token) else {
                return Vec::new();
            };
            node = child;
        }
        let mut results = Vec::new();
        collect(node, prefix.to_owned(), &mut results);
        results
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub fn prefixes(&self) -> impl Iterator<Item = &String> {
        self.keys()
    }

    pub fn keys_with_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = &'a String> + 'a {
        self.entries
            .keys()
            .filter(move |key| key.starts_with(prefix))
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.entries.values()
    }

    pub fn values_with_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = &'a Value> + 'a {
        self.entries
            .iter()
            .filter(move |(key, _)| key.starts_with(prefix))
            .map(|(_, value)| value)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.entries.iter()
    }

    pub fn entries_with_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = (&'a String, &'a Value)> + 'a {
        self.entries
            .iter()
            .filter(move |(key, _)| key.starts_with(prefix))
    }
}

fn delete(node: &mut Node, tokens: &mut impl Iterator<Item = char>) -> bool {
    let Some(token) = tokens.next() else {
        node.value = None;
        return node.children.is_empty();
    };
    let should_prune = match node.children.get_mut(&token) {
        Some(child) => delete(child, tokens),
        None => false,
    };
    if should_prune {
        node.children.shift_remove(&token);
    }
    node.value.is_none() && node.children.is_empty()
}

fn collect(node: &Node, prefix: String, results: &mut Vec<(String, Value)>) {
    if let Some(value) = &node.value {
        results.push((prefix.clone(), value.clone()));
    }
    for (token, child) in node.children.iter().rev() {
        let mut next = prefix.clone();
        next.push(*token);
        collect(child, next, results);
    }
}
