use bevy::prelude::{Component, UVec2};
use std::ops::Add;

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq, Default)]
pub struct Point {
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

impl From<UVec2> for Point {
    fn from(value: UVec2) -> Self {
        Point {x: value.x as i32, y: value.y as i32 }
    }
}

impl Point {
    #[inline(always)]
    pub fn neighbors(&self) -> Vec<Point> {
        NEIGHBOR_COORDINATES_8.map(|p| {
            *self + p
        }).into_iter().collect()
    }
}

const NEIGHBOR_COORDINATES_8: [Point; 8] = [
    // Left
    Point {
        x: -1,
        y: 0,
    },
    // Top Left
    Point {
        x: -1,
        y: -1,
    },
    // Top
    Point {
        x: 0,
        y: -1,
    },
    // Top Right
    Point {
        x: 1,
        y: -1,
    },
    // Right
    Point {
        x: 1,
        y: 0,
    },
    // Bottom Right
    Point {
        x: 1,
        y: 1,
    },
    // Bottom
    Point {
        x: 0,
        y: 1,
    },
    // Bottom Left
    Point {
        x: -1,
        y: 1,
    },
];