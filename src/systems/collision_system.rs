use crate::SpatialTree;
use bevy::prelude::*;

use broccoli::{prelude::*, rect};

pub fn collision_system(
    mut spatial_tree: ResMut<SpatialTree>,
    //mut collision_events: EventWriter<CollisionEvent>,
) {
    spatial_tree
        .tree
        .as_tree_mut()
        .find_colliding_pairs_mut(|a, b| {});

    //find_colliding_pairs(|a, b| {});
}
