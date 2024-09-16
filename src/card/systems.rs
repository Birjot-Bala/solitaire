use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::foundation::{Foundation, format_foundation};
use super::piles::*;
use super::{Card, CardFace, CardSuit};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_card_drag_drop_event, flip_last_card_of_piles, handle_drag_end_event).chain());
    }
}

pub fn handle_drag_end_event(
    mut drag_end_event: EventReader<Pointer<DragEnd>>,
    q_pile: Query<&Children, With<Pile>>,
    q_children: Query<&Children, With<Card>>,
    q_foundation: Query<&Children, With<Foundation>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<Card>>,
        Query<(&mut Visibility, &mut Transform), With<Card>>
    )>
) {
    for _ in drag_end_event.read() {
        format_piles(&q_pile, &q_children, &mut set.p0());
        format_foundation(&q_foundation, &q_children, &mut set.p1());
    }
}

pub fn move_card_drag_drop_event(
    mut commands: Commands,
    mut drag_drop_event: EventReader<Pointer<Drop>>,
    q_child: Query<&Children>,
    q_parent: Query<&Parent>,
    q_piles: Query<Entity, With<Pile>>,
    q_cards: Query<&Card>,
    q_bases: Query<(Entity, &Children), With<Base>>,
    q_foundations: Query<&Foundation>
) {
    for drop in drag_drop_event.read() {
        if let Some(parent) = q_parent.iter_ancestors(drop.target).find(|&parent| q_piles.contains(parent)) {
            dropped_on_pile(&q_parent, drop, &q_piles, parent, &q_child, &q_cards, &mut commands);
        } else if let Ok((_,  children)) = q_bases.get(drop.target)  {
            dropped_on_empty_pile(&q_cards, drop, children, &mut commands);
        } else if let Some(parent) = q_parent.iter_ancestors(drop.target).find(|&parent| q_foundations.contains(parent)) {
            if let Ok(foundation) = q_foundations.get(parent) {
                if let Ok(dropped_card) = q_cards.get(drop.dropped) {
                    if foundation.0 == dropped_card.suit {
                        if let Ok(target_card) = q_cards.get(drop.target) {
                            if dropped_card.face.0.checked_sub(target_card.face.0) == Some(1) {
                                commands.entity(drop.target).add_child(drop.dropped);
                            }
                        }
                    }
                }
            }
        } else if let Ok(foundation) = q_foundations.get(drop.target) {
            if let Ok(dropped_card) = q_cards.get(drop.dropped) {
                if foundation.0 == dropped_card.suit && dropped_card.face == CardFace(1) {
                    commands.entity(drop.target).add_child(drop.dropped);
                }
            }
        }
    }
}

fn dropped_on_empty_pile(q_cards: &Query<&Card>, drop: &Pointer<Drop>, children: &Children, commands: &mut Commands) {
    if let Ok(card_comp) = q_cards.get(drop.dropped) {
        if card_comp.face == CardFace(13) {
            if let Some(&child) = children.first() {
                commands.entity(child).add_child(drop.dropped);
            }
        }
    }
}

fn dropped_on_pile(q_parent: &Query<&Parent>, drop: &Pointer<Drop>, q_piles: &Query<Entity, With<Pile>>, parent: Entity, q_child: &Query<&Children>, q_cards: &Query<&Card>, commands: &mut Commands) {
    if let Some(other_parent) = q_parent.iter_ancestors(drop.dropped).find(|&parent| q_piles.contains(parent)) {
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