mod buffer;
mod drain;
mod into_iter;
mod raw_val_iter;

use buffer::Buffer;
use core::marker::PhantomData;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use into_iter::IntoIter;

use self::drain::Drain;
use self::raw_val_iter::RawValIter;

pub struct Vec<T> {
    buf: Buffer<T>,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "not ready yet to handle ZST's");
        Self {
            buf: Buffer::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.should_grow() {
            self.buf.grow()
        }

        unsafe { ptr::write(self.ptr().add(self.len), item) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        self.check_out_of_bound(index);
        if self.should_grow() {
            self.buf.grow();
        }

        unsafe {
            let p = self.ptr();
            ptr::copy(p.add(index), p.add(index + 1), self.len - index);
            ptr::write(p.add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.check_out_of_bound(index);
        if self.should_grow() {
            self.buf.grow();
        }

        unsafe {
            self.len -= 1;
            let p = self.ptr();
            let result = ptr::read(p.add(index));
            ptr::copy(p.add(index + 1), p.add(index), self.len - index);
            result
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawValIter::new(&self);
            self.len = 0;

            Drain {
                iter,
                vec: PhantomData,
            }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn check_out_of_bound(&self, index: usize) {
        assert!(index <= self.len, "index out of bounds");
    }

    #[inline]
    fn should_grow(&self) -> bool {
        self.cap() >= self.len
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let _ = self.pop() {}
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);

            into_iter::IntoIter::new(buf, iter)
        }
    }
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}
