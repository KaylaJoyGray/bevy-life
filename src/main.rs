use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 400;
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
    node: NodeBundle,
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
        if let Some(entref) = self.cells.get(&point) {
            Some(*entref)
        } else {
            None
        }
    }
}

fn initialize_cells(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
    let mut field = Field::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let cells: Vec<CellBundle> = (0..WINDOW_WIDTH * WINDOW_HEIGHT).filter(|_| { rng.next_u32() % 7 == 0 }).map(|i| {
        let x = i % WINDOW_WIDTH;
        let y = (i -x) / WINDOW_WIDTH;
        CellBundle {
            cell: Cell {},
            point: Point {
                x,
                y: (i - x) / WINDOW_WIDTH,
            },
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x as f32),
                    top: Val::Px(y as f32),
                    width: Val::Px(1.0),
                    height: Val::Px(1.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        }
    }).collect();
    
    commands.spawn_batch(cells);

    commands.insert_resource(field);
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
        .add_systems(Startup, (spawn_camera, initialize_cells.after(spawn_camera)))
        .run();
}
