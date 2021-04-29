use crate::components::{MovementIntent, Velocity};
use bevy::prelude::*;

pub fn movement_intent_system(mut query: Query<(&mut MovementIntent, &mut Velocity)>) {
    for (mut movement_intent, mut velocity) in query.iter_mut() {
        velocity.0 = movement_intent.0 * 2.0;
        movement_intent.0 *= 0.0;
    }
}
