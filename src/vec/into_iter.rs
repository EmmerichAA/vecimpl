use super::{buffer::Buffer, raw_val_iter::RawValIter};

pub struct IntoIter<T> {
    _buf: Buffer<T>,
    iter: RawValIter<T>,
}

impl<T> IntoIter<T> {
    pub fn new(buf: Buffer<T>, iter: RawValIter<T>) -> Self {
        Self { _buf: buf, iter }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}
