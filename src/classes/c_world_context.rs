use crate::classes::UI::c_label_button::ButtonName;
use vek::Vec2;

pub struct WorldContext{
    ballPosition: Vec2<f32>,
    ballVel: Vec2<f32>,

    playerScores: u32,
    enemyScores: u32,


    uiButtonCalled: ButtonName
}


impl WorldContext{
    pub fn new() -> Self{
       Self{
           ballPosition: Vec2::zero(),
           ballVel: Vec2::zero(),
           playerScores: 0,
           enemyScores: 0,
           uiButtonCalled: ButtonName::None
       }
    }

    pub fn set_ball_position(&mut self, x: f32, y: f32) {
        self.ballPosition = Vec2::new(x, y);
    }

    pub fn get_ball_position(&self)-> Vec2<f32> {
        self.ballPosition
    }


    pub fn set_ball_vel(&mut self, x: f32, y: f32) {
        self.ballVel = Vec2::new(x, y);
    }

    pub fn get_ball_vel(&self)-> Vec2<f32> {
        self.ballVel
    }

    pub fn get_player_scores(&self)-> u32 {
        self.playerScores
    }

    pub fn get_enemy_scores(&self) -> u32 {
        self.enemyScores
    }

    pub fn add_scores(&mut self, ballPos: f32) {
        if (ballPos < 0.0){
            self.enemyScores += 1;
        }else{
            self.playerScores += 1;
        }
    }

    pub fn clear(&mut self) {
        self.set_ball_position(0.0, 0.0);
        self.set_ball_vel(0.0, 0.0);
        self.playerScores = 0;
        self.enemyScores = 0;
    }

    pub fn set_ui_action(&mut self, button_name: ButtonName){
        self.uiButtonCalled = button_name;
    }

    pub fn clear_ui_action(&mut self) {
        self.set_ui_action(ButtonName::None)
    }

    pub fn get_ui_action(&self) -> ButtonName{ self.uiButtonCalled }
}