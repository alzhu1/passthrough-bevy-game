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

  Final things to do before ship:

  * Goal condition (once the door is reached, go to next level)
    * Probably will need to extend the Collider with an is_trigger bool
    * Loop back to the beginning
  * Tutorial UI? Just some text for controls should be enough
  * Sounds
    * BGM
    * Sound effect for jumping
    * (not sure) SFX for walking
  * Make 2-3 levels, more if possible

*/

fn main() {
    let mut app = App::new();
    app.init_state::<LevelState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            TilemapPlugin,
            LevelsPlugin,
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
            ..default()
        },
        Fader,
    ));
}
