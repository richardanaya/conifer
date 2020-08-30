use crate::canvas::Canvas;
use crate::framebuffer::Framebuffer;
use std::error::Error;
use std::path::Path;
use std::time::Instant;

use crate::input::event_input::EventInput;
use crate::input::InputEvent;
use crate::point::*;
use crate::streamed_data::*;
use crate::swipe::*;
use std::sync::Arc;
use std::sync::Mutex;

pub enum RunResponse {
    Exit,
    NothingChanged,
    Draw,
}

#[derive(Debug)]
pub struct Config {
    framebuffer: Arc<Mutex<Framebuffer>>,
    input_device: Arc<Mutex<EventInput>>,
}

pub enum RunEvent {
    Startup,
    Time,
    Swipe(Swipe),
}

impl Config {
    pub fn new<P: AsRef<Path>>(
        path_to_input_device: P,
        path_to_framebuffer: P,
        input_min_width: f32,
        input_min_height: f32,
        input_max_width: f32,
        input_max_height: f32,
    ) -> Result<Self, Box<dyn Error>> {
        let framebuffer = Framebuffer::new(path_to_framebuffer)?;
        let input_device = EventInput::new(
            &path_to_input_device,
            input_min_width,
            input_min_height,
            input_max_width,
            input_max_height,
        )?;

        Ok(Config {
            framebuffer: Arc::new(Mutex::new(framebuffer)),
            input_device: Arc::new(Mutex::new(input_device)),
        })
    }

    pub fn auto() -> Result<Self, Box<dyn Error>> {
        let framebuffer = Framebuffer::auto()?;
        let input_device = EventInput::auto()?;
        Ok(Config {
            input_device: Arc::new(Mutex::new(input_device)),
            framebuffer: Arc::new(Mutex::new(framebuffer)),
        })
    }

    pub fn screen_width(&self) -> usize {
        let fb = self.framebuffer.lock().unwrap();
        return fb.width();
    }

    pub fn screen_height(&self) -> usize {
        let fb = self.framebuffer.lock().unwrap();
        return fb.height();
    }

    pub fn run(
        &mut self,
        mut f: impl FnMut(&mut Canvas, RunEvent, usize) -> Result<RunResponse, Box<dyn Error>> + 'static,
    ) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        let mut last_t = 0 as usize;

        let mut fb = self.framebuffer.lock().unwrap();

        let w = fb.width();
        let h = fb.height();
        let line_length = fb.line_length();
        let mut canvas = Canvas {
            width: w,
            height: h,
            line_length,
            bytespp: fb.bytes_per_pixel(),
            pixels: vec![0u8; line_length * h],
        };

        if let Err(err) = fb.setup() {
            // try to shut down because because being stuck in graphics mode is really bad
            fb.shutdown()?;
            eprintln!("Error occured in user run loop: {}", err);
            std::process::exit(0);
        }

        let t = start.elapsed().as_millis() as usize;
        let delta_t = t - last_t;
        last_t = t;

        match f(&mut canvas, RunEvent::Startup, delta_t) {
            Ok(RunResponse::Draw) => {
                fb.write_frame(&canvas.pixels);
            }
            Ok(RunResponse::Exit) => {
                fb.shutdown()?;
                std::process::exit(0);
            }
            Ok(RunResponse::NothingChanged) => {
                //Question: should we show something if the first run doesn't say to paint?
            }
            Err(err) => {
                fb.shutdown()?;
                eprintln!("Error occured in user run loop: {}", err);
                std::process::exit(0);
            }
        }

        let mut swipe_mem = StreamedSwipe {
            swipe: None,
            streamed_point: StreamedPoint::Nothing,
        };

        let (timer_tx, timer_rx) = flume::unbounded();

        std::thread::spawn(move || loop {
            timer_tx.send(42);
            std::thread::sleep(std::time::Duration::from_secs(1));
        });

        let (event_tx, event_rx) = flume::unbounded();

        let id = self.input_device.clone();
        std::thread::spawn(move || {
            let mut i = id.lock().unwrap();
            i.on_event(move |ev| {
                let stream = match ev {
                    InputEvent::PartialX(x, time) => {
                        swipe_mem.update(SwipeFragment::PointFragment(PointFragment::X(time, x)))
                    }
                    InputEvent::PartialY(y, time) => swipe_mem.update(
                        SwipeFragment::PointFragment(PointFragment::Y(time, y as isize)),
                    ),
                    InputEvent::ButtonDown(_) => swipe_mem.update(SwipeFragment::End),
                    _ => StreamedState::Incomplete,
                };
                match stream {
                    StreamedState::Complete(swipe) | StreamedState::Standalone(swipe) => {
                        event_tx.send(swipe);
                    }
                    StreamedState::Incomplete => {}
                }
                Ok(())
            });
        });

        loop {
            let t = start.elapsed().as_millis() as usize;
            let delta_t = t - last_t;
            last_t = t;

            let time = match timer_rx.try_recv() {
                Ok(t) => Some(t),
                Err(flume::TryRecvError::Empty) => None,
                Err(flume::TryRecvError::Disconnected) => panic!("why would timer disconnect!"),
            };

            let swipe = match event_rx.try_recv() {
                Ok(s) => Some(s),
                Err(flume::TryRecvError::Empty) => None,
                Err(flume::TryRecvError::Disconnected) => panic!("why would events disconnect!"),
            };

            if let Some(s) = swipe {
                match f(&mut canvas, RunEvent::Swipe(s.clone()), delta_t) {
                    Ok(RunResponse::Draw) => {
                        fb.write_frame(&canvas.pixels);
                    }
                    Ok(RunResponse::Exit) => {
                        fb.shutdown()?;
                        std::process::exit(0);
                    }
                    Ok(RunResponse::NothingChanged) => {}
                    Err(err) => {
                        fb.shutdown()?;
                        eprintln!("Error occured in user run loop: {}", err);
                        std::process::exit(0);
                    }
                }
            }

            if let Some(t) = time {
                match f(&mut canvas, RunEvent::Time, delta_t) {
                    Ok(RunResponse::Draw) => {
                        fb.write_frame(&canvas.pixels);
                    }
                    Ok(RunResponse::Exit) => {
                        fb.shutdown()?;
                        std::process::exit(0);
                    }
                    Ok(RunResponse::NothingChanged) => {}
                    Err(err) => {
                        fb.shutdown()?;
                        eprintln!("Error occured in user run loop: {}", err);
                        std::process::exit(0);
                    }
                }
            }
        }
    }
}
