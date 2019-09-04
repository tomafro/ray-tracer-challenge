#![feature(test)]
extern crate rayon;

mod camera;
mod canvas;
mod colour;
mod intersection;
mod light;
mod material;
mod matrix;
mod patterns;
pub mod ppm;
mod ray;
mod shapes;
mod transformations;
mod tuple;
mod world;

pub use crate::camera::*;
pub use crate::canvas::{canvas, Canvas};
pub use crate::colour::{colour, Colour, BLACK, WHITE};
pub use crate::intersection::*;
pub use crate::light::{lighting, point_light, PointLight};
pub use crate::material::{material, Material};
pub use crate::matrix::*;
pub use crate::patterns::*;
pub use crate::ppm::{ppm_canvas, ToPpm};
pub use crate::ray::*;
pub use crate::shapes::*;
pub use crate::transformations::*;
pub use crate::tuple::{point, tuple, vector, Tuple};
pub use crate::world::{world, World};

pub use std::f64::consts::PI;
pub type Float = f64;

pub const ROOT2: Float = 1.4142135623730950488;
const EPSILON: Float = 0.001;

pub fn float_eq(a: Float, b: Float) -> bool {
    (a - b).abs() < EPSILON
}
