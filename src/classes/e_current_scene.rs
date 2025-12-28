use crate::classes::scenes::e_scene_id::SceneId;

#[derive(Clone, Copy, Debug)]
pub enum CurrentScene {
    None,
    Switch(SceneId),
    Quit,
}