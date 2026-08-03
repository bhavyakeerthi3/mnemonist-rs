use serde_json::Value;

use crate::bk_tree::levenshtein;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VpNeighbor {
    pub distance: usize,
    pub item: String,
}

/// A direct port of Mnemonist's immutable Vantage Point tree for string data
/// using its standard Levenshtein metric.
#[derive(Debug, Clone, Default)]
pub struct StringVpTree {
    items: Vec<String>,
    nodes: Vec<usize>,
    lefts: Vec<usize>,
    rights: Vec<usize>,
    mus: Vec<f64>,
}

impl StringVpTree {
    pub fn new(items: impl IntoIterator<Item = String>) -> Self {
        let items: Vec<_> = items.into_iter().collect();
        let size = items.len();
        let mut tree = Self {
            items,
            nodes: vec![0; size],
            lefts: vec![0; size],
            rights: vec![0; size],
            mus: vec![0.0; size],
        };

        if size > 0 {
            tree.create_binary_tree();
        }

        tree
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn nearest_neighbors(&self, k: usize, query: &str) -> Vec<VpNeighbor> {
        if k == 0 || self.items.is_empty() {
            return Vec::new();
        }

        let mut heap = Vec::new();
        let mut stack = vec![0];
        let mut tau = f64::INFINITY;

        while let Some(node) = stack.pop() {
            let item = self.nodes[node];
            let distance = levenshtein(&self.items[item], query) as f64;

            if distance < tau {
                sift_down(
                    &mut heap,
                    VpNeighbor {
                        distance: distance as usize,
                        item: self.items[item].clone(),
                    },
                );
                if heap.len() > k {
                    pop(&mut heap);
                }
                if heap.len() >= k {
                    tau = heap[0].distance as f64;
                }
            }

            let left = self.lefts[node];
            let right = self.rights[node];
            if left == 0 && right == 0 {
                continue;
            }

            let mu = self.mus[node];
            if distance < mu {
                if left != 0 && distance < mu + tau {
                    stack.push(left);
                }
                if right != 0 && distance >= mu - tau {
                    stack.push(right);
                }
            } else {
                if right != 0 && distance >= mu - tau {
                    stack.push(right);
                }
                if left != 0 && distance < mu + tau {
                    stack.push(left);
                }
            }
        }

        let mut result = Vec::with_capacity(heap.len());
        while let Some(neighbor) = pop(&mut heap) {
            result.push(neighbor);
        }
        result.reverse();
        result
    }

    pub fn neighbors(&self, radius: usize, query: &str) -> Vec<VpNeighbor> {
        if self.items.is_empty() {
            return Vec::new();
        }

        let mut neighbors = Vec::new();
        let mut stack = vec![0];
        let radius = radius as f64;

        while let Some(node) = stack.pop() {
            let item = self.nodes[node];
            let distance = levenshtein(&self.items[item], query) as f64;
            if distance <= radius {
                neighbors.push(VpNeighbor {
                    distance: distance as usize,
                    item: self.items[item].clone(),
                });
            }

            let left = self.lefts[node];
            let right = self.rights[node];
            if left == 0 && right == 0 {
                continue;
            }

            let mu = self.mus[node];
            if distance < mu {
                if left != 0 && distance < mu + radius {
                    stack.push(left);
                }
                if right != 0 && distance >= mu - radius {
                    stack.push(right);
                }
            } else {
                if right != 0 && distance >= mu - radius {
                    stack.push(right);
                }
                if left != 0 && distance < mu + radius {
                    stack.push(left);
                }
            }
        }

        neighbors
    }

    fn create_binary_tree(&mut self) {
        let size = self.items.len();
        let mut indices: Vec<_> = (0..size).collect();
        let mut distances = vec![0.0; size];
        let mut stack = vec![(0, 0, size)];
        let mut created = 0;

        while let Some((node, lo, mut hi)) = stack.pop() {
            let vantage_point = indices[hi - 1];
            hi -= 1;
            let length = hi - lo;
            self.nodes[node] = vantage_point;

            if length == 0 {
                continue;
            }
            if length == 1 {
                self.mus[node] =
                    levenshtein(&self.items[vantage_point], &self.items[indices[lo]]) as f64;
                created += 1;
                self.rights[node] = created;
                self.nodes[created] = indices[lo];
                continue;
            }

            for index in lo..hi {
                let item = indices[index];
                distances[item] = levenshtein(&self.items[vantage_point], &self.items[item]) as f64;
            }
            quick_sort_indices(&distances, &mut indices, lo, hi);

            let mu = if length % 2 == 0 {
                let median = lo + length / 2 - 1;
                (distances[indices[median]] + distances[indices[median + 1]]) / 2.0
            } else {
                distances[indices[lo + length / 2]]
            };
            self.mus[node] = mu;
            let mid = lower_bound(&distances, &indices, mu, lo, hi);

            if hi > mid {
                created += 1;
                self.rights[node] = created;
                stack.push((created, mid, hi));
            }
            if mid > lo {
                created += 1;
                self.lefts[node] = created;
                stack.push((created, lo, mid));
            }
        }
    }
}

fn quick_sort_indices(values: &[f64], indices: &mut [usize], lo: usize, hi: usize) {
    let mut ranges = vec![(lo, hi)];
    while let Some((mut left, end)) = ranges.pop() {
        if left >= end {
            continue;
        }
        let mut right = end - 1;
        if left >= right {
            continue;
        }

        let item = indices[left];
        let pivot = values[item];
        while left < right {
            while values[indices[right]] >= pivot && left < right {
                right -= 1;
            }
            if left < right {
                indices[left] = indices[right];
                left += 1;
            }
            while values[indices[left]] <= pivot && left < right {
                left += 1;
            }
            if left < right {
                indices[right] = indices[left];
                right -= 1;
            }
        }
        indices[left] = item;
        ranges.push((left + 1, end));
        ranges.push((lo, left));
    }
}

fn lower_bound(
    values: &[f64],
    indices: &[usize],
    value: f64,
    mut lo: usize,
    mut hi: usize,
) -> usize {
    while lo < hi {
        let mid = (lo + hi) >> 1;
        if value <= values[indices[mid]] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn compare(left: &VpNeighbor, right: &VpNeighbor) -> i8 {
    if left.distance < right.distance {
        1
    } else if left.distance > right.distance {
        -1
    } else {
        0
    }
}

fn sift_down(heap: &mut Vec<VpNeighbor>, item: VpNeighbor) {
    let mut index = heap.len();
    heap.push(item.clone());
    while index > 0 {
        let parent = (index - 1) >> 1;
        if compare(&item, &heap[parent]) < 0 {
            heap[index] = heap[parent].clone();
            index = parent;
        } else {
            break;
        }
    }
    heap[index] = item;
}

fn sift_up(heap: &mut [VpNeighbor], mut index: usize) {
    let end = heap.len();
    let item = heap[index].clone();
    let start = index;
    let mut child = index * 2 + 1;
    while child < end {
        let right = child + 1;
        if right < end && compare(&heap[child], &heap[right]) >= 0 {
            child = right;
        }
        heap[index] = heap[child].clone();
        index = child;
        child = index * 2 + 1;
    }
    while index > start {
        let parent = (index - 1) >> 1;
        if compare(&item, &heap[parent]) < 0 {
            heap[index] = heap[parent].clone();
            index = parent;
        } else {
            break;
        }
    }
    heap[index] = item;
}

fn pop(heap: &mut Vec<VpNeighbor>) -> Option<VpNeighbor> {
    let last = heap.pop()?;
    if heap.is_empty() {
        return Some(last);
    }
    let item = std::mem::replace(&mut heap[0], last);
    sift_up(heap, 0);
    Some(item)
}

#[cfg(test)]
mod tests {
    use super::{StringVpTree, VpNeighbor};

    fn words() -> Vec<String> {
        [
            "book",
            "back",
            "bock",
            "lock",
            "mack",
            "shock",
            "ephemeral",
            "magistral",
            "shawarma",
            "falafel",
            "onze",
            "douze",
            "treize",
            "quatorze",
            "quinze",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }

    #[test]
    fn matches_mnemonist_nearest_neighbor_tie_order() {
        let tree = StringVpTree::new(words());

        assert_eq!(
            tree.nearest_neighbors(2, "look"),
            vec![
                VpNeighbor {
                    distance: 1,
                    item: "book".to_owned()
                },
                VpNeighbor {
                    distance: 1,
                    item: "lock".to_owned()
                },
            ]
        );
        assert_eq!(
            tree.nearest_neighbors(5, "look"),
            vec![
                VpNeighbor {
                    distance: 1,
                    item: "lock".to_owned()
                },
                VpNeighbor {
                    distance: 1,
                    item: "book".to_owned()
                },
                VpNeighbor {
                    distance: 2,
                    item: "bock".to_owned()
                },
                VpNeighbor {
                    distance: 3,
                    item: "mack".to_owned()
                },
                VpNeighbor {
                    distance: 3,
                    item: "back".to_owned()
                },
            ]
        );
    }

    #[test]
    fn finds_all_neighbors_within_radius() {
        let tree = StringVpTree::new(words());
        let mut neighbors = tree.neighbors(2, "look");
        neighbors.sort_by(|left, right| left.item.cmp(&right.item));
        assert_eq!(
            neighbors,
            vec![
                VpNeighbor {
                    distance: 2,
                    item: "bock".to_owned()
                },
                VpNeighbor {
                    distance: 1,
                    item: "book".to_owned()
                },
                VpNeighbor {
                    distance: 1,
                    item: "lock".to_owned()
                },
            ]
        );
    }
}

#[derive(Debug, Clone, Default)]
pub struct VpTree {
    points: Vec<(Vec<f64>, Value)>,
}

impl VpTree {
    pub fn new(points: impl IntoIterator<Item = (Vec<f64>, Value)>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn add(&mut self, point: Vec<f64>, value: Value) {
        self.points.push((point, value));
    }

    pub fn nearest(&self, point: &[f64], k: usize) -> Vec<(&Value, f64)> {
        let mut scored: Vec<_> = self
            .points
            .iter()
            .map(|(candidate, value)| (value, crate::kd_tree::euclidean(candidate, point)))
            .collect();
        scored.sort_by(|a, b| a.1.total_cmp(&b.1));
        scored.truncate(k);
        scored
    }
}
