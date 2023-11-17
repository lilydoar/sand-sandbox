use bit_vec::BitVec;

pub struct BitArray2D {
    data: BitVec,
    width: usize,
}

impl BitArray2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: BitVec::from_elem(width * height, false),
            width,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        let index = self.width * y + x;
        self.data.get(index)
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = self.width * y + x;
        self.data.set(index, value);
    }

    pub fn iter_true(&self) -> TrueValuesIterator {
        TrueValuesIterator {
            bit_array: self,
            inner_iter: self.data.iter().enumerate(),
        }
    }
}

pub struct TrueValuesIterator<'a> {
    bit_array: &'a BitArray2D,
    inner_iter: std::iter::Enumerate<bit_vec::Iter<'a>>,
}

impl<'a> Iterator for TrueValuesIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, value)) = self.inner_iter.next() {
            if value {
                let x = index % self.bit_array.width;
                let y = index / self.bit_array.width;
                return Some((x, y));
            }
        }
        None
    }
}
