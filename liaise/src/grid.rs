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

    let white = materials.add(Color::WHITE);

    let mut lines = Vec::new();

    for row in 0..=grid_height {
        let y = row as f32 * tile_size;

        lines.push((
            Vec3::new(0.0, y, 0.0),
            Vec3::new(grid_width as f32 * tile_size, y, 0.0),
        ));
    }

    for col in 0..=grid_width {
        let x = col as f32 * tile_size;

        lines.push((
            Vec3::new(x, 0.0, 0.0),
            Vec3::new(x, grid_height as f32 * tile_size, 0.0),
        ));
    }

    let mesh = meshes.add(LineList { lines });

    commands.spawn((
        Grid,
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: white.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}
