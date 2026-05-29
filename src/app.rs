use bevy::prelude::*;

use crate::components::UiState;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::data::{GameDatabase, LootRng, PlayerProfile, RunState};
use crate::gameplay::{
    enemies_attack, handle_map_transitions, move_enemies, move_player, player_attack,
    resolve_combat_outcomes, spawn_enemy_packs,
};
use crate::ui::{
    handle_bottom_buttons, sync_character_panel, sync_hud_text, sync_inventory_grid,
    update_item_tooltip,
};
use crate::visual::{
    camera_follow, setup, sync_character_visuals, sync_health_bars, sync_progress_bar,
    sync_screen_fixed_entities, tick_timed_entities,
};

pub(crate) fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.08)))
        .insert_resource(GameDatabase::default())
        .insert_resource(PlayerProfile::default())
        .insert_resource(RunState::default())
        .insert_resource(LootRng::default())
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_map_transitions,
                spawn_enemy_packs,
                move_player,
                move_enemies,
                player_attack,
                enemies_attack,
                resolve_combat_outcomes,
                tick_timed_entities,
                sync_health_bars,
                sync_character_visuals,
                camera_follow,
                handle_bottom_buttons,
                update_item_tooltip,
                sync_character_panel,
                sync_screen_fixed_entities,
                sync_progress_bar,
                sync_inventory_grid,
                sync_hud_text,
            )
                .chain(),
        )
        .run();
}
