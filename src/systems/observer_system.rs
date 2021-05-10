use crate::components::Observer;
use bevy::prelude::*;

use broccoli::{prelude::*, rect};

use crate::SpatialTree;

pub fn observer_system<T: bevy::ecs::component::Component>(
    query: Query<(Entity, &mut Observer<T>, &Transform)>,
    object_query: Query<(Entity, &T)>,
    spatial_tree: Res<SpatialTree>,
) {
    query.for_each_mut(|(_entity, mut observer, transform)| {
        let translation = transform.translation;
        let range = observer.range;
        let new_rect = rect(
            translation.x - range / 2.0,
            translation.x + range / 2.0,
            translation.y - range / 2.0,
            translation.y + range / 2.0,
        );
        spatial_tree.tree.as_tree().for_all_in_rect(&new_rect, |a| {
            let found: Vec<(Entity, &T)> = object_query
                .iter()
                .filter(|(entity, _)| *entity == a.inner)
                .collect();

            if found.len() > 0 {
                let found = found.get(0).unwrap();
                match observer.found_entity {
                    Some(existing) => {
                        if existing != found.0 {
                            observer.found_entity = Some(found.0);
                        }
                    }
                    None => {
                        observer.found_entity = Some(found.0);
                    }
                }
            }
        })
    });
}
