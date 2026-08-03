use std::cmp::Ordering;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct KdPoint {
    pub coordinates: Vec<f64>,
    pub value: Value,
}

#[derive(Debug, Clone)]
struct KdNode {
    point: KdPoint,
    order: usize,
    axis: usize,
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
}

/// A median-split KD tree. `add` rebuilds the static tree so the public
/// convenience API retains balanced search behavior without unsafe mutation.
#[derive(Debug, Clone, Default)]
pub struct KdTree {
    points: Vec<KdPoint>,
    root: Option<Box<KdNode>>,
}

impl KdTree {
    pub fn new(points: impl IntoIterator<Item = (Vec<f64>, Value)>) -> Self {
        let points: Vec<_> = points
            .into_iter()
            .map(|(coordinates, value)| KdPoint { coordinates, value })
            .collect();
        let dimensions = points
            .first()
            .map(|point| point.coordinates.len().max(1))
            .unwrap_or(0);
        let root = build(
            points
                .iter()
                .cloned()
                .enumerate()
                .map(|(order, point)| (point, order))
                .collect(),
            0,
            dimensions,
        );

        Self { points, root }
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn add(&mut self, coordinates: Vec<f64>, value: Value) {
        self.points.push(KdPoint { coordinates, value });
        *self = Self::new(
            self.points
                .iter()
                .cloned()
                .map(|point| (point.coordinates, point.value)),
        );
    }

    pub fn nearest(&self, coordinates: &[f64], k: usize) -> Vec<(&Value, f64)> {
        if k == 0 || self.root.is_none() {
            return Vec::new();
        }

        let mut candidates = Vec::with_capacity(k.min(self.points.len()));
        search_nearest(&self.root, coordinates, k, &mut candidates, &mut 0);
        candidates.sort_by(compare_candidates);
        candidates
            .into_iter()
            .map(|candidate| {
                (
                    &candidate.node.point.value,
                    candidate.distance_squared.sqrt(),
                )
            })
            .collect()
    }

    #[cfg(test)]
    fn nearest_with_visits(&self, coordinates: &[f64], k: usize) -> (Vec<(&Value, f64)>, usize) {
        let mut candidates = Vec::with_capacity(k.min(self.points.len()));
        let mut visits = 0;
        search_nearest(&self.root, coordinates, k, &mut candidates, &mut visits);
        candidates.sort_by(compare_candidates);
        (
            candidates
                .into_iter()
                .map(|candidate| {
                    (
                        &candidate.node.point.value,
                        candidate.distance_squared.sqrt(),
                    )
                })
                .collect(),
            visits,
        )
    }
}

#[derive(Clone, Copy)]
struct Candidate<'a> {
    node: &'a KdNode,
    distance_squared: f64,
}

fn build(
    mut entries: Vec<(KdPoint, usize)>,
    axis: usize,
    dimensions: usize,
) -> Option<Box<KdNode>> {
    if entries.is_empty() {
        return None;
    }

    entries.sort_by(|(left, left_order), (right, right_order)| {
        coordinate(left, axis)
            .total_cmp(&coordinate(right, axis))
            .then_with(|| left_order.cmp(right_order))
    });
    let middle = entries.len() / 2;
    let right = entries.split_off(middle + 1);
    let (point, order) = entries
        .pop()
        .expect("median exists for a non-empty KD tree");
    let next_axis = (axis + 1) % dimensions;

    Some(Box::new(KdNode {
        point,
        order,
        axis,
        left: build(entries, next_axis, dimensions),
        right: build(right, next_axis, dimensions),
    }))
}

fn search_nearest<'a>(
    node: &'a Option<Box<KdNode>>,
    query: &[f64],
    k: usize,
    candidates: &mut Vec<Candidate<'a>>,
    visits: &mut usize,
) {
    let Some(node) = node.as_deref() else {
        return;
    };

    *visits += 1;
    let candidate = Candidate {
        node,
        distance_squared: squared_euclidean(&node.point.coordinates, query),
    };
    if candidates.len() < k {
        candidates.push(candidate);
    } else {
        let worst = candidates
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| compare_candidates(left, right))
            .map(|(index, _)| index)
            .expect("a full candidate set has a worst element");
        if compare_candidates(&candidate, &candidates[worst]) == Ordering::Less {
            candidates[worst] = candidate;
        }
    }

    let split = coordinate(&node.point, node.axis);
    let query_value = query.get(node.axis).copied().unwrap_or_default();
    let (near, far) = if query_value < split {
        (&node.left, &node.right)
    } else {
        (&node.right, &node.left)
    };
    search_nearest(near, query, k, candidates, visits);

    let plane_distance_squared = (query_value - split).powi(2);
    let worst_distance_squared = candidates
        .iter()
        .map(|candidate| candidate.distance_squared)
        .max_by(f64::total_cmp)
        .unwrap_or(f64::INFINITY);
    if candidates.len() < k || plane_distance_squared <= worst_distance_squared {
        search_nearest(far, query, k, candidates, visits);
    }
}

fn compare_candidates(left: &Candidate<'_>, right: &Candidate<'_>) -> Ordering {
    left.distance_squared
        .total_cmp(&right.distance_squared)
        // Mnemonist's pivot traversal retains the later source point for an
        // equal-distance fixed-size neighbor set.
        .then_with(|| right.node.order.cmp(&left.node.order))
}

fn coordinate(point: &KdPoint, axis: usize) -> f64 {
    point.coordinates.get(axis).copied().unwrap_or_default()
}

fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum()
}

pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    squared_euclidean(a, b).sqrt()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::KdTree;

    #[test]
    fn nearest_neighbor_search_prunes_distant_branches() {
        let tree = KdTree::new(
            (0..127).map(|index| (vec![index as f64 * 10.0, (index % 3) as f64], json!(index))),
        );

        let (nearest, visits) = tree.nearest_with_visits(&[0.1, 0.0], 1);

        assert_eq!(nearest[0].0, &json!(0));
        assert!(
            visits < tree.size() / 2,
            "visited {visits} of {} points",
            tree.size()
        );
    }

    #[test]
    fn add_rebuilds_a_balanced_search_tree() {
        let mut tree = KdTree::new([(vec![0.0, 0.0], json!("origin"))]);
        tree.add(vec![10.0, 0.0], json!("far"));
        tree.add(vec![1.0, 0.0], json!("near"));

        assert_eq!(tree.nearest(&[0.8, 0.0], 1)[0].0, &json!("near"));
    }
}
