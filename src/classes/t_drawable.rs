use crate::classes::c_canvas::Canvas;

pub trait Drawable {
    fn is_can_draw(&self) -> bool;
    fn draw(&mut self, buffer: &mut [u32], c: &Canvas);
}
