use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Board, Card, CardSuit};

#[derive(Component)]
pub struct Foundation(pub CardSuit);

pub fn spawn_foundations(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    
    spawn_foundation(
        commands, 
        CardSuit::Hearts, 
        asset_server.load("cards/Hearts 1.png"), 
        Transform::from_xyz(-50.0, 275.0, -10.0)
    );

    spawn_foundation(commands,
        CardSuit::Diamonds,
        asset_server.load("cards/Diamonds 1.png"),
        Transform::from_xyz(100.0, 275.0, -10.0)
    );

    spawn_foundation(commands,
        CardSuit::Clubs,
        asset_server.load("cards/Clubs 1.png"),
        Transform::from_xyz(250.0, 275.0, -10.0)
    );

    spawn_foundation(commands,
        CardSuit::Spades,
        asset_server.load("cards/Spades 1.png"),
        Transform::from_xyz(400.0, 275.0, -10.0)
    );
}

fn spawn_foundation(commands: &mut Commands<'_, '_>, suit: CardSuit, texture: Handle<Image>, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            transform,
            texture,
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        Foundation(suit),
        Board,
    ));
}

pub fn format_foundation(
    q_foundation: &Query<&Children, With<Foundation>>,
    q_children: &Query<&Children, With<Card>>,
    component_query: &mut Query<(&mut Visibility, &mut Transform), With<Card>>
) {

    for children in q_foundation {
        for &child in children {
            if let Ok((_, mut transform)) = component_query.get_mut(child) {
                transform.translation.x = 0.0;
                transform.translation.y = 0.0;
                transform.translation.z = 1.0;
            }
            
            if let Some(card_on_top) = q_children.iter_descendants(child).last() {
                for child in q_children.iter_descendants(child) {
                    if let Ok((mut visibility, mut transform)) = component_query.get_mut(child) {
                        transform.translation.x = 0.0;
                        transform.translation.y = 0.0;
                        if child == card_on_top {
                            *visibility = Visibility::Visible;
                            transform.translation.z = 1.0;
                        } else {
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    }
}