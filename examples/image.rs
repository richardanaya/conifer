use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("images/pine.png")?;
    Config::auto()?.run(|frame, swipe, _delta_time| {
        if swipe.is_some() {
            return Ok(RunResponse::Exit);
        }
        // if the user swiped, exit
        frame.draw_frame(&img_pine,0,0)?;
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    });
    Ok(())
}
