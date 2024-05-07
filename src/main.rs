use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod collision;
mod level;
mod player;
mod tilemap;

use level::{Fader, LevelState, LevelsPlugin};
use player::PlayerPlugin;
use tilemap::TilemapPlugin;

/* TODO:
  Then think about the game jam theme (Changing Sides, optional restriction: The Chosen One)
*/

fn main() {
    let mut app = App::new();
    app.init_state::<LevelState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin::default(),
            PlayerPlugin,
            TilemapPlugin,
            LevelsPlugin,
        ))
        .add_systems(Startup, setup);

    // #[cfg(debug_assertions)] // debug/dev builds only
    // {
    //     use bevy::diagnostic::LogDiagnosticsPlugin;
    //     app.add_plugins(LogDiagnosticsPlugin::default());
    // }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scale: 0.3,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            visibility: Visibility::Visible,
            ..default()
        },
        Fader,
    ));
}
