use std::ops::Add;
use bevy::{
    prelude::*,
};
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::GlobalEntropy,
    prelude::WyRand,
};
use rand_core::RngCore;

const WINDOW_WIDTH:  usize = 1920;
const WINDOW_HEIGHT: usize = 1080;
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

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq, Default)]
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
struct CellState {
    alive: bool,
}

#[derive(Bundle)]
struct CellBundle {
    cell: CellState,
    point: Point,
}

#[derive(Resource)]
struct Generation {
    count: i32,
}

#[derive(Resource)]
struct Field {
    width: usize,
    height: usize,
    cells: Vec<u64>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0; (width * height) / 64],
        }
    }

    #[inline(always)]
    pub fn flip(&mut self, point: Point) {
        if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
            let offset = ((point.y * self.width as i32) + point.x) as usize;
            let mask = 1u64 << (offset % 64);
            self.cells[offset / 64] ^= mask;
        } else {
            warn!("Point out of bounds!")
        }
    }

    #[inline(always)]
    pub fn get(&self, point: Point) -> bool {
        if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
            let offset = ((point.y * self.width as i32) + point.x) as usize;
            let mask = 1u64 << (offset % 64);
            self.cells[offset / 64] & mask != 0
        } else {
            false
        }
    }
}

fn initialize_cells(mut commands: Commands) {
    let cells: Vec<CellBundle> = (0..WINDOW_WIDTH * WINDOW_HEIGHT).map(|i| {
        let x = (i % WINDOW_WIDTH) as i32;
        let y = ((i - x as usize) / WINDOW_WIDTH) as i32;
        CellBundle {
            cell: CellState {
                alive: true,
            },
            point: Point {
                x,
                y,
            },
        }
    }).collect();

    commands.spawn_batch(cells);

    info!("Spawned cells!");
}

fn map_cells(mut commands: Commands,
             cells: Query<(&Point, &CellState)>) {
    let mut field = Field::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    cells.iter().for_each(|(p, c)| {
        if c.alive {
            field.flip(*p);
        }
    });
    commands.insert_resource(field);
}

fn update_map_cells(cells: Query<&Point, Changed<CellState>>,
                    mut field: ResMut<Field>) {
    cells.iter().for_each(|p| {
        field.flip(*p);
    });
}

fn randomize_cells(mut rng: ResMut<GlobalEntropy<WyRand>>, mut cells_query: Query<&mut CellState>) {
    cells_query.iter_mut().filter(|_| {
        rng.next_u32() % 7 == 0
    }).for_each(|mut c| {
        c.alive = true;
    });

    info!("Randomized cells!");
}

fn update_cells(mut cells: Query<( &Point, &mut CellState)>,
                field: Res<Field>,
                mut generation: ResMut<Generation>) {
    cells.par_iter_mut().for_each(|(p, mut c)| {
        let neighbors = p.neighbors().iter().filter(|&&n| {
            field.get(n)
        }).count();

        c.alive = matches!(neighbors, 2|3);
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
        .add_systems(FixedUpdate, (update_cells, update_map_cells.after(update_cells)))
        .run();
}
