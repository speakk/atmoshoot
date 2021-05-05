use crate::components::Observer;
use bevy::prelude::*;

use broccoli::{prelude::*, rect};

use crate::SpatialTree;

pub fn observer_system(query: Query<(&Observer, &Transform)>, spatial_tree: Res<SpatialTree>) {
    query.for_each(|(observer, transform)| {
        let translation = transform.translation;
        let range = observer.range;
        let new_rect = rect(
            translation.x - range / 2.0,
            translation.x + range / 2.0,
            translation.y - range / 2.0,
            translation.y + range / 2.0,
        );
        spatial_tree
            .tree
            .as_tree()
            .for_all_in_rect(&new_rect, |a| println!("INTERSECT {:?}", a))
    });
}
