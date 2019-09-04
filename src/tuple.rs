use crate::float_eq;
use crate::Float;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, other: Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }

    pub fn reflect(&self, other: Self) -> Self {
        *self - other * 2.0 * self.dot(other)
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        float_eq(self.x, other.x)
            && float_eq(self.y, other.y)
            && float_eq(self.z, other.z)
            && float_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a> Sub<Tuple> for &'a Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<Float> for Tuple {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<Float> for Tuple {
    type Output = Self;

    fn div(self, rhs: Float) -> Self {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

pub fn tuple<A: Into<Float>, B: Into<Float>, C: Into<Float>>(x: A, y: B, z: C, w: Float) -> Tuple {
    Tuple {
        x: x.into(),
        y: y.into(),
        z: z.into(),
        w,
    }
}

pub fn point<A: Into<Float>, B: Into<Float>, C: Into<Float>>(x: A, y: B, z: C) -> Tuple {
    tuple(x, y, z, 1.0)
}

pub fn vector<A: Into<Float>, B: Into<Float>, C: Into<Float>>(x: A, y: B, z: C) -> Tuple {
    tuple(x, y, z, 0.0)
}
