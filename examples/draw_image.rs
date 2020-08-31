use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;
    Config::auto()?.run(move |canvas, event| {
        if let Event::Swipe(s) = event {
            if s.finished {
                return Ok(RunResponse::Exit);
            }
        }
        // let's draw some images randomly
        for _ in 0..100 {
            canvas.draw_canvas(
                &img_pine,
                (random() * canvas.width as f32) as isize - img_pine.width as isize / 2,
                (random() * canvas.height as f32) as isize - img_pine.height as isize / 2,
            )?;
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
