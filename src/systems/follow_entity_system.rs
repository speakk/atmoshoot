use bevy::prelude::*;

use crate::components::{FollowEntity, MovementIntent};

pub fn follow_entity_system(
    follow_entity_query: Query<(&Transform, &mut MovementIntent, &FollowEntity)>,
    query: Query<&Transform>,
) {
    const DISTANCE_TOLERANCE: f32 = 3.0;

    follow_entity_query.for_each_mut(|(transform, mut movement_intent, follow_entity)| {
        let target = follow_entity.target;
        let target_transform = query.get(target).unwrap();
        let difference = target_transform.translation - transform.translation;
        if difference.length() < DISTANCE_TOLERANCE {
            movement_intent.0 = Vec2::ZERO;
        } else {
            movement_intent.0 = (target_transform.translation - transform.translation)
                .truncate()
                .normalize_or_zero();
        }
    });
}
