extern crate tracer;
use tracer::*;

#[test]
fn tuple_with_w_equals_1_is_a_point() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1.0,
    };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn tuple_with_w_equals_0_is_a_vector() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 0.0,
    };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn point_describes_tuples_with_w_equals_1() {
    assert_eq!(
        point(4.3, -4.2, 3.1),
        Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0
        }
    );
}

#[test]
fn vector_describes_tuples_with_w_equals_0() {
    assert_eq!(
        vector(4.3, -4.2, 3.1),
        Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0
        }
    );
}

// Adding two tuples Given a1 ← tuple(3, -2, 5, 1) And a2 ← tuple(-2, 3, 1, 0)
// Then a1 + a2 = tuple(1, 1, 6, 1)
#[test]
fn adding_two_tuples() {
    let a = tuple(3.0, -2.0, 5.0, 1.0);
    let b = tuple(-2.0, 3.0, 1.0, 0.0);

    assert_eq!(tuple(1.0, 1.0, 6.0, 1.0), a + b);
}

#[test]
fn subtracting_two_points() {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    assert_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(point(-2.0, -4.0, -6.0), p - v);
}

#[test]
fn subtracting_two_vectors() {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    assert_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = vector(0.0, 0.0, 0.0);
    let v = vector(1.0, -2.0, 3.0);
    assert_eq!(vector(-1.0, 2.0, -3.0), zero - v);
}

#[test]
fn negating_a_tuple() {
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple(-1.0, 2.0, -3.0, 4.0), -a);
}

#[test]
fn multiplying_a_tuple_by_a_scalar() {
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple(3.5, -7.0, 10.5, -14.0), a * 3.5);
}

#[test]
fn multiplying_a_tuple_by_a_fraction() {
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple(0.5, -1.0, 1.5, -2.0), a * 0.5);
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let a = tuple(1.0, -2.0, 3.0, -4.0);
    assert_eq!(tuple(0.5, -1.0, 1.5, -2.0), a / 2.0);
}

#[test]
fn magnitude_of_vector_1_0_0() {
    let v = vector(1.0, 0.0, 0.0);
    assert_eq!(1.0, v.magnitude());
}

#[test]
fn magnitude_of_vector_0_1_0() {
    let v = vector(0.0, 1.0, 0.0);
    assert_eq!(1.0, v.magnitude());
}

#[test]
fn magnitude_of_vector_0_0_1() {
    let v = vector(0.0, 0.0, 1.0);
    assert_eq!(1.0, v.magnitude());
}

#[test]
fn magnitude_of_vector_1_2_3() {
    let v = vector(1.0, 2.0, 3.0);
    assert_eq!((14.0 as Float).sqrt(), v.magnitude());
}

#[test]
fn magnitude_of_vector_neg1_neg2_neg3() {
    let v = vector(-1.0, -2.0, -3.0);
    assert_eq!((14.0 as Float).sqrt(), v.magnitude());
}

#[test]
fn normalizing_vector_1_2_3() {
    let v = vector(1.0, 2.0, 3.0);
    assert_eq!(vector(0.26726, 0.53452, 0.80178), v.normalize());
}

#[test]
fn magnitude_of_a_normalized_vector() {
    let v = vector(1.0, 2.0, 3.0);
    assert!(float_eq(1.0, v.normalize().magnitude()));
}

#[test]
fn dot_product_of_two_tuples() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(20.0, a.dot(b));
}

#[test]
fn cross_product_of_two_vectors() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);

    assert_eq!(vector(-1.0, 2.0, -1.0), a.cross(b));
    assert_eq!(vector(1.0, -2.0, 1.0), b.cross(a));
}

#[test]
fn colours_are_red_green_blue_tuples() {
    let c = colour(-0.5, 0.4, 1.7);
    assert_eq!(-0.5, c.red());
    assert_eq!(0.4, c.green());
    assert_eq!(1.7, c.blue());
}

#[test]
fn adding_colours() {
    let c1 = colour(0.9, 0.6, 0.75);
    let c2 = colour(0.7, 0.1, 0.25);
    assert_eq!(colour(1.6, 0.7, 1.0), c1 + c2);
}

#[test]
fn subtracting_colours() {
    let c1 = colour(0.9, 0.6, 0.75);
    let c2 = colour(0.7, 0.1, 0.25);
    assert_eq!(colour(0.2, 0.5, 0.5), c1 - c2);
}

#[test]
fn multiplying_a_colour_by_a_scalar() {
    let c = colour(0.2, 0.3, 0.4);
    assert_eq!(colour(0.4, 0.6, 0.8), c * 2 as Float);
}

#[test]
fn multiplying_colours() {
    let c1 = colour(1.0, 0.2, 0.4);
    let c2 = colour(0.9, 1.0, 0.1);
    assert_eq!(colour(0.9, 0.2, 0.04), c1 * c2);
}

#[test]
fn reflecting_a_vector_approaching_at_45() {
    let v = vector(1, -1, 0);
    let n = vector(0, 1, 0);
    let r = v.reflect(n);
    assert_eq!(r, vector(1, 1, 0));
}

#[test]
fn reflecting_a_vector_off_a_slanted_surface() {
    let v = vector(0, -1, 0);
    let n = vector(ROOT2 / 2.0, ROOT2 / 2.0, 0);
    let r = v.reflect(n);
    assert_eq!(r, vector(1, 0, 0));
}
