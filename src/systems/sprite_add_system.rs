use bevy::prelude::*;
use bevy::render::pipeline::RenderPipeline;
use bevy::render::render_graph::base::MainPass;
use bevy::sprite::{QUAD_HANDLE, SPRITE_PIPELINE_HANDLE};
use std::path::Path;

use crate::components::SpriteAdd;

pub fn sprite_add_system(
    mut query: Query<(Entity, &mut SpriteAdd)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, sprite_add) in query.iter_mut() {
        let texture_handle =
            asset_server.load(Path::new(format!("textures/{}", sprite_add.0).as_str()));
        commands
            .entity(entity)
            .insert(QUAD_HANDLE.typed::<Mesh>())
            .insert(RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                SPRITE_PIPELINE_HANDLE.typed(),
            )]))
            .insert(Visible {
                is_transparent: true,
                ..Default::default()
            })
            .insert(MainPass)
            .insert(Draw::default())
            .insert(Sprite::default())
            .insert(materials.add(texture_handle.into()))
            .insert(GlobalTransform::default())
            .remove::<SpriteAdd>();
    }
}
