use bevy::prelude::*;

#[derive(Component)]
pub struct InfoText {
    pub target: Option<Entity>,
    pub text: String
}