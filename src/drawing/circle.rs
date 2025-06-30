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
    mut current_drawing: ResMut<CurrentDrawing>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        reset_current_drawing(current_drawing);
        return;
    }
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Define center of circle
    if current_drawing.position[0] == DEFAULT_POS {
        current_drawing.position[0] = cursor.position;
    }
    // Define end
    else if current_drawing.position[1] == DEFAULT_POS {
        current_drawing.position[1] = cursor.position;
    }

    let center = current_drawing.position[0];
    let end = current_drawing.position[1];

    // Create circle entity if both center and end are defined
    if center != DEFAULT_POS && end != DEFAULT_POS {
        commands.spawn((
            Dot { position: center },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        commands.spawn((
            Circle {
                center: center,
                radius: (end - center).length(),
            },
            Reloadable {
                level: ReloadLevel::Hard,
            },
        ));
        reset_current_drawing(current_drawing);
    }
}

#[hot]
fn display_circles(
    mut gizmos: Gizmos,
    query: Query<&Circle>,
    cursor: Res<Cursor>,
    state: Res<State<DrawMode>>,
    current_drawing: ResMut<CurrentDrawing>,
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
    if state.get() == &DrawMode::Circle && current_drawing.position[0] != DEFAULT_POS {
        let center = current_drawing.position[0];
        gizmos
            .circle(
                Isometry3d::new(
                    center + Dir3::Y * 0.,
                    Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
                ),
                (cursor.position - center).length(),
                Color::WHITE,
            )
            .resolution(DEFAULT_RESOLUTION);
    }
}
