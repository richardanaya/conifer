use conifer::prelude::*;
use conifer::point::*;
use conifer::canvas::*;

use env_logger;
use log::debug;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut c = Config::auto()?;

    let img_pine = load_image("examples/images/pine.png")?;

    // create a blit mask from any alpha > 0
    let img_pine_blit_map = BlitMap::from_canvas_with_alpha(&img_pine);

    let background = Canvas::from_color(c.screen_width(), c.screen_height(), 0,100,0);

    let (mut x0,mut y0) = (0,0);

    c.run(move |canvas, event| {
        debug!("Enter callback");
        debug!("x0:{},y0:{}", x0,y0);

        canvas.copy_from_canvas(&background);

        canvas.blit_canvas(
            &img_pine,
            x0,y0,
            &img_pine_blit_map,
        )?;

        if let Event::Swipe(swipe) = event {
            debug!("Swipe {:?}", swipe);
            if let Some(Gesture::Tap(Point{x,y,time})) = swipe.tap(10) {
                debug!("Tap x:{},y:{}",x,y);
                if x0 <= x && x <= x0 + img_pine.width as isize && y0 <= y && y <= y0 + img_pine.height as isize {
                    let b_index = (y-y0) as usize * img_pine.width + (x-x0) as usize;
                    if img_pine_blit_map.map[(y-y0) as usize * img_pine.width + (x-x0) as usize] {
                        x0 = (random() * canvas.width as f32) as isize - img_pine.width as isize / 2;
                        y0 = (random() * canvas.height as f32) as isize - img_pine.height as isize / 2;
                    }
                }
            } else if swipe.finished {
                debug!("Exit canvas");
                return Ok(RunResponse::Exit)
            }
        }
        debug!("Draw canvas");
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}

    
