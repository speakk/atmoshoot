use crate::state_machines::basic_ai;
use bevy::prelude::Vec2;
use rust_fsm::*;

#[derive(Default)]
pub struct Player;

#[derive(Default)]
pub struct Name(pub &'static str);

pub struct SpriteAdd(pub &'static str);

#[derive(Default)]
pub struct MovementIntent(pub Vec2);

#[derive(Default)]
pub struct Velocity(pub Vec2);

#[derive(Default)]
pub struct ClearVelocity;

#[derive(Default)]
pub struct MainCamera;

#[derive(Default)]
pub struct Spatial {
    pub w: f32,
    pub h: f32,
}

pub enum ActorType {
    BasicAi,
}

pub struct Actor {
    pub actor_type: ActorType,
    pub current_machine: Option<StateMachine<basic_ai::BasicAi>>,
}

pub struct Observer {
    pub range: f32,
}
