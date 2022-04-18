use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable};
use serde::Deserialize;

#[derive(Debug, Inspectable, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Component)]
pub enum TilesType {
    GRASS, DIRT, SAND
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {

    }
}

fn create_map() {

}