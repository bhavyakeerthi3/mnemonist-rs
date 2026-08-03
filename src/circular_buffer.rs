use serde_json::Value;

/// Ring buffer that overwrites oldest entries when full (unlike `FixedDeque`).
#[derive(Debug, Clone)]
pub struct CircularBuffer {
    capacity: usize,
    items: Vec<Option<Value>>,
    start: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Result<Self, super::fixed_deque::FixedDequeError> {
        if capacity == 0 {
            return Err(super::fixed_deque::FixedDequeError(
                "mnemonist/circular-buffer: `capacity` should be a positive number.".into(),
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

    pub fn push(&mut self, item: Value) -> usize {
        let index = self.push_index();
        self.items[index] = Some(item);

        if self.size == self.capacity {
            let next_start = index + 1;
            if next_start >= self.capacity {
                self.start = 0;
            } else {
                self.start = next_start;
            }
            return self.size;
        }

        self.size += 1;
        self.size
    }

    pub fn unshift(&mut self, item: Value) -> usize {
        let index = if self.start == 0 {
            self.capacity - 1
        } else {
            self.start - 1
        };

        self.items[index] = Some(item);

        if self.size == self.capacity {
            self.start = index;
            return self.size;
        }

        self.start = index;
        self.size += 1;
        self.size
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
        CircularValues {
            buffer: self,
            remaining: self.size,
            index: self.start,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = (usize, &Value)> {
        CircularEntries {
            buffer: self,
            remaining: self.size,
            index: self.start,
            j: 0,
        }
    }

    pub fn from_iter<I>(
        iterable: I,
        capacity: Option<usize>,
    ) -> Result<Self, super::fixed_deque::FixedDequeError>
    where
        I: IntoIterator<Item = Value>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        let cap = capacity.unwrap_or_else(|| iter.len());

        if cap == 0 {
            return Err(super::fixed_deque::FixedDequeError(
                "mnemonist/circular-buffer.from: could not guess iterable length. Please provide desired capacity as last argument.".into(),
            ));
        }

        let mut buffer = Self::new(cap)?;
        for (i, value) in iter.enumerate() {
            buffer.items[i] = Some(value);
            buffer.size = i + 1;
        }
        Ok(buffer)
    }
}

struct CircularValues<'a> {
    buffer: &'a CircularBuffer,
    remaining: usize,
    index: usize,
}

impl<'a> Iterator for CircularValues<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.buffer.items[self.index].as_ref()?;
        self.index += 1;
        self.remaining -= 1;
        if self.index == self.buffer.capacity {
            self.index = 0;
        }
        Some(value)
    }
}

struct CircularEntries<'a> {
    buffer: &'a CircularBuffer,
    remaining: usize,
    index: usize,
    j: usize,
}

impl<'a> Iterator for CircularEntries<'a> {
    type Item = (usize, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.buffer.items[self.index].as_ref()?;
        let entry = (self.j, value);
        self.j += 1;
        self.index += 1;
        self.remaining -= 1;
        if self.index == self.buffer.capacity {
            self.index = 0;
        }
        Some(entry)
    }
}
