mod services;
mod classes;

use std::time::Instant;
use minifb::{Key, Window, WindowOptions};
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
use crate::classes::t_entity::Entity;
use crate::services::collsions_solver::{solve_collisions, solve_collisions_clear};


fn main() {

    let mut config = Config::new();
    config.read_file();
    let mut canvas = Canvas::new(&mut config);
    let mut input = Input::new();
    let mut world_context = WorldContext::new();

    let player = Player::new(
        Rectangle::new(40, 80, 0, canvas.Height()/2, Color::new(255,255,255, 255)),
        &config
    );

    let ball = Ball::new(
        Rectangle::new(25, 25, canvas.Width()/2, canvas.Height()/2, Color::new(255,255,255, 255)),
        &config
    );

    let bot = PlayerBot::new(
        Rectangle::new(40, 80, canvas.Width(), canvas.Height()/2, Color::new(255,255,255, 255)),
        &config
    );

    let score_board = ScoreBoard::new(&canvas);


    let mut entity: Vec<Box<dyn Entity>> = vec![
        Box::new(player),
        Box::new(ball),
        Box::new(bot),
        Box::new(score_board),
    ];

    render_loop(config, &mut canvas, &mut entity, &mut input, &mut world_context);
}

fn render_loop(config: Config, canvas: &mut Canvas, renderers: &mut Vec<Box<dyn Entity>>, input: &mut Input, world_context: &mut WorldContext){

    let width = config.Width() as usize;
    let height = config.Height() as usize;
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(config.Title().as_str(),width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_target_fps(config.Fps() as usize);

    let mut last = Instant::now();



    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut delta_time = 0.0;
        {
            let now = Instant::now();
            delta_time = (now - last).as_secs_f32();
            last = now;

            if delta_time > 0.1 { delta_time = 0.1; }
        }


        input.update(&window, delta_time);
        solve_collisions(renderers);


        canvas.clear(&mut buffer);
        for renderer in renderers.iter_mut() {
            if (renderer.has_update()){
                renderer.update(delta_time, &input, world_context);
            }
            if (renderer.is_can_draw()){
                renderer.draw(&mut buffer, &canvas);
            }
        }


        solve_collisions_clear(renderers);

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}

