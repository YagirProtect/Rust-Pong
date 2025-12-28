use crate::classes::c_canvas::Canvas;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_world_context::WorldContext;
use crate::classes::e_current_scene::CurrentScene;
use crate::classes::t_drawable::Drawable;

pub trait SceneResult: Drawable {
    fn new(config: &Config, canvas: &mut Canvas) -> Self where Self: Sized;
    fn update(&mut self, dt: f32, input: &Input) -> CurrentScene;
    fn solve_physics(&mut self);
    fn clear_solve_physics(&mut self);

}
