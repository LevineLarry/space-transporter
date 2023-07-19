use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::WindowTransform;
use crate::planet::{components::*, PLANET_SIZE};

pub fn spawn_ship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(50. + (window.width() / 2.0), 50. + (window.height() / 2.0), 0.0),
            texture: asset_server.load("sprites/ship_L.png"),
            ..default()
        }
    ).insert(Ship { 
        target: None,
        speed: 100.0,
        x: 0.,
        y: 0.
    });
}

pub fn progress_ships(
    mut ships_query: Query<(&mut Transform, &mut Ship)>,
    time: Res<Time>,
    planets_query: Query<(Entity, &Planet)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, mut ship) in ships_query.iter_mut() {
        if ship.target.is_none() {
            continue;
        }

        let target_entity = ship.target.unwrap();
        let mut target_planet: Option<&Planet> = None;

        //Iterate over the results from the planets query
        for (entity, planet) in planets_query.iter() {
            if entity == target_entity { //If the entity matches the target_entity, then the planet will be the target planet
                target_planet = Some(planet);
            }
        }

        //If the target planet is none, then the target entity is not a planet. Not sure how that would be possible, but it's a good check to have
        if target_planet.is_none() {
            continue;
        }

        let target_position = Vec2::new(target_planet.unwrap().x, target_planet.unwrap().y);
        let ship_position = Vec2::new(ship.x, ship.y);

        let direction = (target_position - ship_position).normalize(); //Normalized unit vector pointing in the direction of the target
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
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    
    //If the user just clicked their mouse
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut mouse_coords = window.cursor_position().unwrap();
        mouse_coords.y = window.height() - mouse_coords.y; //Y = 0 at bottom

        //Find if they clicked a planet
        for (planet, planet_entity) in planets_query.iter() {
            let planet_coords = planet.to_window_coordinates(&window);

            //If the click was within the planet's radius
            if (planet_coords - mouse_coords).length() <= PLANET_SIZE {
                let mut ship = ships_query.get_single_mut().unwrap();
                ship.target = Some(planet_entity);
            }
        }
    }
}