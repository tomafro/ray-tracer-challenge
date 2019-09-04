extern crate tracer;
use tracer::*;

#[test]
fn creating_a_world() {
    let w = world();
    assert_eq!(0, w.objects.len());
    assert!(w.lights.is_empty());
}

#[test]
fn the_default_world() {
    let light = point_light(point(-10, 10, -10), colour(1, 1, 1));
    let s1 = sphere().set_material(
        material()
            .set_colour(0.8, 1.0, 0.6)
            .set_diffuse(0.7)
            .set_specular(0.2),
    );

    let s2 = sphere().set_transform(scaling(0.5, 0.5, 0.5));
    let world = World::default();

    assert_eq!(&light, world.light());
    assert!(world.contains(&s1));
    assert!(world.contains(&s2));
}

#[test]
fn intersect_a_world_with_a_ray() {
    let world = World::default();
    let ray = ray(point(0, 0, -5), vector(0, 0, 1));
    let xs = world.intersect(&ray);
    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(4.5, xs[1].t);
    assert_eq!(5.5, xs[2].t);
    assert_eq!(6.0, xs[3].t);
}

#[test]
fn the_colour_when_a_ray_misses() {
    let world = World::default();
    let ray = ray(point(0, 0, -5), vector(0, 1, 0));
    let c = world.colour_at(&ray);
    assert_eq!(colour(0, 0, 0), c);
}

#[test]
fn the_colour_when_a_ray_hits() {
    let world = World::default();
    let ray = ray(point(0, 0, -5), vector(0, 0, 1));
    let c = world.colour_at(&ray);
    assert_eq!(colour(0.38066, 0.47583, 0.2855), c);
}

#[test]
fn the_colour_with_an_intersection_behind_the_ray() {
    let mut world = World::default();

    world.objects[0] = sphere().set_material(
        material()
            .set_colour(0.8, 1.0, 0.6)
            .set_diffuse(0.7)
            .set_specular(0.2)
            .set_ambient(1.0),
    );

    world.objects[1] = sphere()
        .set_transform(scaling(0.5, 0.5, 0.5))
        .set_material(material().set_colour(1.0, 1.0, 1.0).set_ambient(1.0));

    let ray = ray(point(0, 0, 0.75), vector(0, 0, -1));
    let c = world.colour_at(&ray);

    assert_eq!(colour(1.0, 1.0, 1.0), c);
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let world = World::default();
    let p = point(0, 10, 0);
    assert_eq!(false, world.is_shadowed(&p, &world.lights[0]));
}

#[test]
fn shadow_when_an_object_is_between_the_point_and_the_light() {
    let world = World::default();
    let p = point(10, -10, 10);
    assert_eq!(true, world.is_shadowed(&p, &world.lights[0]));
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    let world = World::default();
    let p = point(-20, 20, -20);
    assert_eq!(false, world.is_shadowed(&p, &world.lights[0]));
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    let world = World::default();
    let p = point(-2, 2, -2);
    assert_eq!(false, world.is_shadowed(&p, &world.lights[0]));
}

#[test]
fn when_shade_hit_is_given_an_intersection_in_shadow() {
    let mut world = World::default();
    world.lights = vec![point_light(point(0, 0, -10), colour(1, 1, 1))];
    let s1 = sphere();
    let s2 = sphere().set_transform(translation(0, 0, 10));
    world.objects = vec![s1, s2.clone()];
    let r = ray(point(0, 0, 5), vector(0, 0, 1));
    let h = intersection(4.0, &s2);
    let prepared = h.prepare(&r);
    let c = prepared.shade(&world);
    assert_eq!(colour(0.1, 0.1, 0.1), c);
}
