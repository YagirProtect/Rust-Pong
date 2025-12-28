use vek::Vec2;
use crate::classes::t_collision::{Intersection};

#[derive(Clone, Copy, Debug)]
pub struct RectCollider {

    has_intersection: bool,
    last_intersection_normal: Vec2<f32>,
    last_penetration: f32,
    min: Vec2<f32>,
    max: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl RectCollider {
    pub fn set_bounds(&mut self, min: Vec2<f32>, max: Vec2<f32>) {
        self.min = min;
        self.max = max;
    }
}
impl RectCollider {
    pub fn new(min: Vec2<f32>, max: Vec2<f32>) -> Self {
        Self {
            min,
            max,
            has_intersection: false,
            last_intersection_normal: Vec2::zero(),
            velocity: Vec2::zero(),
            last_penetration: 0.0,
        }
    }
}

impl Intersection for RectCollider {
    fn has_intersection(&self) -> bool {self.has_intersection}
    fn set_intersection(&mut self, state: bool, normal: Vec2<f32>, velocity: Vec2<f32>, penetration: f32) {
        if (self.last_intersection_normal != normal) {
            self.last_penetration = penetration;
            self.velocity = velocity;
            self.last_intersection_normal = normal;
            self.has_intersection = state;
        }
    }

    fn get_bounds(&self) -> (Vec2<f32>, Vec2<f32>) {(self.min, self. max)}

    fn check_intersection(&self, minmax: (Vec2<f32>, Vec2<f32>)) -> bool {
        let (a_min, a_max) = self.get_bounds();
        let (b_min, b_max) = minmax;

        a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
    }

    fn get_normal(&self) -> Vec2<f32> {
        self.last_intersection_normal
    }

    fn set_velocity(&mut self, velocity: Vec2<f32>) {
        self.velocity = velocity;
    }

    fn velocity(&self) -> Vec2<f32> {
        self.velocity
    }

    fn get_penetration(&self) -> f32 {
        self.last_penetration
    }
}
