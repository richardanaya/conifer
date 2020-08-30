use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;
    let c = Config::auto()?;
    let green = Canvas::from(c.framebuffer.width, c.framebuffer.height, 255, 0, 0);
    c.run(move |canvas, swipe, _delta_time| {
        if let Some(_) = swipe {
            return Ok(RunResponse::Exit);
        }
        canvas.copy_from_canvas(&green);
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
