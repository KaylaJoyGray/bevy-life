use bevy::prelude::*;
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::WyRand,
};
use systems::{Generation, WINDOW_HEIGHT, WINDOW_WIDTH};

mod field;
mod point;
mod systems;
mod cell;

const RANDOM_SEED: u64 = 42;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Life".into(),
                resolution: (WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::with_seed(RANDOM_SEED.to_ne_bytes()))
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Generation { count: 0 })
        .add_systems(Startup, (systems::spawn_camera, systems::initialize_cells.after(systems::spawn_camera), systems::randomize_cells.after(systems::initialize_cells), systems::map_cells.after(systems::randomize_cells)))
        .add_systems(FixedUpdate, (systems::update_cells, systems::update_map_cells.after(systems::update_cells)))
        .run();
}
