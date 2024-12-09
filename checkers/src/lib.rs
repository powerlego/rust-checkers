use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub enum CheckerColor {
    Black,
    Red,
}

#[derive(Component)]
pub struct Checker;

#[derive(Component)]
pub struct Board {
    pub grid: [[Option<Entity>; 8]; 8],
}

#[derive(Component)]
pub enum Piece {
    King,
    Regular,
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera {
            clear_color: ClearColorConfig::Custom(Color::Srgba(Srgba::BLUE)),
            ..Default::default()
        },
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width:  128.0,
                min_height: 128.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

#[cfg(test)]
mod tests {}
