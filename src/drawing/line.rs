use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

#[derive(Component, Debug, Default)]
pub struct Line {
    start: Vec3,
    end: Vec3,
}

use super::{
    dot::Dot,
    draw::{CurrentPositions, DEFAULT_POS, DrawMode, LineChain, reset_drawing},
};

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_lines);
    }
}

#[hot]
pub fn handle_draw_line(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_positions: ResMut<CurrentPositions>,
    mut line_chain: ResMut<LineChain>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_drawing(current_positions, line_chain);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    // Define start of line
    if current_positions.start == DEFAULT_POS {
        current_positions.start = cursor.position;
    }
    // Define end of line
    else if current_positions.end == DEFAULT_POS {
        current_positions.end = cursor.position;
    }

    // Create line and dots entities if both start and end are defined
    if current_positions.start != DEFAULT_POS && current_positions.end != DEFAULT_POS {
        if line_chain.count == 0 {
            commands.spawn((
                Dot {
                    position: current_positions.start,
                },
                Reloadable {
                    level: ReloadLevel::Hard,
                },
            ));
        }

        commands.spawn((
            Dot {
                position: current_positions.end,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        commands.spawn((
            Line {
                start: current_positions.start,
                end: current_positions.end,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        current_positions.start = current_positions.end;
        current_positions.end = DEFAULT_POS;
        line_chain.count += 1;
    }
}

#[hot]
fn display_lines(
    mut gizmos: Gizmos,
    query: Query<&Line>,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_positions: ResMut<CurrentPositions>,
) {
    // Display existing lines
    for line in query.iter() {
        gizmos.line(line.start, line.end, Color::WHITE);
    }
    // Display currently drawn line
    if state.get() == &DrawMode::Line && current_positions.start != DEFAULT_POS {
        gizmos.line(current_positions.start, cursor.position, Color::WHITE);
    }
}
