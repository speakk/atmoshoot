use bevy::prelude::*;

use crate::components::{FollowEntity, MovementIntent};

pub fn follow_entity_system(
    follow_entity_query: Query<(&Transform, &mut MovementIntent, &FollowEntity)>,
    query: Query<&Transform>,
) {
    follow_entity_query.for_each_mut(|(transform, mut movement_intent, follow_entity)| {
        let target = follow_entity.target;
        let target_transform = query.get(target).unwrap();
        movement_intent.0 = (target_transform.translation - transform.translation)
            .truncate()
            .normalize_or_zero();
    });
}
