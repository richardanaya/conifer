use crate::canvas::Canvas;
use image::GenericImageView;
use rand::Rng;
use std::error::Error;
use std::path::Path;

pub fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub fn random_n<T>(n: T) -> T
where
    T: Into<f32> + From<f32>,
{
    f32::floor(n.into() * random()).into()
}

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Canvas, Box<dyn Error>> {
    let img = image::open(path)?;
    let d = img.dimensions();
    let mut pixels = vec![];
    for r in img.pixels() {
        let p = r.2;
        // this ordering works for my framebuffer, does it work for all?
        pixels.push((p[3] as u32) << 24 | (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32);
    }
    Ok(Canvas::new(d.0 as usize, d.1 as usize, &pixels))
}

pub fn color(r: u8, g: u8, b: u8) -> u32 {
    255 << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
