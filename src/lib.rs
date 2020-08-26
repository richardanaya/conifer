use evdev::Device;

use framebuffer::{Framebuffer, KdMode};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use std::time::Instant;

use std::collections::HashMap;

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

enum StreamedCoord {
    X(usize),
    Y(usize),
    XY(usize, usize),
    Nothing,
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

    pub fn run(&mut self, mut f: impl FnMut(&mut Frame, &Pointer, usize) -> bool) {
        let mut pointer = Pointer {
            is_down: false,
            x: 0,
            y: 0,
        };
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

        let exit = f(&mut frame, &mut pointer, delta_t);
        let _ = self.framebuffer.write_frame(&frame.pixels);
        if exit {
            return;
        }

        let mut points: HashMap<String, StreamedCoord> = HashMap::new();

        'outer: loop {
            for ev in self.input_device.events_no_sync().unwrap() {
                println!("{:?}", ev);

                let timeval = format!("{}.{}", ev.time.tv_sec, ev.time.tv_usec);

                let mut did_update = false;
                if ev._type == EV_KEY {
                    if ev.code == BUTTON_LEFT {
                        if ev.value == 1 {
                            pointer.is_down = true;
                            did_update = true;
                        } else {
                            pointer.is_down = false;
                            points.clear();
                        }
                    }
                } else if ev._type == EV_ABS {
                    if ev.code == ABS_X {
                        println!(
                            "{} {} {} {} ",
                            ev.value,
                            self.input_width,
                            w,
                            (ev.value as f32 / self.input_width * w as f32) as usize
                        );
                        if let Some(thing) = points.get(&timeval) {
                            match thing {
                                StreamedCoord::Y(y) => points.insert(
                                    timeval.clone(),
                                    StreamedCoord::XY(
                                        (ev.value as f32 / self.input_width * w as f32) as usize,
                                        *y,
                                    ),
                                ),
                                _ => None,
                            };
                        } else {
                            points.insert(
                                timeval.clone(),
                                StreamedCoord::X(
                                    (ev.value as f32 / self.input_width * w as f32) as usize,
                                ),
                            );
                        }
                    } else if ev.code == ABS_Y {
                        println!(
                            "{} {} {} {} ",
                            ev.value,
                            self.input_height,
                            h,
                            (ev.value as f32 / self.input_height * h as f32) as usize
                        );

                        if let Some(thing) = points.get(&timeval) {
                            match thing {
                                StreamedCoord::X(x) => points.insert(
                                    timeval.clone(),
                                    StreamedCoord::XY(
                                        *x,
                                        (ev.value as f32 / self.input_height * h as f32) as usize,
                                    ),
                                ),
                                _ => (None),
                            };
                        } else {
                            points.insert(
                                timeval.clone(),
                                StreamedCoord::Y(
                                    (ev.value as f32 / self.input_height * h as f32) as usize,
                                ),
                            );
                        }
                    }
                } else if ev._type == SYN {
                }

                let t = start.elapsed().as_millis() as usize;
                let delta_t = t - last_t;
                last_t = t;
                let mut exit = false;
                if let Some(StreamedCoord::XY(x, y)) = points.get(&timeval) {
                    pointer.x = *x;
                    pointer.y = *y;
                    exit = f(&mut frame, &mut pointer, delta_t);
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
