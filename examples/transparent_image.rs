use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;

    // create a blit mask from any alpha > 0
    let img_pine_blit_map = BlitMap::from_canvas_with_alpha(&img_pine);

    Config::auto()?.run(move |canvas, event| {
        if let Event::Swipe(_) = event {
            return Ok(RunResponse::Exit);
        }
        // let's draw some images randomly, but this time with a dithered mask
        for _ in 0..10 {
            canvas.blit_canvas(
                &img_pine,
                (random() * canvas.width as f32) as isize - img_pine.width as isize / 2,
                (random() * canvas.height as f32) as isize - img_pine.height as isize / 2,
                &img_pine_blit_map,
            )?;
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
