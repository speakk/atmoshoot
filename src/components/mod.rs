use bevy::prelude::Vec2;

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

pub enum ActorType {
    BasicAi,
}

pub struct Actor(pub ActorType);
