# Conifer

A simple frame buffer game engine for PinePhone.

* Frame buffer ( no X11 required! )
* Touch screen

```rust
fn main() {
    let mut t = 0;
    conifer::run(|frame, _pointer, delta_time| {
        for y in 0..frame.height {
            for x in 0..frame.width {
                t += delta_time;
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / 255.0) * 255.0) as u8,
                    ((y as f32 / 255.0) * 255.0) as u8,
                    (f32::sin(t as f32) * 255.0) as u8,
                );
            }
        }
        let should_exit = pointer.is_down;
        should_exit
    })
}
```