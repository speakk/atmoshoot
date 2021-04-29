use bevy::prelude::*;

use crate::events::{Direction, PlayerAttackEvent, PlayerMoveEvent};

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut player_move_event: EventWriter<PlayerMoveEvent>,
    mut player_attack_event: EventWriter<PlayerAttackEvent>,
) {
    if keyboard_input.pressed(KeyCode::A) {
        player_move_event.send(PlayerMoveEvent {
            direction: Direction::Left,
        });
    } else if keyboard_input.pressed(KeyCode::D) {
        player_move_event.send(PlayerMoveEvent {
            direction: Direction::Right,
        });
    }

    if keyboard_input.pressed(KeyCode::W) {
        player_move_event.send(PlayerMoveEvent {
            direction: Direction::Up,
        });
    } else if keyboard_input.pressed(KeyCode::S) {
        player_move_event.send(PlayerMoveEvent {
            direction: Direction::Down,
        });
    }

    if mouse_button.pressed(MouseButton::Left) {
        let cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
        player_attack_event.send(PlayerAttackEvent {
            click_position: cursor_position,
            window_id: windows.get_primary().unwrap().id(),
        });
    }
}
