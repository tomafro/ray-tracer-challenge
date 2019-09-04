extern crate tracer;
use tracer::*;

fn main() {
    let red = colour(0.9, 0.1, 0.1);
    let mut canvas = ppm_canvas(200, 200);
    let h1 = point(0.0, 90.0, 0.0);
    let h2 = point(0.0, 89.0, 0.0);
    let h3 = point(0.0, 88.0, 0.0);
    let h4 = point(0.0, 87.0, 0.0);
    let h5 = point(0.0, 86.0, 0.0);
    let h6 = point(0.0, 85.0, 0.0);
    let h7 = point(0.0, 88.0, 0.0);

    let center = translation(100.0, 100.0, 0.0);

    for m in 0..60 {
        let rotation = rotation_z(((2.0 * PI) / 60.0) * m as Float);
        let h1t = center * rotation * h1;

        canvas.set_pixel(h1t.x as usize, h1t.y as usize, red);
    }

    for h in 0..12 {
        let rotation = rotation_z(((2.0 * PI) / 12.0) * h as Float);
        let h1t = center * rotation * h1;
        let h2t = center * rotation * h2;
        let h3t = center * rotation * h3;
        let h4t = center * rotation * h4;
        let h5t = center * rotation * h5;
        let h6t = center * rotation * h6;
        let h7t = center * rotation * h7;

        canvas.set_pixel(h1t.x as usize, h1t.y as usize, red);
        canvas.set_pixel(h2t.x as usize, h2t.y as usize, red);
        canvas.set_pixel(h3t.x as usize, h3t.y as usize, red);
        canvas.set_pixel(h4t.x as usize, h4t.y as usize, red);
        canvas.set_pixel(h5t.x as usize, h5t.y as usize, red);
        canvas.set_pixel(h6t.x as usize, h6t.y as usize, red);
        canvas.set_pixel(h7t.x as usize, h7t.y as usize, red);
    }

    println!("{}", canvas.to_ppm());
}
