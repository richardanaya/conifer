use crate::canvas::*;
use crate::point::*;

#[derive(Debug, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub pixels: Vec<Option<Pixel>>,
    pub width: usize,
    pub height: usize,
}

impl Layer {
    pub fn get_pixel(&self, x: usize, y: usize) -> &Option<Pixel> {
        let index = y * self.width + x;
        &self.pixels[index]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let index = y * self.width + x;
        self.pixels[index] = Some(Pixel { r, g, b })
    }

    pub fn unset_pixel(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;
        self.pixels[index] = None;
    }
}
