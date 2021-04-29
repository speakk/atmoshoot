use bevy::prelude::*;
use bevy::window::WindowId;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerMoveEvent {
    pub direction: Direction,
}

pub struct PlayerAttackEvent {
    pub click_position: Vec2,
    pub window_id: WindowId,
}
