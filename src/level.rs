use bevy::prelude::*;

const FADE_DURATION: f32 = 0.5;
const LEVEL_COUNT: u8 = 3;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
    #[default]
    Init,
    Play,
    End,
}

#[derive(Component)]
pub struct Goal;

// To track despawn
#[derive(Component, Default)]
pub struct Despawnable {
    has_children: bool,
}

#[derive(Component)]
pub struct Fader;

impl Despawnable {
    pub fn with_children(has_children: bool) -> Self {
        Despawnable { has_children }
    }
}

#[derive(Resource, Default)]
pub struct LevelIndex(pub u8, pub bool);

impl LevelIndex {
    pub fn to_next_level(&mut self) {
        self.0 = (self.0 + 1) % LEVEL_COUNT;
        self.1 = true;
    }
}

#[derive(Resource)]
struct LevelTransitionTimer(Timer);

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelTransitionTimer(Timer::from_seconds(
            FADE_DURATION,
            TimerMode::Repeating,
        )))
        .insert_resource(LevelIndex::default())
        .add_systems(Update, level_transition.run_if(in_transition_state))
        .add_systems(OnExit(LevelState::End), (cleanup_entities, reset_camera));
    }
}

fn in_transition_state(state: Res<State<LevelState>>) -> bool {
    match state.get() {
        LevelState::End => true,
        LevelState::Init => true,
        _ => false,
    }
}

fn level_transition(
    time: Res<Time>,
    mut timer: ResMut<LevelTransitionTimer>,
    state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut fader_query: Query<&mut BackgroundColor, With<Fader>>,
) {
    let mut fader_bg = fader_query.single_mut();

    let elapsed_percent = timer.0.elapsed().as_secs_f32() / FADE_DURATION;
    match state.get() {
        LevelState::End => fader_bg.0 = Color::BLACK.with_a(elapsed_percent),
        LevelState::Init => fader_bg.0 = Color::BLACK.with_a(1. - elapsed_percent),
        _ => unreachable!(),
    }

    if timer.0.tick(time.delta()).just_finished() {
        match state.get() {
            LevelState::End => {
                fader_bg.0 = Color::BLACK;
                next_state.set(LevelState::Init);
            }
            LevelState::Init => {
                fader_bg.0 = Color::BLACK.with_a(0.);
                next_state.set(LevelState::Play);
            }
            _ => unreachable!(),
        }
    }
}

fn cleanup_entities(mut commands: Commands, entities: Query<(Entity, &Despawnable)>) {
    for (entity, despawnable) in &entities {
        match despawnable.has_children {
            true => commands.entity(entity).despawn_recursive(),
            false => commands.entity(entity).despawn(),
        }
    }
}

fn reset_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    *(camera_query.single_mut()) = Transform::default();
}
