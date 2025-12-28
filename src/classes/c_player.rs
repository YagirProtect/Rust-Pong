use vek::Vec2;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::{Collision, Intersection};
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;
use crate::services::math;

pub struct Player{
    rectangle: Rectangle,
    move_speed: f32,
    move_limits: Vec2<f32>
}


impl Collision for Player {
    fn get_collision(&mut self) -> Option<&mut dyn Intersection> {
        if self.has_collision(){
         Some(self.rectangle.get_intersections())
        }else{
            None
        }
    }
    fn has_collision(&self) -> bool { true }
}

impl Drawable for Player {
    fn is_can_draw(&self) -> bool {
        self.rectangle.is_can_draw()
    }

    fn draw(&self, buffer: &mut [u32], c: &Canvas) {
        self.rectangle.draw(buffer, c);
    }
}

impl Updatable for Player {
    fn has_update(&self) -> bool { true }
    fn update(&mut self, delta_time: f32, input: &Input, worldContext: &mut WorldContext) {
        let mut dir = input.Axis() * self.move_speed * delta_time;

        if (math::is_on_bounds_y(&self.rectangle, dir, self.move_limits.x, self.move_limits.y)) {
            self.rectangle.move_rect_pos(0.0, dir);
        } else {
            if (dir > 0.0) {
                self.rectangle.change_rect_pos(0.0, self.move_limits.y - self.rectangle.HalfH());
                dir = 0.0;
            }

            if (dir < 0.0) {
                self.rectangle.change_rect_pos(0.0, self.move_limits.x + self.rectangle.HalfH());
                dir = 0.0;
            }
        }

        self.rectangle.get_intersections().set_velocity(Vec2::new(0.0, dir));
    }
}

impl Player{
    pub fn new(rectangle: Rectangle, config: &Config) -> Player{
        Self {
            rectangle,
            move_speed: config.PlayerMoveSpeed(),
            move_limits: Vec2::new(0.0, config.Height() as f32)
        }
    }
}