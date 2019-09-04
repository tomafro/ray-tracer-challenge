use crate::*;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, time: Float) -> Tuple {
        self.origin + (self.direction * time)
    }

    pub fn transform(&self, matrix: Matrix) -> Ray {
        let origin = matrix * self.origin;
        let direction = matrix * self.direction;
        Ray { origin, direction }
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}
