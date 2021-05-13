use bevy::prelude::*;

mod components;
use components::{MainCamera, Player};

mod systems;
use systems::*;

mod events;
use events::*;

mod bundles;

use broccoli::{container::*, node::BBox};

pub struct GamePlugin;

pub struct SpatialTree {
    tree: TreeOwned<BBox<f32, Entity>>,
}

impl Default for SpatialTree {
    fn default() -> Self {
        let a = vec![].into_boxed_slice();
        Self {
            tree: TreeOwned::new(a),
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_system(add_people.system());
        app.init_resource::<SpatialTree>();
        app.add_event::<EntityNoticed>();
        app.add_event::<PlayerMoveEvent>();
        app.add_event::<PlayerAttackEvent>();
        app.add_system(input_system.system().label("input"));
        app.add_system(
            player_movement_system
                .system()
                .label("player_movement")
                .after("input"),
        );
        app.add_system(
            movement_intent_system
                .system()
                .label("movement_intent")
                .after("player_movement"),
        );
        app.add_system(
            movement_system
                .system()
                .label("movement_system")
                .after("movement_intent"),
        );
        app.add_system(clear_velocity_system.system().after("movement_system"));
        app.add_system(
            spatial_system
                .system()
                .label("spatial")
                .after("movement_system"),
        );
        app.add_system(collision_system.system().after("spatial"));
        app.add_system(observer_system::<Player>.system());
        app.add_system(follow_entity_system.system());
        app.add_system(self_destruct.system());
        app.add_system(player_attack_system.system());
        app.add_system(sprite_add_system.system());
        app.add_system(basic_ai_notice_entity::<Player>.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn add_people(mut commands: Commands) {
    commands.spawn_bundle(bundles::Player::default());
    commands.spawn_bundle(bundles::BasicEnemy::default());
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.07, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
