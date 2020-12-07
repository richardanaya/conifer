use crate::canvas::Canvas;
use std::error::Error;
use crate::util::*;

pub struct BlitMap {
    pub width: usize,
    pub height: usize,
    pub map: Vec<bool>,
}

impl BlitMap {
    pub fn bound_check(&self, x: usize, y:usize) -> bool {
        (x >= 0 && x < self.width && y >= 0 && y < self.height)
    }

    pub fn get_bool(&self, x: usize, y: usize) -> Result<bool, Box<dyn Error>>  {
        if self.bound_check(x,y) {
            Ok(self.map[y * self.width + x])
        } else {
            Err("Out of bonds.".into())
        }
    }

    pub fn from_canvas(canvas: &Canvas) -> BlitMap {
        return BlitMap {
            width: canvas.width,
            height: canvas.height,
            map: vec![true; canvas.width * canvas.height],
        };
    }

    pub fn from_canvas_with_alpha(canvas: &Canvas) -> BlitMap {
        let mut b = vec![false; canvas.width * canvas.height];
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                let b_index = (y * canvas.width) + x;
                let cur_index = b_index;
                if canvas.pixels[cur_index] & 4278190080 > 0 {
                    b[b_index] = true;
                }
            }
        }
        BlitMap { 
            width: canvas.width,
            height: canvas.height,
            map: b }
    }

    pub fn from_canvas_with_bg_color(canvas: &Canvas, r: u8, g: u8, b: u8) -> BlitMap {
        let mut bmap = vec![false; canvas.width * canvas.height];
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                let b_index = (y * canvas.width) + x;
                let cur_index = b_index;
                let (_, pix_r, pix_g, pix_b) = rgba_from_color(canvas.pixels[cur_index]);
                if (pix_r, pix_g, pix_b) != (r,g,b) {
                    bmap[b_index] = true;
                }
            }
        }
        BlitMap { 
            width: canvas.width,
            height: canvas.height,
            map: bmap }
    }
}
