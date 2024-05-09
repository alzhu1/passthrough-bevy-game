use bevy::{audio::Volume, prelude::*};

use crate::player::{GoalEvent, JumpEvent, SwitchEvent};

pub struct AudioPlugin;

#[derive(Resource)]
struct JumpSound(Handle<AudioSource>);

#[derive(Resource)]
struct SwitchSound(Handle<AudioSource>);

#[derive(Resource)]
struct GoalSound(Handle<AudioSource>);

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            FixedUpdate,
            (play_jump_sound, play_switch_sound, play_goal_sound),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/bgm.ogg"),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.4)),
    });

    let jump_sound = asset_server.load("sounds/jump.ogg");
    commands.insert_resource(JumpSound(jump_sound));

    let switch_sound = asset_server.load("sounds/switch.ogg");
    commands.insert_resource(SwitchSound(switch_sound));

    let goal_sound = asset_server.load("sounds/goal.ogg");
    commands.insert_resource(GoalSound(goal_sound));
}

fn play_jump_sound(
    commands: Commands,
    jump_event_reader: EventReader<JumpEvent>,
    jump_sound: Res<JumpSound>,
) {
    play_sound_effect(commands, jump_event_reader, jump_sound.0.clone(), 0.5);
}

fn play_switch_sound(
    commands: Commands,
    switch_event_reader: EventReader<SwitchEvent>,
    switch_sound: Res<SwitchSound>,
) {
    play_sound_effect(commands, switch_event_reader, switch_sound.0.clone(), 0.5);
}

fn play_goal_sound(
    commands: Commands,
    goal_event_reader: EventReader<GoalEvent>,
    goal_sound: Res<GoalSound>,
) {
    play_sound_effect(commands, goal_event_reader, goal_sound.0.clone(), 0.5);
}

fn play_sound_effect<T: Event>(
    mut commands: Commands,
    mut event_reader: EventReader<T>,
    source: Handle<AudioSource>,
    volume: f32,
) {
    if !event_reader.is_empty() {
        event_reader.clear();

        commands.spawn(AudioBundle {
            source,
            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(volume)),
        });
    }
}
