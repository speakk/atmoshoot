use bevy::prelude::*;
use rand::Rng;

mod components;
use components::{MainCamera, MovementIntent, Name, Player, Velocity};

mod systems;
use systems::*;

mod events;
use events::*;

mod bundles;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_system(add_people.system());
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
        app.add_system(player_attack_system.system());
        app.add_system(sprite_add_system.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn add_people(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 1..2 {
        let texture_handle = asset_server.load("textures/player.png");
        commands
            .spawn()
            .insert(Player)
            .insert(Velocity::default())
            .insert(MovementIntent::default())
            .insert_bundle(SpriteBundle {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_xyz(
                    rng.gen::<f32>() * 10.0,
                    rng.gen::<f32>() * 10.0,
                    rng.gen::<f32>() * 10.0,
                ),
                ..Default::default()
            })
            .insert(Name("spok".to_string()));
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.07, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}