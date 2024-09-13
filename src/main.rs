mod card;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use card::CardPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(low_latency_window_plugin())
                .set(bevy::log::LogPlugin {
                    filter: "bevy_mod_picking=trace".into(),
                    ..default()
                })
        )
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal)
        .add_plugins(CardPlugin)
        .run();
}