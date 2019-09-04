extern crate rayon;
extern crate tracer;

use rayon::prelude::*;

struct Coordinates {
    width: u32,
    height: u32,
    current: u32,
}

impl Iterator for Coordinates {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let x = self.current / self.width;
        let y = self.current % self.width;

        self.current += 1;

        if x < self.height {
            Some((x, y))
        } else {
            None
        }
    }
}

fn coordinates() -> Coordinates {
    Coordinates {
        width: 8,
        height: 4,
        current: 0,
    }
}

fn main() {
    let coords: Vec<(u32, u32)> = coordinates().collect();
    let transformed: Vec<u32> = coords.par_iter().map(|(x, y)| x * y).collect();
    println!("{:?}", transformed);
}
