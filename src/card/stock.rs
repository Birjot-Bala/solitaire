use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Board, Card, CardBundle};

#[derive(Component)]
pub struct Stock;

pub fn spawn_stock(commands: &mut Commands, stock: Vec<Card>, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-500.0, 275.0, -100.0),
            ..default()
        },
        Stock,
        Board
    ))
    .with_children(|parent| {
        for card in stock {
            parent.spawn((
                CardBundle {
                    card,
                    sprite : SpriteBundle {
                        texture: asset_server.load("cards/Back Blue 1.png"),
                        ..default()
                    },
                    pickable_bundle: PickableBundle::default()
                },
                On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
                On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            ));
        }
    });
}