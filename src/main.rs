mod cursor;
mod reload;

use std::thread::current;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_simple_subsecond_system::*;
use cursor::{Cursor, CursorPlugin};
use reload::{ReloadLevel, ReloadPlugin, Reloadable};

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

impl Default for CurrentPositions {
    fn default() -> Self {
        CurrentPositions {
            start: DEFAULT_POS,
            end: DEFAULT_POS,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(CursorPlugin)
        .add_plugins(ReloadPlugin)
        .init_state::<DrawMode>()
        .insert_resource(CurrentPositions::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                change_draw_mode,
                handle_drawing,
                display_lines,
                display_dots,
            )
                .chain(),
        )
        .run();
}

#[hot]
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 1., 0.).looking_at(Vec3::ZERO, Dir3::Z),
        Reloadable::default(),
    ));

    commands.spawn((DirectionalLight::default(), Reloadable::default()));

    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.5, 1.))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.8, 0.1))),
        Reloadable::default(),
    ));
}

#[hot]
fn change_draw_mode(
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<DrawMode>>,
    current_positions: ResMut<CurrentPositions>,
) {
    // let key = input.get_just_pressed();
    // match key {
    // TODO: try changing this logic to a match statement
    // }
    if input.just_pressed(KeyCode::Escape) {
        reset_current_positions(current_positions);
        state.set(DrawMode::None);
    } else if input.just_pressed(KeyCode::KeyD) {
        state.set(DrawMode::Dot);
    } else if input.just_pressed(KeyCode::KeyS) {
        state.set(DrawMode::Line);
    } else if input.just_pressed(KeyCode::KeyR) {
        state.set(DrawMode::Rectangle);
    } else if input.just_pressed(KeyCode::KeyC) {
        state.set(DrawMode::Circle);
    } else if input.just_pressed(KeyCode::KeyA) {
        state.set(DrawMode::Arc);
    }
}

#[hot]
fn handle_drawing(
    commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    state: Res<State<DrawMode>>,
    cursor: Res<Cursor>,
    current_positions: ResMut<CurrentPositions>,
) {
    // println!("State: {:?}", state);
    match state.get() {
        DrawMode::None => {
            return;
        }
        DrawMode::Dot => {
            handle_draw_dot(commands, input, cursor);
        }
        DrawMode::Line => handle_draw_line(commands, input, cursor, current_positions),
        _ => {
            return;
        }
    }
}

#[hot]
fn handle_draw_dot(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
) {
    if !input.just_pressed(MouseButton::Left) {
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
    input: Res<ButtonInput<MouseButton>>,
    cursor: Res<Cursor>,
    mut current_positions: ResMut<CurrentPositions>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    println!("{:?}", current_positions.start);
    if current_positions.start == DEFAULT_POS {
        current_positions.start = cursor.position;
    } else if current_positions.end == DEFAULT_POS {
        current_positions.end = cursor.position;
    }
    commands.spawn((
        Dot {
            position: cursor.position,
        },
        Reloadable {
            level: ReloadLevel::Hard,
        },
    ));

    // Draw line if start and end are valid
    if current_positions.start != DEFAULT_POS && current_positions.end != DEFAULT_POS {
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
    }
}

#[hot]
fn reset_current_positions(mut current_positions: ResMut<CurrentPositions>) {
    *current_positions = CurrentPositions::default();
}

#[hot]
fn display_dots(mut gizmos: Gizmos, query: Query<&Dot>) {
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
    current_positions: ResMut<CurrentPositions>,
) {
    for line in query.iter() {
        gizmos.line(line.start, line.end, Color::WHITE);
    }
    if current_positions.start != DEFAULT_POS {
        gizmos.line(current_positions.start, cursor.position, Color::WHITE);
    }
}
