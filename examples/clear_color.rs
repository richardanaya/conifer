use conifer::prelude::*;
fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;
    let mut c = Config::auto()?;
    let green = Canvas::from_color(c.screen_width(), c.screen_height(), 0, 100, 0);
    c.run(move |canvas, event, _delta_time| {
        if let RunEvent::Swipe(s) = event {
            if s.finished {
                return Ok(RunResponse::Exit);
            }
        }
        canvas.copy_from_canvas(&green);
        canvas.draw_canvas(
            &img_pine,
            (random() * canvas.width as f32) as isize - img_pine.width as isize / 2,
            (random() * canvas.height as f32) as isize - img_pine.height as isize / 2,
        )?;
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
