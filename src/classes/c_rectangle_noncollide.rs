use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_input::Input;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::Collision;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;

pub struct RectangleNonCollide{
    rectangle: Rectangle
}


impl Collision for RectangleNonCollide {}

impl Drawable for RectangleNonCollide {
    fn is_can_draw(&self) -> bool {true}

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {
        self.rectangle.draw(buffer, c);
    }
}

impl Updatable for RectangleNonCollide {
    fn has_update(&self) -> bool {false}

    fn update(&mut self, deltaTime: f32, input: &Input, worldContext: &mut WorldContext, audio: &mut AudioContext) {}
}

impl RectangleNonCollide {
    pub fn new(width: u32, height: u32, x: u32, y: u32, color: Color) -> Self {
        Self{
            rectangle: Rectangle::new(width, height, x, y, color)
        }
    }
}