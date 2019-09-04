extern crate tracer;
use tracer::*;

fn main() {
    let pixels = 2000;

    let ray_origin = point(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = ppm_canvas(pixels, pixels);
    let red = colour(0.9, 0.1, 0.1);
    let s = sphere().set_transform(scaling(1.0, 0.5, 1.0));

    for y in 0..pixels {
        let world_y = half - pixel_size * y as f32;

        for x in 0..pixels {
            let world_x = -half + pixel_size * x as f32;

            let position = point(world_x, world_y, wall_z);
            let r = ray(ray_origin, (position - ray_origin).normalize());

            if let Some(_) = s.intersect(&r) {
                canvas.set_pixel(x, y, red);
            }
        }
    }

    canvas.write_ppm_to(&mut std::io::stdout());
}
