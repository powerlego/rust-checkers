use bevy::prelude::*;

use crate::COORDINATE_SIZE;

const BOARD_SPRITE_SIZE: f32 = 516.0;
const BORDER_SPRITE_SIZE: f32 = 2.0;
const TILE_SIZE: f32 = ((BOARD_SPRITE_SIZE - BORDER_SPRITE_SIZE * 2.0) / 8.0)
    * ((COORDINATE_SIZE / BOARD_SPRITE_SIZE) * 0.9);

#[derive(Component)]
pub struct Board {
    pub grid:             [[Option<Entity>; 8]; 8],
    pub grid_coordinates: [[Vec2; 8]; 8],
}

pub fn board_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("board-tokyostorm.png");
    let mut grid_coordinates = [[Vec2::ZERO; 8]; 8];
    for i in -4i32..4i32 {
        for j in -4i32..4i32 {
            grid_coordinates[(i + 4) as usize][(j + 4) as usize] =
                Vec2::new((i as f32 * TILE_SIZE) + (TILE_SIZE/2.0), (j as f32 * TILE_SIZE) + (TILE_SIZE/2.0));
        }
    }

    commands.spawn((
        Board {
            grid: [[None; 8]; 8],
            grid_coordinates,
        },
        Sprite {
            image,
            custom_size: Some(Vec2::new(
                COORDINATE_SIZE * 0.9,
                COORDINATE_SIZE * 0.9,
            )),
            ..Default::default()
        },
    ));
}


