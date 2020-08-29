use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Framebuffer {
    fb: framebuffer::Framebuffer,
}

impl Framebuffer {
    pub fn new<P: AsRef<Path>>(path_to_framebuffer: P) -> Result<Self, Box<dyn Error>> {
        Ok(Framebuffer {
            fb: framebuffer::Framebuffer::new(path_to_framebuffer)?,
        })
    }

    pub fn auto() -> Result<Self, Box<dyn Error>> {
        Ok(Framebuffer {
            fb: framebuffer::Framebuffer::new("/dev/fb0")?,
        })
    }

    pub fn setup(&self) {
        framebuffer::Framebuffer::set_kd_mode(framebuffer::KdMode::Graphics).unwrap();
    }

    pub fn shutdown(&self) {
        framebuffer::Framebuffer::set_kd_mode(framebuffer::KdMode::Text).unwrap();
    }

    pub fn width(&self) -> usize {
        return self.fb.var_screen_info.xres as usize;
    }

    pub fn height(&self) -> usize {
        return self.fb.var_screen_info.yres as usize;
    }

    pub fn line_length(&self) -> usize {
        return self.fb.fix_screen_info.line_length as usize;
    }

    pub fn bytes_per_pixel(&self) -> usize {
        return (self.fb.var_screen_info.bits_per_pixel / 8) as usize;
    }

    pub fn write_frame(&mut self, pixels: &[u8]) {
        self.fb.write_frame(pixels);
    }
}
