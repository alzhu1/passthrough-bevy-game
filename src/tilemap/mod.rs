use crate::{
    collision::Collider,
    level::{Despawnable, Goal, LevelIndex, LevelState},
};
use bevy::{prelude::*, utils::HashMap};

use self::level_maps::*;

mod level_maps;

const YELLOW_BLOCKS: [usize; 1] = [9];
const BLUE_BLOCKS: [usize; 10] = [93, 94, 95, 113, 114, 115, 132, 133, 134, 135];

const TILE_SIZE: f32 = 16.0;
const GOAL_COLLIDER_SIZE: f32 = 1.0;

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
struct Levels(HashMap<&'static str, &'static str>);

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        // TODO: in future, figure out how to do file reading
        // Cause normal fs operations don't work in WASM
        let levels = vec![
            ("level0", LEVEL_0),
            ("level1", LEVEL_1),
            ("level2", LEVEL_2),
        ]
        .into_iter()
        .collect::<HashMap<&str, &str>>();

        app.insert_resource(Msaa::Off)
            .insert_resource(Levels(levels))
            .add_systems(OnEnter(LevelState::Init), load_level);
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    levels: Res<Levels>,
    level_index: Res<LevelIndex>,
) {
    let texture = asset_server.load("tilemap_packed.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(18.), 20, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Store texture + layout on tilemap strongly, children inherit weak
    let tilemap_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(-4.0 * TILE_SIZE, 3.0 * TILE_SIZE, -1.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            texture.clone(),
            texture_atlas_layout.clone(),
            Tilemap,
            Despawnable::with_children(true),
        ))
        .id();

    let mut tile_entities = vec![];

    let level = levels
        .0
        .get(format!("level{}", level_index.0).as_str())
        .expect("No level found");

    for (y, line) in level.lines().enumerate() {
        for (x, c) in line.split(",").map(|c| c.trim()).enumerate() {
            let x_pos = x as f32 * 16.0;
            let y_pos = y as f32 * -16.0;

            if let Ok(index) = c.parse::<usize>() {
                let layer_mask = if YELLOW_BLOCKS.contains(&index) {
                    0b100
                } else if BLUE_BLOCKS.contains(&index) {
                    0b10
                } else {
                    1
                };

                // Should be the door
                let is_trigger = index == 110 || index == 130;
                let size = if is_trigger {
                    GOAL_COLLIDER_SIZE
                } else {
                    TILE_SIZE
                };

                let mut tile_entity = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_pos, y_pos, 0.0)
                            .with_scale(Vec3::splat(8.0 / 9.0)),
                        texture: texture.clone_weak(),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout.clone_weak(),
                        index,
                    },
                    Collider {
                        width: size,
                        height: size,
                        layer_mask,
                        is_trigger,
                    },
                ));

                if is_trigger {
                    tile_entity.insert(Goal);
                }

                tile_entities.push(tile_entity.id());
            }
        }
    }

    commands
        .entity(tilemap_entity)
        .push_children(&tile_entities);
}
