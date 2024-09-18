use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Board, Card};

#[derive(Component)]
pub struct Waste;

pub fn spawn_waste(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-350.0, 275.0, -100.0),
            texture: asset_server.load("cards/Back Blue 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Waste,
        Board,
        PickableBundle {
            pickable: Pickable::IGNORE,
            ..default()
        },
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x;
            transform.translation.y -= drag.delta.y;
            transform.translation.z = 1000.0;
        })
    ));
}

pub fn format_waste(
    q_waste: &Query<&Children, With<Waste>>,
    q_children: &Query<&Children, With<Card>>,
    transform_query: &mut Query<&mut Transform, With<Card>>
) {
    for children in q_waste {
        for &child in children {
            if let Ok(mut transform) = transform_query.get_mut(child) {
                transform.translation.x = 0.0;
                transform.translation.y = 0.0;
                transform.translation.z = 1.0;
            }
            
            if let Some(top_card) = q_children.iter_descendants(child).last() {
                for child in q_children.iter_descendants(child) {
                    if let Ok(mut transform) = transform_query.get_mut(child) {
                        transform.translation.x = 0.0;
                        transform.translation.y = 0.0;
                        if child == top_card {
                            transform.translation.z = 1.0;
                        }
                    }
                }
            }
        }
    }
}