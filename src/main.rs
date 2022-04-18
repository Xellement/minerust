#![allow(clippy::redundant_field_names)]
#![allow(dead_code)]
use bevy::{prelude::*, window::PresentMode, render::camera::ScalingMode};

mod asset;
mod tilemap;

use asset::AssetPlugin;
use tilemap::TilemapPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 8.0;
pub const CAMERA_SIZE: f32 = TILE_SIZE * 4.0;

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
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.top = CAMERA_SIZE;
    camera.orthographic_projection.bottom = -CAMERA_SIZE;
    camera.orthographic_projection.right = CAMERA_SIZE * RESOLUTION;
    camera.orthographic_projection.left = -CAMERA_SIZE * RESOLUTION;
    commands.spawn_bundle(camera);
}