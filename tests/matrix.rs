extern crate tracer;
use tracer::*;

#[test]
fn constructing_and_inspecting_a_4x4_matrix() {
    let m = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [5.5, 6.5, 7.5, 8.5],
        [9.0, 10.0, 11.0, 12.0],
        [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(1.0, m[0][0]);
    assert_eq!(4.0, m[0][3]);
    assert_eq!(5.5, m[1][0]);
    assert_eq!(7.5, m[1][2]);
    assert_eq!(11.0, m[2][2]);
    assert_eq!(13.5, m[3][0]);
    assert_eq!(15.5, m[3][2]);

    // Can access matrix via tuple
    assert_eq!(1.0, m[0][0]);
    assert_eq!(4.0, m[0][3]);
    assert_eq!(5.5, m[1][0]);
    assert_eq!(7.5, m[1][2]);
    assert_eq!(11.0, m[2][2]);
    assert_eq!(13.5, m[3][0]);
    assert_eq!(15.5, m[3][2]);
}

#[test]
fn constructing_and_inspecting_a_2x2_matrix() {
    let m = matrix([[-3.0, 5.0], [1.0, -2.0]]);

    println!("{:?}", m);

    assert_eq!(-3.0, m[0][0]);
    assert_eq!(5.0, m[0][1]);
    assert_eq!(1.0, m[1][0]);
    assert_eq!(-2.0, m[1][1]);

    assert_eq!("Matrix ((-3.0, 5.0), (1.0, -2.0))", format!("{:?}", m));
}

#[test]
fn constructing_and_inspecting_a_3x3_matrix() {
    let m = matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, 7.0], [0.0, 1.0, 1.0]]);

    assert_eq!(-3.0, m[0][0]);
    assert_eq!(-2.0, m[1][1]);
    assert_eq!(1.0, m[2][2]);

    assert_eq!("Matrix ((-3.0, 5.0, 0.0), (1.0, -2.0, 7.0), (0.0, 1.0, 1.0))", format!("{:?}", m));
}

#[test]
fn matrix_equality_with_identical_matrices() {
    let a = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    let b = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    assert_eq!(a, b);
}

#[test]
fn matrix_equality_with_different_matrices() {
    let a = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    let b = matrix([
        [0.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    assert_ne!(a, b);
}

#[test]
fn multiplying_two_matrices() {
    let a = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 3.0, 4.0, 5.0],
        [3.0, 4.0, 5.0, 6.0],
        [4.0, 5.0, 6.0, 7.0],
    ]);

    let b = matrix([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    let expected = matrix([
        [24.0, 49.0, 98.0, 196.0],
        [31.0, 64.0, 128.0, 256.0],
        [38.0, 79.0, 158.0, 316.0],
        [45.0, 94.0, 188.0, 376.0],
    ]);

    assert_eq!(expected, a * b);
}

#[test]
fn matrix_multiplied_by_a_tuple() {
    let a = matrix([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let b = tuple(1.0, 2.0, 3.0, 1.0);
    assert_eq!(tuple(18.0, 24.0, 33.0, 1.0), a * b);
}

#[test]
fn multiplying_a_matrix_by_the_identity() {
    let a = matrix([
        [0.0, 1.0, 2.0, 4.0],
        [1.0, 2.0, 4.0, 8.0],
        [2.0, 4.0, 8.0, 16.0],
        [4.0, 8.0, 16.0, 32.0],
    ]);

    let identity = matrix([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_eq!(a, a * identity)
}

#[test]
fn transposing_a_matrix() {
    let a = matrix([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);

    let expected = matrix([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);
    assert_eq!(expected, a.transpose());
}

#[test]
fn transposing_the_identity_matrix() {
    let identity = matrix([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    assert_eq!(identity, identity.transpose());
}

#[test]
fn submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let a = matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

    println!("{:?}", a);

    let expected = matrix([[-3.0, 2.0], [0.0, 6.0]]);

    assert_eq!(expected, a.submatrix(0, 2));
}

#[test]
fn submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let a = matrix([
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);

    let expected = matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

    assert_eq!(expected, a.submatrix(2, 1));
}

#[test]
fn calculating_the_determinant_of_a_2x2_matrix() {
    let a = matrix([[1.0, 5.0], [-3.0, 2.0]]);

    assert_eq!(17.0, a.determinant());
}

#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
    let a = matrix([[-3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    let b = a.submatrix(1, 0);
    assert_eq!(25.0, b.determinant());
    assert_eq!(25.0, a.minor(1, 0));
}

#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
    let a = matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    assert_eq!(-12.0, a.minor(0, 0));
    assert_eq!(-12.0, a.cofactor(0, 0));
    assert_eq!(25.0, a.minor(1, 0));
    assert_eq!(-25.0, a.cofactor(1, 0));
}

#[test]
fn calculating_the_determinant_of_a_3x3_matrix() {
    let a = matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    assert_eq!(56.0, a.cofactor(0, 0));
    assert_eq!(12.0, a.cofactor(0, 1));
    assert_eq!(-46.0, a.cofactor(0, 2));
    assert_eq!(-196.0, a.determinant());
}

#[test]
fn calculating_the_determinant_of_a_4x4_matrix() {
    let a = matrix([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(690.0, a.cofactor(0, 0));
    assert_eq!(447.0, a.cofactor(0, 1));
    assert_eq!(210.0, a.cofactor(0, 2));
    assert_eq!(51.0, a.cofactor(0, 3));
    assert_eq!(-4071.0, a.determinant());
}

#[test]
fn testing_an_invertible_matrix_for_invertibility() {
    let a = matrix([
        [6.0, 4.0, 4.0, 4.0],
        [5.0, 5.0, 7.0, 6.0],
        [4.0, -9.0, 3.0, -7.0],
        [9.0, 1.0, 7.0, -6.0],
    ]);
    assert_eq!(-2120.0, a.determinant());
    assert!(a.is_invertible());
}

#[test]
fn testing_an_non_invertible_matrix_for_invertibility() {
    let a = matrix([
        [-4.0, 2.0, -2.0, -3.0],
        [9.0, 6.0, 2.0, 6.0],
        [0.0, -5.0, 1.0, -5.0],
        [0.0, 0.0, 0.0, 0.0],
    ]);
    assert_eq!(0.0, a.determinant());
    assert!(!a.is_invertible());
}

#[test]
fn calculating_the_inverse_of_a_matrix() {
    let a = matrix([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);

    let b = a.inverse();

    assert_eq!(532.0, a.determinant());
    assert_eq!(-160.0, a.cofactor(2, 3));
    assert_eq!(-160.0 / 532.0, b[3][2]);
    assert_eq!(105.0, a.cofactor(3, 2));
    assert_eq!(105.0 / 532.0, b[2][3]);

    let expected = matrix([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(expected, b);
}

#[test]
fn calculating_the_inverse_of_another_matrix() {
    let a = matrix([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);
    let expected = matrix([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_eq!(expected, a.inverse());
}

#[test]
fn calculating_the_inverse_of_a_third_matrix() {
    let a = matrix([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let expected = matrix([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);
    assert_eq!(expected, a.inverse());
}

#[test]
fn multiplying_a_product_by_its_inverse() {
    let a = matrix([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);

    let b = matrix([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    let c = a * b;
    assert_eq!(a, c * b.inverse());
}

#[test]
fn the_transformation_matrix_for_the_default_orientation() {
    let from = point(0, 0, 0);
    let to = point(0, 0, -1);
    let up = vector(0, 1, 0);
    let t = view_transform(from, to, up);
    assert_eq!(identity(), t);
}

#[test]
fn a_view_transformation_matrix_looking_in_positive_z_direction() {
    let from = point(0, 0, 0);
    let to = point(0, 0, 1);
    let up = vector(0, 1, 0);
    let t = view_transform(from, to, up);
    assert_eq!(scaling(-1, 1, -1), t);
}

#[test]
fn the_view_transformation_moves_the_world() {
    let from = point(0, 0, 8);
    let to = point(0, 0, 0);
    let up = vector(0, 1, 0);
    let t = view_transform(from, to, up);
    assert_eq!(translation(0, 0, -8.0), t);
}

#[test]
fn an_arbitrary_view_transformation() {
    let from = point(1, 3, 2);
    let to = point(4, -2, 8);
    let up = vector(1, 1, 0);
    let t = view_transform(from, to, up);
    let expected = matrix([
        [-0.50709, 0.50709, 0.67612, -2.36643],
        [0.76772, 0.60609, 0.12122, -2.82843],
        [-0.35857, 0.59761, -0.71714, 0.00000],
        [0.00000, 0.00000, 0.00000, 1.00000],
    ]);
    assert_eq!(expected, t);
}
