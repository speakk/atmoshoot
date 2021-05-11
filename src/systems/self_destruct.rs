use bevy::prelude::*;

use crate::components::SelfDestruct;

pub fn self_destruct(
    mut timers_query: Query<(Entity, &mut SelfDestruct)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut self_destruct) in timers_query.iter_mut() {
        if self_destruct.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
