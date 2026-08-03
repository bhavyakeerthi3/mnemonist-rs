use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct StaticDisjointSet {
    size: usize,
    dimension: usize,
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl StaticDisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            dimension: size,
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut y = x;

        while self.parents[y] != y {
            y = self.parents[y];
        }

        let root = y;
        let mut current = x;
        while self.parents[current] != root {
            let parent = self.parents[current];
            self.parents[current] = root;
            current = parent;
        }

        root
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        self.dimension -= 1;

        if self.ranks[x_root] < self.ranks[y_root] {
            self.parents[x_root] = y_root;
        } else if self.ranks[x_root] > self.ranks[y_root] {
            self.parents[y_root] = x_root;
        } else {
            self.parents[y_root] = x_root;
            self.ranks[x_root] += 1;
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn mapping(&mut self) -> Vec<usize> {
        let mut ids: IndexMap<usize, usize> = IndexMap::new();
        let mut mapping = Vec::with_capacity(self.size);

        for i in 0..self.size {
            let root = self.find(i);
            let next = ids.len();
            let id = *ids.entry(root).or_insert(next);
            mapping.push(id);
        }

        mapping
    }

    pub fn compile(&mut self) -> Vec<Vec<usize>> {
        let mut ids: IndexMap<usize, usize> = IndexMap::new();
        let mut result: Vec<Vec<usize>> = Vec::with_capacity(self.dimension);

        for i in 0..self.size {
            let root = self.find(i);
            if let Some(id) = ids.get(&root).copied() {
                result[id].push(i);
            } else {
                let id = result.len();
                ids.insert(root, id);
                result.push(vec![i]);
            }
        }

        result
    }
}
