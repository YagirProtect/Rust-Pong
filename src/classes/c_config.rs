use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::classes::c_color::Color;
use crate::services::file_system;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config{
    width: u32,
    height: u32,
    title: String,
    fps: u32,
    background_color: String,
    player_move_speed: f32,
    ball_move_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        let mut backColor = Color::default();
        backColor.set(20, 20, 20, 255);


        Self {
            width: 640,
            height: 360,
            title: "Pong Of Gods".to_string(),
            fps: 60,
            background_color: backColor.to_8bit().to_string(),
            player_move_speed: 600.0,
            ball_move_speed: 700.0
        }
    }
}

impl Config {
    pub fn Width(&self) -> u32{
        self.width
    }

    pub fn Height(&self) -> u32{
        self.height
    }

    pub fn Title(&self) -> String {
        self.title.clone()
    }

    pub fn Fps(&self) -> u32{
        self.fps
    }

    pub fn BackgroundColor(&self) -> String {
        self.background_color.clone()
    }

    pub fn PlayerMoveSpeed(&self) -> f32 {
        self.player_move_speed
    }

    pub fn BallMoveSpeed(&self) -> f32 {
        self.ball_move_speed
    }
}

impl Config {
    pub fn new() -> Config {
        println!("{}", "Config created");
        Config :: default()
    }

    pub fn file_name() -> String{
        return "config.json".to_string()
    }

    pub fn full_file_path() -> PathBuf {
        return file_system::get_app_dir().join("config.json");
    }

    pub fn read_file(&mut self){
        if (file_system::is_file_exist(Self::file_name().as_str())){
            let file = match File::open(Self::full_file_path()){
                Ok(file) => file,
                Err(error) => panic!("There was a problem opening the file: {:?}", error),
            };



            let reader = BufReader::new(file);

            let this: Config = match serde_json::from_reader(reader) {
                Ok(config) => config,
                Err(error) => panic!("There was a problem deserializing the file: {:?}", error),
            };

            *self = this;
            Self::write_file(self);
            println!("{}", "Config loaded");
            println!("FPS: {}", self.fps);
        }else{
            Self::write_file(self);
        }
    }

    pub fn write_file(&self){
        let file = File::create(Self::full_file_path()).unwrap();
        let writer = BufWriter::new(file);

        println!("{}", "Config saved");
        match serde_json::to_writer_pretty(writer, self){
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}

