use crate::point::Point;
use bevy::prelude::{Bundle, Component};

#[derive(Component)]
pub struct CellState {
    pub(crate) alive: bool,
}

#[derive(Bundle)]
pub struct CellBundle {
    pub(crate) cell: CellState,
    pub(crate) point: Point,
}