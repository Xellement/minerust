use crate::{player::Player, RESOLUTION, CAMERA_SIZE};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct GameCameraPlugin;

#[derive(Component)]
pub struct GameCamera;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            spawn_camera.label("camera"),
        )
        .add_system(camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.top = CAMERA_SIZE;
    camera.orthographic_projection.bottom = -CAMERA_SIZE;
    camera.orthographic_projection.right = CAMERA_SIZE * RESOLUTION;
    camera.orthographic_projection.left = -CAMERA_SIZE * RESOLUTION;

    commands.spawn_bundle(camera).insert(GameCamera);
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let player_transform = player_query.single().translation;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.x;
    camera_transform.translation.y = player_transform.y;
}