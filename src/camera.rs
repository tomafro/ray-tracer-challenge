use crate::*;
use rayon::prelude::*;

struct Coordinates {
    width: usize,
    height: usize,
    current: usize,
}

impl Iterator for Coordinates {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        let x = self.current % self.width;
        let y = self.current / self.width;

        self.current += 1;

        match y < self.height {
            true => Some((x, y)),
            false => None,
        }
    }
}

fn coordinates(width: usize, height: usize) -> Coordinates {
    Coordinates {
        width,
        height,
        current: 0,
    }
}

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub pixel_size: Float,
    pub field_of_view: Float,
    half_height: Float,
    half_width: Float,
    pub transform: Matrix,
    inverse: Matrix,
}

impl Camera {
    pub fn ray_for(&self, x: usize, y: usize) -> Ray {
        self.ray_at(x as Float + 0.5, y as Float + 0.5)
    }

    pub fn ray_at(&self, x: Float, y: Float) -> Ray {
        let xoffset = x * self.pixel_size;
        let yoffset = y * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.inverse * point(world_x, world_y, -1.0);
        let origin = self.inverse * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        ray(origin, direction)
    }

    pub fn render(&self, world: World) -> crate::ppm::Canvas {
        let mut canvas = ppm_canvas(self.hsize, self.vsize);
        let coords: Vec<(usize, usize)> = coordinates(self.hsize, self.vsize).collect();

        let colours: Vec<(usize, usize, Colour)> = coords
            .par_iter()
            .map(|(x, y)| {
                let ray = self.ray_at(*x as f64, *y as f64);
                let colour = world.colour_at(&ray);
                (*x, *y, colour)
            })
            .collect();

        for (x, y, colour) in colours {
            canvas.set_pixel(x, y, colour);
        }
        canvas
    }
}

pub fn camera(
    hsize: usize,
    vsize: usize,
    field_of_view: Float,
    transform: Option<Matrix>,
) -> Camera {
    let transform = match transform {
        None => identity(),
        Some(t) => t,
    };

    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as Float / vsize as Float;
    let mut half_width = half_view * aspect;
    let mut half_height = half_view;

    if aspect >= 1.0 {
        half_width = half_view;
        half_height = half_view / aspect;
    }
    let pixel_size = (half_width * 2.0) / hsize as Float;

    Camera {
        hsize,
        vsize,
        pixel_size,
        field_of_view,
        transform,
        half_height,
        half_width,
        inverse: transform.inverse(),
    }
}
