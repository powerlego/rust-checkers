use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
mod board;
mod camera;
mod checker;
mod game;
mod mouse;

pub const COORDINATE_SIZE: f32 = 256.0;

#[derive(Component)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Component)]
pub struct CurrentVolume(Aabb2d);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            board::BoardPlugin,
            checker::CheckerPlugin,
        ));
    }
}

#[cfg(test)]
mod tests {}
