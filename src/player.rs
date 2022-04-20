use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{TILE_SIZE, PLAYER_BASE_SPEED, asset::PlayerAsset};

pub struct PlayerPlugin;

#[derive(Debug, Inspectable, Component)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Z) {
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, asset: Res<PlayerAsset>) {
    let mut player_sprite = TextureAtlasSprite::new(0);
    player_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_sprite,
            texture_atlas: asset.texture_atlas.clone(),
            transform: Transform {
                translation: Vec3::new(5.0 * TILE_SIZE, 5.0  * TILE_SIZE, 200.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_BASE_SPEED
        });
}
