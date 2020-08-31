use std::error::Error;

pub mod canvas;
pub mod config;
pub mod framebuffer;
pub mod gesture;
pub mod input;
pub mod point;
pub mod prelude;
pub mod streamed_data;
pub mod swipe;
pub mod util;

pub fn run(
    f: impl FnMut(&mut canvas::Canvas, config::Event) -> Result<config::RunResponse, Box<dyn Error>>
        + 'static,
) -> Result<(), Box<dyn Error>> {
    config::Config::auto()?.run(f)
}
