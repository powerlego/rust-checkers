use bevy::color::palettes::css;
use bevy::math::bounding::{Bounded2d, BoundingVolume};
use bevy::prelude::*;

use crate::mouse::DropZone;
use crate::{CurrentVolume, Shape, COORDINATE_SIZE};

const BOARD_SPRITE_SIZE: f32 = 516.0;
const BORDER_SPRITE_SIZE: f32 = 2.0;
const BOARD_SIZE: f32 = COORDINATE_SIZE * 0.95;
pub const TILE_SIZE: f32 = ((BOARD_SPRITE_SIZE - BORDER_SPRITE_SIZE * 2.0)
    / 8.0)
    * (BOARD_SIZE / BOARD_SPRITE_SIZE);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, board_setup)
            .add_systems(PostUpdate, render_tile_bounding);
    }
}

#[derive(Component)]
pub struct Board {
    pub grid:             [[Option<Entity>; 8]; 8],
    pub grid_coordinates: [[Vec2; 8]; 8],
}

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub drop_zone:      DropZone,
    pub shape:          Shape,
    pub current_volume: CurrentVolume,
    pub tile:           Tile,
    pub transform:      Transform,
}
impl Default for TileBundle {
    fn default() -> Self {
        let translation = Transform::default().translation.xy();
        let rotation = Transform::default().rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
        let bounding_rect = Rectangle::from_size(Vec2::splat(TILE_SIZE / 4.0));
        let current_volume = CurrentVolume(bounding_rect.aabb_2d(isometry));

        TileBundle {
            tile: Tile,
            drop_zone: DropZone::default(),
            shape: Shape::Rectangle(bounding_rect),
            transform: Transform::default(),
            current_volume,
        }
    }
}

pub fn board_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("board-tokyostorm.png");
    let mut grid_coordinates = [[Vec2::ZERO; 8]; 8];
    for i in -4i32..4i32 {
        for j in -4i32..4i32 {
            grid_coordinates[(i + 4) as usize][(j + 4) as usize] = Vec2::new(
                (i as f32 * TILE_SIZE) + (TILE_SIZE / 2.0),
                (j as f32 * TILE_SIZE) + (TILE_SIZE / 2.0),
            );
        }
    }

    commands.spawn((
        Board {
            grid: [[None; 8]; 8],
            grid_coordinates,
        },
        Sprite {
            image,
            custom_size: Some(Vec2::new(BOARD_SIZE, BOARD_SIZE)),
            ..Default::default()
        },
    ));

    for (i, grid_row) in grid_coordinates.iter().enumerate() {
        for (j, grid_coordinate) in grid_row.iter().enumerate() {
            let transform = Transform {
                translation: Vec3::new(
                    grid_coordinate.x,
                    grid_coordinate.y,
                    1.0,
                ),
                ..Default::default()
            };
            let translation = transform.translation.xy();
            let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
            let isometry =
                Isometry2d::new(translation, Rot2::radians(rotation));
            let bounding_rect =
                Rectangle::from_size(Vec2::splat(TILE_SIZE / 4.0));
            let current_volume = CurrentVolume(bounding_rect.aabb_2d(isometry));

            commands.spawn(TileBundle {
                drop_zone: DropZone {
                    position: (i as u8, j as u8),
                },
                transform,
                shape: Shape::Rectangle(bounding_rect),
                current_volume,
                ..Default::default()
            });
        }
    }
}

fn render_tile_bounding(
    mut gizmos: Gizmos,
    query: Query<&CurrentVolume, With<Tile>>,
) {
    for vol in query.iter() {
        gizmos.rect_2d(vol.0.center(), vol.0.half_size() * 2., css::GOLD);
    }
}
