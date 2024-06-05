use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::grid::{GridSizes, Position};

pub fn setup_drone(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
) {
    let position = Position::new(0, 0);

    create_drone_at(commands, meshes, materials, res, position)
}

#[derive(Component)]
pub struct Drone;

pub fn create_drone_at(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
    position: Position,
) {
    let GridSizes { tile_size, .. } = *res;

    let blue = materials.add(Color::BLUE);
    let mesh = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(
        tile_size - 2.0,
        tile_size - 2.0,
    ))));

    let abs_position = position.to_vec3(tile_size);

    commands.spawn((
        Drone,
        position,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: blue,
            transform: Transform::from_translation(abs_position),
            ..Default::default()
        },
    ));
}

#[derive(Event)]
pub struct MoveEvent {
    // pub entity: Entity,
    pub direction: Vec2,
}

pub fn move_drones(
    mut move_events: EventReader<MoveEvent>,
    mut drone_query: Query<(&mut Position, &mut Transform), With<Drone>>,
    grid_sizes: Res<GridSizes>,
) {
    let GridSizes {
        tile_size,
        grid_height,
        grid_width,
    } = *grid_sizes;

    for event in move_events.read() {
        for (mut position, mut transform) in drone_query.iter_mut() {
            let new_x = position.x + event.direction.x as i32;
            let new_y = position.y + event.direction.y as i32;

            if new_x < 0 || new_x >= grid_width as i32 {
                continue;
            }

            if new_y < 0 || new_y >= grid_height as i32 {
                continue;
            }

            position.x = new_x;
            position.y = new_y;

            let abs_position = position.to_vec3(tile_size);

            transform.translation = abs_position;
        }
    }
}
