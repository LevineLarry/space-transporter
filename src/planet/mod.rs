use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub const ORBITAL_SPEED: f32 = 10.0;
pub const PLANET_SIZE: f32 = 64.0;
pub const NUM_PLANETS: usize = 4;
pub const MAX_ORBITAL_RADIUS: f32 = 400.0;
pub const MIN_ORBITAL_RADIUS: f32 = 100.0;
pub const PLANET_IMAGE_SIZE: f32 = 1024.;
pub const CREW_ADD_SPEED: f32 = 1./100.0;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, progress_orbits)
            .add_systems(Update, update_crew_waiting);
    }
}