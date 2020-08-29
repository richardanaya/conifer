use conifer::gesture::*;
use conifer::swipe::*;
use conifer::Config;

use env_logger::init;
use log::{debug, info, warn};

fn main() {
    env_logger::init();
    info!("Starting");
    let mut rpi4 = Config::new("/dev/input/event0", "/dev/fb0", 800., 480.);

    rpi4.run(|frame, swipe, delta_time| {
        debug!("Enter callback");
        if let Some(swipe) = swipe {
            debug!("New swipe");
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return true;
            }
            debug!("{:?}", swipe.drag());
            if let Some(Gesture::Drag(point0, point1)) = swipe.drag() {
                debug!("Draw line");
                frame.plotLine(point0, point1);
            }
        }
        false
    })
}
