use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::c_world_context::WorldContext;
use crate::classes::e_current_scene::CurrentScene;
use crate::classes::scenes::e_scene_id::SceneId;
use crate::classes::scenes::t_scene_result::SceneResult;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_entity::Entity;
use crate::classes::t_updatable::Updatable;
use crate::classes::UI::c_label_button::{ButtonLabel, ButtonName};
use crate::classes::UI::c_text_label::TextLabel;
use crate::services::collsions_solver::{solve_collisions, solve_collisions_clear};
use vek::Vec2;

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

        let playButton =  ButtonLabel::new(
            100,
            (canvas.Height() as i32) - 150,
            200,
            40,
            "Play".to_string(),
            Color::new(230, 230, 230, 230),
            Color::new(30, 30, 30, 30),
            ButtonName::Play,
            Vec2::new(-5, -10)
        );

        let creditsButton =  ButtonLabel::new(
            100,
            (canvas.Height() as i32) - 100,
            200,
            40,
            "Credits".to_string(),
            Color::new(230, 230, 230, 230),
            Color::new(30, 30, 30, 30),
            ButtonName::Credits,
            Vec2::new(-85, -10)
        );

        let exitButton =  ButtonLabel::new(
            100,
            (canvas.Height() as i32) - 50,
            200,
            40,
            "Exit".to_string(),
            Color::new(230, 230, 230, 230),
            Color::new(30, 30, 30, 30),
            ButtonName::Exit,
            Vec2::new(-5, -10)
        );

        let nameText = TextLabel::new(
            "Mega Pong".to_string(),
            5,
            20,
            Color::new(255,255,255,255),
            4
        );


        Self{
            entity: vec![
                Box::new(playButton),
                Box::new(creditsButton),
                Box::new(exitButton),
                Box::new(nameText),
            ],
            world_context,
        }
    }
    fn update(&mut self, dt: f32, input: &Input, audio: &mut AudioContext) -> CurrentScene{
        for renderer in self.entity.iter_mut() {
            if (renderer.has_update()) {
                renderer.update(dt, &input, &mut self.world_context, audio);
            }
        }

        let action = self.world_context.get_ui_action();
        self.world_context.clear_ui_action();

        match action {
            ButtonName::Play => {
                return CurrentScene::Switch(SceneId::Game);
            },
            ButtonName::Credits => {
                return CurrentScene::Switch(SceneId::Credits);
            },
            ButtonName::Exit => {
                return CurrentScene::Quit;
            },
            ButtonName::None | ButtonName::Exit | ButtonName::ToMenu => ()
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