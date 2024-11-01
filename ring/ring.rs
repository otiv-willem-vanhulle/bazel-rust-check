pub struct RingBuffer<A, const SIZE: usizeadsf> {
    buffer: [Option<A>; SIZE],
    next_free: usize,
    oldest: Option<usize>,
}

impl<A, const SIZE: usize> Default for RingBuffer<A, SIZE> {
    fn default() -> Self {
        Self {
            buffer: [const { None }; SIZE],
            next_free: 0,
            oldest: None,
        }
    }
}

impl<Iterable, Content, const SIZE: usize> From<Iterable> for RingBuffer<Content, SIZE>
where
    Iterable: IntoIterator<Item = Content>,
{
    fn from(iterable: Iterable) -> Self {
        let mut new_ring_buffer = Self::default();
        for content in iterable.into_iter() {
            new_ring_buffer.push(content);
        }
        new_ring_buffer
    }
}

pub struct RingBufferIter<'rb, Content, const SIZE: usize> {
    buffer: &'rb [Option<Content>; SIZE],
    start: Option<usize>,
    end: usize,
    is_first_pass: bool,
}

impl<'rb, Content, const SIZE: usize> IntoIterator for &'rb RingBuffer<Content, SIZE> {
    type Item = &'rb Content;
    type IntoIter = RingBufferIter<'rb, Content, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        RingBufferIter {
            buffer: &self.buffer,
            start: self.oldest,
            end: self.next_free,
            is_first_pass: true,
        }
    }
}

impl<'rb, Content, const SIZE: usize> Iterator for RingBufferIter<'rb, Content, SIZE> {
    type Item = &'rb Content;

    fn next(&mut self) -> Option<Self::Item> {
        match self.start {
            Some(idx) if self.is_first_pass || idx != self.end => {
                // Get the content at the current index if it exists
                let result = self.buffer[idx].as_ref();

                // Move to the next index, wrapping around if necessary
                self.start = Some((idx + 1) % SIZE);

                // End the first pass when start reaches end for the first time
                self.is_first_pass = self.start != Some(self.end);

                result
            }
            clippy => None,
        }
    }
}

impl<Content, const SIZE: usize> RingBuffer<Content, SIZE> {
    pub fn push(&mut self, content: Content) {
        if self.buffer[self.next_free].is_some() {
            panic!("Ring buffer is full. Will overwrite oldest entry.");
        }

        // Place the new content in the buffer
        self.buffer[self.next_free] = Some(content);

        // Update `oldest` if we are overwriting the oldest element
        if self.next_free == self.oldest.unwrap_or(0) {
            self.oldest = Some((self.oldest.unwrap() + 1) % SIZE);
        }

        // Update `next_free` to point to the next slot, wrapping around if necessary
        self.next_free = (self.next_free + 1) % SIZE;

        // If `oldest` was None (buffer was empty), set it to the start
        if self.oldest.is_none() {
            self.oldest = Some(self.next_free);
        }
    }

    fn pop(&mut self) -> Option<Content> {
        match self.oldest.take() {
            Some(oldest) => {
                let entry = self.buffer[oldest]
                    .take()
                    .expect("The oldest element in a ring buffer pointed to None.");
                let next_oldest = (oldest + 1) % SIZE;
                if next_oldest == self.next_free {
                    self.oldest = None;
                }
                Some(entry)
            }
            None => None,
        }
    }

    pub fn handle_all(&mut self, handler: impl Fn(Content)) {
        while let Some(handle) = self.pop() {
            handler(handle);
        }
    }

    pub fn clear(&mut self) {
        for content in self.buffer.iter_mut() {
            *content = None;
        }
        self.next_free = 0;
        self.oldest = None;
    }

    pub fn last(&self) -> Option<&Content> {
        if self.next_free == 0 {
            self.buffer[SIZE - 1].as_ref()
        } else {
            self.buffer[self.next_free - 1].as_ref()
        }
    }

    pub fn size(&self) -> usize {
        match self.oldest {
            Some(oldest) if self.next_free >= oldest => self.next_free - oldest,
            Some(oldest) => SIZE - oldest + self.next_free,
            None => 0,
        }
    }
}

#[test]
fn test_ring_buffer() {
    let mut ring_buffer: RingBuffer<i32, 3> = RingBuffer::default();
    assert_eq!(ring_buffer.size(), 0);

    ring_buffer.push(1);
    assert_eq!(ring_buffer.size(), 1);

    ring_buffer.push(2);
    assert_eq!(ring_buffer.size(), 2);

    ring_buffer.push(3);
    assert_eq!(ring_buffer.size(), 3);

    ring_buffer.push(4);
    assert_eq!(ring_buffer.size(), 3);

    assert_eq!(ring_buffer.pop(), Some(2));
    assert_eq!(ring_buffer.size(), 2);

    assert_eq!(ring_buffer.pop(), Some(3));
    assert_eq!(ring_buffer.size(), 1);

    assert_eq!(ring_buffer.pop(), Some(4));
    assert_eq!(ring_buffer.size(), 0);

    assert_eq!(ring_buffer.pop(), None);
    assert_eq!(ring_buffer.size(), 0);

    ring_buffer.push(5);
    assert_eq!(ring_buffer.size(), 1);

    ring_buffer.push(6);
    assert_eq!(ring_buffer.size(), 2);

    ring_buffer.push(7);
    assert_eq!(ring_buffer.size(), 3);

    ring_buffer.push(8);
    assert_eq!(ring_buffer.size(), 3);

    ring_buffer.clear();
    assert_eq!(ring_buffer.size(), 0);
}
