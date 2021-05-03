use crate::components;
use crate::components::{Actor, ActorType, MovementIntent, SpriteAdd, Velocity};
use crate::state_machines::basic_ai;
use bevy::prelude::*;
use rust_fsm::*;

#[derive(Bundle)]
pub struct Bullet {
    pub velocity: Velocity,
    pub sprite_add: SpriteAdd,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            sprite_add: SpriteAdd("bullet.png"),
        }
    }
}

#[derive(Bundle)]
pub struct Player {
    pub player: components::Player,
    pub velocity: Velocity,
    pub movement_intent: MovementIntent,
    pub transform: Transform,
    pub sprite_add: SpriteAdd,
    pub name: components::Name,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            player: components::Player::default(),
            velocity: Velocity::default(),
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
    pub name: components::Name,
    pub actor: Actor,
}

impl Default for BasicEnemy {
    fn default() -> Self {
        Self {
            velocity: Velocity::default(),
            transform: Transform::default(),
            movement_intent: MovementIntent::default(),
            sprite_add: SpriteAdd("enemy.png"),
            name: components::Name("Lurcher"),
            actor: Actor {
                actor_type: ActorType::BasicAi,
                current_machine: None::<StateMachine<basic_ai::BasicAi>>,
            },
        }
    }
}

// impl Bullet {
//     fn new(asset_server: AssetServer) -> Self {
//         let texture_handle = asset_server.load("bullet.png");
//
//         Self {
//             velocity: Velocity(Vec2::new(0.0, 0.0)),
//             sprite: SpriteBundle {
//                 material: materials.add(texture_handle.into()),
//                 ..Default::default()
//             },
//         }
//     }
// }
