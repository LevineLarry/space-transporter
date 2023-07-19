use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ship)
            .add_systems(Update, progress_ships)
            .add_systems(Update, handle_ship_targeting);
    }
}