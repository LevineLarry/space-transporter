use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::WindowTransform;
use crate::infotext::systems::spawn_info_text;
use crate::planet::{components::*, PLANET_SIZE};

pub fn spawn_ship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let ship = Ship {
        ..default()
    };

    let mut ship_entity = commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(50. + (window.width() / 2.0), 50. + (window.height() / 2.0), 0.0),
            texture: asset_server.load("sprites/ship_L.png"),
            ..default()
        }
    );
    
    ship_entity.insert(ship.clone());

    spawn_info_text(ship_entity.id(), ship.clone(), &window, &mut commands, "0 crew onboard".to_string(), &asset_server);
}

pub fn progress_ships(
    mut ships_query: Query<(&mut Transform, &mut Ship)>,
    time: Res<Time>,
    mut planets_query: Query<(Entity, &mut Planet)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut ship) in ships_query.iter_mut() {
        if ship.target.is_none() {
            continue;
        }

        let target_entity = ship.target.unwrap();

        //Get the planet from the entity
        let target_tuple = planets_query.get_mut(target_entity);
        if target_tuple.is_err() {
            continue;
        }

        let mut target_planet = target_tuple.unwrap().1;

        let target_position = Vec2::new(target_planet.x, target_planet.y);
        let ship_position = Vec2::new(ship.x, ship.y);

        let direction = (target_position - ship_position).normalize(); //Normalized unit vector pointing in the direction of the target
        
        //Load crew and play SFX if ship arrived at planet this frame
        if ship.update_has_arrived(&target_planet) {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/arrival.ogg"),
                ..default()
            });

            ship.unload_crew();
            ship.load_crew(&mut target_planet);
        }
        
        transform.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x) - (PI / 2.)); //Rotate the ship to face the target
        let ds = time.delta_seconds() * ship.speed;
        let new_position = ship_position + (direction * ds); //Multiply the direction unit vector by the speed to move in that direction
        let ship_window_coords = ship.to_window_coordinates(&window);

        transform.translation = Vec3::new(ship_window_coords.x, ship_window_coords.y, 0.0);
        ship.x = new_position.x;
        ship.y = new_position.y;
    }
}

pub fn handle_ship_targeting(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ships_query: Query<&mut Ship>,
    planets_query: Query<(&Planet, Entity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let window = window_query.get_single().unwrap();
    
    //If the user just clicked their mouse
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut mouse_coords = window.cursor_position().unwrap();
        mouse_coords.y = window.height() - mouse_coords.y; //Y = 0 at bottom

        let mut planet_clicked = false;

        //Find if they clicked a planet
        for (planet, planet_entity) in planets_query.iter() {
            let planet_coords = planet.to_window_coordinates(&window);

            //If the click was within the planet's radius
            if (planet_coords - mouse_coords).length() <= PLANET_SIZE {
                let mut ship = ships_query.get_single_mut().unwrap();
                ship.target = Some(planet_entity);
                ship.has_arrived = false;
                planet_clicked = true;
            }
        }

        if planet_clicked {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/select.ogg"),
                ..default()
            });
        } else {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/pluck_001.ogg"),
                ..default()
            });
        }
    }
}