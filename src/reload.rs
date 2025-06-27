use bevy::prelude::*;

#[derive(Default, Debug)]
pub enum ReloadLevel {
    #[default]
    Soft,
    Hard,
}

#[derive(Component, Default)]
pub struct Reloadable {
    pub level: ReloadLevel,
}
