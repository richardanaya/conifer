use conifer::prelude::*;

use env_logger::init;
use log::{debug, info, warn};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting");
    let mut d = Config::auto().unwrap();

    let white = rgb(255, 255, 255);
    d.run(move |canvas, event| {
        debug!("Enter callback");
        if let Event::Swipe(swipe) = event {
            debug!("New swipe");
            if swipe.points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            debug!("{:?}", swipe.tap(20));
            if let Some(Gesture::Tap(point)) = swipe.tap(20) {
                debug!("Draw tap");
                canvas.set_pixel(point.x as usize, point.y as usize, white);
            }
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
