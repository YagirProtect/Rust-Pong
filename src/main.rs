#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;
mod classes;

use std::str::FromStr;
use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_config::Config;
use crate::classes::c_input::Input;
use crate::classes::e_current_scene::CurrentScene;
use crate::classes::scenes::c_credits_scene::CreditsScene;
use crate::classes::scenes::c_game_scene::GameScene;
use crate::classes::scenes::c_menu_scene::MenuScene;
use crate::classes::scenes::t_scene_result::SceneResult;
use classes::scenes::e_scene_id::SceneId;
use minifb::{Window, WindowOptions};
use std::time::Instant;

fn main() {

    let mut config = Config::new();
    config.read_file();
    let mut canvas = Canvas::new(&mut config);
    let mut input = Input::new();
    let mut audio = AudioContext::new();


    render_loop(config, &mut canvas, &mut input, &mut audio);
}

fn make_scene(id: SceneId, config: &Config, canvas: &mut Canvas) -> Box<dyn SceneResult> {
    match id {
        SceneId::Game => Box::new(GameScene::new(config, canvas)),
        SceneId::Menu => Box::new(MenuScene::new(config, canvas)),
        SceneId::Credits => Box::new(CreditsScene::new(config, canvas))
    }
}

fn render_loop(config: Config, canvas: &mut Canvas, input: &mut Input, audio: &mut AudioContext){

    let width = config.Width() as usize;
    let height = config.Height() as usize;
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(config.Title().as_str(),width, height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    #[cfg(target_os = "windows")]
    {
        // path to icon
        if let Ok(icon) = minifb::Icon::from_str("assets/icon.ico") {
            window.set_icon(icon);
        }
    }

    window.set_target_fps(config.Fps() as usize);

    let mut last = Instant::now();


    let mut scene: Box<dyn SceneResult> = make_scene(SceneId::Menu, &config, canvas);

    while window.is_open(){

        let mut delta_time = 0.0;
        {
            let now = Instant::now();
            delta_time = (now - last).as_secs_f32();
            last = now;

            if delta_time > 0.1 { delta_time = 0.1; }
        }

        input.update(&window, delta_time);



        canvas.clear(&mut buffer);
        scene.solve_physics();
        match scene.update(delta_time, input, audio){
            CurrentScene::None => {}
            CurrentScene::Switch(new_scene_id) => {
                scene = make_scene(new_scene_id, &config, canvas);
                continue;
            }
            CurrentScene::Quit => {
                break;
            }
        };
        scene.draw(& mut buffer, &canvas);

        scene.clear_solve_physics();

        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}

