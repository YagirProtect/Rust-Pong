use vek::{Lerp, Vec2};
use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_input::Input;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::Collision;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_entity::Entity;
use crate::classes::t_updatable::Updatable;
use crate::classes::UI::c_text_label::TextLabel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonName{
    None,
    Play = 1,
    Exit = 2,
    ToMenu = 3,
    Credits = 4
}

pub struct ButtonLabel{
    text_label: TextLabel,
    rectangle: Rectangle,
    button_name: ButtonName,

    is_inside: bool,
    is_down: bool,
    call: bool,


    normal_color: u32,
    hover_color: u32,

    animation_color: f32
}


impl Collision for ButtonLabel{}
impl Updatable for ButtonLabel{
    fn has_update(&self) -> bool {true}

    fn update(&mut self, deltaTime: f32, input: &Input, worldContext: &mut WorldContext, audio: &mut AudioContext) {

        if (self.call){
            self.call = false;
        }

        if (self.hit(input.MousePosition().x, input.MousePosition().y)) {
            self.is_inside = true;
            self.animation_color += deltaTime * 10.0;
        }else{
            self.is_inside = false;
            self.animation_color -= deltaTime * 10.0;
        }

        self.animation_color = f32::clamp(self.animation_color, 0.0, 1.0);

        if (input.MouseIsDown()) {
            if(!self.is_down){
                self.is_down = true;
            }
        }else if (self.is_down){
            self.is_down = false;

            if (self.is_inside) {
                self.call = true;
                audio.beep_button();
                worldContext.set_ui_action(self.button_name);
            }
        }

        let mut colorStart = Color::default();
        colorStart.from_8bit(self.normal_color);


        let mut colorHover = Color::default();
        colorHover.from_8bit(self.hover_color);


        let r = f32::lerp(colorStart.r as f32, colorHover.r as f32, self.animation_color);
        let g = f32::lerp(colorStart.g as f32, colorHover.g as f32, self.animation_color);
        let b = f32::lerp(colorStart.b as f32, colorHover.b as f32, self.animation_color);


        self.rectangle.set_color(Color::new(r as u8,g as u8,b as u8, 255))
    }
}
impl Drawable for ButtonLabel{
    fn is_can_draw(&self) -> bool {true}

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {
        self.rectangle.draw(buffer, c);
        self.text_label.draw(buffer, c);
    }
}
impl ButtonLabel{
    fn hit(&self, mx: f32, my: f32) -> bool {

        let x = self.rectangle.X();
        let y = self.rectangle.Y();
        let h = self.rectangle.HalfH();
        let w = self.rectangle.HalfW();

        let left = x - w;
        let right = x + w;
        let top = y - h;
        let bottom = y + h;
        mx >= left && mx <= right && my >= top && my <= bottom
    }

    pub fn new (x : i32, y : i32, w: u32, h:u32, text: String, color: Color, textColor: Color, buttonName : ButtonName, textOffset: Vec2<i32>) -> Self{
        let defautColor = color.to_8bit();
        let hoverColor = Color::new(color.r/2, color.g/2, color.b/2, color.a).to_8bit();

        Self{
            text_label: TextLabel::new(text, (x + textOffset.x) as u32, (y + textOffset.y) as u32, textColor, 3),
            rectangle: Rectangle::new(w, h, x as u32, y as u32, color),
            is_inside: false,
            button_name: buttonName,
            is_down: false,
            call: false,
            normal_color: defautColor,
            hover_color: hoverColor,
            animation_color: 0.0
        }
    }

    pub fn is_call(&self) -> bool{
        self.call
    }
}