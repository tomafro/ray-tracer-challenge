use crate::float_eq;
use crate::Float;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Colour(Float, Float, Float);

trait ToRGB {
    fn to_rgb(&self) -> u8;
}

impl ToRGB for Float {
    fn to_rgb(&self) -> u8 {
        (self * 255.0).min(255.0).max(0.0) as u8
    }
}

impl Colour {
    pub fn red(&self) -> Float {
        self.0
    }

    pub fn green(&self) -> Float {
        self.1
    }

    pub fn blue(&self) -> Float {
        self.2
    }

    pub fn to_u8_rgb(&self) -> (u8, u8, u8) {
        (self.0.to_rgb(), self.1.to_rgb(), self.2.to_rgb())
    }
}

impl PartialEq<Colour> for Colour {
    fn eq(&self, other: &Colour) -> bool {
        float_eq(self.0, other.0) && float_eq(self.1, other.1) && float_eq(self.2, other.2)
    }
}

impl Hash for Colour {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
        self.1.to_bits().hash(state);
        self.2.to_bits().hash(state);
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Colour) -> Colour {
        Colour(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<&Colour> for Colour {
    type Output = Colour;

    fn add(self, other: &Colour) -> Colour {
        Colour(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<&Colour> for &Colour {
    type Output = Colour;

    fn add(self, other: &Colour) -> Colour {
        Colour(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, other: Colour) -> Colour {
        Colour(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<&Colour> for Colour {
    type Output = Colour;

    fn sub(self, other: &Colour) -> Colour {
        Colour(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<&Colour> for &Colour {
    type Output = Colour;

    fn sub(self, other: &Colour) -> Colour {
        Colour(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Float> for Colour {
    type Output = Self;

    fn mul(self, factor: Float) -> Colour {
        Colour(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}

impl Mul<Colour> for Colour {
    type Output = Self;

    fn mul(self, other: Colour) -> Colour {
        Colour(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

pub fn colour<A: Into<Float>, B: Into<Float>, C: Into<Float>>(red: A, green: B, blue: C) -> Colour {
    Colour(red.into(), green.into(), blue.into())
}

pub static BLACK: Colour = Colour(0.0, 0.0, 0.0);
pub static WHITE: Colour = Colour(1.0, 1.0, 1.0);
