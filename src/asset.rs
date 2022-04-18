use std::fs;
use bevy::prelude::*;
use bevy::sprite::Rect;
use ron::from_str;
use serde::{Deserialize};
use bevy::utils::HashMap;

use crate::TILE_SIZE;
use crate::tilemap::TilesType;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

pub struct GlobalAssets {
    pub texture_atlas: Handle<TextureAtlas>,
    pub tiles_map: HashMap<TilesType, usize>
}

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
struct SRect {
    pub min: Vec2,
    pub max: Vec2
}

#[derive(Deserialize)]
struct TilesDesc {
    map: HashMap<TilesType, SRect>
}

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image_handle = assets.load("spritesheet.png");

    // TILES LOADING
    let tiles_desc = fs::read_to_string("assets/tiles_desc.ron").unwrap();
    let tiles_desc: TilesDesc = from_str(&tiles_desc).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.));
    let mut tiles_map: HashMap<TilesType, usize> = HashMap::default();
    for (tile_type, rect) in tiles_desc.map.iter() {
        println!("Found graphic {:?}", tile_type);
            let index = atlas.add_texture(Rect {
                min: rect.min,
                max: rect.max,
            });
            tiles_map.insert(*tile_type, index);
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
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(GlobalAssets {
        texture_atlas: atlas_handle,
        tiles_map
    });
}

pub fn spawn_sprite(
    commands: &mut Commands,
    texture_atlas: Handle<TextureAtlas>,
    index: usize,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}