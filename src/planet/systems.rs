use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::WindowTransform;
use crate::infotext::systems::spawn_info_text;

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

    spawn_info_text(planet_entity.id(), planet.clone(), &window, commands, "0 crew waiting".to_string(), asset_server);

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