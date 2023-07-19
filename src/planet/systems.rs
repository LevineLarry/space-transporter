use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::WindowTransform;

use super::*;
use super::components::*;

pub fn spawn_planet(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    planets: &Vec<Planet>
) -> Planet {
    println!("Spawning planet...");
    let window = window_query.get_single().unwrap();

    let mut current_absolute_position = Vec2::ZERO;
    let mut orbit_radius = 0.0;
    let mut phase_angle = 0.0;
    
    let mut collision = true;

    //Logic to prevent spawning two planets such they overlap
    while collision {
        orbit_radius = (random::<f32>() * (MAX_ORBITAL_RADIUS - MIN_ORBITAL_RADIUS)) + MIN_ORBITAL_RADIUS;
        phase_angle = random::<f32>() * 2.0 * std::f32::consts::PI; //Phase angle in rads
        current_absolute_position = Vec2::new(
            orbit_radius * phase_angle.cos(),
            orbit_radius * phase_angle.sin()
        );

        collision = false;
        for planet in planets {
            let distance = Vec2::new(planet.x - current_absolute_position.x, planet.y - current_absolute_position.y).length();
            if distance < PLANET_SIZE || f32::abs(planet.orbital_radius - orbit_radius) < PLANET_SIZE {
                collision = true;
            }
        }
    }

    let sprite_scale = PLANET_SIZE / PLANET_IMAGE_SIZE;
    let planet = Planet {
        orbital_radius: orbit_radius,
        x: current_absolute_position.x,
        y: current_absolute_position.y,
        phase_angle,
        ..default()
    };

    let window_coords = planet.to_window_coordinates(&window);

    let mut planet_entity = commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(window_coords.x, window_coords.y, 0.0).with_scale(Vec3::new(sprite_scale, sprite_scale, 1.0)),
            texture: asset_server.load("sprites/sphere0.png"),
            ..default()
        }
    );

    planet_entity.insert(planet.clone());

    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let planet_info_text = PlanetInfoText {
        target: Some(planet_entity.id()),
        text: format!("0 crew waiting")
    };

    commands.spawn(
        Text2dBundle {
            text: Text::from_section(&planet_info_text.text, text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(window_coords.x, window_coords.y - 80., 0.0),
            ..default()
        }
    ).insert(planet_info_text);

    planet
}

pub fn progress_orbits(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut planets_query: Query<(&mut Transform, &mut Planet)>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    let ds = time.delta_seconds() * ORBITAL_SPEED;

    for (mut planet_transform, mut planet) in planets_query.iter_mut() { 
        let d_theta = ds / planet.orbital_radius; //Arc length
        let new_phase_angle = planet.phase_angle + d_theta;
        let new_absolute_position = Vec2::new(
            planet.orbital_radius * new_phase_angle.cos(),
            planet.orbital_radius * new_phase_angle.sin()
        );
        planet.phase_angle = new_phase_angle;
        planet.x = new_absolute_position.x;
        planet.y = new_absolute_position.y;

        let new_window_position = planet.to_window_coordinates(&window);
        planet_transform.translation = Vec3::new(new_window_position.x, new_window_position.y, 0.0);
        planet_transform.rotation = Quat::from_rotation_z(new_phase_angle + 0.5);
    }
}

/**
 * Finds the planet that each text entity is parented by, and updates the position & text content of the text
 */
pub fn update_text(
    window_query: Query<&Window, With<PrimaryWindow>>,
    planets_query: Query<(&Planet, Entity)>,
    mut planet_info_text_query: Query<(&mut Transform, &mut PlanetInfoText, &mut Text)>,
) {
    let window = window_query.get_single().unwrap();

    for (mut text_transform, mut info_text, mut text) in planet_info_text_query.iter_mut() {
        if info_text.target.is_none() {
            println!("Text has no target");
            continue;
        }

        let target_planet_entity = info_text.target.expect("Text has no target");
        let mut target_planet: Option<&Planet> = None;

        for (planet, planet_entity) in planets_query.iter() {
            if planet_entity == target_planet_entity {
                target_planet = Some(planet);
                break;
            }
        }

        if target_planet.is_none() {
            println!("Text has no parent planet");
            continue;
        }

        let target_planet_window_coords = target_planet.unwrap().to_window_coordinates(&window);
        info_text.text = format!("{} crew waiting", target_planet.unwrap().crew_waiting);
        text_transform.translation = Vec3::new(target_planet_window_coords.x, target_planet_window_coords.y - 80., 0.0);
        text.sections[0].value = info_text.text.clone();
    }
}

pub fn update_crew_waiting(
    mut planets_query: Query<&mut Planet>,
    time: Res<Time>
) {
    for mut planet in planets_query.iter_mut() {
        let crew_added = random::<f32>() * (1./CREW_ADD_SPEED) < planet.time_since_last_crew_update; //Increasing odds that crew will be added as the elapsed time increases

        if crew_added {
            planet.crew_waiting += 1;
            planet.time_since_last_crew_update = 0.;
        } else {
            planet.time_since_last_crew_update += time.delta_seconds();
        }
    }
}