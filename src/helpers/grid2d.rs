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

impl<T: Copy + PartialOrd + Sub<Output = T> + Add<Output = T>> Position2D<T> {
    pub fn manhattan(&self, other: &Self) -> T {
        (if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }) + (if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        })
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
pub enum Direction8Way {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}
impl Direction8Way {
    pub const UP: Self = Self::N;
    pub const RIGHT: Self = Self::E;
    pub const DOWN: Self = Self::S;
    pub const LEFT: Self = Self::W;
    pub const EVERY: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];
}
impl<T: Copy + One + Add<Output = T> + Sub<Output = T>> Add<Direction8Way> for Position2D<T> {
    type Output = Self;
    fn add(self, other: Direction8Way) -> Self {
        match other {
            Direction8Way::N => self.sub_y(T::one()),
            Direction8Way::NE => self.sub_y(T::one()).add_x(T::one()),
            Direction8Way::E => self.add_x(T::one()),
            Direction8Way::SE => self.add_y(T::one()).add_x(T::one()),
            Direction8Way::S => self.add_y(T::one()),
            Direction8Way::SW => self.add_y(T::one()).sub_x(T::one()),
            Direction8Way::W => self.sub_x(T::one()),
            Direction8Way::NW => self.sub_y(T::one()).sub_x(T::one()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction4Way {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Direction4Way {
    pub const fn turn_right(self, times: usize) -> Self {
        match times % 4 {
            1 => match self {
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
                Self::Up => Self::Right,
            },
            2 => match self {
                Self::Right => Self::Left,
                Self::Down => Self::Up,
                Self::Left => Self::Right,
                Self::Up => Self::Down,
            },
            3 => match self {
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
                Self::Up => Self::Left,
            },
            _ => self,
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Turn {
    Right,
    Left,
}
impl Add<Turn> for Direction4Way {
    type Output = Self;
    fn add(self, other: Turn) -> Self {
        self.turn_right(if other == Turn::Right { 1 } else { 3 })
    }
}
