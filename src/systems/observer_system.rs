use crate::components::Observer;
use crate::events::EntityNoticed;
use bevy::prelude::*;

use broccoli::{prelude::*, rect};

use crate::SpatialTree;

pub fn observer_system(
    query: Query<(Entity, &Observer, &Transform)>,
    spatial_tree: Res<SpatialTree>,
    mut observer_event: EventWriter<EntityNoticed>,
) {
    query.for_each(|(entity, observer, transform)| {
        let translation = transform.translation;
        let range = observer.range;
        let new_rect = rect(
            translation.x - range / 2.0,
            translation.x + range / 2.0,
            translation.y - range / 2.0,
            translation.y + range / 2.0,
        );
        spatial_tree.tree.as_tree().for_all_in_rect(&new_rect, |a| {
            observer_event.send(EntityNoticed {
                noticed_entity: a.inner,
                observer_entity: entity,
            });
        })
    });
}
