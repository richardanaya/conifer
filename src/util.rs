use rand::Rng;
use crate::frame::Frame;
use std::path::Path;
use image::{Rgb,GenericImageView};
use std::error::Error;

pub fn random() -> f32 {
    rand::thread_rng().gen::<f32>()
}


pub fn load_image<P: AsRef<Path>>(path:P) -> Result<Frame,Box<dyn Error>> {
    let img = image::open(path).unwrap();
    let d = img.dimensions();
    let mut pixels = vec![];
    for r in img.pixels() {
        let p = r.2;
        pixels.push(p[0]);
        pixels.push(p[1]);
        pixels.push(p[2]);
        //TODO figure out alpha?
        //pixels.push(p[3]);
    }
    Frame::new(d.0 as usize,d.1 as usize,&pixels)
}