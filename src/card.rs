mod piles;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;

use self::piles::{spawn_pile, PilesPlugin};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board)
            .add_plugins(PilesPlugin);
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
struct Foundation(CardSuit);

#[derive(Component)]
struct Stock;

#[derive(Component)]
struct Pile;

#[derive(Component)]
struct Waste;

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());

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

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-500.0, 275.0, 0.0),
            ..default()
        },
        Stock
    ))
    .with_children(|parent| {
        for card in stock {
            parent.spawn(CardBundle {
                card,
                sprite : SpriteBundle {
                    texture: asset_server.load("cards/Back Blue 1.png"),
                    ..default()
                },
                pickable_bundle: PickableBundle::default()
            });
        }
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-50.0, 275.0, 0.0),
            texture: asset_server.load("cards/Hearts 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Foundation(CardSuit::Hearts),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 275.0, 0.0),
            texture: asset_server.load("cards/Diamonds 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Foundation(CardSuit::Diamonds)
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(250.0, 275.0, 0.0),
            texture: asset_server.load("cards/Clubs 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Foundation(CardSuit::Clubs)
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(400.0, 275.0, 0.0),
            texture: asset_server.load("cards/Spades 1.png"),
            sprite: Sprite {
                color: Color::Rgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 0.25 },
                ..default()
            },
            ..default()
        },
        Foundation(CardSuit::Spades)
    ));

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
        Waste
    ));
}