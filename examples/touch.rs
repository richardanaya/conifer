fn main() {
    conifer::run(|frame, pointer, delta_time| {
        if pointer.y > 1350 {
            // exit if we touch bottom of screen
            return true;
        }
        if pointer.is_down {
            // draw a pixel where our touch is
            frame.set_pixel(pointer.x, pointer.y, 255, 255, 255);
        }
        false
    })
}
