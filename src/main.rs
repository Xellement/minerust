#![allow(clippy::redundant_field_names)]
#![allow(dead_code)]
use bevy::{prelude::*, window::PresentMode};

mod asset;
mod tilemap;
mod player;
mod debug;
mod game_camera;

use asset::AssetPlugin;
use tilemap::TilemapPlugin;
use player::PlayerPlugin;
use debug::DebugPlugin;
use game_camera::GameCameraPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 16.0;
pub const CAMERA_SIZE: f32 = TILE_SIZE * 2.0;
pub const PLAYER_BASE_SPEED : f32 = 4.0;

fn main() {
    let height: f32 = 720.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Minerust".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(GameCameraPlugin)
        .run();
    }