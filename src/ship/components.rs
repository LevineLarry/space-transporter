use bevy::prelude::*;
use crate::{WindowTransform, planet::{components::Planet, PLANET_SIZE}};

#[derive(Component, Clone, Debug)]
pub struct Ship {
    pub target: Option<Entity>, //Reference to a planet component
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub has_arrived: bool,
    pub crewmembers: u32
}

impl WindowTransform for Ship {
    fn to_window_coordinates(&self, window: &Window) -> Vec2 { //Allows us to convert the ships absolute coordinates to window coordinates
        let x = self.x + (window.width() / 2.0);
        let y = self.y + (window.height() / 2.0);
        Vec2::new(x, y)
    }
} 

impl Default for Ship {
    fn default() -> Self {
        Ship {
            target: None,
            speed: 100.0,
            x: 0.0,
            y: 0.0,
            has_arrived: false,
            crewmembers: 0
        }
    }
}

impl Ship {
    /**
     * Updates the has_arrived bool depending on proximity to the target
     * Returns true if it updated to "true", and false otherwise (even if it was already true previously)
     */
    pub fn update_has_arrived(&mut self, target: &Planet) -> bool {
        let target_position = Vec2::new(target.x, target.y);
        let ship_position = Vec2::new(self.x, self.y);
        
        let distance = (target_position - ship_position).length(); //Distance between the ship and the target

        if distance < PLANET_SIZE / 2. && !self.has_arrived {
            self.has_arrived = true;
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Component, Debug)]
pub struct ShipInfoText {
    pub target: Option<Entity>,
    pub text: String
}