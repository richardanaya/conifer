use conifer::{Config, TimevalSize};

fn main() {
    let mut rpi4 = Config::new("/dev/input/event0", "/dev/fb0", 800., 480., TimevalSize::B8);

    rpi4.run(|frame, pointer, delta_time| {
        if pointer.x > 750 {
            // exit if we touch right of the screen
            return true;
        }
        if pointer.is_down {
            // draw a pixel where our touch is
            frame.set_pixel(pointer.x, pointer.y, 255, 255, 255);
        }
        false
    })
}
