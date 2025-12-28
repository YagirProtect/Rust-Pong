use crate::classes::c_color::Color;
use crate::classes::c_config::Config;

pub struct Canvas {
    w: usize,
    h: usize,
    background_color: u32
}

impl Canvas {
    pub fn Width(&self) -> u32 { self.w as u32 }
    pub fn Height(&self) -> u32 { self.h as u32 }
}

impl Canvas{

    pub fn new(config: &Config) -> Canvas {
        let mut color = Color::default();
        color.load(config.BackgroundColor().as_str());

        Self {
            w: config.Width() as usize,
            h: config.Height() as usize,
            background_color: color.to_8bit()
        }
    }

    pub fn clear(&mut self, buffer: &mut Vec<u32>){
        buffer.fill(self.background_color);
    }
}
