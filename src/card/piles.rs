use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Board, Card, CardBundle};

#[derive(Component)]
pub struct Pile;

#[derive(Component)]
pub struct Base;

pub fn spawn_pile(commands: &mut Commands, cards: Vec<Card>, x_position: u32, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba { 
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 0.2
                },
                ..default()
            },
            transform: Transform::from_xyz(-650.0 + (x_position*150) as f32, 100.0, -100.0),
            texture: asset_server.load(format!("cards/Back Blue 1.png")),
            ..default()
        },
        Base,
        Board,
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
    )).with_children(|base| {
        base.spawn((
            SpatialBundle {
                ..default()
            },
            Pile,
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x;
                transform.translation.y -= drag.delta.y;
                transform.translation.z = 1000.0;
            })
        ))
        .with_children(|pile| {
            for (i, &card) in cards.iter().enumerate() {
                if i == cards.len() - 1 {
                    pile.spawn((
                        CardBundle {
                            card,
                            sprite : SpriteBundle {
                                transform: Transform::from_xyz(0.0, -((i*50) as f32), (i*10+1) as f32),
                                texture: asset_server.load(format!("cards/{} {}.png", card.suit.to_string(), card.face.0)),
                                ..default()
                            },
                            pickable_bundle: PickableBundle::default()
                        },
                    ));
                } else {
                    pile.spawn((
                        CardBundle {
                            card,
                            sprite : SpriteBundle {
                                transform: Transform::from_xyz(0.0, -((i*50) as f32), (i*10+1) as f32),
                                texture: asset_server.load("cards/Back Blue 1.png"),
                                ..default()
                            },
                            pickable_bundle: PickableBundle {
                                pickable: Pickable::IGNORE,
                                ..default()
                            }
                        },
                    ));
                }
            }
        });
    });
    
}

pub fn format_piles(
    q_pile: &Query<&Children, With<Pile>>, 
    q_children: &Query<&Children, With<Card>>,
    transform_query: &mut Query<&mut Transform, With<Card>>
) {
    for children in q_pile.iter() {
        for (i, &child) in children.iter().enumerate() {
            if let Ok(mut transform) = transform_query.get_mut(child) {
                transform.translation.x = 0.0;
                transform.translation.y = -((i*50) as f32);
                transform.translation.z = (i*10+1) as f32;
            }

            for child in q_children.iter_descendants(child) {
                if let Ok(mut transform) = transform_query.get_mut(child) {
                    transform.translation.x = 0.0;
                    transform.translation.y = -50.0;
                    transform.translation.z = 10.0;
                }
            }
        }
    }
}

pub fn flip_last_card_of_piles(
    mut drag_drop_event: EventReader<Pointer<Drop>>,
    q_piles: Query<&Children, With<Pile>>,
    mut q_pickable: Query<(Entity, &mut Pickable, &mut Handle<Image>, &Card)>,
    asset_server: Res<AssetServer>
) {
    for _drop in drag_drop_event.read() {
        for piles in q_piles.iter() {
            if let Some(&last_in_pile) = piles.iter().last() {
                if let Ok((_, mut pickable, mut texture, card))= q_pickable.get_mut(last_in_pile) {
                    pickable.set_if_neq(Pickable::default());
                    texture.set_if_neq(asset_server.load(format!("cards/{} {}.png", card.suit.to_string(), card.face.0)));
                }
            }
        }
    }
}
