extern crate tracer;

use tracer::*;

#[test]
fn precomputing_the_state_of_an_intersection() {
    let ray = ray(point(0, 0, -5), vector(0, 0, 1));
    let shape = sphere();
    let hit = intersection(4.0, &shape);
    let prepared = hit.prepare(&ray);
    assert_eq!(point(0, 0, -1), prepared.point);
    assert_eq!(vector(0, 0, -1), prepared.eyev);
    assert_eq!(vector(0, 0, -1), prepared.normalv);
}

#[test]
fn an_intersection_occurs_on_the_outside() {
    let ray = ray(point(0, 0, -5), vector(0, 0, 1));
    let shape = sphere();
    let hit = intersection(4.0, &shape);
    let prepared = hit.prepare(&ray);
    assert!(!prepared.inside);
}

#[test]
fn an_intersection_occurs_on_the_inside() {
    let ray = ray(point(0, 0, 0), vector(0, 0, 1));
    let shape = sphere();
    let hit = intersection(1.0, &shape);
    let prepared = hit.prepare(&ray);
    assert_eq!(point(0, 0, 1), prepared.point);
    assert_eq!(vector(0, 0, -1), prepared.eyev);
    assert!(prepared.inside);
    assert_eq!(vector(0, 0, -1), prepared.normalv);
}

#[test]
fn shading_an_intersection() {
    let world = World::default();
    let ray = ray(point(0, 0, -5), vector(0, 0, 1));
    let shape = &world.objects[0];
    let hit = intersection(4.0, &shape);
    let prepared = hit.prepare(&ray);
    let c = prepared.shade(&world);
    assert_eq!(colour(0.38066, 0.47583, 0.2855), c);
}

// #[test]
// fn shading_an_intersection_from_the_inside() {
//     let mut world = World::default();
//     world.lights = vec![point_light(point(0, 0.25, 0), colour(1, 1, 1))];
//     let ray = ray(point(0, 0, 0), vector(0, 0, 1));
//     let shape = &world.objects[1];
//     let hit = intersection(0.5, &shape);
//     let prepared = hit.prepare(&ray);
//     let c = prepared.shade(&world);
//     assert_eq!(colour(0.90498, 0.90498, 0.90498), c);
// }
