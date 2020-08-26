use conifer::Config;
use evdev::Device;

fn main() {
    let mut rpi4 = Config::new("/dev/input/event0", "/dev/fb0", 800., 480.);

    rpi4.run(|frame, point, delta_time| {
        if let Some(p) = point {
            if p.x > 750 {
                // exit if we touch right of the screen
                return true;
            }
            // draw a pixel when a point is received
            frame.set_pixel(p.x, p.y, 255, 255, 255);
        }
        false
    })
}
