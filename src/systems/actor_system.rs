use crate::components::{Actor, ActorType};
use bevy::prelude::*;
use rust_fsm::*;

use crate::state_machines::basic_ai;

pub fn actor_system(mut query: Query<(Entity, &mut Actor)>, mut commands: Commands) {
    for (entity, mut actor) in query.iter_mut() {
        let actor = &mut *actor;
        if let Some(machine) = actor.current_machine.as_mut() {
            let state = machine.state().clone();
            match actor.actor_type {
                ActorType::BasicAi => basic_ai::handle(state, machine, &mut commands, entity),
            }
        }
    }
}

pub fn actor_added_system(mut actor_added: Query<&mut Actor, Added<Actor>>) {
    for mut actor in actor_added.iter_mut() {
        actor.current_machine = Some(StateMachine::new());
    }
}
