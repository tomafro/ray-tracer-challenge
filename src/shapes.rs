use crate::*;

#[derive(PartialEq, Debug, Clone)]
enum Kind {
    Sphere,
    Plane,
}

impl Kind {
    fn normal_at(&self, local_point: &Tuple) -> Tuple {
        match self {
            Sphere => local_point - point(0, 0, 0),
            Plane => vector(0, 1, 0),
        }
    }

    fn intersections(&self, ray: &Ray) -> Vec<Float> {
        match self {
            Sphere => self.sphere_intersections(ray),
            Plane => self.plane_intersections(ray)
        }
    }

    fn sphere_intersections(&self, ray: &Ray) -> Vec<Float> {
        let Ray { origin, direction } = ray;
        let sphere_to_ray = origin - point(0.0, 0.0, 0.0);
        let a = direction.dot(direction.clone());
        let b = 2.0 * direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let t1 = (-b - (discriminant_sqrt)) / (2.0 * a);
            let t2 = (-b + (discriminant_sqrt)) / (2.0 * a);

            vec![t1, t2]
        }
        else {
            vec![]
        }
    }

    fn plane_intersections(&self, ray: &Ray) -> Vec<Float> {
        match ray.direction.y.abs() > 0.0001 {
            true => vec![-ray.origin.y / ray.direction.y],
            false => vec![]
        }
    }
}

use self::Kind::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Shape {
    pub material: Material,
    pub transform: Matrix,
    pub inverse: Matrix,
    kind: Kind,
}

impl Shape {
    fn new(kind: Kind) -> Shape {
        Self {
            material: Material::default(),
            transform: identity(),
            inverse: identity().inverse(),
            kind: kind,
        }
    }

    pub fn sphere() -> Shape {
        Shape::new(Sphere)
    }

    pub fn plane() -> Shape {
        Shape::new(Plane)
    }

    pub fn set_material(self, material: Material) -> Self {
        Self { material, ..self }
    }

    pub fn set_transform(self, transform: Matrix) -> Self {
        Self {
            transform,
            inverse: transform.inverse(),
            ..self
        }
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn append_intersections<'a>(
        &'a self,
        ray: &Ray,
        intersections: &mut Vec<Intersection<'a>>,
    ) {
        let local_ray = ray.transform(self.inverse);
        self.append_local_intersections(&local_ray, intersections);
    }

    pub fn append_local_intersections<'a>(
        &'a self,
        local_ray: &Ray,
        intersections: &mut Vec<Intersection<'a>>,
    ) {
        for t in self.kind.intersections(local_ray) {
            intersections.push(Intersection {
                t,
                object: self,
            });
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut result = vec![];

        self.append_intersections(ray, &mut result);

        match result.is_empty() {
            true => None,
            false => Some(result),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let mut result = vec![];

        self.append_local_intersections(ray, &mut result);

        match result.is_empty() {
            true => None,
            false => Some(result),
        }
    }

    pub fn normal_at(&self, p: &Tuple) -> Tuple {
        let local_point = self.inverse * p;
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = self.inverse.transpose() * local_normal;
        world_normal.w = 0.0;
        return world_normal.normalize();
    }

    pub fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        self.kind.normal_at(local_point)
    }
}

pub fn sphere() -> Shape {
    Shape::sphere()
}

pub fn plane() -> Shape {
    Shape::plane()
}
