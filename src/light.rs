use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Colour,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Colour) -> Self {
        Self {
            intensity,
            position,
        }
    }
}

pub fn point_light(position: Tuple, intensity: Colour) -> PointLight {
    PointLight::new(position, intensity)
}

pub fn lighting(
    material: &Material,
    shape: &Shape,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Colour {
    let colour = match &material.pattern {
        Some(pattern) => pattern.colour_at_object(shape, point),
        None => material.colour,
    };

    let effective_color = colour * light.intensity;
    let lightv = (light.position - *point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(*normalv);
    let mut diffuse = BLACK;
    let mut specular = BLACK;

    if light_dot_normal >= 0.0 && !in_shadow {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = -lightv.reflect(*normalv);
        let reflect_dot_eye = reflectv.dot(*eyev).powf(material.shininess as Float);
        if reflect_dot_eye <= 0.0 {
            specular = BLACK;
        } else {
            specular = light.intensity * material.specular * reflect_dot_eye;
        }
    }

    ambient + diffuse + specular
}
