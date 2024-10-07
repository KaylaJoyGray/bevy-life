use crate::field::Field;
use crate::point::Point;
use crate::systems::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::{App, Plugin, Res, Startup, UVec2, Update};
use bevy_pixel_buffer::prelude::{pixel_buffer_setup, GetFrame, Pixel, PixelBufferPlugin, PixelBufferSize, QueryPixelBuffer};

const BUFFER_SIZE: PixelBufferSize = PixelBufferSize {
    size: UVec2::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
    pixel_size: UVec2::new(1, 1),
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelBufferPlugin)
            .add_systems(Startup, pixel_buffer_setup(BUFFER_SIZE))
            .add_systems(Update, update_buffer);
    }
}

fn update_buffer(mut buffer: QueryPixelBuffer,
                 field: Res<Field>) {
    let mut frame = buffer.frame();

    frame.per_pixel_par(|vec, _pixel| {
        if field.get(Point::from(vec)) {
            Pixel::WHITE
        } else {
            Pixel::BLACK
        }
    });
}