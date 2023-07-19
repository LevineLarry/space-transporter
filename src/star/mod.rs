use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_system);
    }
}