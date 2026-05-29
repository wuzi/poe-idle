use bevy::prelude::*;

use crate::components::UiState;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::data::GameDatabase;
use crate::plugins::GameSystemsPlugin;
use crate::save::load_saved_game;
use crate::visual::setup;

pub(crate) fn run() {
    let database = GameDatabase::default();
    let loaded_game = load_saved_game(&database);
    if loaded_game.loaded_from_disk {
        eprintln!(
            "Loaded save from {}",
            loaded_game.save_state.path().display()
        );
    }

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.08)))
        .insert_resource(database)
        .insert_resource(loaded_game.profile)
        .insert_resource(loaded_game.run)
        .insert_resource(loaded_game.rng)
        .insert_resource(loaded_game.save_state)
        .insert_resource(UiState::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PoE Idle Prototype".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameSystemsPlugin)
        .add_systems(Startup, setup)
        .run();
}
