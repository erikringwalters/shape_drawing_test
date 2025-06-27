use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

use crate::setup;

#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ReloadLevel {
    #[default]
    Soft,
    Hard,
}

#[derive(Component, Default)]
pub struct Reloadable {
    pub level: ReloadLevel,
}

pub struct ReloadPlugin;

impl Plugin for ReloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_reload);
    }
}

#[hot]
fn handle_reload(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Reloadable)>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyR) {
        let reload_level = if input.pressed(KeyCode::ShiftLeft) {
            ReloadLevel::Hard
        } else {
            ReloadLevel::Soft
        };
        for (entity, reloadable) in query.iter() {
            if reloadable.level <= reload_level {
                commands.entity(entity).despawn();
            }
        }
        setup(commands, meshes, materials);
        println!("Reloaded.")
    }
}
