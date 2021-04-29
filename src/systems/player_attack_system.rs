use crate::events::PlayerAttackEvent;
use bevy::prelude::*;

use crate::bundles::Bullet;
use crate::components::{MainCamera, Player, Velocity};

pub fn player_attack_system(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(&Player, &Transform)>,
    mut player_attack_event: EventReader<PlayerAttackEvent>,
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = q_camera.iter().next().unwrap();

    for event in player_attack_event.iter() {
        let window = windows.get(event.window_id).unwrap();
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let position = event.click_position - size / 2.0;
        let position_world = camera_transform.compute_matrix() * position.extend(0.0).extend(1.0);

        for (_, player_transform) in query.iter() {
            commands
                .spawn()
                .insert_bundle(Bullet {
                    velocity: Velocity(
                        (position_world.truncate().truncate()
                            - player_transform.translation.truncate())
                        .normalize(),
                    ),
                    ..Default::default()
                })
                .insert(Transform {
                    translation: player_transform.translation,
                    ..Default::default()
                });
        }
    }
}
