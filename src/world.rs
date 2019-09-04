use crate::*;

pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn default() -> World {
        let s1 = sphere().set_material(
            material()
                .set_colour(0.8, 1.0, 0.6)
                .set_diffuse(0.7)
                .set_specular(0.2),
        );

        let s2 = sphere().set_transform(scaling(0.5, 0.5, 0.5));

        World {
            lights: vec![point_light(point(-10, 10, -10), colour(1, 1, 1))],
            objects: vec![s1, s2],
        }
    }

    pub fn light(&self) -> &PointLight {
        &self.lights[0]
    }

    pub fn contains(&self, object: &Shape) -> bool {
        self.objects.contains(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in &self.objects {
            object.append_intersections(ray, &mut result)
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        result
    }

    pub fn colour_at(&self, ray: &Ray) -> Colour {
        let mut intersections = self.intersect(ray);

        match intersections.hit() {
            Some(hit) => hit.prepare(ray).shade(self),
            None => BLACK,
        }
    }

    pub fn is_shadowed(&self, point: &Tuple, light: &PointLight) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = ray(*point, direction);
        let mut intersections = self.intersect(&r);
        let h = intersections.hit();

        match h {
            Some(h) => h.t < distance,
            None => false,
        }
    }
}

pub fn world() -> World {
    World {
        lights: vec![],
        objects: vec![],
    }
}
