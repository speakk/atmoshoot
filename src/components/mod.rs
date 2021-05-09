use bevy::prelude::{Entity, Vec2};

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

#[derive(Default)]
pub struct Observer {
    pub range: f32,
}

pub struct FollowEntity {
    pub target: Entity,
}

#[derive(Default)]
pub struct BasicAi;
