use bevy::prelude::{Entity, Timer, Vec2};

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
pub struct Observer<T: bevy::ecs::component::Component> {
    pub range: f32,
    pub found_entity: Option<Entity>,
    pub phantom: std::marker::PhantomData<T>,
}

pub struct FollowEntity {
    pub target: Entity,
}

#[derive(Default)]
pub struct BasicAi;

#[derive(Default)]
pub struct SelfDestruct {
    pub timer: Timer,
}
