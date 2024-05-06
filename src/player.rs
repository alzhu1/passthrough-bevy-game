use crate::collision::*;
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

#[derive(Component)]
struct Player {
    velocity: (f32, f32),
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            FixedUpdate,
            (update_player_velocity, move_player, camera_follow).chain(),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Figure out how to handle size. The character is 24x24
    let scale = 2.0 / 3.0;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("character.png"),
            transform: Transform::from_scale(Vec3::splat(scale)),
            ..default()
        },
        Player {
            velocity: (0.0, 0.0),
        },
        Collider,
    ));

    // Some thing with a collider
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("character.png"),
    //         transform: Transform::from_xyz(0.0, -200.0, 0.0),
    //         ..default()
    //     },
    //     Collider,
    // ));

    // // Some thing with a collider
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("character.png"),
    //         transform: Transform::from_xyz(-24.0, -224.0, 0.0),
    //         ..default()
    //     },
    //     Collider,
    // ));
    // // Some thing with a collider
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("character.png"),
    //         transform: Transform::from_xyz(24.0, -224.0, 0.0),
    //         ..default()
    //     },
    //     Collider,
    // ));

    // // Some thing with a collider
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("character.png"),
    //         transform: Transform::from_xyz(48.0, -200.0, 0.0),
    //         ..default()
    //     },
    //     Collider,
    // ));

    // // Some thing with a collider
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("character.png"),
    //         transform: Transform::from_xyz(72.0, -200.0, 0.0),
    //         ..default()
    //     },
    //     Collider,
    // ));
}

fn update_player_velocity(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    player.velocity.0 = direction;
    player.velocity.1 -= 0.1;

    if keyboard_input.pressed(KeyCode::Space) {
        player.velocity.1 = 2.0;
    }
}

fn move_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    let next_player_pos = Vec2::new(
        player_transform.translation.x + player.velocity.0,
        player_transform.translation.y + player.velocity.1,
    );
    let next_player_bounding_box = Aabb2d::new(next_player_pos, Vec2::new(8., 8.));

    // Collision detection
    let close_collider_transforms = collider_query
        .iter()
        .filter(|transform| {
            next_player_bounding_box.intersects(&Aabb2d::new(
                transform.translation.truncate(),
                Vec2::new(8., 8.),
            ))
        })
        .collect::<Vec<&Transform>>();
    check_player_collision(&mut player, &player_transform, &close_collider_transforms);

    player_transform.translation.x += player.velocity.0;
    player_transform.translation.y += player.velocity.1;
}

fn check_player_collision(
    player: &mut Player,
    player_transform: &Transform,
    close_collider_transforms: &[&Transform],
) {
    // TODO: Make these constants
    let player_half = Vec2::new(8., 8.);
    let half = Vec2::new(8., 8.);

    let next_player_pos_y = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y + player.velocity.1,
    );

    for &transform in close_collider_transforms {
        let next_player_y_collider = Aabb2d::new(next_player_pos_y, player_half);
        let collider = Aabb2d::new(transform.translation.truncate(), half);

        if next_player_y_collider.intersects(&collider) {
            player.velocity.1 = 0.0;
            break;
        }
    }

    let next_player_pos_x = Vec2::new(
        player_transform.translation.x + player.velocity.0,
        player_transform.translation.y,
    );

    for &transform in close_collider_transforms {
        let next_player_x_collider = Aabb2d::new(next_player_pos_x, player_half);
        let collider = Aabb2d::new(transform.translation.truncate(), half);

        if next_player_x_collider.intersects(&collider) {
            player.velocity.0 = 0.0;
            break;
        }
    }
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
