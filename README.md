# :evergreen_tree: Conifer

<a href="https://docs.rs/conifer"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

A simple canvas buffer game engine for PinePhone, Raspberry Pi, and other devices with touch screens.

- [x] make games without X11!
- [x] auto detect virtual terminal framebuffer
- [x] auto detect touch screen input
- [x] works on pinephone, raspbery pi, desktop
- [x] image support
- [ ] layers
- [ ] text drawing
- [ ] sprites
- [ ] sound
- [ ] web assembly support

```toml
[dependencies]
conifer = "0.0"
```

 Make sure your user is a part of `video` and `input` group

```bash
sudo addusr video richard 
sudo addusr input richard
# Logout and login
```

## Hello World

```rust
use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    Config::auto()?.run(|canvas, swipe, _delta_time| {
        // if the user swiped, exit
        if swipe.is_some() {
            return Ok(RunResponse::Exit);
        }
        // draw something to framebuffer pixels
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                canvas.set_pixel(
                    x,
                    y,
                    ((x as f32 / canvas.width as f32) * 255.0) as u8,
                    ((y as f32 / canvas.height as f32) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    });
    Ok(())
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `conifer` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
