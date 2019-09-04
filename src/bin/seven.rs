extern crate tracer;
use tracer::*;

fn main() {
    let floor = sphere()
        .set_transform(scaling(10, 0.01, 10))
        .set_material(material().set_colour(1, 0.2, 0.2).set_specular(0.0));

    let left_wall = sphere()
        .set_transform(
            translation(0, 0, 5)
                * rotation_y(-PI / 4.0)
                * rotation_x(PI / 2.0)
                * scaling(10, 0.01, 10),
        )
        .set_material(material().set_colour(1, 0.2, 0.2).set_specular(0.0));

    let right_wall = sphere()
        .set_transform(
            translation(0, 0, 5)
                * rotation_y(PI / 4.0)
                * rotation_x(PI / 2.0)
                * scaling(10, 0.01, 10),
        )
        .set_material(material().set_colour(1, 0.2, 0.2).set_specular(0.0));

    let middle = sphere()
        .set_transform(translation(-0.5, 1, 0.5))
        .set_material(
            material()
                .set_colour(1, 0.2, 0.2)
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let right = sphere()
        .set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5))
        .set_material(
            material()
                .set_colour(1, 0.3, 0.3)
                .set_diffuse(0.7)
                .set_specular(0.3),
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
        2000,
        1000,
        PI / 3.0,
        Some(view_transform(
            point(0, 1.5, -5),
            point(0, 1, 0),
            vector(0, 1, 0),
        )),
    );

    let world = World {
        lights: vec![red_light, green_light, blue_light],
        objects: vec![floor, left_wall, right_wall, middle, right, left],
    };

    camera.render(world).write_ppm_to(&mut std::io::stdout());
}
