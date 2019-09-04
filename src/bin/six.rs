extern crate tracer;

fn main() {
    // let pixels = 2000;

    // let ray_origin = point(0, 0, -5);
    // let wall_z = 10.0;
    // let wall_size = 7.0;

    // let pixel_size = wall_size / pixels as f32;
    // let half = wall_size / 2.0;

    // let mut canvas = ppm_canvas(pixels, pixels);

    // let s = sphere().set_material(
    //     material()
    //         .set_shininess(200)
    //         .set_ambient(0.03)
    //         .set_diffuse(0.95)
    //         .set_specular(0.4)
    //         .set_colour(1, 1, 0.2),
    // );

    // let light = point_light(point(-10, 10, -10), colour(1, 1, 1));

    // for y in 0..pixels {
    //     let world_y = half - pixel_size * y as f32;

    //     for x in 0..pixels {
    //         let world_x = -half + pixel_size * x as f32;

    //         let pos = point(world_x, world_y, wall_z);
    //         let ray = Ray {
    //             origin: ray_origin,
    //             direction: (pos - ray_origin).normalize(),
    //         };

    //         if let Some(intersections) = s.intersect(&ray) {
    //             let hit = &intersections[0];
    //             let point = ray.position(hit.t);
    //             let normal = s.normal_at(&point);
    //             let eye = -ray.direction;
    //             let color = lighting(&hit.object.material, &light, &point, &eye, &normal, false);
    //             canvas.set_pixel(x, y, color);
    //         }
    //     }
    // }

    // canvas.write_ppm_to(&mut std::io::stdout());
}
