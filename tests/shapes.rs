extern crate tracer;
use tracer::*;

fn test_shape() -> Shape {
    sphere()
}

#[test]
fn the_default_transformation() {
    let s = test_shape();
    assert_eq!(identity(), s.transform);
}

#[test]
fn the_default_material() {
    let s = test_shape();
    let m = s.material();
    assert_eq!(&material(), m);
}

// #[test]
// fn test_intersecting_a_scaled_shape_with_a_ray() {
//     let r = ray(point(0, 0, -5), vector(0, 0, 1));
//     let s = test_shape().set_transform(scaling(2, 2, 2));
//     let xs = s.intersect(&r);

//     assert_eq!(point(0, 0, -2.5), s.saved_ray.origin);
//     assert_eq!(vector(0, 0, 0.5), s.saved_ray.direction);
// }

// #[test]
// fn test_intersecting_a_translated_shape_with_a_ray() {
//     let r = ray(point(0, 0, -5), vector(0, 0, 1));
//     let s = test_shape().set_transform(translation(5, 0, 0));
//     let xs = s.intersect(&r);

//     assert_eq!(point(-5, 0, -5), s.saved_ray.origin);
//     assert_eq!(vector(0, 0, 1), s.saved_ray.direction);
// }
