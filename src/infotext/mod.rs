use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

use crate::planet::components::Planet;

pub struct InfoTextPlugin;

impl Plugin for InfoTextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_text::<Planet>);
    }
}