use vek::{Lerp, Vec2};
use crate::classes::c_canvas::Canvas;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::{Collision, Intersection};
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;
use crate::services::math;

pub struct PlayerBot{
    rectangle: Rectangle,
    move_speed: f32,
    move_limits: Vec2<f32>,
    axis: f32
}


impl Collision for PlayerBot {
    fn get_collision(&mut self) -> Option<&mut dyn Intersection> {
        if self.has_collision(){
            Some(self.rectangle.get_intersections())
        }else{
            None
        }
    }
    fn has_collision(&self) -> bool { true }
}

impl Drawable for PlayerBot {
    fn is_can_draw(&self) -> bool {
        self.rectangle.is_can_draw()
    }

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {
        self.rectangle.draw(buffer, c);
    }
}

impl Updatable for PlayerBot {
    fn has_update(&self) -> bool { true }
    fn update(&mut self, delta_time: f32, input: &Input, worldContext : &mut WorldContext) {

        let mut dir = 0.0;
        if (worldContext.get_ball_position().y > self.rectangle.Y() + self.rectangle.HalfH()/2.0){
            dir = 1.0;
        }
        if (worldContext.get_ball_position().y < self.rectangle.Y() - self.rectangle.HalfH()/2.0){
            dir = -1.0;
        }

        self.axis = f32::lerp(self.axis, dir, delta_time * 7.0);

        if (math::is_on_bounds_y(&self.rectangle, self.axis, self.move_limits.x, self.move_limits.y)) {
            self.rectangle.move_rect_pos(0.0, self.axis * self.move_speed * delta_time);
        } else {
            if (self.axis > 0.0) {
                self.rectangle.change_rect_pos(self.rectangle.X(), self.move_limits.y - self.rectangle.HalfH());
                self.axis = 0.0;
            }

            if (self.axis < 0.0) {
                self.rectangle.change_rect_pos(self.rectangle.X(), self.move_limits.x + self.rectangle.HalfH());
                self.axis = 0.0;
            }
        }





        self.rectangle.get_intersections().set_velocity(Vec2::new(0.0, self.axis));
    }
}

impl PlayerBot{
    pub fn new(rectangle: Rectangle, config: &Config) -> PlayerBot{
        Self {
            rectangle,
            move_speed: config.PlayerMoveSpeed(),
            move_limits: Vec2::new(0.0, config.Height() as f32),
            axis: 0.0
        }
    }
}