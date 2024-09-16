use bevy::prelude::*;

use super::Board;

#[derive(Component)]
pub struct Waste;

pub fn spawn_waste(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-350.0, 275.0, 0.0),
            texture: asset_server.load("cards/Back Blue 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Waste,
        Board
    ));
}