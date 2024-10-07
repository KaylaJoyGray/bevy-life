use bevy::prelude::{Bundle, Component};
use crate::point::Point;

#[derive(Component)]
pub struct CellState {
    pub(crate) alive: bool,
}

#[derive(Bundle)]
pub struct CellBundle {
    pub(crate) cell: CellState,
    pub(crate) point: Point,
}