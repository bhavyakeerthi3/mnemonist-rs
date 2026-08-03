use indexmap::IndexMap;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct MultiSet {
    items: IndexMap<Value, usize>,
    size: usize,
}

impl MultiSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.size = 0;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dimension(&self) -> usize {
        self.items.len()
    }

    pub fn add(&mut self, item: Value, count: isize) {
        if count == 0 {
            return;
        }
        if count < 0 {
            self.remove(&item, -count);
            return;
        }

        let count = count as usize;
        let entry = self.items.entry(item).or_insert(0);
        *entry += count;
        self.size += count;
    }

    pub fn set(&mut self, item: Value, count: isize) {
        if count <= 0 {
            self.delete(&item);
            return;
        }

        let count = count as usize;
        let previous = self.items.insert(item, count).unwrap_or(0);
        self.size = self.size - previous + count;
    }

    pub fn has(&self, item: &Value) -> bool {
        self.items.contains_key(item)
    }

    pub fn delete(&mut self, item: &Value) -> bool {
        let Some(count) = self.items.shift_remove(item) else {
            return false;
        };
        self.size -= count;
        true
    }

    pub fn remove(&mut self, item: &Value, count: isize) {
        if count == 0 {
            return;
        }
        if count < 0 {
            self.add(item.clone(), -count);
            return;
        }

        let Some(current) = self.items.get_mut(item) else {
            return;
        };

        let count = count as usize;
        if count >= *current {
            self.size -= *current;
            self.items.shift_remove(item);
        } else {
            *current -= count;
            self.size -= count;
        }
    }

    pub fn edit(&mut self, from: &Value, to: Value) {
        let amount = self.multiplicity(from);
        if amount == 0 {
            return;
        }

        self.items.shift_remove(from);
        let entry = self.items.entry(to).or_insert(0);
        *entry += amount;
    }

    pub fn multiplicity(&self, item: &Value) -> usize {
        self.items.get(item).copied().unwrap_or(0)
    }

    pub fn frequency(&self, item: &Value) -> f64 {
        if self.size == 0 {
            return 0.0;
        }
        self.multiplicity(item) as f64 / self.size as f64
    }

    pub fn top(&self, n: usize) -> Result<Vec<(Value, usize)>, String> {
        if n == 0 {
            return Err("mnemonist/multi-set.top: n must be a number > 0.".into());
        }

        // Upstream uses FixedReverseHeap rather than a stable sort. Reproduce
        // its bounded-heap mechanics so equal multiplicities keep the same
        // observable consume order as the JavaScript implementation.
        let mut heap = Vec::with_capacity(n.min(self.items.len()));
        for (value, count) in &self.items {
            let item = (value.clone(), *count);
            if heap.len() < n {
                heap.push(item);
                let last_index = heap.len() - 1;
                fixed_reverse_sift_down(&mut heap, 0, last_index);
            } else if fixed_reverse_compare(&item, &heap[0]).is_gt() {
                heap[0] = item;
                let size = heap.len();
                fixed_reverse_sift_up(&mut heap, size, 0);
            }
        }

        Ok(fixed_reverse_consume(heap))
    }

    pub fn values(&self) -> Vec<Value> {
        let mut result = Vec::with_capacity(self.size);
        for (value, count) in &self.items {
            for _ in 0..*count {
                result.push(value.clone());
            }
        }
        result
    }

    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.items.keys()
    }

    pub fn multiplicities(&self) -> impl Iterator<Item = (&Value, &usize)> {
        self.items.iter()
    }

    pub fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Value>,
    {
        let mut set = Self::new();
        for value in iterable {
            set.add(value, 1);
        }
        set
    }

    pub fn is_subset(a: &Self, b: &Self) -> bool {
        if a.dimension() > b.dimension() {
            return false;
        }
        a.items
            .iter()
            .all(|(value, count)| b.multiplicity(value) >= *count)
    }

    pub fn is_superset(a: &Self, b: &Self) -> bool {
        Self::is_subset(b, a)
    }
}

type MultiSetEntry = (Value, usize);

fn fixed_reverse_compare(a: &MultiSetEntry, b: &MultiSetEntry) -> std::cmp::Ordering {
    // This is reverseComparator(MULTISET_ITEM_COMPARATOR) from upstream.
    a.1.cmp(&b.1)
}

fn fixed_reverse_sift_down(items: &mut [MultiSetEntry], start: usize, mut index: usize) {
    let item = items[index].clone();

    while index > start {
        let parent_index = (index - 1) >> 1;
        if fixed_reverse_compare(&item, &items[parent_index]).is_lt() {
            items[index] = items[parent_index].clone();
            index = parent_index;
        } else {
            break;
        }
    }

    items[index] = item;
}

fn fixed_reverse_sift_up(items: &mut [MultiSetEntry], size: usize, mut index: usize) {
    let start = index;
    let item = items[index].clone();
    let mut child_index = 2 * index + 1;

    while child_index < size {
        let right_index = child_index + 1;
        if right_index < size
            && fixed_reverse_compare(&items[child_index], &items[right_index]).is_ge()
        {
            child_index = right_index;
        }

        items[index] = items[child_index].clone();
        index = child_index;
        child_index = 2 * index + 1;
    }

    items[index] = item;
    fixed_reverse_sift_down(items, start, index);
}

fn fixed_reverse_consume(mut items: Vec<MultiSetEntry>) -> Vec<MultiSetEntry> {
    let mut size = items.len();
    let mut output = vec![None; size];

    while size > 0 {
        size -= 1;
        let mut last_item = items[size].clone();

        if size != 0 {
            let item = items[0].clone();
            items[0] = last_item;
            fixed_reverse_sift_up(&mut items, size, 0);
            last_item = item;
        }

        output[size] = Some(last_item);
    }

    output
        .into_iter()
        .map(|item| item.expect("heap consume fills every output slot"))
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::MultiSet;

    #[test]
    fn top_matches_upstream_tie_order() {
        let set = MultiSet::from_iter(
            "This is a very interesting albeit boring string."
                .chars()
                .map(|character| Value::String(character.to_string())),
        );

        assert_eq!(
            set.top(5).unwrap(),
            vec![
                (Value::String("i".into()), 7),
                (Value::String(" ".into()), 7),
                (Value::String("r".into()), 4),
                (Value::String("e".into()), 4),
                (Value::String("s".into()), 4),
            ]
        );
    }
}
