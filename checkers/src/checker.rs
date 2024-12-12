use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};

const PIECE_SIZE: f32 = TILE_SIZE * 0.8;

#[derive(Component)]
pub enum CheckerColor {
    Black,
    Red,
}

#[derive(Component)]
pub struct Checker {
    pub position: (u8, u8),
}

#[derive(Component)]
pub enum Piece {
    King,
    Regular,
}

pub fn spawn_checkers(
    mut commands: Commands,
    mut board: Query<&mut Board>,
    asset_server: Res<AssetServer>,
) {
    for mut board in board.iter_mut() {
        for i in 0..8 {
            for j in 0..3 {
                if (i + j) % 2 == 0 {
                    let image = asset_server.load("checker_red.png");
                    let entity = commands.spawn((
                        Checker {
                            position: (i as u8, j as u8),
                        },
                        CheckerColor::Red,
                        Piece::Regular,
                        Sprite {
                            image,
                            custom_size: Some(Vec2::new(
                                PIECE_SIZE, PIECE_SIZE,
                            )),
                            ..Default::default()
                        },
                        Transform {
                            translation: Vec3::new(
                                board.grid_coordinates[i][j].x,
                                board.grid_coordinates[i][j].y,
                                1.0,
                            ),
                            ..Default::default()
                        },
                    ));
                    board.grid[i][j] = Some(entity.id());
                }
            }
            for j in 5..8 {
                if (i + j) % 2 == 0 {
                    let image = asset_server.load("checker_black.png");
                    let entity = commands.spawn((
                        Checker {
                            position: (i as u8, j as u8),
                        },
                        CheckerColor::Black,
                        Piece::Regular,
                        Sprite {
                            image,
                            custom_size: Some(Vec2::new(
                                PIECE_SIZE, PIECE_SIZE,
                            )),
                            ..Default::default()
                        },
                        Transform {
                            translation: Vec3::new(
                                board.grid_coordinates[i][j].x,
                                board.grid_coordinates[i][j].y,
                                1.0,
                            ),
                            ..Default::default()
                        },
                    ));
                    board.grid[i][j] = Some(entity.id());
                }
            }
        }
    }
}
