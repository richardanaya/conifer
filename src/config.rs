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

#[derive(Debug)]
pub enum Event {
    Startup,
    Timer(usize, usize),
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
        mut f: impl FnMut(&mut Canvas, Event) -> Result<RunResponse, Box<dyn Error>> + 'static,
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

        match f(&mut canvas, Event::Startup) {
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
            let cur_time = start.elapsed().as_millis() as usize;
            let delta_t = cur_time - last_t;
            last_t = cur_time;
            timer_tx
                .send(Event::Timer(delta_t, cur_time))
                .expect("something went wrong sending timer");
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
        });

        let (swipe_tx, swipe_rx) = flume::unbounded();

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
                        swipe_tx
                            .send(swipe)
                            .expect("something went wrong when sending swipe");
                    }
                    StreamedState::Incomplete => {}
                }
                Ok(())
            })
            .expect("not sure why listening to event device would fail");
        });

        loop {
            match timer_rx.try_recv() {
                Ok(t) => match f(&mut canvas, t) {
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
                },
                Err(flume::TryRecvError::Empty) => (),
                Err(flume::TryRecvError::Disconnected) => panic!("why would timer disconnect!"),
            };

            match swipe_rx.try_recv() {
                Ok(s) => match f(&mut canvas, Event::Swipe(s.clone())) {
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
                },
                Err(flume::TryRecvError::Empty) => (),
                Err(flume::TryRecvError::Disconnected) => panic!("why would events disconnect!"),
            };
        }
    }
}
