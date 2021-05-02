use bevy::prelude::*;

use crate::components::{Actor, ActorType};

pub fn actor_system(query: Query<&Actor>) {
    for actor in query.iter() {
        match actor.0 {
            ActorType::BasicAi => (),
        }
    }
}
