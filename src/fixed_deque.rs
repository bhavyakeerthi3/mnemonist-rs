use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FixedDeque {
    capacity: usize,
    items: Vec<Option<Value>>,
    start: usize,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedDequeError(pub String);

impl std::fmt::Display for FixedDequeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FixedDequeError {}

impl FixedDeque {
    pub fn new(capacity: usize) -> Result<Self, FixedDequeError> {
        if capacity == 0 {
            return Err(FixedDequeError(
                "mnemonist/fixed-deque: `capacity` should be a positive number.".into(),
            ));
        }

        Ok(Self {
            capacity,
            items: vec![None; capacity],
            start: 0,
            size: 0,
        })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.size = 0;
    }

    fn tail_index(&self) -> usize {
        let mut index = self.start + self.size - 1;
        if index >= self.capacity {
            index -= self.capacity;
        }
        index
    }

    fn push_index(&self) -> usize {
        let mut index = self.start + self.size;
        if index >= self.capacity {
            index -= self.capacity;
        }
        index
    }

    pub fn push(&mut self, item: Value) -> Result<usize, FixedDequeError> {
        if self.size == self.capacity {
            return Err(FixedDequeError(format!(
                "mnemonist/fixed-deque.push: deque capacity ({}) exceeded!",
                self.capacity
            )));
        }

        let index = self.push_index();
        self.items[index] = Some(item);
        self.size += 1;
        Ok(self.size)
    }

    pub fn unshift(&mut self, item: Value) -> Result<usize, FixedDequeError> {
        if self.size == self.capacity {
            return Err(FixedDequeError(format!(
                "mnemonist/fixed-deque.unshift: deque capacity ({}) exceeded!",
                self.capacity
            )));
        }

        let index = if self.start == 0 {
            self.capacity - 1
        } else {
            self.start - 1
        };

        self.items[index] = Some(item);
        self.start = index;
        self.size += 1;
        Ok(self.size)
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        let mut index = self.start + self.size;
        if index >= self.capacity {
            index -= self.capacity;
        }
        self.items[index].take()
    }

    pub fn shift(&mut self) -> Option<Value> {
        if self.size == 0 {
            return None;
        }

        let index = self.start;
        self.size -= 1;
        self.start += 1;
        if self.start == self.capacity {
            self.start = 0;
        }
        self.items[index].take()
    }

    pub fn peek_first(&self) -> Option<&Value> {
        if self.size == 0 {
            return None;
        }
        self.items[self.start].as_ref()
    }

    pub fn peek_last(&self) -> Option<&Value> {
        if self.size == 0 {
            return None;
        }
        self.items[self.tail_index()].as_ref()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        if self.size == 0 || index >= self.capacity {
            return None;
        }

        let mut idx = self.start + index;
        if idx >= self.capacity {
            idx -= self.capacity;
        }
        self.items[idx].as_ref()
    }

    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(&Value, usize),
    {
        let c = self.capacity;
        let l = self.size;
        let mut i = self.start;
        let mut j = 0;

        while j < l {
            if let Some(ref item) = self.items[i] {
                callback(item, j);
            }
            i += 1;
            j += 1;
            if i == c {
                i = 0;
            }
        }
    }

    pub fn to_array(&self) -> Vec<Value> {
        let offset = self.start + self.size;
        if offset < self.capacity {
            return self.items[self.start..offset]
                .iter()
                .filter_map(|v| v.clone())
                .collect();
        }

        let mut array = Vec::with_capacity(self.size);
        let c = self.capacity;
        let l = self.size;
        let mut i = self.start;
        let mut j = 0;

        while j < l {
            if let Some(ref item) = self.items[i] {
                array.push(item.clone());
            }
            i += 1;
            j += 1;
            if i == c {
                i = 0;
            }
        }
        array
    }

    pub fn values(&self) -> impl Iterator<Item = &Value> {
        DequeValues {
            deque: self,
            remaining: self.size,
            index: self.start,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        DequeEntries {
            deque: self,
            remaining: self.size,
            index: self.start,
            j: 0,
        }
    }

    pub fn from_iter<I>(iterable: I, capacity: Option<usize>) -> Result<Self, FixedDequeError>
    where
        I: IntoIterator<Item = Value>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        let cap = capacity.unwrap_or_else(|| iter.len());

        if cap == 0 {
            return Err(FixedDequeError(
                "mnemonist/fixed-deque.from: could not guess iterable length. Please provide desired capacity as last argument.".into(),
            ));
        }

        let mut deque = Self::new(cap)?;
        for (i, value) in iter.enumerate() {
            deque.items[i] = Some(value);
            deque.size = i + 1;
        }
        Ok(deque)
    }
}

struct DequeValues<'a> {
    deque: &'a FixedDeque,
    remaining: usize,
    index: usize,
}

impl<'a> Iterator for DequeValues<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.deque.items[self.index].as_ref()?;
        self.index += 1;
        self.remaining -= 1;
        if self.index == self.deque.capacity {
            self.index = 0;
        }
        Some(value)
    }
}

struct DequeEntries<'a> {
    deque: &'a FixedDeque,
    remaining: usize,
    index: usize,
    j: usize,
}

impl<'a> Iterator for DequeEntries<'a> {
    type Item = (usize, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.deque.items[self.index].as_ref()?;
        let entry = (self.j, value);
        self.j += 1;
        self.index += 1;
        self.remaining -= 1;
        if self.index == self.deque.capacity {
            self.index = 0;
        }
        Some(entry)
    }
}
