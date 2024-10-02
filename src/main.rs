use bevy::prelude::*;
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 400;
const RANDOM_SEED: u64 = 42;

#[derive(Component)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
struct Cell {
    alive: bool,
}

#[derive(Resource)]
struct Field {
    width: i32,
    height: i32,
    cells: Vec<Option<Entity>>,
}

impl Field {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cells: vec![None; (width * height) as usize],
        }
    }

    pub fn insert(&mut self, point: Point, entity: Entity) {
        let ind = (point.y * self.width) + point.x;
        if ind < self.width * self.height {
            self.cells.insert(ind as usize, Some(entity));
        } else {
            warn!("Point out of bounds!")
        }
    }

    pub fn get(&self, point: Point) -> Option<Entity> {
        let ind = (point.y * self.width) + point.x;
        if ind < self.width * self.height {
            self.cells[ind as usize]
        } else {
            warn!("Point out of bounds!");
            None
        }
    }
}

fn initialize_cells(mut commands: Commands, mut rng: ResMut<GlobalEntropy<WyRand>>) {
   let mut field = Field::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    for x in 0..(WINDOW_WIDTH - 1) {
        for y in 0..(WINDOW_HEIGHT - 1) {
            info!("Spawning cell at {}, {}", x, y);
            let alive = if rng.next_u32() % 7 == 0 { true } else { false };
            field.insert(Point { x, y },
                         commands.spawn(
                             Point { x, y })
                             .insert(Cell {
                                 alive
                             })
                             .insert(NodeBundle {
                                 style: Style {
                                     position_type: PositionType::Absolute,
                                     left: Val::Px(x as f32),
                                     top: Val::Px(y as f32),
                                     width: Val::Px(1.0),
                                     height: Val::Px(1.0),
                                     ..default()
                                 },
                                 background_color: BackgroundColor(Color::BLACK),
                                 visibility: if alive { Visibility::Visible } else { Visibility::Hidden },
                                 ..default()
                             })
                             .id());
        }
    }    commands.insert_resource(field);
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
