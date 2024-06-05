use bevy::{prelude::*, window::WindowResized};

use liaise::{
    debug::DebugPlugin,
    drone::{move_drones, setup_drone, MoveEvent},
    grid::{create_grid, GridSizes},
    tiles::{reached_target, setup_target},
};

const GRID_WIDTH: u32 = 5;
const GRID_HEIGHT: u32 = 1;

const TILE_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DebugPlugin))
        .insert_resource(GridSizes::new(GRID_WIDTH, GRID_HEIGHT, TILE_SIZE))
        .add_event::<MoveEvent>()
        .add_systems(
            Startup,
            (create_camera, create_grid, setup_drone, setup_target),
        )
        .add_systems(Update, (resize, reached_target, move_drones))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn create_camera(mut commands: Commands) {
    let x = GRID_WIDTH as f32 * TILE_SIZE / 2.0;
    let y = GRID_HEIGHT as f32 * TILE_SIZE / 2.0;

    commands.spawn((
        MainCamera,
        Camera2dBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
    ));
}

fn resize(
    mut resize_events: EventReader<WindowResized>,
    grid_sizes: Res<GridSizes>,
    mut query: Query<&mut Transform>,
) {
    if let Some(resize_event) = resize_events.read().last() {
        let width_ratio = resize_event.width / grid_sizes.grid_width as f32;
        let height_ratio = resize_event.height / grid_sizes.grid_height as f32;

        let ratio = width_ratio.min(height_ratio) / (7.0 * grid_sizes.tile_size);

        for mut transform in query.iter_mut() {
            transform.scale = Vec3::splat(ratio);
        }
    }
}
