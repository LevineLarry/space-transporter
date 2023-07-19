use bevy::prelude::*;
use crate::WindowTransform;

#[derive(Component)]
pub struct Ship {
    pub target: Option<Entity>, //Reference to a planet component
    pub speed: f32,
    pub x: f32,
    pub y: f32
}

impl WindowTransform for Ship {
    fn to_window_coordinates(&self, window: &Window) -> Vec2 { //Allows us to convert the ships absolute coordinates to window coordinates
        let x = self.x + (window.width() / 2.0);
        let y = self.y + (window.height() / 2.0);
        Vec2::new(x, y)
    }
} 

#[derive(Component)]
pub struct ShipDebugText {
    pub target: Option<Entity>,
    pub text: String
}