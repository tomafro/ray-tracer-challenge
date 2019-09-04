extern crate tracer;
use tracer::*;

const ROOT3: Float = 1.7320508075;

#[test]
fn a_ray_intersects_a_sphere_at_two_points() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere();
    let xs = s.intersect(&r).unwrap();
    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(6.0, xs[1].t);
}

#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
    let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere();
    let xs = s.intersect(&r).unwrap();
    assert_eq!(2, xs.len());
    assert_eq!(5.0, xs[0].t);
    assert_eq!(5.0, xs[1].t);
}

#[test]
fn a_ray_misses_a_sphere() {
    let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere();
    assert!(s.intersect(&r).is_none());
}

#[test]
fn a_ray_originates_inside_a_sphere() {
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = sphere();
    let xs = s.intersect(&r).unwrap();
    assert_eq!(2, xs.len());
    assert_eq!(-1.0, xs[0].t);
    assert_eq!(1.0, xs[1].t);
}

#[test]
fn a_sphere_is_behind_a_ray() {
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = sphere();
    let xs = s.intersect(&r).unwrap();
    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0].t);
    assert_eq!(-4.0, xs[1].t);
}

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let s = sphere();
    let i = intersection(3.5, &s);
    assert_eq!(3.5, i.t);
    assert_eq!(&s, i.object);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
    let s = sphere();
    let i1 = intersection(1.0, &s);
    let i2 = intersection(2.0, &s);
    let mut xs = vec![i2, i1];
    assert_eq!(&intersection(1.0, &s), xs.hit().unwrap());
}

#[test]
fn the_hit_when_some_intersections_have_negative_t() {
    let s = sphere();
    let i1 = intersection(-1.0, &s);
    let i2 = intersection(1.0, &s);
    let mut xs = vec![i2, i1];
    assert_eq!(&intersection(1.0, &s), xs.hit().unwrap());
}

#[test]
fn the_hit_when_all_intersections_have_negative_t() {
    let s = sphere();
    let i1 = intersection(-2.0, &s);
    let i2 = intersection(-1.0, &s);
    let mut xs = vec![i2, i1];
    assert!(xs.hit().is_none());
}

#[test]
fn the_hit_is_always_the_lowest_non_negative_intersection() {
    let s = sphere();
    let i1 = intersection(5.0, &s);
    let i2 = intersection(7.0, &s);
    let i3 = intersection(-3.0, &s);
    let i4 = intersection(2.0, &s);
    let mut xs = vec![i1, i2, i3, i4];
    assert_eq!(&intersection(2.0, &s), xs.hit().unwrap());
}

#[test]
fn translating_a_ray() {
    let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = translation(3.0, 4.0, 5.0);
    let r2 = r.transform(m);
    assert_eq!(point(4.0, 6.0, 8.0), r2.origin);
    assert_eq!(vector(0.0, 1.0, 0.0), r2.direction);
}

#[test]
fn scaling_a_ray() {
    let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(m);
    assert_eq!(point(2.0, 6.0, 12.0), r2.origin);
    assert_eq!(vector(0.0, 3.0, 0.0), r2.direction);
}

#[test]
fn a_spheres_default_transformation() {
    let s = sphere();
    assert_eq!(s.transform(), &identity());
}

#[test]
fn changing_a_spheres_transformation() {
    let t = translation(2.0, 3.0, 4.0);
    let s = sphere().set_transform(translation(2.0, 3.0, 4.0));
    assert_eq!(&t, s.transform());
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere().set_transform(scaling(2.0, 2.0, 2.0));
    let xs = s.intersect(&r).unwrap();
    assert_eq!(2, xs.len());
    assert_eq!(3.0, xs[0].t);
    assert_eq!(7.0, xs[1].t);
}

#[test]
fn intersecting_a_translated_sphere_with_a_ray() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere().set_transform(translation(5.0, 0.0, 0.0));
    assert!(s.intersect(&r).is_none());
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = sphere();
    let n = s.normal_at(&point(1, 0, 0));
    assert_eq!(vector(1, 0, 0), n);
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = sphere();
    let n = s.normal_at(&point(0, 1, 0));
    assert_eq!(vector(0, 1, 0), n);
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = sphere();
    let n = s.normal_at(&point(0, 0, 1));
    assert_eq!(vector(0, 0, 1), n);
}

#[test]
fn the_normal_on_a_sphere_at_a_non_axial_point() {
    let s = sphere();
    let n = s.normal_at(&point(ROOT3 / 3.0, ROOT3 / 3.0, ROOT3 / 3.0));
    assert_eq!(vector(ROOT3 / 3.0, ROOT3 / 3.0, ROOT3 / 3.0), n);
}

#[test]
fn the_normal_is_a_normalized_vector() {
    let s = sphere();
    let n = s.normal_at(&point(ROOT3 / 3.0, ROOT3 / 3.0, ROOT3 / 3.0));
    assert_eq!(n.normalize(), n);
}

#[test]
fn computing_the_normal_on_a_translated_sphere() {
    let s = sphere().set_transform(translation(0.0, 1.0, 0.0));
    let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
    assert_eq!(vector(0.0, 0.70711, -0.70711), n);
}

#[test]
fn computing_the_normal_on_a_scaled_sphere() {
    let s = sphere().set_transform(scaling(1.0, 0.5, 1.0));
    let n = s.normal_at(&point(0.0, ROOT2 / 2.0, -ROOT2 / 2.0));
    assert_eq!(vector(0.0, 0.97014, -0.24254), n);
}

#[test]
fn a_sphere_has_a_default_material() {
    let s = sphere();
    assert_eq!(material(), s.material);
}

#[test]
fn a_sphere_may_be_assigned_a_material() {
    let m = material().set_ambient(1.0);
    let s = sphere().set_material(m);
    assert_eq!(material().set_ambient(1.0), s.material);
}
