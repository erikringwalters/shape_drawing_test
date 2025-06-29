use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::Dot,
    draw::{CurrentPositions, DEFAULT_POS, DEFAULT_RESOLUTION, DrawMode, reset_current_positions},
};

#[derive(Component, Debug, Default)]
pub struct Circle {
    pub center: Vec3,
    pub radius: f32,
}

pub struct CirclePlugin;

impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_circles);
    }
}

#[hot]
pub fn handle_draw_circle(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_positions: ResMut<CurrentPositions>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_current_positions(current_positions);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define start (center) of circle
    if current_positions.start == DEFAULT_POS {
        current_positions.start = cursor.position;
    }
    // Define end (radius)
    else if current_positions.end == DEFAULT_POS {
        current_positions.end = cursor.position;
    }

    // Create circle entity if both start and end are defined
    if current_positions.start != DEFAULT_POS && current_positions.end != DEFAULT_POS {
        commands.spawn((
            Dot {
                position: current_positions.start,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        commands.spawn((
            Circle {
                center: current_positions.start,
                radius: (current_positions.end - current_positions.start).length(),
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        reset_current_positions(current_positions);
    }
}

#[hot]
fn display_circles(
    mut gizmos: Gizmos,
    query: Query<&Circle>,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_positions: ResMut<CurrentPositions>,
) {
    // Display existing circles
    for circle in query.iter() {
        gizmos
            .circle(
                Isometry3d::new(
                    circle.center + Dir3::Y * 0.,
                    Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
                ),
                circle.radius,
                Color::WHITE,
            )
            .resolution(DEFAULT_RESOLUTION);
    }
    // Display currently drawn circle
    if state.get() == &DrawMode::Circle && current_positions.start != DEFAULT_POS {
        gizmos
            .circle(
                Isometry3d::new(
                    current_positions.start + Dir3::Y * 0.,
                    Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
                ),
                (cursor.position - current_positions.start).length(),
                Color::WHITE,
            )
            .resolution(DEFAULT_RESOLUTION);
    }
}
