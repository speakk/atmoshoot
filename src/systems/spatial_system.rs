use bevy::prelude::*;
//use broccoli::{bbox, prelude::*, rect};
use broccoli::{bbox, container::*, node::BBox, rect};

use crate::components::Spatial;
use crate::SpatialTree;

pub fn spatial_system(
    query: Query<(Entity, &Spatial, &Transform)>,
    mut spatial_tree: ResMut<SpatialTree>,
) {
    let spatial_entities: Vec<BBox<f32, Entity>> = query
        .iter()
        .map(|(entity, spatial, transform)| {
            let translation = transform.translation;
            let new_rect = rect(
                translation.x,
                translation.x + spatial.w,
                translation.y,
                translation.y + spatial.h,
            );
            bbox(new_rect, entity)
        })
        .collect();

    spatial_tree.tree = TreeOwned::new(spatial_entities.into_boxed_slice());
}
