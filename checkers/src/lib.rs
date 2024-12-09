use bevy::prelude::*;
mod board;

pub use board::board_setup;

pub const COORDINATE_SIZE: f32 = 256.0;

#[derive(Component)]
enum CheckerColor {
    Black,
    Red,
}

#[derive(Component)]
struct Checker {
    pub position: Vec2,
}

#[derive(Component)]
enum Piece {
    King,
    Regular,
}

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
