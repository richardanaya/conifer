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

#[derive(Debug)]
struct InputEvent32 {
    timeval_s: u32,
    timeval_us: u32,
    evtype: u16,
    evcode: u16,
    value: u32,
}

impl InputEvent32 {
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        InputEvent32 {
            timeval_s: (bytes[3] as u32) << 24
                | (bytes[2] as u32) << 16
                | (bytes[1] as u32) << 8
                | (bytes[0] as u32),
            timeval_us: (bytes[7] as u32) << 24
                | (bytes[6] as u32) << 16
                | (bytes[5] as u32) << 8
                | (bytes[4] as u32),
            evtype: (bytes[9] as u16) << 8 | bytes[8] as u16,
            evcode: (bytes[11] as u16) << 8 | bytes[10] as u16,
            value: (bytes[15] as u32) << 24
                | (bytes[14] as u32) << 16
                | (bytes[13] as u32) << 8
                | (bytes[12] as u32),
        }
    }

    pub fn from_reader<R: Read>(reader: R) -> Self {
        let mut buffer = [0; 16];
        let mut b = reader.take(16).into_inner();
        b.read(&mut buffer);

        Self::from_bytes(buffer)
    }

    pub fn timeval(&self) -> String {
        format!("{}.{}", self.timeval_s, self.timeval_us)
    }
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

pub enum TimevalSize {
    B8,
    B16,
}

pub struct Config {
    input_device: File,
    framebuffer: Framebuffer,
    input_width: f32,
    input_height: f32,
    timeval_size: TimevalSize,
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        path_to_input_device: P,
        path_to_framebuffer: P,
        input_width: f32,
        input_height: f32,
        timeval_size: TimevalSize,
    ) -> Self {
        let device = OpenOptions::new()
            .read(true)
            .open(path_to_input_device)
            .unwrap();
        let mut framebuffer = Framebuffer::new(path_to_framebuffer).unwrap();

        Config {
            input_device: device,
            framebuffer: framebuffer,
            input_width,
            input_height,
            timeval_size,
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

        loop {
            let input_event = InputEvent32::from_reader(&self.input_device);
            let timeval = input_event.timeval();

            let mut did_update = false;
            if input_event.evtype == EV_KEY {
                if input_event.evcode == BUTTON_LEFT {
                    if input_event.value == 1 {
                        pointer.is_down = true;
                        did_update = true;
                    } else {
                        pointer.is_down = false;
                        points.clear();
                    }
                }
            } else if input_event.evtype == EV_ABS {
                if input_event.evcode == ABS_X {
                    println!(
                        "{} {} {} {} ",
                        input_event.value,
                        self.input_width,
                        w,
                        (input_event.value as f32 / self.input_width * w as f32) as usize
                    );
                    if let Some(thing) = points.get(&timeval) {
                        match thing {
                            StreamedCoord::Y(y) => points.insert(
                                timeval.clone(),
                                StreamedCoord::XY(
                                    (input_event.value as f32 / self.input_width * w as f32)
                                        as usize,
                                    *y,
                                ),
                            ),
                            _ => None,
                        };
                    } else {
                        points.insert(
                            timeval.clone(),
                            StreamedCoord::X(
                                (input_event.value as f32 / self.input_width * w as f32) as usize,
                            ),
                        );
                    }
                } else if input_event.evcode == ABS_Y {
                    println!(
                        "{} {} {} {} ",
                        input_event.value,
                        self.input_height,
                        h,
                        (input_event.value as f32 / self.input_height * h as f32) as usize
                    );

                    if let Some(thing) = points.get(&timeval) {
                        match thing {
                            StreamedCoord::X(x) => points.insert(
                                timeval.clone(),
                                StreamedCoord::XY(
                                    *x,
                                    (input_event.value as f32 / self.input_height * h as f32)
                                        as usize,
                                ),
                            ),
                            _ => (None),
                        };
                    } else {
                        points.insert(
                            timeval.clone(),
                            StreamedCoord::Y(
                                (input_event.value as f32 / self.input_height * h as f32) as usize,
                            ),
                        );
                    }
                }
            } else if input_event.evtype == SYN {
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
                break;
            }
        }

        let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
    }
}
