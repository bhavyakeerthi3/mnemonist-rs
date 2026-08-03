use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FixedStack {
    capacity: usize,
    items: Vec<Option<Value>>,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedStackError(pub String);

impl std::fmt::Display for FixedStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FixedStackError {}

impl FixedStack {
    pub fn new(capacity: usize) -> Result<Self, FixedStackError> {
        if capacity == 0 {
            return Err(FixedStackError(
                "mnemonist/fixed-stack: `capacity` should be a positive number.".into(),
            ));
        }

        Ok(Self {
            capacity,
            items: vec![None; capacity],
            size: 0,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn push(&mut self, item: Value) -> Result<usize, FixedStackError> {
        if self.size == self.capacity {
            return Err(FixedStackError(format!(
                "mnemonist/fixed-stack.push: stack capacity ({}) exceeded!",
                self.capacity
            )));
        }

        self.items[self.size] = Some(item);
        self.size += 1;
        Ok(self.size)
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.items[self.size].take()
    }

    pub fn peek(&self) -> Option<&Value> {
        if self.size == 0 {
            return None;
        }
        self.items[self.size - 1].as_ref()
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, usize),
    {
        let null = Value::Null;
        let l = self.capacity;
        for i in 0..l {
            let idx = l - i - 1;
            let item = self.items[idx].as_ref().unwrap_or(&null);
            callback(item, i);
        }
    }

    pub fn to_array(&self) -> Vec<Value> {
        let mut array = Vec::with_capacity(self.size);
        let mut i = self.size;
        while i > 0 {
            i -= 1;
            if let Some(ref v) = self.items[i] {
                array.push(v.clone());
            }
        }
        array
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        (0..self.size).rev().filter_map(|i| self.items[i].as_ref())
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        (0..self.size)
            .rev()
            .enumerate()
            .filter_map(|(i, idx)| self.items[idx].as_ref().map(|v| (i, v)))
    }

    pub fn from_iter<I>(iterable: I, capacity: Option<usize>) -> Result<Self, FixedStackError>
    where
        I: IntoIterator<Item = Value>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        let cap = capacity.unwrap_or_else(|| iter.len());

        if cap == 0 {
            return Err(FixedStackError(
                "mnemonist/fixed-stack.from: could not guess iterable length. Please provide desired capacity as last argument.".into(),
            ));
        }

        let mut stack = Self::new(cap)?;
        for (i, value) in iter.enumerate() {
            stack.items[i] = Some(value);
            stack.size = i + 1;
        }
        Ok(stack)
    }
}
