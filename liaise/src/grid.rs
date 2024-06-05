use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    sprite::MaterialMesh2dBundle,
};

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

#[derive(Component)]
pub struct Grid;

#[derive(Resource)]
pub struct GridSizes {
    pub grid_width: u32,
    pub grid_height: u32,
    pub tile_size: f32,
}

impl GridSizes {
    pub fn new(grid_width: u32, grid_height: u32, tile_size: f32) -> Self {
        Self {
            grid_width,
            grid_height,
            tile_size,
        }
    }
}

pub fn create_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res: Res<GridSizes>,
) {
    let GridSizes {
        grid_width,
        grid_height,
        tile_size,
    } = *res;

    for row in 0..grid_height {
        for col in 0..grid_width {
            create_grid_tile_at(
                &mut commands,
                &mut meshes,
                &mut materials,
                tile_size,
                Position::new(col as i32, row as i32),
            )
        }
    }
}

#[derive(Component)]
struct GridTile;

pub fn create_grid_tile_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    tile_size: f32,
    position: Position,
) {
    let white = materials.add(Color::WHITE);

    let half_size = tile_size / 2.0;

    let lines = LineList {
        lines: vec![
            (
                Vec3::new(-half_size, -half_size, 0.0),
                Vec3::new(half_size, -half_size, 0.0),
            ),
            (
                Vec3::new(half_size, -half_size, 0.0),
                Vec3::new(half_size, half_size, 0.0),
            ),
            (
                Vec3::new(half_size, half_size, 0.0),
                Vec3::new(-half_size, half_size, 0.0),
            ),
            (
                Vec3::new(-half_size, half_size, 0.0),
                Vec3::new(-half_size, -half_size, 0.0),
            ),
        ],
    };

    let mesh = meshes.add(lines);

    let abs_position = position.to_vec3(tile_size);

    commands.spawn((
        GridTile,
        position,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: white,
            transform: Transform::from_translation(abs_position),
            ..Default::default()
        },
    ));
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy)]
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
