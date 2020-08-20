use framebuffer::{Framebuffer, KdMode};
use std::time::Instant;
use std::io::Read;
use std::fs::{OpenOptions};

pub struct Pointer {
    pub is_down: bool,
    pub x: usize,
    pub y: usize,
}

pub struct Frame {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
    line_length: usize,
    bytespp: usize,
}

impl Frame {
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
}

pub fn run(mut f: impl FnMut(&mut Frame, &Pointer, usize) -> bool) {
    let device = OpenOptions::new()
            .read(true)
            .open("/dev/input/mouse2").unwrap();
    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
    let mut pointer = Pointer {
        is_down: false,
        x: 0,
        y: 0,
    };
    let start = Instant::now();
    let mut last_t = 0 as usize;

    let w = framebuffer.var_screen_info.xres as usize;
    let h = framebuffer.var_screen_info.yres as usize;
    let line_length = framebuffer.fix_screen_info.line_length as usize;
    let mut frame = Frame {
        width: w,
        height: h,
        line_length,
        bytespp: (framebuffer.var_screen_info.bits_per_pixel / 8) as usize,
        pixels: vec![0u8; line_length * h],
    };

    let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
    let mut buffer = [0;3];
    loop {
        let mut b = (&device).take(3).into_inner();
        b.read(&mut buffer);
      
        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;
        let exit = f(&mut frame, &mut pointer, delta_t);
        let _ = framebuffer.write_frame(&frame.pixels);
        if exit {
            break;
        }
    }

    let _ = std::io::stdin().read_line(&mut String::new());
    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}
