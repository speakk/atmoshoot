use bevy::prelude::*;
use rust_fsm::*;

use crate::components::{Actor, ActorType};
use crate::events::EntityNoticed;

// TODO:
// in NoticeHostile somehow pass the noticed entity to the state. Or... Do we store this state in
// BasicAiState instead.... Hmm, so have BasicAiState be the blackboard

state_machine! {
    derive(Debug, Clone)
    pub BasicAi(Idle)
    Idle(NoticeHostile) => EnterAttack,
    EnterAttack(NoticeHostile) => Attack,
    Attack(NoticeHostile) => Attack,
    Attack(CantSeeHostile) => Idle
}

pub fn handle(
    state: BasicAiState,
    machine: &mut StateMachine<BasicAi>,
    mut commands: &mut Commands,
    entity: Entity,
) {
    match state {
        BasicAiState::Idle => {
            println!("idle");
            //machine.consume(&BasicAiInput::NoticeHostile).unwrap();
        }
        BasicAiState::EnterAttack => {
            //commands.entity(entity).insert(Follow

            machine.consume(&BasicAiInput::NoticeHostile).unwrap();
        }
        BasicAiState::Attack => (println!("Attack!! :O")),
    }
}

pub fn basic_ai_system(
    mut entity_noticed: EventReader<EntityNoticed>,
    mut query: Query<(Entity, &mut Actor)>,
) {
    for event in entity_noticed.iter() {
        query
            .iter_mut()
            .filter(|(entity, _)| *entity == event.observer_entity)
            .for_each(|(entity, mut actor)| {
                let actor = &mut *actor;
                actor
                    .current_machine
                    .as_mut()
                    .unwrap()
                    .consume(&BasicAiInput::NoticeHostile)
                    .unwrap();
            });
    }
}
