use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{TILE_SIZE, PLAYER_BASE_SPEED, asset::PlayerAsset, tilemap::{PlayerSpawn, TileCollider}};

pub struct PlayerPlugin;

#[derive(Debug, Inspectable, Component)]
pub struct Player {
    speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player)
        .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    collision_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (player, mut transform) = player_query.single_mut();

    let mut move_delta: [f32; 2] = [0.0, 0.0];

    if keyboard.pressed(KeyCode::Z) {
        move_delta[1] += player.speed * TILE_SIZE * time.delta_seconds();
        // transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        move_delta[0] -= player.speed * TILE_SIZE * time.delta_seconds();
        // transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        move_delta[1] -= player.speed * TILE_SIZE * time.delta_seconds();
        // transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        move_delta[0] += player.speed * TILE_SIZE * time.delta_seconds();
        // transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(move_delta[0], 0., 0.);
    if !collision_query.iter().any(|&t| collision_check(target, t.translation)) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0., move_delta[1], 0.);
    if !collision_query.iter().any(|&t| collision_check(target, t.translation)) {
        transform.translation = target;
    }

}

fn collision_check(target_pos: Vec3, tile_collider_translation: Vec3) -> bool {
    let collision = collide(
        target_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        tile_collider_translation,
        Vec2::splat(TILE_SIZE),
    );
    collision.is_some()
}

fn spawn_player(mut commands: Commands, asset: Res<PlayerAsset>, player_spawn: Res<PlayerSpawn>) {
    println!("Draw Player");
    let mut player_sprite = TextureAtlasSprite::new(0);
    player_sprite.custom_size = Some(Vec2::new( 8., 13.));

    println!("{:?}", player_spawn);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_sprite,
            texture_atlas: asset.texture_atlas.clone(),
            transform: Transform {
                translation: Vec3::new(player_spawn.x as f32 * TILE_SIZE, player_spawn.y as f32 * TILE_SIZE, 200.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_BASE_SPEED
        });
}
