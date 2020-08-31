use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::auto()?;
    let h = config.screen_height();
    config.run(move |canvas, event| {
        // if the user swiped, exit
        if let Event::Swipe(s) = event {
            // draw something to framebuffer pixels
            for p in s.points {
                // if we touch the bottom, exit
                if p.y > (h as f32 * 0.9) as isize {
                    return Ok(RunResponse::Exit);
                }
                canvas.set_pixel(p.x as usize, p.y as usize, 255, 255, 255);
            }
            return Ok(RunResponse::Exit);
        }
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
