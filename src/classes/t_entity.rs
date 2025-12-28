use crate::classes::t_collision::Collision;
use crate::classes::t_drawable::Drawable;
use crate::classes::t_updatable::Updatable;

pub trait Entity: Drawable + Updatable + Collision {}


impl<T: Drawable + Updatable + Collision> Entity for T {}