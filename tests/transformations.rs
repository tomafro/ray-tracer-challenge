extern crate tracer;
use tracer::*;

#[test]
fn multiplying_by_a_translation_matrix() {
    let transform = translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    assert_eq!(point(2.0, 1.0, 7.0), transform * p);
}

#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = translation(5.0, -3.0, 2.0);
    let inverse = transform.inverse();
    let p = point(-3.0, 4.0, 5.0);
    assert_eq!(point(-8.0, 7.0, 3.0), inverse * p);
}

#[test]
fn translation_does_not_affect_vectors() {
    let transform = translation(5.0, -3.0, 2.0);
    let v = vector(-3.0, 4.0, 5.0);
    assert_eq!(v, transform * v);
}

#[test]
fn scaling_matrix_applied_to_a_point() {
    let transform = scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);
    assert_eq!(point(-8.0, 18.0, 32.0), transform * p);
}

#[test]
fn scaling_matrix_applied_to_a_vector() {
    let transform = scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);
    assert_eq!(vector(-8.0, 18.0, 32.0), transform * v);
}

#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = scaling(2.0, 3.0, 4.0);
    let inverse = transform.inverse();
    let v = vector(-4.0, 6.0, 8.0);
    assert_eq!(vector(-2.0, 2.0, 2.0), inverse * v);
}

#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let transform = scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(-2.0, 3.0, 4.0), transform * p);
}

#[test]
fn rotating_a_point_around_the_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_x(PI / 4.0);
    let full_quarter = rotation_x(PI / 2.0);
    assert_eq!(point(0.0, ROOT2 / 2.0, ROOT2 / 2.0), half_quarter * p);
    assert_eq!(point(0.0, 0.0, 1.0), full_quarter * p);
}

#[test]
fn rotating_a_point_around_the_y_axis() {
    let p = point(0.0, 0.0, 1.0);
    let half_quarter = rotation_y(PI / 4.0);
    let full_quarter = rotation_y(PI / 2.0);
    assert_eq!(point(ROOT2 / 2.0, 0.0, ROOT2 / 2.0), half_quarter * p);
    assert_eq!(point(1.0, 0.0, 0.0), full_quarter * p);
}

#[test]
fn rotating_a_point_around_the_z_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = rotation_z(PI / 4.0);
    let full_quarter = rotation_z(PI / 2.0);
    assert_eq!(point(-ROOT2 / 2.0, ROOT2 / 2.0, 0.0), half_quarter * p);
    assert_eq!(point(-1.0, 0.0, 0.0), full_quarter * p);
}

#[test]
fn shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(5.0, 3.0, 4.0), transform * p);
}
#[test]
fn shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(6.0, 3.0, 4.0), transform * p);
}
#[test]
fn shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(2.0, 5.0, 4.0), transform * p);
}
#[test]
fn shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(2.0, 7.0, 4.0), transform * p);
}
#[test]
fn shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(2.0, 3.0, 6.0), transform * p);
}
#[test]
fn shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(point(2.0, 3.0, 7.0), transform * p);
}

#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p = point(1.0, 0.0, 1.0);
    let a = rotation_x(PI / 2.0);
    let b = scaling(5.0, 5.0, 5.0);
    let c = translation(10.0, 5.0, 7.0);
    let p2 = a * p;
    assert_eq!(point(1.0, -1.0, 0.0), p2);
    let p3 = b * p2;
    assert_eq!(point(5.0, -5.0, 0.0), p3);
    let p4 = c * p3;
    assert_eq!(point(15.0, 0.0, 7.0), p4);
}

#[test]
fn chained_transformations_must_be_applied_in_reverse_order() {
    let p = point(1.0, 0.0, 1.0);
    let a = rotation_x(PI / 2.0);
    let b = scaling(5.0, 5.0, 5.0);
    let c = translation(10.0, 5.0, 7.0);
    let t = c * b * a;
    assert_eq!(point(15.0, 0.0, 7.0), t * p);
}

#[test]
fn fluent_interface_for_chained_transformations() {
    let p = point(1.0, 0.0, 1.0);
    let t = identity()
        .rotation_x(PI / 2.0)
        .scaling(5.0, 5.0, 5.0)
        .translation(10.0, 5.0, 7.0);
    assert_eq!(point(15.0, 0.0, 7.0), t * p);
}
