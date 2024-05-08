use std::time::Duration;

use crate::{
    collision::*,
    level::{Despawnable, Goal, LevelIndex, LevelState},
};
use bevy::{math::bounding::IntersectsVolume, prelude::*};

const PLAYER_ANIMATION_SPEED: f32 = 0.2;
const PLAYER_COLLIDER_SIZE: f32 = 14.0;
const PLAYER_SCALE: f32 = 2.0 / 3.0;

// Not a Bevy state, should pertain only to Player
#[derive(Default)]
enum PlayerAnimationState {
    #[default]
    Idle,
    Air,
    Walk,
}

#[derive(Clone, Copy, Debug, Default)]
enum PlayerType {
    #[default]
    Blue = 0,
    Yellow = 2,
}

#[derive(Component, Default)]
struct Player {
    velocity: (f32, f32),
    can_jump: bool,
    animation_state: PlayerAnimationState,
    player_type: PlayerType,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Init), player_init)
            .add_systems(
                FixedUpdate,
                (handle_player_input, move_player, camera_follow)
                    .chain()
                    .run_if(in_state(LevelState::Play)),
            )
            .add_systems(
                Update,
                (animate_player, check_goal_reached).run_if(in_state(LevelState::Play)),
            );
    }
}

fn player_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("characters.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(24.), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        Player::default(),
        Collider {
            width: PLAYER_COLLIDER_SIZE,
            height: PLAYER_COLLIDER_SIZE,
            layer_mask: 0b11,
            is_trigger: false,
        },
        AnimationTimer(Timer::from_seconds(
            PLAYER_ANIMATION_SPEED,
            TimerMode::Repeating,
        )),
        Despawnable::default(),
    ));
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut AnimationTimer, &mut Collider)>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let (mut player, mut timer, mut collider) = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    player.velocity.0 = direction;
    player.velocity.1 -= 0.1;

    if player.can_jump && keyboard_input.just_pressed(KeyCode::Space) {
        player.velocity.1 = 2.5;
        player.can_jump = false;
    }

    // Restart
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(LevelState::End);
    }

    // Switch types
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        player.player_type = match player.player_type {
            PlayerType::Blue => PlayerType::Yellow,
            PlayerType::Yellow => PlayerType::Blue,
        };
        collider.layer_mask = (player.player_type as u8) + 3;
        timer.tick(Duration::from_secs_f32(PLAYER_ANIMATION_SPEED));
    }
}

fn move_player(
    mut player_query: Query<(&mut Player, &mut Transform, &mut Sprite, &Collider)>,
    collider_query: Query<(&GlobalTransform, &Collider), Without<Player>>,
) {
    let (mut player, mut player_transform, mut player_sprite, player_collider) =
        player_query.single_mut();

    let next_player_pos = Vec2::new(
        player_transform.translation.x + player.velocity.0,
        player_transform.translation.y + player.velocity.1,
    );
    let next_player_bounding_box = player_collider.get_aabb2d(next_player_pos);

    // Collision detection
    let close_collider_transforms = collider_query
        .iter()
        .filter(|(transform, collider)| {
            // Layer mask check
            (player_collider.layer_mask & collider.layer_mask != 0)
                && !collider.is_trigger
                && next_player_bounding_box
                    .intersects(&collider.get_aabb2d(transform.translation().truncate()))
        })
        .collect::<Vec<(&GlobalTransform, &Collider)>>();

    // Technically means that bonking on ceiling allows you to jump
    player.can_jump = check_player_collision(
        &mut player,
        &player_transform,
        &player_collider,
        &close_collider_transforms,
    );

    if player.velocity.0 > 0.0 {
        player_sprite.flip_x = true;
    } else if player.velocity.0 < 0.0 {
        player_sprite.flip_x = false;
    }

    // If a y collision occured, we are in Air state
    // If we have velocity, we are in Walk state
    // Otherwise we are in Idle state
    player.animation_state = if !player.can_jump {
        PlayerAnimationState::Air
    } else if player.velocity.0.abs() > 0.0 {
        PlayerAnimationState::Walk
    } else {
        PlayerAnimationState::Idle
    };

    player_transform.translation.x += player.velocity.0;
    player_transform.translation.y += player.velocity.1;
}

fn check_player_collision(
    player: &mut Player,
    player_transform: &Transform,
    player_collider: &Collider,
    close_collider_transforms: &[(&GlobalTransform, &Collider)],
) -> bool {
    let mut y_collision = false;

    let next_player_pos_y = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y + player.velocity.1,
    );

    for &(transform, collider) in close_collider_transforms {
        let next_player_y_collider = player_collider.get_aabb2d(next_player_pos_y);
        let collider = collider.get_aabb2d(transform.translation().truncate());

        if next_player_y_collider.intersects(&collider) {
            player.velocity.1 = 0.0;
            y_collision = true;
            break;
        }
    }

    let next_player_pos_x = Vec2::new(
        player_transform.translation.x + player.velocity.0,
        player_transform.translation.y,
    );

    for &(transform, collider) in close_collider_transforms {
        let next_player_x_collider = player_collider.get_aabb2d(next_player_pos_x);
        let collider = collider.get_aabb2d(transform.translation().truncate());

        if next_player_x_collider.intersects(&collider) {
            player.velocity.0 = 0.0;
            break;
        }
    }

    y_collision
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    // TODO: Camera dead zone?

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn check_goal_reached(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    goal_query: Query<(&GlobalTransform, &Collider), With<Goal>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut level_index: ResMut<LevelIndex>,
) {
    let (player_transform, player_collider) = player_query.single();
    let player_bounding_box = player_collider.get_aabb2d(player_transform.translation.truncate());

    for (goal_transform, goal_collider) in &goal_query {
        let goal_bounding_box = goal_collider.get_aabb2d(goal_transform.translation().truncate());

        if player_bounding_box.intersects(&goal_bounding_box) {
            // We win
            level_index.to_next_level();
            next_state.set(LevelState::End);
        }
    }
}

fn animate_player(
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    let (player, mut timer, mut atlas) = player_query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = (player.player_type as usize)
            + match player.animation_state {
                PlayerAnimationState::Idle => 0,
                PlayerAnimationState::Air => 1,
                PlayerAnimationState::Walk => (atlas.index % 2 + 1) % 2,
            };
    }
}
