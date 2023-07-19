use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::any::type_name;

use crate::WindowTransform;
use super::components::*;

/**
 * Finds the target that each text entity is parented by, and updates the position & text content of the text
 */
pub fn update_text<T: Component + WindowTransform>(
    window_query: Query<&Window, With<PrimaryWindow>>,
    target_query: Query<(&T, Entity)>,
    mut info_text_query: Query<(&mut Transform, &mut InfoText, &mut Text)>,
) {
    let window = window_query.get_single().unwrap();

    for (mut text_transform, mut info_text, mut text) in info_text_query.iter_mut() {
        if info_text.target.is_none() {
            println!("Text has no target");
            continue;
        }

        let target_entity = info_text.target.expect("Text has no target");
        let mut target: Option<&T> = None;

        for (temp_target, temp_target_entity) in target_query.iter() {
            if target_entity == temp_target_entity {
                target = Some(temp_target);
                break;
            }
        }

        if target.is_none() {
            println!("Text has no parent ship");
            continue;
        }

        let target_window_coords = target.unwrap().to_window_coordinates(&window);

        let name = type_name::<T>();
        println!("Type name: {}", name);

        /*
        info_text.text = format!("{} crew onboard", target.unwrap().crewmembers);
        text_transform.translation = Vec3::new(target_ship_window_coords.x, target_ship_window_coords.y - 80., 0.0);
        text.sections[0].value = info_text.text.clone();
        */
    }
}