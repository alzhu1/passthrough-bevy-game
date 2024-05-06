use crate::collision::Collider;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct Tile;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add stuff here for tilemap. Also figure out what systems/stuff
        // it will add? Maybe a tilemap bundle?

        // Probably some file reading
        app.add_systems(PreStartup, load_level);
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // TODO: Use asset server to fetch some config file (tilemap?)
    // Config file should have info for tile type + tile collision if any
    // Also fetch and store tilemap resource (image)

    let texture = asset_server.load("tilemap_packed.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(18.), 20, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Store texture + layout on tilemap strongly, children inherit weak
    let tilemap_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            texture.clone(),
            texture_atlas_layout.clone(),
            Tilemap,
        ))
        .id();

    // TODO: Read in some level file based on a resource index
    // For this level file, generate a bunch of Children entities to attach to tilemap_entity

    let mut tile_entities = vec![];

    tile_entities.push(
        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, -48.0, 0.0)
                        .with_scale(Vec3::splat(8.0 / 9.0)),
                    texture: texture.clone_weak(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone_weak(),
                    index: 20,
                },
                Collider,
            ))
            .id(),
    );

    commands
        .entity(tilemap_entity)
        .push_children(&tile_entities);
}
