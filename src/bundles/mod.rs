use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Bullet {
    pub velocity: Velocity,
    pub sprite_add: SpriteAdd,
    //#[bundle]
    //sprite: SpriteBundle,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            sprite_add: SpriteAdd("bullet.png"),
        }
    }
}

// impl Bullet {
//     fn new(asset_server: AssetServer) -> Self {
//         let texture_handle = asset_server.load("bullet.png");
//
//         Self {
//             velocity: Velocity(Vec2::new(0.0, 0.0)),
//             sprite: SpriteBundle {
//                 material: materials.add(texture_handle.into()),
//                 ..Default::default()
//             },
//         }
//     }
// }
