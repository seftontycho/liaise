use bevy::prelude::*;

use liaise::{
    drone::setup_drone,
    grid::{create_grid, GridSizes},
};

const GRID_WIDTH: u32 = 5;
const GRID_HEIGHT: u32 = 5;

const TILE_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GridSizes::new(GRID_WIDTH, GRID_HEIGHT, TILE_SIZE))
        .add_systems(Startup, (create_camera, create_grid, setup_drone))
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
