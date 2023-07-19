use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod planet;
mod ship;
mod star;
mod infotext;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(planet::PlanetPlugin)
        .add_plugins(ship::ShipPlugin)
        .add_plugins(star::StarPlugin)
        .add_plugins(infotext::InfoTextPlugin)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Startup, spawn_camera)
        .run();
        
}

pub trait WindowTransform {
    fn to_window_coordinates(&self, window: &Window) -> Vec2;
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
            ..default()
        }
    );
}