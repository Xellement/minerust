use crate::asset::{spawn_sprite, TileAssets};
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
use ron::from_str;
use serde::Deserialize;
use std::fs;

const TILE_Z: f32 = 100.;

#[derive(Debug, Inspectable, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Component)]
pub enum TilesType {
    VOID,
    GRASS,
    DIRT,
    ROCK,
    SAND,
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

#[derive(Component, Inspectable)]
struct Map;

#[derive(Component)]
pub struct TileCollider;

#[derive(Debug)]
pub struct PlayerSpawn {
    pub x: u8,
    pub y: u8
}

#[derive(Deserialize)]
struct Row {
    i: i8,
    tiles: Vec<u8>
}

#[derive(Deserialize)]
struct Level {
    desc_tiles: HashMap<u8, TilesType>,
    spawn_player: (u8, u8),
    tiles_grid: Vec<Row>
}

fn create_map(mut commands: Commands, assets: Res<TileAssets>) {
    println!("Create Map");
    let mut tiles = Vec::new();
    let map_desc = fs::read_to_string("assets/map.ron").unwrap();
    let level: Level = from_str(&map_desc).unwrap_or_else(|e| {
        println!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    let desc_tiles = level.desc_tiles;

    for elem in level.tiles_grid {
        let y = elem.i;
        for (i, tile) in elem.tiles.iter().enumerate() {
            let translation = Vec3::new((i as f32) * TILE_SIZE, (*&y as f32) * TILE_SIZE, TILE_Z);
            let tile_type = desc_tiles
                .get(tile)
                .expect(&format!("No index tile for object {:?}", tile));

            let tile_data = assets
                .tiles_map
                .get(&tile_type)
                .expect(&format!("No graphic for object {:?}", &tile_type));

            let tile = spawn_sprite(&mut commands, &assets.texture_atlas, (*tile_data).0, translation);
            if (*tile_data).1 {
                commands.entity(tile).insert(TileCollider);
            }
            tiles.push(tile);
        }
    }

    commands
        .insert_resource(PlayerSpawn {
            x: level.spawn_player.0,
            y: level.spawn_player.1
        });

    commands
        .spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
