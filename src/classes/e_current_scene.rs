use crate::classes::scenes::e_scene_id::SceneId;

pub enum CurrentScene {
    None,
    Switch(SceneId),
    Quit,
}