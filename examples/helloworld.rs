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
