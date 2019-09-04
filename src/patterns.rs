use crate::*;
use self::Kind::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Kind {
    Solid(Colour),
    Stripe(Colour, Colour),
    Gradient(Colour, Colour),
    Checkers(Colour, Colour),
    Ring(Colour, Colour),
}

fn alternate(t: i64, a: &Colour, b: &Colour) -> Colour {
    match t % 2 == 0 {
        true => *a,
        false => *b
    }
}

impl Kind {
    fn colour_at(&self, point: &Tuple) -> Colour {
        match self {
            Solid(colour) => *colour,
            Stripe(a, b) => {
                let t = point.x.floor() as i64;
                alternate(t, a, b)
            },
            Gradient(a, b) => {
                let distance = b - a;
                let fraction = point.x - point.x.floor();
                let df = distance * fraction;
                a + &df
            }
            Ring(a, b) => {
                let t = (point.x * point.x + point.z * point.z).sqrt().floor() as i64;
                alternate(t, a, b)
            }
            Checkers(a, b) => {
                let t = (point.x.floor() + point.y.floor() + point.z.floor()) as i64;
                alternate(t, a, b)
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Pattern {
    pub kind: Kind,
    pub transform: Matrix,
    inverse: Matrix,
}

impl Pattern {
    pub fn set_transform(self, transform: Matrix) -> Self {
        Self {
            transform,
            inverse: transform.inverse(),
            ..self
        }
    }

    pub fn colour_at(&self, point: &Tuple) -> Colour {
        self.kind.colour_at(point)
    }

    pub fn colour_at_object(&self, shape: &Shape, point: &Tuple) -> Colour {
        let object_point = shape.inverse * point;
        let pattern_point = self.inverse * object_point;
        self.kind.colour_at(&pattern_point)
    }
}

pub fn stripe_pattern(a: Colour, b: Colour) -> Pattern {
    Pattern {
        kind: Stripe(a, b),
        transform: identity(),
        inverse: identity().inverse(),
    }
}

pub fn gradient_pattern(a: Colour, b: Colour) -> Pattern {
    Pattern {
        kind: Gradient(a, b),
        transform: identity(),
        inverse: identity().inverse(),
    }
}

pub fn checkers_pattern(a: Colour, b: Colour) -> Pattern {
    Pattern {
        kind: Checkers(a, b),
        transform: identity(),
        inverse: identity().inverse(),
    }
}

pub fn ring_pattern(a: Colour, b: Colour) -> Pattern {
    Pattern {
        kind: Ring(a, b),
        transform: identity(),
        inverse: identity().inverse(),
    }
}
