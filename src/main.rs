use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod collision;
mod player;
mod state;
mod tilemap;

use player::PlayerPlugin;
use state::{Fader, LevelState, StateDefaultsPlugin};
use tilemap::TilemapPlugin;

/* TODO: Need to figure out the following, probably in listed order:

  * How to display/move around a sprite
  * How to display a tile map (with culling, use Visibility?)
  * How to have the camera follow the player
  * Collisions

  Then think about the game jam theme (Changing Sides, optional restriction: The Chosen One)

    TODO: More stuff:
    * Implement level loading/transition logic. Basically need State transitions
    * 4 states, probably LevelState enum:
        * Init
            * OnEnter: Start with black screen? and run all plugin init systems
            * OnExit: Start fading the screen, move to play state
        * Play
            * OnEnter: Grant player control?
            * Go to OnExit once restart is pressed or goal is reached
        * End
            * OnEnter: Fade to black?
            * OnExit: Despawn all entities

*/

fn main() {
    let mut app = App::new();
    app.init_state::<LevelState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FrameTimeDiagnosticsPlugin::default(),
            PlayerPlugin,
            TilemapPlugin,
            StateDefaultsPlugin,
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
