use crate::components;
use crate::components::{BasicAi, MovementIntent, Observer, Spatial, SpriteAdd, Velocity};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Bullet {
    pub velocity: Velocity,
    pub spatial: Spatial,
    pub sprite_add: SpriteAdd,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            spatial: Spatial { w: 5.0, h: 5.0 },
            sprite_add: SpriteAdd("bullet.png"),
        }
    }
}

#[derive(Bundle)]
pub struct Player {
    pub player: components::Player,
    pub velocity: Velocity,
    pub movement_intent: MovementIntent,
    pub spatial: Spatial,
    pub transform: Transform,
    pub sprite_add: SpriteAdd,
    pub name: components::Name,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            player: components::Player::default(),
            velocity: Velocity::default(),
            spatial: Spatial { w: 10.0, h: 10.0 },
            movement_intent: MovementIntent::default(),
            sprite_add: SpriteAdd("player.png"),
            transform: Transform::default(),
            name: components::Name("Mook"),
        }
    }
}

#[derive(Bundle)]
pub struct BasicEnemy {
    pub velocity: Velocity,
    pub movement_intent: MovementIntent,
    pub sprite_add: SpriteAdd,
    pub transform: Transform,
    pub observer: Observer,
    pub name: components::Name,
    pub basic_ai: BasicAi,
}

impl Default for BasicEnemy {
    fn default() -> Self {
        Self {
            velocity: Velocity::default(),
            transform: Transform::default(),
            observer: Observer { range: 300.0 },
            movement_intent: MovementIntent::default(),
            sprite_add: SpriteAdd("enemy.png"),
            name: components::Name("Lurcher"),
            basic_ai: BasicAi::default(),
        }
    }
}
