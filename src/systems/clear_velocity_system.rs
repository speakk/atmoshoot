use bevy::prelude::*;

use crate::components::{ClearVelocity, Velocity};

pub fn clear_velocity_system(mut query: Query<(&mut Velocity, &ClearVelocity)>) {
    for (mut velocity, _) in query.iter_mut() {
        velocity.0 *= 0.0;
    }
}
