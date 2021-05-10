use bevy::prelude::*;

use crate::components::{BasicAi, FollowEntity, Observer};

pub fn basic_ai_notice_entity<T: bevy::ecs::component::Component>(
    query: Query<(Entity, &BasicAi, &Observer<T>), Changed<Observer<T>>>,
    mut commands: Commands,
) {
    query.for_each(|(entity, _, observer)| {
        println!("Ha, got ya");
        if let Some(target) = observer.found_entity {
            commands.entity(entity).insert(FollowEntity { target });
        }
    });
}
