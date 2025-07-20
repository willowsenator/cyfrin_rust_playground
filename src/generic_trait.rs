pub trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

pub struct TupleIter<T> {
    pub tuple: (T, T, T),
    pub next: usize,
}

pub struct VecIter<T> {
    pub vec: Vec<T>,
    pub next: usize,
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        if self.next < 3 {
            let value = match self.next {
                0 => &self.tuple.0,
                1 => &self.tuple.1,
                2 => &self.tuple.2,
                _ => unreachable!(),
            };
            self.next += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {
        if self.next < self.vec.len() {
            let value = &self.vec[self.next];
            self.next += 1;
            Some(value)
        } else {
            None
        }
    }
}