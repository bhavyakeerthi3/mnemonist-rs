use indexmap::IndexSet;
use serde_json::Value;

pub fn intersection(sets: &[IndexSet<Value>]) -> Result<IndexSet<Value>, String> {
    if sets.len() < 2 {
        return Err("mnemonist/Set.intersection: needs at least two arguments.".into());
    }

    let mut result = IndexSet::new();
    let mut smallest: Option<&IndexSet<Value>> = None;

    for set in sets {
        if set.is_empty() {
            return Ok(result);
        }
        if smallest.map(|s| set.len() < s.len()).unwrap_or(true) {
            smallest = Some(set);
        }
    }

    let smallest = smallest.unwrap();
    for item in smallest {
        if sets.iter().all(|set| set == smallest || set.contains(item)) {
            result.insert(item.clone());
        }
    }
    Ok(result)
}

pub fn union(sets: &[IndexSet<Value>]) -> Result<IndexSet<Value>, String> {
    if sets.len() < 2 {
        return Err("mnemonist/Set.union: needs at least two arguments.".into());
    }

    let mut result = IndexSet::new();
    for set in sets {
        for item in set {
            result.insert(item.clone());
        }
    }
    Ok(result)
}

pub fn difference(a: &IndexSet<Value>, b: &IndexSet<Value>) -> IndexSet<Value> {
    if a.is_empty() {
        return IndexSet::new();
    }
    if b.is_empty() {
        return a.clone();
    }
    a.iter()
        .filter(|item| !b.contains(*item))
        .cloned()
        .collect()
}

pub fn symmetric_difference(a: &IndexSet<Value>, b: &IndexSet<Value>) -> IndexSet<Value> {
    let mut result = IndexSet::new();
    for item in a {
        if !b.contains(item) {
            result.insert(item.clone());
        }
    }
    for item in b {
        if !a.contains(item) {
            result.insert(item.clone());
        }
    }
    result
}

pub fn is_subset(a: &IndexSet<Value>, b: &IndexSet<Value>) -> bool {
    if a.len() > b.len() {
        return false;
    }
    a.iter().all(|item| b.contains(item))
}

pub fn is_superset(a: &IndexSet<Value>, b: &IndexSet<Value>) -> bool {
    is_subset(b, a)
}

pub fn add(a: &mut IndexSet<Value>, b: &IndexSet<Value>) {
    for item in b {
        a.insert(item.clone());
    }
}

pub fn subtract(a: &mut IndexSet<Value>, b: &IndexSet<Value>) {
    for item in b {
        a.shift_remove(item);
    }
}

pub fn intersect(a: &mut IndexSet<Value>, b: &IndexSet<Value>) {
    a.retain(|item| b.contains(item));
}

pub fn disjunct(a: &mut IndexSet<Value>, b: &IndexSet<Value>) {
    let to_remove: Vec<_> = a.iter().filter(|item| b.contains(*item)).cloned().collect();
    for item in b {
        if !a.contains(item) {
            a.insert(item.clone());
        }
    }
    for item in to_remove {
        a.shift_remove(&item);
    }
}

pub fn intersection_size(a: &IndexSet<Value>, b: &IndexSet<Value>) -> usize {
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if small.is_empty() {
        return 0;
    }
    if a == b {
        return a.len();
    }
    small.iter().filter(|item| large.contains(*item)).count()
}

pub fn union_size(a: &IndexSet<Value>, b: &IndexSet<Value>) -> usize {
    a.len() + b.len() - intersection_size(a, b)
}

pub fn jaccard(a: &IndexSet<Value>, b: &IndexSet<Value>) -> f64 {
    let i = intersection_size(a, b);
    if i == 0 {
        return 0.0;
    }
    i as f64 / union_size(a, b) as f64
}

pub fn overlap(a: &IndexSet<Value>, b: &IndexSet<Value>) -> f64 {
    let i = intersection_size(a, b);
    if i == 0 {
        return 0.0;
    }
    i as f64 / a.len().min(b.len()) as f64
}

pub fn chars_set(s: &str) -> IndexSet<Value> {
    s.chars().map(|c| Value::String(c.to_string())).collect()
}
