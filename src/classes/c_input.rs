use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};
use vek::{Lerp, Vec2};

pub struct Input{
    axisRaw: i8,
    axis: f32,
    is_input_used: bool,

    mouse_position: Vec2<f32>,
    mouse_is_down: bool
}

impl Input {
    pub fn SetInputUsed(&mut self, p0: bool) {
        self.is_input_used = p0;
    }
}

impl Default for Input {
    fn default() -> Self {
        Input{axisRaw: 0, axis: 0.0, is_input_used: false, mouse_position: Vec2::new(0.0, 0.0), mouse_is_down: false}
    }
}

impl Input {
    pub fn Axis(&self) -> f32{
        return -self.axis;
    }

    pub fn AxisRaw(&self) -> i8{
        return -self.axisRaw;
    }

    pub fn IsInputUsed(&self) -> bool{
        return self.is_input_used;
    }
}

impl Input{
    pub fn new() -> Self {
        return Input::default();
    }
    pub fn update(&mut self, window: &Window, deltaTime: f32) {
        self.axisRaw = 0;
        if (window.is_key_down(Key::W) | window.is_key_down(Key::Up)) {
            self.axisRaw += 1;
            self.is_input_used = true;
        }
        if (window.is_key_down(Key::S) | window.is_key_down(Key::Down)) {
            self.axisRaw -= 1;
            self.is_input_used = true;
        }


        match window.get_mouse_pos(MouseMode::Clamp){
            Some(mousePosition) => {
                self.mouse_position = Vec2::new(mousePosition.0 as f32, mousePosition.1 as f32);
            }
            None => {}
        }
        
        self.mouse_is_down = window.get_mouse_down(MouseButton::Left);

        self.axis = f32::lerp(self.axis, self.axisRaw as f32, deltaTime * 10.0);
    }
}