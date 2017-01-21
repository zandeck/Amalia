use maths::vector3::Vector3;
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Quaternion<T> {
    pub scal: T,
    pub vec: Vector3<T>
}

//Definition of operators on quaternions
impl<T: Add<Output = T>> Add for Quaternion<T>{
    type Output = Quaternion<T>;

    fn add(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            scal: self.scal + rhs.scal,
            vec: self.vec + rhs.vec
        }
    }
}

impl<T: Sub<Output = T>> Sub for Quaternion<T>{
    type Output = Quaternion<T>;

    fn sub(self, rhs: Quaternion<T>) -> Quaternion<T> {
        Quaternion {
            scal: self.scal - rhs.scal,
            vec: self.vec - rhs.vec
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy> Mul for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: Quaternion<T>) -> Quaternion<T> {
        let (a1, v1) = (self.scal, self.vec);
        let (a2, v2) = (rhs.scal, rhs.vec);
        Quaternion {
            scal: a1 * a2 - v1 * v2,
            vec: v2 * a1 + v1 * a2 + v1.cross(v2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Quaternion;
    use super::super::vector3::Vector3;

    #[test]
    fn add_quaterions() {

        let q1 = Quaternion {
            scal: 1,
            vec: Vector3 {
                x: 1,
                y: 1,
                z: 1
            }
        };

        let q2 = Quaternion {
            scal: 1,
            vec: Vector3 {
                x: 1,
                y: 1,
                z: 1
            }
        };

        let q3 = Quaternion {
            scal: 0,
            vec: Vector3 {
                x: 0,
                y: 0,
                z: 0
            }
        };
        assert_eq!(q1-q2, q3);
    }
}
