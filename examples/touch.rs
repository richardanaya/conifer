use conifer::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut d = Config::auto().unwrap();

    let white = color(255, 255, 255);
    let red = color(0, 0, 255);
    d.run(move |canvas, event| {
        if let Event::Swipe(swipe) = event {
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            // draw a swipe red when it's finished, white when ongoing
            for p in points.iter() {
                if swipe.finished {
                    canvas.set_pixel(p.x as usize, p.y as usize, white);
                } else {
                    canvas.set_pixel(p.x as usize, p.y as usize, red);
                }
            }
        }
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
