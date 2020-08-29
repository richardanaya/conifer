# Conifer

```
# Make sure your user is a part of video and input group
sudo addusr video richard 
sudo addusr input richard
# Logout and login
# verify the touch device in code matches your phone
cat /proc/bus/input/devices
```

A simple frame buffer game engine for PinePhone, Raspberry Pi, and other .

* Frame buffer ( no X11 required! )
* Touch screen

```rust
use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    Config::auto()?.run(|frame, swipe, _delta_time| {
        for y in 0..frame.height {
            for x in 0..frame.width {
                if swipe.is_some() {
                    return Ok(RunResponse::Exit);
                }
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / frame.width as f32) * 255.0) as u8,
                    ((y as f32 / frame.height as f32) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        Ok(RunResponse::Draw)
    });
    Ok(())
}
```
