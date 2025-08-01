use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

use crate::{
    cursor::Cursor,
    reload::{ReloadLevel, Reloadable},
};

use super::{
    dot::Dot,
    draw::{CurrentDrawing, DEFAULT_POS, DEFAULT_RESOLUTION, DrawMode, reset_current_drawing},
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
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_current_drawing(current_drawing);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define center of arc
    if current_drawing.position[0] == DEFAULT_POS {
        current_drawing.position[0] = cursor.position;
    }
    // Define start of arc
    else if current_drawing.position[1] == DEFAULT_POS {
        current_drawing.position[1] = cursor.position;
    }
    // Define end of arc
    else if current_drawing.position[2] == DEFAULT_POS {
        current_drawing.position[2] = cursor.position;
    }

    for position in current_drawing.position {
        println!("{:?}", position);
    }
    println!("");

    let center = current_drawing.position[0];
    let start = current_drawing.position[1];
    let end = current_drawing.position[2];

    // Create arc entity if center, start, and end are all defined
    if center != DEFAULT_POS && start != DEFAULT_POS && end != DEFAULT_POS {
        commands.spawn((
            Dot { position: center },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        commands.spawn((
            Arc {
                center: center,
                start: start,
                end: end,
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        reset_current_drawing(current_drawing);
    }
}

#[hot]
pub fn display_arcs(
    mut gizmos: Gizmos,
    query: Query<&Arc>,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_drawing: ResMut<CurrentDrawing>,
) {
    // Display existing circles
    for arc in query.iter() {
        gizmos.short_arc_3d_between(arc.center, arc.start, arc.end, Color::WHITE);
    }
    // Display currently drawn circle
    if state.get() != &DrawMode::Arc {
        return;
    }

    let center = current_drawing.position[0];
    let start = current_drawing.position[1];

    if center != DEFAULT_POS {
        let radius = if start != DEFAULT_POS {
            (start - center).length()
        } else {
            (cursor.position - center).length()
        };
        gizmos
            .circle(
                Isometry3d::new(
                    center + Dir3::Y * 0.,
                    Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
                ),
                radius,
                Color::WHITE,
            )
            .resolution(DEFAULT_RESOLUTION);
    }
    if center != DEFAULT_POS {
        let to = if start != DEFAULT_POS {
            start
        } else {
            cursor.position
        };
        gizmos.line(center, to, Color::WHITE);
    }
    if start != DEFAULT_POS {
        gizmos.line(center, cursor.position, Color::WHITE);
    }
}
