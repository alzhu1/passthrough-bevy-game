use bevy::prelude::*;

mod audio;
mod collision;
mod level;
mod player;
mod tilemap;
mod ui;

use audio::AudioPlugin;
use level::{Fader, LevelState, LevelsPlugin};
use player::PlayerPlugin;
use tilemap::TilemapPlugin;
use ui::UiPlugin;

/* TODO:
  * Make 2-3 levels, more if possible

*/

fn main() {
    let mut app = App::new();
    app.init_state::<LevelState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AudioPlugin,
            PlayerPlugin,
            TilemapPlugin,
            LevelsPlugin,
            UiPlugin,
            #[cfg(debug_assertions)]
            {
                (
                    bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                    bevy::diagnostic::LogDiagnosticsPlugin::default(),
                )
            },
        ))
        .add_systems(Startup, setup)
        .run();
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
            z_index: ZIndex::Global(0),
            ..default()
        },
        Fader,
    ));
}
