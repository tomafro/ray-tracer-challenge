use crate::{colour, Colour, Float};

use std::collections::{HashMap, HashSet};
use std::io::BufWriter;
use std::io::Write;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    colours: HashSet<(u8, u8, u8)>,
    pixels: Vec<(u8, u8, u8)>,
}

impl Canvas {
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    pub fn pixels(&self) -> &Vec<(u8, u8, u8)> {
        &self.pixels
    }
}

impl crate::Canvas for Canvas {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn set_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        let ix = self.index(x, y);
        let rgb = colour.to_u8_rgb();
        self.colours.insert(rgb);
        self.pixels[ix] = rgb;
    }

    fn get_pixel(&self, x: usize, y: usize) -> Colour {
        let (r, g, b) = self.pixels[self.index(x, y)];
        colour(r as Float / 255.0, g as Float / 255.0, b as Float / 255.0)
    }
}

fn colour_bytes(mut colour: u8, output: &mut [u8]) {
    let h = colour / 100;
    colour = colour % 100;
    let t = colour / 10;
    colour = colour % 10;

    if h > 0 {
        output[0] = 48 + h;
    }

    if t > 0 || h > 0 {
        output[1] = 48 + t;
    }

    output[2] = 48 + colour;
}

pub trait ToPpm {
    fn write_ppm_to<T: std::io::Write>(&self, writer: &mut T);

    fn to_ppm(&self) -> String {
        let mut output = Vec::new();
        self.write_ppm_to(&mut output);
        std::str::from_utf8(&output).unwrap().to_string()
    }
}

impl ToPpm for Canvas {
    fn write_ppm_to<T: std::io::Write>(&self, writer: &mut T) {
        let mut writer = BufWriter::new(writer);
        write!(writer, "P3\n{} {}\n255\n", self.width, self.height).expect("Failed to write ppm");

        let mut bytes_for_colour = HashMap::new();

        for colour in &self.colours {
            let mut bytes = [32 as u8; 11];
            colour_bytes(colour.0, &mut bytes[0..3]);
            colour_bytes(colour.1, &mut bytes[4..7]);
            colour_bytes(colour.2, &mut bytes[8..11]);
            bytes_for_colour.insert(colour, bytes);
        }

        for chunk in self.pixels().chunks(5) {
            let mut n = 0;
            for colour in chunk {
                n = n + 1;
                let bytes = bytes_for_colour.get(colour).unwrap();
                writer.write_all(bytes).unwrap();

                if n != 5 {
                    writer.write_all(b" ").unwrap();
                }
            }
            writer.write_all(b"\n").unwrap();
        }
    }
}

pub fn ppm_canvas(width: usize, height: usize) -> Canvas {
    let default = crate::colour::BLACK.to_u8_rgb();
    let mut colours = HashSet::new();
    colours.insert(default);
    let pixels = vec![default; width * height];
    Canvas {
        width,
        height,
        colours,
        pixels,
    }
}
