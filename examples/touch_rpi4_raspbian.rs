use conifer::Config;

fn main() {
    let mut rpi4 = Config::new("/dev/input/event0", "/dev/fb0", 800., 480.);

    rpi4.run(|frame, swipe, delta_time| {
        if let Some(swipe) = swipe {
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return true;
            }
            // draw a swipe red when it's finished, white when ongoing
            for p in points.iter() {
                if swipe.finished {
                    frame.set_pixel(p.x, p.y, 255, 255, 255);
                } else {
                    frame.set_pixel(p.x, p.y, 0, 0, 255);
                }
            }
        }
        false
    })
}
