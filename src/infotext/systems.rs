use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{WindowTransform, ship::components::Ship, planet::components::Planet};
use super::components::*;

pub fn spawn_info_text<T: Component + WindowTransform>(
    target_entity: Entity,
    target: T,
    window: &Window,
    commands: &mut Commands,
    default_text: String,
    asset_server: &Res<AssetServer>
) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let info_text = InfoText {
        target: Some(target_entity),
        text: default_text
    };

    let target_window_coords = target.to_window_coordinates(&window);

    commands.spawn(
        Text2dBundle {
            text: Text::from_section(&info_text.text, text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(target_window_coords.x, target_window_coords.y - 80., 0.0),
            ..default()
        }
    ).insert(info_text);
}

/**
 * Finds the target that each text entity is parented by, and updates the position & text content of the text
 */
pub fn update_ship_info_text(
    window_query: Query<&Window, With<PrimaryWindow>>,
    target_query: Query<(&Ship, Entity), With<Ship>>,
    mut info_text_query: Query<(&mut Transform, &mut InfoText, &mut Text)>,
) {
    let window = window_query.get_single().unwrap();

    for (mut text_transform, mut info_text, mut text) in info_text_query.iter_mut() {
        if info_text.target.is_none() {
            println!("Text has no target");
            continue;
        }

        let target_entity = info_text.target.expect("Text has no target");
        let mut target_ship: Option<&Ship> = None;

        for (_ship, _target_entity) in target_query.iter() {
            if target_entity == _target_entity {
                target_ship = Some(_ship);
                break;
            }
        }

        if target_ship.is_none() {
            //println!("Text has no parent ship");
            continue;
        }

        let target_window_coords = target_ship.unwrap().to_window_coordinates(&window);

        info_text.text = format!("{} crew onboard", target_ship.unwrap().crewmembers);
        text_transform.translation = Vec3::new(target_window_coords.x, target_window_coords.y - 80., 0.0);
        text.sections[0].value = info_text.text.clone();
    }
}

pub fn update_planet_info_text(
    window_query: Query<&Window, With<PrimaryWindow>>,
    target_query: Query<(&Planet, Entity), With<Planet>>,
    mut info_text_query: Query<(&mut Transform, &mut InfoText, &mut Text)>,
) {
    let window = window_query.get_single().unwrap();

    for (mut text_transform, mut info_text, mut text) in info_text_query.iter_mut() {
        if info_text.target.is_none() {
            println!("Text has no target");
            continue;
        }

        let target_entity = info_text.target.expect("Text has no target");
        let mut target_planet: Option<&Planet> = None;

        for (_planet, _target_entity) in target_query.iter() {
            if target_entity == _target_entity {
                target_planet = Some(_planet);
                break;
            }
        }

        if target_planet.is_none() {
            //println!("Text has no parent planet");
            continue;
        }

        let target_window_coords = target_planet.unwrap().to_window_coordinates(&window);

        info_text.text = format!("{} crew waiting", target_planet.unwrap().crew_waiting);
        text_transform.translation = Vec3::new(target_window_coords.x, target_window_coords.y - 80., 0.0);
        text.sections[0].value = info_text.text.clone();
    }
}