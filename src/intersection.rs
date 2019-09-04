use crate::*;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: Float,
    pub object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn prepare(&self, ray: &Ray) -> PreparedIntersection {
        let mut point = ray.position(self.t);
        let mut normalv = self.object.normal_at(&point);
        point = point + normalv * 0.0001;
        let eyev = -ray.direction;
        let inside = normalv.dot(eyev) < 0.0;

        if inside {
            normalv = -normalv;
        }

        PreparedIntersection {
            hit: &self,
            point,
            normalv,
            eyev,
            inside,
        }
    }
}

#[derive(Debug)]
pub struct PreparedIntersection<'a> {
    pub hit: &'a Intersection<'a>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> PreparedIntersection<'a> {
    pub fn shade(&self, world: &World) -> Colour {
        world
            .lights
            .iter()
            .fold(BLACK, |colour, light| {
                colour
                    + lighting(
                        &self.hit.object.material,
                        &self.hit.object,
                        light,
                        &self.point,
                        &self.eyev,
                        &self.normalv,
                        world.is_shadowed(&self.point, light),
                    )
            })
    }
}

pub trait FindHit {
    fn hit(&mut self) -> Option<&Intersection>;
}

impl<'a> FindHit for Vec<Intersection<'a>> {
    fn hit(&mut self) -> Option<&Intersection> {
        self.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self.iter().find(|i| i.t >= 0.0)
    }
}

pub fn intersection<'a>(t: Float, object: &'a Shape) -> Intersection<'a> {
    Intersection { t, object }
}
