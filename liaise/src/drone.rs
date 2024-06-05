use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::grid::GridSizes;

pub fn setup_drone(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
) {
    let position = Position::new(2, 2);

    create_drone_at(commands, meshes, materials, res, position)
}

#[derive(Component)]
struct Drone;

pub fn create_drone_at(
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
        Drone,
        position,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: green,
            transform: Transform::from_translation(abs_position),
            ..Default::default()
        },
    ));
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_vec3(&self, tile_size: f32) -> Vec3 {
        Vec3::new(
            self.x as f32 * tile_size + tile_size / 2.0,
            self.y as f32 * tile_size + tile_size / 2.0,
            0.0,
        )
    }
}
