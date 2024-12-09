use bevy::prelude::*;
use checkers::{board_setup, camera_setup};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Checkers"),
                        resolution: (640.0, 640.0).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_systems(Startup, (camera_setup, board_setup))
        .run();
}
