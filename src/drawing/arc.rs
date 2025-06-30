use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::Dot,
    draw::{CurrentDrawing, DEFAULT_POS, reset_current_drawing},
};

#[derive(Component, Debug, Default)]
pub struct Arc {
    pub center: Vec3,
    pub start: Vec3,
    pub end: Vec3,
}

pub struct ArcPlugin;

impl Plugin for ArcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_arcs);
    }
}

#[hot]
pub fn handle_draw_arc(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_drawing: ResMut<CurrentDrawing>,
) {
    // if mouse_input.just_pressed(MouseButton::Right) {
    //     reset_current_drawing(current_drawing);
    //     return;
    // }
    // if !mouse_input.just_pressed(MouseButton::Left) {
    //     return;
    // }

    // // Define start (center) of circle
    // if current_drawing.start == DEFAULT_POS {
    //     current_drawing.start = cursor.position;
    // }
    // // Define end (radius)
    // else if current_drawing.end == DEFAULT_POS {
    //     current_drawing.end = cursor.position;
    // }

    // // Create circle entity if both start and end are defined
    // if current_drawing.start != DEFAULT_POS && current_drawing.end != DEFAULT_POS {
    //     commands.spawn((
    //         Dot {
    //             position: current_drawing.start,
    //         },
    //         Reloadable {
    //             level: ReloadLevel::Hard,
    //         },
    //     ));
    //     reset_current_drawing(current_drawing);
    // }
}

#[hot]
pub fn display_arcs() {}
