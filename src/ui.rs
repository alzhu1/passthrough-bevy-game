use bevy::prelude::*;

use crate::level::{Despawnable, LevelIndex, LevelState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Init), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, level_index: Res<LevelIndex>) {
    // Show on tutorial only, first time
    if level_index.0 == 0 && !level_index.1 {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(120.),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.1)),
                    z_index: ZIndex::Global(-1),
                    ..default()
                },
                Despawnable::with_children(true),
            ))
            .with_children(|parent| {
                // Add text to screen
                parent.spawn(TextBundle::from_section(
                    "Arrow Keys to move, Space to Jump\nLeft Shift to Change Characters, R to Restart", TextStyle {
                        font: asset_server.load("Pixellari.ttf"),
                        font_size: 36.0,
                        color: Color::WHITE,
                        ..default()
                    })
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    })
                );
            });
    }
}
