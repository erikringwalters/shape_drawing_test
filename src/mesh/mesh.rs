use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_simple_subsecond_system::hot;

use crate::drawing::rectangle::Rectangle;

pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_create_mesh);
    }
}

#[hot]
pub fn handle_create_mesh(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&Rectangle>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let generated_meshes = create_mesh_from_rectangles(query);
    for mesh in generated_meshes {
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            })),
        ));
    }
}

#[hot]
fn create_mesh_from_rectangles(query: Query<&Rectangle>) -> Vec<Mesh> {
    let mut meshes: Vec<Mesh> = Vec::new();
    for rect in query.iter() {
        let start = rect.start;
        let end = rect.end;
        println!("{:?}, {:?}", start, end);
        let positions = [
            start,
            vec3(end.x, 0., start.z),
            end,
            vec3(start.x, 0., end.z),
        ];
        let mut vertices: Vec<Vec3> = Vec::new();
        let normals = vec![[0f32, 0f32, 1f32]; 5];

        for position in positions {
            vertices.push(position);
        }
        meshes.push(
            Mesh::new(
                bevy::render::mesh::PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            )
            // .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals),
        );
    }
    return meshes;
}
