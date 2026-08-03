use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
    pub value: Value,
}

#[derive(Debug, Clone, Default)]
pub struct StaticIntervalTree {
    intervals: Vec<Interval>,
    tree: Vec<usize>,
    max_ends: Vec<usize>,
    height: usize,
}

impl StaticIntervalTree {
    pub fn new(intervals: impl IntoIterator<Item = Interval>) -> Self {
        let intervals: Vec<_> = intervals.into_iter().collect();
        let size = intervals.len();
        let height = (size + 1).next_power_of_two().ilog2() as usize;
        let mut tree = vec![usize::MAX; (1usize << height).saturating_sub(1)];
        let mut max_ends = vec![usize::MAX; size];
        let mut indices: Vec<_> = (0..size).collect();
        indices.sort_by(|left, right| intervals[*left].start.total_cmp(&intervals[*right].start));

        if size > 0 {
            build(
                &intervals,
                &indices,
                &mut tree,
                &mut max_ends,
                0,
                0,
                size - 1,
            );
        }

        Self {
            intervals,
            tree,
            max_ends,
            height,
        }
    }

    pub fn size(&self) -> usize {
        self.intervals.len()
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    pub fn query_point(&self, point: f64) -> Vec<&Interval> {
        self.query_interval(point, point)
    }

    pub fn query_interval(&self, start: f64, end: f64) -> Vec<&Interval> {
        if self.intervals.is_empty() {
            return Vec::new();
        }
        let mut matches = Vec::new();
        let mut stack = vec![0];
        while let Some(node) = stack.pop() {
            let Some(&index) = self.tree.get(node) else {
                continue;
            };
            if index == usize::MAX || start > self.intervals[self.max_ends[index]].end {
                continue;
            }
            let left = node * 2 + 1;
            if self
                .tree
                .get(left)
                .is_some_and(|index| *index != usize::MAX)
            {
                stack.push(left);
            }
            let interval = &self.intervals[index];
            if end >= interval.start && start <= interval.end {
                matches.push(interval);
            }
            if end < interval.start {
                continue;
            }
            let right = node * 2 + 2;
            if self
                .tree
                .get(right)
                .is_some_and(|index| *index != usize::MAX)
            {
                stack.push(right);
            }
        }
        matches
    }
}

fn build(
    intervals: &[Interval],
    indices: &[usize],
    tree: &mut [usize],
    max_ends: &mut [usize],
    node: usize,
    low: usize,
    high: usize,
) -> usize {
    let middle = low + (high - low) / 2;
    let index = indices[middle];
    tree[node] = index;
    let mut max_index = index;
    if low < middle {
        let left = build(
            intervals,
            indices,
            tree,
            max_ends,
            node * 2 + 1,
            low,
            middle - 1,
        );
        if intervals[left].end > intervals[max_index].end {
            max_index = left;
        }
    }
    if middle < high {
        let right = build(
            intervals,
            indices,
            tree,
            max_ends,
            node * 2 + 2,
            middle + 1,
            high,
        );
        if intervals[right].end > intervals[max_index].end {
            max_index = right;
        }
    }
    max_ends[index] = max_index;
    max_index
}
