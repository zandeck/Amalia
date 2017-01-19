use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

//Definition of operators on vectors
impl<T:Add<Output = T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T:Sub<Output = T>> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Vector3<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector3<T> {

    pub fn cross(self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: (self.x * rhs.y) - (self.y * rhs.x),
            y: (self.y * rhs.z) - (self.z * rhs.y),
            z: (self.z * rhs.x) - (self.x * rhs.z)
        }
    }
}
