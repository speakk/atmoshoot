use bevy::prelude::*;

use crate::components::{BasicAi, FollowEntity};
use crate::events::EntityNoticed;

pub fn basic_ai_notice_entity(
    query: Query<(Entity, &BasicAi)>,
    mut notice_events: EventReader<EntityNoticed>,
    mut commands: Commands,
) {
    for notice_event in notice_events.iter() {
        query
            .iter()
            .filter(|(entity, _)| *entity == notice_event.observer_entity)
            .for_each(|(entity, _)| {
                commands.entity(entity).insert(FollowEntity {
                    target: notice_event.noticed_entity,
                });
            });
    }
}
