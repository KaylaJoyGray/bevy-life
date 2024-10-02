use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const RANDOM_SEED: u64 = 42;

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
struct Cell {}

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    point: Point,
}

#[derive(Resource)]
struct Field {
    width: i32,
    height: i32,
    cells: HashMap<Point, Entity>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: HashMap::new(),
        }
    }

    pub fn insert(&mut self, point: Point, entity: Entity) {
        if point.x < self.width && point.y < self.height {
            self.cells.insert(point, entity);
        } else {
            warn!("Point out of bounds!")
        }
    }

    pub fn get(&self, point: Point) -> Option<Entity> {
        if let Some(e) = self.cells.get(&point) {
            Some(*e)
        } else {
            None
        }
    }
}

fn initialize_cells(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
    let cells: Vec<CellBundle> = (0..WINDOW_WIDTH * WINDOW_HEIGHT).filter(|_| { rng.next_u32() % 7 == 0 }).map(|i| {
        let x = i % WINDOW_WIDTH;
        let y = (i - x) / WINDOW_WIDTH;
        CellBundle {
            cell: Cell {},
            point: Point {
                x,
                y,
            },
        }
    }).collect();

    commands.spawn_batch(cells);

    info!("Spawned cells!");
}

fn map_cells(mut commands: Commands, cells: Query<(Entity, &Point), With<Cell>>) {
    commands.insert_resource(Field {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        cells: cells.iter().map(|(e, p)| {
            (*p, e)
        }).collect::<HashMap<Point, Entity>>(),
    });

    info!("Mapped cells!");
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

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
        .add_systems(Startup, (spawn_camera, initialize_cells.after(spawn_camera), map_cells.after(initialize_cells)))
        .run();
}
