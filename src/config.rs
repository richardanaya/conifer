use crate::frame::Frame;
use crate::framebuffer::Framebuffer;
use std::error::Error;
use std::path::Path;
use std::time::Instant;

use crate::input::{Input, InputEvent};
use crate::point::*;
use crate::streamed_data::*;
use crate::swipe::*;
use std::cell::RefCell;
use std::rc::Rc;

pub enum RunResponse {
    Exit,
    NothingChanged,
    Draw,
}

#[derive(Debug)]
pub struct Config {
    framebuffer: Rc<RefCell<Framebuffer>>,
    input_device: Input,
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
        let framebuffer = Framebuffer::new(path_to_framebuffer).unwrap();
        let input_device = Input::new(
            &path_to_input_device,
            input_min_width,
            input_min_height,
            input_max_width,
            input_max_height,
        );

        Config {
            framebuffer: Rc::new(RefCell::new(framebuffer)),
            input_device,
        }
    }

    pub fn auto() -> Result<Self, &'static str> {
        let framebuffer = Framebuffer::auto().unwrap();
        let input_device = Input::auto().unwrap();
        Ok(Config {
            input_device,
            framebuffer: Rc::new(RefCell::new(framebuffer)),
        })
    }

    pub fn run(
        &mut self,
        mut f: impl FnMut(&mut Frame, Option<&Swipe>, usize) -> Result<RunResponse, Box<dyn Error>>,
    ) {
        let start = Instant::now();
        let mut last_t = 0 as usize;

        let mut fb = self.framebuffer.borrow_mut();

        let w = fb.width();
        let h = fb.height();
        let line_length = fb.line_length();
        let mut frame = Frame {
            width: w,
            height: h,
            line_length,
            bytespp: fb.bytes_per_pixel(),
            pixels: vec![0u8; line_length * h],
        };

        fb.setup();

        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;

        let mut run_response = f(&mut frame, None, delta_t);
        if let Err(err) = run_response {
            fb.shutdown();
            eprintln!("Error occured in user run loop: {}", err);
            std::process::exit(0);
        }
        fb.write_frame(&frame.pixels);
        if let Ok(RunResponse::Exit) = run_response {
            fb.shutdown();
            std::process::exit(0);
        }

        let mut swipe_mem = StreamedSwipe {
            swipe: None,
            streamed_point: StreamedPoint::Nothing,
        };

        self.input_device.on_event(|ev| {
            let stream = match ev {
                InputEvent::PartialX(x, time) => {
                    swipe_mem.update(SwipeFragment::PointFragment(PointFragment::X(time, x)))
                }
                InputEvent::PartialY(y, time) => swipe_mem.update(SwipeFragment::PointFragment(
                    PointFragment::Y(time, y as isize),
                )),
                InputEvent::ButtonDown(_) => swipe_mem.update(SwipeFragment::End),
                _ => StreamedState::Incomplete,
            };

            let t = start.elapsed().as_millis() as usize;
            let delta_t = t - last_t;
            last_t = t;

            if let StreamedState::Complete(swipe) | StreamedState::Standalone(swipe) = stream {
                run_response = f(&mut frame, Some(&swipe), delta_t);
            }
            if let Err(err) = &run_response {
                fb.shutdown();
                eprintln!("Error occured in user run loop: {}", err);
                std::process::exit(0);
            }

            if let Ok(RunResponse::Draw) = run_response {
                fb.write_frame(&frame.pixels);
            } else if let Ok(RunResponse::Exit) = run_response {
                fb.shutdown();
                std::process::exit(0);
            }
        })
    }
}
