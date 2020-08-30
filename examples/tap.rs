use conifer::prelude::*;

use env_logger::init;
use log::{debug, info, warn};

fn main() {
    env_logger::init();
    info!("Starting");
    let mut d = Config::auto().unwrap();

    d.run(|canvas, swipe, delta_time| {
        debug!("Enter callback");
        if let Some(swipe) = swipe {
            debug!("New swipe");
            if swipe.points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            debug!("{:?}", swipe.tap(20));
            if let Some(Gesture::Tap(point)) = swipe.tap(20) {
                debug!("Draw tap");
                canvas.set_pixel(point.x as usize, point.y as usize, 255, 255, 255);
            }
        }
        Ok(RunResponse::Draw)
    })
}
