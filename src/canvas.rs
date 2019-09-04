use crate::colour::*;

pub trait Canvas {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, colour: Colour);
    fn get_pixel(&self, x: usize, y: usize) -> Colour;
}

#[derive(Debug)]
pub struct VecColourCanvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Colour>,
}

impl VecColourCanvas {
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn pixels(&self) -> &Vec<Colour> {
        &self.pixels
    }
}

impl Canvas for VecColourCanvas {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        let ix = self.index(x, y);
        self.pixels[ix] = colour;
    }

    fn get_pixel(&self, x: usize, y: usize) -> Colour {
        self.pixels[self.index(x, y)]
    }
}

pub fn canvas(width: usize, height: usize) -> VecColourCanvas {
    VecColourCanvas {
        width,
        height,
        pixels: vec![BLACK; width * height],
    }
}
