use vek::Vec2;
use crate::classes::c_rectangle::Rectangle;

pub fn get_up_limit_normal() -> Vec2<f32> { Vec2::new(0.0, 1.0) }
pub fn get_bottom_limit_normal() -> Vec2<f32> { Vec2::new(0.0, -1.0) }

pub fn get_left_limit_normal() -> Vec2<f32> { Vec2::new(1.0, 0.0) }
pub fn get_right_limit_normal() -> Vec2<f32> { Vec2::new(-1.0, 0.0) }

pub fn is_on_bounds_y(rectangle: &Rectangle, dir: f32, minY: f32, maxY: f32) -> bool {
    rectangle.Y() - rectangle.HalfH() + dir > minY && rectangle.Y() + rectangle.HalfH() + dir < maxY
}
