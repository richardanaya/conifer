use evdev::Device;

use framebuffer::{Framebuffer, KdMode};
use std::path::Path;
use std::time::Instant;

use std::collections::HashMap;

mod point;
mod streamed_data;

use crate::point::StreamedPoint;
use crate::point::*;
use crate::streamed_data::*;

const EV_KEY: u16 = 1;
const EV_ABS: u16 = 3;
const EV_MSC: u16 = 4;
const ABS_X: u16 = 0;
const ABS_Y: u16 = 1;
const ABS_MT_SLOT: u16 = 47;
const ABS_MT_POSITION_X: u16 = 53;
const ABS_MT_POSITION_Y: u16 = 54;
const ABS_MT_TRACKING_ID: u16 = 57;
const SYN: u16 = 0;
const BUTTON_LEFT: u16 = 330;

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

pub struct Config {
    input_device: Device,
    framebuffer: Framebuffer,
    input_width: f32,
    input_height: f32,
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        path_to_input_device: P,
        path_to_framebuffer: P,
        input_width: f32,
        input_height: f32,
    ) -> Self {
        let device = Device::open(&path_to_input_device).unwrap();
        let mut framebuffer = Framebuffer::new(path_to_framebuffer).unwrap();

        Config {
            input_device: device,
            framebuffer: framebuffer,
            input_width,
            input_height,
        }
    }

    pub fn run(&mut self, mut f: impl FnMut(&mut Frame, Option<&Point>, usize) -> bool) {
        let start = Instant::now();
        let mut last_t = 0 as usize;

        let w = self.framebuffer.var_screen_info.xres as usize;
        let h = self.framebuffer.var_screen_info.yres as usize;
        let line_length = self.framebuffer.fix_screen_info.line_length as usize;
        let mut frame = Frame {
            width: w,
            height: h,
            line_length,
            bytespp: (self.framebuffer.var_screen_info.bits_per_pixel / 8) as usize,
            pixels: vec![0u8; line_length * h],
        };

        let _ = Framebuffer::set_kd_mode(KdMode::Graphics).unwrap();
        let mut buffer = [0; 24];

        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;

        let mut exit = f(&mut frame, None, delta_t);
        let _ = self.framebuffer.write_frame(&frame.pixels);
        if exit {
            return;
        }

        let mut points: HashMap<String, StreamedPoint> = HashMap::new();

        let mut mem = StreamedPoint::Nothing;

        'outer: loop {
            for ev in self.input_device.events_no_sync().unwrap() {
                let stream = match (ev._type, ev.code, ev.value) {
                    (EV_ABS, ABS_X, x) => {
                        mem.update(StreamedPoint::X(Timeval::from_timeval(ev.time), x as usize))
                    }
                    (EV_ABS, ABS_Y, y) => {
                        mem.update(StreamedPoint::Y(Timeval::from_timeval(ev.time), y as usize))
                    }
                    _ => StreamedState::Incomplete(mem),
                };

                exit = false;
                let t = start.elapsed().as_millis() as usize;
                let delta_t = t - last_t;
                last_t = t;

                match stream {
                    StreamedState::Complete(point) => {
                        mem = StreamedPoint::Nothing;
                        exit = f(&mut frame, Some(&point), delta_t);
                    }
                    StreamedState::Incomplete(incomplete_point) => {
                        mem = incomplete_point;
                        exit = f(&mut frame, None, delta_t);
                    }
                }

                let _ = self.framebuffer.write_frame(&frame.pixels);
                if exit {
                    break 'outer;
                }
            }
        }
        let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
    }
}
