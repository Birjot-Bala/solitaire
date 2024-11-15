use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::foundation::{Foundation, format_foundation};
use super::piles::*;
use super::stock::Stock;
use super::waste::{format_waste, Waste};
use super::{Card, CardFace, CardSuit};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_click_event, handle_drag_event, move_card_drag_drop_event, flip_last_card_of_piles, handle_drag_end_event).chain());
    }
}

fn handle_drag_event(
    mut drag_event: EventReader<Pointer<Drag>>,
    mut transform_query: Query<&mut Transform, With<Card>>,
    q_foundation: Query<Entity, With<Foundation>>,
    q_children: Query<&Children>
) {
    for drag in drag_event.read() {
        for foundation in q_foundation.iter() {
            if let Some(top_card) = q_children.iter_descendants(foundation).last() {
                if drag.target == top_card {
                    if let Ok(mut transform) = transform_query.get_mut(drag.target) {
                        transform.translation.x += drag.delta.x;
                        transform.translation.y -= drag.delta.y;
                        transform.translation.z = 1000.0;
                    }
                }
            } 
        }
    }
}

fn handle_click_event(
    mut commands: Commands,
    mut click_event: EventReader<Pointer<Click>>,
    q_stock: Query<(Entity, Option<&Children>), With<Stock>>,
    q_waste: Query<(Entity, Option<&Children>), With<Waste>>,
    q_children: Query<&Children, With<Card>>,
    mut transform_query: Query<(&mut Transform, &mut Handle<Image>, &Card), With<Card>>,
    asset_server: Res<AssetServer>,
) {
    for click in click_event.read() {
        let (stock, opt_children) = q_stock.single();
        if let Some(children) = opt_children {
            if children.contains(&click.target) {
                let (waste, waste_children) = q_waste.single();
                if let Some(waste_children) = waste_children {
                    for &child in waste_children {
                        if let Some(top_card) = q_children.iter_descendants(child).last() {
                            add_child_to_waste(&mut transform_query, click, &mut commands, top_card, &asset_server);
                        } else {
                            add_child_to_waste(&mut transform_query, click, &mut commands, child, &asset_server);
                        }
                    }
                } else {
                    add_child_to_waste(&mut transform_query, click, &mut commands, waste, &asset_server);
                }
            }
        } else {
            if stock == click.target {
                let (_, waste_children) = q_waste.single();
                if let Some(waste_children) = waste_children {
                    for &child in waste_children {
                        if let Ok((mut transform, mut texture, _)) = transform_query.get_mut(child) {
                            commands.entity(stock).add_child(child);
                            texture.set_if_neq(asset_server.load("cards/Back Blue 1.png"));
                            transform.translation.z = 10.0;
                        }

                        for (i, child) in q_children.iter_descendants(child).enumerate() {
                            if let Ok((mut transform, mut texture, _)) = transform_query.get_mut(child) {
                                commands.entity(stock).add_child(child);
                                texture.set_if_neq(asset_server.load("cards/Back Blue 1.png"));
                                transform.translation.z = 10.0 - 0.1 * (i as f32);
                            }
                        }

                    }
                }
            }
        }
    }
}

fn add_child_to_waste(transform_query: &mut Query<(&mut Transform, &mut Handle<Image>, &Card), With<Card>>, click: &Pointer<Click>, commands: &mut Commands, entity: Entity, asset_server: &Res<AssetServer>) {
    if let Ok((mut transform, mut texture, card)) = transform_query.get_mut(click.target) {
        commands.entity(entity).add_child(click.target);
        texture.set_if_neq(asset_server.load(format!("cards/{} {}.png", card.suit.to_string(), card.face.0)));
        transform.translation.z = 1.0;
    }
}

fn handle_drag_end_event(
    mut drag_end_event: EventReader<Pointer<DragEnd>>,
    q_pile: Query<&Children, With<Pile>>,
    q_children: Query<&Children, With<Card>>,
    q_foundation: Query<&Children, With<Foundation>>,
    q_waste: Query<&Children, With<Waste>>,
    mut transform_query: Query<&mut Transform, With<Card>>,
) {
    for _ in drag_end_event.read() {
        format_piles(&q_pile, &q_children, &mut transform_query);
        format_foundation(&q_foundation, &q_children, &mut transform_query);
        format_waste(&q_waste, &q_children, &mut transform_query);
    }
}

fn move_card_drag_drop_event(
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
                            if !q_child.contains(drop.dropped) {
                                if dropped_card.face.0.checked_sub(target_card.face.0) == Some(1) {
                                    commands.entity(drop.target).add_child(drop.dropped);
                                }
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
            move_to_pile(q_piles, parent, q_child, drop, q_cards, commands);
        }
    } else {
        move_to_pile(q_piles, parent, q_child, drop, q_cards, commands);
    }
}

fn move_to_pile(q_piles: &Query<Entity, With<Pile>>, parent: Entity, q_child: &Query<&Children>, drop: &Pointer<Drop>, q_cards: &Query<&Card>, commands: &mut Commands) {
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