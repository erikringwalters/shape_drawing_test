use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::Dot,
    draw::{CurrentDrawing, DEFAULT_POS, DrawMode, reset_current_drawing},
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
    mut current_drawing: ResMut<CurrentDrawing>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_current_drawing(current_drawing);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define start of rectangle
    if current_drawing.position[0] == DEFAULT_POS {
        current_drawing.position[0] = cursor.position;
    }
    // Define end
    else if current_drawing.position[1] == DEFAULT_POS {
        current_drawing.position[1] = cursor.position;
    }

    let start = current_drawing.position[0];
    let end = current_drawing.position[1];

    // Create line and dots entities if both start and end are defined
    if start != DEFAULT_POS && end != DEFAULT_POS {
        let positions = [
            start,
            vec3(end.x, 0., start.z),
            end,
            vec3(start.x, 0., end.z),
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
        reset_current_drawing(current_drawing);
    }
}

#[hot]
pub fn display_rectangles(
    mut gizmos: Gizmos,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_drawing: ResMut<CurrentDrawing>,
) {
    if state.get() == &DrawMode::Rectangle && current_drawing.position[0] != DEFAULT_POS {
        let start = current_drawing.position[0];
        let positions = [
            start,
            vec3(cursor.position.x, 0., start.z),
            cursor.position,
            vec3(start.x, 0., cursor.position.z),
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
