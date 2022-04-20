use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy::utils::HashMap;
use ron::from_str;
use serde::Deserialize;
use std::fs;

use crate::tilemap::TilesType;
use crate::TILE_SIZE;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_tiles_assets)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_player_asset);
    }
}

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
struct SRect {
    pub min: (u16, u16),
    pub max: (u16, u16),
}

pub struct TileAssets {
    pub texture_atlas: Handle<TextureAtlas>,
    pub tiles_map: HashMap<TilesType, usize>
}

pub struct PlayerAsset {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player: usize
}

#[derive(Deserialize)]
struct TilesDesc {
    map: HashMap<TilesType, SRect>,
}

/*
    TODO :
        get image handle
        get image description (.ron) to ObjectDesc
        create empty TextureAtlas
        for (ObjectType, SRect)
            index = add to atlas
            Object_map.insert(*ObjectType, index)
        ...
        commands.insert_resources (GlobalAssets{

        })
    */

/*println!("[ASSET] Loading Player texture");
    let image_handle = assets.load("player.png");*/

fn load_tiles_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image_handle = assets.load("tilessheet.png");
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.));
    
    // TILES LOADING
    println!("[ASSET] Loading Tiles textures");
    let tiles_desc = fs::read_to_string("assets/tiles_desc.ron").unwrap();
    let tiles_desc: TilesDesc = from_str(&tiles_desc).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    let mut tiles_map: HashMap<TilesType, usize> = HashMap::default();
    for (tile_type, rect) in tiles_desc.map.iter() {
        println!("[ASSET] Found graphic {:?}", tile_type);
        let index = atlas.add_texture(Rect {
            min: Vec2::new(rect.min.0.into(), rect.min.1.into()),
            max: Vec2::new(rect.max.0.into(), rect.max.1.into()),
        });
        tiles_map.insert(*tile_type, index);
    }

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(TileAssets {
        texture_atlas: atlas_handle,
        tiles_map,
    });
}

fn load_player_asset(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image_handle = assets.load("player.png");
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(32.));

    let player = atlas.add_texture(Rect {
        min: Vec2::new( 0., 0.),
        max: Vec2::new( 31., 31.)
    });

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(PlayerAsset {
        texture_atlas: atlas_handle,
        player
    });
}

pub fn spawn_sprite(
    commands: &mut Commands,
    texture_atlas: &Handle<TextureAtlas>,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_atlas.clone(),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
