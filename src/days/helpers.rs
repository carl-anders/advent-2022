#![allow(dead_code)]

use std::ops::{Range, RangeBounds};
pub trait BorrowTwo<T> {
    fn borrow_two(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwo<T> for [T] {
    fn borrow_two(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
        assert!(a < self.len() && b < self.len());
        assert!(a != b);
        if a < b {
            if let [first, .., second] = &mut self[a..=b] {
                return (first, second);
            }
        } else if let [second, .., first] = &mut self[b..=a] {
            return (first, second);
        }
        panic!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LongBitArr<const SIZE: usize> {
    data: [u64; SIZE],
}
impl<const SIZE: usize> LongBitArr<SIZE> {
    pub const fn new() -> Self {
        Self { data: [0; SIZE] }
    }
    pub fn get(&self, index: usize) -> bool {
        assert!(index < SIZE * 64);
        unsafe { (self.data.get_unchecked(index / 64) >> (index % 64)) & 1 != 0 }
    }
    pub fn set(&mut self, index: usize) {
        assert!(index < SIZE * 64);
        unsafe {
            *self.data.get_unchecked_mut(index / 64) |= 1 << (index % 64);
        }
    }
    pub fn clear(&mut self, index: usize) {
        assert!(index < SIZE * 64);
        unsafe {
            *self.data.get_unchecked_mut(index / 64) &= !(1 << (index % 64));
        }
    }
    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|d| d.count_ones() as usize).sum()
    }
}

pub struct UsizeIter {
    inner: usize
}

impl UsizeIter {
    pub const fn new(value: usize) -> Self {
        Self { inner: value }
    }
}

impl Iterator for UsizeIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inner == 0 {
            None
        } else {
            let next = self.inner.trailing_zeros();
            self.inner ^= 1 << next;
            Some(next as usize)
        }
    }
}

pub trait BitIter where Self: Sized + PartialEq {
    fn bit_next(&mut self) -> Option<Self>;
}

impl BitIter for usize {
    fn bit_next(&mut self) -> Option<Self> {
        if *self == 0 {
            None
        } else {
            let next = self.trailing_zeros();
            *self ^= 1 << next;
            Some(next as Self)
        }
    }
}

pub trait BitArr {
    fn get(&self, index: Self) -> bool;
    fn set(&mut self, index: Self);
    fn clear(&mut self, index: Self);
}

impl BitArr for usize {
    fn get(&self, index: Self) -> bool {
        (self >> index) & 1 != 0
    }
    fn set(&mut self, index: Self) {
        *self |= 1 << index;
    }
    fn clear(&mut self, index: Self) {
        *self &= !(1 << index);
    }
}

pub trait RangeIntersect<T: Ord, U: RangeBounds<T>>: RangeBounds<T> {
    fn intersect(&self, other: &U) -> Option<U>;
}
impl<T: Ord + Copy> RangeIntersect<T, Self> for Range<T> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(self.start.max(other.start)..self.end.min(other.end))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MergedRange<T> {
    ranges: Vec<Range<T>>,
}
impl<T: Ord + Copy> MergedRange<T> {
    pub const fn new() -> Self {
        Self { ranges: vec![] }
    }
    pub fn add(&mut self, other: Range<T>) {
        self.ranges.push(other);
        if self.ranges.len() > 1 {
            self.simplify();
        }
    }
    fn simplify(&mut self) {
        self.ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
        'outer: loop {
            for index in 0..self.ranges.len()-1 {
                let (a, b) = self.ranges.borrow_two(index, index + 1);
                if a.end >= b.start && a.start <= b.end {
                    *a = a.start.min(b.start)..a.end.max(b.end);
                    self.ranges.remove(index + 1);
                    continue 'outer;
                }
            }
            break;
        }
    }
    /* pub fn add(&mut self, other: Range<T>) {
        self.ranges.push(other);
        if self.ranges.len() > 1 {
            self.simplify();
        }
    }
    pub fn simplify(&mut self) {
        loop {
            let mut to_delete = None;
            for index in (0..self.ranges.len()).combinations(2) {
                let (a, b) = self.ranges.borrow_two(index[0], index[1]);
                //println!("index: {:?}, a: {:?}, b: {:?}", index, a, b);
                if !(a.end < b.start || b.end < a.start) || a.end == b.start || b.end == a.start {
                    //println!("Overlapping!");
                    *a = a.start.min(b.start)..a.end.max(b.end);
                    to_delete = Some(index[1]);
                    break;
                }
            }
            if let Some(index) = to_delete {
                self.ranges.remove(index);
            } else {
                break;
            }
        }
    } */
    pub fn ranges(&self) -> Vec<Range<T>> {
        self.ranges.clone()
    }
}
impl<T: Ord + Copy> FromIterator<Range<T>> for MergedRange<T> {
    fn from_iter<Q>(iter: Q) -> Self
    where
        Q: IntoIterator<Item = Range<T>>,
    {
        let mut new = Self::new();
        for range in iter {
            new.add(range);
        }
        new
    }
}
