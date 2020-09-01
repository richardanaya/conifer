use conifer::prelude::*;

use env_logger::init;
use log::{debug, info, warn};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting");
    let mut d = Config::auto().unwrap();
    let white = color(255, 255, 255);
    d.run(move |canvas, event| {
        debug!("Enter callback");
        if let Event::Swipe(swipe) = event {
            debug!("New swipe");
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            debug!("{:?}", swipe.drag());
            if let Some(Gesture::Drag(point0, point1)) = swipe.drag() {
                debug!("Draw line");
                canvas.plot_line(point0, point1, white);
            }
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
