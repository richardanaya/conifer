use conifer::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;
    let mut c = Config::auto()?;
    let green = Canvas::from_color(c.screen_width(), c.screen_height(), 0, 100, 0);
    c.run(move |canvas, swipe, _delta_time| {
        if let Some(_) = swipe {
            return Ok(RunResponse::Exit);
        }
        canvas.copy_from_canvas(&green);
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
