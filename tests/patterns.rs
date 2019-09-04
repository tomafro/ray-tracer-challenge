extern crate tracer;

use tracer::*;

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = stripe_pattern(WHITE, BLACK);
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 1, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 2, 0)));
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = stripe_pattern(WHITE, BLACK);
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 1)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 2)));
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = stripe_pattern(WHITE, BLACK);
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0.9, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(1, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(-0.1, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(-0.9, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(-1.1, 0, 0)));
}

#[test]
fn stripes_with_an_object_transformation() {
    let object = sphere().set_transform(scaling(2, 2, 2));
    let pattern = stripe_pattern(BLACK, WHITE);
    let c = pattern.colour_at_object(&object, &point(1.5, 0, 0));
    assert_eq!(BLACK, c);
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let object = sphere();
    let pattern = stripe_pattern(BLACK, WHITE).set_transform(scaling(2, 2, 2));
    let c = pattern.colour_at_object(&object, &point(1.5, 0, 0));
    assert_eq!(BLACK, c);
}

#[test]
fn stripes_with_both_an_object_and_a_pattern_transformation() {
    let object = sphere().set_transform(scaling(2, 2, 2));
    let pattern = stripe_pattern(BLACK, WHITE).set_transform(translation(0.5, 0, 0));
    let c = pattern.colour_at_object(&object, &point(2.5, 0, 0));
    assert_eq!(BLACK, c);
}

// #[test]
// fn the_default_pattern_transformation() {
//     let pattern = test_pattern()
//   Then pattern.transform = identity_matrix

// #[test]
// fn assigning_a_transformation() {
//     let pattern = test_pattern()
//   When set_pattern_transform(pattern, translation(1, 2, 3))
//   Then pattern.transform = translation(1, 2, 3)

// #[test]
// fn pattern_with_an_object_transformation() {
//     let shape = sphere()
//     And set_transform(shape, scaling(2, 2, 2))
//     And pattern = test_pattern()
//      let c = pattern_at_shape(pattern, shape, point(2, 3, 4));
//   Then c = colour(1, 1.5, 2)

// #[test]
// fn pattern_with_a_pattern_transformation() {
//     let shape = sphere()
//     And pattern = test_pattern()
//     And set_pattern_transform(pattern, scaling(2, 2, 2))
//      let c = pattern_at_shape(pattern, shape, point(2, 3, 4));
//   Then c = colour(1, 1.5, 2)

// #[test]
// fn pattern_with_both_an_object_and_a_pattern_transformation() {
//     let shape = sphere()
//     And set_transform(shape, scaling(2, 2, 2))
//     And pattern = test_pattern()
//     And set_pattern_transform(pattern, translation(0.5, 1, 1.5))
//      let c = pattern_at_shape(pattern, shape, point(2.5, 3, 3.5));
//   Then c = colour(0.75, 0.5, 0.25)

#[test]
fn gradient_linearly_interpolates_between_colours() {
    let pattern = gradient_pattern(BLACK, WHITE);
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(
        colour(0.25, 0.25, 0.25),
        pattern.colour_at(&point(0.25, 0, 0))
    );
    assert_eq!(colour(0.5, 0.5, 0.5), pattern.colour_at(&point(0.5, 0, 0)));
    assert_eq!(
        colour(0.75, 0.75, 0.75),
        pattern.colour_at(&point(0.75, 0, 0))
    );
}

#[test]
fn ring_should_extend_in_both_x_and_z() {
    let pattern = ring_pattern(BLACK, WHITE);
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(1, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 1)));
    assert_eq!(WHITE, pattern.colour_at(&point(0.708, 0, 0.708)));
}

#[test]
fn checkers_should_repeat_in_x() {
    let pattern = checkers_pattern(BLACK, WHITE);
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(0.99, 0, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(1.01, 0, 0)));
}

#[test]
fn checkers_should_repeat_in_y() {
    let pattern = checkers_pattern(BLACK, WHITE);
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0.99, 0)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 1.01, 0)));
}

#[test]
fn checkers_should_repeat_in_z() {
    let pattern = checkers_pattern(BLACK, WHITE);
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0)));
    assert_eq!(BLACK, pattern.colour_at(&point(0, 0, 0.99)));
    assert_eq!(WHITE, pattern.colour_at(&point(0, 0, 1.01)));
}
