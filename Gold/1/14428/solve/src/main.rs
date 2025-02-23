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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let _ = buf.trim().parse::<i64>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let arr = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

    let mut segment_tree = SegmentTree::from_vec(&arr, std::cmp::min);

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();

    let mut index = Vec::new();

    for i in 0..arr.len() {
        index.push((i, arr[i]));
    }

    use std::fmt::Write;
    let mut stdout = String::new();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let query = buf.split_ascii_whitespace().flat_map(|x|x.parse::<i64>()).collect::<Vec<i64>>();

        match query[0] {
            1 => {
                index[query[1] as usize - 1] = (index[query[1] as usize - 1].0, query[2]);
                segment_tree.update(query[1] as usize - 1, query[2]).unwrap();
            },
            2 => {
                let t = segment_tree.query(query[1] as usize - 1..query[2] as usize).unwrap().unwrap();
                writeln!(stdout, "{}", index[query[1] as usize - 1..query[2] as usize].iter().find(|&&x|x.1 == t).unwrap().0 + 1).unwrap();
            },
            _ => {},
        }
    }

    print!("{stdout}");
}
