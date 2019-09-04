extern crate tracer;
use tracer::Canvas;
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
        velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let world = World {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };

    let red = colour(0.9, 0.1, 0.1);
    let mut canvas = ppm_canvas(900, 550);

    while projectile.position.y >= 0 as Float {
        let x = projectile.position.x as usize;
        let y = (550.0 - projectile.position.y) as usize;
        canvas.set_pixel(x, y, red);
        projectile = tick(&world, projectile);
    }
    println!("{}", canvas.to_ppm());
}

fn tick(world: &World, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + world.gravity + world.wind;
    Projectile { position, velocity }
}
