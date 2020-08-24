use framebuffer::{Framebuffer, KdMode};
use std::fs::OpenOptions;
use std::io::Read;
use std::time::Instant;

use std::collections::HashMap;

const EV_KEY: u32 = 1;
const EV_ABS: u32 = 3;
const EV_MSC: u32 = 4;
const ABS_X: u32 = 0;
const ABS_Y: u32 = 1;
const ABS_MT_SLOT: u32 = 47;
const ABS_MT_POSITION_X: u32 = 53;
const ABS_MT_POSITION_Y: u32 = 54;
const ABS_MT_TRACKING_ID: u32 = 57;
const SYN: u32 = 0;
const BUTTON_LEFT: u32 = 330;

const INPUT_WIDTH: f32 = 800.;
const INPUT_HEIGHT: f32 = 480.;

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
    XY(usize,usize),
    Nothing,
}

pub fn run(mut f: impl FnMut(&mut Frame, &Pointer, usize) -> bool) {
    let device = OpenOptions::new()
        .read(true)
        .open("/dev/input/event0")
        .unwrap();
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
    let mut buffer = [0; 24];

    let t = start.elapsed().as_millis() as usize;
    let delta_t = t - last_t;
    last_t = t;

    let exit = f(&mut frame, &mut pointer, delta_t);
    let _ = framebuffer.write_frame(&frame.pixels);
    if exit {
        return;
    }

    let mut points: HashMap<String, StreamedCoord> = HashMap::new();

    loop {
        let mut b = (&device).take(16).into_inner();
        b.read(&mut buffer);

        let code_a = (buffer[9] as u32) << 8 | buffer[8] as u32;
        let code_b = (buffer[11] as u32) << 8 | buffer[10] as u32;
        let value = (buffer[15] as u32) << 24
            | (buffer[14] as u32) << 16
            | (buffer[13] as u32) << 8
            | (buffer[12] as u32);

        let timeval = format!("{:03}{:03}{:03}{:03}{:03}{:03}{:03}{:03}", buffer[0],buffer[1],buffer[2],buffer[3],buffer[4],buffer[5],buffer[6],buffer[7]);

        let mut did_update = false;
        if code_a == EV_KEY {
            if code_b == BUTTON_LEFT {
                if value == 1 {
                    pointer.is_down = true;
                    did_update = true;
                } else {
                    pointer.is_down = false;
                    points.clear();
                }
            }
        } else if code_a == EV_ABS {
            if code_b == ABS_X {
                println!(
                    "{} {} {} {} ",
                    value,
                    INPUT_WIDTH,
                    w,
                    (value as f32 / INPUT_WIDTH * w as f32) as usize
                );
                if let Some(thing) = points.get(&timeval) {
                    match thing {
                        StreamedCoord::Y(y) => points.insert(timeval.clone(), StreamedCoord::XY((value as f32 / INPUT_WIDTH * w as f32) as usize, *y)),
                        _ => None,
                    };
                } else {
                    points.insert(timeval.clone(), StreamedCoord::X((value as f32 / INPUT_WIDTH * w as f32) as usize));
                }
            } else if code_b == ABS_Y {
                println!(
                    "{} {} {} {} ",
                    value,
                    INPUT_HEIGHT,
                    h,
                    (value as f32 / INPUT_HEIGHT * h as f32) as usize
                );

                if let Some(thing) = points.get(&timeval) {
                    match thing {
                        StreamedCoord::X(x) => points.insert(timeval.clone(), StreamedCoord::XY(*x, (value as f32 / INPUT_HEIGHT * h as f32) as usize)),
                        _ => (None),
                    };
                } else {
                    points.insert(timeval.clone(),StreamedCoord::Y((value as f32 / INPUT_HEIGHT * h as f32) as usize));
                }
            }
        } else if code_a == SYN {
        }

        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;
        let mut exit = false;
        if let Some(StreamedCoord::XY(x,y)) = points.get(&timeval) {
            pointer.x = *x;
            pointer.y = *y;
            exit = f(&mut frame, &mut pointer, delta_t);
        }
        let _ = framebuffer.write_frame(&frame.pixels);
        if exit {
            break;
        }
    }

    let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
}
