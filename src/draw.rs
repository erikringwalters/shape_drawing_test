use crate::cursor::Cursor;
use crate::reload::{ReloadLevel, Reloadable};

use bevy::prelude::*;
use bevy_simple_subsecond_system::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default, Reflect)]
pub enum DrawMode {
    #[default]
    None,
    Dot,
    Line,
    Rectangle,
    Circle,
    Arc,
}

pub const DEFAULT_RESOLUTION: u32 = 64;
pub const DEFAULT_POS: Vec3 = Vec3::splat(f32::MIN);

#[derive(Component, Debug, Default)]
pub struct Dot {
    position: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct Line {
    start: Vec3,
    end: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct Circle {
    center: Vec3,
    radius: f32,
}

#[derive(Resource, Debug, PartialEq)]
pub struct CurrentPositions {
    start: Vec3,
    end: Vec3,
}

#[derive(Resource, Default, Debug, PartialEq, PartialOrd)]
pub struct LineChain {
    count: u32,
}

impl Default for CurrentPositions {
    fn default() -> Self {
        CurrentPositions {
            start: DEFAULT_POS,
            end: DEFAULT_POS,
        }
    }
}

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DrawMode>()
            .insert_resource(CurrentPositions::default())
            .insert_resource(LineChain::default())
            .add_systems(
                Update,
                (
                    change_draw_mode,
                    handle_drawing,
                    display_lines,
                    display_circles,
                    display_dots,
                )
                    .chain(),
            );
    }
}

#[hot]
fn change_draw_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<DrawMode>>,
    current_positions: ResMut<CurrentPositions>,
    line_chain: ResMut<LineChain>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::None);
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::Dot);
    } else if keyboard.just_pressed(KeyCode::KeyS) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::Line);
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::Rectangle);
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::Circle);
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        reset_drawing(current_positions, line_chain);
        state.set(DrawMode::Arc);
    }
}

#[hot]
fn handle_drawing(
    commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<DrawMode>>,
    cursor: Res<Cursor>,
    current_positions: ResMut<CurrentPositions>,
    line_chain: ResMut<LineChain>,
) {
    match state.get() {
        DrawMode::None => {
            return;
        }
        DrawMode::Dot => {
            handle_draw_dot(commands, mouse_input, cursor);
        }
        DrawMode::Line => {
            handle_draw_line(commands, mouse_input, cursor, current_positions, line_chain);
        }
        DrawMode::Circle => {
            handle_draw_circle(commands, mouse_input, cursor, current_positions);
        }
        _ => {
            return;
        }
    }
}

#[hot]
fn handle_draw_dot(
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
fn handle_draw_line(
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
fn handle_draw_circle(
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
pub fn reset_drawing(
    current_positions: ResMut<CurrentPositions>,
    mut line_chain: ResMut<LineChain>,
) {
    reset_current_positions(current_positions);
    line_chain.count = 0
}

#[hot]
fn reset_current_positions(mut current_positions: ResMut<CurrentPositions>) {
    *current_positions = CurrentPositions::default();
}

#[hot]
fn display_dots(mut gizmos: Gizmos, query: Query<&Dot>) {
    // Draw existing dots
    for dot in query.iter() {
        gizmos.circle(
            Isometry3d::new(
                dot.position + Dir3::Y * 0.,
                Quat::from_rotation_arc(Vec3::Z, Dir3::Y.as_vec3()),
            ),
            0.05,
            Color::WHITE,
        );
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
