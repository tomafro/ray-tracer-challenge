extern crate tracer;
use tracer::*;

#[test]
fn constructing_a_camera() {
    let hsize = 160;
    let vsize = 120;
    let field_of_view = PI / 2.0;
    let c = camera(hsize, vsize, field_of_view, None);
    assert_eq!(160, c.hsize);
    assert_eq!(120, c.vsize);
    assert_eq!(PI / 2.0, c.field_of_view);
    assert_eq!(identity(), c.transform);
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    let c = camera(200, 125, PI / 2.0, None);
    assert!(float_eq(0.01, c.pixel_size));
}

#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    let c = camera(125, 200, PI / 2.0, None);
    assert!(float_eq(0.01, c.pixel_size));
}

#[test]
fn construct_a_ray_through_the_center_of_the_canvas() {
    let c = camera(201, 101, PI / 2.0, None);
    let r = c.ray_for(100, 50);
    assert_eq!(point(0, 0, 0), r.origin);
    assert_eq!(vector(0, 0, -1), r.direction);
}

#[test]
fn construct_a_ray_through_a_corner_of_the_canvas() {
    let c = camera(201, 101, PI / 2.0, None);
    let r = c.ray_for(0, 0);
    assert_eq!(point(0, 0, 0), r.origin);
    assert_eq!(vector(0.66519, 0.33259, -0.66851), r.direction);
}

#[test]
fn construct_a_ray_when_the_camera_is_transformed() {
    let transform = rotation_y(PI / 4.0) * translation(0, -2, 5);
    let c = camera(201, 101, PI / 2.0, Some(transform));
    let r = c.ray_for(100, 50);
    assert_eq!(point(0, 2, -5), r.origin);
    assert_eq!(vector(ROOT2 / 2.0, 0, -ROOT2 / 2.0), r.direction);
}

// #[test]
// fn rendering_a_world_with_a_camera() {
//     let w = World::default();
//     let from = point(0, 0, -5);
//     let to = point(0, 0, 0);
//     let up = vector(0, 1, 0);
//     let c = camera(11, 11, PI / 2.0, Some(view_transform(from, to, up)));
//     let image = c.render(w);
//     assert_eq!(colour(0.38066, 0.47583, 0.2855), image.get_pixel(5, 5));
// }
