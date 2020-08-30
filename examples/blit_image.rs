use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let img_pine = load_image("examples/images/pine.png")?;

    // create a dithered blit mask
    let mut dither = img_pine.create_blitmap();
    for x in 0..dither.len() {
        if (x+1)%2 == 0 {
            dither[x] = false;
        }
    }
    
    Config::auto()?.run(move |canvas, swipe, _delta_time| {
        if swipe.is_some() {
            return Ok(RunResponse::Exit);
        }
        // let's draw some images randomly, but this time with a dithered mask
        for _ in 0..10 {
            canvas.blit_canvas(
                &img_pine,
                (random() * canvas.width as f32) as isize - img_pine.width as isize / 2,
                (random() * canvas.height as f32) as isize - img_pine.height as isize / 2,
                &dither,
            )?;
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
