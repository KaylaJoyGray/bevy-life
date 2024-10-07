use bevy::prelude::Resource;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::point::Point;

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
    pub fn flip(&self, point: Point) {
        if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
            self.cells.get(((point.y * self.width as i32) + point.x) as usize).unwrap().fetch_not(Ordering::Relaxed);
        }    
    }

    #[inline(always)]
    pub fn get(&self, point: Point) -> bool {
        if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
            unsafe { self.cells[((point.y * self.width as i32) + point.x) as usize].as_ptr().read() }
        } else {
            false
        }
    }
}