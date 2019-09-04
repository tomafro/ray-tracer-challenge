extern crate tracer;
use tracer::*;
#[macro_use]
extern crate indoc;

#[test]
fn creating_a_canvas() {
    let c = canvas(10, 20);
    assert_eq!(10, c.width);
    assert_eq!(20, c.height);
    for x in 0..10 {
        for y in 0..20 {
            assert_eq!(colour(0.0, 0.0, 0.0), c.get_pixel(x, y));
        }
    }
}

#[test]
fn writing_pixels_to_a_canvas() {
    let mut c = canvas(10, 20);
    let red = colour(1.0, 0.0, 0.0);
    c.set_pixel(2, 3, red);
    assert_eq!(red, c.get_pixel(2, 3));
}

#[test]
fn canvas_to_ppm() {
    let mut canvas = ppm_canvas(5, 3);
    let c1 = colour(1.5, 0.0, 0.0);
    let c2 = colour(0.0, 0.5, 0.0);
    let c3 = colour(-0.5, 0.0, 1.0);
    canvas.set_pixel(0, 0, c1);
    canvas.set_pixel(2, 1, c2);
    canvas.set_pixel(4, 2, c3);

    println!("{:?}", c1.to_u8_rgb());

    let expected = indoc!(
        "
        P3
        5 3
        255
        255   0   0   0   0   0   0   0   0   0   0   0   0   0   0
          0   0   0   0   0   0   0 127   0   0   0   0   0   0   0
          0   0   0   0   0   0   0   0   0   0   0   0   0   0 255
    "
    );
    assert_eq!(expected, canvas.to_ppm());
}
