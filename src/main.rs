use bevy::prelude::*;
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::WyRand,
};
use systems::{Generation, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::rendering::RenderingPlugin;

mod field;
mod point;
mod systems;
mod cell;
mod rendering;

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
        .add_plugins(RenderingPlugin)
        .insert_resource(Time::<Fixed>::from_hz(32.0))
        .insert_resource(Generation { count: 0 })
        .add_systems(Startup, (systems::initialize_cells, systems::randomize_cells.after(systems::initialize_cells), systems::map_cells.after(systems::randomize_cells)))
        .add_systems(FixedUpdate, (systems::update_cells, systems::update_map_cells.after(systems::update_cells)))
        .run();
}
