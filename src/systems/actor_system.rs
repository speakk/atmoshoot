use crate::components::{Actor, ActorType};
use bevy::prelude::*;
use rust_fsm::*;

use crate::state_machines::basic_ai;

pub fn actor_system(mut query: Query<&mut Actor>) {
    // for mut actor in query.iter_mut() {
    //     if let Some(mut machine) = actor.current_machine.as_mut() {
    //         //basic_ai::handle(&machine.state(), &mut machine);
    //         let state = machine.state().clone();
    //         match actor.actor_type {
    //             ActorType::BasicAi => basic_ai::handle(state, machine),
    //         }
    //     }
    // }
    for mut actor in query.iter_mut() {
        match actor.actor_type {
            ActorType::BasicAi => {
                if let Some(mut machine) = actor.current_machine.as_mut() {
                    let state = machine.state().clone();
                    basic_ai::handle(state, machine);
                }
            }
        }
    }
}

// pub fn map_to_handler(
//     actor_type: ActorType,
// ) -> fn(state: basic_ai::BasicAiState, machine: StateMachine<basic_ai::BasicAi>) {
//     match actor_type {
//         ActorType::BasicAi => basic_ai::handle,
//     }
// }

pub fn actor_added_system(mut actor_added: Query<&mut Actor, Added<Actor>>) {
    for mut actor in actor_added.iter_mut() {
        actor.current_machine = Some(StateMachine::new());
    }
}
