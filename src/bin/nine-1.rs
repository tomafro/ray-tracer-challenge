extern crate tracer;
use tracer::*;

fn main() {
    let floor = plane()
        .set_transform(scaling(10, 0.01, 10))
        .set_material(material().set_colour(0.6, 0.6, 0.6).set_specular(0.0));

    let middle = sphere()
        .set_transform(translation(-0.5, 1, 0.5))
        .set_material(
            material()
                .set_colour(0.2, 0.2, 1)
                .set_diffuse(0.7)
                .set_specular(0.3)
                .set_pattern(
                    checkers_pattern(WHITE, BLACK).set_transform(scaling(0.05, 0.05, 0.05)),
                ),
        );

    let right = sphere()
        .set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5))
        .set_material(
            material()
                .set_colour(0.3, 1, 0.3)
                .set_diffuse(0.7)
                .set_specular(0.3)
                .set_pattern(
                    ring_pattern(colour(0.3, 1, 0.3), colour(1, 0.3, 0.3))
                        .set_transform(scaling(0.1, 0.1, 0.1) * rotation_x(1.0) * rotation_y(1.0)),
                ),
        );

    let left = sphere()
        .set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33))
        .set_material(
            material()
                .set_colour(1, 0.3, 0.3)
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let red_light = point_light(point(-10, 10, -10), colour(0.6, 0.4, 0.4));
    let green_light = point_light(point(10, 10, -10), colour(0.4, 0.6, 0.4));
    let blue_light = point_light(point(0, 10, -10), colour(0.4, 0.4, 0.6));

    let camera = camera(
        100,
        100,
        PI / 3.0,
        Some(view_transform(
            point(0, 1.5, -5),
            point(0, 1, 0),
            vector(0, 1, 0),
        )),
    );

    let world = World {
        lights: vec![red_light, green_light, blue_light],
        objects: vec![floor, middle, right, left],
    };

    camera.render(world).write_ppm_to(&mut std::io::stdout());
}
