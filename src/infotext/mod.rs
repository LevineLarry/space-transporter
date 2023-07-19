use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct InfoTextPlugin;

impl Plugin for InfoTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_ship_info_text)
            .add_systems(Update, update_planet_info_text);
    }
}