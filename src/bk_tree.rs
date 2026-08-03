use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct BkNode {
    word: String,
    children: BTreeMap<usize, Box<BkNode>>,
}

/// A Burkhard-Keller tree keyed by Levenshtein edge distance.
#[derive(Debug, Clone, Default)]
pub struct BkTree {
    root: Option<Box<BkNode>>,
    words: Vec<String>,
}

impl BkTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.words.clear();
    }

    pub fn add(&mut self, word: impl Into<String>) {
        let word = word.into();
        self.words.push(word.clone());

        let Some(mut node) = self.root.as_deref_mut() else {
            self.root = Some(Box::new(BkNode {
                word,
                children: BTreeMap::new(),
            }));
            return;
        };

        loop {
            let distance = levenshtein(&word, &node.word);
            if node.children.contains_key(&distance) {
                node = node
                    .children
                    .get_mut(&distance)
                    .expect("the child existed immediately before this lookup");
            } else {
                node.children.insert(
                    distance,
                    Box::new(BkNode {
                        word,
                        children: BTreeMap::new(),
                    }),
                );
                return;
            }
        }
    }

    pub fn search(&self, query: &str, radius: usize) -> Vec<(String, usize)> {
        let mut results = Vec::new();
        search_node(&self.root, query, radius, &mut results, &mut 0);
        results.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
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

    #[cfg(test)]
    fn search_with_visits(&self, query: &str, radius: usize) -> (Vec<(String, usize)>, usize) {
        let mut results = Vec::new();
        let mut visits = 0;
        search_node(&self.root, query, radius, &mut results, &mut visits);
        results.sort_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(&right.0)));
        (results, visits)
    }
}

fn search_node(
    node: &Option<Box<BkNode>>,
    query: &str,
    radius: usize,
    results: &mut Vec<(String, usize)>,
    visits: &mut usize,
) {
    let Some(node) = node.as_deref() else {
        return;
    };

    *visits += 1;
    let distance = levenshtein(&node.word, query);
    if distance <= radius {
        results.push((node.word.clone(), distance));
    }

    let lower = distance.saturating_sub(radius);
    let upper = distance.saturating_add(radius);
    for child in node.children.range(lower..=upper).map(|(_, child)| child) {
        search_node_box(child, query, radius, results, visits);
    }
}

fn search_node_box(
    node: &BkNode,
    query: &str,
    radius: usize,
    results: &mut Vec<(String, usize)>,
    visits: &mut usize,
) {
    *visits += 1;
    let distance = levenshtein(&node.word, query);
    if distance <= radius {
        results.push((node.word.clone(), distance));
    }

    let lower = distance.saturating_sub(radius);
    let upper = distance.saturating_add(radius);
    for child in node.children.range(lower..=upper).map(|(_, child)| child) {
        search_node_box(child, query, radius, results, visits);
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

#[cfg(test)]
mod tests {
    use super::BkTree;

    #[test]
    fn radius_search_prunes_incompatible_edges() {
        let tree = BkTree::from_iter([
            "book", "back", "books", "cake", "cart", "cape", "cook", "cookie", "boon",
        ]);

        let (matches, visits) = tree.search_with_visits("book", 1);

        assert_eq!(
            matches,
            vec![
                ("book".to_owned(), 0),
                ("books".to_owned(), 1),
                ("boon".to_owned(), 1),
                ("cook".to_owned(), 1),
            ]
        );
        assert!(
            visits < tree.size(),
            "the BK search should skip impossible edge ranges"
        );
    }

    #[test]
    fn duplicate_words_remain_observable_entries() {
        let tree = BkTree::from_iter(["same", "same"]);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.search("same", 0).len(), 2);
    }
}
