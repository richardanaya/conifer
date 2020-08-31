use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut d = Config::auto().unwrap();

    d.run(|canvas, event| {
        if let Event::Swipe(swipe) = event {
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            // draw a swipe red when it's finished, white when ongoing
            for p in points.iter() {
                if swipe.finished {
                    canvas.set_pixel(p.x as usize, p.y as usize, 255, 255, 255);
                } else {
                    canvas.set_pixel(p.x as usize, p.y as usize, 0, 0, 255);
                }
            }
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
