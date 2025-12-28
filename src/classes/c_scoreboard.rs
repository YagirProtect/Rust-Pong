use crate::classes::c_canvas::Canvas;
use crate::classes::c_color::Color;
use crate::classes::c_input::Input;
use crate::classes::UI::c_text_label::TextLabel;
use crate::classes::c_world_context::WorldContext;
use crate::classes::t_collision::Collision;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;

pub struct ScoreBoard {
    playerScores: TextLabel,
    enemyScores: TextLabel,
}

impl Collision for ScoreBoard {}

impl Updatable for ScoreBoard {
    fn has_update(&self) -> bool { true }

    fn update(&mut self, deltaTime: f32, input: &Input, worldContext: &mut WorldContext) {
        self.playerScores.update_text(worldContext.get_player_scores());
        self.enemyScores.update_text(worldContext.get_enemy_scores());
    }
}

impl Drawable for ScoreBoard {
    fn is_can_draw(&self) -> bool {true}

    fn draw(&mut self, buffer: &mut [u32], c: &Canvas) {
        self.playerScores.draw(buffer, c);
        self.enemyScores.draw(buffer, c);
    }
}

impl ScoreBoard {
    pub fn new(canvas: &Canvas) -> Self {


        let offset = (canvas.Width() as f32 * 0.3) as u32;

        Self{
            playerScores: TextLabel::new("0".to_string(), offset, 10, Color::new(200,200,200, 255), 3),
            enemyScores: TextLabel::new("0".to_string(), canvas.Width() - offset, 10, Color::new(200,200,200, 255), 3)
        }
    }
}