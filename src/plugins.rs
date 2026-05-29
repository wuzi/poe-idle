use bevy::prelude::*;

use crate::gameplay::{
    enemies_attack, handle_map_transitions, move_enemies, move_player, player_attack,
    regenerate_player_health, resolve_combat_outcomes, spawn_enemy_packs,
};
use crate::save::{autosave_game, save_on_exit};
use crate::ui::{
    handle_bottom_buttons, handle_crafting_input, handle_inventory_input, handle_portal_button,
    handle_talent_panel, sync_character_panel, sync_crafting_panel, sync_dragged_item_visual,
    sync_hud_text, sync_inventory_grid, sync_inventory_panel, sync_portal_panel, sync_talent_panel,
    update_item_tooltip,
};
use crate::visual::{
    camera_follow, sync_character_visuals, sync_health_bars, sync_progress_bar,
    sync_screen_fixed_entities, tick_timed_entities,
};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameSet {
    Gameplay,
    Ui,
    Save,
}

pub(crate) struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameSet::Gameplay, GameSet::Ui, GameSet::Save).chain(),
        )
        .add_systems(
            Update,
            (
                handle_map_transitions,
                spawn_enemy_packs,
                move_player,
                move_enemies,
                player_attack,
                enemies_attack,
                regenerate_player_health,
                resolve_combat_outcomes,
                tick_timed_entities,
                sync_health_bars,
                sync_character_visuals,
                camera_follow,
            )
                .chain()
                .in_set(GameSet::Gameplay),
        )
        .add_systems(
            Update,
            (
                handle_bottom_buttons,
                handle_portal_button,
                handle_talent_panel,
                sync_portal_panel,
                sync_inventory_panel,
                handle_inventory_input,
                handle_crafting_input,
                update_item_tooltip,
                sync_character_panel,
                sync_crafting_panel,
                sync_talent_panel,
                sync_dragged_item_visual,
                sync_screen_fixed_entities,
                sync_progress_bar,
                sync_inventory_grid,
                sync_hud_text,
            )
                .chain()
                .in_set(GameSet::Ui),
        )
        .add_systems(Update, autosave_game.in_set(GameSet::Save))
        .add_systems(Last, save_on_exit);
    }
}
