mod cursor;
mod drawing;
mod reload;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_simple_subsecond_system::*;
use cursor::CursorPlugin;
use drawing::draw::DrawPlugin;
use reload::{ReloadPlugin, Reloadable};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(CursorPlugin)
        .add_plugins(DrawPlugin)
        .add_plugins(ReloadPlugin)
        .add_systems(Startup, setup)
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
        MeshMaterial3d(materials.add(Color::srgba(0.1, 0.4, 0.4, 0.0))),
        Reloadable::default(),
    ));
}
