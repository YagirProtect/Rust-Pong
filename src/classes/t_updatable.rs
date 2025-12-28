use crate::classes::c_audio::AudioContext;
use crate::classes::c_canvas::Canvas;
use crate::classes::c_input::Input;
use crate::classes::c_world_context::WorldContext;

pub trait Updatable {
    fn has_update(&self) -> bool;
    fn update(&mut self, deltaTime: f32, input: &Input, worldContext: &mut WorldContext, audio: &mut AudioContext);
}