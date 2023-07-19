use bevy::prelude::*;

use crate::WindowTransform;

#[derive(Component, Clone)]
pub struct Planet {
    pub orbital_radius: f32,
    pub x: f32,
    pub y: f32,
    pub phase_angle: f32,
    pub crew_waiting: i32
}

impl WindowTransform for Planet {
    fn to_window_coordinates(&self, window: &Window) -> Vec2 { //Allows us to convert the planets absolute coordinates to window coordinates
        let x = self.x + (window.width() / 2.0);
        let y = self.y + (window.height() / 2.0);
        Vec2::new(x, y)
    }
} 

#[derive(Component)]
pub struct PlanetDebugText {
    pub target: Option<Entity>,
    pub text: String
}