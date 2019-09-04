extern crate tracer;
use tracer::*;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct World {
    gravity: Tuple,
    wind: Tuple,
}

fn main() {
    let mut projectile = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };

    let world = World {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y >= 0 as Float {
        println!(
            "{:12.8} {:12.8}",
            projectile.position.x, projectile.position.y
        );
        projectile = tick(&world, projectile);
    }
}

fn tick(world: &World, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + world.gravity + world.wind;
    Projectile { position, velocity }
}
