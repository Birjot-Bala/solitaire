mod foundation;
mod piles;
mod systems;
mod stock;
mod waste;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use piles::Pile;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;

use self::piles::{spawn_pile, format_piles};
use self::foundation::spawn_foundations;
use self::stock::spawn_stock;
use self::systems::SystemsPlugin;
use self::waste::spawn_waste;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_board))
            .add_plugins(SystemsPlugin)
            .add_systems(Update, keyboard_input);
    }
}

#[derive(Component, EnumIter, Copy, Clone, Debug, PartialEq, strum_macros::Display)]
enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
struct CardFace(u8);

#[derive(Component, Copy, Clone, Debug, PartialEq)]
struct Card {
    suit: CardSuit,
    face: CardFace,
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    sprite: SpriteBundle,
    pickable_bundle: PickableBundle
}

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Board;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        Camera
    ));
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut stock = vec![];
    for suit in CardSuit::iter() {
        for face in 1..14 {
            let card = Card { suit, face: CardFace(face) };
            stock.push(card)
        }
    }

    let mut rng = rand::thread_rng();
    stock.shuffle(&mut rng);
    
    spawn_pile(&mut commands, stock.split_off(stock.len() - 1), 1, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 2), 2, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 3), 3, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 4), 4, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 5), 5, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 6), 6, &asset_server);
    spawn_pile(&mut commands, stock.split_off(stock.len() - 7), 7, &asset_server);

    spawn_stock(&mut commands, stock, &asset_server);

    spawn_foundations(&mut commands, &asset_server);

    spawn_waste(&mut commands, &asset_server);
}



fn game_reset(
    commands: &mut Commands, 
    cards: Query<Entity, With<Board>>,
) {
    for card in cards.iter() {
        commands.entity(card).despawn_recursive();
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    cards: Query<Entity, With<Board>>,
    asset_server: Res<AssetServer>,
    q_pile: Query<&Children, With<Pile>>,
    q_children: Query<&Children, With<Card>>,
    mut transform_query: Query<&mut Transform, With<Card>>) {
    if keys.just_pressed(KeyCode::F5) {
        game_reset(&mut commands, cards);
        spawn_board(commands, asset_server);
        format_piles(&q_pile, &q_children, &mut transform_query)
    }
}