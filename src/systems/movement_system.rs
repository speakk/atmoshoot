use crate::components::Velocity;
use bevy::prelude::*;

pub fn movement_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        *translation += velocity.0.extend(0.0) * time.delta_seconds() * 300.0;
    }
}
