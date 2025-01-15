use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Sub<Output = T> + Copy> Point<T> {
    pub fn distance(&self, other: &Point<T>) -> usize
    where
        T: Into<i64>,
    {
        ((self.x - other.x).into().abs() + (self.y - other.y).into().abs())
            .try_into()
            .unwrap()
    }
}

impl<T: Add<Output = T> + Copy> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) -> () {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) -> () {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
            other => other,
        }
    }
}

impl<T: Ord> Ord for Point<T> {
    fn cmp(&self, other: &Point<T>) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            other => other,
        }
    }
}

impl<T: PartialOrd + Ord + Copy> Point<T> {
    pub fn max(self, other: Point<T>) -> Point<T> {
        if self > other {
            self
        } else {
            other
        }
    }

    pub fn min(self, other: Point<T>) -> Point<T> {
        if self < other {
            self
        } else {
            other
        }
    }

    pub fn clamp(self, min: Point<T>, max: Point<T>) -> Point<T> {
        Point::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Direction;

impl Direction {
    pub const UP_LEFT: Point<i32> = Point { y: -1, x: -1 };
    pub const UP: Point<i32> = Point { y: -1, x: 0 };
    pub const UP_RIGHT: Point<i32> = Point { y: -1, x: 1 };
    pub const RIGHT: Point<i32> = Point { y: 0, x: 1 };
    pub const DOWN_RIGHT: Point<i32> = Point { y: 1, x: 1 };
    pub const DOWN: Point<i32> = Point { y: 1, x: 0 };
    pub const DOWN_LEFT: Point<i32> = Point { y: 1, x: -1 };
    pub const LEFT: Point<i32> = Point { y: 0, x: -1 };
    pub const NONE: Point<i32> = Point { y: 0, x: 0 };

    pub const ALL: [Point<i32>; 8] = [
        self::Direction::UP_LEFT,
        self::Direction::UP,
        self::Direction::UP_RIGHT,
        self::Direction::RIGHT,
        self::Direction::DOWN_RIGHT,
        self::Direction::DOWN,
        self::Direction::DOWN_LEFT,
        self::Direction::LEFT,
    ];

    pub const CARDINAL: [Point<i32>; 4] = [
        self::Direction::UP,
        self::Direction::RIGHT,
        self::Direction::DOWN,
        self::Direction::LEFT,
    ];
}
