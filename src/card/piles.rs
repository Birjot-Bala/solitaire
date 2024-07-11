use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{Card, CardBundle, CardSuit};

pub struct PilesPlugin;

#[derive(Component)]
struct Pile;

impl Plugin for PilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_card_drag_drop_event, flip_last_card_of_piles, handle_drag_end_event).chain());
    }
}

pub fn spawn_pile(commands: &mut Commands, cards: Vec<Card>, x_position: u32, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-650.0 + (x_position*150) as f32, 100.0, 0.0),
            ..default()
        },
        Pile,
        On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
        On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.translation.x += drag.delta.x;
            transform.translation.y -= drag.delta.y;
            transform.translation.z = 100.0;
        })
    ))
    .with_children(|pile| {
        for (i, &card) in cards.iter().enumerate() {
            if i == cards.len() - 1 {
                pile.spawn((
                    CardBundle {
                        card,
                        sprite : SpriteBundle {
                            transform: Transform::from_xyz(0.0, -((i*50) as f32), (i*10) as f32),
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
                            transform: Transform::from_xyz(0.0, -((i*50) as f32), (i*10) as f32),
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
}


fn format_piles(
    q_pile: &Query<&Children, With<Pile>>, 
    q_children: &Query<&Children, With<Card>>,
    transform_query: &mut Query<&mut Transform, With<Card>>
) {
    for children in q_pile.iter() {
        for (i, &child) in children.iter().enumerate() {
            if let Ok(mut transform) = transform_query.get_mut(child) {
                transform.translation.x = 0.0;
                transform.translation.y = -((i*50) as f32);
                transform.translation.z = (i*10) as f32;
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

fn handle_drag_end_event(
    mut drag_end_event: EventReader<Pointer<DragEnd>>,
    q_pile: Query<&Children, With<Pile>>,
    q_children: Query<&Children, With<Card>>,
    mut transform_query: Query<&mut Transform, With<Card>>
) {
    for _ in drag_end_event.read() {
        format_piles(&q_pile, &q_children, &mut transform_query);
    }

}

fn move_card_drag_drop_event(
    mut commands: Commands,
    mut drag_drop_event: EventReader<Pointer<Drop>>,
    q_child: Query<&Children>,
    q_parent: Query<&Parent>,
    q_piles: Query<Entity, With<Pile>>,
    q_cards: Query<&Card>
) {
    for drop in drag_drop_event.read() {
        if let Some(parent) = q_parent.iter_ancestors(drop.target).last() {
            if let Some(other_parent) = q_parent.iter_ancestors(drop.dropped).last() {
                if parent != other_parent {
                    if let Ok(pile) = q_piles.get(parent) {
                        if let Some(last_card) = q_child.iter_descendants(pile).last() {
                            if last_card != drop.dropped {
                                if let Ok(last_card_comp) = q_cards.get(last_card) {
                                    if let Ok(dropped_card_comp) = q_cards.get(drop.dropped) {
                                        let black_suits = [CardSuit::Clubs, CardSuit::Spades];
                                        let red_suits = [CardSuit::Hearts, CardSuit::Diamonds];
                                        let alt_suit = (
                                            black_suits.contains(&dropped_card_comp.suit) && red_suits.contains(&last_card_comp.suit)) 
                                            || (red_suits.contains(&dropped_card_comp.suit) && black_suits.contains(&last_card_comp.suit)
                                        );
                                        if alt_suit && (last_card_comp.face.0.checked_sub(dropped_card_comp.face.0) == Some(1)) {
                                            commands.entity(last_card).add_child(drop.dropped);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn flip_last_card_of_piles(
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
