use bevy::color::palettes::css;
use bevy::math::bounding::{Aabb2d, Bounded2d, BoundingVolume};
use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};

const PIECE_SIZE: f32 = TILE_SIZE * 0.8;
const BOUNDING_SIZE: f32 = PIECE_SIZE * 1.1;

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

#[derive(Component)]
pub enum Shape {
    Circle(Circle),
}

#[derive(Component)]
pub struct CurrentVolume(Aabb2d);

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
                        Shape::Circle(Circle::new(BOUNDING_SIZE / 2.0)),
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
                        Shape::Circle(Circle::new(BOUNDING_SIZE / 2.0)),
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

pub fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &Shape, &Transform),
        Or<(Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));

        match shape {
            Shape::Circle(circle) => {
                commands
                    .entity(entity)
                    .insert(CurrentVolume(circle.aabb_2d(isometry)));
            }
        }
    }
}
pub fn render_bounding(
    mut gizmos: Gizmos,
    query: Query<&CurrentVolume, With<Piece>>,
) {
    for vol in query.iter() {
        gizmos.rect_2d(vol.0.center(), vol.0.half_size() * 2., css::AQUA);
    }
}
