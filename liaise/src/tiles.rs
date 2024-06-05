use crate::drone::Drone;
use crate::grid::GridSizes;
use crate::grid::Position;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_target(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
) {
    let position = Position::new(4, 0);

    create_target_at(commands, meshes, materials, res, position)
}

#[derive(Component)]
pub struct Target;

pub fn create_target_at(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
    position: Position,
) {
    let GridSizes { tile_size, .. } = *res;

    let green = materials.add(Color::GREEN);
    let mesh = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(
        tile_size - 2.0,
        tile_size - 2.0,
    ))));

    let abs_position = position.to_vec3(tile_size);

    commands.spawn((
        Target,
        position,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: green,
            transform: Transform::from_translation(abs_position),
            ..Default::default()
        },
    ));
}

pub fn reached_target(
    mut commands: Commands,
    target_query: Query<(Entity, &Position), With<Target>>,
    drone_query: Query<&Position, With<Drone>>,
) {
    for drone_position in drone_query.iter() {
        for (target_entity, target_position) in target_query.iter() {
            if drone_position == target_position {
                println!("Drone reached target at {:?}", drone_position);

                commands.entity(target_entity).despawn();
            }
        }
    }
}
