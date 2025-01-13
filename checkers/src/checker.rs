use bevy::color::palettes::css;
use bevy::math::bounding::{Bounded2d, BoundingVolume};
use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};
use crate::mouse::Draggable;
use crate::{CurrentVolume, Shape};

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

#[derive(Bundle)]
pub struct CheckerBundle {
    pub checker:        Checker,
    pub color:          CheckerColor,
    pub piece:          Piece,
    pub shape:          Shape,
    pub sprite:         Sprite,
    pub transform:      Transform,
    pub current_volume: CurrentVolume,
    pub draggable:      Draggable,
}

impl Default for CheckerBundle {
    fn default() -> Self {
        let translation = Transform::default().translation.xy();
        let rotation = Transform::default().rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        let bounding_circle = Circle::new(BOUNDING_SIZE / 2.0);
        let current_volume = CurrentVolume(bounding_circle.aabb_2d(isometry));

        CheckerBundle {
            checker: Checker { position: (0, 0) },
            color: CheckerColor::Red,
            piece: Piece::Regular,
            shape: Shape::Circle(bounding_circle),
            sprite: Sprite::default(),
            transform: Transform::default(),
            current_volume,
            draggable: Draggable,
        }
    }
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
                    let transform = Transform {
                        translation: Vec3::new(
                            board.grid_coordinates[i][j].x,
                            board.grid_coordinates[i][j].y,
                            1.0,
                        ),
                        ..Default::default()
                    };
                    let translation = transform.translation.xy();
                    let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
                    let isometry =
                        Isometry2d::new(translation, Rot2::radians(rotation));
                    let bounding_circle = Circle::new(BOUNDING_SIZE / 2.0);
                    let current_volume =
                        CurrentVolume(bounding_circle.aabb_2d(isometry));

                    let entity = commands.spawn(CheckerBundle {
                        checker: Checker {
                            position: (i as u8, j as u8),
                        },
                        sprite: Sprite {
                            image,
                            custom_size: Some(Vec2::new(
                                PIECE_SIZE, PIECE_SIZE,
                            )),
                            ..Default::default()
                        },
                        shape: Shape::Circle(bounding_circle),
                        current_volume,
                        transform,
                        ..Default::default()
                    });
                    board.grid[i][j] = Some(entity.id());
                }
            }
            for j in 5..8 {
                if (i + j) % 2 == 0 {
                    let image = asset_server.load("checker_black.png");
                    let transform = Transform {
                        translation: Vec3::new(
                            board.grid_coordinates[i][j].x,
                            board.grid_coordinates[i][j].y,
                            1.0,
                        ),
                        ..Default::default()
                    };
                    let translation = transform.translation.xy();
                    let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
                    let isometry =
                        Isometry2d::new(translation, Rot2::radians(rotation));
                    let bounding_circle = Circle::new(BOUNDING_SIZE / 2.0);
                    let current_volume =
                        CurrentVolume(bounding_circle.aabb_2d(isometry));

                    let entity = commands.spawn(CheckerBundle {
                        checker: Checker {
                            position: (i as u8, j as u8),
                        },
                        color: CheckerColor::Black,
                        sprite: Sprite {
                            image,
                            custom_size: Some(Vec2::new(
                                PIECE_SIZE, PIECE_SIZE,
                            )),
                            ..Default::default()
                        },
                        transform,
                        shape: Shape::Circle(bounding_circle),
                        current_volume,
                        ..Default::default()
                    });
                    board.grid[i][j] = Some(entity.id());
                }
            }
        }
    }
}

type UpdateCheckerVolumesFilter =
    (With<Piece>, Or<(Changed<Shape>, Changed<Transform>)>);

pub fn update_volumes(
    mut commands: Commands,
    query: Query<(Entity, &Shape, &Transform), UpdateCheckerVolumesFilter>,
) {
    for (entity, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));

        if let Shape::Circle(circle) = shape {
            commands
                .entity(entity)
                .insert(CurrentVolume(circle.aabb_2d(isometry)));
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
