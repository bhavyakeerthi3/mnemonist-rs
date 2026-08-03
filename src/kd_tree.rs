use serde_json::Value;

#[derive(Debug, Clone)]
pub struct KdPoint {
    pub coordinates: Vec<f64>,
    pub value: Value,
}

#[derive(Debug, Clone, Default)]
pub struct KdTree {
    points: Vec<KdPoint>,
}

impl KdTree {
    pub fn new(points: impl IntoIterator<Item = (Vec<f64>, Value)>) -> Self {
        Self {
            points: points
                .into_iter()
                .map(|(coordinates, value)| KdPoint { coordinates, value })
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn add(&mut self, coordinates: Vec<f64>, value: Value) {
        self.points.push(KdPoint { coordinates, value });
    }

    pub fn nearest(&self, coordinates: &[f64], k: usize) -> Vec<(&Value, f64)> {
        let mut scored: Vec<_> = self
            .points
            .iter()
            .map(|point| (&point.value, euclidean(&point.coordinates, coordinates)))
            .collect();
        scored.sort_by(|a, b| a.1.total_cmp(&b.1));
        scored.truncate(k);
        scored
    }
}

pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}
