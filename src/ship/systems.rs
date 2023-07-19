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

    let ship = Ship {
        ..default()
    };
    
    let window_coords = ship.to_window_coordinates(&window);

    let mut ship_entity = commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(50. + (window.width() / 2.0), 50. + (window.height() / 2.0), 0.0),
            texture: asset_server.load("sprites/ship_L.png"),
            ..default()
        }
    );
    
    ship_entity.insert(ship.clone());

    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let ship_info_text = ShipInfoText {
        target: Some(ship_entity.id()),
        text: format!("0 crew onboard")
    };

    commands.spawn(
        Text2dBundle {
            text: Text::from_section(&ship_info_text.text, text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(window_coords.x, window_coords.y - 80., 0.0),
            ..default()
        }
    ).insert(ship_info_text);
}

pub fn progress_ships(
    mut ships_query: Query<(&mut Transform, &mut Ship)>,
    time: Res<Time>,
    planets_query: Query<(Entity, &Planet)>,
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
        
        //Play SFX if ship arrived at planet this frame
        if ship.update_has_arrived(&target_planet.unwrap()) {
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/arrival.ogg"),
                ..default()
            });
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

/**
 * Finds the ship that each text entity is parented by, and updates the position & text content of the text
 */
pub fn update_text(
    window_query: Query<&Window, With<PrimaryWindow>>,
    ships_query: Query<(&Ship, Entity)>,
    mut ship_info_text_query: Query<(&mut Transform, &mut ShipInfoText, &mut Text)>,
) {
    let window = window_query.get_single().unwrap();

    for (mut text_transform, mut info_text, mut text) in ship_info_text_query.iter_mut() {
        if info_text.target.is_none() {
            println!("Text has no target");
            continue;
        }

        let target_ship_entity = info_text.target.expect("Text has no target");
        let mut target_ship: Option<&Ship> = None;

        for (ship, ship_entity) in ships_query.iter() {
            if ship_entity == target_ship_entity {
                target_ship = Some(ship);
                break;
            }
        }

        if target_ship.is_none() {
            println!("Text has no parent ship");
            continue;
        }

        let target_ship_window_coords = target_ship.unwrap().to_window_coordinates(&window);
        info_text.text = format!("{} crew onboard", target_ship.unwrap().crewmembers);
        text_transform.translation = Vec3::new(target_ship_window_coords.x, target_ship_window_coords.y - 80., 0.0);
        text.sections[0].value = info_text.text.clone();
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