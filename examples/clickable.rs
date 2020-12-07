use conifer::canvas::*;
use conifer::point::*;
use conifer::prelude::*;
use conifer::util::*;
use std::path::Path;

use env_logger;
use log::debug;

trait Render {
    fn render(&self, canvas: &mut Canvas) -> Result<(), &'static str>;
}

trait InteractiveObject<T> {
    /// Test whether coordinates `x` `y` activate the object.
    fn poke(&self, x: usize, y: usize) -> bool;
    /// Compute interaction when it makes contact.
    fn trigger(&mut self, x: usize, y: usize) -> Option<T>;
}

enum Directions {
    Left,
    Right,
    Top,
    Bottom,
}

struct ClickableImage<T> {
    origin: (usize, usize),
    image: Canvas,
    bmap: BlitMap,
    callback: Box<dyn FnMut() -> T>,
}

impl<T> ClickableImage<T> {
    fn new<P: AsRef<Path>>(
        path: P,
        origin: (usize, usize),
        callback: impl FnMut() -> T + 'static,
    ) -> Result<Self, Box<dyn Error>> {
        let image = load_image(path)?;
        let bmap = BlitMap::from_canvas_with_bg_color(&image, 255, 0, 255);
        Ok(ClickableImage {
            origin,
            image,
            bmap,
            callback: Box::new(callback),
        })
    }
}

impl<T> InteractiveObject<T> for ClickableImage<T> {
    fn poke(&self, x: usize, y: usize) -> bool {
        if coord_in_area(x, y, 0, 0, self.image.width, self.image.height) {
            if let Ok(b) = self.bmap.get_bool(x, y) {
                b
            } else {
                false
            }
        } else {
            false
        }
    }

    fn trigger(&mut self, x: usize, y: usize) -> Option<T> {
        if self.poke(x, y) {
            Some((self.callback)())
        } else {
            None
        }
    }
}

/// Non empty.
struct ComposedClickableImage<T> {
    components: Vec<ClickableImage<T>>,
}

impl<T> ComposedClickableImage<T> {
    fn new(image: ClickableImage<T>) -> Self {
        ComposedClickableImage {
            components: vec![image],
        }
    }

    fn push(&mut self, image: ClickableImage<T>) {
        self.components.push(image);
    }
}

impl<T> InteractiveObject<T> for ComposedClickableImage<T> {
    fn poke(&self, x: usize, y: usize) -> bool {
        self.components.iter().any(|c| c.poke(x, y))
    }

    fn trigger(&mut self, x: usize, y: usize) -> Option<T> {
        self.components
            .iter_mut()
            .fold(None, |acc, c| acc.or_else(|| c.trigger(x, y)))
    }
}

impl<T> Render for ClickableImage<T> {
    fn render(&self, canvas: &mut Canvas) -> Result<(), &'static str> {
        canvas.blit_canvas(
            &self.image,
            self.origin.0 as isize,
            self.origin.1 as isize,
            &self.bmap,
        )
    }
}

impl<T> Render for ComposedClickableImage<T> {
    fn render(&self, canvas: &mut Canvas) -> Result<(), &'static str> {
        self.components
            .iter()
            .fold(Ok(()), |acc, c| acc.and(c.render(canvas)))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut c = Config::auto()?;

    let mut dpad_left = ClickableImage::new("examples/images/d-pad-left.png", (50, 50), || {
        Directions::Left
    })?;
    let mut dpad_right = ClickableImage::new("examples/images/d-pad-right.png", (50, 50), || {
        Directions::Right
    })?;
    let mut dpad_top = ClickableImage::new("examples/images/d-pad-top.png", (50, 50), || {
        Directions::Top
    })?;
    let mut dpad_bottom =
        ClickableImage::new("examples/images/d-pad-bottom.png", (50, 50), || {
            Directions::Bottom
        })?;

    let mut dpad = ComposedClickableImage::new(dpad_left);
    dpad.push(dpad_right);
    dpad.push(dpad_top);
    dpad.push(dpad_bottom);

    let img_pine = load_image("examples/images/pine.png")?;

    // create a blit mask from any alpha > 0
    let img_pine_blit_map = BlitMap::from_canvas_with_alpha(&img_pine);

    let background = Canvas::from_color(c.screen_width(), c.screen_height(), 0, 100, 0);

    let (mut x0, mut y0) = (0, 0);

    c.run(move |canvas, event| {
        canvas.copy_from_canvas(&background);

        canvas.blit_canvas(&img_pine, x0, y0, &img_pine_blit_map)?;

        dpad.render(canvas)?;

        if let Event::Timer(_, _) = event {
            return Ok(RunResponse::Draw);
        }

        debug!("Non-timer event");
        debug!("x0:{},y0:{}", x0, y0);

        if let Event::Swipe(swipe) = event {
            debug!("Swipe {:?}", swipe);
            if let Some(Gesture::Tap(Point { x, y, time })) = swipe.tap(10) {
                debug!("Tap x:{},y:{}", x, y);
                let poke = dpad.poke(
                    isize::max(0, x - 50) as usize,
                    isize::max(0, y - 50) as usize,
                );
                debug!("Poke? {}", poke);
                if let Some(dir) = dpad.trigger(
                    isize::max(0, x - 50) as usize,
                    isize::max(0, y - 50) as usize,
                ) {
                    match dir {
                        Directions::Left => {
                            x0 = isize::max(0, x0 - 10);
                            debug!("x0:{},y0:{}", x0, y0);
                        }
                        Directions::Right => {
                            x0 = isize::min(
                                x0 + 10,
                                canvas.width as isize - img_pine.width as isize,
                            );
                            debug!("x0:{},y0:{}", x0, y0);
                        }
                        Directions::Top => {
                            y0 = isize::max(0, y0 - 10);
                            debug!("x0:{},y0:{}", x0, y0);
                        }
                        Directions::Bottom => {
                            y0 = isize::min(
                                y0 + 10,
                                canvas.height as isize - img_pine.height as isize,
                            );
                            debug!("x0:{},y0:{}", x0, y0);
                        }
                    }
                }
            } else if swipe.finished {
                debug!("Exit canvas");
                return Ok(RunResponse::Exit);
            }
        }
        debug!("Draw canvas");
        Ok(RunResponse::Draw)
    })?;
    Ok(())
}
