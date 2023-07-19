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
        crew_waiting: 0
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

    let planet_debug_text = PlanetDebugText {
        target: Some(planet_entity.id()),
        text: format!("Orbital radius: {}\nPhase angle: {}\nWindow X: {}\nWindow Y: {}", planet.orbital_radius, planet.phase_angle, planet.to_window_coordinates(&window).x, planet.to_window_coordinates(&window).y)
    };

    commands.spawn(
        Text2dBundle {
            text: Text::from_section(&planet_debug_text.text, text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(window_coords.x, window_coords.y - 80., 0.0),
            ..default()
        }
    ).insert(planet_debug_text);

    planet
}

pub fn progress_orbits(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut planets_query: Query<(&mut Transform, &mut Planet, Entity), Without<PlanetDebugText>>,
    mut planet_debug_text_query: Query<(&mut Transform, &mut PlanetDebugText, &mut Text), Without<Planet>>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();
    let ds = time.delta_seconds() * ORBITAL_SPEED;

    for (mut planet_transform, mut planet, planet_entity) in planets_query.iter_mut() { 
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
        
        for (mut text_transform, mut debug_text, mut text) in planet_debug_text_query.iter_mut() {
            if debug_text.target.is_none() {
                println!("Text has no target");
                continue;
            }

            let target = debug_text.target.expect("Text has no target");

            if target == planet_entity {
                //debug_text.text = format!("Orbital radius: {:.2}\nPhase angle: {:.2}\nWindow X: {:.2}\nWindow Y: {:.2}\n Absolute X: {:.2}\nAbsolute Y: {:.2}", planet.orbital_radius, planet.phase_angle, planet.to_window_coordinates(&window).x, planet.to_window_coordinates(&window).y, planet.x, planet.y);
                debug_text.text = format!("{} crew waiting", planet.crew_waiting);
                text_transform.translation = Vec3::new(new_window_position.x, new_window_position.y - 80., 0.0);
                text.sections[0].value = debug_text.text.clone();
            }
        }
    }
}