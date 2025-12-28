use crate::classes::c_ball::Ball;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_player::Player;
use crate::classes::c_player_bot::PlayerBot;
use crate::classes::c_rectangle::Rectangle;
use crate::classes::c_scoreboard::ScoreBoard;
use crate::classes::c_world_context::WorldContext;
use crate::classes::e_current_scene::CurrentScene;
use crate::classes::scenes::t_scene_result::SceneResult;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_entity::Entity;
use crate::classes::t_updatable::Updatable;
use crate::services::collsions_solver::{solve_collisions, solve_collisions_clear};

pub struct MenuScene{
    entity: Vec<Box<dyn Entity>>,
    world_context: WorldContext
}

impl Drawable for MenuScene {
    fn is_can_draw(&self) -> bool {true}

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {

        for renderer in self.entity.iter_mut() {
            if (renderer.is_can_draw()) {
                renderer.draw(buffer, c);
            }
        }
    }
}

impl SceneResult for MenuScene{
    fn new(config: &Config, canvas: &mut Canvas) -> Self{

        let mut world_context = WorldContext::new();
        
        Self{
            entity: Vec::new(),
            world_context,
        }
    }
    fn update(&mut self, deltaTime: f32, input: &Input) -> CurrentScene{
        for renderer in self.entity.iter_mut() {
            if (renderer.has_update()) {
                renderer.update(deltaTime, &input, &mut self.world_context);
            }
        }

        CurrentScene::None
    }

    fn solve_physics(&mut self) {
        solve_collisions(&mut self.entity);
    }

    fn clear_solve_physics(&mut self) {
        solve_collisions_clear(&mut self.entity);
    }

}