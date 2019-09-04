extern crate tracer;
use tracer::*;

#[test]
fn the_default_material() {
    let m = material();
    assert_eq!(colour(1.0, 1.0, 1.0), m.colour);
    assert_eq!(0.1, m.ambient);
    assert_eq!(0.9, m.diffuse);
    assert_eq!(0.9, m.specular);
    assert_eq!(200, m.shininess);
}

#[test]
fn lighting_with_the_eye_between_the_light_and_the_surface() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, 0, -1);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 0, -10), colour(1, 1, 1));
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, false);
    assert_eq!(colour(1.9, 1.9, 1.9), result);
}

#[test]
fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, ROOT2 / 2.0, -ROOT2 / 2.0);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 0, -10), colour(1, 1, 1));
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, false);
    assert_eq!(colour(1.0, 1.0, 1.0), result);
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, 0, -1);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 10, -10), colour(1, 1, 1));
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, false);
    assert_eq!(colour(0.7364, 0.7364, 0.7364), result)
}

#[test]
fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, -ROOT2 / 2.0, -ROOT2 / 2.0);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 10, -10), colour(1, 1, 1));
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, false);
    assert_eq!(colour(1.6364, 1.6364, 1.6364), result)
}

#[test]
fn lighting_with_the_light_behind_the_surface() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, 0, -1);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 0, 10), colour(1, 1, 1));
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, false);
    assert_eq!(colour(0.1, 0.1, 0.1), result);
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let m = material();
    let position = point(0, 0, 0);
    let eyev = vector(0, 0, -1);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 0, -10), colour(1, 1, 1));
    let in_shadow = true;
    let result = lighting(&m, &sphere(), &light, &position, &eyev, &normalv, in_shadow);
    assert_eq!(colour(0.1, 0.1, 0.1), result);
}

#[test]
fn lighting_with_a_pattern_applied() {
    let m = material()
        .set_ambient(1.0)
        .set_diffuse(0.0)
        .set_specular(0.0)
        .set_pattern(stripe_pattern(WHITE, BLACK));

    let eyev = vector(0, 0, -1);
    let normalv = vector(0, 0, -1);
    let light = point_light(point(0, 0, -10), colour(1, 1, 1));
    let c1 = lighting(
        &m,
        &sphere(),
        &light,
        &point(0.9, 0, 0),
        &eyev,
        &normalv,
        false,
    );
    let c2 = lighting(
        &m,
        &sphere(),
        &light,
        &point(1.1, 0, 0),
        &eyev,
        &normalv,
        false,
    );
    assert_eq!(WHITE, c1);
    assert_eq!(BLACK, c2);
}
