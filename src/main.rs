use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_simple_subsecond_system::*;
use cursor::CursorPlugin;

mod cursor;

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

#[derive(Resource, Default, Debug)]
pub struct Current {
    pub draw_mode: DrawMode,
}

pub const DEFAULT_RESOLUTION: u32 = 64;

#[derive(Component)]
pub struct Reloadable;

#[derive(Component, Debug, Default)]
pub struct Dot {
    position: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct Line {
    positions: Vec<Vec3>,
}

#[derive(Component, Debug, Default)]
pub struct Circle {
    center: Vec3,
    radius: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(CursorPlugin)
        .insert_resource(Current::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_reload)
        .run();
}

#[hot]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut current: ResMut<Current>,
) {
    current.draw_mode = DrawMode::None;
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 1., 0.).looking_at(Vec3::ZERO, Dir3::Z),
        Reloadable,
    ));

    commands.spawn((DirectionalLight::default(), Reloadable));

    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.5, 1.))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.8, 0.1))),
        Reloadable,
    ));
}

#[hot]
fn handle_reload(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    current: ResMut<Current>,
    query: Query<Entity, With<Reloadable>>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyR) {
        println!("Reloading...");
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        setup(commands, meshes, materials, current);
        println!("Reloaded.")
    }
}
