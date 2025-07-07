use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

#[derive(Component, Debug, Default)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

use super::{
    dot::Dot,
    draw::{CurrentDrawing, DEFAULT_POS, DrawMode, LineChain, reset_drawing},
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
    mut current_drawing: ResMut<CurrentDrawing>,
    mut line_chain: ResMut<LineChain>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_drawing(current_drawing, line_chain);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define start of line
    if current_drawing.position[0] == DEFAULT_POS {
        current_drawing.position[0] = cursor.position;
    }
    // Define end of line
    else if current_drawing.position[1] == DEFAULT_POS {
        current_drawing.position[1] = cursor.position;
        // TODO: Snap line end to nearby dot
    }

    let start = current_drawing.position[0];
    let end = current_drawing.position[1];

    // Create line and dots entities if both start and end are defined
    if start != DEFAULT_POS && end != DEFAULT_POS {
        if line_chain.count == 0 {
            commands.spawn((
                Dot { position: start },
                Reloadable {
                    level: ReloadLevel::Hard,
                },
            ));
        }

        commands.spawn((
            Dot { position: end },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        commands.spawn((
            Line {
                start: start,
                end: end,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        current_drawing.position[0] = end;
        current_drawing.position[1] = DEFAULT_POS;
        line_chain.count += 1;
    }
}

#[hot]
fn display_lines(
    mut gizmos: Gizmos,
    query: Query<&Line>,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_drawing: ResMut<CurrentDrawing>,
) {
    // Display existing lines
    for line in query.iter() {
        gizmos.line(line.start, line.end, Color::WHITE);
    }
    // Display currently drawn line
    if state.get() == &DrawMode::Line && current_drawing.position[0] != DEFAULT_POS {
        gizmos.line(current_drawing.position[0], cursor.position, Color::WHITE);
    }
}
