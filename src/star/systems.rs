use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::planet::components::Planet;
use crate::planet::systems::spawn_planet;
use crate::planet::NUM_PLANETS;
use super::components::*;

pub fn spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/star_large.png"),
            ..default()
        }
    )
    .insert(Star {});

    let mut planets: Vec<Planet> = Vec::new();
    for _ in 0..NUM_PLANETS {
        planets.push(spawn_planet(&mut commands, &window_query, &asset_server, &planets));
    }
}