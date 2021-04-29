use bevy::prelude::Vec2;

pub struct Player;
pub struct Name(pub String);
pub struct SpriteAdd(pub &'static str);

#[derive(Default)]
pub struct MovementIntent(pub Vec2);

#[derive(Default)]
pub struct Velocity(pub Vec2);

pub struct ClearVelocity;
pub struct MainCamera;
