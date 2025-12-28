use crate::classes::c_canvas::Canvas;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::t_collision::{Collision, Intersection};
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;
use vek::{Vec2, Wrap};
use rand::Rng;
use std::f32::consts::TAU;
use rand::rngs::ThreadRng;
use crate::classes::c_world_context::WorldContext;
use crate::services::math;

pub struct Ball{
    rectangle: Rectangle,
    move_speed: f32,
    move_dir: Vec2<f32>,
    rand: ThreadRng,
    move_limits_y: Vec2<f32>,
    move_limits_x: Vec2<f32>,
    isActive: bool,
    activeTimer: f32,
    speedMult: f32
}

impl Collision for Ball {
    fn get_collision(&mut self) -> Option<&mut dyn Intersection> {
        if self.has_collision(){
            Some(self.rectangle.get_intersections())
        }else{
            None
        }
    }
    fn has_collision(&self) -> bool { true }
}

impl Drawable for Ball {
    fn is_can_draw(&self) -> bool {
        self.rectangle.is_can_draw()
    }

    fn draw(&self, buffer: &mut [u32], c: &Canvas) {
        self.rectangle.draw(buffer, c);
    }
}

impl Updatable for Ball {
    fn has_update(&self) -> bool { true }
    fn update(&mut self, delta_time: f32, input: &Input, worldContext : &mut WorldContext) {

        if (input.AxisRaw() != 0){
            self.isActive = true;
        }
        if (self.isActive){
            self.activeTimer += delta_time;
        }

        if (self.move_dir.magnitude() == 0.0 && self.isActive && self.activeTimer > 0.25){
            self.speedMult = 0.8;
            let mut  randomDir = Self::random_dir(self);
            randomDir.y = randomDir.y * 0.1;
            randomDir.normalize();
            self.move_dir = randomDir;
        }

        if (!self.rectangle.get_intersections().has_intersection()) {
            self.collide_screen_bounds(worldContext);

        }
        else{
            self.collide_object();
        }
        self.speedMult += delta_time * 0.05;
        if (self.speedMult > 1.4){
            self.speedMult = 1.4;
        }
        worldContext.set_ball_position(self.rectangle.X(), self.rectangle.Y());
        self.rectangle.move_rect_pos(self.move_dir.x * self.move_speed * self.speedMult * delta_time, self.move_dir.y * self.move_speed * delta_time)
    }
}

impl Ball{
    pub fn new(rectangle: Rectangle, config: &Config) -> Ball{
        Self {
            rectangle,
            move_speed: config.BallMoveSpeed(),
            move_dir: Vec2::zero(),
            rand: rand::rng(),
            move_limits_y: Vec2::new(0.0, config.Height() as f32),
            move_limits_x: Vec2::new(0.0, config.Width() as f32),
            isActive: false,
            activeTimer: 0.0,
            speedMult: 1.0
        }
    }

    pub fn random_dir(&mut self) -> Vec2<f32> {
        let a: f32 = self.rand.random_range(0.0..TAU);
        Vec2::new(a.cos(), a.sin())
    }

    fn collide_object(&mut self) {
        let normal = self.rectangle.get_intersections().get_normal();
        let additionVel = self.rectangle.get_intersections().velocity() * 0.1;
        let penetration = self.rectangle.get_intersections().get_penetration();

        self.rectangle.move_rect_pos(
            normal.x * (penetration + 0.01),
            normal.y * (penetration + 0.01)
        );

        self.move_dir = self.move_dir.reflected(normal) + additionVel;
        self.move_dir.normalize();

        if (self.move_dir.x.abs() < 0.3) {
            if (self.move_dir.x < 0.0) {
                self.move_dir.x = -0.3;
            } else {
                self.move_dir.x = 0.3;
            }
        }
    }

    fn collide_screen_bounds(&mut self, world_context: &mut WorldContext) {
        if (self.rectangle.Y() + self.rectangle.HalfH() >= self.move_limits_y.y) {
            let penetration = self.rectangle.Y() + self.rectangle.HalfH() - self.move_limits_y.y;
            self.rectangle.move_rect_pos(0.0, -penetration + 0.1);

            self.move_dir = self.move_dir.reflected(math::get_up_limit_normal())
        }
        if (self.rectangle.Y() - self.rectangle.HalfH() < self.move_limits_y.x) {
            let penetration = self.rectangle.Y() - self.rectangle.HalfH();
            self.rectangle.move_rect_pos(0.0, -penetration + 0.1);
            self.move_dir = self.move_dir.reflected(math::get_bottom_limit_normal())
        }

        if (self.rectangle.X() < self.move_limits_x.x || self.rectangle.X() > self.move_limits_x.y) {

            world_context.add_scores(self.rectangle.X());
            
            self.rectangle.change_rect_pos(self.move_limits_x.y / 2.0, self.move_limits_y.y / 2.0);
            self.move_dir = Vec2::zero();
            self.isActive = false;
            self.activeTimer = 0.0;
        }
    }
}