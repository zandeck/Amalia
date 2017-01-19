use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

//Definition of operators on vectors
impl<T:Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T:Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vector2<T> {
    type Output = T;

    fn mul(self, rhs: Vector2<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
