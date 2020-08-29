use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    Config::auto()?.run(|frame, swipe, _delta_time| {
        // if the user swiped, exit
        if swipe.is_some() {
            return Ok(RunResponse::Exit);
        }
        // draw something to framebuffer pixels
        for y in 0..frame.height {
            for x in 0..frame.width {
                frame.set_pixel(
                    x,
                    y,
                    ((x as f32 / frame.width as f32) * 255.0) as u8,
                    ((y as f32 / frame.height as f32) * 255.0) as u8,
                    0 as u8,
                );
            }
        }
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    });
    Ok(())
}
