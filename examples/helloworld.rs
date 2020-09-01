use conifer::prelude::*;

fn main() {
    run(|canvas, event| {
        // if the user swiped, exit
        if let Event::Swipe(s) = event {
            if s.finished {
                return Ok(RunResponse::Exit);
            }
            // draw something to framebuffer pixels
            for p in s.points {
                canvas.set_pixel(p.x as usize, p.y as usize, color(255, 255, 255));
            }
        }
        // let conifer know we want to push framebuffer pixels to screen
        Ok(RunResponse::Draw)
    })
    .expect("something went wrong")
}
