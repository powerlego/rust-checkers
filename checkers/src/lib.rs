use bevy::prelude::*;
mod board;
mod checker;
mod game;

pub use board::board_setup;
pub use checker::{render_bounding, spawn_checkers, update_volumes};

pub const COORDINATE_SIZE: f32 = 256.0;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera::default(),
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width:  COORDINATE_SIZE,
                min_height: COORDINATE_SIZE,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

#[cfg(test)]
mod tests {}
