use crate::frame::Frame;
use evdev::{Device, ABSOLUTE};
use framebuffer::{Framebuffer, KdMode};
use std::error::Error;
use std::path::Path;
use std::time::Instant;

use crate::point::*;
use crate::streamed_data::*;
use crate::swipe::*;

const EV_KEY: u16 = 1;
const EV_ABS: u16 = 3;
const ABS_X: u16 = 0;
const ABS_Y: u16 = 1;
const BTN_TOUCH: u16 = 330;

pub enum RunResponse {
    Exit,
    NothingChanged,
    Draw,
}

#[derive(Debug)]
pub struct Config {
    input_device: Device,
    framebuffer: Framebuffer,
    pub input_min_width: f32,
    pub input_min_height: f32,
    pub input_max_width: f32,
    pub input_max_height: f32,
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        path_to_input_device: P,
        path_to_framebuffer: P,
        input_min_width: f32,
        input_min_height: f32,
        input_max_width: f32,
        input_max_height: f32,
    ) -> Self {
        let device = Device::open(&path_to_input_device).unwrap();
        let framebuffer = Framebuffer::new(path_to_framebuffer).unwrap();

        Config {
            input_device: device,
            framebuffer: framebuffer,
            input_min_width,
            input_min_height,
            input_max_width,
            input_max_height,
        }
    }

    pub fn auto() -> Result<Self, &'static str> {
        let dev = evdev::enumerate();
        // look through all the devices
        for d in dev.into_iter() {
            // if it supports absolute events
            if d.events_supported().contains(ABSOLUTE) {
                // if it supports x and y axis
                let first_axis = 1 << 0;
                if (d.absolute_axes_supported().bits() & first_axis) == 1 {
                    let (x_abs_val, y_abs_val) = {
                        let d_ref = &d;
                        (
                            d_ref.state().abs_vals[0 as usize],
                            d_ref.state().abs_vals[1 as usize],
                        )
                    };

                    let framebuffer = Framebuffer::new("/dev/fb0").unwrap();

                    return Ok(Config {
                        input_device: d,
                        framebuffer: framebuffer,
                        input_min_width: x_abs_val.minimum as f32,
                        input_min_height: y_abs_val.minimum as f32,
                        input_max_width: x_abs_val.maximum as f32,
                        input_max_height: y_abs_val.maximum as f32,
                    });
                }
            }
        }
        Err("could not automatically determine configuration")
    }

    pub fn run(
        &mut self,
        mut f: impl FnMut(&mut Frame, Option<&Swipe>, usize) -> Result<RunResponse, Box<dyn Error>>,
    ) {
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

        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;

        let mut run_response = f(&mut frame, None, delta_t);
        let _ = self.framebuffer.write_frame(&frame.pixels);
        if let Ok(RunResponse::Exit) = run_response {
            return;
        }

        let mut swipe_mem = StreamedSwipe {
            swipe: None,
            streamed_point: StreamedPoint::Nothing,
        };

        'outer: loop {
            for ev in self.input_device.events_no_sync().unwrap() {
                let stream = match (ev._type, ev.code, ev.value) {
                    (EV_ABS, ABS_X, x) => swipe_mem.update(SwipeFragment::PointFragment(
                        PointFragment::X(Timeval::from_timeval(ev.time), x as isize),
                    )),
                    (EV_ABS, ABS_Y, y) => swipe_mem.update(SwipeFragment::PointFragment(
                        PointFragment::Y(Timeval::from_timeval(ev.time), y as isize),
                    )),
                    (EV_KEY, BTN_TOUCH, 0) => swipe_mem.update(SwipeFragment::End),
                    _ => StreamedState::Incomplete,
                };

                let t = start.elapsed().as_millis() as usize;
                let delta_t = t - last_t;
                last_t = t;

                if let StreamedState::Complete(swipe) | StreamedState::Standalone(swipe) = stream {
                    run_response = f(&mut frame, Some(&swipe), delta_t);
                }

                if let Ok(RunResponse::Draw) = run_response {
                    self.framebuffer.write_frame(&frame.pixels);
                } else if let Ok(RunResponse::Exit) = run_response {
                    break 'outer;
                }
            }
        }
        let _ = Framebuffer::set_kd_mode(KdMode::Text).unwrap();
    }
}
