use crate::point::Point;
use std::error::Error;

use crate::layer::*;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    pub line_length: usize,
    pub bytespp: usize,
    pub layers: Vec<Layer>,
    pub background: Pixel,
}

impl Canvas {
    pub fn new(width: usize, height: usize, pixels: &[u8]) -> Result<Self, Box<dyn Error>> {
        let bytespp = pixels.len() / (width * height);
        let line_length = bytespp * width;
        let first_layer = Vec::<Layer>::new();
        Ok(Canvas {
            pixels: pixels.to_owned(),
            width,
            height,
            line_length,
            bytespp,
            layers: first_layer,
            background: Pixel { r: 0, g: 0, b: 0 },
        })
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> (u8, u8, u8) {
        let curr_index = y * self.line_length + x * self.bytespp;
        (
            self.pixels[curr_index],
            self.pixels[curr_index + 1],
            self.pixels[curr_index + 2],
        )
    }

    fn set_raw_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let curr_index = y * self.line_length + x * self.bytespp;
        self.pixels[curr_index] = r;
        self.pixels[curr_index + 1] = g;
        self.pixels[curr_index + 2] = b;
    }

    pub fn draw_canvas(&mut self, canvas: &Canvas, x: isize, y: isize) -> Result<(), &'static str> {
        // TODO figure out if this matterns
        //if self.bytespp != canvas.bytespp {
        //    return Err("cannot draw canvas due to incompatible bits per pixel");
        //}
        let start_y = isize::max(y, 0);
        let end_y = isize::min(y + canvas.height as isize, self.height as isize);
        let start_x = isize::max(x, 0);
        let end_x = isize::min(x + canvas.width as isize, self.width as isize);
        for ry in start_y..end_y {
            let len = ((end_x - start_x) * canvas.bytespp as isize) as usize;
            let cur_index = ((ry * self.width as isize + start_x) * self.bytespp as isize) as usize;
            let r_index = (((ry - y) * canvas.width as isize + (start_x - x))
                * canvas.bytespp as isize) as usize;
            let (_, right) = self.pixels.split_at_mut(cur_index);
            let (_, r_right) = canvas.pixels.split_at(r_index);
            right[..len].copy_from_slice(&r_right[..len])
        }
        Ok(())
    }

    pub fn new_layer(&mut self) {
        let layer = Layer {
            pixels: vec![None; self.width * self.height],
            width: self.width,
            height: self.height,
        };
        self.layers.push(layer);
    }

    fn flatten_pixel(&mut self, x: usize, y: usize) {
        let nlayers = self.layers.len();
        let mut long_pixel = &self.background;
        for i in 0..nlayers - 1 {
            if let Some(px) = self.layers[i].get_pixel(x, y).as_ref() {
                long_pixel = px;
                break;
            }
        }

        self.set_raw_pixel(x, y, long_pixel.r, long_pixel.g, long_pixel.b);
    }

    // fn flatten_layers(&mut self) {
    //     for x in 0..self.width - 1 {
    //         for y in 0..self.height - 1 {
    //             let Pixel { r, g, b } = self.flatten_pixel(x, y);
    //             self.set_pixel(x, y, *r, *g, *b);
    //         }
    //     }
    // }

    pub fn pixels(&mut self) -> &Vec<u8> {
        // self.flatten_layers();
        &self.pixels
    }

    pub fn set_pixel(&mut self, layer: usize, x: usize, y: usize, r: u8, g: u8, b: u8) {
        self.layers[layer].set_pixel(x, y, r, g, b);
        self.flatten_pixel(x, y);
    }

    pub fn unset_pixel(&mut self, layer: usize, x: usize, y: usize) {
        if let Some(_) = self.layers[layer].get_pixel(x, y).as_ref() {
            self.layers[layer].unset_pixel(x, y);
            self.flatten_pixel(x, y);
        }
    }

    pub fn flush(&mut self, layer: usize) {
        for x in 0..self.width - 1 {
            for y in 0..self.height - 1 {
                self.unset_pixel(layer, x, y);
            }
        }
    }

    pub fn plot_line(&mut self, layer: usize, point0: Point, point1: Point) {
        let mut x0 = point0.x as isize;
        let mut y0 = point0.y as isize;
        let x1 = point1.x as isize;
        let y1 = point1.y as isize;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; /* error value e_xy */
        loop {
            self.set_pixel(layer, x0 as usize, y0 as usize, 255, 255, 255);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                // e_xy+e_x > 0
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                // e_xy+e_y < 0
                err += dx;
                y0 += sy;
            }
        }
    }
}
