use std::str::FromStr;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
impl Default for Color {
    fn default() -> Self {
        return Color { r: 0, g: 0, b: 0, a: 255 };
    }
}

impl Color {
    
    pub fn new (r:u8, g:u8, b:u8, a:u8) -> Color {
        Color { r, g, b, a }
    }
    
    pub fn set(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.a = a;
        self.r = r;
        self.g = g;
        self.b = b;
    }
    #[inline]
    pub fn to_8bit(&self) -> u32 {
        ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    pub fn from_8bit(&mut self, color: u32) {
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;

        self.set(r, g, b, self.a);
    }

    pub fn to_string(&self) -> String {
        return Self::to_8bit(self).to_string()
    }

    pub fn load(&mut self, data: &str) {
        let value = match u32::from_str(data) {
            Ok(v) => v,
            Err(e) => {
                println!("color parse failed {}", data);
                0
            }
        };

        self.from_8bit(value);
    }
}