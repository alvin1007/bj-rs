#[derive(Debug, PartialEq, Eq)]
pub enum SegmentTreeError {
    IndexOutOfBounds,
    InvalidRange,
}

struct SegmentTree<T, F>
where 
    T: std::fmt::Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
{
    size: usize,
    nodes: Vec<T>,
    merge_fn: F,
}

impl<T, F> SegmentTree<T, F>
where 
    T: std::fmt::Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
{
    fn from_vec(arr: &[T], merge: F) -> Self {
        let size = arr.len();
        let mut buffer: Vec<T> = vec![T::default(); 2 * size];

        buffer[size..(2 * size)].clone_from_slice(arr);
        for idx in (1..size).rev() {
            buffer[idx] = merge(buffer[2 * idx], buffer[2 * idx + 1]);
        }

        Self {
            size,
            nodes: buffer,
            merge_fn: merge,
        }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> Result<Option<T>, SegmentTreeError> {
        if range.start >= self.size || range.end > self.size {
            return Err(SegmentTreeError::InvalidRange);
        }

        let mut left = range.start + self.size;
        let mut right = range.end + self.size;
        let mut result = None;

        while left < right {
            if left % 2 == 1 {
                result = Some(match result {
                    None => self.nodes[left],
                    Some(old) => (self.merge_fn)(old, self.nodes[left]),
                });
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                result = Some(match result {
                    None => self.nodes[right],
                    Some(old) => (self.merge_fn)(old, self.nodes[right]),
                });
            }
            left /= 2;
            right /= 2;
        }

        Ok(result)
    }

    pub fn update(&mut self, idx: usize, val: T) -> Result<(), SegmentTreeError> {
        if idx >= self.size {
            return Err(SegmentTreeError::IndexOutOfBounds);
        }

        let mut index = idx + self.size;
        if self.nodes[index] == val {
            return Ok(());
        }

        self.nodes[index] = val;
        while index > 1 {
            index /= 2;
            self.nodes[index] = (self.merge_fn)(self.nodes[2 * index], self.nodes[2 * index + 1]);
        }

        Ok(())
    }
}