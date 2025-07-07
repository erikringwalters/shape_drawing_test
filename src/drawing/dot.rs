use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::size::DOT_RADIUS;

#[derive(Component, Debug, Default)]
pub struct Dot {
    pub position: Vec3,
}

pub struct DotPlugin;

impl Plugin for DotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_dots);
    }
}

#[hot]
pub fn handle_draw_dot(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    commands.spawn((
        Dot {
            position: cursor.position,
        },
        Reloadable {
            level: ReloadLevel::Hard,
        },
    ));
}

#[hot]
pub fn display_dots(mut gizmos: Gizmos, query: Query<&Dot>) {
    for dot in query.iter() {
        gizmos.circle(
            Isometry3d::new(
                dot.position + Dir3::Y * 0.,
                Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
            ),
            DOT_RADIUS,
            Color::WHITE,
        );
    }
}
