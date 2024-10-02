use std::collections::HashMap;
use std::ops::Add;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;
const RANDOM_SEED: u64 = 42;

const NEIGHBOR_COORDINATES_8: [Point; 8] = [
    // Left
    Point {
        x: 1,
        y: 0,
    },
    // Top Left
    Point {
        x: -1,
        y: 1,
    },
    // Top
    Point {
        x: 0,
        y: 1,
    },
    // Top Right
    Point {
        x: 1,
        y: 1,
    },
    // Right
    Point {
        x: 1,
        y: 0,
    },
    // Bottom Right
    Point {
        x: 1,
        y: -1,
    },
    // Bottom
    Point {
        x: 0,
        y: -1,
    },
    // Bottom Left
    Point {
        x: -1,
        y: -1,
    },
];

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        NEIGHBOR_COORDINATES_8.map(|p| {
            *self + p
        }).into_iter().collect()
    }
}

#[derive(Component)]
struct Cell {}

#[derive(Component)]
struct Alive {}

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    point: Point,
}

#[derive(Resource)]
struct Generation {
    count: i32,
}

#[derive(Resource)]
struct Field {
    width: i32,
    height: i32,
    cells: HashSet<Point>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: HashSet::new(),
        }
    }

    pub fn insert(&mut self, point: Point) {
        if point.x < self.width && point.y < self.height {
            self.cells.insert(point);
        } else {
            warn!("Point out of bounds!")
        }
    }

    pub fn get(&self, point: Point) -> bool {
        if let Some(e) = self.cells.get(&point) {
            true
        } else {
            false
        }
    }
}

fn initialize_cells(mut commands: Commands) {
    let cells: Vec<CellBundle> = (0..WINDOW_WIDTH * WINDOW_HEIGHT).map(|i| {
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

fn map_cells(mut commands: Commands, cells: Query<&Point, (With<Cell>, With<Alive>)>) {
    commands.insert_resource(Field {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        cells: cells.iter().map(|p| {
            *p
        }).collect::<HashSet<Point>>(),
    });

    info!("Mapped cells!");
}

fn randomize_cells(mut commands: Commands,
                   mut rng: ResMut<GlobalEntropy<WyRand>>,
                   cells_query: Query<Entity, With<Cell>>) {
    cells_query.iter().filter(|_| {
        rng.next_u32() % 7 == 0
    }).for_each(|e| {
        commands.entity(e).insert(Alive {});
    });

    info!("Randomized cells!");
}

fn update_cells(par_commands: ParallelCommands,
                cells: Query<(Entity, &Point), With<Cell>>,
                field: Res<Field>,
                mut generation: ResMut<Generation>) {
    cells.par_iter().for_each(|(e, p)| {
        let neighbors = p.neighbors().iter().filter(|&&n| {
            field.get(n)
        }).count();

        match neighbors {
            0..2 => {
                par_commands.command_scope(|mut commands| {
                    commands.entity(e).remove::<Alive>();
                })
            }
            2..3 => {
                // We're alive!
            }
            3 => {
                par_commands.command_scope(|mut commands| {
                    commands.entity(e).insert(Alive {});
                })
            }
            4..=8 => {
                par_commands.command_scope(|mut commands| {
                    commands.entity(e).remove::<Alive>();
                })
            }
            _ => {}
        }
    });

    generation.count += 1;
    info!("Generation {} passed!", generation.count);
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
        .insert_resource(Generation { count: 0 })
        .add_systems(Startup, (spawn_camera, initialize_cells.after(spawn_camera), map_cells.after(initialize_cells), randomize_cells.after(map_cells)))
        .add_systems(FixedUpdate, (update_cells, map_cells.after(update_cells)))
        .run();
}
