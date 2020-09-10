use conifer::prelude::*;
use conifer::point::*;
use conifer::canvas::*;
use conifer::util::*;
use std::path::Path;

use env_logger;
use log::debug;


trait InteractiveObject<T> {
    /// Whether given coordinate are in the area that is meant to react.
    fn contact(&self, x: usize, y: usize) -> bool;
    /// Trigger the event if contact is made.
    fn trigger(&mut self, x: usize, y:usize) -> Option<T>;
    /// Render the object.
    fn render(&self, canvas: &mut Canvas) -> Result<(), &'static str>;
}

enum Directions {
    Left,
    Right,
    Top,
    Bottom,
}

struct ClickableImage<T> {
    origin: (usize,usize),
    image: Canvas,
    bmap: BlitMap,
    callback: Box<dyn FnMut() -> T>,
}

impl<T> ClickableImage<T> {
    fn new<P: AsRef<Path>>(path: P, origin: (usize, usize), callback: impl FnMut() -> T + 'static) -> Result<Self, Box<dyn Error>> {
        let image = load_image(path)?;
        let bmap = BlitMap::from_canvas_with_bg_color(&image, 255,0,255);
        Ok(ClickableImage {
            origin,
            image,
            bmap,
            callback: Box::new(callback),
        })
    }
}

impl<T> InteractiveObject<T> for ClickableImage<T> {
    fn contact(&self, x: usize, y: usize) -> bool {
        if coord_in_area(x,y,0,0,self.image.width, self.image.height) {
            if let Ok(b) = self.bmap.get_bool(x,y) {
                b
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn trigger(&mut self, x: usize, y:usize) -> Option<T> {
        if self.contact(x,y) {
            Some((self.callback)())
        } else {
            None
        }
    }

    fn render(&self, canvas: &mut Canvas) -> Result<(), &'static str> {
        canvas.blit_canvas(&self.image, self.origin.0 as isize, self.origin.1 as isize, &self.bmap)
    }
}




fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut c = Config::auto()?;

    let mut dpad_left = ClickableImage::new("examples/images/d-pad-left.png", (50,50),|| Directions::Left)?;
    let mut dpad_right = ClickableImage::new("examples/images/d-pad-right.png", (50,50),|| Directions::Right)?;
    let mut dpad_top = ClickableImage::new("examples/images/d-pad-top.png", (50,50),|| Directions::Top)?;
    let mut dpad_bottom = ClickableImage::new("examples/images/d-pad-bottom.png", (50,50),|| Directions::Bottom)?;

    let img_pine = load_image("examples/images/pine.png")?;

    // create a blit mask from any alpha > 0
    let img_pine_blit_map = BlitMap::from_canvas_with_alpha(&img_pine);

    let background = Canvas::from_color(c.screen_width(), c.screen_height(), 0,100,0);

    let (mut x0,mut y0) = (0,0);

    c.run(true, move |canvas, event| {
        debug!("Enter callback");
        debug!("x0:{},y0:{}", x0,y0);

        canvas.copy_from_canvas(&background);

        canvas.blit_canvas(
            &img_pine,
            x0,y0,
            &img_pine_blit_map,
        )?;

        dpad_left.render(canvas)?;
        dpad_right.render(canvas)?;
        dpad_top.render(canvas)?;
        dpad_bottom.render(canvas)?;

        if let Event::Swipe(swipe) = event {
            debug!("Swipe {:?}", swipe);
            if let Some(Gesture::Tap(Point{x,y,time})) = swipe.tap(10) {
                debug!("Tap x:{},y:{}",x,y);
                if dpad_left.contact(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    debug!("Contact on left pad");
                }
                if dpad_right.contact(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    debug!("Contact on right pad");
                }
                if dpad_top.contact(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    debug!("Contact on top pad");
                }
                if dpad_bottom.contact(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    debug!("Contact on bottom pad");
                }
                if let Some(Directions::Left) = dpad_left.trigger(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    x0 = isize::max(0,x0-10);
                    debug!("x0:{},y0:{}",x0,y0);
                }
                if let Some(Directions::Right) = dpad_right.trigger(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    x0 = isize::min(x0+10,canvas.width as isize - img_pine.width as isize);
                    debug!("x0:{},y0:{}",x0,y0);
                }
                if let Some(Directions::Top) = dpad_top.trigger(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    y0 = isize::max(0,y0-10);
                    debug!("x0:{},y0:{}",x0,y0);
                }
                if let Some(Directions::Bottom) = dpad_bottom.trigger(isize::max(0,x-50) as usize,isize::max(0,y-50) as usize) {
                    y0 = isize::min(y0+10, canvas.height as isize - img_pine.height as isize);
                    debug!("x0:{},y0:{}",x0,y0);
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

    
