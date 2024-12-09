use bevy::prelude::*;

const COORDINATE_SIZE: f32 = 256.0;

#[derive(Component)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
enum CheckerColor {
    Black,
    Red,
}

#[derive(Component)]
struct Checker;

#[derive(Component)]
struct Board {
    pub grid: [[Option<Entity>; 8]; 8],
}

#[derive(Component)]
enum Piece {
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
                min_width:  COORDINATE_SIZE,
                min_height: COORDINATE_SIZE
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

pub fn board_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Board {
            grid: [[None; 8]; 8],

        },
        Sprite {
            custom_size: Some(Vec2::new(COORDINATE_SIZE*0.9, COORDINATE_SIZE*0.9)),
            ..Default::default()
        }
    ));
}

#[cfg(test)]
mod tests {}
