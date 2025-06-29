use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::Dot,
    draw::{CurrentPositions, DEFAULT_POS, DrawMode, reset_current_positions},
    line::Line,
};

pub struct RectanglePlugin;

impl Plugin for RectanglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_rectangles);
    }
}

#[hot]
pub fn handle_draw_rectangle(
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

    // Define start of rectangle
    if current_positions.start == DEFAULT_POS {
        current_positions.start = cursor.position;
    }
    // Define end
    else if current_positions.end == DEFAULT_POS {
        current_positions.end = cursor.position;
    }

    // Create line and dots entities if both start and end are defined
    if current_positions.start != DEFAULT_POS && current_positions.end != DEFAULT_POS {
        let positions = [
            current_positions.start,
            vec3(current_positions.end.x, 0., current_positions.start.z),
            current_positions.end,
            vec3(current_positions.start.x, 0., current_positions.end.z),
        ];
        let mut start_index = 0;
        let mut end_index = positions.len() - 1;

        for position in positions {
            commands.spawn((
                Dot { position: position },
                Reloadable {
                    level: ReloadLevel::Hard,
                },
            ));

            commands.spawn((
                Line {
                    start: positions[start_index],
                    end: positions[end_index],
                },
                Reloadable {
                    level: ReloadLevel::Hard,
                },
            ));

            start_index += 1;
            end_index = if end_index >= positions.len() - 1 {
                0
            } else {
                end_index + 1
            };
        }
        reset_current_positions(current_positions);
    }
}

#[hot]
pub fn display_rectangles(
    mut gizmos: Gizmos,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_positions: ResMut<CurrentPositions>,
) {
    if state.get() == &DrawMode::Rectangle && current_positions.start != DEFAULT_POS {
        let positions = [
            current_positions.start,
            vec3(cursor.position.x, 0., current_positions.start.z),
            cursor.position,
            vec3(current_positions.start.x, 0., cursor.position.z),
        ];
        let mut start_index = 0;
        let mut end_index = positions.len() - 1;

        for _ in positions {
            gizmos.line(positions[start_index], positions[end_index], Color::WHITE);
            start_index += 1;
            end_index = if end_index >= positions.len() - 1 {
                0
            } else {
                end_index + 1
            };
        }
    }
}
