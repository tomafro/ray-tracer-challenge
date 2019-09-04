extern crate tracer;
use tracer::*;

#[test]
fn the_normal_of_a_plane_is_constant_everywhere() {
    let p = plane();
    assert_eq!(vector(0, 1, 0), p.local_normal_at(&point(0, 0, 0)));
    assert_eq!(vector(0, 1, 0), p.local_normal_at(&point(10, 0, -10)));
    assert_eq!(vector(0, 1, 0), p.local_normal_at(&point(-5, 0, 150)));
}

#[test]
fn intersect_with_a_ray_parallel_to_the_plane() {
    let p = plane();
    let r = ray(point(0, 10, 0), vector(0, 0, 1));
    let xs = p.local_intersect(&r);
    assert!(xs.is_none());
}

#[test]
fn intersect_with_a_coplanar_ray() {
    let p = plane();
    let r = ray(point(0, 0, 0), vector(0, 0, 1));
    let xs = p.local_intersect(&r);
    assert!(xs.is_none());
}

#[test]
fn a_ray_intersecting_a_plane_from_above() {
    let p = plane();
    let r = ray(point(0, 1, 0), vector(0, -1, 0));
    let xs = p.local_intersect(&r).unwrap();
    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].t);
    assert_eq!(&p, xs[0].object);
}

#[test]
fn a_ray_intersecting_a_plane_from_below() {
    let p = plane();
    let r = ray(point(0, -1, 0), vector(0, 1, 0));
    let xs = p.local_intersect(&r).unwrap();
    assert_eq!(1, xs.len());
    assert_eq!(1.0, xs[0].t);
    assert_eq!(&p, xs[0].object);
}
