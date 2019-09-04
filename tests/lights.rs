extern crate tracer;
use tracer::*;

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let intensity = colour(1, 1, 1);
    let position = point(0, 0, 0);
    let light = point_light(position, intensity);
    assert_eq!(position, light.position);
    assert_eq!(intensity, light.intensity);
}
