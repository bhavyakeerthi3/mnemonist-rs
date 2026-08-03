use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MultiArray {
    containers: Vec<Vec<Value>>,
    size: usize,
}

impl MultiArray {
    pub fn new(width: usize) -> Self {
        Self {
            containers: vec![Vec::new(); width],
            size: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.containers.len()
    }

    pub fn dimension(&self) -> usize {
        self.containers.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        for container in &mut self.containers {
            container.clear();
        }
        self.size = 0;
    }

    pub fn push(&mut self, value: Value) {
        self.containers.push(vec![value]);
        self.size += 1;
    }

    pub fn set(&mut self, index: usize, value: Value) {
        if index >= self.containers.len() {
            self.containers.resize(index + 1, Vec::new());
        }
        self.containers[index].push(value);
        self.size += 1;
    }

    pub fn has(&self, index: usize) -> bool {
        self.containers.get(index).map_or(false, |c| !c.is_empty())
    }

    pub fn count(&self, index: usize) -> usize {
        self.containers.get(index).map_or(0, |c| c.len())
    }

    pub fn get(&self, index: usize) -> Option<&[Value]> {
        self.containers.get(index).map(Vec::as_slice)
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        self.containers
            .iter()
            .flat_map(|container| container.iter())
    }

    pub fn values_at(&self, index: usize) -> impl Iterator<Item = &Value> {
        self.containers
            .get(index)
            .map(|c| c.iter().rev())
            .into_iter()
            .flatten()
    }

    pub fn containers(&self) -> impl Iterator<Item = &[Value]> {
        self.containers.iter().map(Vec::as_slice)
    }

    pub fn keys(&self) -> impl Iterator<Item = usize> {
        0..self.dimension()
    }

    pub fn associations(&self) -> impl Iterator<Item = (usize, &[Value])> {
        self.containers.iter().enumerate().map(|(i, c)| (i, c.as_slice()))
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        self.containers
            .iter()
            .enumerate()
            .flat_map(|(i, container)| container.iter().rev().map(move |value| (i, value)))
    }
}
