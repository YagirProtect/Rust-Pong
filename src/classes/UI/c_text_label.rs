use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_input::Input;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::Collision;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;
use ::font8x8::{UnicodeFonts, BASIC_FONTS};

pub struct TextLabel{
    text: String,
    x : u32,
    y : u32,
    color: u32,
    scale: u16
}

impl TextLabel{
    pub fn new(text: String, x: u32, y: u32, color: Color, scale: u16) -> TextLabel {
        TextLabel { text, x, y, color: color.to_8bit(), scale }
    }

    pub fn update_text(&mut self, score: u32) {
        self.text = format!("{}", score);
    }
}

impl Collision for TextLabel {}
impl Updatable for TextLabel {
    fn has_update(&self) -> bool {false}

    fn update(&mut self, deltaTime: f32, input: &Input, worldContext: &mut WorldContext, audio: &mut AudioContext) {()}
}

impl Drawable for TextLabel{
    fn is_can_draw(&self) -> bool { true }

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {

        let w = c.Width() as i32;
        let h = c.Height() as i32;




        let scale = self.scale.max(1) as i32;
        let color = self.color;

        let char_w = 8 * scale;
        let char_h = 8 * scale;
        let spacing = scale;

        let mut x = self.x as i32;
        let mut y = self.y as i32;

        for ch in self.text.chars() {

            if ch == '\n' { //new line
                x = self.x as i32;
                y += char_h + spacing;
                continue;
            }



            let glyph = match BASIC_FONTS.get(ch) {
                Some(font) => font,
                None => BASIC_FONTS.get('?').unwrap(),
            };

            {//Glyph draw

                for (row, row_bits) in glyph.iter().enumerate() {
                    for col in 0..8 {
                        if (row_bits & (1u8 << col)) == 0 { continue; }

                        let px = x + (col as i32) * scale;
                        let py = y + (row as i32) * scale;

                        // rectangle scale*scale
                        for sy in 0..scale {
                            let yy = py + sy;
                            if yy < 0 || yy >= h { continue; }
                            let row_start = (yy as usize) * (w as usize);

                            for sx in 0..scale {
                                let xx = px + sx;
                                if xx < 0 || xx >= w { continue; }
                                buffer[row_start + (xx as usize)] = color;
                            }
                        }
                    }
                }
            }
            x += char_w + spacing;
        }
    }
}
