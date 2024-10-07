use crate::cell::{CellBundle, CellState};
use crate::field::Field;
use crate::point::Point;
use bevy::log::info;
use bevy::prelude::{Camera2dBundle, Changed, Commands, IsDefaultUiCamera, Query, Res, ResMut, Resource};
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand_core::RngCore;

pub const WINDOW_WIDTH: usize = 1920;
pub const WINDOW_HEIGHT: usize = 1080;

pub fn initialize_cells(mut commands: Commands) {
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

pub fn randomize_cells(mut rng: ResMut<GlobalEntropy<WyRand>>, mut cells_query: Query<&mut CellState>) {
    cells_query.iter_mut().filter(|_| {
        rng.next_u32() % 7 == 0
    }).for_each(|mut c| {
        c.alive = true;
    });

    info!("Randomized cells!");
}

pub fn map_cells(mut commands: Commands,
                 cells: Query<(&Point, &CellState)>) {
    let field = Field::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    cells.iter().for_each(|(p, c)| {
        if c.alive {
            field.flip(*p);
        }
    });
    commands.insert_resource(field);
}

pub fn update_map_cells(cells: Query<&Point, Changed<CellState>>,
                        field: ResMut<Field>,
                        /*mut generation: Local<u32>,*/
                        /*mut duration: Local<Duration>*/) {
    // let start = Instant::now();

    cells.par_iter().for_each(|p| {
        field.flip(*p);
    });

    // *duration += start.elapsed();
    // *generation += 1;
    // if *generation % 1000 == 0 {
    //     info!("update_map_cells: Average time: {}ns", duration.as_nanos() / *generation as u128);
    // }
}

pub fn update_cells(mut cells: Query<(&Point, &mut CellState)>,
                    field: Res<Field>,
                    mut generation: ResMut<Generation>,
                    /*mut duration: Local<Duration>*/) {
    // let start = Instant::now();
    cells.par_iter_mut().for_each(|(p, mut c)| {
        let neighbors = p.neighbors().iter().filter(|&&n| {
            field.get(n)
        }).count();

        c.alive = matches!(neighbors, 2|3);
    });

    generation.count += 1;
    // *duration += start.elapsed();
    // if generation.count % 1000 == 0 {
    //     info!("update_cells: Average time: {}ns", duration.as_nanos() / generation.count as u128);
    // }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

#[derive(Resource)]
pub struct Generation {
    pub count: i32,
}