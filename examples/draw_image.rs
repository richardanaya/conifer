use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;
    Config::auto()?.run(|frame, swipe, _delta_time| {
        if let Some(s) = swipe {
            return Ok(RunResponse::Exit);
        }
        // if the user swiped, exit
        for _ in 0..100 {
            frame.draw_frame(
                &img_pine,
                (random() * frame.width as f32) as isize - img_pine.width as isize / 2,
                (random() * frame.width as f32) as isize - img_pine.height as isize / 2,
            )?;
        }
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    });
    Ok(())
}
