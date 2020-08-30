use crate::point::Point;
use std::error::Error;

#[derive(Debug)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    pub line_length: usize,
    pub bytespp: usize,
}

impl Frame {
    pub fn new(width: usize, height: usize, pixels: &[u8]) -> Result<Self, Box<dyn Error>> {
        let bytespp = pixels.len() / (width * height);
        let line_length = bytespp * width;
        Ok(Frame {
            pixels: pixels.to_owned(),
            width,
            height,
            line_length,
            bytespp,
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

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let curr_index = y * self.line_length + x * self.bytespp;
        self.pixels[curr_index] = r;
        self.pixels[curr_index + 1] = g;
        self.pixels[curr_index + 2] = b;
    }

    pub fn draw_frame(&mut self, frame: &Frame, x: isize, y: isize) -> Result<(), &'static str> {
        // TODO figure out if this matterns
        //if self.bytespp != frame.bytespp {
        //    return Err("cannot draw frame due to incompatible bits per pixel");
        //}
        for ry in isize::max(y,0)..isize::min(y+frame.height as isize,self.height as isize) {
            for rx in isize::max(x,0)..isize::min(x+frame.width as isize, self.width as isize) {
                let curr_index = ((ry * self.width as isize + rx) * self.bytespp as isize) as usize;
                let r_index = (((ry-y) * frame.width as isize + (rx-x)) * frame.bytespp as isize) as usize;
                self.pixels[curr_index] = frame.pixels[r_index + 2];
                self.pixels[curr_index + 1] = frame.pixels[r_index + 1];
                self.pixels[curr_index + 2] = frame.pixels[r_index];
            }
        }
        Ok(())
    }

    pub fn plot_line(&mut self, point0: Point, point1: Point) {
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
            /* loop */
            self.set_pixel(x0 as usize, y0 as usize, 255, 255, 255);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                /* e_xy+e_x > 0 */
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                /* e_xy+e_y < 0 */
                err += dx;
                y0 += sy;
            }
        }
    }
}
