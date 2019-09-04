extern crate tracer;
use tracer::*;

#[test]
fn creating_and_querying_a_ray() {
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);
    let r = ray(origin, direction);
    assert_eq!(origin, r.origin);
    assert_eq!(direction, r.direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let r = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    assert_eq!(point(2.0, 3.0, 4.0), r.position(0.0));
    assert_eq!(point(3.0, 3.0, 4.0), r.position(1.0));
    assert_eq!(point(1.0, 3.0, 4.0), r.position(-1.0));
    assert_eq!(point(4.5, 3.0, 4.0), r.position(2.5));
}
