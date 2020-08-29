use conifer::prelude::*;

fn main() {
    let mut d = Config::auto().unwrap();

    d.run(|frame, swipe, delta_time| {
        if let Some(swipe) = swipe {
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            // draw a swipe red when it's finished, white when ongoing
            for p in points.iter() {
                if swipe.finished {
                    frame.set_pixel(p.x as usize, p.y as usize, 255, 255, 255);
                } else {
                    frame.set_pixel(p.x as usize, p.y as usize, 0, 0, 255);
                }
            }
        }
        Ok(RunResponse::Draw)
    })
}
