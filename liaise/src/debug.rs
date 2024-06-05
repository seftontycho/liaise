use bevy::prelude::*;

use crate::drone::MoveEvent;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut move_events: EventWriter<MoveEvent>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keys.just_pressed(KeyCode::KeyW) {
        move_events.send(MoveEvent { direction: Vec2::Y });
    }

    if keys.just_pressed(KeyCode::KeyA) {
        move_events.send(MoveEvent {
            direction: Vec2::NEG_X,
        });
    }

    if keys.just_pressed(KeyCode::KeyS) {
        move_events.send(MoveEvent {
            direction: Vec2::NEG_Y,
        });
    }

    if keys.just_pressed(KeyCode::KeyD) {
        move_events.send(MoveEvent { direction: Vec2::X });
    }
}
