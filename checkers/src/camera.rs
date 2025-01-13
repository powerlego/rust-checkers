use bevy::prelude::*;

use crate::COORDINATE_SIZE;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
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
