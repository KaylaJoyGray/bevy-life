use crate::point::Point;
use bevy::prelude::Resource;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Resource)]
pub struct Field {
    width: usize,
    height: usize,
    cells: Vec<AtomicBool>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: {
                let mut v = Vec::new();
                for _ in 0..width * height {
                    v.push(AtomicBool::new(false));
                }
                v
            },
        }
    }
    
    #[inline(always)]
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32
    }
    
    #[inline(always)]
    fn get_index(&self, point: Point) -> usize {
        ((point.y * self.width as i32) + point.x) as usize
    }

    #[inline(always)]
    pub fn flip(&self, point: Point) {
        if self.in_bounds(point) {
            self.cells.get(self.get_index(point)).unwrap().fetch_not(Ordering::Relaxed);
        }
    }
    
    #[inline(always)]
    pub fn set(&self, point: Point, value: bool) {
        if self.in_bounds(point) {
            self.cells[self.get_index(point)].store(value, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub fn get(&self, point: Point) -> bool {
        if self.in_bounds(point) {
            self.cells[self.get_index(point)].load(Ordering::Relaxed)
        } else {
            false
        }
    }
}