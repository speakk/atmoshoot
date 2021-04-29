use bevy::prelude::*;

use crate::components::{MovementIntent, Player};
use crate::events::{Direction, PlayerMoveEvent};

pub fn player_movement_system(
    mut query: Query<(&Player, &mut MovementIntent)>,
    mut player_move_event: EventReader<PlayerMoveEvent>,
) {
    let mut direction = Vec2::new(0.0, 0.0);

    for event in player_move_event.iter() {
        match event.direction {
            Direction::Left => direction.x = -1.0,
            Direction::Right => direction.x = 1.0,
            Direction::Down => direction.y = -1.0,
            Direction::Up => direction.y = 1.0,
        }
    }

    for (_, mut movement_intent) in query.iter_mut() {
        movement_intent.0.x = direction.x;
        movement_intent.0.y = direction.y;
    }
}
