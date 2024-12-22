use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Copy> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) -> () {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
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
}
