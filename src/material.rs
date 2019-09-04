use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub colour: Colour,
    pub pattern: Option<Pattern>,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: u32,
}

impl Material {
    pub fn default() -> Self {
        Self {
            colour: colour(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200,
            pattern: None,
        }
    }

    pub fn set_colour<A: Into<Float>, B: Into<Float>, C: Into<Float>>(
        self,
        red: A,
        green: B,
        blue: C,
    ) -> Material {
        Material {
            colour: colour(red, green, blue),
            ..self
        }
    }

    pub fn set_ambient(self, ambient: Float) -> Material {
        Material { ambient, ..self }
    }

    pub fn set_diffuse(self, diffuse: Float) -> Material {
        Material { diffuse, ..self }
    }

    pub fn set_specular(self, specular: Float) -> Material {
        Material { specular, ..self }
    }

    pub fn set_shininess(self, shininess: u32) -> Material {
        Material { shininess, ..self }
    }

    pub fn set_pattern(self, pattern: Pattern) -> Material {
        Material {
            pattern: Some(pattern),
            ..self
        }
    }
}

pub fn material() -> Material {
    Material::default()
}
