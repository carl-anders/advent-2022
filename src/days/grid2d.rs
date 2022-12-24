#![allow(dead_code)]
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::{One, WrappingSub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position2D<T> {
    pub x: T,
    pub y: T,
}
impl<T: Copy> Position2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub const fn xy(&self) -> (T, T) {
        (self.x, self.y)
    }
    pub const fn yx(&self) -> (T, T) {
        (self.y, self.x)
    }
}
impl<T: Copy + Add<Output = T> + Sub<Output = T>> Position2D<T> {
    pub fn add_x(&self, x: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }
    pub fn add_y(&self, y: T) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }
}
impl<T: Copy + Sub<Output = T>> Position2D<T> {
    pub fn sub_x(&self, x: T) -> Self {
        Self {
            x: self.x - x,
            y: self.y,
        }
    }
    pub fn sub_y(&self, y: T) -> Self {
        Self {
            x: self.x,
            y: self.y - y,
        }
    }
}
impl<T: Copy + WrappingSub> Position2D<T> {
    pub fn wrapping_sub_x(&self, x: &T) -> Self {
        Self {
            x: self.x.wrapping_sub(x),
            y: self.y,
        }
    }
    pub fn wrapping_sub_y(&self, y: &T) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_sub(y),
        }
    }
}

impl<T: Add<Output = T>> Add<Self> for Position2D<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: AddAssign> AddAssign for Position2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub<Self> for Position2D<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: SubAssign> SubAssign for Position2D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction4Way {
    Right,
    Down,
    Left,
    Up,
}
impl<T: Copy + One + Add<Output = T> + Sub<Output = T>> Add<Direction4Way> for Position2D<T> {
    type Output = Self;
    fn add(self, other: Direction4Way) -> Self {
        match other {
            Direction4Way::Right => self.add_x(T::one()),
            Direction4Way::Down => self.add_y(T::one()),
            Direction4Way::Left => self.sub_x(T::one()),
            Direction4Way::Up => self.sub_y(T::one()),
        }
    }
}
