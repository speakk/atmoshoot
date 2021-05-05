use bevy::prelude::*;
use rust_fsm::*;

use crate::components::Actor;
use crate::events::EntityNoticed;

state_machine! {
    derive(Debug, Clone)
    pub BasicAi(Idle)
    Idle(NoticeHostile) => Attack,
    Attack(CantSeeHostile) => Idle
}

pub fn handle(state: BasicAiState, machine: &mut StateMachine<BasicAi>) {
    match state {
        BasicAiState::Idle => {
            println!("idle");
            machine.consume(&BasicAiInput::NoticeHostile).unwrap();
        }
        BasicAiState::Attack => (/*println!("Attack!! :O")*/),
    }
}

pub fn basic_ai_system(
    mut entity_noticed: EventReader<EntityNoticed>,
    mut query: Query<(Entity, &mut Actor)>,
) {
    for event in entity_noticed.iter() {
        for (entity, mut actor) in query.iter_mut() {
            let actor = &mut *actor;
            if entity == event.observer_entity {
                actor
                    .current_machine
                    .as_mut()
                    .unwrap()
                    .consume(&BasicAiInput::NoticeHostile)
                    .unwrap();
            }
        }
    }
}
