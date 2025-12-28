use vek::Vec2;

pub trait Intersection{
    fn has_intersection(&self) -> bool;
    fn set_intersection(&mut self, state: bool, normal: Vec2<f32>, velocity: Vec2<f32>, penetration: f32);
    fn get_bounds(&self) -> (Vec2<f32>, Vec2<f32>);
    fn check_intersection(&self, minmax: (Vec2<f32>, Vec2<f32>)) -> bool;
    fn get_normal(&self) -> Vec2<f32>;
    fn set_velocity(&mut self, velocity: Vec2<f32>);

    fn velocity(&self) -> Vec2<f32>;

    fn get_penetration(&self) -> f32;
}




pub trait Collision{
    fn get_collision(&mut self) -> Option<&mut dyn Intersection> { None }
    fn has_collision(&self) -> bool { false }
}