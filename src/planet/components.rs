use bevy::prelude::*;

use crate::WindowTransform;

#[derive(Component, Clone)]
pub struct Planet {
    pub orbital_radius: f32,
    pub x: f32,
    pub y: f32,
    pub phase_angle: f32,
    pub crew_waiting: u32,
    pub time_since_last_crew_update: f32,
}

impl WindowTransform for Planet {
    fn to_window_coordinates(&self, window: &Window) -> Vec2 { //Allows us to convert the planets absolute coordinates to window coordinates
        let x = self.x + (window.width() / 2.0);
        let y = self.y + (window.height() / 2.0);
        Vec2::new(x, y)
    }
} 

impl Default for Planet {
    fn default() -> Self {
        Planet {
            orbital_radius: 0.0,
            x: 0.0,
            y: 0.0,
            phase_angle: 0.0,
            crew_waiting: 0,
            time_since_last_crew_update: 0.0
        }
    }
}

#[derive(Component)]
pub struct PlanetInfoText {
    pub target: Option<Entity>,
    pub text: String
}